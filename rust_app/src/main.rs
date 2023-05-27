use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use lib::org_accordproject_helloworld::*;

#[derive(Deserialize, Serialize, Debug)]
struct Request {
    data: HelloWorldClause,
    request: MyRequest
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<MyResponse, Error> {

    let response = MyResponse {
        _class: event.payload.data._class.clone(),
        output: format!("Hello {}! {}", event.payload.data.name, event.payload.request.input),
        _timestamp: Utc::now(),
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
