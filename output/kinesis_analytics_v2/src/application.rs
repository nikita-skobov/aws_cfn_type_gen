

/// Creates an Amazon Kinesis Data Analytics application. For information about creating a       Kinesis Data Analytics application, see Creating an         Application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplication {


    /// 
    /// Use this parameter to configure the application.
    /// 
    /// Required: No
    ///
    /// Type: ApplicationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationConfiguration")]
    pub application_configuration: Option<ApplicationConfiguration>,


    /// 
    /// The description of the application.
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
    #[serde(rename = "ApplicationDescription")]
    pub application_description: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ApplicationMaintenanceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationMaintenanceConfiguration")]
    pub application_maintenance_configuration: Option<ApplicationMaintenanceConfiguration>,


    /// 
    /// To create a Kinesis Data Analytics Studio notebook, you must set the mode to         INTERACTIVE. However, for a Kinesis Data Analytics for Apache Flink       application, the mode is optional.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: INTERACTIVE | STREAMING
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationMode")]
    pub application_mode: Option<ApplicationApplicationModeEnum>,


    /// 
    /// The name of the application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: RunConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunConfiguration")]
    pub run_configuration: Option<RunConfiguration>,


    /// 
    /// The runtime environment for the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FLINK-1_11 | FLINK-1_13 | FLINK-1_15 | FLINK-1_6 | FLINK-1_8 | SQL-1_0 | ZEPPELIN-FLINK-1_0 | ZEPPELIN-FLINK-2_0
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuntimeEnvironment")]
    pub runtime_environment: ApplicationRuntimeEnvironmentEnum,


    /// 
    /// Specifies the IAM role that the application uses to access external resources.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceExecutionRole")]
    pub service_execution_role: String,


    /// 
    /// A list of one or more tags to assign to the application. A tag is a key-value pair       that identifies an application. Note that the maximum number of application tags       includes system tags. The maximum number of user-defined application tags is 50.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ApplicationApplicationModeEnum {

    /// INTERACTIVE
    #[serde(rename = "INTERACTIVE")]
    Interactive,

    /// STREAMING
    #[serde(rename = "STREAMING")]
    Streaming,

}

