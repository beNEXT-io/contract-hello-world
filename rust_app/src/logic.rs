use chrono::{DateTime, Utc};
use lib::org_accordproject_helloworld::{HelloWorldClause, MyRequest, MyResponse};
use serde_json::Error;

pub struct Contract(HelloWorldClause);

impl Contract {
    fn new<'a>(path: &'a str) -> Result<HelloWorldClause, Error> {
        let Ok(data) = std::fs::read_to_string(path) else {
            panic!("Unable to read HelloWorldClause from data.json")
        };
        let json: HelloWorldClause = serde_json::from_str(&data)?;
        Ok(json)
    }
}

pub struct MyResponseImpl(MyResponse);

impl MyResponseImpl {
    fn new(output: String, _timestamp: DateTime<Utc>) -> Result<MyResponse, Error> {
        let _class = "org.accordproject.helloworld.MyResponse".to_owned();
        Ok(MyResponse {
            _class,
            output,
            _timestamp,
        })
    }
}

pub struct MyRequestImpl(MyRequest);

impl MyRequestImpl {
    fn new(input: String, _timestamp: DateTime<Utc>) -> Result<MyRequest, Error> {
        let _class = "org.accordproject.helloworld.MyRequest".to_owned();
        Ok(MyRequest {
            _class,
            input,
            _timestamp,
        })
    }
}

pub fn logic(request: MyRequest) -> Result<MyResponse, Error> {
    let path = "/home/sk/contract-hello-world/rust_app/src/data.json";
    let contract = Contract::new(path)?;

    MyResponseImpl::new(
        format!("Hello {} {}", contract.name, request.input),
        request._timestamp,
    )
}
