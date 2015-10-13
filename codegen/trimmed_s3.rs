use std::collections::HashMap;
use std::str;
use std::error::Error;
//typeparser

/// Parse LifecycleExpiration from response
struct LifecycleExpirationParser;
impl LifecycleExpirationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<LifecycleExpiration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = LifecycleExpiration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Date is payload.
			if current_name == "Date" {
				obj.date = try!(DateParser::parse_xml("Date", stack));
				continue;
			}
			// heylisten etc: location for Days is payload.
			if current_name == "Days" {
				obj.days = try!(DaysParser::parse_xml("Days", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct LifecycleExpiration {
	/// Indicates at what date the object is to be moved or deleted. Should be in GMT
	/// ISO 8601 Format.
	pub date: Date,
	/// Indicates the lifetime, in days, of the objects that are subject to the rule.
	/// The value must be a non-zero positive integer.
	pub days: Days,
}

/// Write LifecycleExpiration contents to a SignedRequest via headers
struct LifecycleExpirationWriter;
impl LifecycleExpirationWriter {
	fn write_params(request: &mut request, name: &str, obj: &LifecycleExpiration) {
		DateWriter::write_params(params, &(prefix.to_string() + "Date"), &obj.date);
		DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
	}
}
//typeparser

/// Parse Errors from response
struct ErrorsParser;
impl ErrorsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Errors, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Error" {
			obj.push(try!(ErrorParser::parse_xml("Error", stack)));
		}
		Ok(obj)
	}
}
pub type Errors = Vec<Error>;
/// Write Errors contents to a SignedRequest via headers
struct ErrorsWriter;
impl ErrorsWriter {
	fn write_params(request: &mut request, name: &str, obj: &Errors) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ErrorWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse RequestPayer from response
struct RequestPayerParser;
impl RequestPayerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<RequestPayer, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Confirms that the requester knows that she or he will be charged for the
/// request. Bucket owners need not specify this parameter in their requests.
/// Documentation on downloading objects from requester pays buckets can be found
/// at http://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBucket
/// s.html
pub type RequestPayer = String;
/// Write RequestPayer contents to a SignedRequest via headers
struct RequestPayerWriter;
impl RequestPayerWriter {
	fn write_params(request: &mut request, name: &str, obj: &RequestPayer) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CopySourceVersionId from response
struct CopySourceVersionIdParser;
impl CopySourceVersionIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceVersionId, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceVersionId = String;
/// Write CopySourceVersionId contents to a SignedRequest via headers
struct CopySourceVersionIdWriter;
impl CopySourceVersionIdWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceVersionId) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Destination from response
struct DestinationParser;
impl DestinationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Destination, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Destination::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Bucket is payload.
			if current_name == "Bucket" {
				obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Destination {
	/// Amazon resource name (ARN) of the bucket where you want Amazon S3 to store
	/// replicas of the object identified by the rule.
	pub bucket: BucketName,
}

/// Write Destination contents to a SignedRequest via headers
struct DestinationWriter;
impl DestinationWriter {
	fn write_params(request: &mut request, name: &str, obj: &Destination) {
		BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
	}
}
//typeparser

/// Parse IfNoneMatch from response
struct IfNoneMatchParser;
impl IfNoneMatchParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<IfNoneMatch, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type IfNoneMatch = String;
/// Write IfNoneMatch contents to a SignedRequest via headers
struct IfNoneMatchWriter;
impl IfNoneMatchWriter {
	fn write_params(request: &mut request, name: &str, obj: &IfNoneMatch) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse NoSuchBucket from response
struct NoSuchBucketParser;
impl NoSuchBucketParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NoSuchBucket, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = NoSuchBucket::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// The specified bucket does not exist.
#[derive(Debug, Default)]
pub struct NoSuchBucket;

/// Write NoSuchBucket contents to a SignedRequest via headers
struct NoSuchBucketWriter;
impl NoSuchBucketWriter {
	fn write_params(request: &mut request, name: &str, obj: &NoSuchBucket) {
	}
}
//typeparser

/// Parse ObjectVersionStorageClass from response
struct ObjectVersionStorageClassParser;
impl ObjectVersionStorageClassParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectVersionStorageClass, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ObjectVersionStorageClass = String;
/// Write ObjectVersionStorageClass contents to a SignedRequest via headers
struct ObjectVersionStorageClassWriter;
impl ObjectVersionStorageClassWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectVersionStorageClass) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse MultipartUploadId from response
struct MultipartUploadIdParser;
impl MultipartUploadIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MultipartUploadId, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MultipartUploadId = String;
/// Write MultipartUploadId contents to a SignedRequest via headers
struct MultipartUploadIdWriter;
impl MultipartUploadIdWriter {
	fn write_params(request: &mut request, name: &str, obj: &MultipartUploadId) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Role from response
struct RoleParser;
impl RoleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Role, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Role = String;
/// Write Role contents to a SignedRequest via headers
struct RoleWriter;
impl RoleWriter {
	fn write_params(request: &mut request, name: &str, obj: &Role) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse BucketVersioningStatus from response
struct BucketVersioningStatusParser;
impl BucketVersioningStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<BucketVersioningStatus, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type BucketVersioningStatus = String;
/// Write BucketVersioningStatus contents to a SignedRequest via headers
struct BucketVersioningStatusWriter;
impl BucketVersioningStatusWriter {
	fn write_params(request: &mut request, name: &str, obj: &BucketVersioningStatus) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse SSEKMSKeyId from response
struct SSEKMSKeyIdParser;
impl SSEKMSKeyIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<SSEKMSKeyId, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type SSEKMSKeyId = String;
/// Write SSEKMSKeyId contents to a SignedRequest via headers
struct SSEKMSKeyIdWriter;
impl SSEKMSKeyIdWriter {
	fn write_params(request: &mut request, name: &str, obj: &SSEKMSKeyId) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse NoncurrentVersionTransition from response
struct NoncurrentVersionTransitionParser;
impl NoncurrentVersionTransitionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NoncurrentVersionTransition, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = NoncurrentVersionTransition::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for NoncurrentDays is payload.
			if current_name == "NoncurrentDays" {
				obj.noncurrent_days = try!(DaysParser::parse_xml("NoncurrentDays", stack));
				continue;
			}
			// heylisten etc: location for StorageClass is payload.
			if current_name == "StorageClass" {
				obj.storage_class = try!(TransitionStorageClassParser::parse_xml("StorageClass", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Container for the transition rule that describes when noncurrent objects
/// transition to the GLACIER storage class. If your bucket is versioning-enabled
/// (or versioning is suspended), you can set this action to request that Amazon
/// S3 transition noncurrent object versions to the GLACIER storage class at a
/// specific period in the object's lifetime.
#[derive(Debug, Default)]
pub struct NoncurrentVersionTransition {
	/// Specifies the number of days an object is noncurrent before Amazon S3 can
	/// perform the associated action. For information about the noncurrent days
	/// calculations, see [How Amazon S3 Calculates When an Object Became
	/// Noncurrent](/AmazonS3/latest/dev/s3-access-control.html) in the Amazon Simple
	/// Storage Service Developer Guide.
	pub noncurrent_days: Days,
	/// The class of storage used to store the object.
	pub storage_class: TransitionStorageClass,
}

/// Write NoncurrentVersionTransition contents to a SignedRequest via headers
struct NoncurrentVersionTransitionWriter;
impl NoncurrentVersionTransitionWriter {
	fn write_params(request: &mut request, name: &str, obj: &NoncurrentVersionTransition) {
		DaysWriter::write_params(params, &(prefix.to_string() + "NoncurrentDays"), &obj.noncurrent_days);
		TransitionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
	}
}
//typeparser

/// Parse ObjectStorageClass from response
struct ObjectStorageClassParser;
impl ObjectStorageClassParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectStorageClass, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ObjectStorageClass = String;
/// Write ObjectStorageClass contents to a SignedRequest via headers
struct ObjectStorageClassWriter;
impl ObjectStorageClassWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectStorageClass) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Metadata from response
struct MetadataParser;
impl MetadataParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Metadata, XmlParseError> {
		 // map_parser
		let mut obj = HashMap::new();
		while try!(peek_at_name(stack)) == tag_name {
			try!(start_element(tag_name, stack));
			let key = try!(MetadataKeyParser::parse_xml("MetadataKey", stack));
			let value = try!(MetadataValueParser::parse_xml("MetadataValue", stack));
			obj.insert(key, value);
			try!(end_element(tag_name, stack));
		}
		Ok(obj)
	}
}
pub type Metadata = HashMap<MetadataKey,MetadataValue>;
/// Write Metadata contents to a SignedRequest via headers
struct MetadataWriter;
impl MetadataWriter {
	fn write_params(request: &mut request, name: &str, obj: &Metadata) {
		let mut index = 1;
		for (key,value) in obj {
			let prefix = &format!("{}.{}", name, index);
			MetadataKeyWriter::write_params(params, &format!("{}.{}", prefix, "MetadataKey"), &key);
			MetadataValueWriter::write_params(params, &format!("{}.{}", prefix, "MetadataValue"), &value);
			index += 1;
		}
	}
}
//typeparser

/// Parse Body from response
struct BodyParser;
impl BodyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Body, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack)).into_bytes();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Body = Vec<u8>;
/// Write Body contents to a SignedRequest via headers
struct BodyWriter;
impl BodyWriter {
	fn write_params(request: &mut request, name: &str, obj: &Body) {
		request.add_header(name, str::from_utf8(&obj).unwrap());
	}
}
//typeparser

/// Parse SSECustomerKey from response
struct SSECustomerKeyParser;
impl SSECustomerKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<SSECustomerKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type SSECustomerKey = String;
/// Write SSECustomerKey contents to a SignedRequest via headers
struct SSECustomerKeyWriter;
impl SSECustomerKeyWriter {
	fn write_params(request: &mut request, name: &str, obj: &SSECustomerKey) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Rules from response
struct RulesParser;
impl RulesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Rules, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Rule" {
			obj.push(try!(RuleParser::parse_xml("Rule", stack)));
		}
		Ok(obj)
	}
}
pub type Rules = Vec<Rule>;
/// Write Rules contents to a SignedRequest via headers
struct RulesWriter;
impl RulesWriter {
	fn write_params(request: &mut request, name: &str, obj: &Rules) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			RuleWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse RedirectAllRequestsTo from response
struct RedirectAllRequestsToParser;
impl RedirectAllRequestsToParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<RedirectAllRequestsTo, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = RedirectAllRequestsTo::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for HostName is payload.
			if current_name == "HostName" {
				obj.host_name = try!(HostNameParser::parse_xml("HostName", stack));
				continue;
			}
			// heylisten etc: location for Protocol is payload.
			if current_name == "Protocol" {
				obj.protocol = Some(try!(ProtocolParser::parse_xml("Protocol", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct RedirectAllRequestsTo {
	/// Name of the host where requests will be redirected.
	pub host_name: HostName,
	/// Protocol to use (http, https) when redirecting requests. The default is the
	/// protocol that is used in the original request.
	pub protocol: Option<Protocol>,
}

/// Write RedirectAllRequestsTo contents to a SignedRequest via headers
struct RedirectAllRequestsToWriter;
impl RedirectAllRequestsToWriter {
	fn write_params(request: &mut request, name: &str, obj: &RedirectAllRequestsTo) {
		HostNameWriter::write_params(params, &(prefix.to_string() + "HostName"), &obj.host_name);
		if let Some(ref obj) = obj.protocol {
			 request.add_header("Protocol", obj);
		}
	}
}
//typeparser

/// Parse Object from response
struct ObjectParser;
impl ObjectParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Object, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Object::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for LastModified is payload.
			if current_name == "LastModified" {
				obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
				continue;
			}
			// heylisten etc: location for ETag is payload.
			if current_name == "ETag" {
				obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
				continue;
			}
			// heylisten etc: location for StorageClass is payload.
			if current_name == "StorageClass" {
				obj.storage_class = try!(ObjectStorageClassParser::parse_xml("StorageClass", stack));
				continue;
			}
			// heylisten etc: location for Key is payload.
			if current_name == "Key" {
				obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
				continue;
			}
			// heylisten etc: location for Owner is payload.
			if current_name == "Owner" {
				obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
				continue;
			}
			// heylisten etc: location for Size is payload.
			if current_name == "Size" {
				obj.size = try!(SizeParser::parse_xml("Size", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Object {
	pub last_modified: LastModified,
	pub e_tag: ETag,
	/// The class of storage used to store the object.
	pub storage_class: ObjectStorageClass,
	pub key: ObjectKey,
	pub owner: Owner,
	pub size: Size,
}

/// Write Object contents to a SignedRequest via headers
struct ObjectWriter;
impl ObjectWriter {
	fn write_params(request: &mut request, name: &str, obj: &Object) {
		LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
		ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
		ObjectStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
		OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
		SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
	}
}
//typeparser

/// Parse DeleteMarkerEntry from response
struct DeleteMarkerEntryParser;
impl DeleteMarkerEntryParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<DeleteMarkerEntry, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = DeleteMarkerEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Owner is payload.
			if current_name == "Owner" {
				obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
				continue;
			}
			// heylisten etc: location for IsLatest is payload.
			if current_name == "IsLatest" {
				obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
				continue;
			}
			// heylisten etc: location for VersionId is payload.
			if current_name == "VersionId" {
				obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
				continue;
			}
			// heylisten etc: location for Key is payload.
			if current_name == "Key" {
				obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
				continue;
			}
			// heylisten etc: location for LastModified is payload.
			if current_name == "LastModified" {
				obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct DeleteMarkerEntry {
	pub owner: Owner,
	/// Specifies whether the object is (true) or is not (false) the latest version of
	/// an object.
	pub is_latest: IsLatest,
	/// Version ID of an object.
	pub version_id: ObjectVersionId,
	/// The object key.
	pub key: ObjectKey,
	/// Date and time the object was last modified.
	pub last_modified: LastModified,
}

/// Write DeleteMarkerEntry contents to a SignedRequest via headers
struct DeleteMarkerEntryWriter;
impl DeleteMarkerEntryWriter {
	fn write_params(request: &mut request, name: &str, obj: &DeleteMarkerEntry) {
		OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
		IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
		ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
		LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
	}
}
//typeparser

/// Parse TargetBucket from response
struct TargetBucketParser;
impl TargetBucketParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TargetBucket, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type TargetBucket = String;
/// Write TargetBucket contents to a SignedRequest via headers
struct TargetBucketWriter;
impl TargetBucketWriter {
	fn write_params(request: &mut request, name: &str, obj: &TargetBucket) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse MFADeleteStatus from response
struct MFADeleteStatusParser;
impl MFADeleteStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MFADeleteStatus, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MFADeleteStatus = String;
/// Write MFADeleteStatus contents to a SignedRequest via headers
struct MFADeleteStatusWriter;
impl MFADeleteStatusWriter {
	fn write_params(request: &mut request, name: &str, obj: &MFADeleteStatus) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse MaxKeys from response
struct MaxKeysParser;
impl MaxKeysParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MaxKeys, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MaxKeys = i32;
/// Write MaxKeys contents to a SignedRequest via headers
struct MaxKeysWriter;
impl MaxKeysWriter {
	fn write_params(request: &mut request, name: &str, obj: &MaxKeys) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse Part from response
struct PartParser;
impl PartParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Part, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Part::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for LastModified is payload.
			if current_name == "LastModified" {
				obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
				continue;
			}
			// heylisten etc: location for PartNumber is payload.
			if current_name == "PartNumber" {
				obj.part_number = try!(PartNumberParser::parse_xml("PartNumber", stack));
				continue;
			}
			// heylisten etc: location for ETag is payload.
			if current_name == "ETag" {
				obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
				continue;
			}
			// heylisten etc: location for Size is payload.
			if current_name == "Size" {
				obj.size = try!(SizeParser::parse_xml("Size", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Part {
	/// Date and time at which the part was uploaded.
	pub last_modified: LastModified,
	/// Part number identifying the part. This is a positive integer between 1 and
	/// 10,000.
	pub part_number: PartNumber,
	/// Entity tag returned when the part was uploaded.
	pub e_tag: ETag,
	/// Size of the uploaded part data.
	pub size: Size,
}

/// Write Part contents to a SignedRequest via headers
struct PartWriter;
impl PartWriter {
	fn write_params(request: &mut request, name: &str, obj: &Part) {
		LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
		PartNumberWriter::write_params(params, &(prefix.to_string() + "PartNumber"), &obj.part_number);
		ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
		SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
	}
}
#[derive(Debug, Default)]
pub struct GetBucketPolicyRequest {
	pub bucket: BucketName,
}

/// Write GetBucketPolicyRequest contents to a SignedRequest via headers
struct GetBucketPolicyRequestWriter;
impl GetBucketPolicyRequestWriter {
	fn write_params(request: &mut request, name: &str, obj: &GetBucketPolicyRequest) {
		BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
	}
}
//typeparser

/// Parse CloudFunction from response
struct CloudFunctionParser;
impl CloudFunctionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CloudFunction, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CloudFunction = String;
/// Write CloudFunction contents to a SignedRequest via headers
struct CloudFunctionWriter;
impl CloudFunctionWriter {
	fn write_params(request: &mut request, name: &str, obj: &CloudFunction) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse IndexDocument from response
struct IndexDocumentParser;
impl IndexDocumentParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<IndexDocument, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = IndexDocument::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Suffix is payload.
			if current_name == "Suffix" {
				obj.suffix = try!(SuffixParser::parse_xml("Suffix", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct IndexDocument {
	/// A suffix that is appended to a request that is for a directory on the website
	/// endpoint (e.g. if the suffix is index.html and you make a request to
	/// samplebucket/images/ the data that is returned will be for the object with the
	/// key name images/index.html) The suffix must not be empty and must not include
	/// a slash character.
	pub suffix: Suffix,
}

/// Write IndexDocument contents to a SignedRequest via headers
struct IndexDocumentWriter;
impl IndexDocumentWriter {
	fn write_params(request: &mut request, name: &str, obj: &IndexDocument) {
		SuffixWriter::write_params(params, &(prefix.to_string() + "Suffix"), &obj.suffix);
	}
}
//typeparser

/// Parse ReplaceKeyPrefixWith from response
struct ReplaceKeyPrefixWithParser;
impl ReplaceKeyPrefixWithParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ReplaceKeyPrefixWith, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ReplaceKeyPrefixWith = String;
/// Write ReplaceKeyPrefixWith contents to a SignedRequest via headers
struct ReplaceKeyPrefixWithWriter;
impl ReplaceKeyPrefixWithWriter {
	fn write_params(request: &mut request, name: &str, obj: &ReplaceKeyPrefixWith) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse TopicConfiguration from response
struct TopicConfigurationParser;
impl TopicConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TopicConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = TopicConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Id is payload.
			if current_name == "Id" {
				obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
				continue;
			}
			// heylisten etc: location for TopicArn is payload.
			if current_name == "Topic" {
				obj.topic_arn = try!(TopicArnParser::parse_xml("Topic", stack));
				continue;
			}
			// heylisten etc: location for Events is payload.
			if current_name == "Event" {
				obj.events = try!(EventListParser::parse_xml("Event", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Container for specifying the configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Notification Service (Amazon SNS) topic.
#[derive(Debug, Default)]
pub struct TopicConfiguration {
	pub id: Option<NotificationId>,
	/// Amazon SNS topic ARN to which Amazon S3 will publish a message when it detects
	/// events of specified type.
	pub topic_arn: TopicArn,
	pub events: EventList,
}

/// Write TopicConfiguration contents to a SignedRequest via headers
struct TopicConfigurationWriter;
impl TopicConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &TopicConfiguration) {
		if let Some(ref obj) = obj.id {
			 request.add_header("Id", obj);
		}
		TopicArnWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topic_arn);
		EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
	}
}
//typeparser

/// Parse CopySourceSSECustomerAlgorithm from response
struct CopySourceSSECustomerAlgorithmParser;
impl CopySourceSSECustomerAlgorithmParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceSSECustomerAlgorithm, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceSSECustomerAlgorithm = String;
/// Write CopySourceSSECustomerAlgorithm contents to a SignedRequest via headers
struct CopySourceSSECustomerAlgorithmWriter;
impl CopySourceSSECustomerAlgorithmWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceSSECustomerAlgorithm) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse TopicConfigurationList from response
struct TopicConfigurationListParser;
impl TopicConfigurationListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TopicConfigurationList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "TopicConfiguration" {
			obj.push(try!(TopicConfigurationParser::parse_xml("TopicConfiguration", stack)));
		}
		Ok(obj)
	}
}
pub type TopicConfigurationList = Vec<TopicConfiguration>;
/// Write TopicConfigurationList contents to a SignedRequest via headers
struct TopicConfigurationListWriter;
impl TopicConfigurationListWriter {
	fn write_params(request: &mut request, name: &str, obj: &TopicConfigurationList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			TopicConfigurationWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse Days from response
struct DaysParser;
impl DaysParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Days, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Days = i32;
/// Write Days contents to a SignedRequest via headers
struct DaysWriter;
impl DaysWriter {
	fn write_params(request: &mut request, name: &str, obj: &Days) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse Value from response
struct ValueParser;
impl ValueParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Value, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Value = String;
/// Write Value contents to a SignedRequest via headers
struct ValueWriter;
impl ValueWriter {
	fn write_params(request: &mut request, name: &str, obj: &Value) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Tag from response
struct TagParser;
impl TagParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Tag, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Tag::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Value is payload.
			if current_name == "Value" {
				obj.value = try!(ValueParser::parse_xml("Value", stack));
				continue;
			}
			// heylisten etc: location for Key is payload.
			if current_name == "Key" {
				obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Tag {
	/// Value of the tag.
	pub value: Value,
	/// Name of the tag.
	pub key: ObjectKey,
}

/// Write Tag contents to a SignedRequest via headers
struct TagWriter;
impl TagWriter {
	fn write_params(request: &mut request, name: &str, obj: &Tag) {
		ValueWriter::write_params(params, &(prefix.to_string() + "Value"), &obj.value);
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
	}
}
//typeparser

/// Parse KeyMarker from response
struct KeyMarkerParser;
impl KeyMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<KeyMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type KeyMarker = String;
/// Write KeyMarker contents to a SignedRequest via headers
struct KeyMarkerWriter;
impl KeyMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &KeyMarker) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse DeleteMarkers from response
struct DeleteMarkersParser;
impl DeleteMarkersParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<DeleteMarkers, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "DeleteMarkerEntry" {
			obj.push(try!(DeleteMarkerEntryParser::parse_xml("DeleteMarkerEntry", stack)));
		}
		Ok(obj)
	}
}
pub type DeleteMarkers = Vec<DeleteMarkerEntry>;
/// Write DeleteMarkers contents to a SignedRequest via headers
struct DeleteMarkersWriter;
impl DeleteMarkersWriter {
	fn write_params(request: &mut request, name: &str, obj: &DeleteMarkers) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			DeleteMarkerEntryWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse ResponseContentDisposition from response
struct ResponseContentDispositionParser;
impl ResponseContentDispositionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ResponseContentDisposition, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ResponseContentDisposition = String;
/// Write ResponseContentDisposition contents to a SignedRequest via headers
struct ResponseContentDispositionWriter;
impl ResponseContentDispositionWriter {
	fn write_params(request: &mut request, name: &str, obj: &ResponseContentDisposition) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse GrantFullControl from response
struct GrantFullControlParser;
impl GrantFullControlParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<GrantFullControl, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type GrantFullControl = String;
/// Write GrantFullControl contents to a SignedRequest via headers
struct GrantFullControlWriter;
impl GrantFullControlWriter {
	fn write_params(request: &mut request, name: &str, obj: &GrantFullControl) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse SSECustomerAlgorithm from response
struct SSECustomerAlgorithmParser;
impl SSECustomerAlgorithmParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<SSECustomerAlgorithm, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type SSECustomerAlgorithm = String;
/// Write SSECustomerAlgorithm contents to a SignedRequest via headers
struct SSECustomerAlgorithmWriter;
impl SSECustomerAlgorithmWriter {
	fn write_params(request: &mut request, name: &str, obj: &SSECustomerAlgorithm) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CommonPrefixList from response
struct CommonPrefixListParser;
impl CommonPrefixListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CommonPrefixList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "CommonPrefix" {
			obj.push(try!(CommonPrefixParser::parse_xml("CommonPrefix", stack)));
		}
		Ok(obj)
	}
}
pub type CommonPrefixList = Vec<CommonPrefix>;
/// Write CommonPrefixList contents to a SignedRequest via headers
struct CommonPrefixListWriter;
impl CommonPrefixListWriter {
	fn write_params(request: &mut request, name: &str, obj: &CommonPrefixList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			CommonPrefixWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse Protocol from response
struct ProtocolParser;
impl ProtocolParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Protocol, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Protocol = String;
/// Write Protocol contents to a SignedRequest via headers
struct ProtocolWriter;
impl ProtocolWriter {
	fn write_params(request: &mut request, name: &str, obj: &Protocol) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Suffix from response
struct SuffixParser;
impl SuffixParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Suffix, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Suffix = String;
/// Write Suffix contents to a SignedRequest via headers
struct SuffixWriter;
impl SuffixWriter {
	fn write_params(request: &mut request, name: &str, obj: &Suffix) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse AllowedMethod from response
struct AllowedMethodParser;
impl AllowedMethodParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<AllowedMethod, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type AllowedMethod = String;
/// Write AllowedMethod contents to a SignedRequest via headers
struct AllowedMethodWriter;
impl AllowedMethodWriter {
	fn write_params(request: &mut request, name: &str, obj: &AllowedMethod) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse QueueConfiguration from response
struct QueueConfigurationParser;
impl QueueConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<QueueConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = QueueConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Id is payload.
			if current_name == "Id" {
				obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
				continue;
			}
			// heylisten etc: location for QueueArn is payload.
			if current_name == "Queue" {
				obj.queue_arn = try!(QueueArnParser::parse_xml("Queue", stack));
				continue;
			}
			// heylisten etc: location for Events is payload.
			if current_name == "Event" {
				obj.events = try!(EventListParser::parse_xml("Event", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Container for specifying an configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Queue Service (Amazon SQS) queue.
#[derive(Debug, Default)]
pub struct QueueConfiguration {
	pub id: Option<NotificationId>,
	/// Amazon SQS queue ARN to which Amazon S3 will publish a message when it detects
	/// events of specified type.
	pub queue_arn: QueueArn,
	pub events: EventList,
}

/// Write QueueConfiguration contents to a SignedRequest via headers
struct QueueConfigurationWriter;
impl QueueConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &QueueConfiguration) {
		if let Some(ref obj) = obj.id {
			 request.add_header("Id", obj);
		}
		QueueArnWriter::write_params(params, &(prefix.to_string() + "Queue"), &obj.queue_arn);
		EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
	}
}
//typeparser

/// Parse ObjectVersionId from response
struct ObjectVersionIdParser;
impl ObjectVersionIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectVersionId, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ObjectVersionId = String;
/// Write ObjectVersionId contents to a SignedRequest via headers
struct ObjectVersionIdWriter;
impl ObjectVersionIdWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectVersionId) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse TopicConfigurationDeprecated from response
struct TopicConfigurationDeprecatedParser;
impl TopicConfigurationDeprecatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TopicConfigurationDeprecated, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = TopicConfigurationDeprecated::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Topic is payload.
			if current_name == "Topic" {
				obj.topic = try!(TopicArnParser::parse_xml("Topic", stack));
				continue;
			}
			// heylisten etc: location for Id is payload.
			if current_name == "Id" {
				obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
				continue;
			}
			// heylisten etc: location for Event is payload.
			if current_name == "Event" {
				obj.event = try!(EventParser::parse_xml("Event", stack));
				continue;
			}
			// heylisten etc: location for Events is payload.
			if current_name == "Event" {
				obj.events = try!(EventListParser::parse_xml("Event", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct TopicConfigurationDeprecated {
	/// Amazon SNS topic to which Amazon S3 will publish a message to report the
	/// specified events for the bucket.
	pub topic: TopicArn,
	pub id: NotificationId,
	/// Bucket event for which to send notifications.
	pub event: Event,
	pub events: EventList,
}

/// Write TopicConfigurationDeprecated contents to a SignedRequest via headers
struct TopicConfigurationDeprecatedWriter;
impl TopicConfigurationDeprecatedWriter {
	fn write_params(request: &mut request, name: &str, obj: &TopicConfigurationDeprecated) {
		TopicArnWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topic);
		NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
		EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
		EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
	}
}
//typeparser

/// Parse ExposeHeaders from response
struct ExposeHeadersParser;
impl ExposeHeadersParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ExposeHeaders, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ExposeHeader" {
			obj.push(try!(ExposeHeaderParser::parse_xml("ExposeHeader", stack)));
		}
		Ok(obj)
	}
}
pub type ExposeHeaders = Vec<ExposeHeader>;
/// Write ExposeHeaders contents to a SignedRequest via headers
struct ExposeHeadersWriter;
impl ExposeHeadersWriter {
	fn write_params(request: &mut request, name: &str, obj: &ExposeHeaders) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ExposeHeaderWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse Grants from response
struct GrantsParser;
impl GrantsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Grants, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Grant" {
			obj.push(try!(GrantParser::parse_xml("Grant", stack)));
		}
		Ok(obj)
	}
}
pub type Grants = Vec<Grant>;
/// Write Grants contents to a SignedRequest via headers
struct GrantsWriter;
impl GrantsWriter {
	fn write_params(request: &mut request, name: &str, obj: &Grants) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			GrantWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse BucketName from response
struct BucketNameParser;
impl BucketNameParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<BucketName, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type BucketName = String;
/// Write BucketName contents to a SignedRequest via headers
struct BucketNameWriter;
impl BucketNameWriter {
	fn write_params(request: &mut request, name: &str, obj: &BucketName) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse BucketLocationConstraint from response
struct BucketLocationConstraintParser;
impl BucketLocationConstraintParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<BucketLocationConstraint, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type BucketLocationConstraint = String;
/// Write BucketLocationConstraint contents to a SignedRequest via headers
struct BucketLocationConstraintWriter;
impl BucketLocationConstraintWriter {
	fn write_params(request: &mut request, name: &str, obj: &BucketLocationConstraint) {
		request.add_header(name, obj);
	}
}
#[derive(Debug, Default)]
pub struct GetBucketNotificationConfigurationRequest {
	/// Name of the buket to get the notification configuration for.
	pub bucket: BucketName,
}

/// Write GetBucketNotificationConfigurationRequest contents to a SignedRequest via headers
struct GetBucketNotificationConfigurationRequestWriter;
impl GetBucketNotificationConfigurationRequestWriter {
	fn write_params(request: &mut request, name: &str, obj: &GetBucketNotificationConfigurationRequest) {
		BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
	}
}
//typeparser

/// Parse AllowedMethods from response
struct AllowedMethodsParser;
impl AllowedMethodsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<AllowedMethods, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AllowedMethod" {
			obj.push(try!(AllowedMethodParser::parse_xml("AllowedMethod", stack)));
		}
		Ok(obj)
	}
}
pub type AllowedMethods = Vec<AllowedMethod>;
/// Write AllowedMethods contents to a SignedRequest via headers
struct AllowedMethodsWriter;
impl AllowedMethodsWriter {
	fn write_params(request: &mut request, name: &str, obj: &AllowedMethods) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AllowedMethodWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse Event from response
struct EventParser;
impl EventParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Event, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Bucket event for which to send notifications.
pub type Event = String;
/// Write Event contents to a SignedRequest via headers
struct EventWriter;
impl EventWriter {
	fn write_params(request: &mut request, name: &str, obj: &Event) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse VersionIdMarker from response
struct VersionIdMarkerParser;
impl VersionIdMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<VersionIdMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type VersionIdMarker = String;
/// Write VersionIdMarker contents to a SignedRequest via headers
struct VersionIdMarkerWriter;
impl VersionIdMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &VersionIdMarker) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CreateBucketConfiguration from response
struct CreateBucketConfigurationParser;
impl CreateBucketConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CreateBucketConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = CreateBucketConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for LocationConstraint is payload.
			if current_name == "LocationConstraint" {
				obj.location_constraint = try!(BucketLocationConstraintParser::parse_xml("LocationConstraint", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct CreateBucketConfiguration {
	/// Specifies the region where the bucket will be created. If you don't specify a
	/// region, the bucket will be created in US Standard.
	pub location_constraint: BucketLocationConstraint,
}

/// Write CreateBucketConfiguration contents to a SignedRequest via headers
struct CreateBucketConfigurationWriter;
impl CreateBucketConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &CreateBucketConfiguration) {
		BucketLocationConstraintWriter::write_params(params, &(prefix.to_string() + "LocationConstraint"), &obj.location_constraint);
	}
}
//typeparser

/// Parse DisplayName from response
struct DisplayNameParser;
impl DisplayNameParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<DisplayName, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type DisplayName = String;
/// Write DisplayName contents to a SignedRequest via headers
struct DisplayNameWriter;
impl DisplayNameWriter {
	fn write_params(request: &mut request, name: &str, obj: &DisplayName) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Grant from response
struct GrantParser;
impl GrantParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Grant, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Grant::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Grantee is payload.
			if current_name == "Grantee" {
				obj.grantee = try!(GranteeParser::parse_xml("Grantee", stack));
				continue;
			}
			// heylisten etc: location for Permission is payload.
			if current_name == "Permission" {
				obj.permission = try!(PermissionParser::parse_xml("Permission", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Grant {
	pub grantee: Grantee,
	/// Specifies the permission given to the grantee.
	pub permission: Permission,
}

/// Write Grant contents to a SignedRequest via headers
struct GrantWriter;
impl GrantWriter {
	fn write_params(request: &mut request, name: &str, obj: &Grant) {
		GranteeWriter::write_params(params, &(prefix.to_string() + "Grantee"), &obj.grantee);
		PermissionWriter::write_params(params, &(prefix.to_string() + "Permission"), &obj.permission);
	}
}
//typeparser

/// Parse CopySourceIfModifiedSince from response
struct CopySourceIfModifiedSinceParser;
impl CopySourceIfModifiedSinceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceIfModifiedSince, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceIfModifiedSince = String;
/// Write CopySourceIfModifiedSince contents to a SignedRequest via headers
struct CopySourceIfModifiedSinceWriter;
impl CopySourceIfModifiedSinceWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceIfModifiedSince) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse HttpRedirectCode from response
struct HttpRedirectCodeParser;
impl HttpRedirectCodeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<HttpRedirectCode, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type HttpRedirectCode = String;
/// Write HttpRedirectCode contents to a SignedRequest via headers
struct HttpRedirectCodeWriter;
impl HttpRedirectCodeWriter {
	fn write_params(request: &mut request, name: &str, obj: &HttpRedirectCode) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse TargetPrefix from response
struct TargetPrefixParser;
impl TargetPrefixParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TargetPrefix, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type TargetPrefix = String;
/// Write TargetPrefix contents to a SignedRequest via headers
struct TargetPrefixWriter;
impl TargetPrefixWriter {
	fn write_params(request: &mut request, name: &str, obj: &TargetPrefix) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Location from response
struct LocationParser;
impl LocationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Location, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Location = String;
/// Write Location contents to a SignedRequest via headers
struct LocationWriter;
impl LocationWriter {
	fn write_params(request: &mut request, name: &str, obj: &Location) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CloudFunctionInvocationRole from response
struct CloudFunctionInvocationRoleParser;
impl CloudFunctionInvocationRoleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CloudFunctionInvocationRole, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CloudFunctionInvocationRole = String;
/// Write CloudFunctionInvocationRole contents to a SignedRequest via headers
struct CloudFunctionInvocationRoleWriter;
impl CloudFunctionInvocationRoleWriter {
	fn write_params(request: &mut request, name: &str, obj: &CloudFunctionInvocationRole) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse MultipartUpload from response
struct MultipartUploadParser;
impl MultipartUploadParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MultipartUpload, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = MultipartUpload::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Initiator is payload.
			if current_name == "Initiator" {
				obj.initiator = try!(InitiatorParser::parse_xml("Initiator", stack));
				continue;
			}
			// heylisten etc: location for Initiated is payload.
			if current_name == "Initiated" {
				obj.initiated = try!(InitiatedParser::parse_xml("Initiated", stack));
				continue;
			}
			// heylisten etc: location for UploadId is payload.
			if current_name == "UploadId" {
				obj.upload_id = try!(MultipartUploadIdParser::parse_xml("UploadId", stack));
				continue;
			}
			// heylisten etc: location for StorageClass is payload.
			if current_name == "StorageClass" {
				obj.storage_class = try!(StorageClassParser::parse_xml("StorageClass", stack));
				continue;
			}
			// heylisten etc: location for Key is payload.
			if current_name == "Key" {
				obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
				continue;
			}
			// heylisten etc: location for Owner is payload.
			if current_name == "Owner" {
				obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct MultipartUpload {
	/// Identifies who initiated the multipart upload.
	pub initiator: Initiator,
	/// Date and time at which the multipart upload was initiated.
	pub initiated: Initiated,
	/// Upload ID that identifies the multipart upload.
	pub upload_id: MultipartUploadId,
	/// The class of storage used to store the object.
	pub storage_class: StorageClass,
	/// Key of the object for which the multipart upload was initiated.
	pub key: ObjectKey,
	pub owner: Owner,
}

/// Write MultipartUpload contents to a SignedRequest via headers
struct MultipartUploadWriter;
impl MultipartUploadWriter {
	fn write_params(request: &mut request, name: &str, obj: &MultipartUpload) {
		InitiatorWriter::write_params(params, &(prefix.to_string() + "Initiator"), &obj.initiator);
		InitiatedWriter::write_params(params, &(prefix.to_string() + "Initiated"), &obj.initiated);
		MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "UploadId"), &obj.upload_id);
		StorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
		OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
	}
}
//typeparser

/// Parse PartNumber from response
struct PartNumberParser;
impl PartNumberParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<PartNumber, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type PartNumber = i32;
/// Write PartNumber contents to a SignedRequest via headers
struct PartNumberWriter;
impl PartNumberWriter {
	fn write_params(request: &mut request, name: &str, obj: &PartNumber) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse Type from response
struct TypeParser;
impl TypeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Type, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Type = String;
/// Write Type contents to a SignedRequest via headers
struct TypeWriter;
impl TypeWriter {
	fn write_params(request: &mut request, name: &str, obj: &Type) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Expires from response
struct ExpiresParser;
impl ExpiresParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Expires, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Expires = String;
/// Write Expires contents to a SignedRequest via headers
struct ExpiresWriter;
impl ExpiresWriter {
	fn write_params(request: &mut request, name: &str, obj: &Expires) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse MFA from response
struct MFAParser;
impl MFAParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MFA, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MFA = String;
/// Write MFA contents to a SignedRequest via headers
struct MFAWriter;
impl MFAWriter {
	fn write_params(request: &mut request, name: &str, obj: &MFA) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse MetadataValue from response
struct MetadataValueParser;
impl MetadataValueParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MetadataValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MetadataValue = String;
/// Write MetadataValue contents to a SignedRequest via headers
struct MetadataValueWriter;
impl MetadataValueWriter {
	fn write_params(request: &mut request, name: &str, obj: &MetadataValue) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse IfUnmodifiedSince from response
struct IfUnmodifiedSinceParser;
impl IfUnmodifiedSinceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<IfUnmodifiedSince, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type IfUnmodifiedSince = String;
/// Write IfUnmodifiedSince contents to a SignedRequest via headers
struct IfUnmodifiedSinceWriter;
impl IfUnmodifiedSinceWriter {
	fn write_params(request: &mut request, name: &str, obj: &IfUnmodifiedSince) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Permission from response
struct PermissionParser;
impl PermissionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Permission, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Permission = String;
/// Write Permission contents to a SignedRequest via headers
struct PermissionWriter;
impl PermissionWriter {
	fn write_params(request: &mut request, name: &str, obj: &Permission) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse RequestPaymentConfiguration from response
struct RequestPaymentConfigurationParser;
impl RequestPaymentConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<RequestPaymentConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = RequestPaymentConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Payer is payload.
			if current_name == "Payer" {
				obj.payer = try!(PayerParser::parse_xml("Payer", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct RequestPaymentConfiguration {
	/// Specifies who pays for the download and request fees.
	pub payer: Payer,
}

/// Write RequestPaymentConfiguration contents to a SignedRequest via headers
struct RequestPaymentConfigurationWriter;
impl RequestPaymentConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &RequestPaymentConfiguration) {
		PayerWriter::write_params(params, &(prefix.to_string() + "Payer"), &obj.payer);
	}
}
//typeparser

/// Parse CopySourceIfNoneMatch from response
struct CopySourceIfNoneMatchParser;
impl CopySourceIfNoneMatchParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceIfNoneMatch, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceIfNoneMatch = String;
/// Write CopySourceIfNoneMatch contents to a SignedRequest via headers
struct CopySourceIfNoneMatchWriter;
impl CopySourceIfNoneMatchWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceIfNoneMatch) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse VersioningConfiguration from response
struct VersioningConfigurationParser;
impl VersioningConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<VersioningConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = VersioningConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Status is payload.
			if current_name == "Status" {
				obj.status = try!(BucketVersioningStatusParser::parse_xml("Status", stack));
				continue;
			}
			// heylisten etc: location for MFADelete is payload.
			if current_name == "MfaDelete" {
				obj.mfa_delete = try!(MFADeleteParser::parse_xml("MfaDelete", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct VersioningConfiguration {
	/// The versioning state of the bucket.
	pub status: BucketVersioningStatus,
	/// Specifies whether MFA delete is enabled in the bucket versioning
	/// configuration. This element is only returned if the bucket has been configured
	/// with MFA delete. If the bucket has never been so configured, this element is
	/// not returned.
	pub mfa_delete: MFADelete,
}

/// Write VersioningConfiguration contents to a SignedRequest via headers
struct VersioningConfigurationWriter;
impl VersioningConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &VersioningConfiguration) {
		BucketVersioningStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		MFADeleteWriter::write_params(params, &(prefix.to_string() + "MfaDelete"), &obj.mfa_delete);
	}
}
//typeparser

/// Parse CloudFunctionConfiguration from response
struct CloudFunctionConfigurationParser;
impl CloudFunctionConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CloudFunctionConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = CloudFunctionConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for InvocationRole is payload.
			if current_name == "InvocationRole" {
				obj.invocation_role = try!(CloudFunctionInvocationRoleParser::parse_xml("InvocationRole", stack));
				continue;
			}
			// heylisten etc: location for CloudFunction is payload.
			if current_name == "CloudFunction" {
				obj.cloud_function = try!(CloudFunctionParser::parse_xml("CloudFunction", stack));
				continue;
			}
			// heylisten etc: location for Events is payload.
			if current_name == "Event" {
				obj.events = try!(EventListParser::parse_xml("Event", stack));
				continue;
			}
			// heylisten etc: location for Id is payload.
			if current_name == "Id" {
				obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
				continue;
			}
			// heylisten etc: location for Event is payload.
			if current_name == "Event" {
				obj.event = try!(EventParser::parse_xml("Event", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct CloudFunctionConfiguration {
	pub invocation_role: CloudFunctionInvocationRole,
	pub cloud_function: CloudFunction,
	pub events: EventList,
	pub id: NotificationId,
	pub event: Event,
}

/// Write CloudFunctionConfiguration contents to a SignedRequest via headers
struct CloudFunctionConfigurationWriter;
impl CloudFunctionConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &CloudFunctionConfiguration) {
		CloudFunctionInvocationRoleWriter::write_params(params, &(prefix.to_string() + "InvocationRole"), &obj.invocation_role);
		CloudFunctionWriter::write_params(params, &(prefix.to_string() + "CloudFunction"), &obj.cloud_function);
		EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
		NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
		EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
	}
}
#[derive(Debug, Default)]
pub struct CopyObjectRequest {
	pub request_payer: Option<RequestPayer>,
	/// Copies the object if it has been modified since the specified time.
	pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
	/// Copies the object if it hasn't been modified since the specified time.
	pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
	/// Specifies what content encodings have been applied to the object and thus what
	/// decoding mechanisms must be applied to obtain the media-type referenced by the
	/// Content-Type header field.
	pub content_encoding: Option<ContentEncoding>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use to decrypt
	/// the source object. The encryption key provided in this header must be one that
	/// was used when the source object was created.
	pub copy_source_sse_customer_key: Option<CopySourceSSECustomerKey>,
	/// The type of storage to use for the object. Defaults to 'STANDARD'.
	pub storage_class: Option<StorageClass>,
	/// Allows grantee to read the object ACL.
	pub grant_read_acp: Option<GrantReadACP>,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: Option<ServerSideEncryption>,
	/// Specifies the AWS KMS key ID to use for object encryption. All GET and PUT
	/// requests for an object protected by AWS KMS will fail if not made via SSL or
	/// using SigV4. Documentation on configuring any of the officially supported AWS
	/// SDKs and CLI can be found at
	/// http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-
	/// signature-version
	pub ssekms_key_id: Option<SSEKMSKeyId>,
	/// Specifies presentational information for the object.
	pub content_disposition: Option<ContentDisposition>,
	/// A map of metadata to store with the object in S3.
	pub metadata: Option<Metadata>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use in
	/// encrypting data. This value is used to store the object and then it is
	/// discarded; Amazon does not store the encryption key. The key must be
	/// appropriate for use with the algorithm specified in the x-amz-server-side-
	/// encryption-customer-algorithm header.
	pub sse_customer_key: Option<SSECustomerKey>,
	/// If the bucket is configured as a website, redirects requests for this object
	/// to another object in the same bucket or to an external URL. Amazon S3 stores
	/// the value of this header in the object metadata.
	pub website_redirect_location: Option<WebsiteRedirectLocation>,
	/// The name of the source bucket and key name of the source object, separated by
	/// a slash (/). Must be URL-encoded.
	pub copy_source: CopySource,
	/// The date and time at which the object is no longer cacheable.
	pub expires: Option<Expires>,
	pub key: ObjectKey,
	/// Specifies caching behavior along the request/reply chain.
	pub cache_control: Option<CacheControl>,
	/// Specifies the algorithm to use when decrypting the source object (e.g.,
	/// AES256).
	pub copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm>,
	pub bucket: BucketName,
	/// Allows grantee to read the object data and its metadata.
	pub grant_read: Option<GrantRead>,
	/// Allows grantee to write the ACL for the applicable object.
	pub grant_write_acp: Option<GrantWriteACP>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5>,
	/// The canned ACL to apply to the object.
	pub acl: Option<ObjectCannedACL>,
	/// Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.
	pub grant_full_control: Option<GrantFullControl>,
	/// Copies the object if its entity tag (ETag) matches the specified tag.
	pub copy_source_if_match: Option<CopySourceIfMatch>,
	/// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
	pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
	/// A standard MIME type describing the format of the object data.
	pub content_type: Option<ContentType>,
	/// The language the content is in.
	pub content_language: Option<ContentLanguage>,
	/// Specifies whether the metadata is copied from the source object or replaced
	/// with metadata provided in the request.
	pub metadata_directive: Option<MetadataDirective>,
	/// Copies the object if its entity tag (ETag) is different than the specified
	/// ETag.
	pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

/// Write CopyObjectRequest contents to a SignedRequest via headers
struct CopyObjectRequestWriter;
impl CopyObjectRequestWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopyObjectRequest) {
		if let Some(ref obj) = obj.request_payer {
			 request.add_header("x-amz-request-payer", obj);
		}
		if let Some(ref obj) = obj.copy_source_if_modified_since {
			 request.add_header("x-amz-copy-source-if-modified-since", obj);
		}
		if let Some(ref obj) = obj.copy_source_if_unmodified_since {
			 request.add_header("x-amz-copy-source-if-unmodified-since", obj);
		}
		if let Some(ref obj) = obj.content_encoding {
			 request.add_header("Content-Encoding", obj);
		}
		if let Some(ref obj) = obj.copy_source_sse_customer_key {
			 request.add_header("x-amz-copy-source-server-side-encryption-customer-key", obj);
		}
		if let Some(ref obj) = obj.storage_class {
			 request.add_header("x-amz-storage-class", obj);
		}
		if let Some(ref obj) = obj.grant_read_acp {
			 request.add_header("x-amz-grant-read-acp", obj);
		}
		if let Some(ref obj) = obj.server_side_encryption {
			 request.add_header("x-amz-server-side-encryption", obj);
		}
		if let Some(ref obj) = obj.ssekms_key_id {
			 request.add_header("x-amz-server-side-encryption-aws-kms-key-id", obj);
		}
		if let Some(ref obj) = obj.content_disposition {
			 request.add_header("Content-Disposition", obj);
		}
		if let Some(ref obj) = obj.metadata {
			 request.add_header("x-amz-meta-", obj);
		}
		if let Some(ref obj) = obj.sse_customer_key {
			 request.add_header("x-amz-server-side-encryption-customer-key", obj);
		}
		if let Some(ref obj) = obj.website_redirect_location {
			 request.add_header("x-amz-website-redirect-location", obj);
		}
		CopySourceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source"), &obj.copy_source);
		if let Some(ref obj) = obj.expires {
			 request.add_header("Expires", obj);
		}
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
		if let Some(ref obj) = obj.cache_control {
			 request.add_header("Cache-Control", obj);
		}
		if let Some(ref obj) = obj.copy_source_sse_customer_algorithm {
			 request.add_header("x-amz-copy-source-server-side-encryption-customer-algorithm", obj);
		}
		BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
		if let Some(ref obj) = obj.grant_read {
			 request.add_header("x-amz-grant-read", obj);
		}
		if let Some(ref obj) = obj.grant_write_acp {
			 request.add_header("x-amz-grant-write-acp", obj);
		}
		if let Some(ref obj) = obj.copy_source_sse_customer_key_md5 {
			 request.add_header("x-amz-copy-source-server-side-encryption-customer-key-MD5", obj);
		}
		if let Some(ref obj) = obj.acl {
			 request.add_header("x-amz-acl", obj);
		}
		if let Some(ref obj) = obj.grant_full_control {
			 request.add_header("x-amz-grant-full-control", obj);
		}
		if let Some(ref obj) = obj.copy_source_if_match {
			 request.add_header("x-amz-copy-source-if-match", obj);
		}
		if let Some(ref obj) = obj.sse_customer_algorithm {
			 request.add_header("x-amz-server-side-encryption-customer-algorithm", obj);
		}
		if let Some(ref obj) = obj.content_type {
			 request.add_header("Content-Type", obj);
		}
		if let Some(ref obj) = obj.content_language {
			 request.add_header("Content-Language", obj);
		}
		if let Some(ref obj) = obj.metadata_directive {
			 request.add_header("x-amz-metadata-directive", obj);
		}
		if let Some(ref obj) = obj.copy_source_if_none_match {
			 request.add_header("x-amz-copy-source-if-none-match", obj);
		}
		if let Some(ref obj) = obj.sse_customer_key_md5 {
			 request.add_header("x-amz-server-side-encryption-customer-key-MD5", obj);
		}
	}
}
//typeparser

/// Parse IsTruncated from response
struct IsTruncatedParser;
impl IsTruncatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<IsTruncated, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type IsTruncated = bool;
/// Write IsTruncated contents to a SignedRequest via headers
struct IsTruncatedWriter;
impl IsTruncatedWriter {
	fn write_params(request: &mut request, name: &str, obj: &IsTruncated) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse TagSet from response
struct TagSetParser;
impl TagSetParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TagSet, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Tag" {
			obj.push(try!(TagParser::parse_xml("Tag", stack)));
		}
		Ok(obj)
	}
}
pub type TagSet = Vec<Tag>;
/// Write TagSet contents to a SignedRequest via headers
struct TagSetWriter;
impl TagSetWriter {
	fn write_params(request: &mut request, name: &str, obj: &TagSet) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			TagWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse EventList from response
struct EventListParser;
impl EventListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<EventList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Event" {
			obj.push(try!(EventParser::parse_xml("Event", stack)));
		}
		Ok(obj)
	}
}
pub type EventList = Vec<Event>;
/// Write EventList contents to a SignedRequest via headers
struct EventListWriter;
impl EventListWriter {
	fn write_params(request: &mut request, name: &str, obj: &EventList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			EventWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse LambdaFunctionArn from response
struct LambdaFunctionArnParser;
impl LambdaFunctionArnParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<LambdaFunctionArn, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type LambdaFunctionArn = String;
/// Write LambdaFunctionArn contents to a SignedRequest via headers
struct LambdaFunctionArnWriter;
impl LambdaFunctionArnWriter {
	fn write_params(request: &mut request, name: &str, obj: &LambdaFunctionArn) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Quiet from response
struct QuietParser;
impl QuietParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Quiet, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Quiet = bool;
/// Write Quiet contents to a SignedRequest via headers
struct QuietWriter;
impl QuietWriter {
	fn write_params(request: &mut request, name: &str, obj: &Quiet) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse AccessControlPolicy from response
struct AccessControlPolicyParser;
impl AccessControlPolicyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<AccessControlPolicy, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = AccessControlPolicy::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Owner is payload.
			if current_name == "Owner" {
				obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
				continue;
			}
			// heylisten etc: location for Grants is payload.
			if current_name == "Grant" {
				obj.grants = try!(GrantsParser::parse_xml("Grant", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct AccessControlPolicy {
	pub owner: Owner,
	/// A list of grants.
	pub grants: Grants,
}

/// Write AccessControlPolicy contents to a SignedRequest via headers
struct AccessControlPolicyWriter;
impl AccessControlPolicyWriter {
	fn write_params(request: &mut request, name: &str, obj: &AccessControlPolicy) {
		OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
		GrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.grants);
	}
}
//typeparser

/// Parse Range from response
struct RangeParser;
impl RangeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Range, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Range = String;
/// Write Range contents to a SignedRequest via headers
struct RangeWriter;
impl RangeWriter {
	fn write_params(request: &mut request, name: &str, obj: &Range) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse GrantRead from response
struct GrantReadParser;
impl GrantReadParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<GrantRead, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type GrantRead = String;
/// Write GrantRead contents to a SignedRequest via headers
struct GrantReadWriter;
impl GrantReadWriter {
	fn write_params(request: &mut request, name: &str, obj: &GrantRead) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse SSECustomerKeyMD5 from response
struct SSECustomerKeyMD5Parser;
impl SSECustomerKeyMD5Parser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<SSECustomerKeyMD5, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type SSECustomerKeyMD5 = String;
/// Write SSECustomerKeyMD5 contents to a SignedRequest via headers
struct SSECustomerKeyMD5Writer;
impl SSECustomerKeyMD5Writer {
	fn write_params(request: &mut request, name: &str, obj: &SSECustomerKeyMD5) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ObjectIdentifierList from response
struct ObjectIdentifierListParser;
impl ObjectIdentifierListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectIdentifierList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ObjectIdentifier" {
			obj.push(try!(ObjectIdentifierParser::parse_xml("ObjectIdentifier", stack)));
		}
		Ok(obj)
	}
}
pub type ObjectIdentifierList = Vec<ObjectIdentifier>;
/// Write ObjectIdentifierList contents to a SignedRequest via headers
struct ObjectIdentifierListWriter;
impl ObjectIdentifierListWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectIdentifierList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ObjectIdentifierWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse NextVersionIdMarker from response
struct NextVersionIdMarkerParser;
impl NextVersionIdMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NextVersionIdMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type NextVersionIdMarker = String;
/// Write NextVersionIdMarker contents to a SignedRequest via headers
struct NextVersionIdMarkerWriter;
impl NextVersionIdMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &NextVersionIdMarker) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Delete from response
struct DeleteParser;
impl DeleteParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Delete, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Delete::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Objects is payload.
			if current_name == "ObjectIdentifier" {
				obj.objects = try!(ObjectIdentifierListParser::parse_xml("ObjectIdentifier", stack));
				continue;
			}
			// heylisten etc: location for Quiet is payload.
			if current_name == "Quiet" {
				obj.quiet = Some(try!(QuietParser::parse_xml("Quiet", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Delete {
	pub objects: ObjectIdentifierList,
	/// Element to enable quiet mode for the request. When you add this element, you
	/// must set its value to true.
	pub quiet: Option<Quiet>,
}

/// Write Delete contents to a SignedRequest via headers
struct DeleteWriter;
impl DeleteWriter {
	fn write_params(request: &mut request, name: &str, obj: &Delete) {
		ObjectIdentifierListWriter::write_params(params, &(prefix.to_string() + "ObjectIdentifier"), &obj.objects);
		if let Some(ref obj) = obj.quiet {
			 request.add_header("Quiet", obj);
		}
	}
}
//typeparser

/// Parse ResponseContentLanguage from response
struct ResponseContentLanguageParser;
impl ResponseContentLanguageParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ResponseContentLanguage, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ResponseContentLanguage = String;
/// Write ResponseContentLanguage contents to a SignedRequest via headers
struct ResponseContentLanguageWriter;
impl ResponseContentLanguageWriter {
	fn write_params(request: &mut request, name: &str, obj: &ResponseContentLanguage) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse GrantWriteACP from response
struct GrantWriteACPParser;
impl GrantWriteACPParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<GrantWriteACP, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type GrantWriteACP = String;
/// Write GrantWriteACP contents to a SignedRequest via headers
struct GrantWriteACPWriter;
impl GrantWriteACPWriter {
	fn write_params(request: &mut request, name: &str, obj: &GrantWriteACP) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CORSRules from response
struct CORSRulesParser;
impl CORSRulesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CORSRules, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "CORSRule" {
			obj.push(try!(CORSRuleParser::parse_xml("CORSRule", stack)));
		}
		Ok(obj)
	}
}
pub type CORSRules = Vec<CORSRule>;
/// Write CORSRules contents to a SignedRequest via headers
struct CORSRulesWriter;
impl CORSRulesWriter {
	fn write_params(request: &mut request, name: &str, obj: &CORSRules) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			CORSRuleWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse ContentLanguage from response
struct ContentLanguageParser;
impl ContentLanguageParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ContentLanguage, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ContentLanguage = String;
/// Write ContentLanguage contents to a SignedRequest via headers
struct ContentLanguageWriter;
impl ContentLanguageWriter {
	fn write_params(request: &mut request, name: &str, obj: &ContentLanguage) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse NotificationConfiguration from response
struct NotificationConfigurationParser;
impl NotificationConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NotificationConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = NotificationConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for QueueConfigurations is payload.
			if current_name == "QueueConfiguration" {
				obj.queue_configurations = try!(QueueConfigurationListParser::parse_xml("QueueConfiguration", stack));
				continue;
			}
			// heylisten etc: location for LambdaFunctionConfigurations is payload.
			if current_name == "LambdaFunctionConfiguration" {
				obj.lambda_function_configurations = try!(LambdaFunctionConfigurationListParser::parse_xml("LambdaFunctionConfiguration", stack));
				continue;
			}
			// heylisten etc: location for TopicConfigurations is payload.
			if current_name == "TopicConfiguration" {
				obj.topic_configurations = try!(TopicConfigurationListParser::parse_xml("TopicConfiguration", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Container for specifying the notification configuration of the bucket. If this
/// element is empty, notifications are turned off on the bucket.
#[derive(Debug, Default)]
pub struct NotificationConfiguration {
	pub queue_configurations: QueueConfigurationList,
	pub lambda_function_configurations: LambdaFunctionConfigurationList,
	pub topic_configurations: TopicConfigurationList,
}

/// Write NotificationConfiguration contents to a SignedRequest via headers
struct NotificationConfigurationWriter;
impl NotificationConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &NotificationConfiguration) {
		QueueConfigurationListWriter::write_params(params, &(prefix.to_string() + "QueueConfiguration"), &obj.queue_configurations);
		LambdaFunctionConfigurationListWriter::write_params(params, &(prefix.to_string() + "LambdaFunctionConfiguration"), &obj.lambda_function_configurations);
		TopicConfigurationListWriter::write_params(params, &(prefix.to_string() + "TopicConfiguration"), &obj.topic_configurations);
	}
}
//typeparser

/// Parse CORSRule from response
struct CORSRuleParser;
impl CORSRuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CORSRule, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = CORSRule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for AllowedHeaders is payload.
			if current_name == "AllowedHeader" {
				obj.allowed_headers = try!(AllowedHeadersParser::parse_xml("AllowedHeader", stack));
				continue;
			}
			// heylisten etc: location for ExposeHeaders is payload.
			if current_name == "ExposeHeader" {
				obj.expose_headers = try!(ExposeHeadersParser::parse_xml("ExposeHeader", stack));
				continue;
			}
			// heylisten etc: location for AllowedMethods is payload.
			if current_name == "AllowedMethod" {
				obj.allowed_methods = try!(AllowedMethodsParser::parse_xml("AllowedMethod", stack));
				continue;
			}
			// heylisten etc: location for MaxAgeSeconds is payload.
			if current_name == "MaxAgeSeconds" {
				obj.max_age_seconds = try!(MaxAgeSecondsParser::parse_xml("MaxAgeSeconds", stack));
				continue;
			}
			// heylisten etc: location for AllowedOrigins is payload.
			if current_name == "AllowedOrigin" {
				obj.allowed_origins = try!(AllowedOriginsParser::parse_xml("AllowedOrigin", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct CORSRule {
	/// Specifies which headers are allowed in a pre-flight OPTIONS request.
	pub allowed_headers: AllowedHeaders,
	/// One or more headers in the response that you want customers to be able to
	/// access from their applications (for example, from a JavaScript XMLHttpRequest
	/// object).
	pub expose_headers: ExposeHeaders,
	/// Identifies HTTP methods that the domain/origin specified in the rule is
	/// allowed to execute.
	pub allowed_methods: AllowedMethods,
	/// The time in seconds that your browser is to cache the preflight response for
	/// the specified resource.
	pub max_age_seconds: MaxAgeSeconds,
	/// One or more origins you want customers to be able to access the bucket from.
	pub allowed_origins: AllowedOrigins,
}

/// Write CORSRule contents to a SignedRequest via headers
struct CORSRuleWriter;
impl CORSRuleWriter {
	fn write_params(request: &mut request, name: &str, obj: &CORSRule) {
		AllowedHeadersWriter::write_params(params, &(prefix.to_string() + "AllowedHeader"), &obj.allowed_headers);
		ExposeHeadersWriter::write_params(params, &(prefix.to_string() + "ExposeHeader"), &obj.expose_headers);
		AllowedMethodsWriter::write_params(params, &(prefix.to_string() + "AllowedMethod"), &obj.allowed_methods);
		MaxAgeSecondsWriter::write_params(params, &(prefix.to_string() + "MaxAgeSeconds"), &obj.max_age_seconds);
		AllowedOriginsWriter::write_params(params, &(prefix.to_string() + "AllowedOrigin"), &obj.allowed_origins);
	}
}
//typeparser

/// Parse KeyPrefixEquals from response
struct KeyPrefixEqualsParser;
impl KeyPrefixEqualsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<KeyPrefixEquals, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type KeyPrefixEquals = String;
/// Write KeyPrefixEquals contents to a SignedRequest via headers
struct KeyPrefixEqualsWriter;
impl KeyPrefixEqualsWriter {
	fn write_params(request: &mut request, name: &str, obj: &KeyPrefixEquals) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse WebsiteRedirectLocation from response
struct WebsiteRedirectLocationParser;
impl WebsiteRedirectLocationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<WebsiteRedirectLocation, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type WebsiteRedirectLocation = String;
/// Write WebsiteRedirectLocation contents to a SignedRequest via headers
struct WebsiteRedirectLocationWriter;
impl WebsiteRedirectLocationWriter {
	fn write_params(request: &mut request, name: &str, obj: &WebsiteRedirectLocation) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Owner from response
struct OwnerParser;
impl OwnerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Owner, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Owner::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for DisplayName is payload.
			if current_name == "DisplayName" {
				obj.display_name = try!(DisplayNameParser::parse_xml("DisplayName", stack));
				continue;
			}
			// heylisten etc: location for ID is payload.
			if current_name == "ID" {
				obj.id = try!(IDParser::parse_xml("ID", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Owner {
	pub display_name: DisplayName,
	pub id: ID,
}

/// Write Owner contents to a SignedRequest via headers
struct OwnerWriter;
impl OwnerWriter {
	fn write_params(request: &mut request, name: &str, obj: &Owner) {
		DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), &obj.display_name);
		IDWriter::write_params(params, &(prefix.to_string() + "ID"), &obj.id);
	}
}
//typeparser

/// Parse CopyObjectResult from response
struct CopyObjectResultParser;
impl CopyObjectResultParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopyObjectResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = CopyObjectResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for LastModified is payload.
			if current_name == "LastModified" {
				obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
				continue;
			}
			// heylisten etc: location for ETag is payload.
			if current_name == "ETag" {
				obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct CopyObjectResult {
	pub last_modified: LastModified,
	pub e_tag: ETag,
}

//typeparser

/// Parse TrimmedS3Message from response
struct TrimmedS3MessageParser;
impl TrimmedS3MessageParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TrimmedS3Message, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type TrimmedS3Message = String;
/// Write TrimmedS3Message contents to a SignedRequest via headers
struct TrimmedS3MessageWriter;
impl TrimmedS3MessageWriter {
	fn write_params(request: &mut request, name: &str, obj: &TrimmedS3Message) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Size from response
struct SizeParser;
impl SizeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Size, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Size = i32;
/// Write Size contents to a SignedRequest via headers
struct SizeWriter;
impl SizeWriter {
	fn write_params(request: &mut request, name: &str, obj: &Size) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse Redirect from response
struct RedirectParser;
impl RedirectParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Redirect, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Redirect::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for ReplaceKeyWith is payload.
			if current_name == "ReplaceKeyWith" {
				obj.replace_key_with = try!(ReplaceKeyWithParser::parse_xml("ReplaceKeyWith", stack));
				continue;
			}
			// heylisten etc: location for HostName is payload.
			if current_name == "HostName" {
				obj.host_name = try!(HostNameParser::parse_xml("HostName", stack));
				continue;
			}
			// heylisten etc: location for Protocol is payload.
			if current_name == "Protocol" {
				obj.protocol = try!(ProtocolParser::parse_xml("Protocol", stack));
				continue;
			}
			// heylisten etc: location for ReplaceKeyPrefixWith is payload.
			if current_name == "ReplaceKeyPrefixWith" {
				obj.replace_key_prefix_with = try!(ReplaceKeyPrefixWithParser::parse_xml("ReplaceKeyPrefixWith", stack));
				continue;
			}
			// heylisten etc: location for HttpRedirectCode is payload.
			if current_name == "HttpRedirectCode" {
				obj.http_redirect_code = try!(HttpRedirectCodeParser::parse_xml("HttpRedirectCode", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Redirect {
	/// The specific object key to use in the redirect request. For example, redirect
	/// request to error.html. Not required if one of the sibling is present. Can be
	/// present only if ReplaceKeyPrefixWith is not provided.
	pub replace_key_with: ReplaceKeyWith,
	/// The host name to use in the redirect request.
	pub host_name: HostName,
	/// Protocol to use (http, https) when redirecting requests. The default is the
	/// protocol that is used in the original request.
	pub protocol: Protocol,
	/// The object key prefix to use in the redirect request. For example, to redirect
	/// requests for all pages with prefix docs/ (objects in the docs/ folder) to
	/// documents/, you can set a condition block with KeyPrefixEquals set to docs/
	/// and in the Redirect set ReplaceKeyPrefixWith to /documents. Not required if
	/// one of the siblings is present. Can be present only if ReplaceKeyWith is not
	/// provided.
	pub replace_key_prefix_with: ReplaceKeyPrefixWith,
	/// The HTTP redirect code to use on the response. Not required if one of the
	/// siblings is present.
	pub http_redirect_code: HttpRedirectCode,
}

/// Write Redirect contents to a SignedRequest via headers
struct RedirectWriter;
impl RedirectWriter {
	fn write_params(request: &mut request, name: &str, obj: &Redirect) {
		ReplaceKeyWithWriter::write_params(params, &(prefix.to_string() + "ReplaceKeyWith"), &obj.replace_key_with);
		HostNameWriter::write_params(params, &(prefix.to_string() + "HostName"), &obj.host_name);
		ProtocolWriter::write_params(params, &(prefix.to_string() + "Protocol"), &obj.protocol);
		ReplaceKeyPrefixWithWriter::write_params(params, &(prefix.to_string() + "ReplaceKeyPrefixWith"), &obj.replace_key_prefix_with);
		HttpRedirectCodeWriter::write_params(params, &(prefix.to_string() + "HttpRedirectCode"), &obj.http_redirect_code);
	}
}
//typeparser

/// Parse NextMarker from response
struct NextMarkerParser;
impl NextMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NextMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type NextMarker = String;
/// Write NextMarker contents to a SignedRequest via headers
struct NextMarkerWriter;
impl NextMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &NextMarker) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ContentEncoding from response
struct ContentEncodingParser;
impl ContentEncodingParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ContentEncoding, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ContentEncoding = String;
/// Write ContentEncoding contents to a SignedRequest via headers
struct ContentEncodingWriter;
impl ContentEncodingWriter {
	fn write_params(request: &mut request, name: &str, obj: &ContentEncoding) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse WebsiteConfiguration from response
struct WebsiteConfigurationParser;
impl WebsiteConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<WebsiteConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = WebsiteConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for RedirectAllRequestsTo is payload.
			if current_name == "RedirectAllRequestsTo" {
				obj.redirect_all_requests_to = try!(RedirectAllRequestsToParser::parse_xml("RedirectAllRequestsTo", stack));
				continue;
			}
			// heylisten etc: location for IndexDocument is payload.
			if current_name == "IndexDocument" {
				obj.index_document = try!(IndexDocumentParser::parse_xml("IndexDocument", stack));
				continue;
			}
			// heylisten etc: location for ErrorDocument is payload.
			if current_name == "ErrorDocument" {
				obj.error_document = try!(ErrorDocumentParser::parse_xml("ErrorDocument", stack));
				continue;
			}
			// heylisten etc: location for RoutingRules is payload.
			if current_name == "RoutingRule" {
				obj.routing_rules = try!(RoutingRulesParser::parse_xml("RoutingRule", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct WebsiteConfiguration {
	pub redirect_all_requests_to: RedirectAllRequestsTo,
	pub index_document: IndexDocument,
	pub error_document: ErrorDocument,
	pub routing_rules: RoutingRules,
}

/// Write WebsiteConfiguration contents to a SignedRequest via headers
struct WebsiteConfigurationWriter;
impl WebsiteConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &WebsiteConfiguration) {
		RedirectAllRequestsToWriter::write_params(params, &(prefix.to_string() + "RedirectAllRequestsTo"), &obj.redirect_all_requests_to);
		IndexDocumentWriter::write_params(params, &(prefix.to_string() + "IndexDocument"), &obj.index_document);
		ErrorDocumentWriter::write_params(params, &(prefix.to_string() + "ErrorDocument"), &obj.error_document);
		RoutingRulesWriter::write_params(params, &(prefix.to_string() + "RoutingRule"), &obj.routing_rules);
	}
}
//typeparser

/// Parse MFADelete from response
struct MFADeleteParser;
impl MFADeleteParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MFADelete, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MFADelete = String;
/// Write MFADelete contents to a SignedRequest via headers
struct MFADeleteWriter;
impl MFADeleteWriter {
	fn write_params(request: &mut request, name: &str, obj: &MFADelete) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CopySourceSSECustomerKey from response
struct CopySourceSSECustomerKeyParser;
impl CopySourceSSECustomerKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceSSECustomerKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceSSECustomerKey = String;
/// Write CopySourceSSECustomerKey contents to a SignedRequest via headers
struct CopySourceSSECustomerKeyWriter;
impl CopySourceSSECustomerKeyWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceSSECustomerKey) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse StorageClass from response
struct StorageClassParser;
impl StorageClassParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<StorageClass, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type StorageClass = String;
/// Write StorageClass contents to a SignedRequest via headers
struct StorageClassWriter;
impl StorageClassWriter {
	fn write_params(request: &mut request, name: &str, obj: &StorageClass) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse GrantReadACP from response
struct GrantReadACPParser;
impl GrantReadACPParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<GrantReadACP, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type GrantReadACP = String;
/// Write GrantReadACP contents to a SignedRequest via headers
struct GrantReadACPWriter;
impl GrantReadACPWriter {
	fn write_params(request: &mut request, name: &str, obj: &GrantReadACP) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ExposeHeader from response
struct ExposeHeaderParser;
impl ExposeHeaderParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ExposeHeader, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ExposeHeader = String;
/// Write ExposeHeader contents to a SignedRequest via headers
struct ExposeHeaderWriter;
impl ExposeHeaderWriter {
	fn write_params(request: &mut request, name: &str, obj: &ExposeHeader) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CopyPartResult from response
struct CopyPartResultParser;
impl CopyPartResultParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopyPartResult, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = CopyPartResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for LastModified is payload.
			if current_name == "LastModified" {
				obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
				continue;
			}
			// heylisten etc: location for ETag is payload.
			if current_name == "ETag" {
				obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct CopyPartResult {
	/// Date and time at which the object was uploaded.
	pub last_modified: LastModified,
	/// Entity tag of the object.
	pub e_tag: ETag,
}

//typeparser

/// Parse CopyObjectOutput from response
struct CopyObjectOutputParser;
impl CopyObjectOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopyObjectOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = CopyObjectOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for SSECustomerAlgorithm is header.
			if current_name == "x-amz-server-side-encryption-customer-algorithm" {
				obj.sse_customer_algorithm = try!(response.Headers.get("x-amz-server-side-encryption-customer-algorithm"));
				continue;
			}
			// heylisten etc: location for CopySourceVersionId is header.
			if current_name == "x-amz-copy-source-version-id" {
				obj.copy_source_version_id = try!(response.Headers.get("x-amz-copy-source-version-id"));
				continue;
			}
			// heylisten etc: location for ServerSideEncryption is header.
			if current_name == "x-amz-server-side-encryption" {
				obj.server_side_encryption = try!(response.Headers.get("x-amz-server-side-encryption"));
				continue;
			}
			// heylisten etc: location for RequestCharged is header.
			if current_name == "x-amz-request-charged" {
				obj.request_charged = try!(response.Headers.get("x-amz-request-charged"));
				continue;
			}
			// heylisten etc: location for Expiration is header.
			if current_name == "x-amz-expiration" {
				obj.expiration = try!(response.Headers.get("x-amz-expiration"));
				continue;
			}
			// heylisten etc: location for SSECustomerKeyMD5 is header.
			if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
				obj.sse_customer_key_md5 = try!(response.Headers.get("x-amz-server-side-encryption-customer-key-MD5"));
				continue;
			}
			// heylisten etc: location for CopyObjectResult is payload.
			if current_name == "CopyObjectResult" {
				obj.copy_object_result = try!(CopyObjectResultParser::parse_xml("CopyObjectResult", stack));
				continue;
			}
			// heylisten etc: location for SSEKMSKeyId is header.
			if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
				obj.ssekms_key_id = try!(response.Headers.get("x-amz-server-side-encryption-aws-kms-key-id"));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct CopyObjectOutput {
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header confirming the encryption
	/// algorithm used.
	pub sse_customer_algorithm: SSECustomerAlgorithm,
	pub copy_source_version_id: CopySourceVersionId,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	pub request_charged: RequestCharged,
	/// If the object expiration is configured, the response includes this header.
	pub expiration: Expiration,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header to provide round trip message
	/// integrity verification of the customer-provided encryption key.
	pub sse_customer_key_md5: SSECustomerKeyMD5,
	pub copy_object_result: CopyObjectResult,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
}

//typeparser

/// Parse AcceptRanges from response
struct AcceptRangesParser;
impl AcceptRangesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<AcceptRanges, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type AcceptRanges = String;
/// Write AcceptRanges contents to a SignedRequest via headers
struct AcceptRangesWriter;
impl AcceptRangesWriter {
	fn write_params(request: &mut request, name: &str, obj: &AcceptRanges) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ObjectAlreadyInActiveTierError from response
struct ObjectAlreadyInActiveTierErrorParser;
impl ObjectAlreadyInActiveTierErrorParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectAlreadyInActiveTierError, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = ObjectAlreadyInActiveTierError::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// This operation is not allowed against this storage tier
#[derive(Debug, Default)]
pub struct ObjectAlreadyInActiveTierError;

/// Write ObjectAlreadyInActiveTierError contents to a SignedRequest via headers
struct ObjectAlreadyInActiveTierErrorWriter;
impl ObjectAlreadyInActiveTierErrorWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectAlreadyInActiveTierError) {
	}
}
//typeparser

/// Parse Initiated from response
struct InitiatedParser;
impl InitiatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Initiated, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Initiated = String;
/// Write Initiated contents to a SignedRequest via headers
struct InitiatedWriter;
impl InitiatedWriter {
	fn write_params(request: &mut request, name: &str, obj: &Initiated) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CopySource from response
struct CopySourceParser;
impl CopySourceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySource, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySource = String;
/// Write CopySource contents to a SignedRequest via headers
struct CopySourceWriter;
impl CopySourceWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySource) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse QueueConfigurationList from response
struct QueueConfigurationListParser;
impl QueueConfigurationListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<QueueConfigurationList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "QueueConfiguration" {
			obj.push(try!(QueueConfigurationParser::parse_xml("QueueConfiguration", stack)));
		}
		Ok(obj)
	}
}
pub type QueueConfigurationList = Vec<QueueConfiguration>;
/// Write QueueConfigurationList contents to a SignedRequest via headers
struct QueueConfigurationListWriter;
impl QueueConfigurationListWriter {
	fn write_params(request: &mut request, name: &str, obj: &QueueConfigurationList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			QueueConfigurationWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse ObjectNotInActiveTierError from response
struct ObjectNotInActiveTierErrorParser;
impl ObjectNotInActiveTierErrorParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectNotInActiveTierError, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = ObjectNotInActiveTierError::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// The source object of the COPY operation is not in the active tier and is only
/// stored in Amazon Glacier.
#[derive(Debug, Default)]
pub struct ObjectNotInActiveTierError;

/// Write ObjectNotInActiveTierError contents to a SignedRequest via headers
struct ObjectNotInActiveTierErrorWriter;
impl ObjectNotInActiveTierErrorWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectNotInActiveTierError) {
	}
}
//typeparser

/// Parse TransitionStorageClass from response
struct TransitionStorageClassParser;
impl TransitionStorageClassParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TransitionStorageClass, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type TransitionStorageClass = String;
/// Write TransitionStorageClass contents to a SignedRequest via headers
struct TransitionStorageClassWriter;
impl TransitionStorageClassWriter {
	fn write_params(request: &mut request, name: &str, obj: &TransitionStorageClass) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse DeleteMarker from response
struct DeleteMarkerParser;
impl DeleteMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<DeleteMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type DeleteMarker = bool;
/// Write DeleteMarker contents to a SignedRequest via headers
struct DeleteMarkerWriter;
impl DeleteMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &DeleteMarker) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse Rule from response
struct RuleParser;
impl RuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Rule, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Rule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Status is payload.
			if current_name == "Status" {
				obj.status = try!(ExpirationStatusParser::parse_xml("Status", stack));
				continue;
			}
			// heylisten etc: location for NoncurrentVersionExpiration is payload.
			if current_name == "NoncurrentVersionExpiration" {
				obj.noncurrent_version_expiration = Some(try!(NoncurrentVersionExpirationParser::parse_xml("NoncurrentVersionExpiration", stack)));
				continue;
			}
			// heylisten etc: location for Transition is payload.
			if current_name == "Transition" {
				obj.transition = Some(try!(TransitionParser::parse_xml("Transition", stack)));
				continue;
			}
			// heylisten etc: location for Prefix is payload.
			if current_name == "Prefix" {
				obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
				continue;
			}
			// heylisten etc: location for Expiration is payload.
			if current_name == "Expiration" {
				obj.expiration = Some(try!(LifecycleExpirationParser::parse_xml("Expiration", stack)));
				continue;
			}
			// heylisten etc: location for NoncurrentVersionTransition is payload.
			if current_name == "NoncurrentVersionTransition" {
				obj.noncurrent_version_transition = Some(try!(NoncurrentVersionTransitionParser::parse_xml("NoncurrentVersionTransition", stack)));
				continue;
			}
			// heylisten etc: location for ID is payload.
			if current_name == "ID" {
				obj.id = Some(try!(IDParser::parse_xml("ID", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Rule {
	/// If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is
	/// not currently being applied.
	pub status: ExpirationStatus,
	pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
	pub transition: Option<Transition>,
	/// Prefix identifying one or more objects to which the rule applies.
	pub prefix: Prefix,
	pub expiration: Option<LifecycleExpiration>,
	pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,
	/// Unique identifier for the rule. The value cannot be longer than 255
	/// characters.
	pub id: Option<ID>,
}

/// Write Rule contents to a SignedRequest via headers
struct RuleWriter;
impl RuleWriter {
	fn write_params(request: &mut request, name: &str, obj: &Rule) {
		ExpirationStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
		if let Some(ref obj) = obj.noncurrent_version_expiration {
			 request.add_header("NoncurrentVersionExpiration", obj);
		}
		if let Some(ref obj) = obj.transition {
			 request.add_header("Transition", obj);
		}
		PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
		if let Some(ref obj) = obj.expiration {
			 request.add_header("Expiration", obj);
		}
		if let Some(ref obj) = obj.noncurrent_version_transition {
			 request.add_header("NoncurrentVersionTransition", obj);
		}
		if let Some(ref obj) = obj.id {
			 request.add_header("ID", obj);
		}
	}
}
//typeparser

/// Parse Date from response
struct DateParser;
impl DateParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Date, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Date = String;
/// Write Date contents to a SignedRequest via headers
struct DateWriter;
impl DateWriter {
	fn write_params(request: &mut request, name: &str, obj: &Date) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CacheControl from response
struct CacheControlParser;
impl CacheControlParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CacheControl, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CacheControl = String;
/// Write CacheControl contents to a SignedRequest via headers
struct CacheControlWriter;
impl CacheControlWriter {
	fn write_params(request: &mut request, name: &str, obj: &CacheControl) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse AllowedOrigin from response
struct AllowedOriginParser;
impl AllowedOriginParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<AllowedOrigin, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type AllowedOrigin = String;
/// Write AllowedOrigin contents to a SignedRequest via headers
struct AllowedOriginWriter;
impl AllowedOriginWriter {
	fn write_params(request: &mut request, name: &str, obj: &AllowedOrigin) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse IfModifiedSince from response
struct IfModifiedSinceParser;
impl IfModifiedSinceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<IfModifiedSince, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type IfModifiedSince = String;
/// Write IfModifiedSince contents to a SignedRequest via headers
struct IfModifiedSinceWriter;
impl IfModifiedSinceWriter {
	fn write_params(request: &mut request, name: &str, obj: &IfModifiedSince) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Condition from response
struct ConditionParser;
impl ConditionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Condition, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Condition::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for HttpErrorCodeReturnedEquals is payload.
			if current_name == "HttpErrorCodeReturnedEquals" {
				obj.http_error_code_returned_equals = try!(HttpErrorCodeReturnedEqualsParser::parse_xml("HttpErrorCodeReturnedEquals", stack));
				continue;
			}
			// heylisten etc: location for KeyPrefixEquals is payload.
			if current_name == "KeyPrefixEquals" {
				obj.key_prefix_equals = try!(KeyPrefixEqualsParser::parse_xml("KeyPrefixEquals", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Condition {
	/// The HTTP error code when the redirect is applied. In the event of an error, if
	/// the error code equals this value, then the specified redirect is applied.
	/// Required when parent element Condition is specified and sibling
	/// KeyPrefixEquals is not specified. If both are specified, then both must be
	/// true for the redirect to be applied.
	pub http_error_code_returned_equals: HttpErrorCodeReturnedEquals,
	/// The object key name prefix when the redirect is applied. For example, to
	/// redirect requests for ExamplePage.html, the key prefix will be
	/// ExamplePage.html. To redirect request for all pages with the prefix docs/, the
	/// key prefix will be /docs, which identifies all objects in the docs/ folder.
	/// Required when the parent element Condition is specified and sibling
	/// HttpErrorCodeReturnedEquals is not specified. If both conditions are
	/// specified, both must be true for the redirect to be applied.
	pub key_prefix_equals: KeyPrefixEquals,
}

/// Write Condition contents to a SignedRequest via headers
struct ConditionWriter;
impl ConditionWriter {
	fn write_params(request: &mut request, name: &str, obj: &Condition) {
		HttpErrorCodeReturnedEqualsWriter::write_params(params, &(prefix.to_string() + "HttpErrorCodeReturnedEquals"), &obj.http_error_code_returned_equals);
		KeyPrefixEqualsWriter::write_params(params, &(prefix.to_string() + "KeyPrefixEquals"), &obj.key_prefix_equals);
	}
}
//typeparser

/// Parse NextKeyMarker from response
struct NextKeyMarkerParser;
impl NextKeyMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NextKeyMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type NextKeyMarker = String;
/// Write NextKeyMarker contents to a SignedRequest via headers
struct NextKeyMarkerWriter;
impl NextKeyMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &NextKeyMarker) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ErrorDocument from response
struct ErrorDocumentParser;
impl ErrorDocumentParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ErrorDocument, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = ErrorDocument::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Key is payload.
			if current_name == "Key" {
				obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct ErrorDocument {
	/// The object key name to use when a 4XX class error occurs.
	pub key: ObjectKey,
}

/// Write ErrorDocument contents to a SignedRequest via headers
struct ErrorDocumentWriter;
impl ErrorDocumentWriter {
	fn write_params(request: &mut request, name: &str, obj: &ErrorDocument) {
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
	}
}
//typeparser

/// Parse Payer from response
struct PayerParser;
impl PayerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Payer, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Payer = String;
/// Write Payer contents to a SignedRequest via headers
struct PayerWriter;
impl PayerWriter {
	fn write_params(request: &mut request, name: &str, obj: &Payer) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Restore from response
struct RestoreParser;
impl RestoreParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Restore, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Restore = String;
/// Write Restore contents to a SignedRequest via headers
struct RestoreWriter;
impl RestoreWriter {
	fn write_params(request: &mut request, name: &str, obj: &Restore) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ContentLength from response
struct ContentLengthParser;
impl ContentLengthParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ContentLength, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ContentLength = i32;
/// Write ContentLength contents to a SignedRequest via headers
struct ContentLengthWriter;
impl ContentLengthWriter {
	fn write_params(request: &mut request, name: &str, obj: &ContentLength) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse Transition from response
struct TransitionParser;
impl TransitionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Transition, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Transition::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Date is payload.
			if current_name == "Date" {
				obj.date = try!(DateParser::parse_xml("Date", stack));
				continue;
			}
			// heylisten etc: location for Days is payload.
			if current_name == "Days" {
				obj.days = try!(DaysParser::parse_xml("Days", stack));
				continue;
			}
			// heylisten etc: location for StorageClass is payload.
			if current_name == "StorageClass" {
				obj.storage_class = try!(TransitionStorageClassParser::parse_xml("StorageClass", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Transition {
	/// Indicates at what date the object is to be moved or deleted. Should be in GMT
	/// ISO 8601 Format.
	pub date: Date,
	/// Indicates the lifetime, in days, of the objects that are subject to the rule.
	/// The value must be a non-zero positive integer.
	pub days: Days,
	/// The class of storage used to store the object.
	pub storage_class: TransitionStorageClass,
}

/// Write Transition contents to a SignedRequest via headers
struct TransitionWriter;
impl TransitionWriter {
	fn write_params(request: &mut request, name: &str, obj: &Transition) {
		DateWriter::write_params(params, &(prefix.to_string() + "Date"), &obj.date);
		DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
		TransitionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
	}
}
//typeparser

/// Parse QueueConfigurationDeprecated from response
struct QueueConfigurationDeprecatedParser;
impl QueueConfigurationDeprecatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<QueueConfigurationDeprecated, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = QueueConfigurationDeprecated::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Queue is payload.
			if current_name == "Queue" {
				obj.queue = try!(QueueArnParser::parse_xml("Queue", stack));
				continue;
			}
			// heylisten etc: location for Events is payload.
			if current_name == "Event" {
				obj.events = try!(EventListParser::parse_xml("Event", stack));
				continue;
			}
			// heylisten etc: location for Id is payload.
			if current_name == "Id" {
				obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
				continue;
			}
			// heylisten etc: location for Event is payload.
			if current_name == "Event" {
				obj.event = try!(EventParser::parse_xml("Event", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct QueueConfigurationDeprecated {
	pub queue: QueueArn,
	pub events: EventList,
	pub id: NotificationId,
	pub event: Event,
}

/// Write QueueConfigurationDeprecated contents to a SignedRequest via headers
struct QueueConfigurationDeprecatedWriter;
impl QueueConfigurationDeprecatedWriter {
	fn write_params(request: &mut request, name: &str, obj: &QueueConfigurationDeprecated) {
		QueueArnWriter::write_params(params, &(prefix.to_string() + "Queue"), &obj.queue);
		EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
		NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
		EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
	}
}
//typeparser

/// Parse CopySourceSSECustomerKeyMD5 from response
struct CopySourceSSECustomerKeyMD5Parser;
impl CopySourceSSECustomerKeyMD5Parser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceSSECustomerKeyMD5, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceSSECustomerKeyMD5 = String;
/// Write CopySourceSSECustomerKeyMD5 contents to a SignedRequest via headers
struct CopySourceSSECustomerKeyMD5Writer;
impl CopySourceSSECustomerKeyMD5Writer {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceSSECustomerKeyMD5) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse TrimmedS3Error from response
struct TrimmedS3ErrorParser;
impl TrimmedS3ErrorParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TrimmedS3Error, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = TrimmedS3Error::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for VersionId is payload.
			if current_name == "VersionId" {
				obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
				continue;
			}
			// heylisten etc: location for Code is payload.
			if current_name == "Code" {
				obj.code = try!(CodeParser::parse_xml("Code", stack));
				continue;
			}
			// heylisten etc: location for Message is payload.
			if current_name == "Message" {
				obj.message = try!(MessageParser::parse_xml("Message", stack));
				continue;
			}
			// heylisten etc: location for Key is payload.
			if current_name == "Key" {
				obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct TrimmedS3Error {
	pub version_id: ObjectVersionId,
	pub code: Code,
	pub message: TrimmedS3Message,
	pub key: ObjectKey,
}

/// Write TrimmedS3Error contents to a SignedRequest via headers
struct TrimmedS3ErrorWriter;
impl TrimmedS3ErrorWriter {
	fn write_params(request: &mut request, name: &str, obj: &TrimmedS3Error) {
		ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
		CodeWriter::write_params(params, &(prefix.to_string() + "Code"), &obj.code);
		MessageWriter::write_params(params, &(prefix.to_string() + "Message"), &obj.message);
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
	}
}
//typeparser

/// Parse IsLatest from response
struct IsLatestParser;
impl IsLatestParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<IsLatest, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type IsLatest = bool;
/// Write IsLatest contents to a SignedRequest via headers
struct IsLatestWriter;
impl IsLatestWriter {
	fn write_params(request: &mut request, name: &str, obj: &IsLatest) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse MaxUploads from response
struct MaxUploadsParser;
impl MaxUploadsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MaxUploads, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MaxUploads = i32;
/// Write MaxUploads contents to a SignedRequest via headers
struct MaxUploadsWriter;
impl MaxUploadsWriter {
	fn write_params(request: &mut request, name: &str, obj: &MaxUploads) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse RoutingRule from response
struct RoutingRuleParser;
impl RoutingRuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<RoutingRule, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = RoutingRule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Redirect is payload.
			if current_name == "Redirect" {
				obj.redirect = try!(RedirectParser::parse_xml("Redirect", stack));
				continue;
			}
			// heylisten etc: location for Condition is payload.
			if current_name == "Condition" {
				obj.condition = Some(try!(ConditionParser::parse_xml("Condition", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct RoutingRule {
	/// Container for redirect information. You can redirect requests to another host,
	/// to another page, or with another protocol. In the event of an error, you can
	/// can specify a different error code to return.
	pub redirect: Redirect,
	/// A container for describing a condition that must be met for the specified
	/// redirect to apply. For example, 1. If request is for pages in the /docs
	/// folder, redirect to the /documents folder. 2. If request results in HTTP error
	/// 4xx, redirect request to another host where you might process the error.
	pub condition: Option<Condition>,
}

/// Write RoutingRule contents to a SignedRequest via headers
struct RoutingRuleWriter;
impl RoutingRuleWriter {
	fn write_params(request: &mut request, name: &str, obj: &RoutingRule) {
		RedirectWriter::write_params(params, &(prefix.to_string() + "Redirect"), &obj.redirect);
		if let Some(ref obj) = obj.condition {
			 request.add_header("Condition", obj);
		}
	}
}
//typeparser

/// Parse MissingMeta from response
struct MissingMetaParser;
impl MissingMetaParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MissingMeta, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MissingMeta = i32;
/// Write MissingMeta contents to a SignedRequest via headers
struct MissingMetaWriter;
impl MissingMetaWriter {
	fn write_params(request: &mut request, name: &str, obj: &MissingMeta) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse TopicArn from response
struct TopicArnParser;
impl TopicArnParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<TopicArn, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type TopicArn = String;
/// Write TopicArn contents to a SignedRequest via headers
struct TopicArnWriter;
impl TopicArnWriter {
	fn write_params(request: &mut request, name: &str, obj: &TopicArn) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse AllowedOrigins from response
struct AllowedOriginsParser;
impl AllowedOriginsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<AllowedOrigins, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AllowedOrigin" {
			obj.push(try!(AllowedOriginParser::parse_xml("AllowedOrigin", stack)));
		}
		Ok(obj)
	}
}
pub type AllowedOrigins = Vec<AllowedOrigin>;
/// Write AllowedOrigins contents to a SignedRequest via headers
struct AllowedOriginsWriter;
impl AllowedOriginsWriter {
	fn write_params(request: &mut request, name: &str, obj: &AllowedOrigins) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AllowedOriginWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse NoSuchUpload from response
struct NoSuchUploadParser;
impl NoSuchUploadParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NoSuchUpload, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = NoSuchUpload::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// The specified multipart upload does not exist.
#[derive(Debug, Default)]
pub struct NoSuchUpload;

/// Write NoSuchUpload contents to a SignedRequest via headers
struct NoSuchUploadWriter;
impl NoSuchUploadWriter {
	fn write_params(request: &mut request, name: &str, obj: &NoSuchUpload) {
	}
}
//typeparser

/// Parse Code from response
struct CodeParser;
impl CodeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Code, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Code = String;
/// Write Code contents to a SignedRequest via headers
struct CodeWriter;
impl CodeWriter {
	fn write_params(request: &mut request, name: &str, obj: &Code) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse AllowedHeaders from response
struct AllowedHeadersParser;
impl AllowedHeadersParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<AllowedHeaders, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "AllowedHeader" {
			obj.push(try!(AllowedHeaderParser::parse_xml("AllowedHeader", stack)));
		}
		Ok(obj)
	}
}
pub type AllowedHeaders = Vec<AllowedHeader>;
/// Write AllowedHeaders contents to a SignedRequest via headers
struct AllowedHeadersWriter;
impl AllowedHeadersWriter {
	fn write_params(request: &mut request, name: &str, obj: &AllowedHeaders) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			AllowedHeaderWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse ContentMD5 from response
struct ContentMD5Parser;
impl ContentMD5Parser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ContentMD5, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ContentMD5 = String;
/// Write ContentMD5 contents to a SignedRequest via headers
struct ContentMD5Writer;
impl ContentMD5Writer {
	fn write_params(request: &mut request, name: &str, obj: &ContentMD5) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ObjectVersion from response
struct ObjectVersionParser;
impl ObjectVersionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectVersion, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = ObjectVersion::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for LastModified is payload.
			if current_name == "LastModified" {
				obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
				continue;
			}
			// heylisten etc: location for VersionId is payload.
			if current_name == "VersionId" {
				obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
				continue;
			}
			// heylisten etc: location for ETag is payload.
			if current_name == "ETag" {
				obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
				continue;
			}
			// heylisten etc: location for StorageClass is payload.
			if current_name == "StorageClass" {
				obj.storage_class = try!(ObjectVersionStorageClassParser::parse_xml("StorageClass", stack));
				continue;
			}
			// heylisten etc: location for Key is payload.
			if current_name == "Key" {
				obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
				continue;
			}
			// heylisten etc: location for Owner is payload.
			if current_name == "Owner" {
				obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
				continue;
			}
			// heylisten etc: location for IsLatest is payload.
			if current_name == "IsLatest" {
				obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
				continue;
			}
			// heylisten etc: location for Size is payload.
			if current_name == "Size" {
				obj.size = try!(SizeParser::parse_xml("Size", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct ObjectVersion {
	/// Date and time the object was last modified.
	pub last_modified: LastModified,
	/// Version ID of an object.
	pub version_id: ObjectVersionId,
	pub e_tag: ETag,
	/// The class of storage used to store the object.
	pub storage_class: ObjectVersionStorageClass,
	/// The object key.
	pub key: ObjectKey,
	pub owner: Owner,
	/// Specifies whether the object is (true) or is not (false) the latest version of
	/// an object.
	pub is_latest: IsLatest,
	/// Size in bytes of the object.
	pub size: Size,
}

/// Write ObjectVersion contents to a SignedRequest via headers
struct ObjectVersionWriter;
impl ObjectVersionWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectVersion) {
		LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
		ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
		ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
		ObjectVersionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
		OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
		IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
		SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
	}
}
//typeparser

/// Parse MetadataDirective from response
struct MetadataDirectiveParser;
impl MetadataDirectiveParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MetadataDirective, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MetadataDirective = String;
/// Write MetadataDirective contents to a SignedRequest via headers
struct MetadataDirectiveWriter;
impl MetadataDirectiveWriter {
	fn write_params(request: &mut request, name: &str, obj: &MetadataDirective) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Prefix from response
struct PrefixParser;
impl PrefixParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Prefix, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Prefix = String;
/// Write Prefix contents to a SignedRequest via headers
struct PrefixWriter;
impl PrefixWriter {
	fn write_params(request: &mut request, name: &str, obj: &Prefix) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse RestoreObjectOutput from response
struct RestoreObjectOutputParser;
impl RestoreObjectOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<RestoreObjectOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = RestoreObjectOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for RequestCharged is header.
			if current_name == "x-amz-request-charged" {
				obj.request_charged = try!(response.Headers.get("x-amz-request-charged"));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct RestoreObjectOutput {
	pub request_charged: RequestCharged,
}

//typeparser

/// Parse EmailAddress from response
struct EmailAddressParser;
impl EmailAddressParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<EmailAddress, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type EmailAddress = String;
/// Write EmailAddress contents to a SignedRequest via headers
struct EmailAddressWriter;
impl EmailAddressWriter {
	fn write_params(request: &mut request, name: &str, obj: &EmailAddress) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Tagging from response
struct TaggingParser;
impl TaggingParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Tagging, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Tagging::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for TagSet is payload.
			if current_name == "Tag" {
				obj.tag_set = try!(TagSetParser::parse_xml("Tag", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Tagging {
	pub tag_set: TagSet,
}

/// Write Tagging contents to a SignedRequest via headers
struct TaggingWriter;
impl TaggingWriter {
	fn write_params(request: &mut request, name: &str, obj: &Tagging) {
		TagSetWriter::write_params(params, &(prefix.to_string() + "Tag"), &obj.tag_set);
	}
}
//typeparser

/// Parse Marker from response
struct MarkerParser;
impl MarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Marker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Marker = String;
/// Write Marker contents to a SignedRequest via headers
struct MarkerWriter;
impl MarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &Marker) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ObjectCannedACL from response
struct ObjectCannedACLParser;
impl ObjectCannedACLParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectCannedACL, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ObjectCannedACL = String;
/// Write ObjectCannedACL contents to a SignedRequest via headers
struct ObjectCannedACLWriter;
impl ObjectCannedACLWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectCannedACL) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse QueueArn from response
struct QueueArnParser;
impl QueueArnParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<QueueArn, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type QueueArn = String;
/// Write QueueArn contents to a SignedRequest via headers
struct QueueArnWriter;
impl QueueArnWriter {
	fn write_params(request: &mut request, name: &str, obj: &QueueArn) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ObjectList from response
struct ObjectListParser;
impl ObjectListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Object" {
			obj.push(try!(ObjectParser::parse_xml("Object", stack)));
		}
		Ok(obj)
	}
}
pub type ObjectList = Vec<Object>;
/// Write ObjectList contents to a SignedRequest via headers
struct ObjectListWriter;
impl ObjectListWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ObjectWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse HttpErrorCodeReturnedEquals from response
struct HttpErrorCodeReturnedEqualsParser;
impl HttpErrorCodeReturnedEqualsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<HttpErrorCodeReturnedEquals, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type HttpErrorCodeReturnedEquals = String;
/// Write HttpErrorCodeReturnedEquals contents to a SignedRequest via headers
struct HttpErrorCodeReturnedEqualsWriter;
impl HttpErrorCodeReturnedEqualsWriter {
	fn write_params(request: &mut request, name: &str, obj: &HttpErrorCodeReturnedEquals) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse NotificationConfigurationDeprecated from response
struct NotificationConfigurationDeprecatedParser;
impl NotificationConfigurationDeprecatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NotificationConfigurationDeprecated, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = NotificationConfigurationDeprecated::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for CloudFunctionConfiguration is payload.
			if current_name == "CloudFunctionConfiguration" {
				obj.cloud_function_configuration = try!(CloudFunctionConfigurationParser::parse_xml("CloudFunctionConfiguration", stack));
				continue;
			}
			// heylisten etc: location for QueueConfiguration is payload.
			if current_name == "QueueConfiguration" {
				obj.queue_configuration = try!(QueueConfigurationDeprecatedParser::parse_xml("QueueConfiguration", stack));
				continue;
			}
			// heylisten etc: location for TopicConfiguration is payload.
			if current_name == "TopicConfiguration" {
				obj.topic_configuration = try!(TopicConfigurationDeprecatedParser::parse_xml("TopicConfiguration", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct NotificationConfigurationDeprecated {
	pub cloud_function_configuration: CloudFunctionConfiguration,
	pub queue_configuration: QueueConfigurationDeprecated,
	pub topic_configuration: TopicConfigurationDeprecated,
}

/// Write NotificationConfigurationDeprecated contents to a SignedRequest via headers
struct NotificationConfigurationDeprecatedWriter;
impl NotificationConfigurationDeprecatedWriter {
	fn write_params(request: &mut request, name: &str, obj: &NotificationConfigurationDeprecated) {
		CloudFunctionConfigurationWriter::write_params(params, &(prefix.to_string() + "CloudFunctionConfiguration"), &obj.cloud_function_configuration);
		QueueConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "QueueConfiguration"), &obj.queue_configuration);
		TopicConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "TopicConfiguration"), &obj.topic_configuration);
	}
}
//typeparser

/// Parse CORSConfiguration from response
struct CORSConfigurationParser;
impl CORSConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CORSConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = CORSConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for CORSRules is payload.
			if current_name == "CORSRule" {
				obj.cors_rules = try!(CORSRulesParser::parse_xml("CORSRule", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct CORSConfiguration {
	pub cors_rules: CORSRules,
}

/// Write CORSConfiguration contents to a SignedRequest via headers
struct CORSConfigurationWriter;
impl CORSConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &CORSConfiguration) {
		CORSRulesWriter::write_params(params, &(prefix.to_string() + "CORSRule"), &obj.cors_rules);
	}
}
//typeparser

/// Parse LastModified from response
struct LastModifiedParser;
impl LastModifiedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<LastModified, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type LastModified = String;
/// Write LastModified contents to a SignedRequest via headers
struct LastModifiedWriter;
impl LastModifiedWriter {
	fn write_params(request: &mut request, name: &str, obj: &LastModified) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ContentRange from response
struct ContentRangeParser;
impl ContentRangeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ContentRange, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ContentRange = String;
/// Write ContentRange contents to a SignedRequest via headers
struct ContentRangeWriter;
impl ContentRangeWriter {
	fn write_params(request: &mut request, name: &str, obj: &ContentRange) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Grantee from response
struct GranteeParser;
impl GranteeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Grantee, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Grantee::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for EmailAddress is payload.
			if current_name == "EmailAddress" {
				obj.email_address = Some(try!(EmailAddressParser::parse_xml("EmailAddress", stack)));
				continue;
			}
			// heylisten etc: location for Type is payload.
			if current_name == "xsi:type" {
				obj.foo_type = try!(TypeParser::parse_xml("xsi:type", stack));
				continue;
			}
			// heylisten etc: location for DisplayName is payload.
			if current_name == "DisplayName" {
				obj.display_name = Some(try!(DisplayNameParser::parse_xml("DisplayName", stack)));
				continue;
			}
			// heylisten etc: location for ID is payload.
			if current_name == "ID" {
				obj.id = Some(try!(IDParser::parse_xml("ID", stack)));
				continue;
			}
			// heylisten etc: location for URI is payload.
			if current_name == "URI" {
				obj.uri = Some(try!(URIParser::parse_xml("URI", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Grantee {
	/// Email address of the grantee.
	pub email_address: Option<EmailAddress>,
	/// Type of grantee
	pub foo_type: Type,
	/// Screen name of the grantee.
	pub display_name: Option<DisplayName>,
	/// The canonical user ID of the grantee.
	pub id: Option<ID>,
	/// URI of the grantee group.
	pub uri: Option<URI>,
}

/// Write Grantee contents to a SignedRequest via headers
struct GranteeWriter;
impl GranteeWriter {
	fn write_params(request: &mut request, name: &str, obj: &Grantee) {
		if let Some(ref obj) = obj.email_address {
			 request.add_header("EmailAddress", obj);
		}
		TypeWriter::write_params(params, &(prefix.to_string() + "xsi:type"), &obj.foo_type);
		if let Some(ref obj) = obj.display_name {
			 request.add_header("DisplayName", obj);
		}
		if let Some(ref obj) = obj.id {
			 request.add_header("ID", obj);
		}
		if let Some(ref obj) = obj.uri {
			 request.add_header("URI", obj);
		}
	}
}
//typeparser

/// Parse ExpirationStatus from response
struct ExpirationStatusParser;
impl ExpirationStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ExpirationStatus, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ExpirationStatus = String;
/// Write ExpirationStatus contents to a SignedRequest via headers
struct ExpirationStatusWriter;
impl ExpirationStatusWriter {
	fn write_params(request: &mut request, name: &str, obj: &ExpirationStatus) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CopySourceIfUnmodifiedSince from response
struct CopySourceIfUnmodifiedSinceParser;
impl CopySourceIfUnmodifiedSinceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceIfUnmodifiedSince, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceIfUnmodifiedSince = String;
/// Write CopySourceIfUnmodifiedSince contents to a SignedRequest via headers
struct CopySourceIfUnmodifiedSinceWriter;
impl CopySourceIfUnmodifiedSinceWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceIfUnmodifiedSince) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse RoutingRules from response
struct RoutingRulesParser;
impl RoutingRulesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<RoutingRules, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "RoutingRule" {
			obj.push(try!(RoutingRuleParser::parse_xml("RoutingRule", stack)));
		}
		Ok(obj)
	}
}
pub type RoutingRules = Vec<RoutingRule>;
/// Write RoutingRules contents to a SignedRequest via headers
struct RoutingRulesWriter;
impl RoutingRulesWriter {
	fn write_params(request: &mut request, name: &str, obj: &RoutingRules) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			RoutingRuleWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse Parts from response
struct PartsParser;
impl PartsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Parts, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Part" {
			obj.push(try!(PartParser::parse_xml("Part", stack)));
		}
		Ok(obj)
	}
}
pub type Parts = Vec<Part>;
/// Write Parts contents to a SignedRequest via headers
struct PartsWriter;
impl PartsWriter {
	fn write_params(request: &mut request, name: &str, obj: &Parts) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			PartWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse LambdaFunctionConfigurationList from response
struct LambdaFunctionConfigurationListParser;
impl LambdaFunctionConfigurationListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<LambdaFunctionConfigurationList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "LambdaFunctionConfiguration" {
			obj.push(try!(LambdaFunctionConfigurationParser::parse_xml("LambdaFunctionConfiguration", stack)));
		}
		Ok(obj)
	}
}
pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;
/// Write LambdaFunctionConfigurationList contents to a SignedRequest via headers
struct LambdaFunctionConfigurationListWriter;
impl LambdaFunctionConfigurationListWriter {
	fn write_params(request: &mut request, name: &str, obj: &LambdaFunctionConfigurationList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			LambdaFunctionConfigurationWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse ServerSideEncryption from response
struct ServerSideEncryptionParser;
impl ServerSideEncryptionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ServerSideEncryption, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ServerSideEncryption = String;
/// Write ServerSideEncryption contents to a SignedRequest via headers
struct ServerSideEncryptionWriter;
impl ServerSideEncryptionWriter {
	fn write_params(request: &mut request, name: &str, obj: &ServerSideEncryption) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Policy from response
struct PolicyParser;
impl PolicyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Policy, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Policy = String;
/// Write Policy contents to a SignedRequest via headers
struct PolicyWriter;
impl PolicyWriter {
	fn write_params(request: &mut request, name: &str, obj: &Policy) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse NoncurrentVersionExpiration from response
struct NoncurrentVersionExpirationParser;
impl NoncurrentVersionExpirationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NoncurrentVersionExpiration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = NoncurrentVersionExpiration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for NoncurrentDays is payload.
			if current_name == "NoncurrentDays" {
				obj.noncurrent_days = try!(DaysParser::parse_xml("NoncurrentDays", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Specifies when noncurrent object versions expire. Upon expiration, Amazon S3
/// permanently deletes the noncurrent object versions. You set this lifecycle
/// configuration action on a bucket that has versioning enabled (or suspended) to
/// request that Amazon S3 delete noncurrent object versions at a specific period
/// in the object's lifetime.
#[derive(Debug, Default)]
pub struct NoncurrentVersionExpiration {
	/// Specifies the number of days an object is noncurrent before Amazon S3 can
	/// perform the associated action. For information about the noncurrent days
	/// calculations, see [How Amazon S3 Calculates When an Object Became
	/// Noncurrent](/AmazonS3/latest/dev/s3-access-control.html) in the Amazon Simple
	/// Storage Service Developer Guide.
	pub noncurrent_days: Days,
}

/// Write NoncurrentVersionExpiration contents to a SignedRequest via headers
struct NoncurrentVersionExpirationWriter;
impl NoncurrentVersionExpirationWriter {
	fn write_params(request: &mut request, name: &str, obj: &NoncurrentVersionExpiration) {
		DaysWriter::write_params(params, &(prefix.to_string() + "NoncurrentDays"), &obj.noncurrent_days);
	}
}
//typeparser

/// Parse NextUploadIdMarker from response
struct NextUploadIdMarkerParser;
impl NextUploadIdMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NextUploadIdMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type NextUploadIdMarker = String;
/// Write NextUploadIdMarker contents to a SignedRequest via headers
struct NextUploadIdMarkerWriter;
impl NextUploadIdMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &NextUploadIdMarker) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ContentDisposition from response
struct ContentDispositionParser;
impl ContentDispositionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ContentDisposition, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ContentDisposition = String;
/// Write ContentDisposition contents to a SignedRequest via headers
struct ContentDispositionWriter;
impl ContentDispositionWriter {
	fn write_params(request: &mut request, name: &str, obj: &ContentDisposition) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse MetadataKey from response
struct MetadataKeyParser;
impl MetadataKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MetadataKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MetadataKey = String;
/// Write MetadataKey contents to a SignedRequest via headers
struct MetadataKeyWriter;
impl MetadataKeyWriter {
	fn write_params(request: &mut request, name: &str, obj: &MetadataKey) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ResponseContentEncoding from response
struct ResponseContentEncodingParser;
impl ResponseContentEncodingParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ResponseContentEncoding, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ResponseContentEncoding = String;
/// Write ResponseContentEncoding contents to a SignedRequest via headers
struct ResponseContentEncodingWriter;
impl ResponseContentEncodingWriter {
	fn write_params(request: &mut request, name: &str, obj: &ResponseContentEncoding) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse UploadIdMarker from response
struct UploadIdMarkerParser;
impl UploadIdMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<UploadIdMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type UploadIdMarker = String;
/// Write UploadIdMarker contents to a SignedRequest via headers
struct UploadIdMarkerWriter;
impl UploadIdMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &UploadIdMarker) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Buckets from response
struct BucketsParser;
impl BucketsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Buckets, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Bucket" {
			obj.push(try!(BucketParser::parse_xml("Bucket", stack)));
		}
		Ok(obj)
	}
}
pub type Buckets = Vec<Bucket>;
/// Write Buckets contents to a SignedRequest via headers
struct BucketsWriter;
impl BucketsWriter {
	fn write_params(request: &mut request, name: &str, obj: &Buckets) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			BucketWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse HostName from response
struct HostNameParser;
impl HostNameParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<HostName, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type HostName = String;
/// Write HostName contents to a SignedRequest via headers
struct HostNameWriter;
impl HostNameWriter {
	fn write_params(request: &mut request, name: &str, obj: &HostName) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse GrantWrite from response
struct GrantWriteParser;
impl GrantWriteParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<GrantWrite, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type GrantWrite = String;
/// Write GrantWrite contents to a SignedRequest via headers
struct GrantWriteWriter;
impl GrantWriteWriter {
	fn write_params(request: &mut request, name: &str, obj: &GrantWrite) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ReplaceKeyWith from response
struct ReplaceKeyWithParser;
impl ReplaceKeyWithParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ReplaceKeyWith, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ReplaceKeyWith = String;
/// Write ReplaceKeyWith contents to a SignedRequest via headers
struct ReplaceKeyWithWriter;
impl ReplaceKeyWithWriter {
	fn write_params(request: &mut request, name: &str, obj: &ReplaceKeyWith) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ObjectKey from response
struct ObjectKeyParser;
impl ObjectKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ObjectKey = String;
/// Write ObjectKey contents to a SignedRequest via headers
struct ObjectKeyWriter;
impl ObjectKeyWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectKey) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse GetBucketPolicyOutput from response
struct GetBucketPolicyOutputParser;
impl GetBucketPolicyOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<GetBucketPolicyOutput, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = GetBucketPolicyOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Policy is payload.
			if current_name == "Policy" {
				obj.policy = try!(PolicyParser::parse_xml("Policy", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct GetBucketPolicyOutput {
	/// The bucket policy as a JSON document.
	pub policy: Policy,
}

//typeparser

/// Parse MaxAgeSeconds from response
struct MaxAgeSecondsParser;
impl MaxAgeSecondsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MaxAgeSeconds, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MaxAgeSeconds = i32;
/// Write MaxAgeSeconds contents to a SignedRequest via headers
struct MaxAgeSecondsWriter;
impl MaxAgeSecondsWriter {
	fn write_params(request: &mut request, name: &str, obj: &MaxAgeSeconds) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse CopySourceRange from response
struct CopySourceRangeParser;
impl CopySourceRangeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceRange, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceRange = String;
/// Write CopySourceRange contents to a SignedRequest via headers
struct CopySourceRangeWriter;
impl CopySourceRangeWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceRange) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Initiator from response
struct InitiatorParser;
impl InitiatorParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Initiator, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Initiator::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for DisplayName is payload.
			if current_name == "DisplayName" {
				obj.display_name = try!(DisplayNameParser::parse_xml("DisplayName", stack));
				continue;
			}
			// heylisten etc: location for ID is payload.
			if current_name == "ID" {
				obj.id = try!(IDParser::parse_xml("ID", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Initiator {
	/// Name of the Principal.
	pub display_name: DisplayName,
	/// If the principal is an AWS account, it provides the Canonical User ID. If the
	/// principal is an IAM User, it provides a user ARN value.
	pub id: ID,
}

/// Write Initiator contents to a SignedRequest via headers
struct InitiatorWriter;
impl InitiatorWriter {
	fn write_params(request: &mut request, name: &str, obj: &Initiator) {
		DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), &obj.display_name);
		IDWriter::write_params(params, &(prefix.to_string() + "ID"), &obj.id);
	}
}
//typeparser

/// Parse CommonPrefix from response
struct CommonPrefixParser;
impl CommonPrefixParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CommonPrefix, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = CommonPrefix::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Prefix is payload.
			if current_name == "Prefix" {
				obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct CommonPrefix {
	pub prefix: Prefix,
}

/// Write CommonPrefix contents to a SignedRequest via headers
struct CommonPrefixWriter;
impl CommonPrefixWriter {
	fn write_params(request: &mut request, name: &str, obj: &CommonPrefix) {
		PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
	}
}
//typeparser

/// Parse NoSuchKey from response
struct NoSuchKeyParser;
impl NoSuchKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NoSuchKey, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = NoSuchKey::default();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// The specified key does not exist.
#[derive(Debug, Default)]
pub struct NoSuchKey;

/// Write NoSuchKey contents to a SignedRequest via headers
struct NoSuchKeyWriter;
impl NoSuchKeyWriter {
	fn write_params(request: &mut request, name: &str, obj: &NoSuchKey) {
	}
}
//typeparser

/// Parse ObjectVersionList from response
struct ObjectVersionListParser;
impl ObjectVersionListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectVersionList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "ObjectVersion" {
			obj.push(try!(ObjectVersionParser::parse_xml("ObjectVersion", stack)));
		}
		Ok(obj)
	}
}
pub type ObjectVersionList = Vec<ObjectVersion>;
/// Write ObjectVersionList contents to a SignedRequest via headers
struct ObjectVersionListWriter;
impl ObjectVersionListWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectVersionList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			ObjectVersionWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse MultipartUploadList from response
struct MultipartUploadListParser;
impl MultipartUploadListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MultipartUploadList, XmlParseError> {
		 // list_parser
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "MultipartUpload" {
			obj.push(try!(MultipartUploadParser::parse_xml("MultipartUpload", stack)));
		}
		Ok(obj)
	}
}
pub type MultipartUploadList = Vec<MultipartUpload>;
/// Write MultipartUploadList contents to a SignedRequest via headers
struct MultipartUploadListWriter;
impl MultipartUploadListWriter {
	fn write_params(request: &mut request, name: &str, obj: &MultipartUploadList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			MultipartUploadWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
//typeparser

/// Parse AllowedHeader from response
struct AllowedHeaderParser;
impl AllowedHeaderParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<AllowedHeader, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type AllowedHeader = String;
/// Write AllowedHeader contents to a SignedRequest via headers
struct AllowedHeaderWriter;
impl AllowedHeaderWriter {
	fn write_params(request: &mut request, name: &str, obj: &AllowedHeader) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Bucket from response
struct BucketParser;
impl BucketParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Bucket, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = Bucket::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for CreationDate is payload.
			if current_name == "CreationDate" {
				obj.creation_date = try!(CreationDateParser::parse_xml("CreationDate", stack));
				continue;
			}
			// heylisten etc: location for Name is payload.
			if current_name == "Name" {
				obj.name = try!(BucketNameParser::parse_xml("Name", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Bucket {
	/// Date the bucket was created.
	pub creation_date: CreationDate,
	/// The name of the bucket.
	pub name: BucketName,
}

/// Write Bucket contents to a SignedRequest via headers
struct BucketWriter;
impl BucketWriter {
	fn write_params(request: &mut request, name: &str, obj: &Bucket) {
		CreationDateWriter::write_params(params, &(prefix.to_string() + "CreationDate"), &obj.creation_date);
		BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
	}
}
//typeparser

/// Parse URI from response
struct URIParser;
impl URIParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<URI, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type URI = String;
/// Write URI contents to a SignedRequest via headers
struct URIWriter;
impl URIWriter {
	fn write_params(request: &mut request, name: &str, obj: &URI) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse RequestCharged from response
struct RequestChargedParser;
impl RequestChargedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<RequestCharged, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// If present, indicates that the requester was successfully charged for the
/// request.
pub type RequestCharged = String;
/// Write RequestCharged contents to a SignedRequest via headers
struct RequestChargedWriter;
impl RequestChargedWriter {
	fn write_params(request: &mut request, name: &str, obj: &RequestCharged) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse Delimiter from response
struct DelimiterParser;
impl DelimiterParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Delimiter, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Delimiter = String;
/// Write Delimiter contents to a SignedRequest via headers
struct DelimiterWriter;
impl DelimiterWriter {
	fn write_params(request: &mut request, name: &str, obj: &Delimiter) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ResponseContentType from response
struct ResponseContentTypeParser;
impl ResponseContentTypeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ResponseContentType, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ResponseContentType = String;
/// Write ResponseContentType contents to a SignedRequest via headers
struct ResponseContentTypeWriter;
impl ResponseContentTypeWriter {
	fn write_params(request: &mut request, name: &str, obj: &ResponseContentType) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse LifecycleConfiguration from response
struct LifecycleConfigurationParser;
impl LifecycleConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<LifecycleConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = LifecycleConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for Rules is payload.
			if current_name == "Rule" {
				obj.rules = try!(RulesParser::parse_xml("Rule", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct LifecycleConfiguration {
	pub rules: Rules,
}

/// Write LifecycleConfiguration contents to a SignedRequest via headers
struct LifecycleConfigurationWriter;
impl LifecycleConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &LifecycleConfiguration) {
		RulesWriter::write_params(params, &(prefix.to_string() + "Rule"), &obj.rules);
	}
}
//typeparser

/// Parse Expiration from response
struct ExpirationParser;
impl ExpirationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<Expiration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type Expiration = String;
/// Write Expiration contents to a SignedRequest via headers
struct ExpirationWriter;
impl ExpirationWriter {
	fn write_params(request: &mut request, name: &str, obj: &Expiration) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse IfMatch from response
struct IfMatchParser;
impl IfMatchParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<IfMatch, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type IfMatch = String;
/// Write IfMatch contents to a SignedRequest via headers
struct IfMatchWriter;
impl IfMatchWriter {
	fn write_params(request: &mut request, name: &str, obj: &IfMatch) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ResponseExpires from response
struct ResponseExpiresParser;
impl ResponseExpiresParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ResponseExpires, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ResponseExpires = String;
/// Write ResponseExpires contents to a SignedRequest via headers
struct ResponseExpiresWriter;
impl ResponseExpiresWriter {
	fn write_params(request: &mut request, name: &str, obj: &ResponseExpires) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse LambdaFunctionConfiguration from response
struct LambdaFunctionConfigurationParser;
impl LambdaFunctionConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<LambdaFunctionConfiguration, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = LambdaFunctionConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for LambdaFunctionArn is payload.
			if current_name == "CloudFunction" {
				obj.lambda_function_arn = try!(LambdaFunctionArnParser::parse_xml("CloudFunction", stack));
				continue;
			}
			// heylisten etc: location for Id is payload.
			if current_name == "Id" {
				obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
				continue;
			}
			// heylisten etc: location for Events is payload.
			if current_name == "Event" {
				obj.events = try!(EventListParser::parse_xml("Event", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Container for specifying the AWS Lambda notification configuration.
#[derive(Debug, Default)]
pub struct LambdaFunctionConfiguration {
	/// Lambda cloud function ARN that Amazon S3 can invoke when it detects events of
	/// the specified type.
	pub lambda_function_arn: LambdaFunctionArn,
	pub id: Option<NotificationId>,
	pub events: EventList,
}

/// Write LambdaFunctionConfiguration contents to a SignedRequest via headers
struct LambdaFunctionConfigurationWriter;
impl LambdaFunctionConfigurationWriter {
	fn write_params(request: &mut request, name: &str, obj: &LambdaFunctionConfiguration) {
		LambdaFunctionArnWriter::write_params(params, &(prefix.to_string() + "CloudFunction"), &obj.lambda_function_arn);
		if let Some(ref obj) = obj.id {
			 request.add_header("Id", obj);
		}
		EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
	}
}
//typeparser

/// Parse EncodingType from response
struct EncodingTypeParser;
impl EncodingTypeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<EncodingType, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Requests Amazon S3 to encode the object keys in the response and specifies the
/// encoding method to use. An object key may contain any Unicode character;
/// however, XML 1.0 parser cannot parse some characters, such as characters with
/// an ASCII value from 0 to 10. For characters that are not supported in XML 1.0,
/// you can add this parameter to request that Amazon S3 encode the keys in the
/// response.
pub type EncodingType = String;
/// Write EncodingType contents to a SignedRequest via headers
struct EncodingTypeWriter;
impl EncodingTypeWriter {
	fn write_params(request: &mut request, name: &str, obj: &EncodingType) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ID from response
struct IDParser;
impl IDParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ID, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ID = String;
/// Write ID contents to a SignedRequest via headers
struct IDWriter;
impl IDWriter {
	fn write_params(request: &mut request, name: &str, obj: &ID) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse PartNumberMarker from response
struct PartNumberMarkerParser;
impl PartNumberMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<PartNumberMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type PartNumberMarker = i32;
/// Write PartNumberMarker contents to a SignedRequest via headers
struct PartNumberMarkerWriter;
impl PartNumberMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &PartNumberMarker) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse DeleteMarkerVersionId from response
struct DeleteMarkerVersionIdParser;
impl DeleteMarkerVersionIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<DeleteMarkerVersionId, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type DeleteMarkerVersionId = String;
/// Write DeleteMarkerVersionId contents to a SignedRequest via headers
struct DeleteMarkerVersionIdWriter;
impl DeleteMarkerVersionIdWriter {
	fn write_params(request: &mut request, name: &str, obj: &DeleteMarkerVersionId) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CopySourceIfMatch from response
struct CopySourceIfMatchParser;
impl CopySourceIfMatchParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CopySourceIfMatch, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CopySourceIfMatch = String;
/// Write CopySourceIfMatch contents to a SignedRequest via headers
struct CopySourceIfMatchWriter;
impl CopySourceIfMatchWriter {
	fn write_params(request: &mut request, name: &str, obj: &CopySourceIfMatch) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ContentType from response
struct ContentTypeParser;
impl ContentTypeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ContentType, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ContentType = String;
/// Write ContentType contents to a SignedRequest via headers
struct ContentTypeWriter;
impl ContentTypeWriter {
	fn write_params(request: &mut request, name: &str, obj: &ContentType) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse NextPartNumberMarker from response
struct NextPartNumberMarkerParser;
impl NextPartNumberMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NextPartNumberMarker, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type NextPartNumberMarker = i32;
/// Write NextPartNumberMarker contents to a SignedRequest via headers
struct NextPartNumberMarkerWriter;
impl NextPartNumberMarkerWriter {
	fn write_params(request: &mut request, name: &str, obj: &NextPartNumberMarker) {
		request.add_header(name, &obj.to_string());
	}
}
//typeparser

/// Parse ResponseCacheControl from response
struct ResponseCacheControlParser;
impl ResponseCacheControlParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ResponseCacheControl, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ResponseCacheControl = String;
/// Write ResponseCacheControl contents to a SignedRequest via headers
struct ResponseCacheControlWriter;
impl ResponseCacheControlWriter {
	fn write_params(request: &mut request, name: &str, obj: &ResponseCacheControl) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ETag from response
struct ETagParser;
impl ETagParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ETag, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type ETag = String;
/// Write ETag contents to a SignedRequest via headers
struct ETagWriter;
impl ETagWriter {
	fn write_params(request: &mut request, name: &str, obj: &ETag) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse ObjectIdentifier from response
struct ObjectIdentifierParser;
impl ObjectIdentifierParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<ObjectIdentifier, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // struct_parser
		let mut obj = ObjectIdentifier::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			// heylisten etc: location for VersionId is payload.
			if current_name == "VersionId" {
				obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("VersionId", stack)));
				continue;
			}
			// heylisten etc: location for Key is payload.
			if current_name == "Key" {
				obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct ObjectIdentifier {
	/// VersionId for the specific version of the object to delete.
	pub version_id: Option<ObjectVersionId>,
	/// Key name of the object to delete.
	pub key: ObjectKey,
}

/// Write ObjectIdentifier contents to a SignedRequest via headers
struct ObjectIdentifierWriter;
impl ObjectIdentifierWriter {
	fn write_params(request: &mut request, name: &str, obj: &ObjectIdentifier) {
		if let Some(ref obj) = obj.version_id {
			 request.add_header("VersionId", obj);
		}
		ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
	}
}
//typeparser

/// Parse NotificationId from response
struct NotificationIdParser;
impl NotificationIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<NotificationId, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Optional unique identifier for configurations in a notification configuration.
/// If you don't provide one, Amazon S3 will assign an ID.
pub type NotificationId = String;
/// Write NotificationId contents to a SignedRequest via headers
struct NotificationIdWriter;
impl NotificationIdWriter {
	fn write_params(request: &mut request, name: &str, obj: &NotificationId) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse CreationDate from response
struct CreationDateParser;
impl CreationDateParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<CreationDate, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type CreationDate = String;
/// Write CreationDate contents to a SignedRequest via headers
struct CreationDateWriter;
impl CreationDateWriter {
	fn write_params(request: &mut request, name: &str, obj: &CreationDate) {
		request.add_header(name, obj);
	}
}
//typeparser

/// Parse MaxParts from response
struct MaxPartsParser;
impl MaxPartsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<MaxParts, XmlParseError> {
		try!(start_element(tag_name, stack));
		 // primitive_parser
		let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
pub type MaxParts = i32;
/// Write MaxParts contents to a SignedRequest via headers
struct MaxPartsWriter;
impl MaxPartsWriter {
	fn write_params(request: &mut request, name: &str, obj: &MaxParts) {
		request.add_header(name, &obj.to_string());
	}
}
pub struct TrimmedS3<'a> {
	creds: &'a AWSCredentials,
	region: &'a str
}

impl<'a> TrimmedS3<'a> { 
	pub fn new(creds: &'a AWSCredentials, region: &'a str) -> TrimmedS3<'a> {
		TrimmedS3 { creds: creds, region: region }
	}
	/// Creates a copy of an object that is already stored in Amazon S3.
	pub fn copy_object(&self, input: &CopyObjectRequest) -> Result<CopyObjectOutput, AWSError> {
		let mut uri = String::from("/");
		uri = uri +  &input.key.to_string();
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		CopyObjectRequestWriter::write_params(&mut request, "", &input);
		let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
		request.set_hostname(Some(hostname));

		let result = request.sign_and_execute(&self.creds);
		let status = result.status.to_u16();

		match status {
			200 => { 
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next();

				Ok(try!(CopyObjectOutputParser::parse_response("CopyObjectOutput", &result, &mut stack)))
			}
			_ => { 
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();
				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in copy_object"))
			 }
		}
	}
}