impl Default for ApplicationApplicationModeEnum {
    fn default() -> Self {
        ApplicationApplicationModeEnum::Interactive
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ApplicationRuntimeEnvironmentEnum {

    /// FLINK-1_11
    #[serde(rename = "FLINK-1_11")]
    Flink111,

    /// FLINK-1_13
    #[serde(rename = "FLINK-1_13")]
    Flink113,

    /// FLINK-1_15
    #[serde(rename = "FLINK-1_15")]
    Flink115,

    /// FLINK-1_6
    #[serde(rename = "FLINK-1_6")]
    Flink16,

    /// FLINK-1_8
    #[serde(rename = "FLINK-1_8")]
    Flink18,

    /// SQL-1_0
    #[serde(rename = "SQL-1_0")]
    Sql10,

    /// ZEPPELIN-FLINK-1_0
    #[serde(rename = "ZEPPELIN-FLINK-1_0")]
    Zeppelinflink10,

    /// ZEPPELIN-FLINK-2_0
    #[serde(rename = "ZEPPELIN-FLINK-2_0")]
    Zeppelinflink20,

}

impl Default for ApplicationRuntimeEnvironmentEnum {
    fn default() -> Self {
        ApplicationRuntimeEnvironmentEnum::Flink111
    }
}


impl cfn_resources::CfnResource for CfnApplication {
    fn type_string() -> &'static str {
        "AWS::KinesisAnalyticsV2::Application"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes code configuration for an application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplicationCodeConfiguration {


    /// 
    /// The location and type of the application code.
    /// 
    /// Required: Yes
    ///
    /// Type: CodeContent
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeContent")]
    pub code_content: CodeContent,


    /// 
    /// Specifies whether the code content is in text or zip format.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: PLAINTEXT | ZIPFILE
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeContentType")]
    pub code_content_type: ApplicationCodeConfigurationCodeContentTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ApplicationCodeConfigurationCodeContentTypeEnum {

    /// PLAINTEXT
    #[serde(rename = "PLAINTEXT")]
    Plaintext,

    /// ZIPFILE
    #[serde(rename = "ZIPFILE")]
    Zipfile,

}

impl Default for ApplicationCodeConfigurationCodeContentTypeEnum {
    fn default() -> Self {
        ApplicationCodeConfigurationCodeContentTypeEnum::Plaintext
    }
}



/// Specifies the creation parameters for a Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplicationConfiguration {


    /// 
    /// The code location and type parameters for a Flink-based Kinesis Data Analytics application.
    /// 
    /// Required: Conditional
    ///
    /// Type: ApplicationCodeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationCodeConfiguration")]
    pub application_code_configuration: Option<ApplicationCodeConfiguration>,


    /// 
    /// Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.
    /// 
    /// Required: No
    ///
    /// Type: ApplicationSnapshotConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationSnapshotConfiguration")]
    pub application_snapshot_configuration: Option<ApplicationSnapshotConfiguration>,


    /// 
    /// Describes execution properties for a Flink-based Kinesis Data Analytics application.
    /// 
    /// Required: No
    ///
    /// Type: EnvironmentProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentProperties")]
    pub environment_properties: Option<EnvironmentProperties>,


    /// 
    /// The creation and update parameters for a Flink-based Kinesis Data Analytics application.
    /// 
    /// Required: No
    ///
    /// Type: FlinkApplicationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlinkApplicationConfiguration")]
    pub flink_application_configuration: Option<FlinkApplicationConfiguration>,


    /// 
    /// The creation and update parameters for a SQL-based Kinesis Data Analytics application.
    /// 
    /// Required: No
    ///
    /// Type: SqlApplicationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlApplicationConfiguration")]
    pub sql_application_configuration: Option<SqlApplicationConfiguration>,


    /// 
    /// The array of descriptions of VPC configurations available to the application.
    /// 
    /// Required: No
    ///
    /// Type: List of VpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfigurations")]
    pub vpc_configurations: Option<Vec<VpcConfiguration>>,


    /// 
    /// The configuration parameters for a Kinesis Data Analytics Studio notebook.
    /// 
    /// Required: No
    ///
    /// Type: ZeppelinApplicationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZeppelinApplicationConfiguration")]
    pub zeppelin_application_configuration: Option<ZeppelinApplicationConfiguration>,

}




/// Specifies the maintence window parameters for a Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplicationMaintenanceConfiguration {


    /// Specifies the start time of the maintence window.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationMaintenanceWindowStartTime")]
    pub application_maintenance_window_start_time: String,

}




/// Specifies the method and snapshot to use when restarting an application using previously saved application state.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplicationRestoreConfiguration {


    /// 
    /// Specifies how the application should be restored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: RESTORE_FROM_CUSTOM_SNAPSHOT | RESTORE_FROM_LATEST_SNAPSHOT | SKIP_RESTORE_FROM_SNAPSHOT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationRestoreType")]
    pub application_restore_type: ApplicationRestoreConfigurationApplicationRestoreTypeEnum,


    /// 
    /// The identifier of an existing snapshot of application state to use to restart an application.    The application uses this value if RESTORE_FROM_CUSTOM_SNAPSHOT is specified for the    ApplicationRestoreType.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotName")]
    pub snapshot_name: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ApplicationRestoreConfigurationApplicationRestoreTypeEnum {

    /// RESTORE_FROM_CUSTOM_SNAPSHOT
    #[serde(rename = "RESTORE_FROM_CUSTOM_SNAPSHOT")]
    Restorefromcustomsnapshot,

    /// RESTORE_FROM_LATEST_SNAPSHOT
    #[serde(rename = "RESTORE_FROM_LATEST_SNAPSHOT")]
    Restorefromlatestsnapshot,

    /// SKIP_RESTORE_FROM_SNAPSHOT
    #[serde(rename = "SKIP_RESTORE_FROM_SNAPSHOT")]
    Skiprestorefromsnapshot,

}

impl Default for ApplicationRestoreConfigurationApplicationRestoreTypeEnum {
    fn default() -> Self {
        ApplicationRestoreConfigurationApplicationRestoreTypeEnum::Restorefromcustomsnapshot
    }
}



/// Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplicationSnapshotConfiguration {


    /// 
    /// Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotsEnabled")]
    pub snapshots_enabled: bool,

}




/// For a SQL-based Kinesis Data Analytics application, provides additional mapping information when the record    format uses delimiters, such as CSV. For example, the following sample records use CSV format,    where the records use the '\n' as the row delimiter and a comma (",") as    the column delimiter:
///
/// "name1", "address1"
///
/// "name2", "address2"
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CSVMappingParameters {


    /// 
    /// The column delimiter. For example, in a CSV format, a comma (",") is the typical column    delimiter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,


    /// 
    /// The row delimiter. For example, in a CSV format, '\n' is the typical    row delimiter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,

}




/// The configuration parameters for the default Amazon Glue database. You use this database       for SQL queries that you write in a Kinesis Data Analytics Studio notebook.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CatalogConfiguration {


