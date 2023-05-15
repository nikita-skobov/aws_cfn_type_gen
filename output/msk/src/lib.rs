
pub mod cfn_batch_scram_secret {

#[derive(serde::Serialize, Default)]
pub struct CfnBatchScramSecret {
    /// No documentation provided by AWS
    #[serde(rename = "SecretArnList")]
    pub secret_arn_list: Option<SecretArnList>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,

}

pub type SecretArnList = Vec<String>;

}

pub mod cfn_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCluster {
    /// No documentation provided by AWS
    #[serde(rename = "BrokerNodeGroupInfo")]
    pub broker_node_group_info: BrokerNodeGroupInfo,
    /// No documentation provided by AWS
    #[serde(rename = "KafkaVersion")]
    pub kafka_version: String,
    /// No documentation provided by AWS
    #[serde(rename = "NumberOfBrokerNodes")]
    pub number_of_broker_nodes: usize,
    /// No documentation provided by AWS
    #[serde(rename = "ClientAuthentication")]
    pub client_authentication: Option<ClientAuthentication>,
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationInfo")]
    pub configuration_info: Option<ConfigurationInfo>,
    /// No documentation provided by AWS
    #[serde(rename = "StorageMode")]
    pub storage_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LoggingInfo")]
    pub logging_info: Option<LoggingInfo>,
    /// No documentation provided by AWS
    #[serde(rename = "EnhancedMonitoring")]
    pub enhanced_monitoring: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OpenMonitoring")]
    pub open_monitoring: Option<OpenMonitoring>,
    /// The current version of the MSK cluster
    #[serde(rename = "CurrentVersion")]
    pub current_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EncryptionInfo")]
    pub encryption_info: Option<EncryptionInfo>,

}


