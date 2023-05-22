/// The AWS::Neptune::DBInstance type creates an Amazon Neptune DB instance.
///
/// Updating DB Instances
///
/// You can set a deletion policy for your DB instance to control how AWS CloudFormation handles the    instance when the stack is deleted. For Neptune DB instances, you can choose to     retain the instance, to delete the    instance, or to create a snapshot of the instance. The default    AWS CloudFormation behavior depends on the DBClusterIdentifier property:
///
/// Deleting DB Instances
///
/// When properties labeled Update requires: Replacement are updated,      AWS CloudFormation first creates a       replacement DB instance, changes references from other dependent resources to point to       the replacement DB instance, and finally deletes the old DB instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBInstance {
    ///
    /// Indicates that major version upgrades are allowed. Changing this    parameter doesn't result in an outage and the change is asynchronously    applied as soon as possible. This parameter must be set to true when specifying    a value for the EngineVersion parameter that is a different major version than    the DB instance's current version.
    ///
    /// WarningWhen you change this parameter for an existing DB cluster, CloudFormation will replace your existing DB cluster    with a new, empty one that uses the engine version you specified.
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
    /// Indicates that minor version patches are applied automatically.
    ///
    /// When updating this property, some interruptions may occur.
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
    /// Specifies the name of the Availability Zone the DB instance is located in.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// If the DB instance is a member of a DB cluster, contains the name of the DB cluster that    the DB instance is a member of.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbcluster_identifier: Option<cfn_resources::StrVal>,

    ///
    /// Contains the name of the compute and memory capacity class of the DB instance.
    ///
    /// If you update this property, some interruptions may occur.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DBInstanceClass")]
    pub dbinstance_class: cfn_resources::StrVal,

    ///
    /// Contains a user-supplied database identifier. This identifier is the unique key that    identifies a DB instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbinstance_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The name of an existing DB parameter group or a reference to an        AWS::Neptune::DBParameterGroup resource created in the template.        If any of the data members of the referenced parameter          group are changed during an update, the DB instance might need to be restarted,          which causes some interruption. If the parameter group contains static parameters,          whether they were changed or not, an update triggers a reboot.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbparameter_group_name: Option<cfn_resources::StrVal>,

    ///
    /// This parameter is not supported.
    ///
    /// AWS::Neptune::DBInstance does not support restoring from snapshots.
    ///
    /// AWS::Neptune::DBCluster does support restoring from snapshots.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbsnapshot_identifier: Option<cfn_resources::StrVal>,

    ///
    /// A DB subnet group to associate with the DB instance. If you update this value,          the new subnet group must be a subnet group in a new virtual private cloud          (VPC).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbsubnet_group_name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the weekly time range during which system maintenance can occur, in Universal    Coordinated Time (UTC).
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
    /// An arbitrary set of tags (key-value pairs) for this DB instance.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_endpoint: CfnDBInstanceendpoint,

    #[serde(skip_serializing)]
    pub att_port: CfnDBInstanceport,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBInstanceendpoint;
impl CfnDBInstanceendpoint {
    pub fn att_name(&self) -> &'static str {
        r#"Endpoint"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBInstanceport;
impl CfnDBInstanceport {
    pub fn att_name(&self) -> &'static str {
        r#"Port"#
    }
}

impl cfn_resources::CfnResource for CfnDBInstance {
    fn type_string(&self) -> &'static str {
        "AWS::Neptune::DBInstance"
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
