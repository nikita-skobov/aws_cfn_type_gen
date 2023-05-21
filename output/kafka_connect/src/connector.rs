

/// Creates a connector using the specified properties.
#[derive(Default, serde::Serialize)]
pub struct CfnConnector {


    /// 
    /// The version of Kafka Connect. It has to be compatible with both the Apache Kafka     cluster's version and the plugins.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KafkaConnectVersion")]
    pub kafka_connect_version: String,


    /// 
    /// The type of client authentication used to connect to the Apache Kafka cluster. The value     is NONE when no client authentication is used.
    /// 
    /// Required: Yes
    ///
    /// Type: KafkaClusterClientAuthentication
    ///
    /// Update requires: Replacement
    #[serde(rename = "KafkaClusterClientAuthentication")]
    pub kafka_cluster_client_authentication: KafkaClusterClientAuthentication,


    /// 
    /// Specifies which plugin to use for the connector. You must specify a single-element list. Amazon MSK Connect does not currently support specifying multiple plugins.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Plugin
    ///
    /// Update requires: Replacement
    #[serde(rename = "Plugins")]
    pub plugins: Vec<Plugin>,


    /// 
    /// The connector's compute capacity settings.
    /// 
    /// Required: Yes
    ///
    /// Type: Capacity
    ///
    /// Update requires: No interruption
    #[serde(rename = "Capacity")]
    pub capacity: Capacity,


    /// 
    /// The name of the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorName")]
    pub connector_name: String,


    /// 
    /// The worker configurations that are in use with the connector.
    /// 
    /// Required: No
    ///
    /// Type: WorkerConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkerConfiguration")]
    pub worker_configuration: Option<WorkerConfiguration>,


    /// 
    /// The details of the Apache Kafka cluster to which the connector is connected.
    /// 
    /// Required: Yes
    ///
    /// Type: KafkaCluster
    ///
    /// Update requires: Replacement
    #[serde(rename = "KafkaCluster")]
    pub kafka_cluster: KafkaCluster,


    /// 
    /// Details of encryption in transit to the Apache Kafka cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: KafkaClusterEncryptionInTransit
    ///
    /// Update requires: Replacement
    #[serde(rename = "KafkaClusterEncryptionInTransit")]
    pub kafka_cluster_encryption_in_transit: KafkaClusterEncryptionInTransit,


    /// 
    /// The description of the connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorDescription")]
    pub connector_description: Option<String>,


    /// The configuration of the connector.
    ///
    /// Required: Yes
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorConfiguration")]
    pub connector_configuration: std::collections::HashMap<String, String>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role used by the connector to access Amazon     Web Services resources.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceExecutionRoleArn")]
    pub service_execution_role_arn: String,


    /// 
    /// The settings for delivering connector logs to Amazon CloudWatch Logs.
    /// 
    /// Required: No
    ///
    /// Type: LogDelivery
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogDelivery")]
    pub log_delivery: Option<LogDelivery>,

}


/// The client authentication information used in order to authenticate with the Apache     Kafka cluster.
#[derive(Default, serde::Serialize)]
pub struct KafkaClusterClientAuthentication {


    /// 
    /// The type of client authentication used to connect to the Apache Kafka cluster. Value     NONE means that no client authentication is used.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,

}


/// The details of the Apache Kafka cluster to which the connector is connected.
#[derive(Default, serde::Serialize)]
pub struct KafkaCluster {


    /// 
    /// The Apache Kafka cluster to which the connector is connected.
    /// 
    /// Required: Yes
    ///
    /// Type: ApacheKafkaCluster
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApacheKafkaCluster")]
    pub apache_kafka_cluster: ApacheKafkaCluster,

}


/// Workers can send worker logs to different destination types. This configuration     specifies the details of these destinations.
#[derive(Default, serde::Serialize)]
pub struct WorkerLogDelivery {


    /// 
    /// Details about delivering logs to Amazon CloudWatch Logs.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLogsLogDelivery
    ///
    /// Update requires: Replacement
    #[serde(rename = "CloudWatchLogs")]
    pub cloud_watch_logs: Option<CloudWatchLogsLogDelivery>,


    /// 
    /// Details about delivering logs to Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3LogDelivery
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3")]
    pub s3: Option<S3LogDelivery>,


    /// 
    /// Details about delivering logs to Amazon Kinesis Data Firehose.
    /// 
    /// Required: No
    ///
    /// Type: FirehoseLogDelivery
    ///
    /// Update requires: Replacement
    #[serde(rename = "Firehose")]
    pub firehose: Option<FirehoseLogDelivery>,

}


/// A plugin is an AWS resource that contains the code that defines a connector's logic.
#[derive(Default, serde::Serialize)]
pub struct CustomPlugin {


    /// 
    /// The Amazon Resource Name (ARN) of the custom plugin.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomPluginArn")]
    pub custom_plugin_arn: String,


    /// 
    /// The revision of the custom plugin.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Revision")]
    pub revision: i64,

}


/// The settings for delivering logs to Amazon Kinesis Data Firehose.
#[derive(Default, serde::Serialize)]
pub struct FirehoseLogDelivery {


    /// 
    /// The name of the Kinesis Data Firehose delivery stream that is the destination for log     delivery.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: Option<String>,


