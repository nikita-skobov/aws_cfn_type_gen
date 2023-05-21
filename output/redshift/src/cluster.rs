/// Specifies a cluster. A cluster is a fully managed data warehouse       that consists of a set of compute nodes.
///
/// To create a cluster in Virtual Private Cloud (VPC), you must provide a cluster subnet       group name. The cluster subnet group identifies the subnets of your VPC that Amazon       Redshift uses when creating the cluster. For more information about managing clusters,       go to Amazon Redshift Clusters in the Amazon Redshift Cluster         Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCluster {
    ///
    /// If true, major version upgrades can be applied during the maintenance       window to the Amazon Redshift engine that is running on the cluster.
    ///
    /// When a new major version of the Amazon Redshift engine is released, you can request that       the service automatically apply upgrades during the maintenance window to the Amazon Redshift       engine that is running on your cluster.
    ///
    /// Default: true
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowVersionUpgrade")]
    pub allow_version_upgrade: Option<bool>,

    ///
    /// This parameter is retired. It does not set the AQUA configuration status. Amazon Redshift automatically determines whether to use AQUA (Advanced Query Accelerator).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: auto | disabled | enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "AquaConfigurationStatus")]
    pub aqua_configuration_status: Option<ClusterAquaConfigurationStatusEnum>,

    ///
    /// The number of days that automated snapshots are retained. If the value is 0, automated       snapshots are disabled. Even if automated snapshots are disabled, you can still create       manual snapshots when you want with CreateClusterSnapshot in the Amazon Redshift API         Reference.
    ///
    /// Default: 1
    ///
    /// Constraints: Must be a value from 0 to 35.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    pub automated_snapshot_retention_period: Option<i64>,

    ///
    /// The EC2 Availability Zone (AZ) in which you want Amazon Redshift to provision the       cluster. For example, if you have several EC2 instances running in a specific       Availability Zone, then you might want the cluster to be provisioned in the same zone in       order to decrease network latency.
    ///
    /// Default: A random, system-chosen Availability Zone in the region that is specified       by the endpoint.
    ///
    /// Example: us-east-2d
    ///
    /// Constraint: The specified Availability Zone must be in the same region as the       current endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,

    ///
    /// The option to enable relocation for an Amazon Redshift cluster between Availability Zones after the cluster is created.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZoneRelocation")]
    pub availability_zone_relocation: Option<bool>,

    ///
    /// Describes the status of the Availability Zone relocation operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZoneRelocationStatus")]
    pub availability_zone_relocation_status: Option<String>,

    ///
    /// A boolean value indicating whether the resize operation is using the classic resize       process. If you don't provide this parameter or set the value to       false, the resize type is elastic.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Classic")]
    pub classic: Option<bool>,

    ///
    /// A unique identifier for the cluster. You use this identifier to refer to the       cluster for any subsequent cluster operations such as deleting or modifying. The       identifier also appears in the Amazon Redshift console.
    ///
    /// Constraints:
    ///
    /// Must contain from 1 to 63 alphanumeric characters or hyphens.               Alphabetic characters must be lowercase.               First character must be a letter.               Cannot end with a hyphen or contain two consecutive hyphens.               Must be unique for all clusters within an AWS account.
    ///
    /// Example: myexamplecluster
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: Option<String>,

    ///
    /// The name of the parameter group to be associated with this cluster.
    ///
    /// Default: The default Amazon Redshift cluster parameter group. For information about the       default parameter group, go to Working with Amazon         Redshift Parameter Groups
    ///
    /// Constraints:
    ///
    /// Must be 1 to 255 alphanumeric characters or hyphens.               First character must be a letter.               Cannot end with a hyphen or contain two consecutive hyphens.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterParameterGroupName")]
    pub cluster_parameter_group_name: Option<String>,

    ///
    /// A list of security groups to be associated with this cluster.
    ///
    /// Default: The default cluster security group for Amazon Redshift.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterSecurityGroups")]
    pub cluster_security_groups: Option<Vec<String>>,

    ///
    /// The name of a cluster subnet group to be associated with this cluster.
    ///
    /// If this parameter is not provided the resulting cluster will be deployed outside       virtual private cloud (VPC).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterSubnetGroupName")]
    pub cluster_subnet_group_name: Option<String>,

    ///
    /// The type of the cluster. When cluster type is specified as
    ///
    /// single-node, the NumberOfNodes           parameter is not required.                        multi-node, the NumberOfNodes           parameter is required.
    ///
    /// Valid Values: multi-node | single-node
    ///
    /// Default: multi-node
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterType")]
    pub cluster_type: ClusterClusterTypeEnum,

    ///
    /// The version of the Amazon Redshift engine software that you want to deploy on the       cluster.
    ///
    /// The version selected runs on all the nodes in the cluster.
    ///
    /// Constraints: Only version 1.0 is currently available.
    ///
    /// Example: 1.0
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterVersion")]
    pub cluster_version: Option<String>,

    ///
    /// The name of the first database to be created when the cluster is created.
    ///
    /// To create additional databases after the cluster is created, connect to the cluster       with a SQL client and use SQL commands to create a database. For more information, go to         Create         a Database in the Amazon Redshift Database Developer Guide.
    ///
    /// Default: dev
    ///
    /// Constraints:
    ///
    /// Must contain 1 to 64 alphanumeric characters.               Must contain only lowercase letters.               Cannot be a word that is reserved by the service. A list of reserved words           can be found in Reserved Words in the           Amazon Redshift Database Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBName")]
    pub dbname: String,

    ///
    /// A Boolean indicating whether to enable the deferred maintenance window.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeferMaintenance")]
    pub defer_maintenance: Option<bool>,

    ///
    /// An integer indicating the duration of the maintenance window in days. If you specify a duration, you can't specify an end time.       The duration must be 45 days or less.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeferMaintenanceDuration")]
    pub defer_maintenance_duration: Option<i64>,

    ///
    /// A timestamp for the end of the time period when we defer maintenance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeferMaintenanceEndTime")]
    pub defer_maintenance_end_time: Option<String>,

    ///
    /// A timestamp indicating the start time for the deferred maintenance window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeferMaintenanceStartTime")]
    pub defer_maintenance_start_time: Option<String>,

    ///
    /// The destination region that snapshots are automatically copied to when cross-region       snapshot copy is enabled.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationRegion")]
    pub destination_region: Option<String>,

    ///
    /// The Elastic IP (EIP) address for the cluster.
    ///
    /// Constraints: The cluster must be provisioned in EC2-VPC and publicly-accessible       through an Internet gateway. Don't specify the Elastic IP address for a publicly accessible       cluster with availability zone relocation turned on. For more information about provisioning clusters in       EC2-VPC, go to Supported         Platforms to Launch Your Cluster in the Amazon Redshift Cluster Management Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticIp")]
    pub elastic_ip: Option<String>,

    ///
    /// If true, the data in the cluster is encrypted at rest.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,

    ///
    /// The connection endpoint.
    ///
    /// Required: No
    ///
    /// Type: Endpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<Endpoint>,

    ///
    /// An option that specifies whether to create the cluster with enhanced VPC routing       enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a       VPC. For more information, see Enhanced VPC Routing in       the Amazon Redshift Cluster Management Guide.
    ///
    /// If this option is true, enhanced VPC routing is enabled.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnhancedVpcRouting")]
    pub enhanced_vpc_routing: Option<bool>,

    ///
    /// Specifies the name of the HSM client certificate the Amazon Redshift cluster uses to       retrieve the data encryption keys stored in an HSM.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "HsmClientCertificateIdentifier")]
    pub hsm_client_certificate_identifier: Option<String>,

    ///
    /// Specifies the name of the HSM configuration that contains the information the       Amazon Redshift cluster can use to retrieve and store keys in an HSM.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "HsmConfigurationIdentifier")]
    pub hsm_configuration_identifier: Option<String>,

    ///
    /// A list of AWS Identity and Access Management (IAM) roles that can be used by the       cluster to access other AWS services. You must supply the IAM roles in their Amazon       Resource Name (ARN) format.
    ///
    /// The maximum number of IAM roles that you can associate is subject to a quota.       For more information, go to Quotas and limits       in the Amazon Redshift Cluster Management Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRoles")]
    pub iam_roles: Option<Vec<String>>,

    ///
    /// The AWS Key Management Service (KMS) key ID of the encryption key that you want to       use to encrypt data in the cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

    ///
    /// Specifies logging information, such as queries and connection attempts, for the       specified Amazon Redshift cluster.
    ///
    /// Required: No
    ///
    /// Type: LoggingProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingProperties")]
    pub logging_properties: Option<LoggingProperties>,

    ///
    /// An optional parameter for the name of the maintenance track for the cluster. If you       don't provide a maintenance track name, the cluster is assigned to the         current track.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceTrackName")]
    pub maintenance_track_name: Option<String>,

    ///
    /// The default number of days to retain a manual snapshot. If the value is -1, the       snapshot is retained indefinitely. This setting doesn't change the retention period       of existing snapshots.
    ///
    /// The value must be either -1 or an integer between 1 and 3,653.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    pub manual_snapshot_retention_period: Option<i64>,

    ///
    /// The password associated with the admin user for the cluster that is being       created.
    ///
    /// Constraints:
    ///
    /// Must be between 8 and 64 characters in length.               Must contain at least one uppercase letter.               Must contain at least one lowercase letter.               Must contain one number.               Can be any printable ASCII character (ASCII code 33-126) except '           (single quote), " (double quote), \, /, or @.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: String,

    ///
    /// The user name associated with the admin user for the cluster that is being       created.
    ///
    /// Constraints:
    ///
    /// Must be 1 - 128 alphanumeric characters or hyphens. The user name can't be             PUBLIC.               Must contain only lowercase letters, numbers, underscore, plus sign, period (dot), at symbol (@), or hyphen.               The first character must be a letter.               Must not contain a colon (:) or a slash (/).               Cannot be a reserved word. A list of reserved words can be found in Reserved             Words in the Amazon Redshift Database Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "MasterUsername")]
    pub master_username: String,

    ///
    /// The node type to be provisioned for the cluster. For information about node types,       go to Working with         Clusters in the Amazon Redshift Cluster Management Guide.
    ///
    /// Valid Values: ds2.xlarge | ds2.8xlarge |         dc1.large | dc1.8xlarge |         dc2.large | dc2.8xlarge |         ra3.xlplus | ra3.4xlarge | ra3.16xlarge
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeType")]
    pub node_type: ClusterNodeTypeEnum,

    ///
    /// The number of compute nodes in the cluster. This parameter is required when the         ClusterType parameter is specified as         multi-node.
    ///
    /// For information about determining how many nodes you need, go to Working with         Clusters in the Amazon Redshift Cluster Management Guide.
    ///
    /// If you don't specify this parameter, you get a single-node cluster. When requesting       a multi-node cluster, you must specify the number of nodes that you want in the       cluster.
    ///
    /// Default: 1
    ///
    /// Constraints: Value must be at least 1 and no more than 100.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfNodes")]
    pub number_of_nodes: Option<i64>,

    ///
    /// The AWS account used to create or copy the snapshot. Required if you are       restoring a snapshot you do not own, optional if you own the snapshot.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerAccount")]
    pub owner_account: Option<String>,

    ///
    /// The port number on which the cluster accepts incoming connections.
    ///
    /// The cluster is accessible only via the JDBC and ODBC connection strings. Part of       the connection string requires the port on which the cluster will listen for incoming       connections.
    ///
    /// Default: 5439
    ///
    /// Valid Values: 1150-65535
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,

    ///
    /// The weekly time range (in UTC) during which automated cluster maintenance can       occur.
    ///
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    ///
    /// Default: A 30-minute window selected at random from an 8-hour block of time per       region, occurring on a random day of the week. For more information about the time       blocks for each region, see Maintenance Windows in Amazon Redshift Cluster Management Guide.
    ///
    /// Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun
    ///
    /// Constraints: Minimum 30-minute window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,

    ///
    /// If true, the cluster can be accessed from a public network.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,

    ///
    /// The Amazon Redshift operation to be performed. Supported operations are pause-cluster and     resume-cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceAction")]
    pub resource_action: Option<String>,

    ///
    /// Describes a RevisionTarget object.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RevisionTarget")]
    pub revision_target: Option<String>,

    ///
    /// Rotates the encryption keys for a cluster.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotateEncryptionKey")]
    pub rotate_encryption_key: Option<bool>,

    ///
    /// The name of the cluster the source snapshot was created from. This parameter is       required if your user or role has a policy containing a snapshot resource element that       specifies anything other than * for the cluster name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotClusterIdentifier")]
    pub snapshot_cluster_identifier: Option<String>,

    ///
    /// The name of the snapshot copy grant.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotCopyGrantName")]
    pub snapshot_copy_grant_name: Option<String>,

    ///
    /// Indicates whether to apply the snapshot retention period to newly copied manual       snapshots instead of automated snapshots.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotCopyManual")]
    pub snapshot_copy_manual: Option<bool>,

    ///
    /// The number of days to retain automated snapshots in the destination AWS Region       after they are copied from the source AWS Region.
    ///
    /// By default, this only changes the retention period of copied automated snapshots.
    ///
    /// If you decrease the retention period for automated snapshots that are copied to a       destination AWS Region, Amazon Redshift deletes any existing automated snapshots that were       copied to the destination AWS Region and that fall outside of the new retention       period.
    ///
    /// Constraints: Must be at least 1 and no more than 35 for automated snapshots.
    ///
    /// If you specify the manual option, only newly copied manual snapshots will       have the new retention period.
    ///
    /// If you specify the value of -1 newly copied manual snapshots are retained       indefinitely.
    ///
    /// Constraints: The number of days must be either -1 or an integer between 1 and 3,653       for manual snapshots.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotCopyRetentionPeriod")]
    pub snapshot_copy_retention_period: Option<i64>,

    ///
    /// The name of the snapshot from which to create the new cluster. This parameter isn't       case sensitive. You must specify this parameter or snapshotArn, but not both.
    ///
    /// Example: my-snapshot-id
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotIdentifier")]
    pub snapshot_identifier: Option<String>,

    ///
    /// A list of tag instances.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// A list of Virtual Private Cloud (VPC) security groups to be associated with the       cluster.
    ///
    /// Default: The default VPC security group is associated with the cluster.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ClusterAquaConfigurationStatusEnum {
    /// auto
    #[serde(rename = "auto")]
    Auto,

    /// disabled
    #[serde(rename = "disabled")]
    Disabled,

    /// enabled
    #[serde(rename = "enabled")]
    Enabled,
}

