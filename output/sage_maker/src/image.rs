

/// Creates a custom SageMaker image. A SageMaker image is a set of image versions. Each image     version represents a container image stored in Amazon Elastic Container Registry (ECR). For more information, see     Bring your own SageMaker image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnImage {


    /// 
    /// The display name of the image.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of       128.
    /// 
    /// Pattern: ^\S(.*\S)?$
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageDisplayName")]
    pub image_display_name: Option<String>,


    /// 
    /// A list of key-value pairs to apply to this resource.
    /// 
    /// Array Members: Minimum number of 0 items. Maximum number of 50       items.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform       tasks on your behalf.
    /// 
    /// Length Constraints: Minimum length of 20. Maximum length of       2048.
    /// 
    /// Pattern:         ^arn:aws[a-z\-]*:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageRoleArn")]
    pub image_role_arn: String,


    /// 
    /// The name of the Image. Must be unique by region in your account.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of       63.
    /// 
    /// Pattern:       ^[a-zA-Z0-9]([-.]?[a-zA-Z0-9]){0,62}$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageName")]
    pub image_name: String,


    /// 
    /// The description of the image.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of       512.
    /// 
    /// Pattern: .*
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageDescription")]
    pub image_description: Option<String>,

}

impl cfn_resources::CfnResource for CfnImage {
    fn type_string() -> &'static str {
        "AWS::SageMaker::Image"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
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
