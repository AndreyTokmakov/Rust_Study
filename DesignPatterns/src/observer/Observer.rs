
mod temperature_sensor
{
    struct Logger;
    struct Display;
    struct Alarm {
        limit: f64,
    }

    trait Observer {
        fn update(&self, temperature: f64);
    }

    impl Observer for Logger {
        fn update(&self, temperature: f64) {
            println!("[LOGGER] temperature = {temperature}");
        }
    }

    impl Observer for Display {
        fn update(&self, temperature: f64) {
            println!("[DISPLAY] {:.1}°C", temperature);
        }
    }

    impl Observer for Alarm {
        fn update(&self, temperature: f64) {
            if temperature > self.limit {
                println!("[ALARM] temperature {} exceeds limit {}", temperature, self.limit);
            }
        }
    }


    struct TemperatureSensor
    {
        temperature: f64,
        observers: Vec<Box<dyn Observer>>,
    }

    impl TemperatureSensor
    {
        fn new() -> Self {
            Self {
                temperature: 0.0,
                observers: Vec::new(),
            }
        }

        fn subscribe(&mut self, observer: Box<dyn Observer>) {
            self.observers.push(observer);
        }

        fn notify(&self)
        {
            for observer in &self.observers {
                observer.update(self.temperature);
            }
        }

        fn set_temperature(&mut self, temperature: f64)
        {
            self.temperature = temperature;
            println!("\nSensor updated: {}", temperature);
            self.notify();
        }
    }

    pub fn demo()
    {
        let mut sensor: TemperatureSensor = TemperatureSensor::new();

        sensor.subscribe(Box::new(Logger));
        sensor.subscribe(Box::new(Display));
        sensor.subscribe(Box::new(Alarm { limit: 30.0, }));

        sensor.set_temperature(22.5);
        sensor.set_temperature(28.0);
        sensor.set_temperature(35.2);

        // Sensor updated: 22.5
        // [LOGGER] temperature = 22.5
        // [DISPLAY] 22.5°C
        //
        // Sensor updated: 28
        // [LOGGER] temperature = 28
        // [DISPLAY] 28.0°C
        //
        // Sensor updated: 35.2
        // [LOGGER] temperature = 35.2
        // [DISPLAY] 35.2°C
        // [ALARM] temperature 35.2 exceeds limit 30
    }
}

pub fn test_all()
{
    temperature_sensor::demo();
}