    /// 
    /// The configuration parameters for the default Amazon Glue database. You use this database       for Apache Flink SQL queries and table API transforms that you write in a Kinesis Data       Analytics Studio notebook.
    /// 
    /// Required: No
    ///
    /// Type: GlueDataCatalogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueDataCatalogConfiguration")]
    pub glue_data_catalog_configuration: Option<GlueDataCatalogConfiguration>,

}




/// Describes an application's checkpointing configuration. Checkpointing is the process of persisting application state for fault    tolerance.    For more information, see         Checkpoints for Fault Tolerance in the    Apache Flink Documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CheckpointConfiguration {


    /// 
    /// Describes the interval in milliseconds between checkpoint operations.
    /// 
    /// NoteIf CheckpointConfiguration.ConfigurationType is DEFAULT,   the application will use a CheckpointInterval value of 60000, even if this value is set    to another value using this API or in application code.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CheckpointInterval")]
    pub checkpoint_interval: Option<i64>,


    /// 
    /// Describes whether checkpointing is enabled for a Flink-based Kinesis Data Analytics application.
    /// 
    /// NoteIf CheckpointConfiguration.ConfigurationType is DEFAULT,   the application will use a CheckpointingEnabled value of true, even if this value    is set to another value using this API or in application code.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CheckpointingEnabled")]
    pub checkpointing_enabled: Option<bool>,


    /// 
    /// Describes whether the application uses Kinesis Data Analytics' default checkpointing behavior.   You must set this property to CUSTOM in order to set the    CheckpointingEnabled, CheckpointInterval, or MinPauseBetweenCheckpoints parameters.
    /// 
    /// NoteIf this value is set to DEFAULT, the application will use the following values, even if they are set to other values using APIs or   application code:                                                              CheckpointingEnabled: true                             CheckpointInterval: 60000                             MinPauseBetweenCheckpoints: 5000
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOM | DEFAULT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationType")]
    pub configuration_type: CheckpointConfigurationConfigurationTypeEnum,


    /// 
    /// Describes the minimum time in milliseconds after a checkpoint operation completes that a    new checkpoint operation can start. If a checkpoint operation takes longer than the     CheckpointInterval, the application otherwise performs continual checkpoint    operations. For more information, see Tuning Checkpointing in the Apache Flink     Documentation.
    /// 
    /// NoteIf CheckpointConfiguration.ConfigurationType is DEFAULT,   the application will use a MinPauseBetweenCheckpoints value of 5000, even if this value is set using this    API or in application code.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinPauseBetweenCheckpoints")]
    pub min_pause_between_checkpoints: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CheckpointConfigurationConfigurationTypeEnum {

    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// DEFAULT
    #[serde(rename = "DEFAULT")]
    Default,

}

impl Default for CheckpointConfigurationConfigurationTypeEnum {
    fn default() -> Self {
        CheckpointConfigurationConfigurationTypeEnum::Custom
    }
}



