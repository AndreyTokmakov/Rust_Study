use rwf::{async_trait, http};
use rwf::controller::{Controller, Error};
use rwf::http::{Request, Response, Server};
use rwf::logging::Logger;
use rwf::macros::route;
use rwf::prelude::OffsetDateTime;

#[derive(Default)]
struct Index;

#[derive(Default)]
struct TimeController;

#[async_trait]
impl Controller for Index 
{
    async fn handle(&self, request: &Request) -> Result<Response, Error> {
        // Get the `Accept` header from the request.
        let accept = request
            .headers()
            .get("accept");

        if let Some(accept) = accept {
            Ok(Response::new().text(format!("Accept: {:?}", accept)))
        } else {
            Ok(Response::bad_request())
        }
    }
}

#[async_trait]
impl Controller for TimeController
{
    async fn handle(&self, request: &Request) -> Result<Response, Error> {
        let time: OffsetDateTime = OffsetDateTime::now_utc();

        // This creates an HTTP "200 OK" response, with "Content-Type: text/plain" header.
        let response: Response = Response::new().text(format!("The current time is: {:?}", time));
        Ok(response)
    }
}



#[tokio::main]
async fn start() -> Result<(), http::Error>
{
    // Configure the logger.
    Logger::init();

    // Define routes.
    let routes = vec![
        route!("/" => Index),
        route!("/time" => TimeController), 
    ];

    // Launch the HTTP server.
    Server::new(routes).launch().await
}

pub fn server_test()
{
    // http://localhost:8000/
    // http://localhost:8000/time

    start();
}
