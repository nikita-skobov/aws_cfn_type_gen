

/// The AWS::Elasticsearch::Domain resource creates an Amazon OpenSearch Service    domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomain {


    /// 
    /// An AWS Identity and Access Management (IAM) policy document that specifies who can    access the OpenSearch Service domain and their permissions. For more information, see Configuring access policies in the Amazon OpenSearch Service Developer     Guide.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicies")]
    pub access_policies: Option<serde_json::Value>,


    /// 
    /// Additional options to specify for the OpenSearch Service domain. For more information, see Advanced cluster parameters in the Amazon OpenSearch Service     Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedOptions")]
    pub advanced_options: Option<std::collections::HashMap<String, String>>,


    /// Specifies options for fine-grained access control.
    ///
    /// Required: No
    ///
    /// Type: AdvancedSecurityOptionsInput
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AdvancedSecurityOptions")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,


    /// 
    /// Configures OpenSearch Service to use Amazon Cognito authentication for OpenSearch Dashboards.
    /// 
    /// Required: No
    ///
    /// Type: CognitoOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoOptions")]
    pub cognito_options: Option<CognitoOptions>,


    /// Specifies additional options for the domain endpoint, such as whether to require HTTPS for all traffic or whether to use a custom endpoint rather than the default endpoint.
    ///
    /// Required: No
    ///
    /// Type: DomainEndpointOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainEndpointOptions")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,


    /// 
    /// A name for the OpenSearch Service domain. For valid values, see the DomainName data type in the Amazon OpenSearch Service Developer     Guide. If you don't specify a name, AWS CloudFormation generates a unique    physical ID and uses that ID for the domain name. For more information, see Name     Type.
    /// 
    /// ImportantIf you specify a name, you cannot perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you must replace     the resource, specify a new name.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,


    /// 
    /// The configurations of Amazon Elastic Block Store (Amazon EBS) volumes that are attached to    data nodes in the OpenSearch Service domain. For more information, see EBS volume size limits in the Amazon OpenSearch Service Developer     Guide.
    /// 
    /// Required: No
    ///
    /// Type: EBSOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "EBSOptions")]
    pub ebsoptions: Option<EBSOptions>,


    /// 
    /// ElasticsearchClusterConfig is a property of the AWS::Elasticsearch::Domain resource that    configures the cluster of an Amazon OpenSearch Service domain.
    /// 
    /// Required: No
    ///
    /// Type: ElasticsearchClusterConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticsearchClusterConfig")]
    pub elasticsearch_cluster_config: Option<ElasticsearchClusterConfig>,


    /// 
    /// The version of Elasticsearch to use, such as 2.3. If not specified, 1.5 is used as the    default. For information about the versions that OpenSearch Service supports, see Supported     versions of OpenSearch and Elasticsearch in the Amazon OpenSearch Service     Developer Guide.
    /// 
    /// If you set the EnableVersionUpgrade update policy to true, you can update     ElasticsearchVersion without interruption. When     EnableVersionUpgrade is set to false, or is not specified,    updating ElasticsearchVersion results in replacement.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ElasticsearchVersion")]
    pub elasticsearch_version: Option<String>,


    /// 
    /// Whether the domain should encrypt data at rest, and if so, the AWS Key Management Service    key to use. See Encryption of data at     rest for Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionAtRestOptions
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "EncryptionAtRestOptions")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,


    /// 
    /// An object with one or more of the following keys: SEARCH_SLOW_LOGS,     ES_APPLICATION_LOGS, INDEX_SLOW_LOGS, AUDIT_LOGS,    depending on the types of logs you want to publish. Each key needs a valid     LogPublishingOption value.
    /// 
    /// Required: No
    ///
    /// Type: Map of LogPublishingOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogPublishingOptions")]
    pub log_publishing_options: Option<std::collections::HashMap<String, LogPublishingOption>>,


    /// 
    /// Specifies whether node-to-node encryption is enabled. See Node-to-node encryption for Amazon     OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: NodeToNodeEncryptionOptions
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,


    /// 
    /// DEPRECATED. The automated snapshot configuration for the    OpenSearch Service domain indices.
    /// 
    /// Required: No
    ///
    /// Type: SnapshotOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotOptions")]
    pub snapshot_options: Option<SnapshotOptions>,


    /// 
    /// An arbitrary set of tags (key–value pairs) to associate with the OpenSearch Service domain.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The virtual private cloud (VPC) configuration for the OpenSearch Service domain. For more    information, see Launching your Amazon OpenSearch     Service domains within a VPC in the Amazon OpenSearch Service Developer     Guide.
    /// 
    /// Required: No
    ///
    /// Type: VPCOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "VPCOptions")]
    pub vpcoptions: Option<VPCOptions>,

}



