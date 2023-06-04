/// A broker is a message broker environment running on Amazon MQ. It is    the basic building block of Amazon MQ.
///
/// The AWS::AmazonMQ::Broker resource lets you create Amazon MQ for ActiveMQ and Amazon MQ for RabbitMQ brokers, add    configuration changes or modify users for a speified ActiveMQ broker, return information about the    specified broker, and delete the broker. For more information, see How Amazon MQ works in the Amazon MQ Developer    Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnBroker {
    ///
    /// Optional. The authentication strategy used to secure the broker. The default is          SIMPLE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthenticationStrategy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub authentication_strategy: Option<cfn_resources::StrVal>,

    ///
    /// Enables automatic upgrades to new minor versions for brokers, as new broker engine versions    are released and supported by Amazon MQ. Automatic upgrades occur during the scheduled maintenance window of the broker or after a    manual broker reboot.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,

    ///
    /// The name of the broker. This value must be unique in your AWS account, 1-50 characters    long, must contain only letters, numbers, dashes, and underscores, and must not contain white    spaces, brackets, wildcard characters, or special characters.
    ///
    /// Important     Do not add personally identifiable information (PII) or other confidential or sensitive information in broker names.     Broker names are accessible to other AWS services, including CCloudWatch Logs. Broker names are not intended to be     used for private or sensitive data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BrokerName")]
    pub broker_name: cfn_resources::StrVal,

    ///
    /// A list of information about the configuration. Does not apply to RabbitMQ brokers.
    ///
    /// Required: No
    ///
    /// Type: ConfigurationId
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub configuration: Option<ConfigurationId>,

    ///
    /// The deployment mode of the broker. Available values:
    ///
    /// SINGLE_INSTANCEACTIVE_STANDBY_MULTI_AZCLUSTER_MULTI_AZ
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentMode")]
    pub deployment_mode: cfn_resources::StrVal,

    ///
    /// Encryption options for the broker. Does not apply to RabbitMQ brokers.
    ///
    /// Required: No
    ///
    /// Type: EncryptionOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionOptions")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub encryption_options: Option<EncryptionOptions>,

    ///
    /// The type of broker engine. Currently, Amazon MQ supports ACTIVEMQ and RABBITMQ.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineType")]
    pub engine_type: cfn_resources::StrVal,

    ///
    /// The version of the broker engine. For a list of supported engine versions, see Engine in the Amazon MQ Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    pub engine_version: cfn_resources::StrVal,

    ///
    /// The broker's instance type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "HostInstanceType")]
    pub host_instance_type: cfn_resources::StrVal,

    ///
    /// Optional. The metadata of the LDAP server used to authenticate and authorize        connections to the broker. Does not apply to RabbitMQ brokers.
    ///
    /// Required: No
    ///
    /// Type: LdapServerMetadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "LdapServerMetadata")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub ldap_server_metadata: Option<LdapServerMetadata>,

    ///
    /// Enables Amazon CloudWatch logging for brokers.
    ///
    /// Required: No
    ///
    /// Type: LogList
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub logs: Option<LogList>,

    ///
    /// The scheduled time period relative to UTC during which Amazon MQ begins to apply pending    updates or patches to the broker.
    ///
    /// Required: No
    ///
    /// Type: MaintenanceWindow
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceWindowStartTime")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub maintenance_window_start_time: Option<MaintenanceWindow>,

    ///
    /// Enables connections from applications outside of the VPC that hosts the broker's    subnets.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: bool,

    ///
    /// The list of rules (1 minimum, 125 maximum) that authorize connections to        brokers.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// The broker's storage type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub storage_type: Option<cfn_resources::StrVal>,

    ///
    /// The list of groups that define which subnets and IP ranges the broker can use from different Availability Zones.    If you specify more than one subnet, the subnets must be in different Availability Zones. Amazon MQ will not be able to create    VPC endpoints for your broker with multiple subnets in the same Availability Zone.    A SINGLE_INSTANCE deployment requires one subnet (for example, the default subnet).    An ACTIVE_STANDBY_MULTI_AZ deployment (ACTIVEMQ) requires two subnets. A CLUSTER_MULTI_AZ deployment (RABBITMQ)     has no subnet requirements when deployed with public accessibility, deployment without public accessibility requires at least one subnet.
    ///
    /// Important     If you specify subnets in a shared VPC for a RabbitMQ broker, the associated VPC to which the specified subnets     belong must be owned by your AWS account. Amazon MQ will not be able to create VPC enpoints in VPCs that are not owned by     your AWS account.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub subnet_ids: Option<Vec<String>>,

    ///
    /// An array of key-value pairs. For more information, see Using Cost Allocation Tags in the Billing and Cost Management User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<TagsEntry>>,

    ///
    /// The list of broker users (persons or applications) who can access queues and topics.    For Amazon MQ for RabbitMQ brokers, one and only one administrative user is accepted and created when a broker is first provisioned.    All subsequent RabbitMQ users are created by via the RabbitMQ web console or by using the RabbitMQ management API.
    ///
    /// Required: Yes
    ///
    /// Type: List of User
    ///
    /// Update requires: No interruption
    #[serde(rename = "Users")]
    pub users: Vec<User>,

    #[serde(skip_serializing)]
    pub att_arn: CfnBrokerarn,

    #[serde(skip_serializing)]
    pub att_configuration_id: CfnBrokerconfigurationid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnBrokerarn;
