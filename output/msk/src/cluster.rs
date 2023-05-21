/// Creates a new MSK cluster. The following Python 3.6 examples shows how you can create a cluster that's distributed over two Availability Zones.         Before you run this Python script, replace the example subnet and security-group IDs with the IDs of your subnets and security group. When you create an MSK cluster, its brokers get evenly distributed over a number of Availability Zones that's equal to the number of subnets that you specify in the BrokerNodeGroupInfo parameter. In this example, you can add a third subnet to get a cluster that's distributed over three Availability Zones.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCluster {
    ///
    /// Information about the broker nodes in the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: BrokerNodeGroupInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "BrokerNodeGroupInfo")]
    pub broker_node_group_info: BrokerNodeGroupInfo,

    ///
    /// Includes all client authentication related information.
    ///
    /// Required: No
    ///
    /// Type: ClientAuthentication
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientAuthentication")]
    pub client_authentication: Option<ClientAuthentication>,

    ///
    /// The name of the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,

    ///
    /// Represents the configuration that you want MSK to use for the cluster.
    ///
    /// Required: No
    ///
    /// Type: ConfigurationInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationInfo")]
    pub configuration_info: Option<ConfigurationInfo>,

    ///
    /// The version of the cluster that you want to update.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrentVersion")]
    pub current_version: Option<String>,

    ///
    /// Includes all encryption-related information.
    ///
    /// Required: No
    ///
    /// Type: EncryptionInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionInfo")]
    pub encryption_info: Option<EncryptionInfo>,

    ///
    /// Specifies the level of monitoring for the MSK cluster. The possible values are DEFAULT, PER_BROKER, and PER_TOPIC_PER_BROKER.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnhancedMonitoring")]
    pub enhanced_monitoring: Option<String>,

    ///
    /// The version of Apache Kafka. You can use Amazon MSK to create clusters that use Apache Kafka versions 1.1.1 and 2.2.1.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KafkaVersion")]
    pub kafka_version: String,

    ///
    /// Logging Info details.
    ///
    /// Required: No
    ///
    /// Type: LoggingInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingInfo")]
    pub logging_info: Option<LoggingInfo>,

    ///
    /// The number of broker nodes in the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfBrokerNodes")]
    pub number_of_broker_nodes: i64,

    ///
    /// The settings for open monitoring.
    ///
    /// Required: No
    ///
    /// Type: OpenMonitoring
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenMonitoring")]
    pub open_monitoring: Option<OpenMonitoring>,

    ///
    /// This controls storage mode for supported storage tiers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageMode")]
    pub storage_mode: Option<String>,

    ///
    /// Create tags when creating the cluster.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for CfnCluster {
    fn type_string(&self) -> &'static str {
        "AWS::MSK::Cluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.broker_node_group_info.validate()?;

        self.client_authentication
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.configuration_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.encryption_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.logging_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.open_monitoring
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The broker logs configuration for this MSK cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BrokerLogs {
    ///
    /// Details of the CloudWatch Logs destination for broker logs.
    ///
    /// Required: No
    ///
    /// Type: CloudWatchLogs
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogs")]
    pub cloud_watch_logs: Option<CloudWatchLogs>,

    ///
    /// Details of the Kinesis Data Firehose delivery stream that is the destination for broker logs.
    ///
    /// Required: No
    ///
    /// Type: Firehose
    ///
    /// Update requires: No interruption
    #[serde(rename = "Firehose")]
    pub firehose: Option<Firehose>,

    ///
    /// Details of the Amazon S3 destination for broker logs.
    ///
    /// Required: No
    ///
    /// Type: S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3>,
}