    /// 
    /// Specifies whether connector logs get delivered to Amazon Kinesis Data Firehose.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}


/// The scale-in policy for the connector.
#[derive(Default, serde::Serialize)]
pub struct ScaleInPolicy {


    /// 
    /// Specifies the CPU utilization percentage threshold at which you want connector scale in     to be triggered.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CpuUtilizationPercentage")]
    pub cpu_utilization_percentage: i64,

}


/// Details about a connector's provisioned capacity.
#[derive(Default, serde::Serialize)]
pub struct ProvisionedCapacity {


    /// 
    /// The number of workers that are allocated to the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkerCount")]
    pub worker_count: i64,


    /// 
    /// The number of microcontroller units (MCUs) allocated to each connector worker. The valid     values are 1,2,4,8.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "McuCount")]
    pub mcu_count: Option<i64>,

}


/// The configuration of the workers, which are the processes that run the connector     logic.
#[derive(Default, serde::Serialize)]
pub struct WorkerConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the worker configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkerConfigurationArn")]
    pub worker_configuration_arn: String,


    /// 
    /// The revision of the worker configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Revision")]
    pub revision: i64,

}


/// The settings for delivering connector logs to Amazon CloudWatch Logs.
#[derive(Default, serde::Serialize)]
pub struct CloudWatchLogsLogDelivery {


    /// 
    /// Whether log delivery to Amazon CloudWatch Logs is enabled.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Enabled")]
    pub enabled: bool,


    /// 
    /// The name of the CloudWatch log group that is the destination for log delivery.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogGroup")]
    pub log_group: Option<String>,

}


/// The details of the Apache Kafka cluster to which the connector is connected.
#[derive(Default, serde::Serialize)]
pub struct ApacheKafkaCluster {


    /// 
    /// Details of an Amazon VPC which has network connectivity to the Apache Kafka     cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: Vpc
    ///
    /// Update requires: Replacement
    #[serde(rename = "Vpc")]
    pub vpc: Vpc,


    /// 
    /// The bootstrap servers of the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BootstrapServers")]
    pub bootstrap_servers: String,

}


/// Specifies how the connector scales.
#[derive(Default, serde::Serialize)]
pub struct AutoScaling {


    /// 
    /// The sacle-out policy for the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: ScaleOutPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScaleOutPolicy")]
    pub scale_out_policy: ScaleOutPolicy,


    /// 
    /// The sacle-in policy for the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: ScaleInPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScaleInPolicy")]
    pub scale_in_policy: ScaleInPolicy,


    /// 
    /// The maximum number of workers allocated to the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxWorkerCount")]
    pub max_worker_count: i64,


    /// 
    /// The minimum number of workers allocated to the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinWorkerCount")]
    pub min_worker_count: i64,


    /// 
    /// The number of microcontroller units (MCUs) allocated to each connector worker. The valid     values are 1,2,4,8.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "McuCount")]
    pub mcu_count: i64,

}


/// Details about delivering logs to Amazon S3.
#[derive(Default, serde::Serialize)]
pub struct S3LogDelivery {


    /// 
    /// The S3 prefix that is the destination for log delivery.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// Specifies whether connector logs get sent to the specified Amazon S3 destination.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Enabled")]
    pub enabled: bool,


    /// 
    /// The name of the S3 bucket that is the destination for log delivery.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,

}


/// Details about log delivery.
#[derive(Default, serde::Serialize)]
pub struct LogDelivery {


    /// 
    /// The workers can send worker logs to different destination types. This configuration     specifies the details of these destinations.
    /// 
    /// Required: Yes
    ///
    /// Type: WorkerLogDelivery
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkerLogDelivery")]
    pub worker_log_delivery: WorkerLogDelivery,

}


/// The scale-out policy for the connector.
#[derive(Default, serde::Serialize)]
pub struct ScaleOutPolicy {


    /// 
    /// The CPU utilization percentage threshold at which you want connector scale out to be     triggered.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CpuUtilizationPercentage")]
    pub cpu_utilization_percentage: i64,

}


/// Information about the VPC in which the connector resides.
#[derive(Default, serde::Serialize)]
pub struct Vpc {


    /// 
    /// The subnets for the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,


    /// 
    /// The security groups for the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Vec<String>,

}


/// A plugin is an AWS resource that contains the code that defines your connector logic.
#[derive(Default, serde::Serialize)]
pub struct Plugin {


    /// 
    /// Details about a custom plugin.
    /// 
    /// Required: Yes
    ///
    /// Type: CustomPlugin
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomPlugin")]
    pub custom_plugin: CustomPlugin,

}


/// Information about the capacity of the connector, whether it is auto scaled or     provisioned.
#[derive(Default, serde::Serialize)]
pub struct Capacity {


    /// 
    /// Information about the auto scaling parameters for the connector.
    /// 
    /// Required: No
    ///
    /// Type: AutoScaling
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScaling")]
    pub auto_scaling: Option<AutoScaling>,


    /// 
    /// Details about a fixed capacity allocated to a connector.
    /// 
    /// Required: No
    ///
    /// Type: ProvisionedCapacity
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedCapacity")]
    pub provisioned_capacity: Option<ProvisionedCapacity>,

}


/// Details of encryption in transit to the Apache Kafka cluster.
#[derive(Default, serde::Serialize)]
pub struct KafkaClusterEncryptionInTransit {


    /// 
    /// The type of encryption in transit to the Apache Kafka cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,

}