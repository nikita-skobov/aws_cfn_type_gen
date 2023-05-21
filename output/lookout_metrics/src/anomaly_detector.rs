

/// The AWS::LookoutMetrics::AnomalyDetector type creates an anomaly detector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAnomalyDetector {


    /// 
    /// The name of the detector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AnomalyDetectorName")]
    pub anomaly_detector_name: Option<String>,


    /// 
    /// A description of the detector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnomalyDetectorDescription")]
    pub anomaly_detector_description: Option<String>,


    /// 
    /// Contains information about the configuration of the anomaly detector.
    /// 
    /// Required: Yes
    ///
    /// Type: AnomalyDetectorConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnomalyDetectorConfig")]
    pub anomaly_detector_config: AnomalyDetectorConfig,


    /// 
    /// The ARN of the KMS key to use to encrypt your data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,


    /// 
    /// The detector's dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: List of MetricSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSetList")]
    pub metric_set_list: Vec<MetricSet>,

}



impl cfn_resources::CfnResource for CfnAnomalyDetector {
    fn type_string() -> &'static str {
        "AWS::LookoutMetrics::AnomalyDetector"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains information about the Amazon Relational Database Service (RDS) configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RDSSourceConfig {


    /// 
    /// The name of the RDS database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// 
    /// An object containing information about the Amazon Virtual Private Cloud (VPC) configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: VpcConfiguration,


    /// 
    /// The port number where the database can be accessed.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabasePort")]
    pub database_port: i64,


    /// 
    /// The name of the table in the database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Secrets Manager role.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: String,


    /// 
    /// The Amazon Resource Name (ARN) of the role.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The host name of the database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseHost")]
    pub database_host: String,


    /// 
    /// A string identifying the database instance.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBInstanceIdentifier")]
    pub dbinstance_identifier: String,

}




/// Contains information about how the source data should be interpreted.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricSource {


    /// 
    /// Details about an AppFlow datasource.
    /// 
    /// Required: No
    ///
    /// Type: AppFlowConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppFlowConfig")]
    pub app_flow_config: Option<AppFlowConfig>,


    /// 
    /// Details about an Amazon Relational Database Service (RDS) datasource.
    /// 
    /// Required: No
    ///
    /// Type: RDSSourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RDSSourceConfig")]
    pub rdssource_config: Option<RDSSourceConfig>,


    /// 
    /// Details about an Amazon CloudWatch monitoring datasource.
    /// 
    /// Required: No
    ///
    /// Type: CloudwatchConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchConfig")]
    pub cloudwatch_config: Option<CloudwatchConfig>,


    /// 
    /// Contains information about the configuration of the S3 bucket that contains source files.
    /// 
    /// Required: No
    ///
    /// Type: S3SourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3SourceConfig")]
    pub s3_source_config: Option<S3SourceConfig>,


    /// 
    /// Details about an Amazon Redshift database datasource.
    /// 
    /// Required: No
    ///
    /// Type: RedshiftSourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedshiftSourceConfig")]
    pub redshift_source_config: Option<RedshiftSourceConfig>,

}




/// Contains information about a detector's configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnomalyDetectorConfig {


    /// 
    /// The frequency at which the detector analyzes its source data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnomalyDetectorFrequency")]
    pub anomaly_detector_frequency: String,

}




/// Details about an Amazon CloudWatch datasource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudwatchConfig {


    /// 
    /// An IAM role that gives Amazon Lookout for Metrics permission to access data in Amazon CloudWatch.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}




/// Contains information about the column used to track time in a source data file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TimestampColumn {


    /// 
    /// The name of the timestamp column.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnName")]
    pub column_name: Option<String>,


    /// 
    /// The format of the timestamp column.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnFormat")]
    pub column_format: Option<String>,

}




/// Contains information about how a source CSV data file should be analyzed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CsvFormatDescriptor {


    /// 
    /// Whether or not the source CSV file contains a header.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainsHeader")]
    pub contains_header: Option<bool>,


    /// 
    /// The character used to delimit the source CSV file.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,


    /// 
    /// The level of compression of the source CSV file.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileCompression")]
    pub file_compression: Option<String>,


    /// 
    /// The character set in which the source CSV file is written.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Charset")]
    pub charset: Option<String>,


    /// 
    /// A list of the source CSV file's headers, if any.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderList")]
    pub header_list: Option<Vec<String>>,


    /// 
    /// The character used as a quote character.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QuoteSymbol")]
    pub quote_symbol: Option<String>,

}




