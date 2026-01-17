
mod config
{
    enum DatabaseConfig {
        MySQL {
            host: String,
            port: u16,
            username: String,
            password: String,
            database_name: String,
        },
        PostgreSQL {
            connection_string: String,
        },
        SQLite {
            file_path: String,
        },
    }

    fn connect_to_database(config: DatabaseConfig) {
        match config {
            DatabaseConfig::MySQL { host, port, username, database_name, .. } => {
                println!("Connecting to MySQL database '{}' on {}:{} as user '{}'",
                         database_name, host, port, username);
                // MySQL-specific connection code would go here
            },
            DatabaseConfig::PostgreSQL { connection_string } => {
                println!("Connecting to PostgreSQL with connection string: {}", connection_string);
                // PostgreSQL-specific connection code would go here
            },
            DatabaseConfig::SQLite { file_path } => {
                println!("Opening SQLite database at: {}", file_path);
                // SQLite-specific connection code would go here
            },
        }
    }

    pub fn demo()
    {
        let configs: [DatabaseConfig; 3] = [
            DatabaseConfig::MySQL {
                host: "localhost".to_string(),
                port: 3306,
                username: "admin".to_string(),
                password: "password123".to_string(),
                database_name: "users".to_string(),
            },
            DatabaseConfig::PostgreSQL {
                connection_string: "postgresql://user:pass@localhost/mydb".to_string(),
            },
            DatabaseConfig::SQLite {
                file_path: "/var/data/app.db".to_string(),
            },
        ];

        for config in configs {
            connect_to_database(config);
        }
    }
}

pub fn test_all()
{
    config::demo();
}