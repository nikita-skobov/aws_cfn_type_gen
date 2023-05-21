

/// Creates an AWS AppConfig extension. An extension augments your ability to inject     logic or behavior at different points during the AWS AppConfig workflow of creating     or deploying a configuration.
///
/// You can create your own extensions or use the AWS authored extensions provided by       AWS AppConfig. For an AWS AppConfig extension that uses       AWS Lambda, you must create a Lambda function to perform any     computation and processing defined in the extension. If you plan to create custom versions     of the AWS authored notification extensions, you only need to specify an Amazon Resource     Name (ARN) in the Uri field for the new extension version.
///
/// For more information about extensions, see Working with        AWS AppConfig extensions in the             AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnExtension {


    /// 
    /// Adds one or more tags for the specified extension. Tags are metadata that help you     categorize resources in different ways, for example, by purpose, owner, or environment.     Each tag consists of a key and an optional value, both of which you define.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A name for the extension. Each extension name in your account must be unique. Extension     versions use the same name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The parameters accepted by the extension. You specify parameter values when you     associate the extension to an AWS AppConfig resource by using the       CreateExtensionAssociation API action. For AWS Lambda extension     actions, these parameters are included in the Lambda request object.
    /// 
    /// Required: No
    ///
    /// Type: Map of Parameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<std::collections::HashMap<String, Parameter>>,


    /// 
    /// You can omit this field when you create an extension. When you create a new version,     specify the most recent current version number. For example, you create version 3, enter 2     for this field.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "LatestVersionNumber")]
    pub latest_version_number: Option<i64>,


    /// 
    /// The actions defined in the extension.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: serde_json::Value,


    /// 
    /// Information about the extension.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



impl cfn_resources::CfnResource for CfnExtension {
    fn type_string() -> &'static str {
        "AWS::AppConfig::Extension"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A value such as an Amazon Resource Name (ARN) or an Amazon Simple Notification Service topic entered     in an extension when invoked. Parameter values are specified in an extension association.     For more information about extensions, see Working with        AWS AppConfig extensions in the             AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Parameter {


    /// 
    /// A parameter value must be specified in the extension association.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Required")]
    pub required: bool,


    /// 
    /// Information about the parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
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
#[derive(Clone, Debug, Default, serde::Serialize)]
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


