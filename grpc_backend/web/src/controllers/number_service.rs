use crate::protos::number_service::number_service_server::NumberService;
use crate::protos::number_service::{NumberRequest, NumberResponse};
use tonic::async_trait;
use tonic::{Request, Response, Status};

cali_derive::controller!(NumberServiceController);
#[async_trait]
impl NumberService for NumberServiceController {
    async fn get_number(
        &self,
        _request: Request<NumberRequest>,
    ) -> Result<Response<NumberResponse>, Status> {
        let response = Response::new(NumberResponse { number: 5 });
        return Ok(response);
    }
}
