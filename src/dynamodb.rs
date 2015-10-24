//! DynamoDB bindings for Rust
#![allow(unused_variables, unused_mut)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use std::str::FromStr;
use regions::*;

// include the code generated from the DynamoDB botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/dynamodb/dynamodb.rs"));

pub struct DynamoDBHelper<'a> {
	client: DynamoDBClient<'a>
}

impl<'a> DynamoDBHelper<'a> {
	pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> DynamoDBHelper<'a> {
		DynamoDBHelper { client: DynamoDBClient::new(credentials, region) }
	}

}
