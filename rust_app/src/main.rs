use aws_sdk_dynamodb::{model::AttributeValue, Client};
use chrono::Utc;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::env;

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

fn handle_my_request(my_request: MyRequest) -> MyResponse {
    MyResponse {
        _class: my_request._class,
        output: format!("MyRequest - input = {}", my_request.input),
        _timestamp: Utc::now(),
    }
}

async fn handle_hello_world_clause(
    hello_world_clause: HelloWorldClause,
) -> Result<
    HelloWorldClause,
    aws_sdk_dynamodb::types::SdkError<aws_sdk_dynamodb::error::PutItemError>,
> {
    // Initialize the AWS SDK for Rust
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);

    let result = dynamodb_client
        .put_item()
        .table_name(table_name)
        .item("id", AttributeValue::S("data".to_string()))
        .item(
            "_identifier",
            AttributeValue::S(hello_world_clause._identifier.clone()),
        )
        .item(
            "clause_id",
            AttributeValue::S(hello_world_clause.clause_id.clone()),
        )
        .item(
            "_class",
            AttributeValue::S(hello_world_clause._class.clone()),
        )
        .item("name", AttributeValue::S(hello_world_clause.name.clone()))
        .send()
        .await;

    match result {
        Ok(_) => {
            println!("Successfully saved HelloWorldClause to DynamoDB: _class: {}, clause_id: {}, _identifier: {}, name: {}", hello_world_clause._class, hello_world_clause.clause_id, hello_world_clause._identifier, hello_world_clause.name);
            Ok(HelloWorldClause {
                _class: hello_world_clause._class,
                clause_id: hello_world_clause.clause_id,
                _identifier: hello_world_clause._identifier,
                name: hello_world_clause.name,
            })
        }
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error)
        }
    }
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<ResponseType, Error> {
    let response = match event.payload.request {
        RequestType::MyRequest(my_request) => {
            ResponseType::MyResponse(handle_my_request(my_request))
        }
        RequestType::HelloWorldClause(hello_world_clause) => {
            match handle_hello_world_clause(hello_world_clause).await {
                Ok(hello_world_clause) => ResponseType::HelloWorldClause(hello_world_clause),
                Err(error) => {
                    return Err(lambda_runtime::Error::from(format!("Error: {:?}", error)))
                }
            }
        }
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