/// Specifies either the application code, or the location of the application code, for a    Flink-based Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CodeContent {


    /// 
    /// Information about the Amazon S3 bucket that contains the application code.
    /// 
    /// Required: No
    ///
    /// Type: S3ContentLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3ContentLocation")]
    pub s3_content_location: Option<S3ContentLocation>,


    /// 
    /// The text-format code for a Flink-based Kinesis Data Analytics application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 102400
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextContent")]
    pub text_content: Option<String>,


    /// 
    /// The zip-format code for a Flink-based Kinesis Data Analytics application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZipFileContent")]
    pub zip_file_content: Option<String>,

}




/// The configuration of connectors and user-defined functions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomArtifactConfiguration {


    /// 
    /// Set this to either UDF or DEPENDENCY_JAR. UDF stands for user-defined functions. This type of artifact must be in an       S3 bucket. A DEPENDENCY_JAR can be in either Maven or an S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DEPENDENCY_JAR | UDF
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactType")]
    pub artifact_type: CustomArtifactConfigurationArtifactTypeEnum,


    /// 
    /// The parameters required to fully specify a Maven reference.
    /// 
    /// Required: No
    ///
    /// Type: MavenReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "MavenReference")]
    pub maven_reference: Option<MavenReference>,


    /// The location of the custom artifacts.
    ///
    /// Required: No
    ///
    /// Type: S3ContentLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3ContentLocation")]
    pub s3_content_location: Option<S3ContentLocation>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CustomArtifactConfigurationArtifactTypeEnum {

    /// DEPENDENCY_JAR
    #[serde(rename = "DEPENDENCY_JAR")]
    Dependencyjar,

    /// UDF
    #[serde(rename = "UDF")]
    Udf,

}

impl Default for CustomArtifactConfigurationArtifactTypeEnum {
    fn default() -> Self {
        CustomArtifactConfigurationArtifactTypeEnum::Dependencyjar
    }
}



/// The information required to deploy a Kinesis Data Analytics Studio notebook as an       application with durable state.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeployAsApplicationConfiguration {


    /// 
    /// The description of an Amazon S3 object that contains the Amazon Data Analytics       application, including the Amazon Resource Name (ARN) of the S3 bucket, the name of the       Amazon S3 object that contains the data, and the version number of the Amazon S3 object       that contains the data.
    /// 
    /// Required: Yes
    ///
    /// Type: S3ContentBaseLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3ContentLocation")]
    pub s3_content_location: S3ContentBaseLocation,

}




/// Describes execution properties for a Flink-based Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EnvironmentProperties {


    /// 
    /// Describes the execution property groups.
    /// 
    /// Required: No
    ///
    /// Type: List of PropertyGroup
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyGroups")]
    pub property_groups: Option<Vec<PropertyGroup>>,

}




/// Describes configuration parameters for a Flink-based Kinesis Data Analytics application or a Studio notebook.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FlinkApplicationConfiguration {


    /// 
    /// Describes an application's checkpointing configuration. Checkpointing is the    process of persisting application state for fault tolerance.    For more information, see         Checkpoints for Fault Tolerance in the    Apache Flink Documentation.
    /// 
    /// Required: No
    ///
    /// Type: CheckpointConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CheckpointConfiguration")]
    pub checkpoint_configuration: Option<CheckpointConfiguration>,


    /// 
    /// Describes configuration parameters for Amazon CloudWatch logging for an    application.
    /// 
    /// Required: No
    ///
    /// Type: MonitoringConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringConfiguration")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,


    /// 
    /// Describes parameters for how an application executes multiple tasks simultaneously.
    /// 
    /// Required: No
    ///
    /// Type: ParallelismConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelismConfiguration")]
    pub parallelism_configuration: Option<ParallelismConfiguration>,

}




/// Describes the starting parameters for a Flink-based Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FlinkRunConfiguration {


    /// 
    /// When restoring from a snapshot, specifies whether the runtime is allowed to skip a state that cannot     be mapped to the new program.   This will happen if the program is updated between snapshots to remove stateful parameters, and    state data in the snapshot no longer    corresponds to valid application data. For more information, see        Allowing Non-Restored State in the Apache Flink      documentation.
    /// 
    /// NoteThis value defaults to false. If you update your application without   specifying this parameter, AllowNonRestoredState will be set to false,   even if it was previously set to true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowNonRestoredState")]
    pub allow_non_restored_state: Option<bool>,

}




