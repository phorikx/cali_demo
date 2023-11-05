use cali_derive::setup_server;
use grpc_backend_web::config::Config;
use std::sync::Arc;
use std::{error::Error, str::FromStr};

#[derive(Debug, Clone)]
pub struct ServerContext {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_context = Arc::new(ServerContext {});
    setup_server!("grpc_backend", "0.1.1", my_context);
    Ok(())
}
