/// Creates a new Amazon DocumentDB elastic cluster and returns its cluster structure.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCluster {
    ///
    /// The name of the Amazon DocumentDB elastic clusters administrator.
    ///
    /// Constraints:
    ///
    /// Must be from 1 to 63 letters or numbers.The first character must be a letter.Cannot be a reserved word.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AdminUserName")]
    pub admin_user_name: String,

    ///
    /// The password for the Elastic DocumentDB cluster administrator and can    contain any printable ASCII characters.
    ///
    /// Constraints:
    ///
    /// Must contain from 8 to 100 characters.Cannot contain a forward slash (/), double quote ("), or the "at" symbol (@).A valid AdminUserName entry is also required.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminUserPassword")]
    pub admin_user_password: Option<String>,

    ///
    /// The authentication type used to determine where to fetch the password used for accessing the elastic cluster.    Valid types are PLAIN_TEXT or SECRET_ARN.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthType")]
    pub auth_type: String,

    ///
    /// The name of the new elastic cluster. This parameter is stored as    a lowercase string.
    ///
    /// Constraints:
    ///
    /// Must contain from 1 to 63 letters, numbers, or hyphens.The first character must be a letter.Cannot end with a hyphen or contain two consecutive hyphens.
    ///
    /// Example: my-cluster
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,

    ///
    /// The KMS key identifier to use to encrypt the new elastic cluster.
    ///
    /// The KMS key identifier is the Amazon Resource Name (ARN) for the KMS     encryption key. If you are creating a cluster using the same Amazon account     that owns this KMS encryption key, you can use the KMS key alias instead     of the ARN as the KMS encryption key.
    ///
    /// If an encryption key is not specified, Amazon DocumentDB uses the     default encryption key that KMS creates for your account. Your account     has a different default encryption key for each Amazon Region.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

    ///
    /// The weekly time range during which system maintenance can occur,    in Universal Coordinated Time (UTC).
    ///
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    ///
    /// Default: a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.
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
    /// The number of vCPUs assigned to each elastic cluster shard. Maximum is 64. Allowed values are 2, 4, 8, 16, 32, 64.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShardCapacity")]
    pub shard_capacity: i64,

    ///
    /// The number of shards assigned to the elastic cluster. Maximum is 32.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShardCount")]
    pub shard_count: i64,

    ///
    /// The Amazon EC2 subnet IDs for the new elastic cluster.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

    ///
    /// The tags to be assigned to the new elastic cluster.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// A list of EC2 VPC security groups to associate with the new    elastic cluster.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CfnCluster {
    fn type_string(&self) -> &'static str {
        "AWS::DocDBElastic::Cluster"
    }

    fn properties(&self) -> serde_json::Value {
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