impl CfnBrokerarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnBrokerconfigurationid;
impl CfnBrokerconfigurationid {
    pub fn att_name(&self) -> &'static str {
        r#"ConfigurationId"#
    }
}

impl cfn_resources::CfnResource for CfnBroker {
    fn type_string(&self) -> &'static str {
        "AWS::AmazonMQ::Broker"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.encryption_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ldap_server_metadata
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.logs.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.maintenance_window_start_time
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A list of information about the configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigurationId {
    ///
    /// The unique ID that Amazon MQ generates for the configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The revision number of the configuration.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Revision")]
    pub revision: i64,
}

impl cfn_resources::CfnResource for ConfigurationId {
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

/// Encryption options for the broker.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncryptionOptions {
    ///
    /// The customer master key (CMK) to use for the A AWS KMS (KMS).        This key is used to encrypt your data at rest. If not provided, Amazon MQ will use a        default CMK to encrypt your data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Enables the use of an AWS owned CMK using AWS KMS (KMS). Set to true by default, if no value is provided, for example,        for RabbitMQ brokers.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseAwsOwnedKey")]
    pub use_aws_owned_key: bool,
}

impl cfn_resources::CfnResource for EncryptionOptions {
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

/// Optional. The metadata of the LDAP server used to authenticate and authorize        connections to the broker.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LdapServerMetadata {
    ///
    /// Specifies the location of the LDAP server such as AWS Directory Service for Microsoft Active Directory. Optional failover server.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hosts")]
    pub hosts: Vec<String>,

    /// The distinguished name of the node in the directory information tree (DIT) to search for roles or groups.   For example, ou=group, ou=corp, dc=corp, dc=example, dc=com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleBase")]
    pub role_base: cfn_resources::StrVal,

    ///
    /// The group name attribute in a role entry whose value is the name of that role. For example, you can specify cn for a group entry's common name.    If authentication succeeds, then the user is assigned the the value of the cn attribute for each role entry that they are a member of.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub role_name: Option<cfn_resources::StrVal>,

    ///
    /// The LDAP search filter used to find roles within the roleBase. The distinguished name of the user matched by userSearchMatching    is substituted into the {0} placeholder in the search filter. The client's username is substituted into the    {1} placeholder. For example, if you set this option to (member=uid={1}) for the user janedoe, the search filter becomes (member=uid=janedoe)    after string substitution. It matches all role entries that have a member attribute equal to uid=janedoe under the subtree selected by the RoleBases.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleSearchMatching")]
    pub role_search_matching: cfn_resources::StrVal,

    /// The directory search scope for the role. If set to true, scope is to search the entire subtree.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleSearchSubtree")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub role_search_subtree: Option<bool>,

    /// Service account password. A service account is an account in your LDAP server that has access to initiate a connection. For example,   cn=admin,dc=corp, dc=example, dc=com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccountPassword")]
    pub service_account_password: cfn_resources::StrVal,

