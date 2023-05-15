
pub mod cfn_resource_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResourcePolicy {
    /// No documentation provided by AWS
    #[serde(rename = "ResourcePolicy")]
    pub resource_policy: (),
    /// No documentation provided by AWS
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<bool>,

}



}

pub mod cfn_rotation_schedule {

#[derive(serde::Serialize, Default)]
pub struct CfnRotationSchedule {
    /// No documentation provided by AWS
    #[serde(rename = "RotationRules")]
    pub rotation_rules: Option<RotationRules>,
    /// No documentation provided by AWS
    #[serde(rename = "RotateImmediatelyOnUpdate")]
    pub rotate_immediately_on_update: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "RotationLambdaARN")]
    pub rotation_lambda_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "HostedRotationLambda")]
    pub hosted_rotation_lambda: Option<HostedRotationLambda>,

}


#[derive(serde::Serialize, Default)]
pub struct HostedRotationLambda {
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<String>,
    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "MasterSecretKmsKeyArn")]
    pub master_secret_kms_key_arn: Option<String>,
    #[serde(rename = "ExcludeCharacters")]
    pub exclude_characters: Option<String>,
    #[serde(rename = "RotationLambdaName")]
    pub rotation_lambda_name: Option<String>,
    #[serde(rename = "SuperuserSecretArn")]
    pub superuser_secret_arn: Option<String>,
    #[serde(rename = "RotationType")]
    pub rotation_type: String,
    #[serde(rename = "VpcSubnetIds")]
    pub vpc_subnet_ids: Option<String>,
    #[serde(rename = "MasterSecretArn")]
    pub master_secret_arn: Option<String>,
    #[serde(rename = "SuperuserSecretKmsKeyArn")]
    pub superuser_secret_kms_key_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RotationRules {
    #[serde(rename = "AutomaticallyAfterDays")]
    pub automatically_after_days: Option<usize>,
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "Duration")]
    pub duration: Option<String>,

}


}

pub mod cfn_secret {

#[derive(serde::Serialize, Default)]
pub struct CfnSecret {
    /// (Optional) Specifies a user-provided description of the secret.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// (Optional) A list of ReplicaRegion objects. The ReplicaRegion type consists of a Region (required) and the KmsKeyId which can be an ARN, Key ID, or Alias.
    #[serde(rename = "ReplicaRegions")]
    pub replica_regions: Option<Vec<ReplicaRegion>>,
    /// (Optional) Specifies text data that you want to encrypt and store in this new version of the secret.
    #[serde(rename = "GenerateSecretString")]
    pub generate_secret_string: Option<GenerateSecretString>,
    /// (Optional) Specifies the ARN, Key ID, or alias of the AWS KMS customer master key (CMK) used to encrypt the SecretString.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// (Optional) Specifies text data that you want to encrypt and store in this new version of the secret.
    #[serde(rename = "SecretString")]
    pub secret_string: Option<String>,
    /// The list of user-defined tags associated with the secret. Use tags to manage your AWS resources. For additional information about tags, see TagResource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The friendly name of the secret. You can use forward slashes in the name to represent a path hierarchy.
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicaRegion {
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GenerateSecretString {
    #[serde(rename = "SecretStringTemplate")]
    pub secret_string_template: Option<String>,
    #[serde(rename = "ExcludeNumbers")]
    pub exclude_numbers: Option<bool>,
    #[serde(rename = "ExcludeCharacters")]
    pub exclude_characters: Option<String>,
    #[serde(rename = "IncludeSpace")]
    pub include_space: Option<bool>,
    #[serde(rename = "ExcludeUppercase")]
    pub exclude_uppercase: Option<bool>,
    #[serde(rename = "RequireEachIncludedType")]
    pub require_each_included_type: Option<bool>,
    #[serde(rename = "GenerateStringKey")]
    pub generate_string_key: Option<String>,
    #[serde(rename = "PasswordLength")]
    pub password_length: Option<usize>,
    #[serde(rename = "ExcludePunctuation")]
    pub exclude_punctuation: Option<bool>,
    #[serde(rename = "ExcludeLowercase")]
    pub exclude_lowercase: Option<bool>,

}


}

pub mod cfn_secret_target_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnSecretTargetAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "TargetType")]
    pub target_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "TargetId")]
    pub target_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "SecretId")]
    pub secret_id: String,

}



}
