/// The AWS::S3::Bucket resource creates an Amazon S3 bucket in the same AWS Region where you create the AWS CloudFormation stack.
///
/// To control how AWS CloudFormation handles the bucket when the stack is    deleted, you can set a deletion policy for your bucket. You can choose to     retain the bucket or to delete the bucket. For    more information, see DeletionPolicy     Attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucket {
    ///
    /// Configures the transfer acceleration state for an Amazon S3 bucket. For more information, see       Amazon S3       Transfer Acceleration in the Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: AccelerateConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccelerateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerate_configuration: Option<AccelerateConfiguration>,

    ///
    /// A canned access control list (ACL) that grants predefined permissions to the bucket. For    more information about canned ACLs, see Canned ACL in the     Amazon S3 User Guide.
    ///
    /// Be aware that the syntax for this property differs from the information provided in the     Amazon S3 User Guide. The AccessControl property is case-sensitive and    must be one of the following values: Private, PublicRead, PublicReadWrite, AuthenticatedRead,    LogDeliveryWrite, BucketOwnerRead, BucketOwnerFullControl, or AwsExecRead.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the configuration and any analyses for the analytics filter of an Amazon S3     bucket.
    ///
    /// Required: No
    ///
    /// Type: List of AnalyticsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnalyticsConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configurations: Option<Vec<AnalyticsConfiguration>>,

    ///
    /// Specifies default encryption for a bucket using server-side encryption with Amazon    S3-managed keys (SSE-S3) or AWS KMS-managed keys (SSE-KMS) bucket. For    information about the Amazon S3 default encryption feature, see Amazon S3 Default Encryption for S3     Buckets in the Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: BucketEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_encryption: Option<BucketEncryption>,

    ///
    /// A name for the bucket. If you don't specify a name, AWS CloudFormation    generates a unique ID and uses that ID for the bucket name. The bucket name must contain only    lowercase letters, numbers, periods (.), and dashes (-) and must follow Amazon S3 bucket     restrictions and limitations. For more information, see Rules for naming Amazon     S3 buckets in the Amazon S3 User Guide.
    ///
    /// ImportantIf you specify a name, you can't perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you need to     replace the resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<cfn_resources::StrVal>,

    ///
    /// Describes the cross-origin access configuration for objects in an Amazon S3 bucket. For more     information, see Enabling       Cross-Origin Resource Sharing in the     Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: CorsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<CorsConfiguration>,

    ///
    /// Defines how Amazon S3 handles Intelligent-Tiering storage.
    ///
    /// Required: No
    ///
    /// Type: List of IntelligentTieringConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntelligentTieringConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligent_tiering_configurations: Option<Vec<IntelligentTieringConfiguration>>,

    ///
    /// Specifies the inventory configuration for an Amazon S3 bucket. For more information, see       GET Bucket inventory in the Amazon S3 API Reference.
    ///
    /// Required: No
    ///
    /// Type: List of InventoryConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InventoryConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_configurations: Option<Vec<InventoryConfiguration>>,

    ///
    /// Specifies the lifecycle configuration for objects in an Amazon S3 bucket. For more     information, see Object Lifecycle Management     in the Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: LifecycleConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecycleConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_configuration: Option<LifecycleConfiguration>,

    ///
    /// Settings that define where logs are stored.
    ///
    /// Required: No
    ///
    /// Type: LoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,

    ///
    /// Specifies a metrics configuration for the CloudWatch request metrics (specified by the     metrics configuration ID) from an Amazon S3 bucket. If you're updating an existing metrics     configuration, note that this is a full replacement of the existing metrics configuration.     If you don't include the elements you want to keep, they are erased. For more information,     see PutBucketMetricsConfiguration.
    ///
    /// Required: No
    ///
    /// Type: List of MetricsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricsConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_configurations: Option<Vec<MetricsConfiguration>>,

    ///
    /// Configuration that defines how Amazon S3 handles bucket notifications.
    ///
    /// Required: No
    ///
    /// Type: NotificationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,

    ///
    /// Places an Object Lock configuration on the specified bucket. The rule specified in the     Object Lock configuration will be applied by default to every new object placed in the     specified bucket. For more information, see Locking Objects.
    ///
    /// Note                                          The DefaultRetention settings require both a mode and a          period.                  The DefaultRetention period can be either Days or           Years but you must select one. You cannot specify           Days and Years at the same time.                  You can only enable Object Lock for new buckets. If you want to turn on Object          Lock for an existing bucket, contact AWS Support.
    ///
    /// Required: No
    ///
    /// Type: ObjectLockConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectLockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_configuration: Option<ObjectLockConfiguration>,

    ///
    /// Indicates whether this bucket has an Object Lock configuration enabled. Enable     ObjectLockEnabled when you apply ObjectLockConfiguration to a    bucket.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObjectLockEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_enabled: Option<bool>,

    ///
    /// Configuration that defines how Amazon S3 handles Object Ownership rules.
    ///
    /// Required: No
    ///
    /// Type: OwnershipControls
    ///
    /// Update requires: No interruption
    #[serde(rename = "OwnershipControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_controls: Option<OwnershipControls>,

    ///
    /// Configuration that defines how Amazon S3 handles public access.
    ///
    /// Required: No
    ///
    /// Type: PublicAccessBlockConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,

    ///
    /// Configuration for replicating objects in an S3 bucket. To enable replication, you must    also enable versioning by using the VersioningConfiguration property.
    ///
    /// Amazon S3 can store replicated objects in a single destination bucket or multiple    destination buckets. The destination bucket or buckets must already exist.
    ///
    /// Required: No
    ///
    /// Type: ReplicationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<ReplicationConfiguration>,

    ///
    /// An arbitrary set of tags (key-value pairs) for this S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Enables multiple versions of all objects in this bucket. You might enable versioning to    prevent objects from being deleted or overwritten by mistake or to archive objects so that you    can retrieve previous versions of them.
    ///
    /// Required: No
    ///
    /// Type: VersioningConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersioningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_configuration: Option<VersioningConfiguration>,

    ///
    /// Information used to configure the bucket as a static website. For more information, see     Hosting Websites     on Amazon S3.
    ///
    /// Required: No
    ///
    /// Type: WebsiteConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebsiteConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_configuration: Option<WebsiteConfiguration>,

    #[serde(skip_serializing)]
    pub att_arn: CfnBucketarn,

    #[serde(skip_serializing)]
    pub att_domain_name: CfnBucketdomainname,

    #[serde(skip_serializing)]
    pub att_dual_stack_domain_name: CfnBucketdualstackdomainname,

    #[serde(skip_serializing)]
    pub att_regional_domain_name: CfnBucketregionaldomainname,

    #[serde(skip_serializing)]
    pub att_website_url: CfnBucketwebsiteurl,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucketarn;
