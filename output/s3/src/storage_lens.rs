/// The AWS::S3::StorageLens resource creates an Amazon S3 Storage Lens    configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageLens {
    ///
    /// This resource contains the details Amazon S3 Storage Lens configuration.
    ///
    /// Required: Yes
    ///
    /// Type: StorageLensConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageLensConfiguration")]
    pub storage_lens_configuration: StorageLensConfiguration,

    ///
    /// A set of tags (key–value pairs) to associate with the Storage Lens configuration.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_storage_lens_configuration_storage_lens_arn:
        CfnStorageLensstoragelensconfigurationstoragelensarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageLensstoragelensconfigurationstoragelensarn;
impl CfnStorageLensstoragelensconfigurationstoragelensarn {
    pub fn att_name(&self) -> &'static str {
        r#"StorageLensConfiguration.StorageLensArn"#
    }
}

impl cfn_resources::CfnResource for CfnStorageLens {
    fn type_string(&self) -> &'static str {
        "AWS::S3::StorageLens"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.storage_lens_configuration.validate()?;

        Ok(())
    }
}

/// This resource contains the details of the account-level metrics for Amazon S3 Storage    Lens.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountLevel {
    ///
    /// This property contains the details of account-level activity metrics for S3    Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: ActivityMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActivityMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_metrics: Option<ActivityMetrics>,

    ///
    /// This property contains the details of account-level advanced cost optimization metrics for S3    Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: AdvancedCostOptimizationMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedCostOptimizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_cost_optimization_metrics: Option<AdvancedCostOptimizationMetrics>,

    ///
    /// This property contains the details of account-level advanced data protection metrics for S3    Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: AdvancedDataProtectionMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedDataProtectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_data_protection_metrics: Option<AdvancedDataProtectionMetrics>,

    ///
    /// This property contains the details of the account-level bucket-level configurations for    Amazon S3 Storage Lens.
    ///
    /// Required: Yes
    ///
    /// Type: BucketLevel
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketLevel")]
    pub bucket_level: BucketLevel,

    ///
    /// This property contains the details of account-level detailed status code metrics for S3    Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: DetailedStatusCodesMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetailedStatusCodesMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status_codes_metrics: Option<DetailedStatusCodesMetrics>,
}

impl cfn_resources::CfnResource for AccountLevel {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.activity_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.advanced_cost_optimization_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.advanced_data_protection_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.bucket_level.validate()?;

        self.detailed_status_codes_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This resource enables Amazon S3 Storage Lens activity metrics. Activity metrics show details about    how your storage is requested, such as requests (for example, All requests, Get requests,    Put requests), bytes uploaded or downloaded, and errors.
///
/// For more information, see    Assessing your storage activity and usage with S3 Storage Lens in the Amazon S3 User Guide.    For a complete list of metrics, see     S3 Storage Lens metrics glossary in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ActivityMetrics {
    ///
    /// A property that indicates whether the activity metrics is enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for ActivityMetrics {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This resource enables Amazon S3 Storage Lens advanced cost optimization metrics. Advanced    cost optimization metrics provide insights that you can use to manage and optimize your    storage costs, for example, lifecycle rule counts for transitions, expirations, and    incomplete multipart uploads.
///
/// For more information, see    Assessing your storage activity and usage with S3 Storage Lens in the Amazon S3 User Guide.    For a complete list of metrics, see     S3 Storage Lens metrics glossary in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AdvancedCostOptimizationMetrics {
    ///
    /// Indicates whether advanced cost optimization metrics are    enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for AdvancedCostOptimizationMetrics {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This resource enables Amazon S3 Storage Lens advanced data protection metrics. Advanced data    protection metrics provide insights that you can use to perform audits and protect your data,    for example replication rule counts within and across Regions.
///
/// For more information, see    Assessing your storage activity and usage with S3 Storage Lens in the Amazon S3 User Guide.    For a complete list of metrics, see     S3 Storage Lens metrics glossary in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AdvancedDataProtectionMetrics {
    ///
    /// Indicates whether advanced data protection metrics are enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for AdvancedDataProtectionMetrics {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This resource contains the details of the AWS Organization for Amazon S3    Storage Lens.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AwsOrg {
    ///
    /// This resource contains the ARN of the AWS Organization.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AwsOrg {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A property for the bucket-level storage metrics for Amazon S3 Storage Lens.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BucketLevel {
    ///
    /// A property for bucket-level activity metrics for S3 Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: ActivityMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActivityMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_metrics: Option<ActivityMetrics>,

