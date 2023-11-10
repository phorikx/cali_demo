use tonic::async_trait;
use tonic::{Status, Response, Request};
use crate::protos::normal_service::{NormalResponse, NumbersRequest};
use crate::protos::normal_service::normal_service_server::NormalService;


cali_derive::controller!(NormalServiceController);
#[async_trait]
impl NormalService for NormalServiceController {
    async fn check_against_normal_distribution(&self, request: Request<tonic::Streaming<NumbersRequest>>) -> Result<Response<NormalResponse>, Status>{
        let normal_result = Response::new( NormalResponse {mean: 2, standard_deviation: 2});
        return Ok(normal_result);
    }
}