impl Default for ClusterAquaConfigurationStatusEnum {
    fn default() -> Self {
        ClusterAquaConfigurationStatusEnum::Auto
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ClusterClusterTypeEnum {
    /// multi-node
    #[serde(rename = "multi-node")]
    Multinode,

    /// single-node
    #[serde(rename = "single-node")]
    Singlenode,
}

impl Default for ClusterClusterTypeEnum {
    fn default() -> Self {
        ClusterClusterTypeEnum::Multinode
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ClusterNodeTypeEnum {
    /// ds2.xlarge
    #[serde(rename = "ds2.xlarge")]
    Ds2xlarge,

    /// ds2.8xlarge
    #[serde(rename = "ds2.8xlarge")]
    Ds28xlarge,

    /// dc1.large
    #[serde(rename = "dc1.large")]
    Dc1large,

    /// dc1.8xlarge
    #[serde(rename = "dc1.8xlarge")]
    Dc18xlarge,

    /// dc2.large
    #[serde(rename = "dc2.large")]
    Dc2large,

    /// dc2.8xlarge
    #[serde(rename = "dc2.8xlarge")]
    Dc28xlarge,

    /// ra3.xlplus
    #[serde(rename = "ra3.xlplus")]
    Ra3xlplus,

    /// ra3.4xlarge
    #[serde(rename = "ra3.4xlarge")]
    Ra34xlarge,

    /// ra3.16xlarge
    #[serde(rename = "ra3.16xlarge")]
    Ra316xlarge,
}

impl Default for ClusterNodeTypeEnum {
    fn default() -> Self {
        ClusterNodeTypeEnum::Ds2xlarge
    }
}

impl cfn_resources::CfnResource for CfnCluster {
    fn type_string(&self) -> &'static str {
        "AWS::Redshift::Cluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.availability_zone {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'availability_zone'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.availability_zone_relocation_status {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'availability_zone_relocation_status'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.cluster_identifier {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'cluster_identifier'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.cluster_parameter_group_name {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'cluster_parameter_group_name'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.cluster_subnet_group_name {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'cluster_subnet_group_name'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.cluster_version {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'cluster_version'. {} is greater than 2147483647", the_val.len()));
            }
        }

