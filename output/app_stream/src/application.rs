

/// This resource creates an application. Applications store the details about how to launch applications on streaming instances. This is only supported for Elastic fleets.
#[derive(Default, serde::Serialize)]
pub struct CfnApplication {


    /// The display name of the application. This name is visible to users in the application catalog.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,


    /// The launch path of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchPath")]
    pub launch_path: String,


    /// The description of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// The working directory of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,


    /// The launch parameters of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchParameters")]
    pub launch_parameters: Option<String>,


    /// The instance families the application supports.
    ///
    /// Allowed Values: GENERAL_PURPOSE | GRAPHICS_G4
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceFamilies")]
    pub instance_families: Vec<String>,


    /// The app block ARN with which the application should be associated.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws(?:\-cn|\-iso\-b|\-iso|\-us\-gov)?:[A-Za-z0-9][A-Za-z0-9_/.-]{0,62}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9][A-Za-z0-9:_/+=,@.\\-]{0,1023}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppBlockArn")]
    pub app_block_arn: String,


    /// The platforms the application supports.
    /// 
    /// Allowed Values: WINDOWS_SERVER_2019 | AMAZON_LINUX2
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 4
    ///
    /// Update requires: Replacement
    #[serde(rename = "Platforms")]
    pub platforms: Vec<String>,


    /// The name of the application. This name is visible to users when a name is not specified in the     DisplayName property.
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,100}$
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// The tags of the application.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// The icon S3 location of the application.
    ///
    /// Required: Yes
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "IconS3Location")]
    pub icon_s3_location: S3Location,


    /// A list of attributes to delete from an application.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributesToDelete")]
    pub attributes_to_delete: Option<Vec<String>>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The S3 location of the application icon.
#[derive(Default, serde::Serialize)]
pub struct S3Location {


    /// 
    /// The S3 bucket of the S3 object.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,


    /// 
    /// The S3 key of the S3 object.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}
