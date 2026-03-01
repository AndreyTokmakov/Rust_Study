
mod ssh2_example
{
    use ssh2::Session;
    use std::io::Read;
    use std::net::TcpStream;

    fn exec_ssh_command() -> Result<(), Box<dyn std::error::Error>>
    {
        let tcp: TcpStream = TcpStream::connect("127.0.0.1:22022")?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        session.userauth_password("test", "test")?;
        assert!(session.authenticated());

        let mut channel = session.channel_session()?;
        channel.exec("ls -lar")?;

        let mut buffer: String = String::new();
        channel.read_to_string(&mut buffer)?;
        println!("{}", buffer);

        channel.wait_close()?;
        Ok(())
    }

    pub fn run()
    {
        exec_ssh_command().expect("TODO: panic message");
    }
}

mod openssl_pool_example
{
    use std::process::Output;
    use async_trait::async_trait;
    use deadpool::managed::{Manager, Pool, RecycleResult, RecycleError};
    use openssh::{KnownHosts, Session};
    use std::time::Duration;

    pub struct SshManager
    {
        host: String,
        user: String,
    }

    impl SshManager
    {
        pub fn new(user: &str, host: &str) -> Self
        {
            Self
            {
                host: host.to_string(),
                user: user.to_string(),
            }
        }
    }

    #[async_trait]
    impl Manager for SshManager
    {
        type Type = Session;
        type Error = openssh::Error;

        async fn create(&self) -> Result<Session, openssh::Error>
        {
            let addr: String = format!("{}@{}", self.user, self.host);
            println!("Creating new SSH connection: {}", addr);
            Session::connect(addr, KnownHosts::Strict).await
        }

        async fn recycle(&self,
                         conn: &mut Session,
                         _: &deadpool::managed::Metrics, ) -> RecycleResult<openssh::Error>
        {
            conn.check()
                .await
                .map_err(RecycleError::Backend)
        }
    }

    pub async fn run_command(pool: &Pool<SshManager>,
                             cmd: &str, ) -> Result<String, Box<dyn std::error::Error>>
    {
        let conn = pool.get().await?;
        let output: Output = conn
            .command("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .await?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[tokio::main]
    async fn run_test() -> Result<(), Box<dyn std::error::Error>>
    {
        let manager = SshManager::new("user", "127.0.0.1");
        let pool = Pool::builder(manager)
            .max_size(8)
            .timeouts(deadpool::managed::Timeouts
            {
                wait: Some(Duration::from_secs(5)),
                create: Some(Duration::from_secs(10)),
                recycle: Some(Duration::from_secs(5)),
            })
            .build()?;

        for i in 0..5 {
            let result = run_command(&pool, "uname -a").await?;
            println!("Result {}: {}", i, result.trim());
        }
        Ok(())
    }

    pub fn run()
    {
        let _ = run_test();
    }
}

mod ssh_pool_demo
{
    use async_trait::async_trait;
    use deadpool::managed::{Manager, Metrics, Object, Pool, RecycleError, RecycleResult, PoolError};
    use ssh2::{Channel, Session};
    use std::io::Read;
    use std::net::TcpStream;
    use std::sync::{Mutex, MutexGuard};
    use std::thread::JoinHandle;
    use std::time::Duration;

    type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
    type Result<T> = std::result::Result<T, DynError>;

    pub struct SshConnection {
        session: Session,
    }

    impl SshConnection
    {
        pub fn connect(host: &str,
                       port: u16,
                       user: &str,
                       password: &str) -> Result<Self>
        {
            let tcp: TcpStream = TcpStream::connect((host, port))?;
            tcp.set_read_timeout(Some(Duration::from_secs(10)))?;
            tcp.set_write_timeout(Some(Duration::from_secs(10)))?;
            let mut session: Session = Session::new()?;
            session.set_tcp_stream(tcp);
            session.handshake()?;
            session.userauth_password(user, password)?;
            if !session.authenticated() {
                return Err("authentication failed".into());
            }
            session.set_keepalive(true, 30);
            Ok(Self { session })
        }

        pub fn exec(&mut self, cmd: &str) -> Result<String>
        {
            let mut channel: Channel = self.session.channel_session()?;
            channel.exec(cmd)?;
            let mut output = String::new();
            channel.read_to_string(&mut output)?;
            channel.wait_close()?;
            Ok(output)
        }

        pub fn is_alive(&self) -> bool {
            self.session.authenticated()
        }
    }

    pub struct SshManager
    {
        host: String,
        port: u16,
        user: String,
        password: String,
    }

    impl SshManager {
        pub fn new(host: &str, port: u16, user: &str, password: &str) -> Self {
            Self {
                host: host.to_string(),
                port,
                user: user.to_string(),
                password: password.to_string(),
            }
        }
    }
    #[async_trait]
    impl Manager for SshManager
    {
        type Type = Mutex<SshConnection>;
        type Error = DynError;

        async fn create(&self) -> std::result::Result<Self::Type, Self::Error> {
            let conn: SshConnection = SshConnection::connect(&self.host, self.port, &self.user, &self.password)?;
            Ok(Mutex::new(conn))
        }

        async fn recycle(&self, conn: &mut Self::Type, _: &Metrics) -> RecycleResult<Self::Error> {
            let conn: MutexGuard<SshConnection> = conn.lock().unwrap();
            if conn.is_alive() { Ok(()) } else { Err(RecycleError::Message("SSH connection dead".into())) }
        }
    }

    async fn run_command(pool: &Pool<SshManager>, cmd: &str) -> Result<String>
    {
        let conn: Object<SshManager> = pool.get().await
            .map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(format!("{}", e)))?;
        let cmd = cmd.to_string();
        let output = tokio::task::spawn_blocking(move || {
            let mut conn = conn.lock().unwrap();
            conn.exec(&cmd)
        }).await??;
        Ok(output)
    }

    #[tokio::main]
    async fn exec_cmd_sequential() -> Result<()>
    {
        let manager: SshManager = SshManager::new("127.0.0.1", 22022, "test", "test");
        let pool: Pool<SshManager, Object<SshManager>> = Pool::builder(manager).max_size(4).build()?;

        for i in 0..3 {
            let result = run_command(&pool, "echo hello").await?;
            println!("Sequential {}: {}", i, result.trim());
        }

        Ok(())
    }

    #[tokio::main]
    async fn exec_cmd_parallel() -> Result<()>
    {
        let manager: SshManager = SshManager::new("127.0.0.1", 22022, "test", "test");
        let pool: Pool<SshManager, Object<SshManager>> = Pool::builder(manager).max_size(4).build()?;

        let mut handles = Vec::new();
        for i in 0..5 {
            let pool = pool.clone();
            handles.push(tokio::spawn(async move {
                let result = run_command(&pool, "hostname").await.unwrap();
                println!("Parallel {}: {}", i, result.trim());
            }));
        }

        for handle in handles {
            handle.await?;
        }

        Ok(())
    }

    pub fn run()
    {
        // let _ = exec_cmd_sequential();
        let _ = exec_cmd_parallel();
    }
}


pub fn test_all()
{
    // ssh2_example::run();
    // openssl_pool_example::run();
    ssh_pool_demo::run();
}