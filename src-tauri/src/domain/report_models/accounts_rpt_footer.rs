use chrono::{DateTime, Local};

// Data needed to render the universal report footer.
pub struct ReportFooter {
    pub printed_on: DateTime<Local>,
    pub accounts_total: i32,
    pub page_number: i32,
}
