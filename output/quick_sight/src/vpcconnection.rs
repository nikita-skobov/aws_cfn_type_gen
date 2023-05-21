

/// Creates a new VPC connection.
#[derive(Default, serde::Serialize)]
pub struct CfnVPCConnection {


    /// 
    /// The availability status of the VPC connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVAILABLE | PARTIALLY_AVAILABLE | UNAVAILABLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityStatus")]
    pub availability_status: Option<String>,


    /// 
    /// A list of IP addresses of DNS resolver endpoints for the VPC connection.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsResolvers")]
    pub dns_resolvers: Option<Vec<String>>,


    /// 
    /// The AWS account ID of the account where you want to create a new VPC 			connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,


    /// 
    /// A map of the key-value pairs for the resource tag or tags assigned to the VPC 			connection.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The display name for the VPC connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The ID of the VPC connection that you're creating. This ID is a unique identifier for each AWS Region in an         AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "VPCConnectionId")]
    pub vpcconnection_id: Option<String>,


    /// 
    /// The Amazon EC2 security group IDs associated with the VPC connection.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The ARN of the         IAM role associated with the VPC       connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// A list of subnet IDs for the VPC connection.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 15
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

}


/// The structure that contains information about a network interface.
#[derive(Default, serde::Serialize)]
pub struct NetworkInterface {


    /// 
    /// The availability zone that the network interface resides in.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The network interface ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^eni-[0-9a-z]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,


    /// 
    /// The subnet ID associated with the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^subnet-[0-9a-z]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


    /// 
    /// The status of the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ATTACHMENT_FAILED_ROLLBACK_FAILED | AVAILABLE | CREATING | CREATION_FAILED | DELETED | DELETING | DELETION_FAILED | DELETION_SCHEDULED | UPDATE_FAILED | UPDATING
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


    /// 
    /// An error message.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorMessage")]
    pub error_message: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}
