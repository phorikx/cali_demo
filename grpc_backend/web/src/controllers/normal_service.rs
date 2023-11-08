use tonic::async_trait;
use tonic::{Status, Response, Request};
use crate::protos::normal_service::{};
use crate::protos::normal_service::normal_service_server::NormalService;


cali_derive::controller!(NormalServiceController);
#[async_trait]
impl NormalService for NormalServiceController {

}