impl cfn_resources::CfnResource for CfnDomain {
    fn type_string() -> &'static str {
        "AWS::Elasticsearch::Domain"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.advanced_security_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.cognito_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.domain_endpoint_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.ebsoptions.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.elasticsearch_cluster_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.encryption_at_rest_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.node_to_node_encryption_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.snapshot_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.vpcoptions.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies options for fine-grained access control.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdvancedSecurityOptionsInput {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnonymousAuthEnabled")]
    pub anonymous_auth_enabled: Option<bool>,


    /// True to enable fine-grained access control. You must also enable encryption of data at rest    and node-to-node encryption.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// True to enable the internal user database.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "InternalUserDatabaseEnabled")]
    pub internal_user_database_enabled: Option<bool>,


    /// Specifies information about the master user.
    ///
    /// Required: No
    ///
    /// Type: MasterUserOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserOptions")]
    pub master_user_options: Option<MasterUserOptions>,

}



impl cfn_resources::CfnResource for AdvancedSecurityOptionsInput {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.master_user_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Configures OpenSearch Service to use Amazon Cognito authentication for OpenSearch Dashboards.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CognitoOptions {


    /// 
    /// Whether to enable or disable Amazon Cognito authentication for OpenSearch Dashboards. See Amazon     Cognito authentication for OpenSearch Dashboards.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The Amazon Cognito identity pool ID that you want OpenSearch Service to use for OpenSearch    Dashboards authentication. Required if you enable Cognito authentication.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: Option<String>,


    /// 
    /// The AmazonESCognitoAccess role that allows OpenSearch Service to configure    your user pool and identity pool. Required if you enable Cognito authentication.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The Amazon Cognito user pool ID that you want OpenSearch Service to use for OpenSearch    Dashboards authentication. Required if you enable Cognito authentication.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: Option<String>,

}



impl cfn_resources::CfnResource for CognitoOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies options for cold storage. For more information, see Cold storage for Amazon     Elasticsearch Service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColdStorageOptions {


    /// 
    /// Whether to enable or disable cold storage on the domain. You must enable UltraWarm storage in order to enable cold storage.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}



impl cfn_resources::CfnResource for ColdStorageOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies additional options for the domain endpoint, such as whether to require HTTPS for all traffic or whether to use a custom endpoint rather than the default endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DomainEndpointOptions {


    /// The fully qualified URL for your custom endpoint. Required if you enabled a custom endpoint    for the domain.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpoint")]
    pub custom_endpoint: Option<String>,


    /// The AWS Certificate Manager ARN for your domain's SSL/TLS certificate. Required if you    enabled a custom endpoint for the domain.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpointCertificateArn")]
    pub custom_endpoint_certificate_arn: Option<String>,


    /// True to enable a custom endpoint for the domain. If enabled, you must also provide values for CustomEndpoint and CustomEndpointCertificateArn.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpointEnabled")]
    pub custom_endpoint_enabled: Option<bool>,


    /// True to require that all traffic to the domain arrive over HTTPS.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnforceHTTPS")]
    pub enforce_https: Option<bool>,


    /// The minimum TLS version required for traffic to the domain. Valid values are TLS 1.0 (default) or 1.2:
    /// 
    /// Policy-Min-TLS-1-0-2019-07Policy-Min-TLS-1-2-2019-07
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TLSSecurityPolicy")]
    pub tlssecurity_policy: Option<String>,

}



impl cfn_resources::CfnResource for DomainEndpointOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The configurations of Amazon Elastic Block Store (Amazon EBS) volumes that are attached to    data nodes in the OpenSearch Service domain. For more information, see EBS volume size limits in the Amazon OpenSearch Service Developer     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EBSOptions {


