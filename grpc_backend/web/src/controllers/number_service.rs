use crate::protos::number_service::number_service_server::NumberService;
use crate::protos::number_service::{NumberRequest, NumberResponse};
use tonic::async_trait;
use tonic::{Request, Response, Status};
use rand::prelude::*;

cali_derive::controller!(NumberServiceController);
#[async_trait]
impl NumberService for NumberServiceController {
    async fn get_number(
        &self,
        _request: Request<NumberRequest>,
    ) -> Result<Response<NumberResponse>, Status> {
        let mut rng = rand::thread_rng();
        let num: i32 = rng.gen();
        let response = Response::new(NumberResponse { number: num });
        return Ok(response);
    }
}
