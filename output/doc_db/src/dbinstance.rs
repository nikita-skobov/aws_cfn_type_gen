/// The AWS::DocDB::DBInstance Amazon DocumentDB (with MongoDB compatibility) resource describes a DBInstance.      For more information, see DBInstance in the Amazon DocumentDB Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBInstance {
    ///
    /// This parameter does not apply to Amazon DocumentDB. Amazon DocumentDB does not perform minor version upgrades regardless of the value set.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,

    ///
    /// The Amazon EC2 Availability Zone that the instance is created in.
    ///
    /// Default: A random, system-chosen Availability Zone in the endpoint's AWS Region.
    ///
    /// Example: us-east-1d
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,

    ///
    /// The identifier of the cluster that the instance will belong to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: String,

    ///
    /// The compute and memory capacity of the instance; for example,       db.m4.large. If you change the class of an instance there       can be some interruption in the cluster's service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBInstanceClass")]
    pub dbinstance_class: String,

    ///
    /// The instance identifier. This parameter is stored as a lowercase string.
    ///
    /// Constraints:
    ///
    /// Must contain from 1 to 63 letters, numbers, or hyphens.               The first character must be a letter.               Cannot end with a hyphen or contain two consecutive hyphens.
    ///
    /// Example: mydbinstance
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBInstanceIdentifier")]
    pub dbinstance_identifier: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnablePerformanceInsights")]
    pub enable_performance_insights: Option<bool>,

    ///
    /// The time range each week during which system maintenance can occur, in Universal       Coordinated Time (UTC).
    ///
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    ///
    /// The default is a 30-minute window selected at random from an 8-hour block of time for       each AWS Region, occurring on a random day of the week.
    ///
    /// Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun
    ///
    /// Constraints: Minimum 30-minute window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,

    ///
    /// The tags to be assigned to the instance. You can assign up to      10 tags to an instance.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnDBInstance {
    fn type_string(&self) -> &'static str {
        "AWS::DocDB::DBInstance"
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
