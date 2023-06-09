/// The AWS::DMS::ReplicationInstance resource creates an AWS DMS replication instance.       To create a ReplicationInstance, you need permissions to create instances. You'll need similar permissions to terminate       instances when you delete stacks with instances.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnReplicationInstance {
    ///
    /// The amount of storage (in gigabytes) to be initially allocated for the replication instance.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,

    ///
    /// Indicates that major version upgrades are allowed. Changing this parameter does not     result in an outage, and the change is asynchronously applied as soon as possible.
    ///
    /// This parameter must be set to true when specifying a value for the       EngineVersion parameter that is a different major version than the     replication instance's current version.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,

    ///
    /// A value that indicates whether minor engine upgrades are applied automatically to the       replication instance during the maintenance window. This parameter defaults to true.
    ///
    /// Default: true
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,

    ///
    /// The Availability Zone that the replication instance will be created in.
    ///
    /// The default value is a random, system-chosen Availability Zone in the endpoint's AWS Region,       for example us-east-1d.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// The engine version number of the replication instance.
    ///
    /// If an engine version number is not specified when a replication       instance is created, the default is the latest engine version available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<cfn_resources::StrVal>,

    ///
    /// An AWS KMS key identifier that is used to encrypt the data on the replication instance.
    ///
    /// If you don't specify a value for the KmsKeyId parameter, AWS DMS uses your default encryption key.
    ///
    /// AWS KMS creates the default encryption key for your AWS account. Your AWS account       has a different default encryption key for each AWS Region.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether the replication instance is a Multi-AZ deployment. You can't set the       AvailabilityZone parameter if the Multi-AZ parameter is set to true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,

    ///
    /// The weekly time range during which system maintenance can occur, in UTC.
    ///
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    ///
    /// Default: A 30-minute window selected at random from an 8-hour block of time per AWS Region,       occurring on a random day of the week.
    ///
    /// Valid days (ddd): Mon | Tue |       Wed | Thu | Fri | Sat | Sun
    ///
    /// Constraints: Minimum 30-minute window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the accessibility options for the replication instance. A value of       true represents an instance with a public IP address. A value of       false represents an instance with a private IP address.       The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,

    ///
    /// The compute and memory capacity of the replication instance as defined for the specified       replication instance class. For example, to specify the instance class dms.c4.large, set this parameter to "dms.c4.large".       For more information on the settings and capacities for the available replication instance classes, see            Selecting the right AWS DMS replication instance for your migration       in the AWS Database Migration Service User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationInstanceClass")]
    pub replication_instance_class: cfn_resources::StrVal,

    ///
    /// The replication instance identifier. This parameter is stored as a lowercase string.
    ///
    /// Constraints:
    ///
    /// Must contain 1-63 alphanumeric characters or hyphens.               First character must be a letter.               Can't end with a hyphen or contain two consecutive hyphens.
    ///
    /// Example: myrepinstance
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<cfn_resources::StrVal>,

    ///
    /// A subnet group to associate with the replication instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<cfn_resources::StrVal>,

    ///
    /// A display name for the resource identifier at the end of the EndpointArn     response parameter that is returned in the created Endpoint object. The value     for this parameter can have up to 31 characters. It can contain only ASCII letters, digits,     and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens,     and can only begin with a letter, such as Example-App-ARN1. For example, this     value might result in the EndpointArn value     arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1. If you don't     specify a ResourceIdentifier value, AWS DMS generates a default identifier     value for the end of EndpointArn.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<cfn_resources::StrVal>,

    ///
    /// One or more tags to be assigned to the replication instance.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Specifies the virtual private cloud (VPC) security group to be used with the replication instance. The VPC     security group must work with the VPC containing the replication instance.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub att_replication_instance_private_ip_addresses:
        CfnReplicationInstancereplicationinstanceprivateipaddresses,

    #[serde(skip_serializing)]
    pub att_replication_instance_public_ip_addresses:
        CfnReplicationInstancereplicationinstancepublicipaddresses,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationInstancereplicationinstanceprivateipaddresses;
impl CfnReplicationInstancereplicationinstanceprivateipaddresses {
    pub fn att_name(&self) -> &'static str {
        r#"ReplicationInstancePrivateIpAddresses"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationInstancereplicationinstancepublicipaddresses;
impl CfnReplicationInstancereplicationinstancepublicipaddresses {
    pub fn att_name(&self) -> &'static str {
        r#"ReplicationInstancePublicIpAddresses"#
    }
}

impl cfn_resources::CfnResource for CfnReplicationInstance {
    fn type_string(&self) -> &'static str {
        "AWS::DMS::ReplicationInstance"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
