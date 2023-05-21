

/// The AWS::OpenSearchService::Domain resource creates an Amazon OpenSearch Service    domain.
#[derive(Default, serde::Serialize)]
pub struct CfnDomain {


    /// 
    /// DEPRECATED. The automated snapshot configuration for the    OpenSearch Service domain indexes.
    /// 
    /// Required: No
    ///
    /// Type: SnapshotOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotOptions")]
    pub snapshot_options: Option<SnapshotOptions>,


    /// Specifies additional options for the domain endpoint, such as whether to require HTTPS for all traffic or whether to use a custom endpoint rather than the default endpoint.
    /// 
    /// Required: No
    ///
    /// Type: DomainEndpointOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainEndpointOptions")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,


    /// Specifies options for fine-grained access control and SAML authentication.
    /// 
    /// If you specify advanced security options, you must also enable node-to-node encryption (NodeToNodeEncryptionOptions)    and encryption at rest (EncryptionAtRestOptions).    You must also enable EnforceHTTPS within DomainEndpointOptions,    which requires HTTPS for all traffic to the domain.
    /// 
    /// Required: No
    ///
    /// Type: AdvancedSecurityOptionsInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedSecurityOptions")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,


    /// 
    /// A name for the OpenSearch Service domain. The name must have a minimum length of 3 and a maximum length of 28. If you don't specify a name, AWS CloudFormation generates a unique    physical ID and uses that ID for the domain name. For more information, see Name     Type.
    /// 
    /// Required when creating a new domain.
    /// 
    /// ImportantIf you specify a name, you can't perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you must replace     the resource, specify a new name.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,


    /// 
    /// An object with one or more of the following keys: SEARCH_SLOW_LOGS,     ES_APPLICATION_LOGS, INDEX_SLOW_LOGS, AUDIT_LOGS,    depending on the types of logs you want to publish. Each key needs a valid     LogPublishingOption value. For the full syntax, see the examples.
    /// 
    /// Required: No
    ///
    /// Type: Map of LogPublishingOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogPublishingOptions")]
    pub log_publishing_options: Option<std::collections::HashMap<String, LogPublishingOption>>,


    /// 
    /// An arbitrary set of tags (key–value pairs) to associate with the OpenSearch Service    domain.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Additional options to specify for the OpenSearch Service domain. For more information, see     AdvancedOptions in the OpenSearch Service API reference.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedOptions")]
    pub advanced_options: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Options for configuring service software updates for a domain.
    /// 
    /// Required: No
    ///
    /// Type: SoftwareUpdateOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SoftwareUpdateOptions")]
    pub software_update_options: Option<SoftwareUpdateOptions>,


    /// 
    /// The version of OpenSearch to use. The value must be in the format    OpenSearch_X.Y or Elasticsearch_X.Y. If not specified, the latest version of OpenSearch is used. For    information about the versions that OpenSearch Service supports, see Supported     versions of OpenSearch and Elasticsearch in the Amazon OpenSearch Service     Developer Guide.
    /// 
    /// If you set the EnableVersionUpgrade update policy to true, you can update     EngineVersion without interruption. When EnableVersionUpgrade is    set to false, or is not specified, updating EngineVersion results in     replacement.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 14
    ///
    /// Maximum: 18
    ///
    /// Pattern: ^Elasticsearch_[0-9]{1}\.[0-9]{1,2}$|^OpenSearch_[0-9]{1,2}\.[0-9]{1,2}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,


    /// 
    /// Specifies whether node-to-node encryption is enabled. See Node-to-node encryption for Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: NodeToNodeEncryptionOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,


    /// 
    /// Configures OpenSearch Service to use Amazon Cognito authentication for OpenSearch    Dashboards.
    /// 
    /// Required: No
    ///
    /// Type: CognitoOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoOptions")]
    pub cognito_options: Option<CognitoOptions>,


    /// 
    /// Whether the domain should encrypt data at rest, and if so, the AWS KMS key to    use. See Encryption of data at rest for Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionAtRestOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionAtRestOptions")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,


