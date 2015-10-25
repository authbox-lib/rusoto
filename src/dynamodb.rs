//! DynamoDB bindings for Rust
#![allow(unused_variables, unused_mut, non_snake_case)]
use credentials::*;
use signature::*;
use error::*;
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

	pub fn list_tables(&mut self) -> Result<ListTablesOutput, AWSError> {
		let mut req = ListTablesInput::default();
		self.client.list_tables(&req)
	}

}
