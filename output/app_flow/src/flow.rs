

/// The AWS::AppFlow::Flow resource is an Amazon AppFlow resource type that    specifies a new flow.
#[derive(Default, serde::Serialize)]
pub struct CfnFlow {


    /// 
    /// The trigger settings that determine how and when Amazon AppFlow runs the specified    flow.
    /// 
    /// Required: Yes
    ///
    /// Type: TriggerConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerConfig")]
    pub trigger_config: TriggerConfig,


    /// 
    /// The tags used to organize, track, or control access for your flow.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Contains information about the configuration of the source connector used in the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: SourceFlowConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFlowConfig")]
    pub source_flow_config: SourceFlowConfig,


    /// 
    /// The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens    (-) only.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9][\w!@#.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "FlowName")]
    pub flow_name: String,


    /// 
    /// Indicates the current status of the flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Active | Deleted | Deprecated | Draft | Errored | Suspended
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowStatus")]
    pub flow_status: Option<String>,


    /// 
    /// The configuration that controls how Amazon AppFlow places data in the destination    connector.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DestinationFlowConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationFlowConfigList")]
    pub destination_flow_config_list: Vec<DestinationFlowConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: MetadataCatalogConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetadataCatalogConfig")]
    pub metadata_catalog_config: Option<MetadataCatalogConfig>,


    /// 
    /// A user-entered description of the flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\w!@#\-.?,\s]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A list of tasks that Amazon AppFlow performs while transferring the data in the flow    run.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Task
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tasks")]
    pub tasks: Vec<Task>,


    /// 
    /// The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for    encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS    key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws:kms:.*:[0-9]+:.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KMSArn")]
    pub kmsarn: Option<String>,

}


/// The properties that are applied when Amazon S3 is used as a destination.
#[derive(Default, serde::Serialize)]
pub struct S3DestinationProperties {


    /// 
    /// The object key for the destination bucket in which Amazon AppFlow places the files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The configuration that determines how Amazon AppFlow should format the flow output    data when Amazon S3 is used as the destination.
    /// 
    /// Required: No
    ///
    /// Type: S3OutputFormatConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3OutputFormatConfig")]
    pub s3_output_format_config: Option<S3OutputFormatConfig>,


    /// 
    /// The Amazon S3 bucket name in which Amazon AppFlow places the transferred    data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

}


/// The properties that are applied when Zendesk is used as a destination.
#[derive(Default, serde::Serialize)]
pub struct ZendeskDestinationProperties {


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the destination. For example, this setting would determine if the flow should fail after one    insertion error, or continue and attempt to insert every record regardless of the initial    failure. ErrorHandlingConfig is a part of the destination connector details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The possible write operations in the destination connector. When this value is not    provided, this defaults to the INSERT operation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<String>,


    /// 
    /// A list of field names that can be used as an ID field when performing a write operation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,


    /// 
    /// The object specified in the Zendesk flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when the custom connector is being used as a    source.
#[derive(Default, serde::Serialize)]
pub struct CustomConnectorSourceProperties {


    /// 
    /// The entity specified in the custom connector as a source in the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityName")]
    pub entity_name: String,


    /// 
    /// Custom properties that are required to use the custom connector as a source.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomProperties")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,

}


/// Specifies the information that is required to query a particular connector.
#[derive(Default, serde::Serialize)]
pub struct SourceConnectorProperties {


    /// 
    /// Specifies the information that is required for querying Veeva.
    /// 
    /// Required: No
    ///
    /// Type: VeevaSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Veeva")]
    pub veeva: Option<VeevaSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3SourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3SourceProperties>,


    /// 
    /// Specifies the information that is required for querying Marketo.
    /// 
    /// Required: No
    ///
    /// Type: MarketoSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Infor Nexus.
    /// 
    /// Required: No
    ///
    /// Type: InforNexusSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<InforNexusSourceProperties>,


    /// 
    /// Specifies the information that is required for querying ServiceNow.
    /// 
    /// Required: No
    ///
    /// Type: ServiceNowSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ServiceNowSourceProperties>,


