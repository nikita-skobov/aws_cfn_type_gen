
pub mod cfn_data_catalog {

#[derive(serde::Serialize, Default)]
pub struct CfnDataCatalog {
    /// A list of comma separated tags to add to the data catalog that is created.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// Specifies the Lambda function or functions to use for creating the data catalog. This is a mapping whose values depend on the catalog type.
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// A description of the data catalog to be created.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The name of the data catalog to create. The catalog name must be unique for the AWS account and can use a maximum of 128 alphanumeric, underscore, at sign, or hyphen characters.
    #[serde(rename = "Name")]
    pub name: String,
    /// The type of data catalog to create: LAMBDA for a federated catalog, GLUE for AWS Glue Catalog, or HIVE for an external hive metastore.
    #[serde(rename = "Type")]
    pub ty: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_named_query {

#[derive(serde::Serialize, Default)]
pub struct CfnNamedQuery {
    /// The contents of the query with all query statements.
    #[serde(rename = "QueryString")]
    pub query_string: String,
    /// The database to which the query belongs.
    #[serde(rename = "Database")]
    pub database: String,
    /// The name of the workgroup that contains the named query.
    #[serde(rename = "WorkGroup")]
    pub work_group: Option<String>,
    /// The query description.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The query name.
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



}

pub mod cfn_prepared_statement {

#[derive(serde::Serialize, Default)]
pub struct CfnPreparedStatement {
    /// The name of the prepared statement.
    #[serde(rename = "StatementName")]
    pub statement_name: String,
    /// The name of the workgroup to which the prepared statement belongs.
    #[serde(rename = "WorkGroup")]
    pub work_group: String,
    /// The description of the prepared statement.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The query string for the prepared statement.
    #[serde(rename = "QueryStatement")]
    pub query_statement: String,

}



}

pub mod cfn_work_group {

#[derive(serde::Serialize, Default)]
pub struct CfnWorkGroup {
    /// The workgroup configuration
    #[serde(rename = "WorkGroupConfiguration")]
    pub work_group_configuration: Option<WorkGroupConfiguration>,
    /// The workgroup configuration update object
    #[serde(rename = "WorkGroupConfigurationUpdates")]
    pub work_group_configuration_updates: Option<WorkGroupConfigurationUpdates>,
    /// One or more tags, separated by commas, that you want to attach to the workgroup as you create it
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The workGroup name.
    #[serde(rename = "Name")]
    pub name: String,
    /// The state of the workgroup: ENABLED or DISABLED.
    #[serde(rename = "State")]
    pub state: Option<String>,
    /// The option to delete the workgroup and its contents even if the workgroup contains any named queries.
    #[serde(rename = "RecursiveDeleteOption")]
    pub recursive_delete_option: Option<bool>,
    /// The workgroup description.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

pub type S3AclOption = String;pub type OutputLocation = String;
#[derive(serde::Serialize, Default)]
pub struct WorkGroupConfigurationUpdates {
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<EngineVersion>,
    #[serde(rename = "ResultConfigurationUpdates")]
    pub result_configuration_updates: Option<ResultConfigurationUpdates>,
    #[serde(rename = "AdditionalConfiguration")]
    pub additional_configuration: Option<AdditionalConfiguration>,
    #[serde(rename = "ExecutionRole")]
    pub execution_role: Option<ExecutionRole>,
    #[serde(rename = "RemoveCustomerContentEncryptionConfiguration")]
    pub remove_customer_content_encryption_configuration: Option<RemoveCustomerContentEncryptionConfiguration>,
    #[serde(rename = "RemoveBytesScannedCutoffPerQuery")]
    pub remove_bytes_scanned_cutoff_per_query: Option<RemoveBytesScannedCutoffPerQuery>,
    #[serde(rename = "CustomerContentEncryptionConfiguration")]
    pub customer_content_encryption_configuration: Option<CustomerContentEncryptionConfiguration>,
    #[serde(rename = "BytesScannedCutoffPerQuery")]
    pub bytes_scanned_cutoff_per_query: Option<BytesScannedCutoffPerQuery>,
    #[serde(rename = "RequesterPaysEnabled")]
    pub requester_pays_enabled: Option<RequesterPaysEnabled>,
    #[serde(rename = "EnforceWorkGroupConfiguration")]
    pub enforce_work_group_configuration: Option<EnforceWorkGroupConfiguration>,
    #[serde(rename = "PublishCloudWatchMetricsEnabled")]
    pub publish_cloud_watch_metrics_enabled: Option<PublishCloudWatchMetricsEnabled>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionConfiguration {
    #[serde(rename = "EncryptionOption")]
    pub encryption_option: EncryptionOption,
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<KmsKey>,

}
pub type AdditionalConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct CustomerContentEncryptionConfiguration {
    #[serde(rename = "KmsKey")]
    pub kms_key: KmsKey,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type RemoveBytesScannedCutoffPerQuery = bool;pub type ExpectedBucketOwner = String;pub type RemoveAclConfiguration = bool;
#[derive(serde::Serialize, Default)]
pub struct Tags {

}
pub type EffectiveEngineVersion = String;pub type BytesScannedCutoffPerQuery = usize;pub type KmsKey = String;pub type RemoveOutputLocation = bool;pub type EncryptionOption = String;pub type RemoveEncryptionConfiguration = bool;
#[derive(serde::Serialize, Default)]
pub struct AclConfiguration {
    #[serde(rename = "S3AclOption")]
    pub s3_acl_option: S3AclOption,

}

#[derive(serde::Serialize, Default)]
pub struct ResultConfiguration {
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<OutputLocation>,
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "AclConfiguration")]
    pub acl_configuration: Option<AclConfiguration>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<ExpectedBucketOwner>,

}
pub type RequesterPaysEnabled = bool;pub type PublishCloudWatchMetricsEnabled = bool;
#[derive(serde::Serialize, Default)]
pub struct WorkGroupConfiguration {
    #[serde(rename = "RequesterPaysEnabled")]
    pub requester_pays_enabled: Option<RequesterPaysEnabled>,
    #[serde(rename = "EnforceWorkGroupConfiguration")]
    pub enforce_work_group_configuration: Option<EnforceWorkGroupConfiguration>,
    #[serde(rename = "CustomerContentEncryptionConfiguration")]
    pub customer_content_encryption_configuration: Option<CustomerContentEncryptionConfiguration>,
    #[serde(rename = "ExecutionRole")]
    pub execution_role: Option<ExecutionRole>,
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<EngineVersion>,
    #[serde(rename = "PublishCloudWatchMetricsEnabled")]
    pub publish_cloud_watch_metrics_enabled: Option<PublishCloudWatchMetricsEnabled>,
    #[serde(rename = "BytesScannedCutoffPerQuery")]
    pub bytes_scanned_cutoff_per_query: Option<BytesScannedCutoffPerQuery>,
    #[serde(rename = "ResultConfiguration")]
    pub result_configuration: Option<ResultConfiguration>,
    #[serde(rename = "AdditionalConfiguration")]
    pub additional_configuration: Option<AdditionalConfiguration>,

}
pub type EnforceWorkGroupConfiguration = bool;pub type RemoveExpectedBucketOwner = bool;
#[derive(serde::Serialize, Default)]
pub struct ResultConfigurationUpdates {
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<ExpectedBucketOwner>,
    #[serde(rename = "RemoveOutputLocation")]
    pub remove_output_location: Option<RemoveOutputLocation>,
    #[serde(rename = "RemoveAclConfiguration")]
    pub remove_acl_configuration: Option<RemoveAclConfiguration>,
    #[serde(rename = "RemoveExpectedBucketOwner")]
    pub remove_expected_bucket_owner: Option<RemoveExpectedBucketOwner>,
    #[serde(rename = "RemoveEncryptionConfiguration")]
    pub remove_encryption_configuration: Option<RemoveEncryptionConfiguration>,
    #[serde(rename = "AclConfiguration")]
    pub acl_configuration: Option<AclConfiguration>,
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<OutputLocation>,

}
pub type SelectedEngineVersion = String;pub type RemoveCustomerContentEncryptionConfiguration = bool;
#[derive(serde::Serialize, Default)]
pub struct EngineVersion {
    #[serde(rename = "SelectedEngineVersion")]
    pub selected_engine_version: Option<SelectedEngineVersion>,
    #[serde(rename = "EffectiveEngineVersion")]
    pub effective_engine_version: Option<EffectiveEngineVersion>,

}
pub type ExecutionRole = String;

}