    /// 
    /// The virtual private cloud (VPC) configuration for the OpenSearch Service domain. For more    information, see Launching your Amazon OpenSearch Service domains within a VPC in the Amazon OpenSearch Service Developer     Guide.
    /// 
    /// If you remove this entity altogether, along with its associated properties, it causes a replacement. You might encounter this scenario if you're updating your security configuration from a VPC to a public endpoint.
    /// 
    /// Required: No
    ///
    /// Type: VPCOptions
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "VPCOptions")]
    pub vpcoptions: Option<VPCOptions>,


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
    /// Options for a domain's off-peak window, during which OpenSearch Service can perform mandatory configuration changes on the domain.
    /// 
    /// Required: No
    ///
    /// Type: OffPeakWindowOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "OffPeakWindowOptions")]
    pub off_peak_window_options: Option<OffPeakWindowOptions>,


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
    /// Container for the cluster configuration of a domain.
    /// 
    /// Required: No
    ///
    /// Type: ClusterConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: Option<ClusterConfig>,

}


/// DEPRECATED. This setting is only relevant to domains    running legacy Elasticsearch OSS versions earlier than 5.3. It does not apply to OpenSearch    domains.
///
/// The automated snapshot configuration for the OpenSearch Service domain indexes.
#[derive(Default, serde::Serialize)]
pub struct SnapshotOptions {


    /// 
    /// The hour in UTC during which the service takes an automated daily snapshot of the indexes    in the OpenSearch Service domain. For example, if you specify 0, OpenSearch Service takes an    automated snapshot everyday between midnight and 1 am. You can specify a value between 0 and    23.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomatedSnapshotStartHour")]
    pub automated_snapshot_start_hour: Option<i64>,

}


/// Options for configuring service software updates for a domain.
#[derive(Default, serde::Serialize)]
pub struct SoftwareUpdateOptions {


    /// 
    /// Specifies whether automatic service software updates are enabled for the domain.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoSoftwareUpdateEnabled")]
    pub auto_software_update_enabled: Option<bool>,

}


/// Specifies whether the OpenSearch Service domain publishes application, search slow logs,    or index slow logs to Amazon CloudWatch. Each option must be an object of name     SEARCH_SLOW_LOGS, ES_APPLICATION_LOGS,     INDEX_SLOW_LOGS, or AUDIT_LOGS depending on the type of logs you    want to publish. For the full syntax, see the examples.
///
/// Before you enable log publishing, you need to create a CloudWatch log group and provide    OpenSearch Service the correct permissions to write to it. To learn more, see Enabling log publishing (AWS CloudFormation).
#[derive(Default, serde::Serialize)]
pub struct LogPublishingOption {


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


    /// 
    /// Specifies the CloudWatch log group to publish to. Required if you enable log    publishing.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: Option<String>,

}


/// Specifies options for fine-grained access control.
///
/// If you specify advanced security options,    you must also enable node-to-node encryption (NodeToNodeEncryptionOptions) and encryption at rest (EncryptionAtRestOptions). You must also enable EnforceHTTPS within     DomainEndpointOptions, which requires HTTPS for all traffic to the domain.
#[derive(Default, serde::Serialize)]
pub struct AdvancedSecurityOptionsInput {


    /// True to enable the internal user database.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "InternalUserDatabaseEnabled")]
    pub internal_user_database_enabled: Option<bool>,


    /// 
    /// Date and time when the migration period will be disabled. Only necessary when enabling    fine-grained access control on an existing domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnonymousAuthDisableDate")]
    pub anonymous_auth_disable_date: Option<String>,


    /// 
    /// Container for information about the SAML configuration for OpenSearch Dashboards.
    /// 
    /// Required: No
    ///
    /// Type: SAMLOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAMLOptions")]
    pub samloptions: Option<SAMLOptions>,


    /// Specifies information about the master user.
    /// 
    /// Required: No
    ///
    /// Type: MasterUserOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserOptions")]
    pub master_user_options: Option<MasterUserOptions>,


    /// 
    /// True to enable a 30-day migration period during which administrators can create role  mappings. Only necessary when enabling   fine-grained access control on an existing domain.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnonymousAuthEnabled")]
    pub anonymous_auth_enabled: Option<bool>,


    /// True to enable fine-grained access control. You must also enable encryption of data at rest    and node-to-node encryption. See Fine-grained access control in     Amazon OpenSearch Service.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}


/// Specifies additional options for the domain endpoint, such as whether to require HTTPS for all traffic or whether to use a custom endpoint rather than the default endpoint.
#[derive(Default, serde::Serialize)]
pub struct DomainEndpointOptions {


    /// The AWS Certificate Manager ARN for your domain's SSL/TLS certificate. Required if you    enabled a custom endpoint for the domain.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpointCertificateArn")]
    pub custom_endpoint_certificate_arn: Option<String>,