impl cfn_resources::CfnResource for BrokerLogs {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_watch_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.firehose
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the setup to be used for the broker nodes in the cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BrokerNodeGroupInfo {
    ///
    /// This parameter is currently not in use.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BrokerAZDistribution")]
    pub broker_azdistribution: Option<String>,

    ///
    /// The list of subnets to connect to in the client virtual private cloud (VPC). Amazon creates elastic network interfaces inside these subnets.         Client applications use elastic network interfaces to produce and consume data.
    ///
    /// If you use the US West (N. California) Region, specify exactly two subnets. For other Regions where        Amazon MSK is available, you can specify either two or three subnets.        The subnets that you specify must be in distinct Availability Zones.        When you create a cluster, Amazon MSK distributes the broker nodes        evenly across the subnets that you specify.
    ///
    /// Client subnets can't occupy the Availability Zone with ID use1-az3.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientSubnets")]
    pub client_subnets: Vec<String>,

    ///
    /// Information about the cluster's connectivity setting.
    ///
    /// Required: No
    ///
    /// Type: ConnectivityInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectivityInfo")]
    pub connectivity_info: Option<ConnectivityInfo>,

    ///
    /// The type of Amazon EC2 instances to use for brokers. The following instance types are allowed: kafka.m5.large, kafka.m5.xlarge, kafka.m5.2xlarge,           kafka.m5.4xlarge, kafka.m5.8xlarge, kafka.m5.12xlarge, kafka.m5.16xlarge, and kafka.m5.24xlarge, and kafka.t3.small.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

    ///
    /// The security groups to associate with the elastic network interfaces in order to specify who can connect to and communicate with the Amazon MSK cluster. If you don't specify a security group, Amazon MSK uses the default security group associated with the VPC. If you specify security groups that were shared with you, you must ensure that you have permissions to them. Specifically, you need the ec2:DescribeSecurityGroups permission.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// Contains information about storage volumes attached to Amazon MSK broker nodes.
    ///
    /// Required: No
    ///
    /// Type: StorageInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageInfo")]
    pub storage_info: Option<StorageInfo>,
}

impl cfn_resources::CfnResource for BrokerNodeGroupInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.connectivity_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.storage_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Includes all client authentication information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientAuthentication {
    ///
    /// Details for client authentication using SASL. To turn on SASL, you must also turn on EncryptionInTransit by setting inCluster to true. You must set clientBroker to either TLS or TLS_PLAINTEXT. If you choose TLS_PLAINTEXT, then you must also set unauthenticated to true.
    ///
    /// Required: No
    ///
    /// Type: Sasl
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sasl")]
    pub sasl: Option<Sasl>,

    ///
    /// Details for ClientAuthentication using TLS. To turn on TLS access control, you must also turn on EncryptionInTransit by setting inCluster to true and clientBroker to TLS.
    ///
    /// Required: No
    ///
    /// Type: Tls
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tls")]
    pub tls: Option<Tls>,

    ///
    /// Details for ClientAuthentication using no authentication.
    ///
    /// Required: No
    ///
    /// Type: Unauthenticated
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unauthenticated")]
    pub unauthenticated: Option<Unauthenticated>,
}

impl cfn_resources::CfnResource for ClientAuthentication {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.sasl.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.tls.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.unauthenticated
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Details of the CloudWatch Logs destination for broker logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchLogs {
    ///
    /// Specifies whether broker logs get sent to the specified CloudWatch Logs destination.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// The CloudWatch log group that is the destination for broker logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroup")]
    pub log_group: Option<String>,
}

impl cfn_resources::CfnResource for CloudWatchLogs {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the configuration to use for the brokers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfigurationInfo {
    ///
    /// ARN of the configuration to use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: String,

    ///
    /// The revision of the configuration to use.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Revision")]
    pub revision: i64,
}

impl cfn_resources::CfnResource for ConfigurationInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Broker access controls.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectivityInfo {
    ///
    /// Access control settings for the cluster's brokers.
    ///
    /// Required: No
    ///
    /// Type: PublicAccess
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicAccess")]
    pub public_access: Option<PublicAccess>,

    ///
    /// VPC connection control settings for brokers
    ///
    /// Required: No
    ///
    /// Type: VpcConnectivity
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConnectivity")]
    pub vpc_connectivity: Option<VpcConnectivity>,
}

impl cfn_resources::CfnResource for ConnectivityInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.public_access
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc_connectivity
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about the EBS storage volumes attached to the broker nodes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EBSStorageInfo {
    ///
    /// EBS volume provisioned throughput information.
    ///
    /// Required: No
    ///
    /// Type: ProvisionedThroughput
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,

    ///
    /// The size in GiB of the EBS volume for the data drive on each broker node.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<i64>,
}

impl cfn_resources::CfnResource for EBSStorageInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.provisioned_throughput
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The data-volume encryption details. You can't update encryption at rest settings for existing clusters.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionAtRest {
    ///
    /// The ARN of the Amazon KMS key for encrypting data at rest. If you don't specify a KMS key, MSK creates one for you and uses it.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataVolumeKMSKeyId")]
    pub data_volume_kmskey_id: String,
}

impl cfn_resources::CfnResource for EncryptionAtRest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The settings for encrypting data in transit.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionInTransit {
    ///
    /// Indicates the encryption setting for data in transit between clients and brokers. You must set it to one of the following values.
    ///
    /// TLS means that client-broker communication is enabled with TLS only.
    ///
    /// TLS_PLAINTEXT means that client-broker communication is enabled for both TLS-encrypted, as well as plaintext data.
    ///
    /// PLAINTEXT means that client-broker communication is enabled in plaintext only.
    ///
    /// The default value is TLS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientBroker")]
    pub client_broker: Option<String>,