        let the_val = &self.dbname;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'dbname'. {} is greater than 2147483647",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.destination_region {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'destination_region'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.elastic_ip {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'elastic_ip'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        self.endpoint
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.hsm_client_certificate_identifier {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'hsm_client_certificate_identifier'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.hsm_configuration_identifier {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'hsm_configuration_identifier'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'kms_key_id'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        self.logging_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.maintenance_track_name {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'maintenance_track_name'. {} is greater than 2147483647", the_val.len()));
            }
        }

        let the_val = &self.master_user_password;

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'master_user_password'. {} is greater than 2147483647", the_val.len()));
        }

        let the_val = &self.master_username;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'master_username'. {} is greater than 2147483647",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.owner_account {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'owner_account'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.preferred_maintenance_window {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'preferred_maintenance_window'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.snapshot_cluster_identifier {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'snapshot_cluster_identifier'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.snapshot_copy_grant_name {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'snapshot_copy_grant_name'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.snapshot_identifier {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'snapshot_identifier'. {} is greater than 2147483647", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Describes a connection endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Endpoint {
    ///
    /// The DNS address of the cluster. This property is read only.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<String>,

    ///
    /// The port that the database engine is listening on. This property is read only.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<String>,
}

impl cfn_resources::CfnResource for Endpoint {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.address {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'address'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies logging information, such as queries and connection attempts, for the       specified Amazon Redshift cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingProperties {
    ///
    /// The name of an existing S3 bucket where the log files are to be stored.
    ///
    /// Constraints:
    ///
    /// Must be in the same region as the cluster               The cluster must have read bucket and put object permissions
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

    ///
    /// The prefix applied to the log file names.
    ///
    /// Constraints:
    ///
    /// Cannot exceed 512 characters               Cannot contain spaces( ), double quotes ("), single quotes ('), a backslash           (\), or control characters. The hexadecimal codes for invalid characters are:                                                                              x00 to x20                     x22                     x27                     x5c                     x7f or larger
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,
}

impl cfn_resources::CfnResource for LoggingProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket_name;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'bucket_name'. {} is greater than 2147483647",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.s3_key_prefix {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_key_prefix'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

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