    /// 
    /// The properties that are applied when the custom connector is being used as a    source.
    /// 
    /// Required: No
    ///
    /// Type: CustomConnectorSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Zendesk.
    /// 
    /// Required: No
    ///
    /// Type: ZendeskSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Dynatrace.
    /// 
    /// Required: No
    ///
    /// Type: DynatraceSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<DynatraceSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Singular.
    /// 
    /// Required: No
    ///
    /// Type: SingularSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Singular")]
    pub singular: Option<SingularSourceProperties>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: PardotSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pardot")]
    pub pardot: Option<PardotSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Google Analytics.
    /// 
    /// Required: No
    ///
    /// Type: GoogleAnalyticsSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "GoogleAnalytics")]
    pub google_analytics: Option<GoogleAnalyticsSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Slack.
    /// 
    /// Required: No
    ///
    /// Type: SlackSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slack")]
    pub slack: Option<SlackSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Salesforce.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Amplitude.
    /// 
    /// Required: No
    ///
    /// Type: AmplitudeSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Amplitude")]
    pub amplitude: Option<AmplitudeSourceProperties>,


    /// 
    /// The properties that are applied when using SAPOData as a flow source.
    /// 
    /// Required: No
    ///
    /// Type: SAPODataSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Datadog.
    /// 
    /// Required: No
    ///
    /// Type: DatadogSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datadog")]
    pub datadog: Option<DatadogSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Trend Micro.
    /// 
    /// Required: No
    ///
    /// Type: TrendmicroSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trendmicro")]
    pub trendmicro: Option<TrendmicroSourceProperties>,

}


/// The properties that are applied when ServiceNow is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct ServiceNowSourceProperties {


    /// 
    /// The object specified in the ServiceNow flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when Amazon Redshift is being used as a destination.
#[derive(Default, serde::Serialize)]
pub struct RedshiftDestinationProperties {


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the Amazon Redshift destination. For example, this setting would determine if the flow    should fail after one insertion error, or continue and attempt to insert every record    regardless of the initial failure. ErrorHandlingConfig is a part of the    destination connector details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The intermediate bucket that Amazon AppFlow uses when moving data into Amazon Redshift.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntermediateBucketName")]
    pub intermediate_bucket_name: String,


    /// 
    /// The object specified in the Amazon Redshift flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,


    /// 
    /// The object key for the bucket in which Amazon AppFlow places the destination files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,

}


/// Specifies elements that Amazon AppFlow includes in the file and folder names in the flow    destination.
#[derive(Default, serde::Serialize)]
pub struct PrefixConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathPrefixHierarchy")]
    pub path_prefix_hierarchy: Option<Vec<String>>,


    /// 
    /// Determines the format of the prefix, and whether it applies to the file name, file path,    or both.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FILENAME | PATH | PATH_AND_FILENAME
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixType")]
    pub prefix_type: Option<String>,


    /// 
    /// Determines the level of granularity for the date and time that's included in the prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MINUTE | MONTH | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixFormat")]
    pub prefix_format: Option<String>,

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


/// The properties that are applied when Upsolver is used as a destination.
#[derive(Default, serde::Serialize)]
pub struct UpsolverDestinationProperties {


    /// 
    /// The object key for the destination Upsolver Amazon S3 bucket in which Amazon AppFlow places the files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The configuration that determines how data is formatted when Upsolver is used as the flow    destination.
    /// 
    /// Required: Yes
    ///
    /// Type: UpsolverS3OutputFormatConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3OutputFormatConfig")]
    pub s3_output_format_config: UpsolverS3OutputFormatConfig,


    /// 
    /// The Upsolver Amazon S3 bucket name in which Amazon AppFlow places the    transferred data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 16
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^(upsolver-appflow)\S*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

}


/// The properties that Amazon AppFlow applies when you use Marketo as a flow    destination.
#[derive(Default, serde::Serialize)]
pub struct MarketoDestinationProperties {


    /// 
    /// The object specified in the Marketo flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the destination. For example, this setting would determine if the flow should fail after one    insertion error, or continue and attempt to insert every record regardless of the initial    failure. ErrorHandlingConfig is a part of the destination connector details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,

}


/// The properties that are applied when Singular is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct SingularSourceProperties {


    /// 
    /// The object specified in the Singular flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when Dynatrace is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct DynatraceSourceProperties {


    /// 
    /// The object specified in the Dynatrace flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// A map used to store task-related information. The execution service looks for particular    information based on the TaskType.
#[derive(Default, serde::Serialize)]
pub struct TaskPropertiesObject {


