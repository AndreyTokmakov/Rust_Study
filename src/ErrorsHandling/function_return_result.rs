use std::fs;
use std::fs::File;
use std::io::Error;
use std::net::{ AddrParseError, Ipv6Addr };

fn dev_as_string(a: i32, b: i32) -> Result<String, String> {
    if (0 == b) {
        Err(String::from("Error: b == 0"))
    }
    else {
        Ok((a/b).to_string())
    }
}

pub fn return_result_or_err()
{
    {
        let res = dev_as_string(10, 2);
        println!("{}", res.unwrap());
    }
    {
        let res = dev_as_string(10, 0);
        println!("{}", res.unwrap());
    }
}


mod return_multiple_errors_from_functions
{
    use std::error::Error;
    use std::fs;
    use std::net::{Ipv4Addr, Ipv6Addr};

    struct Config
    {
        host: String,
        file_path: String,
    }
    
    impl Config
    {
        fn new(host: &str, path: &str) -> Config {
            Config {
                host: String::from(host),
                file_path: String::from(path)
            }
        }
    }

    fn handle_config_bad_(config: Config) -> Result<(), std::io::Error>
    {
        let contents: String = fs::read_to_string(config.file_path)?;
        
        // let localhost = "::1".parse::<Ipv6Addr>()?;
        // Compile error: `?` could not convert error type `AddrParseError` to `Error`
        
        Ok(())
    }

    fn handle_config(config: Config) -> Result<(), Box<dyn Error>>
    {
        println!("Reading file {} .... ", &config.file_path);
        let contents: String = fs::read_to_string(config.file_path)?;
        println!("OK");
        
        println!("Resolving address {} as Ipv4Addr .... ", &config.host);
        let localhost = config.host.parse::<Ipv4Addr>()?;
        println!("OK");

        println!("Resolving address {} as Ipv6Addr .... ", &config.host);
        let localhost = config.host.parse::<Ipv6Addr>()?;
        println!("OK");
        
        Ok(())
    }
    
    
    pub fn demo()
    {
        let cfg: Config = Config::new("0.0.0.0", "/tmp/test/file.txt");
        handle_config(cfg).expect("Failed to handle config");
    }
}




pub fn test_all()
{
    // return_result_or_err();

    return_multiple_errors_from_functions::demo();
}