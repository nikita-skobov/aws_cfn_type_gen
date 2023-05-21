

/// Create a new FeatureGroup. A FeatureGroup is a group of         Features defined in the FeatureStore to describe a         Record.
///
/// The FeatureGroup defines the schema and features contained in the       FeatureGroup. A FeatureGroup definition is composed of a list of         Features, a RecordIdentifierFeatureName, an         EventTimeFeatureName and configurations for its         OnlineStore and OfflineStore. Check AWS service quotas to see the FeatureGroups quota       for your AWS account.
#[derive(Default, serde::Serialize)]
pub struct CfnFeatureGroup {


    /// 
    /// A list of Features. Each Feature must include a       FeatureName and a FeatureType.
    /// 
    /// Valid FeatureTypes are Integral, Fractional and       String.
    /// 
    /// FeatureNames cannot be any of the following: is_deleted,       write_time, api_invocation_time.
    /// 
    /// You can create up to 2,500 FeatureDefinitions per     FeatureGroup.
    /// 
    /// Required: Yes
    ///
    /// Type: List of FeatureDefinition
    ///
    /// Maximum: 2500
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeatureDefinitions")]
    pub feature_definitions: Vec<FeatureDefinition>,


    /// 
    /// Tags used to define a FeatureGroup.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The name of the FeatureGroup.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9]([_-]*[a-zA-Z0-9]){0,63}
    ///
    /// Update requires: Replacement
    #[serde(rename = "FeatureGroupName")]
    pub feature_group_name: String,


    /// 
    /// A free form description of a FeatureGroup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The configuration of an OnlineStore.
    /// 
    /// Required: No
    ///
    /// Type: OnlineStoreConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "OnlineStoreConfig")]
    pub online_store_config: Option<OnlineStoreConfig>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM execution role used to create the feature     group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:aws[a-z\-]*:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The configuration of an OfflineStore.
    /// 
    /// Required: No
    ///
    /// Type: OfflineStoreConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "OfflineStoreConfig")]
    pub offline_store_config: Option<OfflineStoreConfig>,


    /// 
    /// The name of the Feature whose value uniquely identifies a    Record defined in the FeatureGroup       FeatureDefinitions.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9]([-_]*[a-zA-Z0-9]){0,63}
    ///
    /// Update requires: Replacement
    #[serde(rename = "RecordIdentifierFeatureName")]
    pub record_identifier_feature_name: String,


    /// 
    /// The name of the feature that stores the EventTime of a Record in a     FeatureGroup.
    /// 
    /// A EventTime is point in time when a new event     occurs that corresponds to the creation or update of a Record in     FeatureGroup. All Records in the FeatureGroup     must have a corresponding EventTime.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9]([-_]*[a-zA-Z0-9]){0,63}
    ///
    /// Update requires: Replacement
    #[serde(rename = "EventTimeFeatureName")]
    pub event_time_feature_name: String,

}


/// Use this to specify the AWS Key Management Service (KMS) Key ID, or       KMSKeyId, for at rest data encryption. You can turn       OnlineStore on or off by specifying the EnableOnlineStore flag     at General Assembly.
///
/// The default value is False.
#[derive(Default, serde::Serialize)]
pub struct OnlineStoreConfig {


    /// 
    /// Use to specify KMS Key ID (KMSKeyId) for at-rest encryption of your       OnlineStore.
    /// 
    /// Required: No
    ///
    /// Type: OnlineStoreSecurityConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityConfig")]
    pub security_config: Option<OnlineStoreSecurityConfig>,


    /// 
    /// Turn OnlineStore off by specifying False    for the EnableOnlineStore flag. Turn OnlineStore    on by specifying True    for the EnableOnlineStore flag.
    /// 
    /// The default value is False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableOnlineStore")]
    pub enable_online_store: Option<bool>,

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


/// A list of features. You must include FeatureName and       FeatureType. Valid feature FeatureTypes are       Integral, Fractional and String.
#[derive(Default, serde::Serialize)]
pub struct FeatureDefinition {


