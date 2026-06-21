
mod tokio_broadcast_observer
{
    use tokio::sync::broadcast;

    #[derive(Debug, Clone)]
    struct TemperatureEvent {
        value: f64,
    }

    struct TemperatureSensor {
        tx: broadcast::Sender<TemperatureEvent>,
    }

    impl TemperatureSensor
    {
        fn new() -> Self {
            let (tx, _) = broadcast::channel(32);
            Self { tx }
        }

        fn subscribe(&self) -> broadcast::Receiver<TemperatureEvent> {
            return self.tx.subscribe()
        }

        fn set_temperature(&self, value: f64) {
            println!("Sensor updated:  {:.1}°C", value);
            let _ = self.tx.send(TemperatureEvent { value });
        }
    }

    // Observer #1
    async fn logger(mut rx: broadcast::Receiver<TemperatureEvent>)
    {
        while let Ok(event) = rx.recv().await {
            println!("[LOGGER ] {:.1}°C", event.value);
        }
    }

    // Observer #2
    async fn display(mut rx: broadcast::Receiver<TemperatureEvent>)
    {
        while let Ok(event) = rx.recv().await {
            println!("[DISPLAY] {:.1}°C", event.value);
        }
    }

    async fn alarm(mut rx: broadcast::Receiver<TemperatureEvent>,
                   limit: f64 )
    {
        while let Ok(event) = rx.recv().await {
            if event.value > limit {
                println!("[ALARM  ] temperature {} exceeds limit {}", event.value, limit);
            }
        }
    }

    pub async fn demo()
    {
        let sensor: TemperatureSensor = TemperatureSensor::new();
        tokio::spawn(logger(sensor.subscribe()));
        tokio::spawn(display(sensor.subscribe()));
        tokio::spawn(alarm(sensor.subscribe(), 30.0));

        sensor.set_temperature(20.0);
        sensor.set_temperature(25.0);
        sensor.set_temperature(35.0);

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main]
pub async fn demo()
{
    tokio_broadcast_observer::demo().await;

    // [LOGGER ] 20
    // [LOGGER ] 25
    // [ALARM  ] temperature 35 exceeds limit 30
    // [DISPLAY] 20.0°C
    // [DISPLAY] 25.0°C
    // [DISPLAY] 35.0°C
    // [LOGGER ] 35
}
