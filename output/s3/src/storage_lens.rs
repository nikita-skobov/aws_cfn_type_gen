

/// The AWS::S3::StorageLens resource creates an Amazon S3 Storage Lens    configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnStorageLens {
    fn type_string() -> &'static str {
        "AWS::S3::StorageLens"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// This resource contains the details of the account-level metrics for Amazon S3 Storage    Lens.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub detailed_status_codes_metrics: Option<DetailedStatusCodesMetrics>,

}




/// This resource enables Amazon S3 Storage Lens activity metrics. Activity metrics show details about    how your storage is requested, such as requests (for example, All requests, Get requests,    Put requests), bytes uploaded or downloaded, and errors.
///
/// For more information, see    Assessing your storage activity and usage with S3 Storage Lens in the Amazon S3 User Guide.    For a complete list of metrics, see     S3 Storage Lens metrics glossary in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub is_enabled: Option<bool>,

}




/// This resource enables Amazon S3 Storage Lens advanced cost optimization metrics. Advanced    cost optimization metrics provide insights that you can use to manage and optimize your    storage costs, for example, lifecycle rule counts for transitions, expirations, and    incomplete multipart uploads.
///
/// For more information, see    Assessing your storage activity and usage with S3 Storage Lens in the Amazon S3 User Guide.    For a complete list of metrics, see     S3 Storage Lens metrics glossary in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub is_enabled: Option<bool>,

}




/// This resource enables Amazon S3 Storage Lens advanced data protection metrics. Advanced data    protection metrics provide insights that you can use to perform audits and protect your data,    for example replication rule counts within and across Regions.
///
/// For more information, see    Assessing your storage activity and usage with S3 Storage Lens in the Amazon S3 User Guide.    For a complete list of metrics, see     S3 Storage Lens metrics glossary in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub is_enabled: Option<bool>,

}




/// This resource contains the details of the AWS Organization for Amazon S3    Storage Lens.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub arn: String,

}




/// A property for the bucket-level storage metrics for Amazon S3 Storage Lens.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub prefix_level: Option<PrefixLevel>,

}




/// This resource contains the details of the buckets and Regions for the Amazon S3 Storage    Lens configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub regions: Option<Vec<String>>,

}




/// This resource enables the Amazon CloudWatch publishing option for Amazon S3 Storage Lens    metrics.
///
/// For more information, see Monitor S3     Storage Lens metrics in CloudWatch in the Amazon S3 User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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




/// This resource contains the details of the Amazon S3 Storage Lens metrics export.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub s3_bucket_destination: Option<S3BucketDestination>,

}




/// This resource enables Amazon S3 Storage Lens detailed status code metrics. Detailed status    code metrics generate metrics for HTTP status codes, such as 200 OK, 403     Forbidden, 503 Service Unavailable and others.
///
/// For more information, see    Assessing your storage activity and usage with S3 Storage Lens in the Amazon S3 User Guide.    For a complete list of metrics, see     S3 Storage Lens metrics glossary in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub is_enabled: Option<bool>,

}




/// This resource contains the type of server-side encryption used to encrypt an Amazon S3    Storage Lens metrics export. For valid values, see the     StorageLensDataExportEncryption in the Amazon S3 API    Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub sses3: Option<serde_json::Value>,

}




/// This resource contains the details of the prefix-level of the Amazon S3 Storage    Lens.
#[derive(Clone, Debug, Default, serde::Serialize)]
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




/// This resource contains the details of the prefix-level storage metrics for Amazon S3    Storage Lens.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub selection_criteria: Option<SelectionCriteria>,

}




/// This resource contains the details of the bucket where the Amazon S3 Storage Lens metrics    export will be placed.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub account_id: String,


    /// 
    /// This property contains the details of the ARN of the bucket destination of the S3 Storage    Lens export.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: String,


    /// 
    /// This property contains the details of the encryption of the bucket destination of the    Amazon S3 Storage Lens metrics export.
    /// 
    /// Required: No
    ///
    /// Type: Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
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
    pub format: String,


    /// 
    /// This property contains the details of the output schema version of the S3 Storage Lens    export bucket destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputSchemaVersion")]
    pub output_schema_version: String,


    /// 
    /// This property contains the details of the prefix of the bucket destination of the S3    Storage Lens export .
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}




/// Specifies the use of server-side encryption using an AWS Key Management Service key (SSE-KMS) to    encrypt the delivered S3 Storage Lens metrics export file.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub key_id: String,

}




/// This resource contains the details of the Amazon S3 Storage Lens selection    criteria.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub delimiter: Option<String>,


    /// 
    /// This property contains the details of the max depth that S3 Storage Lens will collect    metrics up to.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxDepth")]
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
    pub min_storage_bytes_percentage: Option<f64>,

}




/// This is the property of the Amazon S3 Storage Lens configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub id: String,


    /// 
    /// This property contains the details of the bucket and or Regions included for Amazon S3    Storage Lens configuration.
    /// 
    /// Required: No
    ///
    /// Type: BucketsAndRegions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Include")]
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
    pub storage_lens_arn: Option<String>,

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


