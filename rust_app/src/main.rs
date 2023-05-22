use lambda_runtime::Error;
use lambda_runtime::{run, service_fn, LambdaEvent};
use lib::org_accordproject_helloworld::MyRequest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub mod logic;
use logic::{logic, MyRequestImpl};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize, Serialize, Debug)]
struct Request {
    request: Value,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    response: String,
}

fn into_valid_response(response: Response) -> Result<Value, Error> {
    Ok(json!({
        "isBase64Encoded": false,
        "statusCode": 200,
        "headers": { },
        "body": response.response

    }))
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _) = event.into_parts();
    let request = Request {
        request: event["body"].clone(),
    };

    let input = request.request["input"].to_string();
    let _timestamp = request.request["$timestamp"].p

    let response = {
        let my_request: MyRequest = 
        let my_response = logic(my_request)?;
        let string_response = serde_json::to_string(&my_response)?;
        Response {
            response: string_response,
        }
    };

    into_valid_response(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
