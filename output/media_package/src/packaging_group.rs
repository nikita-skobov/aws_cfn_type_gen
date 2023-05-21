

/// Creates a packaging group.
///
/// The packaging group holds one or more packaging configurations. When you create an asset, you specify the packaging group associated with the asset. The asset has playback endpoints for each packaging configuration within the group.
#[derive(Default, serde::Serialize)]
pub struct CfnPackagingGroup {


    /// 
    /// Parameters for CDN authorization.
    /// 
    /// Required: No
    ///
    /// Type: Authorization
    ///
    /// Update requires: No interruption
    #[serde(rename = "Authorization")]
    pub authorization: Option<Authorization>,


    /// 
    /// The tags to assign to the packaging group.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Unique identifier that you assign to the packaging group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// The configuration parameters for egress access logging.
    /// 
    /// Required: No
    ///
    /// Type: LogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressAccessLogs")]
    pub egress_access_logs: Option<LogConfiguration>,

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


/// Parameters for enabling CDN authorization.
#[derive(Default, serde::Serialize)]
pub struct Authorization {


    /// 
    /// The Amazon Resource Name (ARN) for the secret in AWS Secrets Manager that is used for CDN authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdnIdentifierSecret")]
    pub cdn_identifier_secret: String,


    /// 
    /// The Amazon Resource Name (ARN) for the IAM role that allows AWS Elemental MediaPackage to communicate with AWS Secrets Manager.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsRoleArn")]
    pub secrets_role_arn: String,

}


/// Sets a custom Amazon CloudWatch log group name for egress logs. If a log group name isn't specified, the default name is used: /aws/MediaPackage/EgressAccessLogs.
#[derive(Default, serde::Serialize)]
pub struct LogConfiguration {


    /// 
    /// Sets a custom Amazon CloudWatch log group name for egress logs. If a log group name isn't specified, the default name is used: /aws/MediaPackage/EgressAccessLogs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,

}
