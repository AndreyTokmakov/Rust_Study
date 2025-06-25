
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


pub fn test_all()
{
    // write_logs();
    write_logs_with_level();
}