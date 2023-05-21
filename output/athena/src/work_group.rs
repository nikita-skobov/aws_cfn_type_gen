

/// The AWS::Athena::WorkGroup resource specifies an Amazon Athena workgroup,       which contains a name, description, creation time, state, and other configuration,       listed under WorkGroupConfiguration. Each workgroup enables you to       isolate queries for you or your group from other queries in the same account. For more       information, see CreateWorkGroup in       the Amazon Athena API Reference.
#[derive(Default, serde::Serialize)]
pub struct CfnWorkGroup {


    /// 
    /// The configuration of the workgroup, which includes the location in Amazon S3 where       query results are stored, the encryption option, if any, used for query results, whether       Amazon CloudWatch Metrics are enabled for the workgroup, and the limit for the amount of       bytes scanned (cutoff) per query, if it is specified. The EnforceWorkGroupConfiguration option determines whether workgroup       settings override client-side query settings.
    /// 
    /// Required: No
    ///
    /// Type: WorkGroupConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkGroupConfiguration")]
    pub work_group_configuration: Option<WorkGroupConfiguration>,


    /// 
    /// The option to delete a workgroup and its contents even if the workgroup contains any       named queries. The default is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecursiveDeleteOption")]
    pub recursive_delete_option: Option<bool>,


    /// 
    /// The tags (key-value pairs) to associate with this resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The workgroup name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-zA-Z0-9._-]{1,128}
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The workgroup description.
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


    /// 
    /// The state of the workgroup: ENABLED or DISABLED.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: Option<String>,

}


/// If query results are encrypted in Amazon S3, indicates the encryption option used (for       example, SSE_KMS or CSE_KMS) and key information.
#[derive(Default, serde::Serialize)]
pub struct EncryptionConfiguration {


    /// 
    /// For SSE_KMS and CSE_KMS, this is the KMS key ARN or       ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<String>,


    /// 
    /// Indicates whether Amazon S3 server-side encryption with Amazon S3-managed keys         (SSE_S3), server-side encryption with KMS-managed keys         (SSE_KMS), or client-side encryption with KMS-managed keys         (CSE_KMS) is used.
    /// 
    /// If a query runs in a workgroup and the workgroup overrides client-side settings, then       the workgroup's setting for encryption is used. It specifies whether query results must       be encrypted, for all queries that run in this workgroup.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CSE_KMS | SSE_KMS | SSE_S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionOption")]
    pub encryption_option: String,

}


/// The AclConfiguration property type specifies Property description not available. for an AWS::Athena::WorkGroup.
#[derive(Default, serde::Serialize)]
pub struct AclConfiguration {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3AclOption")]
    pub s3_acl_option: String,

}


/// The configuration of the workgroup, which includes the location in Amazon S3 where       query results are stored, the encryption option, if any, used for query results, whether       Amazon CloudWatch Metrics are enabled for the workgroup, and the limit for the amount of       bytes scanned (cutoff) per query, if it is specified. The EnforceWorkGroupConfiguration option determines whether workgroup       settings override client-side query settings.
#[derive(Default, serde::Serialize)]
pub struct WorkGroupConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: EngineVersion
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<EngineVersion>,


    /// 
    /// If set to "true", the settings for the workgroup override client-side settings. If set       to "false", client-side settings are used. For more information, see Workgroup Settings Override Client-Side Settings.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnforceWorkGroupConfiguration")]
    pub enforce_work_group_configuration: Option<bool>,


    /// 
    /// If set to true, allows members assigned to a workgroup to reference       Amazon S3 Requester Pays buckets in queries. If set to false, workgroup       members cannot query data from Requester Pays buckets, and queries that retrieve data       from Requester Pays buckets cause an error. The default is false. For more       information about Requester Pays buckets, see Requester Pays Buckets       in the Amazon Simple Storage Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequesterPaysEnabled")]
    pub requester_pays_enabled: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRole")]
    pub execution_role: Option<String>,


    /// 
    /// The upper limit (cutoff) for the amount of bytes a single query in a workgroup is       allowed to scan. No default is defined.
    /// 
    /// NoteThis property currently supports integer types. Support for long values is         planned.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BytesScannedCutoffPerQuery")]
    pub bytes_scanned_cutoff_per_query: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: CustomerContentEncryptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerContentEncryptionConfiguration")]
    pub customer_content_encryption_configuration: Option<CustomerContentEncryptionConfiguration>,


    /// 
    /// Indicates that the Amazon CloudWatch metrics are enabled for the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublishCloudWatchMetricsEnabled")]
    pub publish_cloud_watch_metrics_enabled: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalConfiguration")]
    pub additional_configuration: Option<String>,


    /// 
    /// Specifies the location in Amazon S3 where query results are stored and the encryption       option, if any, used for query results. For more information, see Working with Query         Results, Output Files, and Query History.
    /// 
    /// Required: No
    ///
    /// Type: ResultConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResultConfiguration")]
    pub result_configuration: Option<ResultConfiguration>,

}


/// The Athena engine version for running queries, or the PySpark engine       version for running sessions.
#[derive(Default, serde::Serialize)]
pub struct EngineVersion {


    /// 
    /// The engine version requested by the user. Possible values are determined by the output       of ListEngineVersions, including AUTO. The default is AUTO.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectedEngineVersion")]
    pub selected_engine_version: Option<String>,


    /// 
    /// Read only. The engine version on which the query runs. If the user requests a valid       engine version other than Auto, the effective engine version is the same as the engine       version that the user requested. If the user requests Auto, the effective engine version       is chosen by Athena. When a request to update the engine version is made by       a CreateWorkGroup or UpdateWorkGroup operation, the         EffectiveEngineVersion field is ignored.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "EffectiveEngineVersion")]
    pub effective_engine_version: Option<String>,

}


/// The location in Amazon S3 where query and calculation results are stored and the encryption       option, if any, used for query and calculation results. These are known as "client-side settings". If       workgroup settings override client-side settings, then the query uses the workgroup       settings.
#[derive(Default, serde::Serialize)]
pub struct ResultConfiguration {


    /// 
    /// The location in Amazon S3 where your query results are stored, such as         s3://path/to/query/bucket/. To run a query, you must specify the query       results location using either a client-side setting for individual queries or a location       specified by the workgroup. If workgroup settings override client-side settings, then       the query uses the location specified for the workgroup. If no query location is set,       Athena issues an error. For more information, see Working with Query Results, Output Files, and         Query History and EnforceWorkGroupConfiguration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<String>,


    /// 
    /// If query results are encrypted in Amazon S3, indicates the encryption option used (for       example, SSE_KMS or CSE_KMS) and key information. This is a       client-side setting. If workgroup settings override client-side settings, then the query       uses the encryption configuration that is specified for the workgroup, and also uses the       location for storing query results specified in the workgroup. See EnforceWorkGroupConfiguration and Workgroup Settings Override         Client-Side Settings.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AclConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AclConfiguration")]
    pub acl_configuration: Option<AclConfiguration>,

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


/// The CustomerContentEncryptionConfiguration property type specifies Property description not available. for an AWS::Athena::WorkGroup.
#[derive(Default, serde::Serialize)]
pub struct CustomerContentEncryptionConfiguration {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKey")]
    pub kms_key: String,

}