    ///
    /// Service account username. A service account is an account in your LDAP server that has access to initiate a connection. For example,    cn=admin, ou=corp, dc=corp, dc=example, dc=com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccountUsername")]
    pub service_account_username: cfn_resources::StrVal,

    ///
    /// Select a particular subtree of the directory information tree (DIT) to search for user entries.    The subtree is specified by a DN, which specifies the base node of the subtree. For example, by setting this option to    ou=Users,ou=corp, dc=corp, dc=example, dc=com, the search for user entries is restricted to the subtree beneath    ou=Users,ou=corp, dc=corp, dc=example, dc=com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserBase")]
    pub user_base: cfn_resources::StrVal,

    ///
    /// The name of the LDAP attribute in the user's directory entry for the user's group membership. In some cases, user roles may be    identified by the value of an attribute in the user's directory entry. The UserRoleName option allows you to provide the name of this attribute.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserRoleName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_role_name: Option<cfn_resources::StrVal>,

    ///
    /// The LDAP search filter used to find users within the userBase. The client's username is substituted into the {0}    placeholder in the search filter. For example, if this option is set to (uid={0}) and the received username is janedoe,    the search filter becomes (uid=janedoe) after string substitution. It will result in matching an entry like uid=janedoe,    ou=Users, ou=corp, dc=corp, dc=example, dc=com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserSearchMatching")]
    pub user_search_matching: cfn_resources::StrVal,

    ///
    /// The directory search scope for the user.    If set to true, scope is to search the entire subtree.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserSearchSubtree")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_search_subtree: Option<bool>,
}

impl cfn_resources::CfnResource for LdapServerMetadata {
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

/// The list of information about logs to be enabled for the specified broker.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LogList {
    ///
    /// Enables audit logging. Every user management action made using JMX or the ActiveMQ        Web Console is logged. Does not apply to RabbitMQ brokers.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Audit")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub audit: Option<bool>,

    ///
    /// Enables general logging.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "General")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub general: Option<bool>,
}

impl cfn_resources::CfnResource for LogList {
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

/// The parameters that determine the WeeklyStartTime to apply pending updates or    patches to the broker.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MaintenanceWindow {
    ///
    /// The day of the week.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: cfn_resources::StrVal,

    ///
    /// The time, in 24-hour format.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeOfDay")]
    pub time_of_day: cfn_resources::StrVal,

    ///
    /// The time zone, UTC by default, in either the Country/City format, or the UTC offset    format.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeZone")]
    pub time_zone: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for MaintenanceWindow {
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

/// A key-value pair to associate with the broker.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagsEntry {
    ///
    /// The key in a key-value pair.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value in a key-value pair.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TagsEntry {
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

/// The list of broker users (persons or applications) who can access queues and topics.    For Amazon MQ for RabbitMQ brokers, one and only one administrative user is accepted and created when a broker is first provisioned.     All subsequent broker users are created via the RabbitMQ web console or by using the RabbitMQ management API.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct User {
    ///
    /// Enables access to the ActiveMQ web console for the ActiveMQ user. Does not apply to RabbitMQ brokers.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConsoleAccess")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub console_access: Option<bool>,

    ///
    /// The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can        contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _        ~). This value must be 2-100 characters long. Does not apply to RabbitMQ brokers.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub groups: Option<Vec<String>>,

    ///
    /// The password of the user. This value must be at least 12 characters long, must contain at least 4 unique characters, and must not contain commas, colons, or equal signs (,:=).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: cfn_resources::StrVal,

    ///
    /// The username of the broker user. For Amazon MQ for ActiveMQ brokers, this value can contain only alphanumeric    characters, dashes, periods, underscores, and tildes (- . _ ~). For Amazon MQ for RabbitMQ brokers, this value can contain only    alphanumeric characters, dashes, periods, underscores (- . _). This value must not contain a tilde (~) character. Amazon MQ prohibts    using guest as a valid usename. This value must be 2-100 characters long.
    ///
    /// Important     Do not add personally identifiable information (PII) or other confidential or sensitive information in broker usernames.     Broker usernames are accessible to other AWS services, including CloudWatch Logs. Broker usernames are not intended to be     used for private or sensitive data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for User {
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
