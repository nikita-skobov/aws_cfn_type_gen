

/// The AWS::SageMaker::DeviceFleet resource is an Amazon SageMaker resource       type that allows you to create a DeviceFleet that manages your SageMaker Edge Manager       Devices. You must register your devices against the DeviceFleet       separately.
#[derive(Default, serde::Serialize)]
pub struct CfnDeviceFleet {


    /// 
    /// An array of key-value pairs that contain metadata to help you categorize and organize       your device fleets. Each tag consists of a key and a value, both of which you       define.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The output configuration for storing sample data collected by the fleet.
    /// 
    /// Required: Yes
    ///
    /// Type: EdgeOutputConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputConfig")]
    pub output_config: EdgeOutputConfig,


    /// 
    /// The Amazon Resource Name (ARN) that has access to AWS Internet of       Things (IoT).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:aws[a-z\-]*:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// Name of the device fleet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceFleetName")]
    pub device_fleet_name: String,


    /// 
    /// A description of the fleet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

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


/// The output configuration for storing sample data collected by the fleet.
#[derive(Default, serde::Serialize)]
pub struct EdgeOutputConfig {


    /// 
    /// The AWS Key Management Service (AWS KMS) key that       Amazon SageMaker uses to encrypt data on the storage volume after compilation job. If       you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3       for your role's account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The Amazon Simple Storage (S3) bucket URI.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3OutputLocation")]
    pub s3_output_location: String,

}