    /// The fully qualified URL for your custom endpoint. Required if you enabled a custom endpoint    for the domain.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpoint")]
    pub custom_endpoint: Option<String>,


    /// True to require that all traffic to the domain arrive over HTTPS. Required if you enable    fine-grained access control in AdvancedSecurityOptions.
    /// 
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
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
    /// Allowed values: Policy-Min-TLS-1-0-2019-07 | Policy-Min-TLS-1-2-2019-07
    ///
    /// Update requires: No interruption
    #[serde(rename = "TLSSecurityPolicy")]
    pub tlssecurity_policy: Option<String>,


    /// True to enable a custom endpoint for the domain. If enabled, you must also provide values for CustomEndpoint and CustomEndpointCertificateArn.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpointEnabled")]
    pub custom_endpoint_enabled: Option<bool>,

}


/// The virtual private cloud (VPC) configuration for the OpenSearch Service domain. For more    information, see Launching your Amazon OpenSearch     Service domains using a VPC in the Amazon OpenSearch Service Developer     Guide.
#[derive(Default, serde::Serialize)]
pub struct VPCOptions {


    /// 
    /// Provide one subnet ID for each Availability Zone that your domain uses. For example, you    must specify three subnet IDs for a three-AZ domain. To learn more, see VPCs and subnets in    the Amazon VPC User Guide.
    /// 
    /// If you specify more than one subnet, you must also configure     ZoneAwarenessEnabled and ZoneAwarenessConfig within ClusterConfig, otherwise you'll see the error "You must specify exactly one subnet"    during template creation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,


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

}


/// The configurations of Amazon Elastic Block Store (Amazon EBS) volumes that are attached to    data nodes in the OpenSearch Service domain. For more information, see EBS volume size limits in the Amazon OpenSearch Service Developer     Guide.
#[derive(Default, serde::Serialize)]
pub struct EBSOptions {


    /// 
    /// The throughput (in MiB/s) of the EBS volumes attached to data nodes. Applies only to the     gp3 volume type.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Throughput")]
    pub throughput: Option<i64>,


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
    /// The EBS volume type to use with the OpenSearch Service domain. If you choose     gp3, you must also specify values for Iops and     Throughput. For more information about each type, see Amazon EBS volume     types in the Amazon EC2 User Guide for Linux Instances.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: gp2 | gp3 | io1 | standard
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,


    /// 
    /// The number of I/O operations per second (IOPS) that the volume supports. This property    applies only to the gp3 and provisioned IOPS EBS volume types.
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

}


/// Container for information about the SAML configuration for OpenSearch Dashboards.
#[derive(Default, serde::Serialize)]
pub struct SAMLOptions {


    /// 
    /// True to enable SAML authentication for a domain.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The backend role that the SAML master user is mapped to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterBackendRole")]
    pub master_backend_role: Option<String>,


    /// 
    /// The SAML Identity Provider's information.
    /// 
    /// Required: No
    ///
    /// Type: Idp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Idp")]
    pub idp: Option<Idp>,


    /// 
    /// The duration, in minutes, after which a user session becomes inactive. Acceptable values are between 1 and 1440,  and the default value is 60.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionTimeoutMinutes")]
    pub session_timeout_minutes: Option<i64>,


    /// 
    /// The SAML master user name, which is stored in the domain's internal user database.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserName")]
    pub master_user_name: Option<String>,


    /// 
    /// Element of the SAML assertion to use for the user name. Default is  NameID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectKey")]
    pub subject_key: Option<String>,


    /// 
    /// Element of the SAML assertion to use for backend roles. Default is  roles.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RolesKey")]
    pub roles_key: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// Whether the domain should encrypt data at rest, and if so, the AWS Key Management Service key to use.
#[derive(Default, serde::Serialize)]
pub struct EncryptionAtRestOptions {


    /// 
    /// Specify true to enable encryption at rest. Required if you enable    fine-grained access control in AdvancedSecurityOptionsInput.
    /// 
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The KMS key ID. Takes the form 1a2a3a4-1a2a-3a4a-5a6a-1a2a3a4a5a6a. Required    if you enable encryption at rest.
    /// 
    /// You can also use keyAlias as a value.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 500
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


/// The current status of the service software for an Amazon OpenSearch Service domain. For more  information, see Service software updates in   Amazon OpenSearch Service.
#[derive(Default, serde::Serialize)]
pub struct ServiceSoftwareOptions {


