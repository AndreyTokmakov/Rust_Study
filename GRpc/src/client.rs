use tonic::Request;

pub mod service {
    tonic::include_proto!("service");
}

use service::user_service_client::UserServiceClient;
use service::UserRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(UserRequest { id: 42 });
    let response = client.get_user(request).await?;
    println!("Response: {:?}", response.into_inner());

    Ok(())
}