/// The configuration of the Glue Data Catalog that you use for Apache Flink SQL queries       and table API transforms that you write in an application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GlueDataCatalogConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the database.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseARN")]
    pub database_arn: Option<String>,

}




/// When you configure the application input for a SQL-based Kinesis Data Analytics application, you specify the streaming source, the in-application stream    name that is created,    and the mapping between the two.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Input {


    /// 
    /// Describes the number of in-application streams to create.
    /// 
    /// Required: No
    ///
    /// Type: InputParallelism
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputParallelism")]
    pub input_parallelism: Option<InputParallelism>,


    /// 
    /// The InputProcessingConfiguration for the input. An input processor transforms       records as they are received from the stream, before the application's SQL code       executes. Currently, the only input processing configuration available is InputLambdaProcessor.
    /// 
    /// Required: No
    ///
    /// Type: InputProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputProcessingConfiguration")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,


    /// 
    /// Describes the format of the data in the streaming source, and how each data element maps    to corresponding columns in the in-application stream that is being created.
    /// 
    /// Also used to describe the format of the reference data source.
    /// 
    /// Required: Yes
    ///
    /// Type: InputSchema
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputSchema")]
    pub input_schema: InputSchema,


    /// 
    /// If the streaming source is an Amazon Kinesis Data Firehose delivery stream, identifies the delivery stream's ARN.
    /// 
    /// Required: No
    ///
    /// Type: KinesisFirehoseInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseInput")]
    pub kinesis_firehose_input: Option<KinesisFirehoseInput>,


    /// 
    /// If the streaming source is an Amazon Kinesis data stream, identifies the stream's Amazon Resource Name (ARN).
    /// 
    /// Required: No
    ///
    /// Type: KinesisStreamsInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamsInput")]
    pub kinesis_streams_input: Option<KinesisStreamsInput>,


    /// 
    /// The name prefix to use when creating an in-application stream. Suppose that you specify a    prefix "MyInApplicationStream." Kinesis Data Analytics then creates one or more    (as per the InputParallelism count you specified) in-application streams with the    names "MyInApplicationStream_001," "MyInApplicationStream_002," and    so on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [^-\s<>&]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamePrefix")]
    pub name_prefix: String,

}




/// An object that contains the Amazon Resource Name (ARN) of the Amazon Lambda function that is    used to preprocess records in the stream in a SQL-based Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputLambdaProcessor {


    /// 
    /// The ARN of the Amazon Lambda function that operates on records in the stream.
    /// 
    /// NoteTo specify an earlier version of the Lambda function than the latest, include the         Lambda function version in the Lambda function ARN. For more information about         Lambda ARNs, see Example           ARNs: Amazon Lambda
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}




/// For a SQL-based Kinesis Data Analytics application, describes the number of    in-application streams to create for a given streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputParallelism {


    /// 
    /// The number of in-application streams to create.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,

}




/// For an SQL-based Amazon Kinesis Data Analytics application, describes a processor that       is used to preprocess the records in the stream before being processed by your       application code. Currently, the only input processor available is Amazon Lambda.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputProcessingConfiguration {


    /// 
    /// The InputLambdaProcessor that is used to preprocess the records in the stream       before being processed by your application code.
    /// 
    /// Required: No
    ///
    /// Type: InputLambdaProcessor
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLambdaProcessor")]
    pub input_lambda_processor: Option<InputLambdaProcessor>,

}