    /// 
    /// The task property key.
    /// 
    /// Allowed Values: VALUE | VALUES | DATA_TYPE | UPPER_BOUND |     LOWER_BOUND | SOURCE_DATA_TYPE | DESTINATION_DATA_TYPE | VALIDATION_ACTION | MASK_VALUE |     MASK_LENGTH | TRUNCATE_LENGTH | MATH_OPERATION_FIELDS_ORDER | CONCAT_FORMAT |     SUBFIELD_CATEGORY_MAP | EXCLUDE_SOURCE_FIELDS_LIST
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The task property value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


/// The properties that are applied when using Veeva as a flow source.
#[derive(Default, serde::Serialize)]
pub struct VeevaSourceProperties {


    /// 
    /// The object specified in the Veeva flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,


    /// 
    /// Boolean value to include file renditions in Veeva document extract flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeRenditions")]
    pub include_renditions: Option<bool>,


    /// 
    /// The document type specified in the Veeva document extract flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\s\w_-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentType")]
    pub document_type: Option<String>,


    /// 
    /// Boolean value to include All Versions of files in Veeva document extract flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeAllVersions")]
    pub include_all_versions: Option<bool>,


    /// 
    /// Boolean value to include source files in Veeva document extract flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeSourceFiles")]
    pub include_source_files: Option<bool>,

}


/// The settings that determine how Amazon AppFlow handles an error when placing data in    the destination. For example, this setting would determine if the flow should fail after one    insertion error, or continue and attempt to insert every record regardless of the initial    failure. ErrorHandlingConfig is a part of the destination connector details.
#[derive(Default, serde::Serialize)]
pub struct ErrorHandlingConfig {


    /// 
    /// Specifies the name of the Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,


    /// 
    /// Specifies the Amazon S3 bucket prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// Specifies if the flow should fail after the first instance of a failure when attempting    to place data in the destination.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailOnFirstError")]
    pub fail_on_first_error: Option<bool>,

}


/// The trigger settings that determine how and when Amazon AppFlow runs the specified    flow.
#[derive(Default, serde::Serialize)]
pub struct TriggerConfig {


    /// 
    /// Specifies the configuration details of a schedule-triggered flow as defined by the user.    Currently, these settings only apply to the Scheduled trigger type.
    /// 
    /// Required: No
    ///
    /// Type: ScheduledTriggerProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerProperties")]
    pub trigger_properties: Option<ScheduledTriggerProperties>,


    /// 
    /// Specifies the type of flow trigger. This can be OnDemand,     Scheduled, or Event.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Event | OnDemand | Scheduled
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerType")]
    pub trigger_type: String,

}


/// A class for modeling different type of tasks. Task implementation varies based on the     TaskType.
#[derive(Default, serde::Serialize)]
pub struct Task {


    /// 
    /// Specifies the particular task implementation that Amazon AppFlow performs.
    /// 
    /// Allowed values: Arithmetic | Filter |     Map | Map_all | Mask | Merge |     Truncate | Validate
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskType")]
    pub task_type: String,


    /// 
    /// A field in a destination connector, or a field value against which Amazon AppFlow    validates a source field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationField")]
    pub destination_field: Option<String>,


    /// 
    /// A map used to store task-related information. The execution service looks for particular    information based on the TaskType.
    /// 
    /// Required: No
    ///
    /// Type: List of TaskPropertiesObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskProperties")]
    pub task_properties: Option<Vec<TaskPropertiesObject>>,


    /// 
    /// The source fields to which a particular task is applied.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFields")]
    pub source_fields: Vec<String>,


    /// 
    /// The operation to be performed on the provided source fields.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorOperator
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOperator")]
    pub connector_operator: Option<ConnectorOperator>,

}


/// The properties that are applied when using SAPOData as a flow source.
#[derive(Default, serde::Serialize)]
pub struct SAPODataSourceProperties {


    /// 
    /// The object path specified in the SAPOData flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectPath")]
    pub object_path: String,

}


/// The properties that are applied when using SAPOData as a flow destination
#[derive(Default, serde::Serialize)]
pub struct SAPODataDestinationProperties {


