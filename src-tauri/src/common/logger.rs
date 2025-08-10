use fern::Dispatch;
use log::LevelFilter;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .level(LevelFilter::Info)
        // // filter spammy tao / winit event loop spam in app.log
        .level_for(
            "tao::platform_impl::platform::event_loop::runner",
            LevelFilter::Error,
        )
        .chain(
            Dispatch::new()
                .level(LevelFilter::Info)
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}] {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        message
                    ));
                })
                .chain(std::io::stdout()),
        )
        .chain(
            Dispatch::new()
                .level(LevelFilter::Debug)
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{} [{}][{}][{}:{}] {}",
                        chrono::Local::now().to_rfc3339(),
                        record.level(),
                        record.target(),
                        record.file().unwrap_or("unknown"),
                        record.line().unwrap_or(0),
                        message
                    ));
                })
                .chain(fern::log_file("app.log")?),
        )
        .apply()?;
    Ok(())
}

// convert to our AppError and log with context, then return the AppError
#[macro_export]
macro_rules! try_log {
    ($expr:expr, $ctx:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let ae: $crate::common::error::AppError = e.into();
                log::error!("{} failed: {:?}", $ctx, ae);
                return Err(ae);
            }
        }
    };
}

#[tauri::command]
pub fn process_frontend_error(level: &str, message: &str) {
    match level {
        "debug" => log::debug!("{message}"),
        "warn" => log::warn!("{message}"),
        "error" => log::error!("{message}"),
        _ => log::info!("{message}"),
    }
}
