
mod state_machine
{
    enum ConnectionState {
        Disconnected,
        Connecting { retries: u32 },
        Connected { server_id: String },
        Disconnecting,
    }

    struct Connection {
        state: ConnectionState,
    }

    impl Connection
    {
        fn new() -> Self {
            Self { state: ConnectionState::Disconnected }
        }

        fn connect(&mut self, server: &str) {
            match self.state {
                ConnectionState::Disconnected => {
                    println!("Initiating connection to {}", server);
                    self.state = ConnectionState::Connecting { retries: 0 };
                }
                ConnectionState::Connecting { retries } if retries < 3 => {
                    println!("Retry {} connecting to {}", retries + 1, server);
                    self.state = ConnectionState::Connecting { retries: retries + 1 };
                }
                ConnectionState::Connecting { .. } => {
                    println!("Connection successful to {}", server);
                    self.state = ConnectionState::Connected { server_id: server.to_string() };
                }
                _ => println!("Cannot connect in current state")
            }
        }

        fn status(&self) -> String {
            match &self.state {
                ConnectionState::Disconnected => "Disconnected".to_string(),
                ConnectionState::Connecting { retries } => format!("Connecting (attempt {})", retries + 1),
                ConnectionState::Connected { server_id } => format!("Connected to {}", server_id),
                ConnectionState::Disconnecting => "Disconnecting".to_string(),
            }
        }
    }

    pub fn demo()
    {
        let mut conn: Connection = Connection::new();
        println!("Status: {}", conn.status());

        conn.connect("db.example.com");
        println!("Status: {}", conn.status());

        conn.connect("db.example.com");
        println!("Status: {}", conn.status());

        conn.connect("db.example.com");
        println!("Status: {}", conn.status());

        conn.connect("db.example.com");
        println!("Status: {}", conn.status());

        // Status: Disconnected
        // Initiating connection to db.example.com
        // Status: Connecting (attempt 1)
        // Retry 1 connecting to db.example.com
        // Status: Connecting (attempt 2)
        // Retry 2 connecting to db.example.com
        // Status: Connecting (attempt 3)
        // Retry 3 connecting to db.example.com
        // Status: Connecting (attempt 4)
    }
}

pub fn test_all()
{
    state_machine::demo();
}