    /// 
    /// Determines how Amazon AppFlow handles the success response that it gets from the    connector after placing data.
    /// 
    /// For example, this setting would determine where to write the response from a destination    connector upon a successful insert operation.
    /// 
    /// Required: No
    ///
    /// Type: SuccessResponseHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuccessResponseHandlingConfig")]
    pub success_response_handling_config: Option<SuccessResponseHandlingConfig>,


    /// 
    /// The possible write operations in the destination connector. When this value is not    provided, this defaults to the INSERT operation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<String>,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the destination. For example, this setting would determine if the flow should fail after one    insertion error, or continue and attempt to insert every record regardless of the initial    failure. ErrorHandlingConfig is a part of the destination connector details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The object path specified in the SAPOData flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectPath")]
    pub object_path: String,


    /// 
    /// A list of field names that can be used as an ID field when performing a write operation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,

}


/// The properties that are applied when Infor Nexus is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct InforNexusSourceProperties {


    /// 
    /// The object specified in the Infor Nexus flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when using Zendesk as a flow source.
#[derive(Default, serde::Serialize)]
pub struct ZendeskSourceProperties {


    /// 
    /// The object specified in the Zendesk flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The configuration that determines how Amazon AppFlow formats the flow output data    when Upsolver is used as the destination.
#[derive(Default, serde::Serialize)]
pub struct UpsolverS3OutputFormatConfig {


    /// 
    /// Specifies elements that Amazon AppFlow includes in the file and folder names in the flow    destination.
    /// 
    /// Required: Yes
    ///
    /// Type: PrefixConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixConfig")]
    pub prefix_config: PrefixConfig,


    /// 
    /// The aggregation settings that you can use to customize the output format of your flow    data.
    /// 
    /// Required: No
    ///
    /// Type: AggregationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationConfig")]
    pub aggregation_config: Option<AggregationConfig>,


    /// 
    /// Indicates the file type that Amazon AppFlow places in the Upsolver Amazon S3    bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON | PARQUET
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileType")]
    pub file_type: Option<String>,

}


/// The GlueDataCatalog property type specifies Property description not available. for an AWS::AppFlow::Flow.
#[derive(Default, serde::Serialize)]
pub struct GlueDataCatalog {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TablePrefix")]
    pub table_prefix: String,

}


/// The aggregation settings that you can use to customize the output format of your flow    data.
#[derive(Default, serde::Serialize)]
pub struct AggregationConfig {


    /// 
    /// Specifies whether Amazon AppFlow aggregates the flow records into a single file, or    leave them unaggregated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: None | SingleFile
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationType")]
    pub aggregation_type: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetFileSize")]
    pub target_file_size: Option<i64>,

}


/// The properties that are applied when Datadog is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct DatadogSourceProperties {


    /// 
    /// The object specified in the Datadog flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when Salesforce is being used as a destination.
#[derive(Default, serde::Serialize)]
pub struct SalesforceDestinationProperties {


    /// 
    /// This specifies the type of write operation to be performed in Salesforce. When the value    is UPSERT, then idFieldNames is required.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DELETE | INSERT | UPDATE | UPSERT
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<String>,


    /// 
    /// Specifies which Salesforce API is used by Amazon AppFlow when your flow transfers    data to Salesforce.
    /// 
    /// AUTOMATIC                  The default. Amazon AppFlow selects which API to use based on the number of       records that your flow transfers to Salesforce. If your flow transfers fewer than 1,000       records, Amazon AppFlow uses Salesforce REST API. If your flow transfers 1,000       records or more, Amazon AppFlow uses Salesforce Bulk API 2.0.          Each of these Salesforce APIs structures data differently. If Amazon AppFlow       selects the API automatically, be aware that, for recurring flows, the data output might       vary from one flow run to the next. For example, if a flow runs daily, it might use REST       API on one day to transfer 900 records, and it might use Bulk API 2.0 on the next day to       transfer 1,100 records. For each of these flow runs, the respective Salesforce API       formats the data differently. Some of the differences include how dates are formatted       and null values are represented. Also, Bulk API 2.0 doesn't transfer Salesforce compound       fields.          By choosing this option, you optimize flow performance for both small and large data       transfers, but the tradeoff is inconsistent formatting in the output.                       BULKV2                  Amazon AppFlow uses only Salesforce Bulk API 2.0. This API runs asynchronous       data transfers, and it's optimal for large sets of data. By choosing this option, you       ensure that your flow writes consistent output, but you optimize performance only for       large data transfers.          Note that Bulk API 2.0 does not transfer Salesforce compound fields.                       REST_SYNC                  Amazon AppFlow uses only Salesforce REST API. By choosing this option, you       ensure that your flow writes consistent output, but you decrease performance for large       data transfers that are better suited for Bulk API 2.0. In some cases, if your flow       attempts to transfer a vary large set of data, it might fail with a timed out       error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | BULKV2 | REST_SYNC
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTransferApi")]
    pub data_transfer_api: Option<String>,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the Salesforce destination. For example, this setting would determine if the flow should fail    after one insertion error, or continue and attempt to insert every record regardless of the    initial failure. ErrorHandlingConfig is a part of the destination connector    details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The name of the field that Amazon AppFlow uses as an ID when performing a write    operation such as update or delete.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,