#[derive(serde::Serialize, Default)]
pub struct ProvisionedThroughput {
    #[serde(rename = "VolumeThroughput")]
    pub volume_throughput: Option<usize>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct BrokerNodeGroupInfo {
    #[serde(rename = "BrokerAZDistribution")]
    pub broker_azdistribution: Option<String>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "ClientSubnets")]
    pub client_subnets: Vec<String>,
    #[serde(rename = "ConnectivityInfo")]
    pub connectivity_info: Option<ConnectivityInfo>,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "StorageInfo")]
    pub storage_info: Option<StorageInfo>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionInTransit {
    #[serde(rename = "InCluster")]
    pub in_cluster: Option<bool>,
    #[serde(rename = "ClientBroker")]
    pub client_broker: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigurationInfo {
    #[serde(rename = "Revision")]
    pub revision: usize,
    #[serde(rename = "Arn")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Prometheus {
    #[serde(rename = "JmxExporter")]
    pub jmx_exporter: Option<JmxExporter>,
    #[serde(rename = "NodeExporter")]
    pub node_exporter: Option<NodeExporter>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionAtRest {
    #[serde(rename = "DataVolumeKMSKeyId")]
    pub data_volume_kmskey_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct StorageInfo {
    #[serde(rename = "EBSStorageInfo")]
    pub ebsstorage_info: Option<EBSStorageInfo>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConnectivitySasl {
    #[serde(rename = "Scram")]
    pub scram: Option<VpcConnectivityScram>,
    #[serde(rename = "Iam")]
    pub iam: Option<VpcConnectivityIam>,

}

#[derive(serde::Serialize, Default)]
pub struct Iam {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Firehose {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct JmxExporter {
    #[serde(rename = "EnabledInBroker")]
    pub enabled_in_broker: bool,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionInfo {
    #[serde(rename = "EncryptionAtRest")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
    #[serde(rename = "EncryptionInTransit")]
    pub encryption_in_transit: Option<EncryptionInTransit>,

}

#[derive(serde::Serialize, Default)]
pub struct S3 {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConnectivity {
    #[serde(rename = "ClientAuthentication")]
    pub client_authentication: Option<VpcConnectivityClientAuthentication>,

}

#[derive(serde::Serialize, Default)]
pub struct PublicAccess {
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Scram {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Tls {
    #[serde(rename = "CertificateAuthorityArnList")]
    pub certificate_authority_arn_list: Option<Vec<String>>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ClientAuthentication {
    #[serde(rename = "Unauthenticated")]
    pub unauthenticated: Option<Unauthenticated>,
    #[serde(rename = "Tls")]
    pub tls: Option<Tls>,
    #[serde(rename = "Sasl")]
    pub sasl: Option<Sasl>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConnectivityClientAuthentication {
    #[serde(rename = "Sasl")]
    pub sasl: Option<VpcConnectivitySasl>,
    #[serde(rename = "Tls")]
    pub tls: Option<VpcConnectivityTls>,

}

#[derive(serde::Serialize, Default)]
pub struct EBSStorageInfo {
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConnectivityTls {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingInfo {
    #[serde(rename = "BrokerLogs")]
    pub broker_logs: BrokerLogs,

}

#[derive(serde::Serialize, Default)]
pub struct BrokerLogs {
    #[serde(rename = "S3")]
    pub s3: Option<S3>,
    #[serde(rename = "Firehose")]
    pub firehose: Option<Firehose>,
    #[serde(rename = "CloudWatchLogs")]
    pub cloud_watch_logs: Option<CloudWatchLogs>,

}

#[derive(serde::Serialize, Default)]
pub struct Sasl {
    #[serde(rename = "Scram")]
    pub scram: Option<Scram>,
    #[serde(rename = "Iam")]
    pub iam: Option<Iam>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConnectivityScram {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchLogs {
    #[serde(rename = "LogGroup")]
    pub log_group: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Unauthenticated {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct OpenMonitoring {
    #[serde(rename = "Prometheus")]
    pub prometheus: Prometheus,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectivityInfo {
    #[serde(rename = "VpcConnectivity")]
    pub vpc_connectivity: Option<VpcConnectivity>,
    #[serde(rename = "PublicAccess")]
    pub public_access: Option<PublicAccess>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConnectivityIam {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct NodeExporter {
    #[serde(rename = "EnabledInBroker")]
    pub enabled_in_broker: bool,

}


}

pub mod cfn_cluster_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnClusterPolicy {
    /// The arn of the cluster for the resource policy.
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,
    /// A policy document containing permissions to add to the specified cluster.
    #[serde(rename = "Policy")]
    pub policy: (),

}



}

pub mod cfn_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnConfiguration {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "KafkaVersionsList")]
    pub kafka_versions_list: Option<KafkaVersionsList>,
    /// No documentation provided by AWS
    #[serde(rename = "ServerProperties")]
    pub server_properties: String,

}

pub type KafkaVersionsList = Vec<String>;

}

pub mod cfn_serverless_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnServerlessCluster {
    /// No documentation provided by AWS
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ClientAuthentication")]
    pub client_authentication: ClientAuthentication,
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// List of VpcConfig
    #[serde(rename = "VpcConfigs")]
    pub vpc_configs: Vec<VpcConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClientAuthentication {
    #[serde(rename = "Sasl")]
    pub sasl: Sasl,

}

#[derive(serde::Serialize, Default)]
pub struct Iam {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Sasl {
    #[serde(rename = "Iam")]
    pub iam: Iam,

}


}

pub mod cfn_vpc_connection {

#[derive(serde::Serialize, Default)]
pub struct CfnVpcConnection {
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: VpcId,
    /// The Amazon Resource Name (ARN) of the target cluster
    #[serde(rename = "TargetClusterArn")]
    pub target_cluster_arn: String,
    /// The type of private link authentication
    #[serde(rename = "Authentication")]
    pub authentication: Authentication,
    /// No documentation provided by AWS
    #[serde(rename = "ClientSubnets")]
    pub client_subnets: ClientSubnets,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroups")]
    pub security_groups: SecurityGroups,
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,

}

pub type ClientSubnets = Vec<String>;pub type Authentication = String;pub type SecurityGroups = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct Tags {

}
pub type VpcId = String;

}
