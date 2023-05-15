
pub mod cfn_connector {

#[derive(serde::Serialize, Default)]
pub struct CfnConnector {
    /// A summary description of the connector.
    #[serde(rename = "ConnectorDescription")]
    pub connector_description: Option<String>,
    /// Details of encryption in transit to the Kafka cluster.
    #[serde(rename = "KafkaClusterEncryptionInTransit")]
    pub kafka_cluster_encryption_in_transit: KafkaClusterEncryptionInTransit,
    /// The name of the connector.
    #[serde(rename = "ConnectorName")]
    pub connector_name: String,
    /// Information about the capacity allocated to the connector.
    #[serde(rename = "Capacity")]
    pub capacity: Capacity,
    /// The Amazon Resource Name (ARN) of the IAM role used by the connector to access Amazon S3 objects and other external resources.
    #[serde(rename = "ServiceExecutionRoleArn")]
    pub service_execution_role_arn: String,
    /// The version of Kafka Connect. It has to be compatible with both the Kafka cluster's version and the plugins.
    #[serde(rename = "KafkaConnectVersion")]
    pub kafka_connect_version: String,
    /// Specifies the worker configuration to use with the connector.
    #[serde(rename = "WorkerConfiguration")]
    pub worker_configuration: Option<WorkerConfiguration>,
    /// Details of what logs are delivered and where they are delivered.
    #[serde(rename = "LogDelivery")]
    pub log_delivery: Option<LogDelivery>,
    /// The configuration for the connector.
    #[serde(rename = "ConnectorConfiguration")]
    pub connector_configuration: (),
    /// Details of how to connect to the Kafka cluster.
    #[serde(rename = "KafkaCluster")]
    pub kafka_cluster: KafkaCluster,
    /// List of plugins to use with the connector.
    #[serde(rename = "Plugins")]
    pub plugins: Vec<Plugin>,
    /// Details of the client authentication used by the Kafka cluster.
    #[serde(rename = "KafkaClusterClientAuthentication")]
    pub kafka_cluster_client_authentication: KafkaClusterClientAuthentication,

}


#[derive(serde::Serialize, Default)]
pub struct AutoScaling {
    #[serde(rename = "MinWorkerCount")]
    pub min_worker_count: usize,
    #[serde(rename = "ScaleOutPolicy")]
    pub scale_out_policy: ScaleOutPolicy,
    #[serde(rename = "McuCount")]
    pub mcu_count: usize,
    #[serde(rename = "ScaleInPolicy")]
    pub scale_in_policy: ScaleInPolicy,
    #[serde(rename = "MaxWorkerCount")]
    pub max_worker_count: usize,

}

#[derive(serde::Serialize, Default)]
pub struct FirehoseLogDelivery {
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct WorkerConfiguration {
    #[serde(rename = "WorkerConfigurationArn")]
    pub worker_configuration_arn: String,
    #[serde(rename = "Revision")]
    pub revision: usize,

}

#[derive(serde::Serialize, Default)]
pub struct S3LogDelivery {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,

}
pub type KafkaClusterEncryptionInTransitType = String;
#[derive(serde::Serialize, Default)]
pub struct KafkaClusterClientAuthentication {
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: KafkaClusterClientAuthenticationType,

}

#[derive(serde::Serialize, Default)]
pub struct ScaleInPolicy {
    #[serde(rename = "CpuUtilizationPercentage")]
    pub cpu_utilization_percentage: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Vpc {
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Vec<String>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct WorkerLogDelivery {
    #[serde(rename = "S3")]
    pub s3: Option<S3LogDelivery>,
    #[serde(rename = "CloudWatchLogs")]
    pub cloud_watch_logs: Option<CloudWatchLogsLogDelivery>,
    #[serde(rename = "Firehose")]
    pub firehose: Option<FirehoseLogDelivery>,

}

#[derive(serde::Serialize, Default)]
pub struct Capacity {
    #[serde(rename = "ProvisionedCapacity")]
    pub provisioned_capacity: Option<ProvisionedCapacity>,
    #[serde(rename = "AutoScaling")]
    pub auto_scaling: Option<AutoScaling>,

}

#[derive(serde::Serialize, Default)]
pub struct KafkaClusterEncryptionInTransit {
    #[serde(rename = "EncryptionType")]
    pub encryption_type: KafkaClusterEncryptionInTransitType,

}

#[derive(serde::Serialize, Default)]
pub struct CustomPlugin {
    #[serde(rename = "Revision")]
    pub revision: usize,
    #[serde(rename = "CustomPluginArn")]
    pub custom_plugin_arn: String,

}
pub type KafkaClusterClientAuthenticationType = String;
#[derive(serde::Serialize, Default)]
pub struct ScaleOutPolicy {
    #[serde(rename = "CpuUtilizationPercentage")]
    pub cpu_utilization_percentage: usize,

}

#[derive(serde::Serialize, Default)]
pub struct LogDelivery {
    #[serde(rename = "WorkerLogDelivery")]
    pub worker_log_delivery: WorkerLogDelivery,

}

#[derive(serde::Serialize, Default)]
pub struct ApacheKafkaCluster {
    #[serde(rename = "Vpc")]
    pub vpc: Vpc,
    #[serde(rename = "BootstrapServers")]
    pub bootstrap_servers: String,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchLogsLogDelivery {
    #[serde(rename = "LogGroup")]
    pub log_group: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct KafkaCluster {
    #[serde(rename = "ApacheKafkaCluster")]
    pub apache_kafka_cluster: ApacheKafkaCluster,

}

#[derive(serde::Serialize, Default)]
pub struct ProvisionedCapacity {
    #[serde(rename = "McuCount")]
    pub mcu_count: Option<usize>,
    #[serde(rename = "WorkerCount")]
    pub worker_count: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Plugin {
    #[serde(rename = "CustomPlugin")]
    pub custom_plugin: CustomPlugin,

}


}
