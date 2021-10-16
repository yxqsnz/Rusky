use crate::Result;
use chrono::Local;
use log::Level;
use tempfile::Builder;
use tracing::info;
use yansi::Paint;
pub fn setup() -> Result<()> {
    let f = Builder::new()
        .prefix("rusky")
        .suffix("log")
        .rand_bytes(8)
        .tempfile()?;
    fern::Dispatch::new()
        .format(|out, message, record| {
            /*out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))*/

            let s = format!(
                "   ╭ {} \n ● │ {}\n   ╰ at {} on {}\n",
                message,
                record.level(),
                record.target(),
                Local::now().format("%Y-%m-%d/%H:%M:%S")
            );
            out.finish(format_args!("{}", match record.level() {
                Level::Warn => Paint::yellow(s),
                Level::Error => Paint::red(s),
                Level::Info => Paint::green(s),
                Level::Trace => Paint::cyan(s),
                _ => Paint::default(s),
            }));
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file(f.path().to_str().expect("a file"))?)
        .apply()?;

    info!("Rusky LOG_FILE: {}", f.path().to_str().expect("a file."));
    Ok(())
}
