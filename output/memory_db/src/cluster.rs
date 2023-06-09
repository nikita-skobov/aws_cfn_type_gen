/// Specifies a cluster. All nodes in the cluster run the same     protocol-compliant engine software.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnCluster {
    ///
    /// The name of the Access Control List to associate with the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Pattern: [a-zA-Z][a-zA-Z0-9\-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ACLName")]
    pub aclname: cfn_resources::StrVal,

    ///
    /// When set to true, the cluster will automatically receive minor engine     version upgrades after launch.
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
    /// The cluster's configuration endpoint.
    ///
    /// Required: No
    ///
    /// Type: Endpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<Endpoint>,

    ///
    /// The name of the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: cfn_resources::StrVal,

    /// Enables data tiering. Data tiering is only supported for replication groups using the r6gd node type. This parameter must be set to true when using r6gd nodes. For more information, see Data tiering.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataTiering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tiering: Option<cfn_resources::StrVal>,

    ///
    /// A description of the cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The Redis engine version used by the cluster.
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
    /// The user-supplied name of a final cluster snapshot. This is the unique     name that identifies the snapshot. MemoryDB creates the snapshot, and then     deletes the cluster immediately afterward.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FinalSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_snapshot_name: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the KMS key used to encrypt the cluster.
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
    /// Specifies the weekly time range during which maintenance on the cluster     is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi     (24H Clock UTC). The minimum maintenance window is a 60 minute period.
    ///
    /// Pattern: ddd:hh24:mi-ddd:hh24:mi
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<cfn_resources::StrVal>,

    ///
    /// The cluster's node type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeType")]
    pub node_type: cfn_resources::StrVal,

    ///
    /// The number of replicas to apply to each shard.
    ///
    /// Default value: 1
    ///
    /// Maximum value: 5
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumReplicasPerShard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_replicas_per_shard: Option<i64>,

    ///
    /// The number of shards in the cluster.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumShards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_shards: Option<i64>,

    ///
    /// The name of the parameter group used by the cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The port used by the cluster.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// A list of security group names to associate with this cluster.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of Amazon Resource Names (ARN) that uniquely identify the RDB snapshot files     stored in Amazon S3. The snapshot files are used to populate the new cluster. The Amazon S3 object name in the ARN cannot contain any commas.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arns: Option<Vec<String>>,

    ///
    /// The name of a snapshot from which to restore data into the new cluster.     The snapshot status changes to restoring while the new cluster is being     created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<cfn_resources::StrVal>,

    ///
    /// The number of days for which MemoryDB retains automatic snapshots before     deleting them. For example, if you set SnapshotRetentionLimit to 5, a snapshot that was     taken today is retained for 5 days before being deleted.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i64>,

    ///
    /// The daily time range (in UTC) during which MemoryDB begins taking a daily     snapshot of your shard. Example: 05:00-09:00 If you do not specify this parameter, MemoryDB automatically chooses an appropriate time range.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<cfn_resources::StrVal>,

    ///
    /// When you pass the logical ID of this resource to the intrinsic Ref function, Ref returns the ARN of the SNS topic,      such as arn:aws:memorydb:us-east-1:123456789012:mySNSTopic
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<cfn_resources::StrVal>,

    ///
    /// The SNS topic must be in Active status to receive notifications.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_status: Option<cfn_resources::StrVal>,

    ///
    /// The name of the subnet group used by the cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<cfn_resources::StrVal>,

    ///
    /// A flag to indicate if In-transit encryption is enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "TLSEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tlsenabled: Option<bool>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
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
    pub att_arn: CfnClusterarn,

    #[serde(skip_serializing)]
    pub att_cluster_endpoint_address: CfnClusterclusterendpointaddress,

    #[serde(skip_serializing)]
    pub att_parameter_group_status: CfnClusterparametergroupstatus,

    #[serde(skip_serializing)]
    pub att_status: CfnClusterstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnClusterarn;
impl CfnClusterarn {
    pub fn att_name(&self) -> &'static str {
        r#"ARN"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnClusterclusterendpointaddress;
impl CfnClusterclusterendpointaddress {
    pub fn att_name(&self) -> &'static str {
        r#"ClusterEndpoint.Address"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnClusterparametergroupstatus;
impl CfnClusterparametergroupstatus {
    pub fn att_name(&self) -> &'static str {
        r#"ParameterGroupStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnClusterstatus;
impl CfnClusterstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnCluster {
    fn type_string(&self) -> &'static str {
        "AWS::MemoryDB::Cluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.aclname;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'aclname'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.cluster_endpoint
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents the information required for client programs to connect to the cluster and     its nodes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Endpoint {
    ///
    /// The DNS hostname of the node.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<cfn_resources::StrVal>,

    ///
    /// The port number that the engine is listening on.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

impl cfn_resources::CfnResource for Endpoint {
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
