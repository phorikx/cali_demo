use grpc_backend_web::config::Config;
use cali_derive::setup_server;
use std::{error::Error, str::FromStr};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ServerContext {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_context = Arc::new(ServerContext {});
    setup_server!("grpc_backend", "0.1.1", my_context);
    Ok(())
}
