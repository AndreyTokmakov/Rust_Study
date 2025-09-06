

mod logging
{

    use log::{info, warn, error, debug, LevelFilter};
    use simple_logger::SimpleLogger;

    pub fn write_logs()
    {
        // Уровень логирования можно указать явно:
        SimpleLogger::new().init().unwrap();

        info!("Это информационное сообщение.");
        warn!("Это предупреждение.");
        error!("Это ошибка.");
        debug!("Это отладочное сообщение.");
    }


    pub fn write_logs_with_level()
    {
        SimpleLogger::new().with_level(LevelFilter::Warn).init().unwrap();

        info!("Это информационное сообщение.");
        warn!("Это предупреждение.");
        error!("Это ошибка.");
        debug!("Это отладочное сообщение.");
    }
}

pub fn test_all()
{
    logging::write_logs();
    // logging::write_logs_with_level();
}