    /// 
    /// The value type of a feature. Valid values are Integral, Fractional, or String.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Fractional | Integral | String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeatureType")]
    pub feature_type: String,


    /// 
    /// The name of a feature. The type must be a string. FeatureName cannot be any     of the following: is_deleted, write_time,       api_invocation_time.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9]([-_]*[a-zA-Z0-9]){0,63}
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeatureName")]
    pub feature_name: String,

}


/// The security configuration for OnlineStore.
#[derive(Default, serde::Serialize)]
pub struct OnlineStoreSecurityConfig {


    /// 
    /// The AWS Key Management Service (KMS) key ARN that SageMaker Feature Store uses     to encrypt the Amazon S3 objects at rest using Amazon S3 server-side encryption.
    /// 
    /// The caller (either user or IAM role) of CreateFeatureGroup must have     below permissions to the OnlineStore       KmsKeyId:
    /// 
    /// "kms:Encrypt"                                "kms:Decrypt"                                "kms:DescribeKey"                                "kms:CreateGrant"                                "kms:RetireGrant"                                "kms:ReEncryptFrom"                                "kms:ReEncryptTo"                                "kms:GenerateDataKey"                                "kms:ListAliases"                                "kms:ListGrants"                                "kms:RevokeGrant"
    /// 
    /// The caller (either user or IAM role) to all DataPlane operations       (PutRecord, GetRecord, DeleteRecord) must have     the following permissions to the KmsKeyId:
    /// 
    /// "kms:Decrypt"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


/// The meta data of the Glue table which serves as data catalog for the       OfflineStore.
#[derive(Default, serde::Serialize)]
pub struct DataCatalogConfig {


    /// 
    /// The name of the Glue table catalog.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Catalog")]
    pub catalog: String,


    /// 
    /// The name of the Glue table database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// The name of the Glue table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableName")]
    pub table_name: String,

}


/// The configuration of an OfflineStore.
///
/// Provide an OfflineStoreConfig in a request to       CreateFeatureGroup to create an OfflineStore.
///
/// To encrypt an OfflineStore using at rest data encryption, specify AWS Key     Management Service (KMS) key ID, or KMSKeyId, in     S3StorageConfig.
#[derive(Default, serde::Serialize)]
pub struct OfflineStoreConfig {


    /// 
    /// Set to True to disable the automatic creation of an AWS Glue table when    configuring an OfflineStore. If set to False, Feature Store will name the      OfflineStore Glue table following      Athena's naming recommendations.
    /// 
    /// The default value is False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DisableGlueTableCreation")]
    pub disable_glue_table_creation: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableFormat")]
    pub table_format: Option<String>,


    /// 
    /// The Amazon Simple Storage (Amazon S3) location of OfflineStore.
    /// 
    /// Required: Yes
    ///
    /// Type: S3StorageConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3StorageConfig")]
    pub s3_storage_config: S3StorageConfig,


    /// 
    /// The meta data of the Glue table that is autogenerated when an OfflineStore     is created.
    /// 
    /// Required: No
    ///
    /// Type: DataCatalogConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataCatalogConfig")]
    pub data_catalog_config: Option<DataCatalogConfig>,

}


/// The Amazon Simple Storage (Amazon S3) location and and security configuration for OfflineStore.
#[derive(Default, serde::Serialize)]
pub struct S3StorageConfig {


    /// 
    /// The S3 URI, or location in Amazon S3, of OfflineStore.
    /// 
    /// S3 URIs have a format similar to the following: s3://example-bucket/prefix/.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,


    /// 
    /// The AWS Key Management Service (KMS) key ARN of the key used to encrypt any objects     written into the OfflineStore S3 location.
    /// 
    /// The IAM roleARN that is passed as a parameter to       CreateFeatureGroup must have below permissions to the     KmsKeyId:
    /// 
    /// "kms:GenerateDataKey"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}