    /// 
    /// A description of the service software update status.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// True if you're able to cancel your service software version update. False if you can't  cancel your service software update.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cancellable")]
    pub cancellable: Option<bool>,


    /// 
    /// The new service software version, if one is available.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewVersion")]
    pub new_version: Option<String>,


    /// 
    /// True if a service software is never automatically updated. False if a service software is  automatically updated after the automated update date.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionalDeployment")]
    pub optional_deployment: Option<bool>,


    /// 
    /// The current service software version present on the domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrentVersion")]
    pub current_version: Option<String>,


    /// 
    /// True if you're able to update your service software version. False if you can't update your  service software version.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateAvailable")]
    pub update_available: Option<bool>,


    /// 
    /// The status of your service software update.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COMPLETED | ELIGIBLE | IN_PROGRESS | NOT_ELIGIBLE | PENDING_UPDATE
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateStatus")]
    pub update_status: Option<String>,


    /// 
    /// The timestamp, in Epoch time, until which you can manually request a service software update. After this date,  we automatically update your service software.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomatedUpdateDate")]
    pub automated_update_date: Option<String>,

}


/// A custom start time for the off-peak window, in Coordinated Universal Time (UTC). The window length will always be 10 hours, so you can't    specify an end time. For example, if you specify 11:00 P.M. UTC as a start time, the end time will automatically be set to 9:00 A.M.
#[derive(Default, serde::Serialize)]
pub struct WindowStartTime {


    /// 
    /// The start minute of the window, in UTC. The minimum value is 0 and the maximum value is 59.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Minutes")]
    pub minutes: i64,


    /// 
    /// The start hour of the window in Coordinated Universal Time (UTC), using 24-hour time. For example, 17 refers to 5:00 P.M. UTC.    The minimum value is 0 and the maximum value is 23.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hours")]
    pub hours: i64,

}


/// The SAML Identity Provider's information.
#[derive(Default, serde::Serialize)]
pub struct Idp {


    /// 
    /// The unique entity ID of the application in the SAML identity provider.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 8
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityId")]
    pub entity_id: String,


    /// 
    /// The metadata of the SAML application, in XML format.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1048576
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetadataContent")]
    pub metadata_content: String,

}


/// The cluster configuration for the OpenSearch Service domain. You can specify options such    as the instance type and the number of instances. For more information, see Creating and managing Amazon OpenSearch Service domains in the Amazon OpenSearch Service     Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct ClusterConfig {


    /// 
    /// The instance type for the cluster's warm nodes.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ultrawarm1.large.search | ultrawarm1.medium.search | ultrawarm1.xlarge.search
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmType")]
    pub warm_type: Option<String>,


    /// 
    /// Whether to enable UltraWarm storage for the cluster. See UltraWarm storage for Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmEnabled")]
    pub warm_enabled: Option<bool>,


    /// 
    /// Specifies zone awareness configuration options. Only use if     ZoneAwarenessEnabled is true.
    /// 
    /// Required: No
    ///
    /// Type: ZoneAwarenessConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZoneAwarenessConfig")]
    pub zone_awareness_config: Option<ZoneAwarenessConfig>,


    /// 
    /// The instance type for your data nodes, such as m3.medium.search. For valid    values, see Supported     instance types in Amazon OpenSearch Service .
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: c4.2xlarge.search | c4.4xlarge.search | c4.8xlarge.search | c4.large.search | c4.xlarge.search | c5.18xlarge.search | c5.2xlarge.search | c5.4xlarge.search | c5.9xlarge.search | c5.large.search | c5.xlarge.search | c6g.12xlarge.search | c6g.2xlarge.search | c6g.4xlarge.search | c6g.8xlarge.search | c6g.large.search | c6g.xlarge.search | d2.2xlarge.search | d2.4xlarge.search | d2.8xlarge.search | d2.xlarge.search | i2.2xlarge.search | i2.xlarge.search | i3.16xlarge.search | i3.2xlarge.search | i3.4xlarge.search | i3.8xlarge.search | i3.large.search | i3.xlarge.search | m3.2xlarge.search | m3.large.search | m3.medium.search | m3.xlarge.search | m4.10xlarge.search | m4.2xlarge.search | m4.4xlarge.search | m4.large.search | m4.xlarge.search | m5.12xlarge.search | m5.24xlarge.search | m5.2xlarge.search | m5.4xlarge.search | m5.large.search | m5.xlarge.search | m6g.12xlarge.search | m6g.2xlarge.search | m6g.4xlarge.search | m6g.8xlarge.search | m6g.large.search | m6g.xlarge.search | r3.2xlarge.search | r3.4xlarge.search | r3.8xlarge.search | r3.large.search | r3.xlarge.search | r4.16xlarge.search | r4.2xlarge.search | r4.4xlarge.search | r4.8xlarge.search | r4.large.search | r4.xlarge.search | r5.12xlarge.search | r5.24xlarge.search | r5.2xlarge.search | r5.4xlarge.search | r5.large.search | r5.xlarge.search | r6g.12xlarge.search | r6g.2xlarge.search | r6g.4xlarge.search | r6g.8xlarge.search | r6g.large.search | r6g.xlarge.search | r6gd.12xlarge.search | r6gd.16xlarge.search | r6gd.2xlarge.search | r6gd.4xlarge.search | r6gd.8xlarge.search | r6gd.large.search | r6gd.xlarge.search | t2.medium.search | t2.micro.search | t2.small.search | t3.2xlarge.search | t3.large.search | t3.medium.search | t3.micro.search | t3.nano.search | t3.small.search | t3.xlarge.search | t4g.medium.search | t4g.small.search | ultrawarm1.large.search | ultrawarm1.medium.search | ultrawarm1.xlarge.search
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,


