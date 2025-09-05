#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]


use log::{info, warn, error, debug, LevelFilter};
use simple_logger::SimpleLogger;

fn write_logs()
{
    // Уровень логирования можно указать явно:
    SimpleLogger::new().init().unwrap();

    info!("Это информационное сообщение.");
    warn!("Это предупреждение.");
    error!("Это ошибка.");
    debug!("Это отладочное сообщение.");
}


fn write_logs_with_level()
{
    SimpleLogger::new().with_level(LevelFilter::Warn).init().unwrap();

    info!("Это информационное сообщение.");
    warn!("Это предупреждение.");
    error!("Это ошибка.");
    debug!("Это отладочное сообщение.");
}



fn main()
{
    // write_logs();
    write_logs_with_level();
}