/// For a SQL-based Kinesis Data Analytics application, describes the format of the    data in the streaming source, and how each data element maps to corresponding columns created    in the in-application stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputSchema {


    /// 
    /// A list of RecordColumn objects.
    /// 
    /// Required: Yes
    ///
    /// Type: List of RecordColumn
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordColumns")]
    pub record_columns: Vec<RecordColumn>,


    /// 
    /// Specifies the encoding of the records in the streaming source. For example, UTF-8.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Maximum: 5
    ///
    /// Pattern: UTF-8
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordEncoding")]
    pub record_encoding: Option<String>,


    /// 
    /// Specifies the format of the records on the streaming source.
    /// 
    /// Required: Yes
    ///
    /// Type: RecordFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordFormat")]
    pub record_format: RecordFormat,

}




/// For a SQL-based Kinesis Data Analytics application, provides additional mapping    information when JSON is the record format on the streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JSONMappingParameters {


    /// 
    /// The path to the top-level parent that contains the records.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Pattern: ^(?=^\$)(?=^\S+$).*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,

}




/// For a SQL-based Kinesis Data Analytics application, identifies a Kinesis Data    Firehose delivery stream as the streaming source. You provide the delivery stream's Amazon    Resource Name (ARN).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisFirehoseInput {


    /// 
    /// The Amazon Resource Name (ARN) of the delivery stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}




/// Identifies a Kinesis data stream as the streaming source. You provide the    stream's Amazon Resource Name (ARN).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisStreamsInput {


    /// 
    /// The ARN of the input Kinesis data stream to read.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}




/// When you configure a SQL-based Kinesis Data Analytics application's input at the    time of creating or updating an application, provides additional mapping information specific    to the record format (such as JSON, CSV, or record fields delimited by some delimiter) on the    streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MappingParameters {


    /// 
    /// Provides additional mapping information when the record format uses delimiters    (for example, CSV).
    /// 
    /// Required: No
    ///
    /// Type: CSVMappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "CSVMappingParameters")]
    pub csvmapping_parameters: Option<CSVMappingParameters>,


    /// 
    /// Provides additional mapping information when JSON is the record format on the streaming source.
    /// 
    /// Required: No
    ///
    /// Type: JSONMappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "JSONMappingParameters")]
    pub jsonmapping_parameters: Option<JSONMappingParameters>,

}




/// The information required to specify a Maven reference. You can use Maven references to       specify dependency JAR files.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MavenReference {


    /// 
    /// The artifact ID of the Maven reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactId")]
    pub artifact_id: String,


    /// 
    /// The group ID of the Maven reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupId")]
    pub group_id: String,


    /// 
    /// The version of the Maven reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: String,

}




/// Describes configuration parameters for Amazon CloudWatch logging for a Java-based       Kinesis Data Analytics application. For more information about CloudWatch logging, see         Monitoring.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringConfiguration {


    /// 
    /// Describes whether to use the default CloudWatch logging configuration for an application.   You must set this property to CUSTOM in order to set the LogLevel or   MetricsLevel parameters.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOM | DEFAULT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationType")]
    pub configuration_type: MonitoringConfigurationConfigurationTypeEnum,


    /// 
    /// Describes the verbosity of the CloudWatch Logs for an application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEBUG | ERROR | INFO | WARN
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    pub log_level: Option<MonitoringConfigurationLogLevelEnum>,


    /// 
    /// Describes the granularity of the CloudWatch Logs for an application. The Parallelism   level is not recommended for applications with a Parallelism over 64 due to excessive costs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: APPLICATION | OPERATOR | PARALLELISM | TASK
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricsLevel")]
    pub metrics_level: Option<MonitoringConfigurationMetricsLevelEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum MonitoringConfigurationLogLevelEnum {

    /// DEBUG
    #[serde(rename = "DEBUG")]
    Debug,

    /// ERROR
    #[serde(rename = "ERROR")]
    Error,

    /// INFO
    #[serde(rename = "INFO")]
    Info,

    /// WARN
    #[serde(rename = "WARN")]
    Warn,

}

