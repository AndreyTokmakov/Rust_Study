
mod stock_price_observer
{
    use crossbeam_channel::{ Receiver, Sender, unbounded };
    use std::thread;
    use std::time::Duration;

    #[derive(Debug, Clone)]
    struct StockPriceEvent
    {
        symbol: String,
        price: f64,
    }

    struct StockExchange {
        subscribers: Vec<Sender<StockPriceEvent>>
    }

    impl StockExchange
    {
        fn new() -> Self {
            return Self { subscribers: Vec::new() }
        }

        fn subscribe(&mut self,
                     subscriber: Sender<StockPriceEvent>)
        {
            self.subscribers.push(subscriber);
        }

        fn update_price(&self, symbol: &str, price: f64)
        {
            let event: StockPriceEvent = StockPriceEvent {
                symbol: symbol.to_string(),
                price
            };
            for subscriber in &self.subscribers {
                let _ = subscriber.send(event.clone());
            }
        }
    }

    struct Logger {
        rx: Receiver<StockPriceEvent>,
    }

    struct Dashboard {
        rx: Receiver<StockPriceEvent>,
    }

    struct TradingBot {
        rx: Receiver<StockPriceEvent>,
        buy_threshold: f64
    }


    impl Logger
    {
        fn new(rx: Receiver<StockPriceEvent>) -> Self {
            Self { rx }
        }

        fn run(self) {
            while let Ok(event) = self.rx.recv() {
                println!("[LOGGER   ] {} -> {}", event.symbol, event.price);
            }
        }
    }

    impl TradingBot
    {
        fn new(rx: Receiver<StockPriceEvent>,
               buy_threshold: f64 ) -> Self
        {
            Self { rx, buy_threshold }
        }

        fn run(self) {
            while let Ok(event) = self.rx.recv() {
                if event.price < self.buy_threshold {
                    println!("[BOT      ] BUY {} at {}", event.symbol, event.price);
                }
            }
        }
    }

    impl Dashboard
    {
        fn new(rx: Receiver<StockPriceEvent>) -> Self {
            Self { rx }
        }

        fn run(self) {
            while let Ok(event) = self.rx.recv() {
                println!("[DASHBOARD] {} = {}", event.symbol, event.price);
            }
        }
    }

    pub fn demo()
    {
        let mut exchange: StockExchange = StockExchange::new();

        let (tx_logger, rx_logger) = unbounded();
        let (tx_bot, rx_bot) = unbounded();
        let (tx_dashboard, rx_dashboard) = unbounded();

        exchange.subscribe(tx_logger);
        exchange.subscribe(tx_bot);
        exchange.subscribe(tx_dashboard);

        let logger = Logger::new(rx_logger);
        let bot = TradingBot::new(rx_bot, 100.0 );
        let dashboard = Dashboard::new(rx_dashboard );

        thread::spawn(move || logger.run());
        thread::spawn(move || bot.run());
        thread::spawn(move || dashboard.run());

        exchange.update_price("AAPL", 180.0);
        exchange.update_price("TSLA", 95.0);
        exchange.update_price("NVDA", 210.0);

        thread::sleep(Duration::from_secs(1));
    }
}

pub fn demo()
{
    stock_price_observer::demo();

    // [BOT      ] BUY TSLA at 95
    // [LOGGER   ] AAPL -> 180
    // [LOGGER   ] TSLA -> 95
    // [LOGGER   ] NVDA -> 210
    // [DASHBOARD] AAPL = 180
    // [DASHBOARD] TSLA = 95
    // [DASHBOARD] NVDA = 210
}