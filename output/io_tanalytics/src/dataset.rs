

/// The AWS::IoTAnalytics::Dataset resource stores data retrieved from a data store by applying a       queryAction (an SQL query) or a containerAction (executing a containerized application).       The data set can be populated manually by calling CreateDatasetContent or automatically according       to a trigger you specify. For more information, see             How to Use AWS IoT Analytics in the AWS IoT Analytics User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataset {


    /// 
    /// The DatasetAction objects that automatically create the dataset    contents.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Action
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,


    /// 
    /// When dataset contents are created they are delivered to destinations specified    here.
    /// 
    /// Required: No
    ///
    /// Type: List of DatasetContentDeliveryRule
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentDeliveryRules")]
    pub content_delivery_rules: Option<Vec<DatasetContentDeliveryRule>>,


    /// 
    /// Optional. How long, in days, message data is kept for the dataset.
    /// 
    /// Required: No
    ///
    /// Type: RetentionPeriod
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionPeriod")]
    pub retention_period: Option<RetentionPeriod>,


    /// 
    /// The name of the dataset.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (^(?!_{2}))(^[a-zA-Z0-9_]+$)
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetName")]
    pub dataset_name: Option<String>,


    /// 
    /// The DatasetTrigger objects that specify when the dataset is automatically    updated.
    /// 
    /// Required: No
    ///
    /// Type: List of Trigger
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Triggers")]
    pub triggers: Option<Vec<Trigger>>,


    /// 
    /// A list of data rules that send notifications to CloudWatch, when data arrives late. To specify lateDataRules, the dataset must use a DeltaTimer filter.
    /// 
    /// Required: No
    ///
    /// Type: List of LateDataRule
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "LateDataRules")]
    pub late_data_rules: Option<Vec<LateDataRule>>,


    /// 
    /// Optional. How many versions of dataset contents are kept. If not specified or set to null,    only the latest version plus the latest succeeded version (if they are different) are kept for    the time period specified by the retentionPeriod parameter. For more information,    see     Keeping Multiple Versions of AWS IoT Analytics datasets in the             AWS IoT Analytics User Guide.
    /// 
    /// Required: No
    ///
    /// Type: VersioningConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersioningConfiguration")]
    pub versioning_configuration: Option<VersioningConfiguration>,


    /// 
    /// Metadata which can be used to manage the data set.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnDataset {
    fn type_string() -> &'static str {
        "AWS::IoTAnalytics::Dataset"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// How long, in days, message data is kept.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RetentionPeriod {


    /// 
    /// The number of days that message data is kept. The unlimited parameter must be    false.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfDays")]
    pub number_of_days: Option<i64>,


    /// 
    /// If true, message data is kept indefinitely.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unlimited")]
    pub unlimited: Option<bool>,

}




/// Configuration information for delivery of dataset contents to AWS IoT Events.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IotEventsDestinationConfiguration {


    /// 
    /// The ARN of the role that grants AWS IoT Analytics permission to deliver dataset contents to an AWS IoT Events    input.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The name of the AWS IoT Events input to which dataset contents are delivered.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputName")]
    pub input_name: String,

}




/// Information about the versioning of dataset contents.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VersioningConfiguration {


    /// 
    /// If true, unlimited versions of dataset contents are kept.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unlimited")]
    pub unlimited: Option<bool>,


    /// 
    /// How many versions of dataset contents are kept. The unlimited parameter must    be false.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxVersions")]
    pub max_versions: Option<i64>,

}




/// Configuration information for delivery of dataset contents to Amazon Simple Storage Service (Amazon S3).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3DestinationConfiguration {


    /// 
    /// The key of the dataset contents object in an S3 bucket. Each object has a key that is a    unique identifier. Each object has exactly one key.
    /// 
    /// You can create a unique key with the following options:
    /// 
    /// Use !{iotanalytics:scheduleTime} to insert the time of a scheduled SQL      query run.               Use !{iotanalytics:versionId} to insert a unique hash that identifies a      dataset content.               Use !{iotanalytics:creationTime} to insert the creation time of a dataset      content.
    /// 
    /// The following example creates a unique key for a CSV file:     dataset/mydataset/!{iotanalytics:scheduleTime}/!{iotanalytics:versionId}.csv
    /// 
    /// NoteIf you don't use !{iotanalytics:versionId} to specify the key, you might     get duplicate keys. For example, you might have two dataset contents with the same      scheduleTime but different versionIds. This means that one     dataset content overwrites the other.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9!_.*'()/{}:-]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The ARN of the role that grants AWS IoT Analytics permission to interact with your Amazon S3 and AWS Glue    resources.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// Configuration information for coordination with AWS Glue, a fully managed extract, transform    and load (ETL) service.
    /// 
    /// Required: No
    ///
    /// Type: GlueConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueConfiguration")]
    pub glue_configuration: Option<GlueConfiguration>,


    /// 
    /// The name of the S3 bucket to which dataset contents are delivered.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9.\-_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,

}