impl CfnBucketarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucketdomainname;
impl CfnBucketdomainname {
    pub fn att_name(&self) -> &'static str {
        r#"DomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucketdualstackdomainname;
impl CfnBucketdualstackdomainname {
    pub fn att_name(&self) -> &'static str {
        r#"DualStackDomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucketregionaldomainname;
impl CfnBucketregionaldomainname {
    pub fn att_name(&self) -> &'static str {
        r#"RegionalDomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucketwebsiteurl;
impl CfnBucketwebsiteurl {
    pub fn att_name(&self) -> &'static str {
        r#"WebsiteURL"#
    }
}

impl cfn_resources::CfnResource for CfnBucket {
    fn type_string(&self) -> &'static str {
        "AWS::S3::Bucket"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.accelerate_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.bucket_encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cors_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.lifecycle_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.logging_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.notification_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.object_lock_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ownership_controls
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.public_access_block_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.replication_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.versioning_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.website_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the days since the initiation of an incomplete multipart upload that Amazon S3    will wait before permanently removing all parts of the upload. For more information, see         Stopping Incomplete Multipart Uploads Using a Bucket Lifecycle Policy in the     Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AbortIncompleteMultipartUpload {
    ///
    /// Specifies the number of days after which Amazon S3 stops an incomplete multipart    upload.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DaysAfterInitiation")]
    pub days_after_initiation: i64,
}

impl cfn_resources::CfnResource for AbortIncompleteMultipartUpload {
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

/// Configures the transfer acceleration state for an Amazon S3 bucket. For more information, see       Amazon S3       Transfer Acceleration in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccelerateConfiguration {
    ///
    /// Specifies the transfer acceleration status of the bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Enabled | Suspended
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccelerationStatus")]
    pub acceleration_status: AccelerateConfigurationAccelerationStatusEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AccelerateConfigurationAccelerationStatusEnum {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,

    /// Suspended
    #[serde(rename = "Suspended")]
    Suspended,
}

impl Default for AccelerateConfigurationAccelerationStatusEnum {
    fn default() -> Self {
        AccelerateConfigurationAccelerationStatusEnum::Enabled
    }
}

impl cfn_resources::CfnResource for AccelerateConfiguration {
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

/// Specify this only in a cross-account scenario (where source and destination bucket     owners are not the same), and you want to change replica ownership to the AWS account     that owns the destination bucket. If this is not specified in the replication     configuration, the replicas are owned by same AWS account that owns the source     object.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessControlTranslation {
    ///
    /// Specifies the replica ownership. For default and valid values, see PUT bucket       replication in the Amazon S3 API Reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Destination
    ///
    /// Update requires: No interruption
    #[serde(rename = "Owner")]
    pub owner: AccessControlTranslationOwnerEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AccessControlTranslationOwnerEnum {
    /// Destination
    #[serde(rename = "Destination")]
    Destination,
}

impl Default for AccessControlTranslationOwnerEnum {
    fn default() -> Self {
        AccessControlTranslationOwnerEnum::Destination
    }
}

impl cfn_resources::CfnResource for AccessControlTranslation {
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

/// Specifies the configuration and any analyses for the analytics filter of an Amazon S3     bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalyticsConfiguration {
    ///
    /// The ID that identifies the analytics configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The prefix that an object must have to be included in the analytics results.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,

    ///
    /// Contains data related to access patterns to be collected and made available to analyze     the tradeoffs between different storage classes.
    ///
    /// Required: Yes
    ///
    /// Type: StorageClassAnalysis
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageClassAnalysis")]
    pub storage_class_analysis: StorageClassAnalysis,

    ///
    /// The tags to use when evaluating an analytics filter.
    ///
    /// The analytics only includes objects that meet the filter's criteria. If no filter is    specified, all of the contents of the bucket are included in the analysis.
    ///
    /// Required: No
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

impl cfn_resources::CfnResource for AnalyticsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.storage_class_analysis.validate()?;

        Ok(())
    }
}

/// Specifies default encryption for a bucket using server-side encryption with Amazon    S3-managed keys (SSE-S3) or AWS KMS-managed keys (SSE-KMS) bucket. For    information about the Amazon S3 default encryption feature, see Amazon S3 Default Encryption for S3     Buckets in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BucketEncryption {
    ///
    /// Specifies the default server-side-encryption configuration.
    ///
    /// Required: Yes
    ///
    /// Type: List of ServerSideEncryptionRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub server_side_encryption_configuration: Vec<ServerSideEncryptionRule>,
}

impl cfn_resources::CfnResource for BucketEncryption {
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

/// Describes the cross-origin access configuration for objects in an Amazon S3 bucket. For more     information, see Enabling       Cross-Origin Resource Sharing in the     Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CorsConfiguration {
    ///
    /// A set of origins and methods (cross-origin access that you want to allow). You can add     up to 100 rules to the configuration.
    ///
    /// Required: Yes
    ///
    /// Type: List of CorsRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "CorsRules")]
    pub cors_rules: Vec<CorsRule>,
}

impl cfn_resources::CfnResource for CorsConfiguration {
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

/// Specifies a cross-origin access rule for an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CorsRule {
    ///
    /// Headers that are specified in the Access-Control-Request-Headers header.     These headers are allowed in a preflight OPTIONS request. In response to any preflight     OPTIONS request, Amazon S3 returns any requested headers that are allowed.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_headers: Option<Vec<String>>,

    ///
    /// An HTTP method that you allow the origin to run.
    ///
    /// Allowed values: GET | PUT |     HEAD | POST | DELETE
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: Vec<String>,

    ///
    /// One or more origins you want customers to be able to access the bucket from.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: Vec<String>,

    ///
    /// One or more headers in the response that you want customers to be able to access from    their applications (for example, from a JavaScript XMLHttpRequest object).
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExposedHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_headers: Option<Vec<String>>,

    ///
    /// A unique identifier for this rule. The value must be no more than 255 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,

    ///
    /// The time in seconds that your browser is to cache the preflight response for the     specified resource.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
}

impl cfn_resources::CfnResource for CorsRule {
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

/// Specifies how data related to the storage class analysis for an Amazon S3 bucket should be     exported.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataExport {
    ///
    /// The place to store the data for an analysis.
    ///
    /// Required: Yes
    ///
    /// Type: Destination
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: Destination,

    ///
    /// The version of the output schema to use when exporting data. Must be     V_1.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: V_1
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputSchemaVersion")]
    pub output_schema_version: DataExportOutputSchemaVersionEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DataExportOutputSchemaVersionEnum {
    /// V_1
    #[serde(rename = "V_1")]
    V1,
}

impl Default for DataExportOutputSchemaVersionEnum {
    fn default() -> Self {
        DataExportOutputSchemaVersionEnum::V1
    }
}

impl cfn_resources::CfnResource for DataExport {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination.validate()?;

        Ok(())
    }
}

/// The container element for specifying the default Object Lock retention settings for new     objects placed in the specified bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DefaultRetention {
    ///
    /// The number of days that you want to specify for the default retention period. If Object    Lock is turned on, you must specify Mode and specify either Days or     Years.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,

    ///
    /// The default Object Lock retention mode you want to apply to new objects placed in the    specified bucket. If Object Lock is turned on, you must specify Mode and specify    either Days or Years.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: COMPLIANCE | GOVERNANCE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<DefaultRetentionModeEnum>,

    ///
    /// The number of years that you want to specify for the default retention period. If Object    Lock is turned on, you must specify Mode and specify either Days or     Years.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Years")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DefaultRetentionModeEnum {
    /// COMPLIANCE
    #[serde(rename = "COMPLIANCE")]
    Compliance,