    /// 
    /// The object specified in the Salesforce flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when Slack is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct SlackSourceProperties {


    /// 
    /// The object specified in the Slack flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when Marketo is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct MarketoSourceProperties {


    /// 
    /// The object specified in the Marketo flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The operation to be performed on the provided source fields.
#[derive(Default, serde::Serialize)]
pub struct ConnectorOperator {


    /// 
    /// Operators supported by the custom connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<String>,


    /// 
    /// The operation to be performed on the provided Slack source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slack")]
    pub slack: Option<String>,


    /// 
    /// The operation to be performed on the provided Trend Micro source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trendmicro")]
    pub trendmicro: Option<String>,


    /// 
    /// The operation to be performed on the provided Infor Nexus source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pardot")]
    pub pardot: Option<String>,


    /// 
    /// The operation to be performed on the provided Datadog source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datadog")]
    pub datadog: Option<String>,


    /// 
    /// The operation to be performed on the provided ServiceNow source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<String>,


    /// 
    /// The operation to be performed on the provided Veeva source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Veeva")]
    pub veeva: Option<String>,


    /// 
    /// The operation to be performed on the provided Zendesk source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | DIVISION | GREATER_THAN | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<String>,


    /// 
    /// The operation to be performed on the provided SAPOData source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<String>,


    /// 
    /// The operation to be performed on the provided Salesforce source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<String>,


    /// 
    /// The operation to be performed on the provided Dynatrace source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<String>,


    /// 
    /// The operation to be performed on the provided Amazon S3 source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<String>,


    /// 
    /// The operation to be performed on the provided Amplitude source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BETWEEN
    ///
    /// Update requires: No interruption
    #[serde(rename = "Amplitude")]
    pub amplitude: Option<String>,


    /// 
    /// The operation to be performed on the provided Marketo source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | GREATER_THAN | LESS_THAN | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    pub marketo: Option<String>,


    /// 
    /// The operation to be performed on the provided Google Analytics source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BETWEEN | PROJECTION
    ///
    /// Update requires: No interruption
    #[serde(rename = "GoogleAnalytics")]
    pub google_analytics: Option<String>,


    /// 
    /// The operation to be performed on the provided Singular source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Singular")]
    pub singular: Option<String>,

}


/// The properties that are applied when Amazon EventBridge is being used as a    destination.
#[derive(Default, serde::Serialize)]
pub struct EventBridgeDestinationProperties {


    /// 
    /// The object specified in the Amplitude flow source.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The object specified in the Amazon EventBridge flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when Amplitude is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct AmplitudeSourceProperties {


    /// 
    /// The object specified in the Amplitude flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// Contains information about the configuration of the source connector used in the flow.
#[derive(Default, serde::Serialize)]
pub struct SourceFlowConfig {


    /// 
    /// The name of the connector profile. This name must be unique for each connector profile in    the AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\w/!@#+=.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProfileName")]
    pub connector_profile_name: Option<String>,


    /// 
    /// Defines the configuration for a scheduled incremental data pull. If a valid configuration    is provided, the fields specified in the configuration are used when querying for the    incremental data pull.
    /// 
    /// Required: No
    ///
    /// Type: IncrementalPullConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncrementalPullConfig")]
    pub incremental_pull_config: Option<IncrementalPullConfig>,


    /// 
    /// The API version of the connector when it's used as a source in the flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<String>,


