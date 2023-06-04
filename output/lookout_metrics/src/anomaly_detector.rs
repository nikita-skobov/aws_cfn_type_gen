/// The AWS::LookoutMetrics::AnomalyDetector type creates an anomaly detector.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAnomalyDetector {
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
    /// A description of the detector.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnomalyDetectorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the detector.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AnomalyDetectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_name: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the KMS key to use to encrypt your data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<cfn_resources::StrVal>,

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

    #[serde(skip_serializing)]
    pub att_arn: CfnAnomalyDetectorarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAnomalyDetectorarn;
impl CfnAnomalyDetectorarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnAnomalyDetector {
    fn type_string(&self) -> &'static str {
        "AWS::LookoutMetrics::AnomalyDetector"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.anomaly_detector_config.validate()?;

        Ok(())
    }
}

/// Contains information about a detector's configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub anomaly_detector_frequency: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AnomalyDetectorConfig {
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

/// Details about an Amazon AppFlow flow datasource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub flow_name: cfn_resources::StrVal,

    ///
    /// An IAM role that gives Amazon Lookout for Metrics permission to access the flow.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AppFlowConfig {
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

/// Details about an Amazon CloudWatch datasource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CloudwatchConfig {
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

/// Contains information about how a source CSV data file should be analyzed.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CsvFormatDescriptor {
    ///
    /// The character set in which the source CSV file is written.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Charset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<cfn_resources::StrVal>,

    ///
    /// Whether or not the source CSV file contains a header.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainsHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<cfn_resources::StrVal>,

    ///
    /// The level of compression of the source CSV file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_compression: Option<cfn_resources::StrVal>,

    ///
    /// A list of the source CSV file's headers, if any.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderList")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_symbol: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CsvFormatDescriptor {
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

/// Contains information about a source file's formatting.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FileFormatDescriptor {
    ///
    /// Contains information about how a source CSV data file should be analyzed.
    ///
    /// Required: No
    ///
    /// Type: CsvFormatDescriptor
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvFormatDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_format_descriptor: Option<CsvFormatDescriptor>,

    ///
    /// Contains information about how a source JSON data file should be analyzed.
    ///
    /// Required: No
    ///
    /// Type: JsonFormatDescriptor
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonFormatDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_format_descriptor: Option<JsonFormatDescriptor>,
}

impl cfn_resources::CfnResource for FileFormatDescriptor {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.csv_format_descriptor
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.json_format_descriptor
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about how a source JSON data file should be analyzed.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JsonFormatDescriptor {
    ///
    /// The character set in which the source JSON file is written.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Charset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<cfn_resources::StrVal>,

    ///
    /// The level of compression of the source CSV file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_compression: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for JsonFormatDescriptor {
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

/// A calculation made by contrasting a measure and a dimension from your source data.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Metric {
    ///
    /// The function with which the metric is calculated.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: cfn_resources::StrVal,

    ///
    /// The name of the metric.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: cfn_resources::StrVal,

    ///
    /// The namespace for the metric.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Metric {
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

/// Contains information about a dataset.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricSet {
    ///
    /// A list of the fields you want to treat as dimensions.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// A description of the dataset you are creating.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSetDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_set_description: Option<cfn_resources::StrVal>,

    ///
    /// The frequency with which the source data will be analyzed for anomalies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSetFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_set_frequency: Option<cfn_resources::StrVal>,

    ///
    /// The name of the dataset.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSetName")]
    pub metric_set_name: cfn_resources::StrVal,

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
    /// After an interval ends, the amount of seconds that the detector waits before importing data. Offset is only supported for S3, Redshift, Athena and datasources.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    ///
    /// Contains information about the column used for tracking time in your source data.
    ///
    /// Required: No
    ///
    /// Type: TimestampColumn
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_column: Option<TimestampColumn>,

    ///
    /// The time zone in which your source data was recorded.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MetricSet {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.metric_source.validate()?;

        self.timestamp_column
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about how the source data should be interpreted.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_flow_config: Option<AppFlowConfig>,

    ///
    /// Details about an Amazon CloudWatch monitoring datasource.
    ///
    /// Required: No
    ///
    /// Type: CloudwatchConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_config: Option<CloudwatchConfig>,

    ///
    /// Details about an Amazon Relational Database Service (RDS) datasource.
    ///
    /// Required: No
    ///
    /// Type: RDSSourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RDSSourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdssource_config: Option<RDSSourceConfig>,

    ///
    /// Details about an Amazon Redshift database datasource.
    ///
    /// Required: No
    ///
    /// Type: RedshiftSourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedshiftSourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_source_config: Option<RedshiftSourceConfig>,

    ///
    /// Contains information about the configuration of the S3 bucket that contains source files.
    ///
    /// Required: No
    ///
    /// Type: S3SourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3SourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_source_config: Option<S3SourceConfig>,
}

impl cfn_resources::CfnResource for MetricSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.app_flow_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cloudwatch_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rdssource_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.redshift_source_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_source_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about the Amazon Relational Database Service (RDS) configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RDSSourceConfig {
    ///
    /// A string identifying the database instance.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBInstanceIdentifier")]
    pub dbinstance_identifier: cfn_resources::StrVal,

    ///
    /// The host name of the database.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseHost")]
    pub database_host: cfn_resources::StrVal,

    ///
    /// The name of the RDS database.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: cfn_resources::StrVal,

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
    /// The Amazon Resource Name (ARN) of the role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the AWS Secrets Manager role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: cfn_resources::StrVal,

    ///
    /// The name of the table in the database.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: cfn_resources::StrVal,

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
}

impl cfn_resources::CfnResource for RDSSourceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.vpc_configuration.validate()?;

        Ok(())
    }
}

/// Provides information about the Amazon Redshift database configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RedshiftSourceConfig {
    ///
    /// A string identifying the Redshift cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: cfn_resources::StrVal,

    ///
    /// The name of the database host.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseHost")]
    pub database_host: cfn_resources::StrVal,

    ///
    /// The Redshift database name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: cfn_resources::StrVal,

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
    /// The Amazon Resource Name (ARN) of the role providing access to the database.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the AWS Secrets Manager role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: cfn_resources::StrVal,

    ///
    /// The table name of the Redshift database.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: cfn_resources::StrVal,

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
}

impl cfn_resources::CfnResource for RedshiftSourceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.vpc_configuration.validate()?;

        Ok(())
    }
}

/// Contains information about the configuration of the S3 bucket that contains source files.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct S3SourceConfig {
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

    ///
    /// A list of paths to the historical data files.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HistoricalDataPathList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_data_path_list: Option<Vec<String>>,

    ///
    /// The ARN of an IAM role that has read and write access permissions to the source S3 bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// A list of templated paths to the source files.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplatedPathList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templated_path_list: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for S3SourceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.file_format_descriptor.validate()?;

        Ok(())
    }
}

/// Contains information about the column used to track time in a source data file.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TimestampColumn {
    ///
    /// The format of the timestamp column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_format: Option<cfn_resources::StrVal>,

    ///
    /// The name of the timestamp column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TimestampColumn {
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

/// Contains configuration information about the Amazon Virtual Private Cloud (VPC).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for VpcConfiguration {
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