impl Default for MonitoringConfigurationLogLevelEnum {
    fn default() -> Self {
        MonitoringConfigurationLogLevelEnum::Debug
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MonitoringConfigurationMetricsLevelEnum {

    /// APPLICATION
    #[serde(rename = "APPLICATION")]
    Application,

    /// OPERATOR
    #[serde(rename = "OPERATOR")]
    Operator,

    /// PARALLELISM
    #[serde(rename = "PARALLELISM")]
    Parallelism,

    /// TASK
    #[serde(rename = "TASK")]
    Task,

}

impl Default for MonitoringConfigurationMetricsLevelEnum {
    fn default() -> Self {
        MonitoringConfigurationMetricsLevelEnum::Application
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MonitoringConfigurationConfigurationTypeEnum {

    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// DEFAULT
    #[serde(rename = "DEFAULT")]
    Default,

}

impl Default for MonitoringConfigurationConfigurationTypeEnum {
    fn default() -> Self {
        MonitoringConfigurationConfigurationTypeEnum::Custom
    }
}



/// Describes parameters for how a Flink-based Kinesis Data Analytics application    executes multiple tasks simultaneously. For more information about parallelism,    see Parallel Execution in the Apache Flink     Documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ParallelismConfiguration {


    /// 
    /// Describes whether the Kinesis Data Analytics service can increase the parallelism of the application in response to increased throughput.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingEnabled")]
    pub auto_scaling_enabled: Option<bool>,


    /// 
    /// Describes whether the application uses the default parallelism for the Kinesis Data Analytics service. You must set this property to CUSTOM   in order to change your application's AutoScalingEnabled, Parallelism, or ParallelismPerKPU properties.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOM | DEFAULT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationType")]
    pub configuration_type: ParallelismConfigurationConfigurationTypeEnum,


    /// 
    /// Describes the initial number of parallel tasks that a Java-based Kinesis Data       Analytics application can perform. The Kinesis Data Analytics service can increase this       number automatically if ParallelismConfiguration:AutoScalingEnabled is set to       true.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parallelism")]
    pub parallelism: Option<i64>,


    /// 
    /// Describes the number of parallel tasks that a Java-based Kinesis Data Analytics       application can perform per Kinesis Processing Unit (KPU) used by the application. For       more information about KPUs, see Amazon Kinesis Data Analytics       Pricing.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelismPerKPU")]
    pub parallelism_per_kpu: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ParallelismConfigurationConfigurationTypeEnum {

    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// DEFAULT
    #[serde(rename = "DEFAULT")]
    Default,

}

impl Default for ParallelismConfigurationConfigurationTypeEnum {
    fn default() -> Self {
        ParallelismConfigurationConfigurationTypeEnum::Custom
    }
}



/// Property key-value pairs passed into an application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PropertyGroup {


    /// 
    /// Describes the key of an application execution property key-value pair.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyGroupId")]
    pub property_group_id: Option<String>,


    /// 
    /// Describes the value of an application execution property key-value pair.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyMap")]
    pub property_map: Option<std::collections::HashMap<String, String>>,

}




/// For a SQL-based Kinesis Data Analytics application, describes the mapping of each    data element in the streaming source to the corresponding column in the in-application    stream.
///
/// Also used to describe the format of the reference data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordColumn {


    /// 
    /// A reference to the data element in the streaming input or the reference data    source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mapping")]
    pub mapping: Option<String>,


    /// 
    /// The name of the column that is created in the in-application input stream or reference    table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [^-\s<>&]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The type of column created in the in-application input stream or reference table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlType")]
    pub sql_type: String,

}




/// For a SQL-based Kinesis Data Analytics application, describes the record format    and relevant mapping information that should be applied to schematize the records on the    stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordFormat {


    /// 
    /// When you configure application input at the time of creating or updating an application,    provides additional mapping information specific to the record format (such as JSON, CSV, or    record fields delimited by some delimiter) on the streaming source.
    /// 
    /// Required: No
    ///
    /// Type: MappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappingParameters")]
    pub mapping_parameters: Option<MappingParameters>,


    /// 
    /// The type of record format.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: RecordFormatRecordFormatTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RecordFormatRecordFormatTypeEnum {

    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

}

impl Default for RecordFormatRecordFormatTypeEnum {
    fn default() -> Self {
        RecordFormatRecordFormatTypeEnum::Csv
    }
}



/// Describes the starting parameters for an Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RunConfiguration {