/// Contains configuration information about the Amazon Virtual Private Cloud (VPC).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfiguration {


    /// 
    /// An array of strings containing the list of security groups.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIdList")]
    pub security_group_id_list: Vec<String>,


    /// 
    /// An array of strings containing the Amazon VPC subnet IDs (e.g., subnet-0bb1c79de3EXAMPLE.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIdList")]
    pub subnet_id_list: Vec<String>,

}




/// A calculation made by contrasting a measure and a dimension from your source data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Metric {


    /// 
    /// The name of the metric.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// The namespace for the metric.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,


    /// 
    /// The function with which the metric is calculated.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: String,

}




/// Contains information about a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricSet {


    /// 
    /// The frequency with which the source data will be analyzed for anomalies.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSetFrequency")]
    pub metric_set_frequency: Option<String>,


    /// 
    /// After an interval ends, the amount of seconds that the detector waits before importing data. Offset is only supported for S3, Redshift, Athena and datasources.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Offset")]
    pub offset: Option<i64>,


    /// 
    /// A list of the fields you want to treat as dimensions.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionList")]
    pub dimension_list: Option<Vec<String>>,


    /// 
    /// A list of metrics that the dataset will contain.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Metric
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricList")]
    pub metric_list: Vec<Metric>,


    /// 
    /// Contains information about the column used for tracking time in your source data.
    /// 
    /// Required: No
    ///
    /// Type: TimestampColumn
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampColumn")]
    pub timestamp_column: Option<TimestampColumn>,


    /// 
    /// Contains information about how the source data should be interpreted.
    /// 
    /// Required: Yes
    ///
    /// Type: MetricSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSource")]
    pub metric_source: MetricSource,


    /// 
    /// A description of the dataset you are creating.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSetDescription")]
    pub metric_set_description: Option<String>,


    /// 
    /// The time zone in which your source data was recorded.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,


    /// 
    /// The name of the dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSetName")]
    pub metric_set_name: String,

}




/// Provides information about the Amazon Redshift database configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedshiftSourceConfig {


    /// 
    /// The Amazon Resource Name (ARN) of the role providing access to the database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// Contains information about the Amazon Virtual Private Cloud (VPC) configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: VpcConfiguration,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Secrets Manager role.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: String,


    /// 
    /// The port number where the database can be accessed.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabasePort")]
    pub database_port: i64,


    /// 
    /// The name of the database host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseHost")]
    pub database_host: String,


    /// 
    /// A string identifying the Redshift cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,


    /// 
    /// The Redshift database name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// 
    /// The table name of the Redshift database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,

}




/// Contains information about a source file's formatting.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FileFormatDescriptor {


    /// 
    /// Contains information about how a source JSON data file should be analyzed.
    /// 
    /// Required: No
    ///
    /// Type: JsonFormatDescriptor
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonFormatDescriptor")]
    pub json_format_descriptor: Option<JsonFormatDescriptor>,


    /// 
    /// Contains information about how a source CSV data file should be analyzed.
    /// 
    /// Required: No
    ///
    /// Type: CsvFormatDescriptor
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvFormatDescriptor")]
    pub csv_format_descriptor: Option<CsvFormatDescriptor>,

}




/// Details about an Amazon AppFlow flow datasource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AppFlowConfig {


    /// 
    /// name of the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowName")]
    pub flow_name: String,


    /// 
    /// An IAM role that gives Amazon Lookout for Metrics permission to access the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}




/// Contains information about the configuration of the S3 bucket that contains source files.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3SourceConfig {


    /// 
    /// A list of templated paths to the source files.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplatedPathList")]
    pub templated_path_list: Option<Vec<String>>,


    /// 
    /// The ARN of an IAM role that has read and write access permissions to the source S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// A list of paths to the historical data files.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HistoricalDataPathList")]
    pub historical_data_path_list: Option<Vec<String>>,


    /// 
    /// Contains information about a source file's formatting.
    /// 
    /// Required: Yes
    ///
    /// Type: FileFormatDescriptor
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileFormatDescriptor")]
    pub file_format_descriptor: FileFormatDescriptor,

}




/// Contains information about how a source JSON data file should be analyzed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonFormatDescriptor {


    /// 
    /// The level of compression of the source CSV file.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileCompression")]
    pub file_compression: Option<String>,


    /// 
    /// The character set in which the source JSON file is written.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Charset")]
    pub charset: Option<String>,

}


