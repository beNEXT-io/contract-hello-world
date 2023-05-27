use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use lib::org_accordproject_helloworld::*;

#[derive(Deserialize, Serialize, Debug)]
pub enum RequestType {
    MyRequest(MyRequest),
    HelloWorldClause(HelloWorldClause),
    // Add other request types here
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseType {
    MyResponse(MyResponse),
    HelloWorldClause(HelloWorldClause),
    // Add other response types here
}

#[derive(Deserialize, Serialize, Debug)]
struct Request {
    request: RequestType,
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    resonse: ResponseType,
}


async fn function_handler(event: LambdaEvent<Request>) -> Result<ResponseType, Error> {

    let response = match event.payload.request {
        RequestType::MyRequest(my_request) => {
            ResponseType::MyResponse(
                MyResponse {
                    _class: my_request._class,
                    output: format!("MyRequest - input = {}", my_request.input),
                    _timestamp: Utc::now()
                }       
            )
        },
        RequestType::HelloWorldClause(hello_world_clause) => {
            ResponseType::HelloWorldClause(
                HelloWorldClause {
                    _class: hello_world_clause._class,
                    clause_id: hello_world_clause.clause_id,
                    _identifier: hello_world_clause._identifier,
                    name: hello_world_clause.name
                }       
            )
        },
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
