use crate::common::error::AppError;
use crate::domain::models::club_transaction::TransactionType;
use crate::domain::models::{ClubImport, ClubTransaction, Customer};
use crate::domain::repos::{ClubImportRepoTrait, ClubTransactionRepoTrait, CustomerRepoTrait};
use crate::infrastructure::pdf_parser::PdfParser;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use regex::Regex;
use std::sync::Arc;

pub struct PdfParseUseCases {
    parser: Arc<dyn PdfParser>,
    import_repo: Arc<dyn ClubImportRepoTrait>,
    tx_repo: Arc<dyn ClubTransactionRepoTrait>,
    cust_repo: Arc<dyn CustomerRepoTrait>,
}

impl PdfParseUseCases {
    pub fn new(
        parser: Arc<dyn PdfParser>,
        import_repo: Arc<dyn ClubImportRepoTrait>,
        tx_repo: Arc<dyn ClubTransactionRepoTrait>,
        cust_repo: Arc<dyn CustomerRepoTrait>,
    ) -> Self {
        Self {
            parser,
            import_repo,
            tx_repo,
            cust_repo,
        }
    }

    fn parse_date_midnight(s: &str, label: &str) -> Result<NaiveDateTime, AppError> {
        NaiveDate::parse_from_str(s, "%-m/%-d/%Y")
            .map_err(|e| {
                let msg = format!("Invalid {} '{}': {}", label, s, e);
                log::error!("{}", msg);
                AppError::Unexpected(msg)
            })?
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| {
                let msg = format!("Invalid {} date time '{}'", label, s);
                log::error!("{}", msg);
                AppError::Unexpected(msg)
            })
    }

    fn parse_money(raw: &str, label: &str) -> Result<f64, AppError> {
        let cleaned = raw.replace(',', "");
        cleaned.parse::<f64>().map_err(|e| {
            let msg = format!("Invalid {} amount '{}': {}", label, raw, e);
            log::error!("{}", msg);
            AppError::Unexpected(msg)
        })
    }

    pub fn pdf_parse(&self, filename: String) -> Result<ClubImport, AppError> {
        let raw = self.parser.parse(filename.clone()).map_err(|e| {
            log::error!("Error reading PDF bytes: {}", e);
            AppError::Unexpected(e.to_string())
        })?;

        log::info!("{}", raw);

        const RE_DATE_GRP: &str = r"(\d{1,2}/\d{1,2}/\d{4})";
        const RE_CURRENCY_GRP: &str = r"(\(?\$[\d,]+\.\d\d\)?)";
        const RE_RANGE: &str = r"(\d{1,2}/\d{1,2}/\d{4})\s*-\s*(\d{1,2}/\d{1,2}/\d{4})";

        // Match date
        let date_re = Regex::new(RE_DATE_GRP).map_err(|e| {
            log::error!("Bad date regex: {}", e);
            AppError::Unexpected("Internal parser regex error".into())
        })?;

        let import_date_str = date_re
            .captures(&raw)
            .and_then(|c| c.get(1).map(|m| m.as_str()))
            .ok_or_else(|| {
                log::error!("Missing import date in PDF text");
                AppError::Unexpected("Missing import date".into())
            })?;
        let import_date = Self::parse_date_midnight(import_date_str, "import date")?;

        // Match range
        let range_re = Regex::new(RE_RANGE).map_err(|e| {
            log::error!("Bad range regex: {}", e);
            AppError::Unexpected("Internal parser regex error".into())
        })?;
        let caps = range_re.captures(&raw).ok_or_else(|| {
            log::error!("Missing date range in PDF text");
            AppError::Unexpected("Missing date range".into())
        })?;
        let from = Self::parse_date_midnight(&caps[1], "from date")?;
        let to = Self::parse_date_midnight(&caps[2], "to date")?;

        // Build model
        let import = ClubImport {
            id: 0,
            date: import_date,
            activity_from: from,
            activity_to: to,
            source_file: filename.clone(),
        };
        self.import_repo.create(&import)?;

        // Compile transaction regex
        let tx_re = Regex::new(&format!(
            r"^{}\s+(Recd Client Donation/Dues|W/D General)\s+(.+?)\s+{}\s+{}$",
            RE_DATE_GRP, RE_CURRENCY_GRP, RE_CURRENCY_GRP
        ))
        .map_err(|e| {
            log::error!("Bad transaction regex: {}", e);
            AppError::Unexpected("Internal parser regex error".into())
        })?;
        log::info!("d");
        let mdoc_re = Regex::new(r"\((\d+)\)").map_err(|e| {
            log::error!("Bad mdoc regex: {}", e);
            AppError::Unexpected("Internal parser regex error".into())
        })?;

        // Find every transaction
        for c in tx_re.captures_iter(&raw) {
            let tdate = Self::parse_date_midnight(&c[1], "transaction date")?;
            let tx_type = if &c[2] == "Recd Client Donation/Dues" {
                TransactionType::Deposit
            } else {
                TransactionType::Withdrawal
            };
            // extract name + optional mdoc
            let desc_part = &c[3];
            let mdoc = match mdoc_re.captures(desc_part) {
                Some(mc) => Some(mc[1].parse::<i32>().map_err(|e| {
                    let msg = format!("Invalid mdoc '{}' in '{}': {}", &mc[1], desc_part, e);
                    log::error!("{}", msg);
                    AppError::Unexpected(msg)
                })?),
                None => None,
            };
            let name = mdoc_re.replace(desc_part, "").trim().to_string();
            // parse as dollars then convert to cents
            let dollars = Self::parse_money(&c[4], format!("{:?}", tx_type).as_str())?;
            let mut cents = (dollars * 100.0).round() as i32;
            if let TransactionType::Withdrawal = tx_type {
                cents = -cents;
            }
            // build tx
            let tx = ClubTransaction {
                id: 0,
                import_id: import.id,
                date: tdate,
                tx_type: tx_type.clone(),
                mdoc,
                entity_name: name.clone(),
                amount: cents,
            };
            log::info!("Parsed ClubTransaction: {:?}", tx);
            self.tx_repo.create(&tx)?;

            if let Some(m) = mdoc {
                match self.cust_repo.get_by_mdoc(m)? {
                    Some(mut existing) => {
                        existing.updated = Utc::now().naive_utc();
                        self.cust_repo.update(&existing)?;
                    }
                    None => {
                        let new_c = Customer {
                            mdoc: m,
                            name: name.clone(),
                            added: Utc::now().naive_utc(),
                            updated: Utc::now().naive_utc(),
                        };
                        log::info!("Creating new customer: {:?}", new_c);
                        self.cust_repo.create(&new_c)?;
                    }
                }
            }
        }

        Ok(import)
    }
}
