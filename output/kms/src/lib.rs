
pub mod cfn_alias {

#[derive(serde::Serialize, Default)]
pub struct CfnAlias {
    /// Identifies the AWS KMS key to which the alias refers. Specify the key ID or the Amazon Resource Name (ARN) of the AWS KMS key. You cannot specify another alias. For help finding the key ID and ARN, see Finding the Key ID and ARN in the AWS Key Management Service Developer Guide.
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
    /// Specifies the alias name. This value must begin with alias/ followed by a name, such as alias/ExampleAlias. The alias name cannot begin with alias/aws/. The alias/aws/ prefix is reserved for AWS managed keys.
    #[serde(rename = "AliasName")]
    pub alias_name: String,

}



}

pub mod cfn_key {

#[derive(serde::Serialize, Default)]
pub struct CfnKey {
    /// Specifies whether the AWS KMS key is enabled. Disabled AWS KMS keys cannot be used in cryptographic operations.
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// Determines the cryptographic operations for which you can use the AWS KMS key. The default value is ENCRYPT_DECRYPT. This property is required only for asymmetric AWS KMS keys. You can't change the KeyUsage value after the AWS KMS key is created.
    #[serde(rename = "KeyUsage")]
    pub key_usage: Option<String>,
    /// Specifies whether the AWS KMS key should be Multi-Region. You can't change the MultiRegion value after the AWS KMS key is created.
    #[serde(rename = "MultiRegion")]
    pub multi_region: Option<bool>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The key policy that authorizes use of the AWS KMS key. The key policy must observe the following rules.
    #[serde(rename = "KeyPolicy")]
    pub key_policy: (),
    /// A description of the AWS KMS key. Use a description that helps you to distinguish this AWS KMS key from others in the account, such as its intended use.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Specifies the type of AWS KMS key to create. The default value is SYMMETRIC_DEFAULT. This property is required only for asymmetric AWS KMS keys. You can't change the KeySpec value after the AWS KMS key is created.
    #[serde(rename = "KeySpec")]
    pub key_spec: Option<String>,
    /// Specifies the number of days in the waiting period before AWS KMS deletes an AWS KMS key that has been removed from a CloudFormation stack. Enter a value between 7 and 30 days. The default value is 30 days.
    #[serde(rename = "PendingWindowInDays")]
    pub pending_window_in_days: Option<usize>,
    /// Enables automatic rotation of the key material for the specified AWS KMS key. By default, automation key rotation is not enabled.
    #[serde(rename = "EnableKeyRotation")]
    pub enable_key_rotation: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_replica_key {

#[derive(serde::Serialize, Default)]
pub struct CfnReplicaKey {
    /// Specifies whether the AWS KMS key is enabled. Disabled AWS KMS keys cannot be used in cryptographic operations.
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// Identifies the primary AWS KMS key to create a replica of. Specify the Amazon Resource Name (ARN) of the AWS KMS key. You cannot specify an alias or key ID. For help finding the ARN, see Finding the Key ID and ARN in the AWS Key Management Service Developer Guide.
    #[serde(rename = "PrimaryKeyArn")]
    pub primary_key_arn: String,
    /// A description of the AWS KMS key. Use a description that helps you to distinguish this AWS KMS key from others in the account, such as its intended use.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The key policy that authorizes use of the AWS KMS key. The key policy must observe the following rules.
    #[serde(rename = "KeyPolicy")]
    pub key_policy: (),
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Specifies the number of days in the waiting period before AWS KMS deletes an AWS KMS key that has been removed from a CloudFormation stack. Enter a value between 7 and 30 days. The default value is 30 days.
    #[serde(rename = "PendingWindowInDays")]
    pub pending_window_in_days: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