    /// 
    /// Specifies whether Amazon EBS volumes are attached to data nodes in the OpenSearch Service    domain.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EBSEnabled")]
    pub ebsenabled: Option<bool>,


    /// 
    /// The number of I/O operations per second (IOPS) that the volume supports. This property    applies only to provisioned IOPS EBS volume types.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,


    /// 
    /// The size (in GiB) of the EBS volume for each data node. The minimum and maximum size of an    EBS volume depends on the EBS volume type and the instance type to which it is attached. For    more information, see EBS volume size     limits in the Amazon OpenSearch Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<i64>,


    /// 
    /// The EBS volume type to use with the OpenSearch Service domain, such as standard, gp2, or    io1. For more information about each type, see Amazon EBS volume types in the     Amazon EC2 User Guide for Linux Instances.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,

}



impl cfn_resources::CfnResource for EBSOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The cluster configuration for the OpenSearch Service domain. You can specify options such    as the instance type and the number of instances. For more information, see Creating and managing Amazon OpenSearch Service domains in the Amazon     OpenSearch Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticsearchClusterConfig {


    /// 
    /// Specifies cold storage options for the domain.
    ///
    /// Required: No
    ///
    /// Type: ColdStorageOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColdStorageOptions")]
    pub cold_storage_options: Option<ColdStorageOptions>,


    /// 
    /// The number of instances to use for the master node. If you specify this property, you must    specify true for the DedicatedMasterEnabled property.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DedicatedMasterCount")]
    pub dedicated_master_count: Option<i64>,


    /// 
    /// Indicates whether to use a dedicated master node for the OpenSearch Service domain. A    dedicated master node is a cluster node that performs cluster management tasks, but doesn't    hold data or respond to data upload requests. Dedicated master nodes offload cluster    management tasks to increase the stability of your search clusters. See Dedicated master nodes in Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DedicatedMasterEnabled")]
    pub dedicated_master_enabled: Option<bool>,


    /// 
    /// The hardware configuration of the computer that hosts the dedicated master node, such as     m3.medium.elasticsearch. If you specify this property, you must specify true    for the DedicatedMasterEnabled property. For valid values, see Supported     instance types in Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DedicatedMasterType")]
    pub dedicated_master_type: Option<String>,


    /// 
    /// The number of data nodes (instances) to use in the OpenSearch Service domain.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    pub instance_count: Option<i64>,


    /// 
    /// The instance type for your data nodes, such as m3.medium.elasticsearch. For    valid values, see Supported     instance types in Amazon OpenSearch Service .
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,


    /// 
    /// The number of warm nodes in the cluster. Required if you enable warm storage.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmCount")]
    pub warm_count: Option<i64>,


    /// 
    /// Whether to enable warm storage for the cluster.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmEnabled")]
    pub warm_enabled: Option<bool>,


    /// 
    /// The instance type for the cluster's warm nodes. Required if you enable warm    storage.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmType")]
    pub warm_type: Option<String>,


    /// 
    /// Specifies zone awareness configuration options. Only use if     ZoneAwarenessEnabled is true.
    /// 
    /// Required: Conditional
    ///
    /// Type: ZoneAwarenessConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZoneAwarenessConfig")]
    pub zone_awareness_config: Option<ZoneAwarenessConfig>,


    /// 
    /// Indicates whether to enable zone awareness for the OpenSearch Service domain. When you    enable zone awareness, OpenSearch Service allocates the nodes and replica index shards that    belong to a cluster across two Availability Zones (AZs) in the same region to prevent data    loss and minimize downtime in the event of node or data center failure. Don't enable zone    awareness if your cluster has no replica index shards or is a single-node cluster. For more    information, see Configuring a     multi-AZ domain in Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZoneAwarenessEnabled")]
    pub zone_awareness_enabled: Option<bool>,

}



impl cfn_resources::CfnResource for ElasticsearchClusterConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.cold_storage_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.zone_awareness_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Whether the domain should encrypt data at rest, and if so, the AWS Key Management Service key to use.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionAtRestOptions {


    /// 
    /// Specify true to enable encryption at rest.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The KMS key ID. Takes the form 1a2a3a4-1a2a-3a4a-5a6a-1a2a3a4a5a6a. Required    if you enable encryption at rest.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}



impl cfn_resources::CfnResource for EncryptionAtRestOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies whether the OpenSearch Service domain publishes the Elasticsearch application,    search slow logs, or index slow logs to Amazon CloudWatch. Each option must be an object of    name SEARCH_SLOW_LOGS, ES_APPLICATION_LOGS,     INDEX_SLOW_LOGS, or AUDIT_LOGS depending on the type of logs you    want to publish.
///
/// If you enable a slow log, you still have to enable the collection of    slow logs using the Configuration API. To learn more, see Enabling log publishing (AWS CLI).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogPublishingOption {


    /// 
    /// Specifies the CloudWatch log group to publish to. Required if you enable log publishing    for the domain.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: Option<String>,


    /// 
    /// If true, enables the publishing of logs to CloudWatch.
    /// 
    /// Default: false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}



impl cfn_resources::CfnResource for LogPublishingOption {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies information about the master user. Required if you enabled the internal user    database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MasterUserOptions {


    /// ARN for the master user. Only specify if InternalUserDatabaseEnabled is false in AdvancedSecurityOptions.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserARN")]
    pub master_user_arn: Option<String>,


    /// Username for the master user. Only specify if InternalUserDatabaseEnabled is true in AdvancedSecurityOptions.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserName")]
    pub master_user_name: Option<String>,


    /// Password for the master user. Only specify if InternalUserDatabaseEnabled is true in AdvancedSecurityOptions.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,

}



impl cfn_resources::CfnResource for MasterUserOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies whether node-to-node encryption is enabled.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NodeToNodeEncryptionOptions {


    /// 
    /// Specifies whether node-to-node encryption is enabled, as a Boolean.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}



impl cfn_resources::CfnResource for NodeToNodeEncryptionOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// DEPRECATED. For domains running Elasticsearch 5.3 and    later, OpenSearch Service takes hourly automated snapshots, making this setting irrelevant. For domains    running earlier versions of Elasticsearch, OpenSearch Service takes daily automated snapshots.
///
/// The automated snapshot configuration for the OpenSearch Service domain indices.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SnapshotOptions {


    /// 
    /// The hour in UTC during which the service takes an automated daily snapshot of the indices    in the OpenSearch Service domain. For example, if you specify 0, OpenSearch Service takes an automated snapshot    everyday between midnight and 1 am. You can specify a value between 0 and 23.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomatedSnapshotStartHour")]
    pub automated_snapshot_start_hour: Option<i64>,

}



impl cfn_resources::CfnResource for SnapshotOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
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



impl cfn_resources::CfnResource for Tag {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The virtual private cloud (VPC) configuration for the OpenSearch Service domain. For more    information, see Launching your Amazon OpenSearch     Service domains using a VPC in the Amazon OpenSearch Service Developer     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VPCOptions {


    /// 
    /// The list of security group IDs that are associated with the VPC endpoints for the domain.    If you don't provide a security group ID, OpenSearch Service uses the default security group    for the VPC. To learn more, see Security groups for your VPC in    the Amazon VPC User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// Provide one subnet ID for each Availability Zone that your domain uses. For example, you    must specify three subnet IDs for a three Availability Zone domain. To learn more, see VPCs and subnets in    the Amazon VPC User Guide.
    /// 
    /// Required if you're creating your domain inside a VPC.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for VPCOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies zone awareness configuration options. Only use if     ZoneAwarenessEnabled is true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ZoneAwarenessConfig {


    /// 
    /// If you enabled multiple Availability Zones (AZs), the number of AZs that you want the    domain to use.
    /// 
    /// Valid values are 2 and 3. Default is 2.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZoneCount")]
    pub availability_zone_count: Option<i64>,

}



impl cfn_resources::CfnResource for ZoneAwarenessConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}