    /// GOVERNANCE
    #[serde(rename = "GOVERNANCE")]
    Governance,
}

impl Default for DefaultRetentionModeEnum {
    fn default() -> Self {
        DefaultRetentionModeEnum::Compliance
    }
}

impl cfn_resources::CfnResource for DefaultRetention {
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

/// Specifies whether Amazon S3 replicates delete markers. If you specify a Filter     in your replication configuration, you must also include a       DeleteMarkerReplication element. If your Filter includes a       Tag element, the DeleteMarkerReplication       Status must be set to Disabled, because Amazon S3 does not support replicating     delete markers for tag-based rules. For an example configuration, see Basic Rule Configuration.
///
/// For more information about delete marker replication, see Basic Rule       Configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeleteMarkerReplication {
    ///
    /// Indicates whether to replicate delete markers. Disabled by default.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeleteMarkerReplicationStatusEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DeleteMarkerReplicationStatusEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for DeleteMarkerReplicationStatusEnum {
    fn default() -> Self {
        DeleteMarkerReplicationStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for DeleteMarkerReplication {
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

/// Specifies information about where to publish analysis or configuration results for an    Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Destination {
    ///
    /// The account ID that owns the destination S3 bucket. If no account ID is provided, the     owner is not validated before exporting data.
    ///
    /// Note Although this value is optional, we strongly recommend that you set it to help       prevent problems if the destination bucket ownership changes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_account_id: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the bucket to which data is exported.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketArn")]
    pub bucket_arn: cfn_resources::StrVal,

    ///
    /// Specifies the file format used when exporting data to Amazon S3.
    ///
    /// Allowed values: CSV | ORC |     Parquet
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: DestinationFormatEnum,

    ///
    /// The prefix to use when exporting data. The prefix is prepended to all results.
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

#[derive(Clone, Debug, serde::Serialize)]
pub enum DestinationFormatEnum {
    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// ORC
    #[serde(rename = "ORC")]
    Orc,

    /// Parquet
    #[serde(rename = "Parquet")]
    Parquet,
}

impl Default for DestinationFormatEnum {
    fn default() -> Self {
        DestinationFormatEnum::Csv
    }
}

impl cfn_resources::CfnResource for Destination {
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

/// Specifies encryption-related information for an Amazon S3 bucket that is a destination for     replicated objects.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionConfiguration {
    ///
    /// Specifies the ID (Key ARN or Alias ARN) of the customer managed AWS KMS key stored in     AWS Key Management Service (KMS) for the destination bucket. Amazon S3 uses this key to     encrypt replica objects. Amazon S3 only supports symmetric encryption KMS keys. For more     information, see Asymmetric keys in AWS       KMS in the         AWS Key Management Service Developer     Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicaKmsKeyID")]
    pub replica_kms_key_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EncryptionConfiguration {
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

/// Amazon S3 can send events to Amazon EventBridge whenever certain events happen in your    bucket, see Using     EventBridge in the Amazon S3 User Guide.
///
/// Unlike other destinations, delivery of events to EventBridge can be either enabled or    disabled for a bucket. If enabled, all events will be sent to EventBridge and you can use    EventBridge rules to route events to additional targets. For more information, see What Is Amazon     EventBridge in the Amazon EventBridge User Guide
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EventBridgeConfiguration {
    ///
    /// Enables delivery of events to Amazon EventBridge.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBridgeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for EventBridgeConfiguration {
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

/// Specifies the Amazon S3 object key name to filter on and whether to filter on the suffix or     prefix of the key name.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterRule {
    ///
    /// The object key name prefix or suffix identifying one or more objects to which the     filtering rule applies. The maximum length is 1,024 characters. Overlapping prefixes and     suffixes are not supported. For more information, see Configuring Event Notifications     in the Amazon S3 User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: prefix | suffix
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: FilterRuleNameEnum,

    ///
    /// The value that the filter searches for in object key names.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FilterRuleNameEnum {
    /// prefix
    #[serde(rename = "prefix")]
    Prefix,

    /// suffix
    #[serde(rename = "suffix")]
    Suffix,
}

impl Default for FilterRuleNameEnum {
    fn default() -> Self {
        FilterRuleNameEnum::Prefix
    }
}

impl cfn_resources::CfnResource for FilterRule {
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

/// Specifies the S3 Intelligent-Tiering configuration for an Amazon S3 bucket.
///
/// For information about the S3 Intelligent-Tiering storage class, see Storage class       for automatically optimizing frequently and infrequently accessed     objects.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IntelligentTieringConfiguration {
    ///
    /// The ID used to identify the S3 Intelligent-Tiering configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// An object key name prefix that identifies the subset of objects to which the rule    applies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the status of the configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: IntelligentTieringConfigurationStatusEnum,

    ///
    /// A container for a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,

    ///
    /// Specifies a list of S3 Intelligent-Tiering storage class tiers in the configuration. At    least one tier must be defined in the list. At most, you can specify two tiers in the list,    one for each available AccessTier: ARCHIVE_ACCESS and     DEEP_ARCHIVE_ACCESS.
    ///
    /// NoteYou only need Intelligent Tiering Configuration enabled on a bucket if you want to     automatically move objects stored in the Intelligent-Tiering storage class to Archive Access     or Deep Archive Access tiers.
    ///
    /// Required: Yes
    ///
    /// Type: List of Tiering
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tierings")]
    pub tierings: Vec<Tiering>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum IntelligentTieringConfigurationStatusEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for IntelligentTieringConfigurationStatusEnum {
    fn default() -> Self {
        IntelligentTieringConfigurationStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for IntelligentTieringConfiguration {
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

/// Specifies the inventory configuration for an Amazon S3 bucket. For more information, see       GET Bucket inventory in the Amazon S3 API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InventoryConfiguration {
    ///
    /// Contains information about where to publish the inventory results.
    ///
    /// Required: Yes
    ///
    /// Type: Destination
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: Destination,

    ///
    /// Specifies whether the inventory is enabled or disabled. If set to True, an     inventory list is generated. If set to False, no inventory list is     generated.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// The ID used to identify the inventory configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// Object versions to include in the inventory list. If set to All, the list     includes all the object versions, which adds the version-related fields       VersionId, IsLatest, and DeleteMarker to the     list. If set to Current, the list does not contain these version-related     fields.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: All | Current
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedObjectVersions")]
    pub included_object_versions: InventoryConfigurationIncludedObjectVersionsEnum,

    ///
    /// Contains the optional fields that are included in the inventory results.
    ///
    /// Valid values: Size | LastModifiedDate | StorageClass | ETag |     IsMultipartUploaded | ReplicationStatus | EncryptionStatus | ObjectLockRetainUntilDate |     ObjectLockMode | ObjectLockLegalHoldStatus | IntelligentTieringAccessTier | BucketKeyStatus
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionalFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_fields: Option<Vec<String>>,

    ///
    /// Specifies the inventory filter prefix.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the schedule for generating inventory results.
    ///
    /// Allowed values: Daily | Weekly
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleFrequency")]
    pub schedule_frequency: InventoryConfigurationScheduleFrequencyEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InventoryConfigurationIncludedObjectVersionsEnum {
    /// All
    #[serde(rename = "All")]
    All,

    /// Current
    #[serde(rename = "Current")]
    Current,
}

impl Default for InventoryConfigurationIncludedObjectVersionsEnum {
    fn default() -> Self {
        InventoryConfigurationIncludedObjectVersionsEnum::All
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InventoryConfigurationScheduleFrequencyEnum {
    /// Daily
    #[serde(rename = "Daily")]
    Daily,

    /// Weekly
    #[serde(rename = "Weekly")]
    Weekly,
}

impl Default for InventoryConfigurationScheduleFrequencyEnum {
    fn default() -> Self {
        InventoryConfigurationScheduleFrequencyEnum::Daily
    }
}

impl cfn_resources::CfnResource for InventoryConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination.validate()?;

        Ok(())
    }
}

/// Describes the AWS Lambda functions to invoke and the events for which to invoke     them.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LambdaConfiguration {
    ///
    /// The Amazon S3 bucket event for which to invoke the AWS Lambda function. For more information,     see Supported       Event Types in the Amazon S3 User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Event")]
    pub event: cfn_resources::StrVal,

    ///
    /// The filtering rules that determine which objects invoke the AWS Lambda    function. For example, you can create a filter so that only image files with a     .jpg extension invoke the function when they are added to the Amazon S3    bucket.
    ///
    /// Required: No
    ///
    /// Type: NotificationFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NotificationFilter>,

    ///
    /// The Amazon Resource Name (ARN) of the AWS Lambda function that Amazon S3 invokes when the     specified event type occurs.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Function")]
    pub function: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LambdaConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.filter.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the lifecycle configuration for objects in an Amazon S3 bucket. For more     information, see Object Lifecycle Management     in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LifecycleConfiguration {
    ///
    /// A lifecycle rule for individual objects in an Amazon S3 bucket.
    ///
    /// Required: Yes
    ///
    /// Type: List of Rule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,
}

impl cfn_resources::CfnResource for LifecycleConfiguration {
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

/// Describes where logs are stored and the prefix that Amazon S3 assigns to all log object    keys for a bucket. For examples and more information, see PUT Bucket logging in the     Amazon S3 API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingConfiguration {
    ///
    /// The name of the bucket where Amazon S3 should store server access log files. You can store    log files in any bucket that you own. By default, logs are stored in the bucket where the     LoggingConfiguration property is defined.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_bucket_name: Option<cfn_resources::StrVal>,

    ///
    /// A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets in a     single bucket, you can use a prefix to distinguish which log files came from which     bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogFilePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LoggingConfiguration {
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

/// A container specifying replication metrics-related settings enabling replication     metrics and events.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Metrics {
    ///
    /// A container specifying the time threshold for emitting the       s3:Replication:OperationMissedThreshold event.
    ///
    /// Required: No
    ///
    /// Type: ReplicationTimeValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_threshold: Option<ReplicationTimeValue>,

    ///
    /// Specifies whether the replication metrics are enabled.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: MetricsStatusEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MetricsStatusEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for MetricsStatusEnum {
    fn default() -> Self {
        MetricsStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for Metrics {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.event_threshold
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies a metrics configuration for the CloudWatch request metrics (specified by the    metrics configuration ID) from an Amazon S3 bucket. If you're updating an existing metrics    configuration, note that this is a full replacement of the existing metrics configuration. If    you don't include the elements you want to keep, they are erased. For examples, see AWS::S3::Bucket. For more information, see PUT Bucket metrics    in the Amazon S3 API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricsConfiguration {
    ///
    /// The access point that was used while performing operations on the object. The metrics    configuration only includes objects that meet the filter's criteria.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ID used to identify the metrics configuration. This can be any value you choose that    helps you identify your metrics configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The prefix that an object must have to be included in the metrics results.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,

    ///
    /// Specifies a list of tag filters to use as a metrics configuration filter. The metrics    configuration includes only objects that meet the filter's criteria.
    ///
    /// Required: No
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

impl cfn_resources::CfnResource for MetricsConfiguration {
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

/// Specifies when noncurrent object versions expire. Upon expiration, Amazon S3    permanently deletes the noncurrent object versions. You set this lifecycle configuration    action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's    lifetime.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NoncurrentVersionExpiration {
    ///
    /// Specifies how many noncurrent versions Amazon S3 will retain. If there are this many more recent     noncurrent versions, Amazon S3 will take the associated action. For more information about noncurrent    versions, see Lifecycle configuration elements    in the Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewerNoncurrentVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newer_noncurrent_versions: Option<i64>,

    ///
    /// Specifies the number of days an object is noncurrent before Amazon S3 can    perform the associated action. For information about the noncurrent days calculations, see     How     Amazon S3 Calculates When an Object Became Noncurrent in the Amazon S3     User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: i64,
}

impl cfn_resources::CfnResource for NoncurrentVersionExpiration {
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

/// Container for the transition rule that describes when noncurrent objects transition to the     STANDARD_IA, ONEZONE_IA, INTELLIGENT_TIERING,     GLACIER_IR, GLACIER, or DEEP_ARCHIVE storage class.    If your bucket is versioning-enabled (or versioning is suspended), you can set this action to    request that Amazon S3 transition noncurrent object versions to the STANDARD_IA,     ONEZONE_IA, INTELLIGENT_TIERING, GLACIER_IR,     GLACIER, or DEEP_ARCHIVE storage class at a specific period in the    object's lifetime. If you specify this property, don't specify the     NoncurrentVersionTransitions property.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NoncurrentVersionTransition {
    ///
    /// Specifies how many noncurrent versions Amazon S3 will retain. If there are this    many more recent noncurrent versions, Amazon S3 will take the associated action. For    more information about noncurrent versions, see Lifecycle configuration     elements in the Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewerNoncurrentVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newer_noncurrent_versions: Option<i64>,

    ///
    /// The class of storage used to store the object.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DEEP_ARCHIVE | GLACIER | GLACIER_IR | INTELLIGENT_TIERING | ONEZONE_IA | STANDARD_IA
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageClass")]
    pub storage_class: NoncurrentVersionTransitionStorageClassEnum,

    ///
    /// Specifies the number of days an object is noncurrent before Amazon S3 can perform the     associated action. For information about the noncurrent days calculations, see How       Amazon S3 Calculates How Long an Object Has Been Noncurrent in the       Amazon S3 User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitionInDays")]
    pub transition_in_days: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NoncurrentVersionTransitionStorageClassEnum {
    /// DEEP_ARCHIVE
    #[serde(rename = "DEEP_ARCHIVE")]
    Deeparchive,

    /// GLACIER
    #[serde(rename = "GLACIER")]
    Glacier,

    /// GLACIER_IR
    #[serde(rename = "GLACIER_IR")]
    Glacierir,

    /// INTELLIGENT_TIERING
    #[serde(rename = "INTELLIGENT_TIERING")]
    Intelligenttiering,

    /// ONEZONE_IA
    #[serde(rename = "ONEZONE_IA")]
    Onezoneia,

    /// STANDARD_IA
    #[serde(rename = "STANDARD_IA")]
    Standardia,
}

impl Default for NoncurrentVersionTransitionStorageClassEnum {
    fn default() -> Self {
        NoncurrentVersionTransitionStorageClassEnum::Deeparchive
    }
}

impl cfn_resources::CfnResource for NoncurrentVersionTransition {
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

/// Describes the notification configuration for an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NotificationConfiguration {
    ///
    /// Enables delivery of events to Amazon EventBridge.
    ///
    /// Required: No
    ///
    /// Type: EventBridgeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBridgeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_configuration: Option<EventBridgeConfiguration>,

    ///
    /// Describes the AWS Lambda functions to invoke and the events for which to invoke     them.
    ///
    /// Required: No
    ///
    /// Type: List of LambdaConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_configurations: Option<Vec<LambdaConfiguration>>,

    ///
    /// The Amazon Simple Queue Service queues to publish messages to and the events for which     to publish messages.
    ///
    /// Required: No
    ///
    /// Type: List of QueueConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueueConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_configurations: Option<Vec<QueueConfiguration>>,

    ///
    /// The topic to which notifications are sent and the events for which notifications are     generated.
    ///
    /// Required: No
    ///
    /// Type: List of TopicConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopicConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_configurations: Option<Vec<TopicConfiguration>>,
}

impl cfn_resources::CfnResource for NotificationConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.event_bridge_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies object key name filtering rules. For information about key name filtering, see     Configuring event notifications using object key name filtering in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NotificationFilter {
    ///
    /// A container for object key name prefix and suffix filtering rules.
    ///
    /// Required: Yes
    ///
    /// Type: S3KeyFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Key")]
    pub s3_key: S3KeyFilter,
}

impl cfn_resources::CfnResource for NotificationFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_key.validate()?;

        Ok(())
    }
}

/// Places an Object Lock configuration on the specified bucket. The rule specified in the    Object Lock configuration will be applied by default to every new object placed in the    specified bucket. For more information, see Locking Objects.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ObjectLockConfiguration {
    ///
    /// Indicates whether this bucket has an Object Lock configuration enabled. Enable       ObjectLockEnabled when you apply ObjectLockConfiguration to a     bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectLockEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_enabled: Option<ObjectLockConfigurationObjectLockEnabledEnum>,

    ///
    /// Specifies the Object Lock rule for the specified object. Enable this rule when you    apply ObjectLockConfiguration to a bucket. If Object Lock is turned on, bucket    settings require both Mode and a period of either Days or     Years. You cannot specify Days and Years at the same    time. For more information, see ObjectLockRule and DefaultRetention.
    ///
    /// Required: Conditional
    ///
    /// Type: ObjectLockRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<ObjectLockRule>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ObjectLockConfigurationObjectLockEnabledEnum {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for ObjectLockConfigurationObjectLockEnabledEnum {
    fn default() -> Self {
        ObjectLockConfigurationObjectLockEnabledEnum::Enabled
    }
}

impl cfn_resources::CfnResource for ObjectLockConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.rule.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the Object Lock rule for the specified object. Enable the this rule when you    apply ObjectLockConfiguration to a bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ObjectLockRule {
    ///
    /// The default Object Lock retention mode and period that you want to apply to new objects    placed in the specified bucket. If Object Lock is turned on, bucket settings require both     Mode and a period of either Days or Years. You cannot    specify Days and Years at the same time. For more information about    allowable values for mode and period, see DefaultRetention.
    ///
    /// Required: Conditional
    ///
    /// Type: DefaultRetention
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_retention: Option<DefaultRetention>,
}

impl cfn_resources::CfnResource for ObjectLockRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.default_retention
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the container element for Object Ownership rules.
///
/// S3 Object Ownership is an Amazon S3 bucket-level setting that you can use to disable    access control lists (ACLs) and take ownership of every object in your bucket, simplifying    access management for data stored in Amazon S3. For more information, see Controlling     ownership of objects and disabling ACLs in the Amazon S3 User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OwnershipControls {
    ///
    /// Specifies the container element for Object Ownership rules.
    ///
    /// Required: Yes
    ///
    /// Type: List of OwnershipControlsRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<OwnershipControlsRule>,
}

impl cfn_resources::CfnResource for OwnershipControls {
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

/// Specifies an Object Ownership rule.
///
/// S3 Object Ownership is an Amazon S3 bucket-level setting that you can use to disable    access control lists (ACLs) and take ownership of every object in your bucket, simplifying    access management for data stored in Amazon S3. For more information, see Controlling     ownership of objects and disabling ACLs in the Amazon S3 User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OwnershipControlsRule {
    ///
    /// Specifies an Object Ownership rule.
    ///
    /// Allowed values: BucketOwnerEnforced |     ObjectWriter | BucketOwnerPreferred
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectOwnership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_ownership: Option<OwnershipControlsRuleObjectOwnershipEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OwnershipControlsRuleObjectOwnershipEnum {
    /// BucketOwnerEnforced
    #[serde(rename = "BucketOwnerEnforced")]
    Bucketownerenforced,

    /// ObjectWriter
    #[serde(rename = "ObjectWriter")]
    Objectwriter,

    /// BucketOwnerPreferred
    #[serde(rename = "BucketOwnerPreferred")]
    Bucketownerpreferred,
}

impl Default for OwnershipControlsRuleObjectOwnershipEnum {
    fn default() -> Self {
        OwnershipControlsRuleObjectOwnershipEnum::Bucketownerenforced
    }
}

impl cfn_resources::CfnResource for OwnershipControlsRule {
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

/// The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You can     enable the configuration options in any combination. For more information about when Amazon S3     considers a bucket or object public, see The Meaning of "Public" in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PublicAccessBlockConfiguration {
    ///
    /// Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket     and objects in this bucket. Setting this element to TRUE causes the following     behavior:
    ///
    /// PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is        public.               PUT Object calls fail if the request includes a public ACL.               PUT Bucket calls fail if the request includes a public ACL.
    ///
    /// Enabling this setting doesn't affect existing policies or ACLs.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockPublicAcls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,

    ///
    /// Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this     element to TRUE causes Amazon S3 to reject calls to PUT Bucket policy if the     specified bucket policy allows public access.
    ///
    /// Enabling this setting doesn't affect existing bucket policies.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockPublicPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,

    ///
    /// Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this     bucket. Setting this element to TRUE causes Amazon S3 to ignore all public ACLs on     this bucket and objects in this bucket.
    ///
    /// Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't     prevent new public ACLs from being set.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnorePublicAcls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,

    ///
    /// Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting     this element to TRUE restricts access to this bucket to only AWS service principals and authorized users within this account if the bucket has     a public policy.
    ///
    /// Enabling this setting doesn't affect previously stored bucket policies, except that     public and cross-account access within any public bucket policy, including non-public     delegation to specific accounts, is blocked.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestrictPublicBuckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_public_buckets: Option<bool>,
}

impl cfn_resources::CfnResource for PublicAccessBlockConfiguration {
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

/// Specifies the configuration for publishing messages to an Amazon Simple Queue Service     (Amazon SQS) queue when Amazon S3 detects specified events.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct QueueConfiguration {
    ///
    /// The Amazon S3 bucket event about which you want to publish messages to Amazon SQS. For    more information, see Supported Event Types in the     Amazon S3 User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Event")]
    pub event: cfn_resources::StrVal,

    ///
    /// The filtering rules that determine which objects trigger notifications. For example, you    can create a filter so that Amazon S3 sends notifications only when image files with a     .jpg extension are added to the bucket. For more information, see Configuring event notifications using object key name filtering in the     Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: NotificationFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NotificationFilter>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon SQS queue to which Amazon S3 publishes a    message when it detects events of the specified type. FIFO queues are not allowed when    enabling an SQS queue as the event notification destination.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Queue")]
    pub queue: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for QueueConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.filter.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the redirect behavior of all requests to a website endpoint of an Amazon S3     bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedirectAllRequestsTo {
    ///
    /// Name of the host where requests are redirected.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostName")]
    pub host_name: cfn_resources::StrVal,

    ///
    /// Protocol to use when redirecting requests. The default is the protocol that is used in     the original request.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: http | https
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<RedirectAllRequestsToProtocolEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RedirectAllRequestsToProtocolEnum {
    /// http
    #[serde(rename = "http")]
    Http,

    /// https
    #[serde(rename = "https")]
    Https,
}

impl Default for RedirectAllRequestsToProtocolEnum {
    fn default() -> Self {
        RedirectAllRequestsToProtocolEnum::Http
    }
}

impl cfn_resources::CfnResource for RedirectAllRequestsTo {
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

/// Specifies how requests are redirected. In the event of an error, you can specify a     different error code to return.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedirectRule {
    ///
    /// The host name to use in the redirect request.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<cfn_resources::StrVal>,

    ///
    /// The HTTP redirect code to use on the response. Not required if one of the siblings is     present.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpRedirectCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_redirect_code: Option<cfn_resources::StrVal>,

    ///
    /// Protocol to use when redirecting requests. The default is the protocol that is used in     the original request.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: http | https
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<RedirectRuleProtocolEnum>,

    ///
    /// The object key prefix to use in the redirect request. For example, to redirect requests     for all pages with prefix docs/ (objects in the docs/ folder) to       documents/, you can set a condition block with KeyPrefixEquals     set to docs/ and in the Redirect set ReplaceKeyPrefixWith to       /documents. Not required if one of the siblings is present. Can be present     only if ReplaceKeyWith is not provided.
    ///
    /// ImportantReplacement must be made for object keys containing special characters (such as carriage returns) when using      XML requests. For more information, see       XML related object key constraints.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplaceKeyPrefixWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_key_prefix_with: Option<cfn_resources::StrVal>,

    ///
    /// The specific object key to use in the redirect request. For example, redirect request to       error.html. Not required if one of the siblings is present. Can be present     only if ReplaceKeyPrefixWith is not provided.
    ///
    /// ImportantReplacement must be made for object keys containing special characters (such as carriage returns) when using      XML requests. For more information, see       XML related object key constraints.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplaceKeyWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_key_with: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RedirectRuleProtocolEnum {
    /// http
    #[serde(rename = "http")]
    Http,

    /// https
    #[serde(rename = "https")]
    Https,
}

impl Default for RedirectRuleProtocolEnum {
    fn default() -> Self {
        RedirectRuleProtocolEnum::Http
    }
}

impl cfn_resources::CfnResource for RedirectRule {
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

/// A filter that you can specify for selection for modifications on replicas.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicaModifications {
    ///
    /// Specifies whether Amazon S3 replicates modifications on replicas.
    ///
    /// Allowed values: Enabled | Disabled
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: ReplicaModificationsStatusEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ReplicaModificationsStatusEnum {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,

    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,
}

impl Default for ReplicaModificationsStatusEnum {
    fn default() -> Self {
        ReplicaModificationsStatusEnum::Enabled
    }
}

impl cfn_resources::CfnResource for ReplicaModifications {
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

/// A container for replication rules. You can add up to 1,000 rules. The maximum size of a     replication configuration is 2 MB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationConfiguration {
    ///
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that Amazon S3 assumes when     replicating objects. For more information, see How to Set Up Replication     in the Amazon S3 User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: cfn_resources::StrVal,

    ///
    /// A container for one or more replication rules. A replication configuration must have at     least one rule and can contain a maximum of 1,000 rules.
    ///
    /// Required: Yes
    ///
    /// Type: List of ReplicationRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<ReplicationRule>,
}

impl cfn_resources::CfnResource for ReplicationConfiguration {
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

/// A container for information about the replication destination and its configurations     including enabling the S3 Replication Time Control (S3 RTC).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationDestination {
    ///
    /// Specify this only in a cross-account scenario (where source and destination bucket     owners are not the same), and you want to change replica ownership to the AWS account     that owns the destination bucket. If this is not specified in the replication     configuration, the replicas are owned by same AWS account that owns the source     object.
    ///
    /// Required: No
    ///
    /// Type: AccessControlTranslation
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlTranslation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_translation: Option<AccessControlTranslation>,

    ///
    /// Destination bucket owner account ID. In a cross-account scenario, if you direct Amazon S3    to change replica ownership to the AWS account that owns the destination    bucket by specifying the AccessControlTranslation property, this is the account    ID of the destination bucket owner. For more information, see Cross-Region Replication Additional     Configuration: Change Replica Owner in the Amazon S3 User    Guide.
    ///
    /// If you specify the AccessControlTranslation property, the     Account property is required.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store the     results.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: cfn_resources::StrVal,

    ///
    /// Specifies encryption-related information.
    ///
    /// Required: No
    ///
    /// Type: EncryptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,

    ///
    /// A container specifying replication metrics-related settings enabling replication     metrics and events.
    ///
    /// Required: No
    ///
    /// Type: Metrics
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,

    ///
    /// A container specifying S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time     when all objects and operations on objects must be replicated. Must be specified together     with a Metrics block.
    ///
    /// Required: No
    ///
    /// Type: ReplicationTime
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_time: Option<ReplicationTime>,

    ///
    /// The storage class to use when replicating objects, such as S3 Standard or reduced     redundancy. By default, Amazon S3 uses the storage class of the source object to create the     object replica.
    ///
    /// For valid values, see the StorageClass element of the PUT Bucket       replication action in the Amazon S3 API Reference.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEEP_ARCHIVE | GLACIER | GLACIER_IR | INTELLIGENT_TIERING | ONEZONE_IA | OUTPOSTS | REDUCED_REDUNDANCY | SNOW | STANDARD | STANDARD_IA
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<ReplicationDestinationStorageClassEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ReplicationDestinationStorageClassEnum {
    /// DEEP_ARCHIVE
    #[serde(rename = "DEEP_ARCHIVE")]
    Deeparchive,

    /// GLACIER
    #[serde(rename = "GLACIER")]
    Glacier,

    /// GLACIER_IR
    #[serde(rename = "GLACIER_IR")]
    Glacierir,

    /// INTELLIGENT_TIERING
    #[serde(rename = "INTELLIGENT_TIERING")]
    Intelligenttiering,

    /// ONEZONE_IA
    #[serde(rename = "ONEZONE_IA")]
    Onezoneia,

    /// OUTPOSTS
    #[serde(rename = "OUTPOSTS")]
    Outposts,

    /// REDUCED_REDUNDANCY
    #[serde(rename = "REDUCED_REDUNDANCY")]
    Reducedredundancy,

    /// SNOW
    #[serde(rename = "SNOW")]
    Snow,

    /// STANDARD
    #[serde(rename = "STANDARD")]
    Standard,

    /// STANDARD_IA
    #[serde(rename = "STANDARD_IA")]
    Standardia,
}

impl Default for ReplicationDestinationStorageClassEnum {
    fn default() -> Self {
        ReplicationDestinationStorageClassEnum::Deeparchive
    }
}

impl cfn_resources::CfnResource for ReplicationDestination {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.access_control_translation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.encryption_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.metrics.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.replication_time
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies which Amazon S3 objects to replicate and where to store the replicas.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationRule {
    ///
    /// Specifies whether Amazon S3 replicates delete markers. If you specify a Filter     in your replication configuration, you must also include a       DeleteMarkerReplication element. If your Filter includes a       Tag element, the DeleteMarkerReplication       Status must be set to Disabled, because Amazon S3 does not support replicating     delete markers for tag-based rules. For an example configuration, see Basic Rule Configuration.
    ///
    /// For more information about delete marker replication, see Basic Rule       Configuration.
    ///
    /// NoteIf you are using an earlier version of the replication configuration, Amazon S3 handles       replication of delete markers differently. For more information, see Backward Compatibility.
    ///
    /// Required: No
    ///
    /// Type: DeleteMarkerReplication
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteMarkerReplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_marker_replication: Option<DeleteMarkerReplication>,

    ///
    /// A container for information about the replication destination and its configurations     including enabling the S3 Replication Time Control (S3 RTC).
    ///
    /// Required: Yes
    ///
    /// Type: ReplicationDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: ReplicationDestination,

    ///
    /// A filter that identifies the subset of objects to which the replication rule applies. A     Filter must specify exactly one Prefix, TagFilter, or    an And child element. The use of the filter field indicates that this is a V2    replication configuration. This field isn't supported in a V1 replication    configuration.
    ///
    /// NoteV1 replication configuration only supports filtering by key prefix. To filter using a V1     replication configuration, add the Prefix directly as a child element of the      Rule element.
    ///
    /// Required: No
    ///
    /// Type: ReplicationRuleFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReplicationRuleFilter>,

    ///
    /// A unique identifier for the rule. The maximum value is 255 characters. If you don't    specify a value, AWS CloudFormation generates a random ID. When using a V2    replication configuration this property is capitalized as "ID".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,

    ///
    /// An object key name prefix that identifies the object or objects to which the rule applies.    The maximum prefix length is 1,024 characters. To include all objects in a bucket, specify an    empty string. To filter using a V1 replication configuration, add the Prefix    directly as a child element of the Rule element.
    ///
    /// ImportantReplacement must be made for object keys containing special characters (such as carriage     returns) when using XML requests. For more information, see XML      related object key constraints.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,

    ///
    /// The priority indicates which rule has precedence whenever two or more replication rules     conflict. Amazon S3 will attempt to replicate objects according to all replication rules.     However, if there are two or more rules with the same destination bucket, then objects will     be replicated according to the rule with the highest priority. The higher the number, the     higher the priority.
    ///
    /// For more information, see Replication in the       Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    ///
    /// A container that describes additional filters for identifying the source objects that you    want to replicate. You can choose to enable or disable the replication of these    objects.
    ///
    /// Required: No
    ///
    /// Type: SourceSelectionCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceSelectionCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_selection_criteria: Option<SourceSelectionCriteria>,

    ///
    /// Specifies whether the rule is enabled.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: ReplicationRuleStatusEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ReplicationRuleStatusEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for ReplicationRuleStatusEnum {
    fn default() -> Self {
        ReplicationRuleStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for ReplicationRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.delete_marker_replication
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.destination.validate()?;

        self.filter.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.source_selection_criteria
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A container for specifying rule filters. The filters determine the subset of objects to    which the rule applies. This element is required only if you specify more than one filter.
///
/// For example:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationRuleAndOperator {
    ///
    /// An object key name prefix that identifies the subset of objects to which the rule     applies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,

    ///
    /// An array of tags containing key and value pairs.
    ///
    /// Required: No
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

impl cfn_resources::CfnResource for ReplicationRuleAndOperator {
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

/// A filter that identifies the subset of objects to which the replication rule applies. A     Filter must specify exactly one Prefix, TagFilter, or    an And child element.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationRuleFilter {
    ///
    /// A container for specifying rule filters. The filters determine the subset of objects to    which the rule applies. This element is required only if you specify more than one filter. For    example:
    ///
    /// If you specify both a Prefix and a TagFilter, wrap these      filters in an And tag.        If you specify a filter based on multiple tags, wrap the TagFilter      elements in an And tag.
    ///
    /// Required: No
    ///
    /// Type: ReplicationRuleAndOperator
    ///
    /// Update requires: No interruption
    #[serde(rename = "And")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<ReplicationRuleAndOperator>,

    ///
    /// An object key name prefix that identifies the subset of objects to which the rule     applies.
    ///
    /// ImportantReplacement must be made for object keys containing special characters (such as carriage returns) when using      XML requests. For more information, see       XML related object key constraints.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,

    ///
    /// A container for specifying a tag key and value.
    ///
    /// The rule applies only to objects that have the tag in their tag set.
    ///
    /// Required: No
    ///
    /// Type: TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
}

impl cfn_resources::CfnResource for ReplicationRuleFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.and.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.tag_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A container specifying S3 Replication Time Control (S3 RTC) related information, including whether S3 RTC is     enabled and the time when all objects and operations on objects must be replicated. Must be     specified together with a Metrics block.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationTime {
    ///
    /// Specifies whether the replication time is enabled.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: ReplicationTimeStatusEnum,

    ///
    /// A container specifying the time by which replication should be complete for all objects     and operations on objects.
    ///
    /// Required: Yes
    ///
    /// Type: ReplicationTimeValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: ReplicationTimeValue,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ReplicationTimeStatusEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for ReplicationTimeStatusEnum {
    fn default() -> Self {
        ReplicationTimeStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for ReplicationTime {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.time.validate()?;

        Ok(())
    }
}

/// A container specifying the time value for S3 Replication Time Control (S3 RTC) and replication metrics       EventThreshold.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationTimeValue {
    ///
    /// Contains an integer specifying time in minutes.
    ///
    /// Valid value: 15
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Minutes")]
    pub minutes: i64,
}

impl cfn_resources::CfnResource for ReplicationTimeValue {
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

/// Specifies the redirect behavior and when a redirect is applied. For more information     about routing rules, see Configuring advanced conditional redirects in the       Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RoutingRule {
    ///
    /// Container for redirect information. You can redirect requests to another host, to     another page, or with another protocol. In the event of an error, you can specify a     different error code to return.
    ///
    /// Required: Yes
    ///
    /// Type: RedirectRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedirectRule")]
    pub redirect_rule: RedirectRule,

    ///
    /// A container for describing a condition that must be met for the specified redirect to     apply. For example, 1. If request is for pages in the /docs folder, redirect     to the /documents folder. 2. If request results in HTTP error 4xx, redirect     request to another host where you might process the error.
    ///
    /// Required: No
    ///
    /// Type: RoutingRuleCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoutingRuleCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_condition: Option<RoutingRuleCondition>,
}

impl cfn_resources::CfnResource for RoutingRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.redirect_rule.validate()?;

        self.routing_rule_condition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A container for describing a condition that must be met for the specified redirect to     apply. For example, 1. If request is for pages in the /docs folder, redirect     to the /documents folder. 2. If request results in HTTP error 4xx, redirect     request to another host where you might process the error.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RoutingRuleCondition {
    ///
    /// The HTTP error code when the redirect is applied. In the event of an error, if the error    code equals this value, then the specified redirect is applied.
    ///
    /// Required when parent element Condition is specified and sibling     KeyPrefixEquals is not specified. If both are specified, then both must be true    for the redirect to be applied.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_error_code_returned_equals: Option<cfn_resources::StrVal>,

    ///
    /// The object key name prefix when the redirect is applied. For example, to redirect requests    for ExamplePage.html, the key prefix will be ExamplePage.html. To    redirect request for all pages with the prefix docs/, the key prefix will be     /docs, which identifies all objects in the docs/ folder.
    ///
    /// Required when the parent element Condition is specified and sibling     HttpErrorCodeReturnedEquals is not specified. If both conditions are specified,    both must be true for the redirect to be applied.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyPrefixEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix_equals: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for RoutingRuleCondition {
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

/// Specifies lifecycle rules for an Amazon S3 bucket. For more information, see Put Bucket     Lifecycle Configuration in the Amazon S3 API Reference.
///
/// You must specify at least one of the following properties:     AbortIncompleteMultipartUpload, ExpirationDate,     ExpirationInDays, NoncurrentVersionExpirationInDays,     NoncurrentVersionTransition, NoncurrentVersionTransitions,     Transition, or Transitions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Rule {
    ///
    /// Specifies a lifecycle rule that stops incomplete multipart uploads to an Amazon S3    bucket.
    ///
    /// Required: Conditional
    ///
    /// Type: AbortIncompleteMultipartUpload
    ///
    /// Update requires: No interruption
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,

    ///
    /// Indicates when objects are deleted from Amazon S3 and Amazon S3 Glacier. The date value    must be in ISO 8601 format. The time is always midnight UTC. If you specify an expiration and    transition time, you must use the same time unit for both properties (either in days or by    date). The expiration time must also be later than the transition time.
    ///
    /// Required: Conditional
    ///
    /// Type: Timestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<cfn_resources::StrVal>,

    ///
    /// Indicates the number of days after creation when objects are deleted from Amazon S3 and    Amazon S3 Glacier. If you specify an expiration and transition time, you must use the same    time unit for both properties (either in days or by date). The expiration time must also be    later than the transition time.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpirationInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_in_days: Option<i64>,

    ///
    /// Indicates whether Amazon S3 will remove a delete marker without any noncurrent versions.    If set to true, the delete marker will be removed if there are no noncurrent versions. This    cannot be specified with ExpirationInDays, ExpirationDate, or     TagFilters.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_object_delete_marker: Option<bool>,

    ///
    /// Unique identifier for the rule. The value can't be longer than 255 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies when noncurrent object versions expire. Upon expiration, Amazon S3    permanently deletes the noncurrent object versions. You set this lifecycle configuration    action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's    lifetime.
    ///
    /// Required: No
    ///
    /// Type: NoncurrentVersionExpiration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoncurrentVersionExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,

    ///
    /// (Deprecated.) For buckets with versioning enabled (or suspended), specifies the time, in    days, between when a new version of the object is uploaded to the bucket and when old versions    of the object expire. When object versions expire, Amazon S3 permanently deletes them. If you    specify a transition and expiration time, the expiration time must be later than the    transition time.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoncurrentVersionExpirationInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_expiration_in_days: Option<i64>,

    ///
    /// (Deprecated.) For buckets with versioning enabled (or suspended), specifies when    non-current objects transition to a specified storage class. If you specify a transition and    expiration time, the expiration time must be later than the transition time. If you specify    this property, don't specify the NoncurrentVersionTransitions property.
    ///
    /// Required: Conditional
    ///
    /// Type: NoncurrentVersionTransition
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoncurrentVersionTransition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,

    ///
    /// For buckets with versioning enabled (or suspended), one or more transition rules that    specify when non-current objects transition to a specified storage class. If you specify a    transition and expiration time, the expiration time must be later than the transition time. If    you specify this property, don't specify the NoncurrentVersionTransition    property.
    ///
    /// Required: Conditional
    ///
    /// Type: List of NoncurrentVersionTransition
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoncurrentVersionTransitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_transitions: Option<Vec<NoncurrentVersionTransition>>,

    ///
    /// Specifies the minimum object size in bytes for this rule to apply to. Objects must be larger than this value in bytes. For more information    about size based rules, see Lifecycle configuration using size-based rules in the Amazon S3 User     Guide.
    ///
    /// Required: No
    ///
    /// Type: Long
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectSizeGreaterThan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_greater_than: Option<i64>,

    ///
    /// Specifies the maximum object size in bytes for this rule to apply to. Objects must be smaller than this value in bytes. For more information    about sized based rules, see Lifecycle configuration using size-based rules in the Amazon S3 User     Guide.
    ///
    /// Required: No
    ///
    /// Type: Long
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectSizeLessThan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_less_than: Option<i64>,

    ///
    /// Object key prefix that identifies one or more objects to which this rule applies.
    ///
    /// ImportantReplacement must be made for object keys containing special characters (such as carriage returns) when using      XML requests. For more information, see       XML related object key constraints.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,

    ///
    /// If Enabled, the rule is currently being applied. If Disabled,     the rule is not currently being applied.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: RuleStatusEnum,

    ///
    /// Tags to use to identify a subset of objects to which the lifecycle rule applies.
    ///
    /// Required: No
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,

    ///
    /// (Deprecated.) Specifies when an object transitions to a specified storage class. If you    specify an expiration and transition time, you must use the same time unit for both properties    (either in days or by date). The expiration time must also be later than the transition time.    If you specify this property, don't specify the Transitions property.
    ///
    /// Required: Conditional
    ///
    /// Type: Transition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Transition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<Transition>,

    ///
    /// One or more transition rules that specify when an object transitions to a specified    storage class. If you specify an expiration and transition time, you must use the same time    unit for both properties (either in days or by date). The expiration time must also be later    than the transition time. If you specify this property, don't specify the     Transition property.
    ///
    /// Required: Conditional
    ///
    /// Type: List of Transition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Transitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<Transition>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RuleStatusEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for RuleStatusEnum {
    fn default() -> Self {
        RuleStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for Rule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.abort_incomplete_multipart_upload
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.noncurrent_version_expiration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.noncurrent_version_transition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.transition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A container for object key name prefix and suffix filtering rules. For more information about object key name filtering, see Configuring event notifications using object key name filtering in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3KeyFilter {
    ///
    /// A list of containers for the key-value pair that defines the criteria for the filter     rule.
    ///
    /// Required: Yes
    ///
    /// Type: List of FilterRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<FilterRule>,
}

impl cfn_resources::CfnResource for S3KeyFilter {
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

/// Describes the default server-side encryption to apply to new objects in the bucket. If a     PUT Object request doesn't specify any server-side encryption, this default encryption will     be applied. If you don't specify a customer managed key at configuration, Amazon S3 automatically creates     an AWS KMS key in your AWS account the first time that you add an object encrypted     with SSE-KMS to a bucket. By default, Amazon S3 uses this KMS key for SSE-KMS. For more     information, see PUT Bucket encryption in     the Amazon S3 API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerSideEncryptionByDefault {
    ///
    /// KMS key ID to use for the default encryption. This parameter is allowed if SSEAlgorithm is    aws:kms.
    ///
    /// You can specify the key ID or the Amazon Resource Name (ARN) of the CMK. However, if you    are using encryption with cross-account operations, you must use a fully qualified CMK ARN.    For more information, see Using encryption for cross-account operations.
    ///
    /// For example:
    ///
    /// Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab        Key ARN:       arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab
    ///
    /// ImportantAmazon S3 only supports symmetric KMS keys and not asymmetric KMS keys. For more     information, see Using Symmetric and Asymmetric      Keys in the AWS Key Management Service Developer      Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KMSMasterKeyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kmsmaster_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Server-side encryption algorithm to use for the default encryption.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AES256 | aws:kms
    ///
    /// Update requires: No interruption
    #[serde(rename = "SSEAlgorithm")]
    pub ssealgorithm: ServerSideEncryptionByDefaultSSEAlgorithmEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ServerSideEncryptionByDefaultSSEAlgorithmEnum {
    /// AES256
    #[serde(rename = "AES256")]
    Aes256,

    /// aws:kms
    #[serde(rename = "aws:kms")]
    Awskms,
}

impl Default for ServerSideEncryptionByDefaultSSEAlgorithmEnum {
    fn default() -> Self {
        ServerSideEncryptionByDefaultSSEAlgorithmEnum::Aes256
    }
}

impl cfn_resources::CfnResource for ServerSideEncryptionByDefault {
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

/// Specifies the default server-side encryption configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerSideEncryptionRule {
    ///
    /// Specifies whether Amazon S3 should use an S3 Bucket Key with server-side encryption using    KMS (SSE-KMS) for new objects in the bucket. Existing objects are not affected. Setting the     BucketKeyEnabled element to true causes Amazon S3 to use an S3    Bucket Key. By default, S3 Bucket Key is not enabled.
    ///
    /// For more information, see Amazon S3 Bucket Keys in the     Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,

    ///
    /// Specifies the default server-side encryption to apply to new objects in the bucket. If a     PUT Object request doesn't specify any server-side encryption, this default encryption will     be applied.
    ///
    /// Required: No
    ///
    /// Type: ServerSideEncryptionByDefault
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerSideEncryptionByDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
}

impl cfn_resources::CfnResource for ServerSideEncryptionRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.server_side_encryption_by_default
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A container that describes additional filters for identifying the source objects that you    want to replicate. You can choose to enable or disable the replication of these    objects.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceSelectionCriteria {
    ///
    /// A filter that you can specify for selection for modifications on replicas.
    ///
    /// Required: No
    ///
    /// Type: ReplicaModifications
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicaModifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_modifications: Option<ReplicaModifications>,

    ///
    /// A container for filter information for the selection of Amazon S3 objects encrypted with     AWS KMS.
    ///
    /// Required: No
    ///
    /// Type: SseKmsEncryptedObjects
    ///
    /// Update requires: No interruption
    #[serde(rename = "SseKmsEncryptedObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

impl cfn_resources::CfnResource for SourceSelectionCriteria {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.replica_modifications
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sse_kms_encrypted_objects
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A container for filter information for the selection of S3 objects encrypted with AWS     KMS.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SseKmsEncryptedObjects {
    ///
    /// Specifies whether Amazon S3 replicates objects created with server-side encryption using an     AWS KMS key stored in AWS Key Management Service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: SseKmsEncryptedObjectsStatusEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SseKmsEncryptedObjectsStatusEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for SseKmsEncryptedObjectsStatusEnum {
    fn default() -> Self {
        SseKmsEncryptedObjectsStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for SseKmsEncryptedObjects {
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

/// Specifies data related to access patterns to be collected and made available to analyze     the tradeoffs between different storage classes for an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StorageClassAnalysis {
    ///
    /// Specifies how data related to the storage class analysis for an Amazon S3 bucket should be     exported.
    ///
    /// Required: No
    ///
    /// Type: DataExport
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataExport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_export: Option<DataExport>,
}

impl cfn_resources::CfnResource for StorageClassAnalysis {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_export
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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

/// Specifies tags to use to identify a subset of objects for an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagFilter {
    ///
    /// The tag key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The tag value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TagFilter {
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

/// The S3 Intelligent-Tiering storage class is designed to optimize storage costs by     automatically moving data to the most cost-effective storage access tier, without     additional operational overhead.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tiering {
    ///
    /// S3 Intelligent-Tiering access tier. See Storage class       for automatically optimizing frequently and infrequently accessed objects for a     list of access tiers in the S3 Intelligent-Tiering storage class.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ARCHIVE_ACCESS | DEEP_ARCHIVE_ACCESS
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessTier")]
    pub access_tier: TieringAccessTierEnum,

    ///
    /// The number of consecutive days of no access after which an object will be eligible to be     transitioned to the corresponding tier. The minimum number of days specified for     Archive Access tier must be at least 90 days and Deep Archive Access tier must be at least     180 days. The maximum can be up to 2 years (730 days).
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Days")]
    pub days: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TieringAccessTierEnum {
    /// ARCHIVE_ACCESS
    #[serde(rename = "ARCHIVE_ACCESS")]
    Archiveaccess,

    /// DEEP_ARCHIVE_ACCESS
    #[serde(rename = "DEEP_ARCHIVE_ACCESS")]
    Deeparchiveaccess,
}

impl Default for TieringAccessTierEnum {
    fn default() -> Self {
        TieringAccessTierEnum::Archiveaccess
    }
}

impl cfn_resources::CfnResource for Tiering {
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

/// A container for specifying the configuration for publication of messages to an Amazon     Simple Notification Service (Amazon SNS) topic when Amazon S3 detects specified events.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicConfiguration {
    ///
    /// The Amazon S3 bucket event about which to send notifications. For more information, see       Supported       Event Types in the Amazon S3 User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Event")]
    pub event: cfn_resources::StrVal,

    ///
    /// The filtering rules that determine for which objects to send notifications. For example,    you can create a filter so that Amazon S3 sends notifications only when image files with a     .jpg extension are added to the bucket.
    ///
    /// Required: No
    ///
    /// Type: NotificationFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NotificationFilter>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon SNS topic to which Amazon S3 publishes a message     when it detects events of the specified type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Topic")]
    pub topic: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TopicConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.filter.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies when an object transitions to a specified storage class. For more information     about Amazon S3 lifecycle configuration rules, see Transitioning       Objects Using Amazon S3 Lifecycle in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Transition {
    ///
    /// The storage class to which you want the object to transition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DEEP_ARCHIVE | GLACIER | GLACIER_IR | INTELLIGENT_TIERING | ONEZONE_IA | STANDARD_IA
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageClass")]
    pub storage_class: TransitionStorageClassEnum,

    ///
    /// Indicates when objects are transitioned to the specified storage class. The date value     must be in ISO 8601 format. The time is always midnight UTC.
    ///
    /// Required: Conditional
    ///
    /// Type: Timestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_date: Option<cfn_resources::StrVal>,

    ///
    /// Indicates the number of days after creation when objects are transitioned to the     specified storage class. The value must be a positive integer.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitionInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_in_days: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TransitionStorageClassEnum {
    /// DEEP_ARCHIVE
    #[serde(rename = "DEEP_ARCHIVE")]
    Deeparchive,

    /// GLACIER
    #[serde(rename = "GLACIER")]
    Glacier,

    /// GLACIER_IR
    #[serde(rename = "GLACIER_IR")]
    Glacierir,

    /// INTELLIGENT_TIERING
    #[serde(rename = "INTELLIGENT_TIERING")]
    Intelligenttiering,

    /// ONEZONE_IA
    #[serde(rename = "ONEZONE_IA")]
    Onezoneia,

    /// STANDARD_IA
    #[serde(rename = "STANDARD_IA")]
    Standardia,
}

impl Default for TransitionStorageClassEnum {
    fn default() -> Self {
        TransitionStorageClassEnum::Deeparchive
    }
}

impl cfn_resources::CfnResource for Transition {
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

/// Describes the versioning state of an Amazon S3 bucket. For more information, see PUT       Bucket versioning in the Amazon S3 API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VersioningConfiguration {
    ///
    /// The versioning state of the bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Enabled | Suspended
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: VersioningConfigurationStatusEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum VersioningConfigurationStatusEnum {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,

    /// Suspended
    #[serde(rename = "Suspended")]
    Suspended,
}

impl Default for VersioningConfigurationStatusEnum {
    fn default() -> Self {
        VersioningConfigurationStatusEnum::Enabled
    }
}

impl cfn_resources::CfnResource for VersioningConfiguration {
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

/// Specifies website configuration parameters for an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebsiteConfiguration {
    ///
    /// The name of the error document for the website.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_document: Option<cfn_resources::StrVal>,

    ///
    /// The name of the index document for the website.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_document: Option<cfn_resources::StrVal>,

    ///
    /// The redirect behavior for every request to this bucket's website endpoint.
    ///
    /// ImportantIf you specify this property, you can't specify any other property.
    ///
    /// Required: No
    ///
    /// Type: RedirectAllRequestsTo
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedirectAllRequestsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,

    ///
    /// Rules that define when a redirect is applied and the redirect behavior.
    ///
    /// Required: No
    ///
    /// Type: List of RoutingRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoutingRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<Vec<RoutingRule>>,
}

impl cfn_resources::CfnResource for WebsiteConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.redirect_all_requests_to
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