    ///
    /// When set to true, it indicates that data communication among the broker nodes of the cluster is encrypted. When set to false, the communication happens in plaintext.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "InCluster")]
    pub in_cluster: Option<bool>,
}

impl cfn_resources::CfnResource for EncryptionInTransit {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Includes encryption-related information, such as the Amazon KMS key used for encrypting data at rest and whether you want MSK to encrypt your data in transit.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionInfo {
    ///
    /// The data-volume encryption details.
    ///
    /// Required: No
    ///
    /// Type: EncryptionAtRest
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionAtRest")]
    pub encryption_at_rest: Option<EncryptionAtRest>,

    ///
    /// The details for encryption in transit.
    ///
    /// Required: No
    ///
    /// Type: EncryptionInTransit
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionInTransit")]
    pub encryption_in_transit: Option<EncryptionInTransit>,
}

impl cfn_resources::CfnResource for EncryptionInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption_at_rest
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.encryption_in_transit
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Firehose details for BrokerLogs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Firehose {
    ///
    /// The Kinesis Data Firehose delivery stream that is the destination for broker logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: Option<String>,

    ///
    /// Specifies whether broker logs get send to the specified Kinesis Data Firehose delivery stream.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for Firehose {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Details for SASL/IAM client authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Iam {
    ///
    /// SASL/IAM authentication is enabled or not.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for Iam {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Indicates whether you want to enable or disable the JMX Exporter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JmxExporter {
    ///
    /// Indicates whether you want to enable or disable the JMX Exporter.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnabledInBroker")]
    pub enabled_in_broker: bool,
}

impl cfn_resources::CfnResource for JmxExporter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// You can configure your MSK cluster to send broker logs to different destination types. This is a container for the configuration details related to broker logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingInfo {
    ///
    /// You can configure your MSK cluster to send broker logs to different destination types. This configuration specifies the details of these destinations.
    ///
    /// Required: Yes
    ///
    /// Type: BrokerLogs
    ///
    /// Update requires: No interruption
    #[serde(rename = "BrokerLogs")]
    pub broker_logs: BrokerLogs,
}

impl cfn_resources::CfnResource for LoggingInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.broker_logs.validate()?;

        Ok(())
    }
}

/// Indicates whether you want to enable or disable the Node Exporter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NodeExporter {
    ///
    /// Indicates whether you want to enable or disable the Node Exporter.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnabledInBroker")]
    pub enabled_in_broker: bool,
}

impl cfn_resources::CfnResource for NodeExporter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// JMX and Node monitoring for the MSK cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OpenMonitoring {
    ///
    /// Prometheus exporter settings.
    ///
    /// Required: Yes
    ///
    /// Type: Prometheus
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prometheus")]
    pub prometheus: Prometheus,
}

impl cfn_resources::CfnResource for OpenMonitoring {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.prometheus.validate()?;

        Ok(())
    }
}

/// Prometheus settings for open monitoring.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Prometheus {
    ///
    /// Indicates whether you want to enable or disable the JMX Exporter.
    ///
    /// Required: No
    ///
    /// Type: JmxExporter
    ///
    /// Update requires: No interruption
    #[serde(rename = "JmxExporter")]
    pub jmx_exporter: Option<JmxExporter>,

    ///
    /// Indicates whether you want to enable or disable the Node Exporter.
    ///
    /// Required: No
    ///
    /// Type: NodeExporter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeExporter")]
    pub node_exporter: Option<NodeExporter>,
}

impl cfn_resources::CfnResource for Prometheus {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.jmx_exporter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.node_exporter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about provisioned throughput for EBS storage volumes attached to kafka broker nodes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProvisionedThroughput {
    ///
    /// Provisioned throughput is enabled or not.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

