
pub mod cfn_access_point {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessPoint {
    /// Virtual Private Cloud (VPC) from which requests can be made to the AccessPoint.
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: VpcConfiguration,
    /// The Amazon Resource Name (ARN) of the bucket you want to associate this AccessPoint with.
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// The access point policy associated with this access point.
    #[serde(rename = "Policy")]
    pub policy: Option<()>,
    /// A name for the AccessPoint.
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct VpcConfiguration {
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

}


}

pub mod cfn_bucket {

#[derive(serde::Serialize, Default)]
pub struct CfnBucket {
    /// The id of the customer outpost on which the bucket resides.
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,
    /// A name for the bucket.
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    /// An arbitrary set of tags (key-value pairs) for this S3Outposts bucket.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Rules that define how Amazon S3Outposts manages objects during their lifetime.
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: Option<LifecycleConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct FilterTag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type FilterPrefix = String;pub type iso8601UTC = String;
#[derive(serde::Serialize, Default)]
pub struct AbortIncompleteMultipartUpload {
    #[serde(rename = "DaysAfterInitiation")]
    pub days_after_initiation: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Rule {
    #[serde(rename = "ExpirationInDays")]
    pub expiration_in_days: Option<usize>,
    #[serde(rename = "Filter")]
    pub filter: Option<()>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "ExpirationDate")]
    pub expiration_date: Option<iso8601UTC>,
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterAndOperator {

}

#[derive(serde::Serialize, Default)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,

}


}

pub mod cfn_bucket_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnBucketPolicy {
    /// The Amazon Resource Name (ARN) of the specified bucket.
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// A policy document containing permissions to add to the specified bucket.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),

}



}

pub mod cfn_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnEndpoint {
    /// The ID of the subnet in the selected VPC. The subnet must belong to the Outpost.
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// The type of access for the on-premise network connectivity for the Outpost endpoint. To access endpoint from an on-premises network, you must specify the access type and provide the customer owned Ipv4 pool.
    #[serde(rename = "AccessType")]
    pub access_type: Option<String>,
    /// The ID of the security group to use with the endpoint.
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: String,
    /// The id of the customer outpost on which the bucket resides.
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,
    /// The ID of the customer-owned IPv4 pool for the Endpoint. IP addresses will be allocated from this pool for the endpoint.
    #[serde(rename = "CustomerOwnedIpv4Pool")]
    pub customer_owned_ipv4_pool: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct NetworkInterface {
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,

}
pub type iso8601UTC = String;

}