    /// 
    /// The number of warm nodes in the cluster.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmCount")]
    pub warm_count: Option<i64>,


    /// 
    /// The hardware configuration of the computer that hosts the dedicated master node, such as     m3.medium.search. If you specify this property, you must specify     true for the DedicatedMasterEnabled property. For valid values,    see Supported     instance types in Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: c4.2xlarge.search | c4.4xlarge.search | c4.8xlarge.search | c4.large.search | c4.xlarge.search | c5.18xlarge.search | c5.2xlarge.search | c5.4xlarge.search | c5.9xlarge.search | c5.large.search | c5.xlarge.search | c6g.12xlarge.search | c6g.2xlarge.search | c6g.4xlarge.search | c6g.8xlarge.search | c6g.large.search | c6g.xlarge.search | d2.2xlarge.search | d2.4xlarge.search | d2.8xlarge.search | d2.xlarge.search | i2.2xlarge.search | i2.xlarge.search | i3.16xlarge.search | i3.2xlarge.search | i3.4xlarge.search | i3.8xlarge.search | i3.large.search | i3.xlarge.search | m3.2xlarge.search | m3.large.search | m3.medium.search | m3.xlarge.search | m4.10xlarge.search | m4.2xlarge.search | m4.4xlarge.search | m4.large.search | m4.xlarge.search | m5.12xlarge.search | m5.24xlarge.search | m5.2xlarge.search | m5.4xlarge.search | m5.large.search | m5.xlarge.search | m6g.12xlarge.search | m6g.2xlarge.search | m6g.4xlarge.search | m6g.8xlarge.search | m6g.large.search | m6g.xlarge.search | r3.2xlarge.search | r3.4xlarge.search | r3.8xlarge.search | r3.large.search | r3.xlarge.search | r4.16xlarge.search | r4.2xlarge.search | r4.4xlarge.search | r4.8xlarge.search | r4.large.search | r4.xlarge.search | r5.12xlarge.search | r5.24xlarge.search | r5.2xlarge.search | r5.4xlarge.search | r5.large.search | r5.xlarge.search | r6g.12xlarge.search | r6g.2xlarge.search | r6g.4xlarge.search | r6g.8xlarge.search | r6g.large.search | r6g.xlarge.search | r6gd.12xlarge.search | r6gd.16xlarge.search | r6gd.2xlarge.search | r6gd.4xlarge.search | r6gd.8xlarge.search | r6gd.large.search | r6gd.xlarge.search | t2.medium.search | t2.micro.search | t2.small.search | t3.2xlarge.search | t3.large.search | t3.medium.search | t3.micro.search | t3.nano.search | t3.small.search | t3.xlarge.search | t4g.medium.search | t4g.small.search | ultrawarm1.large.search | ultrawarm1.medium.search | ultrawarm1.xlarge.search
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
    /// Indicates whether to enable zone awareness for the OpenSearch Service domain. When you    enable zone awareness, OpenSearch Service allocates the nodes and replica index shards that    belong to a cluster across two Availability Zones (AZs) in the same region to prevent data    loss and minimize downtime in the event of node or data center failure. Don't enable zone    awareness if your cluster has no replica index shards or is a single-node cluster. For more    information, see Configuring a multi-AZ domain in Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZoneAwarenessEnabled")]
    pub zone_awareness_enabled: Option<bool>,


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
    /// The number of instances to use for the master node. If you specify this property, you must    specify true for the DedicatedMasterEnabled property.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DedicatedMasterCount")]
    pub dedicated_master_count: Option<i64>,

}


/// Specifies options for node-to-node encryption.
#[derive(Default, serde::Serialize)]
pub struct NodeToNodeEncryptionOptions {


