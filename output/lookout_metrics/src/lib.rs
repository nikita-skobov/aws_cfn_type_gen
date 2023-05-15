
pub mod cfn_alert {

#[derive(serde::Serialize, Default)]
pub struct CfnAlert {
    /// The name of the alert. If not provided, a name is generated automatically.
    #[serde(rename = "AlertName")]
    pub alert_name: Option<String>,
    /// The action to be taken by the alert when an anomaly is detected.
    #[serde(rename = "Action")]
    pub action: Action,
    /// The Amazon resource name (ARN) of the Anomaly Detector to alert.
    #[serde(rename = "AnomalyDetectorArn")]
    pub anomaly_detector_arn: String,
    /// A description for the alert.
    #[serde(rename = "AlertDescription")]
    pub alert_description: Option<String>,
    /// A number between 0 and 100 (inclusive) that tunes the sensitivity of the alert.
    #[serde(rename = "AlertSensitivityThreshold")]
    pub alert_sensitivity_threshold: usize,

}

pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct SNSConfiguration {
    #[serde(rename = "RoleArn")]
    pub role_arn: Arn,
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaConfiguration {
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: Arn,
    #[serde(rename = "RoleArn")]
    pub role_arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "SNSConfiguration")]
    pub snsconfiguration: Option<SNSConfiguration>,
    #[serde(rename = "LambdaConfiguration")]
    pub lambda_configuration: Option<LambdaConfiguration>,

}


}

pub mod cfn_anomaly_detector {

#[derive(serde::Serialize, Default)]
pub struct CfnAnomalyDetector {
    /// KMS key used to encrypt the AnomalyDetector data
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,
    /// A description for the AnomalyDetector.
    #[serde(rename = "AnomalyDetectorDescription")]
    pub anomaly_detector_description: Option<String>,
    /// Name for the Amazon Lookout for Metrics Anomaly Detector
    #[serde(rename = "AnomalyDetectorName")]
    pub anomaly_detector_name: Option<String>,
    /// Configuration options for the AnomalyDetector
    #[serde(rename = "AnomalyDetectorConfig")]
    pub anomaly_detector_config: AnomalyDetectorConfig,
    /// List of metric sets for anomaly detection
    #[serde(rename = "MetricSetList")]
    pub metric_set_list: Vec<MetricSet>,

}

pub type SecretManagerArn = String;
#[derive(serde::Serialize, Default)]
pub struct CloudwatchConfig {
    #[serde(rename = "RoleArn")]
    pub role_arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct AppFlowConfig {
    #[serde(rename = "RoleArn")]
    pub role_arn: Arn,
    #[serde(rename = "FlowName")]
    pub flow_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct CsvFormatDescriptor {
    #[serde(rename = "FileCompression")]
    pub file_compression: Option<String>,
    #[serde(rename = "Charset")]
    pub charset: Option<Charset>,
    #[serde(rename = "ContainsHeader")]
    pub contains_header: Option<bool>,
    #[serde(rename = "QuoteSymbol")]
    pub quote_symbol: Option<String>,
    #[serde(rename = "HeaderList")]
    pub header_list: Option<Vec<ColumnName>>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

}
pub type DatabaseHost = String;pub type Arn = String;pub type DatabasePort = usize;pub type TableName = String;
#[derive(serde::Serialize, Default)]
pub struct RDSSourceConfig {
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: VpcConfiguration,
    #[serde(rename = "DatabaseHost")]
    pub database_host: DatabaseHost,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: SecretManagerArn,
    #[serde(rename = "DatabasePort")]
    pub database_port: DatabasePort,
    #[serde(rename = "DBInstanceIdentifier")]
    pub dbinstance_identifier: String,
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct Metric {
    #[serde(rename = "AggregationFunction")]
    pub aggregation_function: String,
    #[serde(rename = "MetricName")]
    pub metric_name: ColumnName,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

}
pub type ColumnName = String;
#[derive(serde::Serialize, Default)]
pub struct AnomalyDetectorConfig {
    #[serde(rename = "AnomalyDetectorFrequency")]
    pub anomaly_detector_frequency: AnomalyDetectorFrequency,

}

#[derive(serde::Serialize, Default)]
pub struct FileFormatDescriptor {
    #[serde(rename = "CsvFormatDescriptor")]
    pub csv_format_descriptor: Option<CsvFormatDescriptor>,
    #[serde(rename = "JsonFormatDescriptor")]
    pub json_format_descriptor: Option<JsonFormatDescriptor>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfiguration {
    #[serde(rename = "SubnetIdList")]
    pub subnet_id_list: SubnetIdList,
    #[serde(rename = "SecurityGroupIdList")]
    pub security_group_id_list: SecurityGroupIdList,

}

#[derive(serde::Serialize, Default)]
pub struct JsonFormatDescriptor {
    #[serde(rename = "FileCompression")]
    pub file_compression: Option<String>,
    #[serde(rename = "Charset")]
    pub charset: Option<Charset>,

}
pub type SecurityGroupIdList = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct MetricSet {
    #[serde(rename = "MetricSetFrequency")]
    pub metric_set_frequency: Option<String>,
    #[serde(rename = "MetricList")]
    pub metric_list: Vec<Metric>,
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,
    #[serde(rename = "DimensionList")]
    pub dimension_list: Option<Vec<ColumnName>>,
    #[serde(rename = "MetricSetDescription")]
    pub metric_set_description: Option<String>,
    #[serde(rename = "MetricSource")]
    pub metric_source: MetricSource,
    #[serde(rename = "TimestampColumn")]
    pub timestamp_column: Option<TimestampColumn>,
    #[serde(rename = "Offset")]
    pub offset: Option<usize>,
    #[serde(rename = "MetricSetName")]
    pub metric_set_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3SourceConfig {
    #[serde(rename = "TemplatedPathList")]
    pub templated_path_list: Option<Vec<String>>,
    #[serde(rename = "HistoricalDataPathList")]
    pub historical_data_path_list: Option<Vec<String>>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Arn,
    #[serde(rename = "FileFormatDescriptor")]
    pub file_format_descriptor: FileFormatDescriptor,

}
pub type Charset = String;
#[derive(serde::Serialize, Default)]
pub struct RedshiftSourceConfig {
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    #[serde(rename = "DatabasePort")]
    pub database_port: DatabasePort,
    #[serde(rename = "RoleArn")]
    pub role_arn: Arn,
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: SecretManagerArn,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "DatabaseHost")]
    pub database_host: DatabaseHost,
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: VpcConfiguration,

}
pub type AnomalyDetectorFrequency = String;
#[derive(serde::Serialize, Default)]
pub struct MetricSource {
    #[serde(rename = "AppFlowConfig")]
    pub app_flow_config: Option<AppFlowConfig>,
    #[serde(rename = "RDSSourceConfig")]
    pub rdssource_config: Option<RDSSourceConfig>,
    #[serde(rename = "S3SourceConfig")]
    pub s3_source_config: Option<S3SourceConfig>,
    #[serde(rename = "RedshiftSourceConfig")]
    pub redshift_source_config: Option<RedshiftSourceConfig>,
    #[serde(rename = "CloudwatchConfig")]
    pub cloudwatch_config: Option<CloudwatchConfig>,

}
pub type SubnetIdList = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct TimestampColumn {
    #[serde(rename = "ColumnFormat")]
    pub column_format: Option<String>,
    #[serde(rename = "ColumnName")]
    pub column_name: Option<ColumnName>,

}


}