/// The schedule for when to trigger an update.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Schedule {


    /// 
    /// The expression that defines when to trigger an update. For more information, see            Schedule Expressions for Rules in the Amazon CloudWatch documentation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,

}




/// The configuration of the resource used to execute the containerAction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceConfiguration {


    /// 
    /// The type of the compute resource used to execute the containerAction.    Possible values are: ACU_1 (vCPU=4, memory=16 GiB) or ACU_2 (vCPU=8,    memory=32 GiB).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ACU_1 | ACU_2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeType")]
    pub compute_type: ResourceConfigurationComputeTypeEnum,


    /// 
    /// The size, in GB, of the persistent storage available to the resource instance used to    execute the containerAction (min: 1, max: 50).
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: i64,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ResourceConfigurationComputeTypeEnum {

    /// ACU_1
    #[serde(rename = "ACU_1")]
    Acu1,

    /// ACU_2
    #[serde(rename = "ACU_2")]
    Acu2,

}

impl Default for ResourceConfigurationComputeTypeEnum {
    fn default() -> Self {
        ResourceConfigurationComputeTypeEnum::Acu1
    }
}



/// The "DatasetTrigger"   that specifies when the data set is automatically updated.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Trigger {


    /// 
    /// The "Schedule" when the trigger is initiated.
    /// 
    /// Required: No
    ///
    /// Type: Schedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Option<Schedule>,


    /// 
    /// Information about the data set whose content generation triggers the new data set content      generation.
    /// 
    /// Required: No
    ///
    /// Type: TriggeringDataset
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggeringDataset")]
    pub triggering_dataset: Option<TriggeringDataset>,

}




/// Used to limit data to that which has arrived since the last execution of the    action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeltaTime {


    /// 
    /// An expression by which the time of the message data might be determined. This can be the    name of a timestamp field or a SQL expression that is used to derive the time the message data    was generated.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeExpression")]
    pub time_expression: String,


    /// 
    /// The number of seconds of estimated in-flight lag time of message data. When you create    dataset contents using message data from a specified timeframe, some message data might still    be in flight when processing begins, and so do not arrive in time to be processed. Use this    field to make allowances for the in flight time of your message data, so that data not    processed from a previous timeframe is included with the next timeframe. Otherwise, missed    message data would be excluded from processing during the next timeframe too, because its    timestamp places it within the previous timeframe.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "OffsetSeconds")]
    pub offset_seconds: i64,

}




/// A structure that contains the name and configuration information of a late data    rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LateDataRule {


    /// 
    /// The name of the late data rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9_]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleName")]
    pub rule_name: Option<String>,


    /// 
    /// The information needed to configure the late data rule.
    /// 
    /// Required: Yes
    ///
    /// Type: LateDataRuleConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleConfiguration")]
    pub rule_configuration: LateDataRuleConfiguration,

}




/// The dataset whose latest contents are used as input to the notebook or application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatasetContentVersionValue {


    /// 
    /// The name of the dataset whose latest contents are used as input to the notebook or    application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (^(?!_{2}))(^[a-zA-Z0-9_]+$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,

}




/// Information needed to run the "containerAction" to produce data set contents.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {


    /// 
    /// Information which allows the system to run a containerized application in order to create      the data set contents. The application must be in a Docker container along with any needed      support libraries.
    /// 
    /// Required: No
    ///
    /// Type: ContainerAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerAction")]
    pub container_action: Option<ContainerAction>,


    /// 
    /// An "SqlQueryDatasetAction" object that uses an SQL query to automatically create data set contents.
    /// 
    /// Required: No
    ///
    /// Type: QueryAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryAction")]
    pub query_action: Option<QueryAction>,


    /// 
    /// The name of the data set action by which data set contents are automatically created.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9_]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionName")]
    pub action_name: String,

}




/// An "SqlQueryDatasetAction" object that uses an SQL query to automatically create data set contents.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct QueryAction {


    /// 
    /// Pre-filters applied to message data.
    /// 
    /// Required: No
    ///
    /// Type: List of Filter
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Option<Vec<Filter>>,


    /// 
    /// An "SqlQueryDatasetAction" object that uses an SQL query to automatically create data set contents.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlQuery")]
    pub sql_query: String,

}




/// The information needed to configure a delta time session window.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LateDataRuleConfiguration {


    /// 
    /// The information needed to configure a delta time session window.
    /// 
    /// Required: No
    ///
    /// Type: DeltaTimeSessionWindowConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaTimeSessionWindowConfiguration")]
    pub delta_time_session_window_configuration: Option<DeltaTimeSessionWindowConfiguration>,

}




