use service_auth::{
    envoy::{
        r#type::v3::{HttpStatus, StatusCode},
        service::auth::v3::{
            authorization_server::Authorization, authorization_server::AuthorizationServer,
            check_response::HttpResponse, CheckRequest, CheckResponse, DeniedHttpResponse,
            OkHttpResponse,
        },
    },
    google::rpc::Status,
};
use tonic::{transport::Server, Request, Response};

pub mod service_auth {
    tonic::include_proto!("mod");
}

#[derive(Debug, Default)]
pub struct ExtAuth {}

impl ExtAuth {
    fn deny(&self) -> Result<Response<CheckResponse>, tonic::Status> {
        let resp = CheckResponse {
            status: Some(Status {
                code: StatusCode::Forbidden as i32,
                message: String::from("Forbidden"),
                details: vec![],
            }),
            dynamic_metadata: None,
            http_response: Some(HttpResponse::DeniedResponse(DeniedHttpResponse {
                status: Some(HttpStatus {
                    code: StatusCode::Forbidden as i32,
                }),
                headers: vec![],
                body: String::from("Forbidden access"),
            })),
        };

        Ok(Response::new(resp))
    }

    fn allow(&self) -> Result<Response<CheckResponse>, tonic::Status> {
        let resp = CheckResponse {
            status: Some(Status {
                code: StatusCode::Ok as i32,
                message: String::from("Ok"),
                details: vec![],
            }),
            dynamic_metadata: None,
            http_response: Some(HttpResponse::OkResponse(OkHttpResponse {
                headers: vec![],
                headers_to_remove: vec![],
                dynamic_metadata: None,
                response_headers_to_add: vec![],
                query_parameters_to_set: vec![],
                query_parameters_to_remove: vec![],
            })),
        };

        Ok(Response::new(resp))
    }
}

#[tonic::async_trait]
impl Authorization for ExtAuth {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, tonic::Status> {
        let token = request
            .into_inner()
            .attributes
            .and_then(|a| a.request)
            .and_then(|r| r.http)
            .map(|h| h.headers)
            .and_then(|headers| headers.get("token").cloned());

        if token == Some(String::from("secured")) {
            return self.allow();
        }

        return self.deny();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:5005".parse()?;
    let ext_auth = ExtAuth::default();

    println!("Server started at :5005");

    Server::builder()
        .add_service(AuthorizationServer::new(ext_auth))
        .serve(addr)
        .await?;

    Ok(())
}
