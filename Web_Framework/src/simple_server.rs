
use rwf::prelude::*;
use rwf::http::{self, Server};

#[controller]
async fn index() -> Response
{
    Response::new().html("<body bgcolor='gray'><h1>My first Rwf app!</h1></body>")
}

#[derive(Default)]
struct CurrentTime;

#[async_trait]
impl Controller for CurrentTime
{
    /// This function handles incoming HTTP requests.
    async fn handle(&self, request: &Request) -> Result<Response, Error> {
        let time: OffsetDateTime = OffsetDateTime::now_utc();

        // This creates an HTTP "200 OK" response, with "Content-Type: text/plain" header.
        let response: Response = Response::new().text(format!("The current time is: {:?}", time));
        Ok(response)
    }
}

#[tokio::main]
async fn start_server() -> Result<(), http::Error>
{
    // Configure the logger.
    Logger::init();

    // Define routes.
    let routes = vec![
        route!("/" => index),
        route!("/time" => CurrentTime),         // Map the `/time` route to the `CurrentTime` controller.
    ];

    // Launch the HTTP server.
    Server::new(routes).launch().await
}

pub fn server_test()
{
    // http://localhost:8000/
    // http://localhost:8000/time
    
    let _ = start_server();
}