/// Configuration information for coordination with AWS Glue, a fully managed extract, transform    and load (ETL) service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GlueConfiguration {


    /// 
    /// The name of the table in your AWS Glue Data Catalog that is used to perform the ETL    operations. An AWS Glue Data Catalog table contains partitioned data and descriptions of data    sources and targets.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 150
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,


    /// 
    /// The name of the database in your AWS Glue Data Catalog in which the table is located. An    AWS Glue Data Catalog database contains metadata tables.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 150
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: String,

}




/// When dataset contents are created, they are delivered to destination specified    here.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatasetContentDeliveryRule {


    /// 
    /// The destination to which dataset contents are delivered.
    /// 
    /// Required: Yes
    ///
    /// Type: DatasetContentDeliveryRuleDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: DatasetContentDeliveryRuleDestination,


    /// 
    /// The name of the dataset content delivery rules entry.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntryName")]
    pub entry_name: Option<String>,

}




/// An instance of a variable to be passed to the containerAction execution. Each    variable must have a name and a value given by one of stringValue,     datasetContentVersionValue, or outputFileUriValue.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Variable {


    /// 
    /// The value of the variable as a double (numeric).
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<f64>,


    /// 
    /// The value of the variable as a structure that specifies a dataset content version.
    /// 
    /// Required: No
    ///
    /// Type: DatasetContentVersionValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetContentVersionValue")]
    pub dataset_content_version_value: Option<DatasetContentVersionValue>,


    /// 
    /// The value of the variable as a string.
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
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,


    /// 
    /// The name of the variable.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariableName")]
    pub variable_name: String,


    /// 
    /// The value of the variable as a structure that specifies an output file URI.
    /// 
    /// Required: No
    ///
    /// Type: OutputFileUriValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputFileUriValue")]
    pub output_file_uri_value: Option<OutputFileUriValue>,

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




/// Information needed to run the "containerAction" to produce data set contents.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContainerAction {


    /// 
    /// The ARN of the role which gives permission to the system to access needed resources in order      to run the "containerAction". This includes, at minimum, permission to retrieve the data set      contents which are the input to the containerized application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,


    /// 
    /// The ARN of the Docker container stored in your account. The Docker container contains an      application and needed support libraries and is used to generate data set contents.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Image")]
    pub image: String,


    /// 
    /// Configuration of the resource which executes the "containerAction".
    /// 
    /// Required: Yes
    ///
    /// Type: ResourceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceConfiguration")]
    pub resource_configuration: ResourceConfiguration,


    /// 
    /// The values of variables used within the context of the execution of the containerized      application (basically, parameters passed to the application). Each variable must have a      name and a value given by one of "stringValue", "datasetContentVersionValue",      or "outputFileUriValue".
    /// 
    /// Required: No
    ///
    /// Type: List of Variable
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variables")]
    pub variables: Option<Vec<Variable>>,

}




/// A structure that contains the configuration information of a delta time session    window.
///
/// DeltaTime specifies a time interval. You can use     DeltaTime to create dataset contents with data that has arrived in the data    store since the last execution. For an example of DeltaTime, see Creating     a SQL dataset with a delta window (CLI) in the             AWS IoT Analytics User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeltaTimeSessionWindowConfiguration {


    /// 
    /// A time interval. You can use timeoutInMinutes so that AWS IoT Analytics can batch up late    data notifications that have been generated since the last execution. AWS IoT Analytics sends one batch of    notifications to Amazon CloudWatch Events at one time.
    /// 
    /// For more information about how to write a timestamp expression, see Date and Time Functions and     Operators, in the Presto 0.172 Documentation.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 60
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutInMinutes")]
    pub timeout_in_minutes: i64,

}




/// Information about the dataset whose content generation triggers the new dataset content    generation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TriggeringDataset {


    /// 
    /// The name of the data set whose content generation triggers the new data set content      generation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (^(?!_{2}))(^[a-zA-Z0-9_]+$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,

}




/// The value of the variable as a structure that specifies an output file URI.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputFileUriValue {


    /// 
    /// The URI of the location where dataset contents are stored, usually the URI of a file in an    S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [\w\.-]{1,255}
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileName")]
    pub file_name: String,

}




/// The destination to which dataset contents are delivered.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatasetContentDeliveryRuleDestination {


    /// 
    /// Configuration information for delivery of dataset contents to Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3DestinationConfiguration")]
    pub s3_destination_configuration: Option<S3DestinationConfiguration>,


    /// 
    /// Configuration information for delivery of dataset contents to AWS IoT Events.
    /// 
    /// Required: No
    ///
    /// Type: IotEventsDestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotEventsDestinationConfiguration")]
    pub iot_events_destination_configuration: Option<IotEventsDestinationConfiguration>,

}




/// Information which is used to filter message data, to segregate it according to the time      frame in which it arrives.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Filter {


    /// 
    /// Used to limit data to that which has arrived since the last execution of the action.
    /// 
    /// Required: No
    ///
    /// Type: DeltaTime
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaTime")]
    pub delta_time: Option<DeltaTime>,

}


