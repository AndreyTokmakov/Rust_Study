use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod service {
    tonic::include_proto!("service");
}

use service::user_service_server::{UserService, UserServiceServer};
use service::{UserRequest, UserResponse};

#[derive(Default)]
pub struct MyUserService;

#[tonic::async_trait]
impl UserService for MyUserService
{
    async fn get_user(&self, request: Request<UserRequest>, ) -> Result<Response<UserResponse>, Status>
    {
        let id: i32 = request.into_inner().id;
        println!("Request received: id={}", id);
        let reply = UserResponse {
            name: format!("User {}", id),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let addr: SocketAddr = "[::1]:50051".parse()?;
    println!("gRPC Server listening on {}", addr);
    Server::builder().add_service(UserServiceServer::new(MyUserService::default())).serve(addr).await?;
    Ok(())
}