    /// 
    /// The type of connector, such as Salesforce, Amplitude, and so on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Amplitude | CustomConnector | CustomerProfiles | Datadog | Dynatrace | EventBridge | Googleanalytics | Honeycode | Infornexus | LookoutMetrics | Marketo | Pardot | Redshift | S3 | Salesforce | SAPOData | Servicenow | Singular | Slack | Snowflake | Trendmicro | Upsolver | Veeva | Zendesk
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorType")]
    pub connector_type: String,


    /// 
    /// Specifies the information that is required to query a particular source connector.
    /// 
    /// Required: Yes
    ///
    /// Type: SourceConnectorProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceConnectorProperties")]
    pub source_connector_properties: SourceConnectorProperties,

}


/// The properties that are applied when Salesforce is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct SalesforceSourceProperties {


    /// 
    /// Specifies which Salesforce API is used by Amazon AppFlow when your flow transfers    data from Salesforce.
    /// 
    /// AUTOMATIC                  The default. Amazon AppFlow selects which API to use based on the number of       records that your flow transfers from Salesforce. If your flow transfers fewer than       1,000,000 records, Amazon AppFlow uses Salesforce REST API. If your flow transfers       1,000,000 records or more, Amazon AppFlow uses Salesforce Bulk API 2.0.          Each of these Salesforce APIs structures data differently. If Amazon AppFlow       selects the API automatically, be aware that, for recurring flows, the data output might       vary from one flow run to the next. For example, if a flow runs daily, it might use REST       API on one day to transfer 900,000 records, and it might use Bulk API 2.0 on the next       day to transfer 1,100,000 records. For each of these flow runs, the respective       Salesforce API formats the data differently. Some of the differences include how dates       are formatted and null values are represented. Also, Bulk API 2.0 doesn't transfer       Salesforce compound fields.          By choosing this option, you optimize flow performance for both small and large data       transfers, but the tradeoff is inconsistent formatting in the output.                       BULKV2                  Amazon AppFlow uses only Salesforce Bulk API 2.0. This API runs asynchronous       data transfers, and it's optimal for large sets of data. By choosing this option, you       ensure that your flow writes consistent output, but you optimize performance only for       large data transfers.          Note that Bulk API 2.0 does not transfer Salesforce compound fields.                       REST_SYNC                  Amazon AppFlow uses only Salesforce REST API. By choosing this option, you       ensure that your flow writes consistent output, but you decrease performance for large       data transfers that are better suited for Bulk API 2.0. In some cases, if your flow       attempts to transfer a vary large set of data, it might fail wituh a timed out       error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | BULKV2 | REST_SYNC
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTransferApi")]
    pub data_transfer_api: Option<String>,


    /// 
    /// Indicates whether Amazon AppFlow includes deleted files in the flow run.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeDeletedRecords")]
    pub include_deleted_records: Option<bool>,


    /// 
    /// The flag that enables dynamic fetching of new (recently added) fields in the Salesforce    objects while running a flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDynamicFieldUpdate")]
    pub enable_dynamic_field_update: Option<bool>,


    /// 
    /// The object specified in the Salesforce flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// This stores the information that is required to query a particular connector.
#[derive(Default, serde::Serialize)]
pub struct DestinationConnectorProperties {


    /// 
    /// The properties required to query Amazon Redshift.
    /// 
    /// Required: No
    ///
    /// Type: RedshiftDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Redshift")]
    pub redshift: Option<RedshiftDestinationProperties>,


    /// 
    /// The properties required to query Salesforce.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceDestinationProperties>,


    /// 
    /// The properties required to query Zendesk.
    /// 
    /// Required: No
    ///
    /// Type: ZendeskDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskDestinationProperties>,


    /// 
    /// The properties that are required to query the custom Connector.
    /// 
    /// Required: No
    ///
    /// Type: CustomConnectorDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorDestinationProperties>,


    /// 
    /// The properties required to query SAPOData.
    /// 
    /// Required: No
    ///
    /// Type: SAPODataDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataDestinationProperties>,


    /// 
    /// The properties required to query Amazon EventBridge.
    /// 
    /// Required: No
    ///
    /// Type: EventBridgeDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBridge")]
    pub event_bridge: Option<EventBridgeDestinationProperties>,


    /// 
    /// The properties required to query Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3DestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3DestinationProperties>,


    /// 
    /// The properties required to query Upsolver.
    /// 
    /// Required: No
    ///
    /// Type: UpsolverDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Upsolver")]
    pub upsolver: Option<UpsolverDestinationProperties>,


