

/// The AWS::Glue::Schema is an AWS Glue resource type that manages schemas in the AWS Glue Schema Registry.
#[derive(Default, serde::Serialize)]
pub struct CfnSchema {


    /// 
    /// Specify the VersionNumber or the IsLatest for setting the checkpoint for the schema. This is only required for updating a checkpoint.
    /// 
    /// Required: No
    ///
    /// Type: SchemaVersion
    ///
    /// Update requires: No interruption
    #[serde(rename = "CheckpointVersion")]
    pub checkpoint_version: Option<SchemaVersion>,


    /// 
    /// The registry where a schema is stored.
    /// 
    /// Required: No
    ///
    /// Type: Registry
    ///
    /// Update requires: Replacement
    #[serde(rename = "Registry")]
    pub registry: Option<Registry>,


    /// 
    /// Name of the schema to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark. No whitespace.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A description of the schema if specified when created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// AWS tags that contain a key value pair and may be searched by console, command line, or API.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The schema definition using the DataFormat setting for SchemaName.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaDefinition")]
    pub schema_definition: String,


    /// 
    /// The data format of the schema definition. Currently only AVRO is supported.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataFormat")]
    pub data_format: String,


    /// 
    /// The compatibility mode of the schema.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compatibility")]
    pub compatibility: String,

}


/// Specifies the version of a schema.
#[derive(Default, serde::Serialize)]
pub struct SchemaVersion {


    /// 
    /// The version number of the schema.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionNumber")]
    pub version_number: Option<i64>,


    /// 
    /// Indicates if this version is the latest version of the schema.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsLatest")]
    pub is_latest: Option<bool>,

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


/// Specifies a registry in the AWS Glue Schema Registry.
#[derive(Default, serde::Serialize)]
pub struct Registry {


    /// 
    /// The name of the registry.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the registry.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}