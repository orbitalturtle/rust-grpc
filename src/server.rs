use tonic::{transport::Server, Request, Response, Status};
use chrono::prelude::*;
use console::Style;

use tower_service::tower_service_server::{TowerService, TowerServiceServer};
use tower_service::{TowerRequest, TowerResponse};

pub mod tower_service {
    tonic::include_proto!("tower_service");
}

#[derive(Debug, Default)]
pub struct MyTower {}

#[tonic::async_trait]
impl TowerService for MyTower {
    async fn get_tower(
        &self,
        request: Request<TowerRequest>,
    ) -> Result<Response<TowerResponse>, Status> {
        println!("Got a request for a tower: {:?}", request);

        let response = tower_service::TowerResponse {
            id: format!("Hello {}!", request.into_inner().id).into(),
	    date_created: Local::now().to_rfc2822(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let tower = MyTower::default();

    let blue = Style::new()
        .blue();   

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    Server::builder()
        .add_service(TowerServiceServer::new(tower))
        .serve(addr)
        .await?;

    Ok(())
}
