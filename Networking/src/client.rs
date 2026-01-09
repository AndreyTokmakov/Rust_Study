use std::io::Write;
use std::net::TcpStream;
use http::{Request, Response};
use http::request::Builder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};


fn test_1()
{
    /*
    let mut request: Builder = Request::builder()
        .uri("https://www.rust-lang.org/")
        .header("User-Agent", "my-awesome-agent/1.0");

    let response = send(request.body(()).unwrap());
    */
}


fn send_request()
{
    let response= reqwest::blocking::get("https://www.rust-lang.org");
    println!("{:#?}", response);
}

fn send_request_1()
{
    let response = reqwest::blocking::get("http://127.0.0.1:52525");
    println!("{:#?}", response);
}

fn send_request_get_response_text()
{
    let response = reqwest::blocking::get("http://127.0.0.1:52525");

    let content = response.unwrap().text();
    println!("{:#?}", content);
}

fn send_request_2()
{
    let result = reqwest::blocking::get("http://127.0.0.1:52525/binance/api/v3/account");
    let response = result.unwrap();

    println!("{:#?}", response.status());
    //println!("Response: {:?}", response.text());

    let body = response.text().unwrap().replace("\\\"", "\"");
    println!("Response: {}", body);


    // let json_body: Value = json!(body);
    // println!("{:#?}", json_body);
}

fn send_request_TCP_Stream() -> std::io::Result<()>
{
    let host: &str = "0.0.0.0:52525";

    let mut conn = TcpStream::connect(host)?;
    
    conn.write_all(format!("GET {} HTTP/1.1\r\n", host).as_bytes())?;
    conn.write_all(b"Host: www.rustinaction.com\r\n")?;
    conn.write_all(b"\r\n")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;
    Ok(())
}

pub fn test_all()
{
    send_request();
    // send_request_1();
    // send_request_get_response_text();
    // send_request_2();

    // send_request_TCP_Stream().expect("TODO: panic message");
}