    ///
    /// A property for bucket-level advanced cost optimization metrics for S3 Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: AdvancedCostOptimizationMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedCostOptimizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_cost_optimization_metrics: Option<AdvancedCostOptimizationMetrics>,

    ///
    /// A property for bucket-level advanced data protection metrics for S3 Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: AdvancedDataProtectionMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedDataProtectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_data_protection_metrics: Option<AdvancedDataProtectionMetrics>,

    ///
    /// A property for bucket-level detailed status code metrics for S3 Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: DetailedStatusCodesMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetailedStatusCodesMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status_codes_metrics: Option<DetailedStatusCodesMetrics>,

    ///
    /// A property for bucket-level prefix-level storage metrics for S3 Storage Lens.
    ///
    /// Required: No
    ///
    /// Type: PrefixLevel
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_level: Option<PrefixLevel>,
}

impl cfn_resources::CfnResource for BucketLevel {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.activity_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.advanced_cost_optimization_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.advanced_data_protection_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.detailed_status_codes_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.prefix_level
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This resource contains the details of the buckets and Regions for the Amazon S3 Storage    Lens configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BucketsAndRegions {
    ///
    /// This property contains the details of the buckets for the Amazon S3 Storage Lens    configuration. This should be the bucket Amazon Resource Name(ARN). For valid values, see     Buckets ARN     format here in the Amazon S3 API Reference.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Buckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<String>>,

    ///
    /// This property contains the details of the Regions for the S3 Storage Lens    configuration.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for BucketsAndRegions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This resource enables the Amazon CloudWatch publishing option for Amazon S3 Storage Lens    metrics.
///
/// For more information, see Monitor S3     Storage Lens metrics in CloudWatch in the Amazon S3 User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CloudWatchMetrics {
    ///
    /// This property identifies whether the CloudWatch publishing option for S3 Storage    Lens is enabled.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
}

impl cfn_resources::CfnResource for CloudWatchMetrics {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This resource contains the details of the Amazon S3 Storage Lens metrics export.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DataExport {
    ///
    /// This property enables the Amazon CloudWatch publishing option for S3 Storage Lens    metrics.
    ///
    /// Required: No
    ///
    /// Type: CloudWatchMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics: Option<CloudWatchMetrics>,

    ///
    /// This property contains the details of the bucket where the S3 Storage Lens metrics export    will be placed.
    ///
    /// Required: No
    ///
    /// Type: S3BucketDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_destination: Option<S3BucketDestination>,
}

impl cfn_resources::CfnResource for DataExport {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_watch_metrics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_bucket_destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This resource enables Amazon S3 Storage Lens detailed status code metrics. Detailed status    code metrics generate metrics for HTTP status codes, such as 200 OK, 403     Forbidden, 503 Service Unavailable and others.
///
/// For more information, see    Assessing your storage activity and usage with S3 Storage Lens in the Amazon S3 User Guide.    For a complete list of metrics, see     S3 Storage Lens metrics glossary in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DetailedStatusCodesMetrics {
    ///
    /// Indicates whether detailed status code metrics are enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for DetailedStatusCodesMetrics {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This resource contains the type of server-side encryption used to encrypt an Amazon S3    Storage Lens metrics export. For valid values, see the     StorageLensDataExportEncryption in the Amazon S3 API    Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Encryption {
    ///
    /// Specifies the use of AWS Key Management Service keys (SSE-KMS) to encrypt the S3 Storage Lens metrics export file.
    ///
    /// Required: No
    ///
    /// Type: SSEKMS
    ///
    /// Update requires: No interruption
    #[serde(rename = "SSEKMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssekms: Option<SSEKMS>,

    ///
    /// Specifies the use of an Amazon S3-managed key (SSE-S3) to encrypt the S3 Storage Lens metrics export file.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "SSES3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sses3: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for Encryption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ssekms.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This resource contains the details of the prefix-level of the Amazon S3 Storage    Lens.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PrefixLevel {
    ///
    /// A property for the prefix-level storage metrics for Amazon S3 Storage Lens.
    ///
    /// Required: Yes
    ///
    /// Type: PrefixLevelStorageMetrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageMetrics")]
    pub storage_metrics: PrefixLevelStorageMetrics,
}

impl cfn_resources::CfnResource for PrefixLevel {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.storage_metrics.validate()?;