    ///
    /// Throughput value of the EBS volumes for the data drive on each kafka broker node in MiB per second.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeThroughput")]
    pub volume_throughput: Option<i64>,
}

impl cfn_resources::CfnResource for ProvisionedThroughput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Broker access controls
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PublicAccess {
    ///
    /// DISABLED means that public access is turned off. SERVICE_PROVIDED_EIPS means that public access is turned on.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,
}

impl cfn_resources::CfnResource for PublicAccess {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The details of the Amazon S3 destination for broker logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3 {
    ///
    /// The name of the S3 bucket that is the destination for broker logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,

    ///
    /// Specifies whether broker logs get sent to the specified Amazon S3 destination.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// The S3 prefix that is the destination for broker logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
}

impl cfn_resources::CfnResource for S3 {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Details for client authentication using SASL. To turn on SASL, you must also turn on EncryptionInTransit by setting inCluster to true. You must set clientBroker to either TLS or TLS_PLAINTEXT. If you choose TLS_PLAINTEXT, then you must also set unauthenticated to true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Sasl {
    ///
    /// Details for ClientAuthentication using IAM.
    ///
    /// Required: No
    ///
    /// Type: Iam
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iam")]
    pub iam: Option<Iam>,

    ///
    /// Details for SASL/SCRAM client authentication.
    ///
    /// Required: No
    ///
    /// Type: Scram
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scram")]
    pub scram: Option<Scram>,
}

impl cfn_resources::CfnResource for Sasl {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.iam.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.scram.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Details for SASL/SCRAM client authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scram {
    ///
    /// SASL/SCRAM authentication is enabled or not.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for Scram {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Contains information about storage volumes attached to Amazon MSK broker nodes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StorageInfo {
    ///
    /// EBS volume information.
    ///
    /// Required: No
    ///
    /// Type: EBSStorageInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "EBSStorageInfo")]
    pub ebsstorage_info: Option<EBSStorageInfo>,
}

impl cfn_resources::CfnResource for StorageInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.ebsstorage_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Details for client authentication using TLS.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tls {
    ///
    /// List of AWS Private CA ARNs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateAuthorityArnList")]
    pub certificate_authority_arn_list: Option<Vec<String>>,

    ///
    /// TLS authentication is enabled or not.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl cfn_resources::CfnResource for Tls {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Details for allowing no client authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Unauthenticated {
    ///
    /// Unauthenticated is enabled or not.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for Unauthenticated {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// VPC connection control settings for brokers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConnectivity {
    ///
    /// VPC connection control settings for brokers.
    ///
    /// Required: No
    ///
    /// Type: VpcConnectivityClientAuthentication
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientAuthentication")]
    pub client_authentication: Option<VpcConnectivityClientAuthentication>,
}

impl cfn_resources::CfnResource for VpcConnectivity {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.client_authentication
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Includes all client authentication information for VpcConnectivity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConnectivityClientAuthentication {
    ///
    /// Details for VpcConnectivity ClientAuthentication using SASL.
    ///
    /// Required: No
    ///
    /// Type: VpcConnectivitySasl
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sasl")]
    pub sasl: Option<VpcConnectivitySasl>,

    ///
    /// Details for VpcConnectivity ClientAuthentication using TLS.
    ///
    /// Required: No
    ///
    /// Type: VpcConnectivityTls
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tls")]
    pub tls: Option<VpcConnectivityTls>,
}

impl cfn_resources::CfnResource for VpcConnectivityClientAuthentication {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.sasl.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.tls.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Details for SASL/IAM client authentication for VpcConnectivity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConnectivityIam {
    ///
    /// SASL/IAM authentication is enabled or not.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for VpcConnectivityIam {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Details for client authentication using SASL for VpcConnectivity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConnectivitySasl {
    ///
    /// Details for ClientAuthentication using IAM for VpcConnectivity.
    ///
    /// Required: No
    ///
    /// Type: VpcConnectivityIam
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iam")]
    pub iam: Option<VpcConnectivityIam>,

    ///
    /// Details for SASL/SCRAM client authentication for VpcConnectivity.
    ///
    /// Required: No
    ///
    /// Type: VpcConnectivityScram
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scram")]
    pub scram: Option<VpcConnectivityScram>,
}

impl cfn_resources::CfnResource for VpcConnectivitySasl {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.iam.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.scram.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Details for SASL/SCRAM client authentication for vpcConnectivity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConnectivityScram {
    ///
    /// SASL/SCRAM authentication is enabled or not.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for VpcConnectivityScram {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Details for client authentication using TLS for vpcConnectivity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConnectivityTls {
    ///
    /// TLS authentication is enabled or not.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for VpcConnectivityTls {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