    /// 
    /// The properties required to query Snowflake.
    /// 
    /// Required: No
    ///
    /// Type: SnowflakeDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Snowflake")]
    pub snowflake: Option<SnowflakeDestinationProperties>,


    /// 
    /// The properties required to query Marketo.
    /// 
    /// Required: No
    ///
    /// Type: MarketoDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoDestinationProperties>,


    /// 
    /// The properties required to query Amazon Lookout for Metrics.
    /// 
    /// Required: No
    ///
    /// Type: LookoutMetricsDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "LookoutMetrics")]
    pub lookout_metrics: Option<LookoutMetricsDestinationProperties>,

}


/// The configuration that determines how Amazon AppFlow should format the flow output    data when Amazon S3 is used as the destination.
#[derive(Default, serde::Serialize)]
pub struct S3OutputFormatConfig {


    /// 
    /// Determines the prefix that Amazon AppFlow applies to the folder name in the Amazon S3 bucket. You can name folders according to the flow frequency and date.
    /// 
    /// Required: No
    ///
    /// Type: PrefixConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixConfig")]
    pub prefix_config: Option<PrefixConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreserveSourceDataTyping")]
    pub preserve_source_data_typing: Option<bool>,


    /// 
    /// Indicates the file type that Amazon AppFlow places in the Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON | PARQUET
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileType")]
    pub file_type: Option<String>,


    /// 
    /// The aggregation settings that you can use to customize the output format of your flow    data.
    /// 
    /// Required: No
    ///
    /// Type: AggregationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationConfig")]
    pub aggregation_config: Option<AggregationConfig>,

}


/// The properties that are applied when using Trend Micro as a flow source.
#[derive(Default, serde::Serialize)]
pub struct TrendmicroSourceProperties {


    /// 
    /// The object specified in the Trend Micro flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when the custom connector is being used as a    destination.
#[derive(Default, serde::Serialize)]
pub struct CustomConnectorDestinationProperties {


    /// 
    /// The entity specified in the custom connector as a destination in the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityName")]
    pub entity_name: String,


    /// 
    /// The name of the field that Amazon AppFlow uses as an ID when performing a write    operation such as update, delete, or upsert.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,


    /// 
    /// Specifies the type of write operation to be performed in the custom connector when it's    used as destination.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DELETE | INSERT | UPDATE | UPSERT
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<String>,


    /// 
    /// The custom properties that are specific to the connector when it's used as a destination    in the flow.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomProperties")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the custom connector as destination.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,

}


/// The properties that are applied when Google Analytics is being used as a source.
#[derive(Default, serde::Serialize)]
pub struct GoogleAnalyticsSourceProperties {


    /// 
    /// The object specified in the Google Analytics flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// The properties that are applied when Snowflake is being used as a destination.
#[derive(Default, serde::Serialize)]
pub struct SnowflakeDestinationProperties {


    /// 
    /// The object key for the destination bucket in which Amazon AppFlow places the files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The object specified in the Snowflake flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,


    /// 
    /// The intermediate bucket that Amazon AppFlow uses when moving data into Snowflake.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntermediateBucketName")]
    pub intermediate_bucket_name: String,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the Snowflake destination. For example, this setting would determine if the flow should fail    after one insertion error, or continue and attempt to insert every record regardless of the    initial failure. ErrorHandlingConfig is a part of the destination connector    details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,

}


/// Specifies the configuration details of a schedule-triggered flow as defined by the user.    Currently, these settings only apply to the Scheduled trigger type.
#[derive(Default, serde::Serialize)]
pub struct ScheduledTriggerProperties {


    /// 
    /// The scheduling expression that determines the rate at which the schedule will run, for    example rate(5minutes).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,


    /// 
    /// The time at which the scheduled flow ends. The time is formatted as a timestamp that    follows the ISO 8601 standard, such as 2022-04-27T13:00:00-07:00.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleEndTime")]
    pub schedule_end_time: Option<f64>,


    /// 
    /// Specifies the time zone used when referring to the dates and times of a scheduled flow,    such as America/New_York. This time zone is only a descriptive label. It doesn't    affect how Amazon AppFlow interprets the timestamps that you specify to schedule the    flow.
    /// 
    /// If you want to schedule a flow by using times in a particular time zone, indicate the time    zone as a UTC offset in your timestamps. For example, the UTC offsets for the     America/New_York timezone are -04:00 EDT and -05:00     EST.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,