        Ok(())
    }
}

/// This resource contains the details of the prefix-level storage metrics for Amazon S3    Storage Lens.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PrefixLevelStorageMetrics {
    ///
    /// This property identifies whether the details of the prefix-level storage metrics for S3    Storage Lens are enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,

    ///
    /// This property identifies whether the details of the prefix-level storage metrics for S3    Storage Lens are enabled.
    ///
    /// Required: No
    ///
    /// Type: SelectionCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectionCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_criteria: Option<SelectionCriteria>,
}

impl cfn_resources::CfnResource for PrefixLevelStorageMetrics {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.selection_criteria
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This resource contains the details of the bucket where the Amazon S3 Storage Lens metrics    export will be placed.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct S3BucketDestination {
    ///
    /// This property contains the details of the AWS account ID of the S3    Storage Lens export bucket destination.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountId")]
    pub account_id: cfn_resources::StrVal,

    ///
    /// This property contains the details of the ARN of the bucket destination of the S3 Storage    Lens export.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: cfn_resources::StrVal,

    ///
    /// This property contains the details of the encryption of the bucket destination of the    Amazon S3 Storage Lens metrics export.
    ///
    /// Required: No
    ///
    /// Type: Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,

    ///
    /// This property contains the details of the format of the S3 Storage Lens export bucket    destination.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: cfn_resources::StrVal,

    ///
    /// This property contains the details of the output schema version of the S3 Storage Lens    export bucket destination.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputSchemaVersion")]
    pub output_schema_version: cfn_resources::StrVal,

    ///
    /// This property contains the details of the prefix of the bucket destination of the S3    Storage Lens export .
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3BucketDestination {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the use of server-side encryption using an AWS Key Management Service key (SSE-KMS) to    encrypt the delivered S3 Storage Lens metrics export file.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SSEKMS {
    ///
    /// Specifies the Amazon Resource Name (ARN) of the customer managed AWS KMS key to    use for encrypting the S3 Storage Lens metrics export file. Amazon S3 only supports symmetric    encryption keys. For more information, see Special-purpose keys in the      AWS Key Management Service Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyId")]
    pub key_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SSEKMS {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This resource contains the details of the Amazon S3 Storage Lens selection    criteria.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SelectionCriteria {
    ///
    /// This property contains the details of the S3 Storage Lens delimiter being used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<cfn_resources::StrVal>,

    ///
    /// This property contains the details of the max depth that S3 Storage Lens will collect    metrics up to.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i64>,

    ///
    /// This property contains the details of the minimum storage bytes percentage threshold that    S3 Storage Lens will collect metrics up to.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinStorageBytesPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_bytes_percentage: Option<f64>,
}

impl cfn_resources::CfnResource for SelectionCriteria {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This is the property of the Amazon S3 Storage Lens configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StorageLensConfiguration {
    ///
    /// This property contains the details of the account-level metrics for Amazon S3 Storage Lens    configuration.
    ///
    /// Required: Yes
    ///
    /// Type: AccountLevel
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountLevel")]
    pub account_level: AccountLevel,

    ///
    /// This property contains the details of the AWS Organization for the S3    Storage Lens configuration.
    ///
    /// Required: No
    ///
    /// Type: AwsOrg
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_org: Option<AwsOrg>,

    ///
    /// This property contains the details of this S3 Storage Lens configuration's metrics    export.
    ///
    /// Required: No
    ///
    /// Type: DataExport
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataExport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_export: Option<DataExport>,

    ///
    /// This property contains the details of the bucket and or Regions excluded for Amazon S3    Storage Lens configuration.
    ///
    /// Required: No
    ///
    /// Type: BucketsAndRegions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<BucketsAndRegions>,

    ///
    /// This property contains the details of the ID of the S3 Storage Lens configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// This property contains the details of the bucket and or Regions included for Amazon S3    Storage Lens configuration.
    ///
    /// Required: No
    ///
    /// Type: BucketsAndRegions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<BucketsAndRegions>,

    ///
    /// This property contains the details of whether the Amazon S3 Storage Lens configuration is    enabled.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,

    ///
    /// This property contains the details of the ARN of the S3 Storage Lens configuration. This    property is read-only.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageLensArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for StorageLensConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.account_level.validate()?;

        self.aws_org.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.data_export
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.exclude.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.include.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
