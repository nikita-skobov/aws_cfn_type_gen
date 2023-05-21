

/// The AWS::Logs::LogGroup resource specifies a log group. A log group defines common properties for log streams,      such as their retention and access control rules. Each log stream must belong to one log group.
///
/// You can create up to 1,000,000 log groups per Region per account. You must use the following guidelines when naming a log group:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLogGroup {


    /// 
    /// The Amazon Resource Name (ARN) of the AWS KMS key to use when encrypting log data.
    /// 
    /// To associate an AWS KMS key with the log group, specify the ARN of that KMS key here. If you do so,    ingested data is encrypted using this key.     This association is stored as long as the data encrypted with the KMS key is still within CloudWatch Logs.    This enables CloudWatch Logs to decrypt this data whenever it is requested.
    /// 
    /// If you attempt to associate a KMS key with the log group but the KMS key doesn't exist or is deactivated, you will    receive an InvalidParameterException error.
    /// 
    /// Log group data is always encrypted in CloudWatch Logs. If you omit this key, the encryption does not use    AWS KMS. For more information, see      Encrypt log data in CloudWatch Logs using AWS Key Management Service
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// An array of key-value pairs to apply to the log group.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Creates a data protection policy and assigns it to the log group. A data protection policy can help safeguard sensitive      data that's ingested by the log group by auditing and masking the sensitive log data. When a user who does not have      permission to view masked data     views a log event that includes masked data, the sensitive data is replaced by asterisks.
    /// 
    /// For more information, including a list of types of data that can be audited and masked, see     Protect sensitive log data with masking.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataProtectionPolicy")]
    pub data_protection_policy: Option<serde_json::Value>,


    /// 
    /// The name of the log group. If you don't specify a name, AWS CloudFormation generates a unique ID for the log group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\.\-_/#A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,


    /// 
    /// The number of days to retain the log events in the specified log group.    Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, and 3653.
    /// 
    /// To set a log group so that its log events do not expire, use DeleteRetentionPolicy.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionInDays")]
    pub retention_in_days: Option<i64>,

}

impl cfn_resources::CfnResource for CfnLogGroup {
    fn type_string() -> &'static str {
        "AWS::Logs::LogGroup"
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