    /// 
    /// Describes the restore behavior of a restarting application.
    /// 
    /// Required: No
    ///
    /// Type: ApplicationRestoreConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationRestoreConfiguration")]
    pub application_restore_configuration: Option<ApplicationRestoreConfiguration>,


    /// 
    /// Describes the starting parameters for a Flink-based Kinesis Data Analytics application.
    /// 
    /// Required: No
    ///
    /// Type: FlinkRunConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlinkRunConfiguration")]
    pub flink_run_configuration: Option<FlinkRunConfiguration>,

}




/// The base location of the Amazon Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3ContentBaseLocation {


    /// 
    /// The base path for the S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [a-zA-Z0-9/!-_.*'()]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasePath")]
    pub base_path: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,

}




/// The location of an application or a custom artifact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3ContentLocation {


    /// 
    /// The Amazon Resource Name (ARN) for the S3 bucket containing the application code.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,


    /// 
    /// The file key for the object containing the application code.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileKey")]
    pub file_key: String,


    /// 
    /// The version of the object containing the application code.
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
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,

}




/// Describes the inputs, outputs, and reference data sources for a SQL-based Kinesis Data Analytics application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SqlApplicationConfiguration {


    /// 
    /// The array of Input objects describing       the input streams used by the application.
    /// 
    /// Required: No
    ///
    /// Type: List of Input
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inputs")]
    pub inputs: Option<Vec<Input>>,

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




/// Describes the parameters of a VPC used by the application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfiguration {


    /// 
    /// The array of SecurityGroup     IDs used by the VPC configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,


    /// 
    /// The array of Subnet IDs     used by the VPC configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}




/// The configuration of a Kinesis Data Analytics Studio notebook.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ZeppelinApplicationConfiguration {


    /// 
    /// The Amazon Glue Data Catalog that you use in queries in a Kinesis Data Analytics Studio       notebook.
    /// 
    /// Required: No
    ///
    /// Type: CatalogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogConfiguration")]
    pub catalog_configuration: Option<CatalogConfiguration>,


    /// 
    /// A list of CustomArtifactConfiguration objects.
    /// 
    /// Required: No
    ///
    /// Type: List of CustomArtifactConfiguration
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomArtifactsConfiguration")]
    pub custom_artifacts_configuration: Option<Vec<CustomArtifactConfiguration>>,


    /// 
    /// The information required to deploy a Kinesis Data Analytics Studio notebook as an       application with durable state.
    /// 
    /// Required: No
    ///
    /// Type: DeployAsApplicationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeployAsApplicationConfiguration")]
    pub deploy_as_application_configuration: Option<DeployAsApplicationConfiguration>,


    /// 
    /// The monitoring configuration of a Kinesis Data Analytics Studio notebook.
    /// 
    /// Required: No
    ///
    /// Type: ZeppelinMonitoringConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringConfiguration")]
    pub monitoring_configuration: Option<ZeppelinMonitoringConfiguration>,

}




/// Describes configuration parameters for Amazon CloudWatch logging for a Kinesis Data       Analytics Studio notebook. For more information about CloudWatch logging, see Monitoring.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ZeppelinMonitoringConfiguration {


    /// 
    /// The verbosity of the CloudWatch Logs for an application. You can set it to INFO, WARN, ERROR, or DEBUG.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEBUG | ERROR | INFO | WARN
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    pub log_level: Option<ZeppelinMonitoringConfigurationLogLevelEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ZeppelinMonitoringConfigurationLogLevelEnum {

    /// DEBUG
    #[serde(rename = "DEBUG")]
    Debug,

    /// ERROR
    #[serde(rename = "ERROR")]
    Error,

    /// INFO
    #[serde(rename = "INFO")]
    Info,

    /// WARN
    #[serde(rename = "WARN")]
    Warn,

}

impl Default for ZeppelinMonitoringConfigurationLogLevelEnum {
    fn default() -> Self {
        ZeppelinMonitoringConfigurationLogLevelEnum::Debug
    }
}