    /// 
    /// Specifies to enable or disable node-to-node encryption on the domain. Required if you    enable fine-grained access control in AdvancedSecurityOptionsInput.
    /// 
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}


/// Specifies zone awareness configuration options. Only use if     ZoneAwarenessEnabled is true.
#[derive(Default, serde::Serialize)]
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


/// Off-peak window settings for the domain.
#[derive(Default, serde::Serialize)]
pub struct OffPeakWindowOptions {


    /// 
    /// Off-peak window settings for the domain.
    /// 
    /// Required: No
    ///
    /// Type: OffPeakWindow
    ///
    /// Update requires: No interruption
    #[serde(rename = "OffPeakWindow")]
    pub off_peak_window: Option<OffPeakWindow>,


    /// 
    /// Specifies whether off-peak window settings are enabled for the domain.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}


/// A custom 10-hour, low-traffic window during which OpenSearch Service can perform mandatory configuration changes on the domain.    These actions can include scheduled service software updates and blue/green Auto-Tune enhancements. OpenSearch Service will    schedule these actions during the window that you specify. If you don't specify a window start time, it defaults to 10:00 P.M. local time.
#[derive(Default, serde::Serialize)]
pub struct OffPeakWindow {


    /// 
    /// The desired start time for an off-peak maintenance window.
    /// 
    /// Required: No
    ///
    /// Type: WindowStartTime
    ///
    /// Update requires: No interruption
    #[serde(rename = "WindowStartTime")]
    pub window_start_time: Option<WindowStartTime>,

}


/// Configures OpenSearch Service to use Amazon Cognito authentication for OpenSearch    Dashboards.
#[derive(Default, serde::Serialize)]
pub struct CognitoOptions {


    /// 
    /// The Amazon Cognito identity pool ID that you want OpenSearch Service to use for OpenSearch    Dashboards authentication.
    /// 
    /// Required if you enabled Cognito Authentication for OpenSearch Dashboards.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 55
    ///
    /// Pattern: [\w-]+:[0-9a-f-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: Option<String>,


    /// 
    /// Whether to enable or disable Amazon Cognito authentication for OpenSearch Dashboards. See     Amazon Cognito     authentication for OpenSearch Dashboards.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The Amazon Cognito user pool ID that you want OpenSearch Service to use for OpenSearch    Dashboards authentication.
    /// 
    /// Required if you enabled Cognito Authentication for OpenSearch Dashboards.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 55
    ///
    /// Pattern: [\w-]+_[0-9a-zA-Z]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: Option<String>,


    /// 
    /// The AmazonOpenSearchServiceCognitoAccess role that allows OpenSearch Service    to configure your user pool and identity pool.
    /// 
    /// Required if you enabled Cognito Authentication for OpenSearch Dashboards.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:(aws|aws\-cn|aws\-us\-gov|aws\-iso|aws\-iso\-b):iam::[0-9]+:role\/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,

}


/// Specifies information about the master user.
///
/// Required if if InternalUserDatabaseEnabled is true in AdvancedSecurityOptions.
#[derive(Default, serde::Serialize)]
pub struct MasterUserOptions {


    /// Amazon Resource Name (ARN) for the master user. The ARN can point to an IAM user or role. This    property is required for Amazon Cognito to work, and it must match the role configured for    Cognito. Only specify if InternalUserDatabaseEnabled is false in AdvancedSecurityOptionsInput.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserARN")]
    pub master_user_arn: Option<String>,


    /// Username for the master user. Only specify if InternalUserDatabaseEnabled is true    in AdvancedSecurityOptionsInput.
    /// 
    /// If you don't want to specify this value directly within the template, you can use a dynamic reference instead.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserName")]
    pub master_user_name: Option<String>,


    /// Password for the master user. Only specify if InternalUserDatabaseEnabled is true    in AdvancedSecurityOptionsInput.
    /// 
    /// If you don't want to specify this value directly within the template, you can use a dynamic reference instead.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 8
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,

}