    /// 
    /// Specifies whether a scheduled flow has an incremental data transfer or a complete data    transfer for each flow run.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Complete | Incremental
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPullMode")]
    pub data_pull_mode: Option<String>,


    /// 
    /// Specifies the date range for the records to import from the connector in the first flow    run.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirstExecutionFrom")]
    pub first_execution_from: Option<f64>,


    /// 
    /// Specifies the optional offset that is added to the time interval for a schedule-triggered    flow.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleOffset")]
    pub schedule_offset: Option<f64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowErrorDeactivationThreshold")]
    pub flow_error_deactivation_threshold: Option<i64>,


    /// 
    /// The time at which the scheduled flow starts. The time is formatted as a timestamp that    follows the ISO 8601 standard, such as 2022-04-26T13:00:00-07:00.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleStartTime")]
    pub schedule_start_time: Option<f64>,

}


/// Determines how Amazon AppFlow handles the success response that it gets from the    connector after placing data.
///
/// For example, this setting would determine where to write the response from the destination    connector upon a successful insert operation.
#[derive(Default, serde::Serialize)]
pub struct SuccessResponseHandlingConfig {


    /// 
    /// The Amazon S3 bucket prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The name of the Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,

}


/// Specifies the configuration used when importing incremental records from the source.
#[derive(Default, serde::Serialize)]
pub struct IncrementalPullConfig {


    /// 
    /// A field that specifies the date time or timestamp field as the criteria to use when    importing incremental records from the source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatetimeTypeFieldName")]
    pub datetime_type_field_name: Option<String>,

}


/// Contains information about the configuration of destination connectors present in the    flow.
#[derive(Default, serde::Serialize)]
pub struct DestinationFlowConfig {


    /// 
    /// This stores the information that is required to query a particular connector.
    /// 
    /// Required: Yes
    ///
    /// Type: DestinationConnectorProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationConnectorProperties")]
    pub destination_connector_properties: DestinationConnectorProperties,


    /// 
    /// The type of destination connector, such as Sales force, Amazon S3, and so on.
    /// 
    /// Allowed Values: EventBridge | Redshift | S3 | Salesforce |     Snowflake
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorType")]
    pub connector_type: String,


    /// 
    /// The API version that the destination connector uses.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<String>,


    /// 
    /// The name of the connector profile. This name must be unique for each connector profile in    the AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\w/!@#+=.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProfileName")]
    pub connector_profile_name: Option<String>,

}


/// The properties that are applied when Amazon S3 is being used as the flow source.
#[derive(Default, serde::Serialize)]
pub struct S3SourceProperties {


    /// 
    /// The Amazon S3 bucket name where the source files are stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// 
    /// The object key for the Amazon S3 bucket in which the source files are stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: String,


    /// 
    /// When you use Amazon S3 as the source, the configuration format that you provide    the flow input data.
    /// 
    /// Required: No
    ///
    /// Type: S3InputFormatConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3InputFormatConfig")]
    pub s3_input_format_config: Option<S3InputFormatConfig>,

}


/// The properties that are applied when Amazon Lookout for Metrics is used as a destination.
#[derive(Default, serde::Serialize)]
pub struct LookoutMetricsDestinationProperties {


    /// 
    /// The object specified in the Amazon Lookout for Metrics flow destination.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: Option<String>,

}


/// The MetadataCatalogConfig property type specifies Property description not available. for an AWS::AppFlow::Flow.
#[derive(Default, serde::Serialize)]
pub struct MetadataCatalogConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: GlueDataCatalog
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueDataCatalog")]
    pub glue_data_catalog: Option<GlueDataCatalog>,

}


/// The PardotSourceProperties property type specifies Property description not available. for an AWS::AppFlow::Flow.
#[derive(Default, serde::Serialize)]
pub struct PardotSourceProperties {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


/// When you use Amazon S3 as the source, the configuration format that you provide    the flow input data.
#[derive(Default, serde::Serialize)]
pub struct S3InputFormatConfig {


    /// 
    /// The file type that Amazon AppFlow gets from your Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3InputFileType")]
    pub s3_input_file_type: Option<String>,

}
