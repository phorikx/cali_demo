use tonic::async_trait;
use tonic::{Status, Response, Request};
use crate::protos::number_service::{NumberRequest, NumberResponse};
use crate::protos::number_service::number_service_server::NumberService;


cali_derive::controller!(NumberServiceController);
#[async_trait]
impl NumberService for NumberServiceController {


async fn get_number(
        &self,
        request: Request<NumberRequest>,
    ) -> Result<Response<NumberResponse>, Status> {
        todo!()
    }

}