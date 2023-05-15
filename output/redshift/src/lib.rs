
pub mod cfn_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCluster {
    /// The version of the Amazon Redshift engine software that you want to deploy on the cluster.The version selected runs on all the nodes in the cluster.
    #[serde(rename = "ClusterVersion")]
    pub cluster_version: Option<String>,
    /// The number of days to retain newly copied snapshots in the destination AWS Region after they are copied from the source AWS Region. If the value is -1, the manual snapshot is retained indefinitely.
    /// 
    /// The value must be either -1 or an integer between 1 and 3,653.
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    pub manual_snapshot_retention_period: Option<usize>,
    /// The number of days that automated snapshots are retained. If the value is 0, automated snapshots are disabled. Default value is 1
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    pub automated_snapshot_retention_period: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "LoggingProperties")]
    pub logging_properties: Option<LoggingProperties>,
    /// The destination AWS Region that you want to copy snapshots to. Constraints: Must be the name of a valid AWS Region. For more information, see Regions and Endpoints in the Amazon Web Services [https://docs.aws.amazon.com/general/latest/gr/rande.html#redshift_region] General Reference
    #[serde(rename = "DestinationRegion")]
    pub destination_region: Option<String>,
    /// A unique identifier for the cluster. You use this identifier to refer to the cluster for any subsequent cluster operations such as deleting or modifying. All alphabetical characters must be lower case, no hypens at the end, no two consecutive hyphens. Cluster name should be unique for all clusters within an AWS account
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: Option<String>,
    /// The name of the snapshot from which to create the new cluster. This parameter isn't case sensitive.
    #[serde(rename = "SnapshotIdentifier")]
    pub snapshot_identifier: Option<String>,
    /// The number of days to retain automated snapshots in the destination region after they are copied from the source region.
    /// 
    /// Default is 7.
    /// 
    /// Constraints: Must be at least 1 and no more than 35.
    #[serde(rename = "SnapshotCopyRetentionPeriod")]
    pub snapshot_copy_retention_period: Option<usize>,
    /// The value represents how the cluster is configured to use AQUA (Advanced Query Accelerator) after the cluster is restored. Possible values include the following.
    /// 
    /// enabled - Use AQUA if it is available for the current Region and Amazon Redshift node type.
    /// disabled - Don't use AQUA.
    /// auto - Amazon Redshift determines whether to use AQUA.
    #[serde(rename = "AquaConfigurationStatus")]
    pub aqua_configuration_status: Option<String>,
    /// The AWS Key Management Service (KMS) key ID of the encryption key that you want to use to encrypt data in the cluster.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// An option that specifies whether to create the cluster with enhanced VPC routing enabled. To create a cluster that uses enhanced VPC routing, the cluster must be in a VPC. For more information, see Enhanced VPC Routing in the Amazon Redshift Cluster Management Guide.
    /// 
    /// If this option is true , enhanced VPC routing is enabled.
    /// 
    /// Default: false
    #[serde(rename = "EnhancedVpcRouting")]
    pub enhanced_vpc_routing: Option<bool>,
    /// An integer indicating the duration of the maintenance window in days. If you specify a duration, you can't specify an end time. The duration must be 45 days or less.
    #[serde(rename = "DeferMaintenanceDuration")]
    pub defer_maintenance_duration: Option<usize>,
    /// The node type to be provisioned for the cluster.Valid Values: ds2.xlarge | ds2.8xlarge | dc1.large | dc1.8xlarge | dc2.large | dc2.8xlarge | ra3.4xlarge | ra3.16xlarge
    #[serde(rename = "NodeType")]
    pub node_type: String,
    /// The name of the parameter group to be associated with this cluster.
    #[serde(rename = "ClusterParameterGroupName")]
    pub cluster_parameter_group_name: Option<String>,
    /// The number of compute nodes in the cluster. This parameter is required when the ClusterType parameter is specified as multi-node.
    #[serde(rename = "NumberOfNodes")]
    pub number_of_nodes: Option<usize>,
    /// A timestamp indicating end time for the deferred maintenance window. If you specify an end time, you can't specify a duration.
    #[serde(rename = "DeferMaintenanceEndTime")]
    pub defer_maintenance_end_time: Option<String>,
    /// Specifies the name of the HSM configuration that contains the information the Amazon Redshift cluster can use to retrieve and store keys in an HSM.
    #[serde(rename = "HsmConfigurationIdentifier")]
    pub hsm_configuration_identifier: Option<String>,
    /// The type of the cluster. When cluster type is specified as single-node, the NumberOfNodes parameter is not required and if multi-node, the NumberOfNodes parameter is required
    #[serde(rename = "ClusterType")]
    pub cluster_type: String,
    /// Indicates whether to apply the snapshot retention period to newly copied manual snapshots instead of automated snapshots.
    #[serde(rename = "SnapshotCopyManual")]
    pub snapshot_copy_manual: Option<bool>,
    /// If true, the cluster can be accessed from a public network.
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,
    /// The name for the maintenance track that you want to assign for the cluster. This name change is asynchronous. The new track name stays in the PendingModifiedValues for the cluster until the next maintenance window. When the maintenance track changes, the cluster is switched to the latest cluster release available for the maintenance track. At this point, the maintenance track name is applied.
    #[serde(rename = "MaintenanceTrackName")]
    pub maintenance_track_name: Option<String>,
    /// A timestamp indicating the start time for the deferred maintenance window.
    #[serde(rename = "DeferMaintenanceStartTime")]
    pub defer_maintenance_start_time: Option<String>,
    /// The identifier of the database revision. You can retrieve this value from the response to the DescribeClusterDbRevisions request.
    #[serde(rename = "RevisionTarget")]
    pub revision_target: Option<String>,
    /// The name of the first database to be created when the cluster is created. To create additional databases after the cluster is created, connect to the cluster with a SQL client and use SQL commands to create a database.
    #[serde(rename = "DBName")]
    pub dbname: String,
    /// The availability zone relocation status of the cluster
    #[serde(rename = "AvailabilityZoneRelocationStatus")]
    pub availability_zone_relocation_status: Option<String>,
    /// A boolean indicating whether to enable the deferred maintenance window.
    #[serde(rename = "DeferMaintenance")]
    pub defer_maintenance: Option<bool>,
    /// A list of security groups to be associated with this cluster.
    #[serde(rename = "ClusterSecurityGroups")]
    pub cluster_security_groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "OwnerAccount")]
    pub owner_account: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<Endpoint>,
    /// A list of AWS Identity and Access Management (IAM) roles that can be used by the cluster to access other AWS services. You must supply the IAM roles in their Amazon Resource Name (ARN) format. You can supply up to 50 IAM roles in a single request
    #[serde(rename = "IamRoles")]
    pub iam_roles: Option<Vec<String>>,
    /// The list of tags for the cluster parameter group.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the snapshot copy grant to use when snapshots of an AWS KMS-encrypted cluster are copied to the destination region.
    #[serde(rename = "SnapshotCopyGrantName")]
    pub snapshot_copy_grant_name: Option<String>,
    /// Major version upgrades can be applied during the maintenance window to the Amazon Redshift engine that is running on the cluster. Default value is True
    #[serde(rename = "AllowVersionUpgrade")]
    pub allow_version_upgrade: Option<bool>,
    /// If true, the data in the cluster is encrypted at rest.
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    /// The option to enable relocation for an Amazon Redshift cluster between Availability Zones after the cluster modification is complete.
    #[serde(rename = "AvailabilityZoneRelocation")]
    pub availability_zone_relocation: Option<bool>,
    /// A boolean value indicating whether the resize operation is using the classic resize process. If you don't provide this parameter or set the value to false , the resize type is elastic.
    #[serde(rename = "Classic")]
    pub classic: Option<bool>,
    /// The user name associated with the master user account for the cluster that is being created. The user name can't be PUBLIC and first character must be a letter.
    #[serde(rename = "MasterUsername")]
    pub master_username: String,
    /// The weekly time range (in UTC) during which automated cluster maintenance can occur.
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// The name of a cluster subnet group to be associated with this cluster.
    #[serde(rename = "ClusterSubnetGroupName")]
    pub cluster_subnet_group_name: Option<String>,
    /// The Redshift operation to be performed. Resource Action supports pause-cluster, resume-cluster APIs
    #[serde(rename = "ResourceAction")]
    pub resource_action: Option<String>,
    /// The password associated with the master user account for the cluster that is being created. Password must be between 8 and 64 characters in length, should have at least one uppercase letter.Must contain at least one lowercase letter.Must contain one number.Can be any printable ASCII character.
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: String,
    /// The EC2 Availability Zone (AZ) in which you want Amazon Redshift to provision the cluster. Default: A random, system-chosen Availability Zone in the region that is specified by the endpoint
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// The name of the cluster the source snapshot was created from. This parameter is required if your IAM user has a policy containing a snapshot resource element that specifies anything other than * for the cluster name.
    #[serde(rename = "SnapshotClusterIdentifier")]
    pub snapshot_cluster_identifier: Option<String>,
    /// A boolean indicating if we want to rotate Encryption Keys.
    #[serde(rename = "RotateEncryptionKey")]
    pub rotate_encryption_key: Option<bool>,
    /// The Elastic IP (EIP) address for the cluster.
    #[serde(rename = "ElasticIp")]
    pub elastic_ip: Option<String>,
    /// A list of Virtual Private Cloud (VPC) security groups to be associated with the cluster.
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// Specifies the name of the HSM client certificate the Amazon Redshift cluster uses to retrieve the data encryption keys stored in an HSM
    #[serde(rename = "HsmClientCertificateIdentifier")]
    pub hsm_client_certificate_identifier: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Endpoint {
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingProperties {
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,

}


}

pub mod cfn_cluster_parameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnClusterParameterGroup {
    /// The name of the cluster parameter group.
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: Option<String>,
    /// The Amazon Redshift engine version to which the cluster parameter group applies. The cluster engine version determines the set of parameters.
    #[serde(rename = "ParameterGroupFamily")]
    pub parameter_group_family: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// An array of parameters to be modified. A maximum of 20 parameters can be modified in a single request.
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<Parameter>>,
    /// A description of the parameter group.
    #[serde(rename = "Description")]
    pub description: String,

}


#[derive(serde::Serialize, Default)]
pub struct Parameter {
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_cluster_security_group {

#[derive(serde::Serialize, Default)]
pub struct CfnClusterSecurityGroup {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_cluster_security_group_ingress {

#[derive(serde::Serialize, Default)]
pub struct CfnClusterSecurityGroupIngress {
    /// No documentation provided by AWS
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CIDRIP")]
    pub cidrip: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EC2SecurityGroupName")]
    pub ec2_security_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterSecurityGroupName")]
    pub cluster_security_group_name: String,

}



}

pub mod cfn_cluster_subnet_group {

#[derive(serde::Serialize, Default)]
pub struct CfnClusterSubnetGroup {
    /// The list of VPC subnet IDs
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// The description of the parameter group.
    #[serde(rename = "Description")]
    pub description: String,
    /// The list of tags for the cluster parameter group.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_endpoint_access {

#[derive(serde::Serialize, Default)]
pub struct CfnEndpointAccess {
    /// A list of vpc security group ids to apply to the created endpoint access.
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Vec<String>,
    /// The name of the endpoint.
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// The subnet group name where Amazon Redshift chooses to deploy the endpoint.
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: String,
    /// The AWS account ID of the owner of the cluster.
    #[serde(rename = "ResourceOwner")]
    pub resource_owner: Option<String>,
    /// A unique identifier for the cluster. You use this identifier to refer to the cluster for any subsequent cluster operations such as deleting or modifying. All alphabetical characters must be lower case, no hypens at the end, no two consecutive hyphens. Cluster name should be unique for all clusters within an AWS account
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,

}


#[derive(serde::Serialize, Default)]
pub struct VpcSecurityGroup {
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "VpcSecurityGroupId")]
    pub vpc_security_group_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkInterface {
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,

}


}

pub mod cfn_endpoint_authorization {

#[derive(serde::Serialize, Default)]
pub struct CfnEndpointAuthorization {
    /// The target AWS account ID to grant or revoke access for.
    #[serde(rename = "Account")]
    pub account: AwsAccount,
    /// The virtual private cloud (VPC) identifiers to grant or revoke access to.
    #[serde(rename = "VpcIds")]
    pub vpc_ids: Option<Vec<VpcId>>,
    /// Indicates whether to force the revoke action. If true, the Redshift-managed VPC endpoints associated with the endpoint authorization are also deleted.
    #[serde(rename = "Force")]
    pub force: Option<bool>,
    /// The cluster identifier.
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,

}

pub type AwsAccount = String;pub type VpcId = String;

}

pub mod cfn_event_subscription {

#[derive(serde::Serialize, Default)]
pub struct CfnEventSubscription {
    /// A list of one or more identifiers of Amazon Redshift source objects.
    #[serde(rename = "SourceIds")]
    pub source_ids: Option<Vec<String>>,
    /// The Amazon Resource Name (ARN) of the Amazon SNS topic used to transmit the event notifications.
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: Option<String>,
    /// Specifies the Amazon Redshift event categories to be published by the event notification subscription.
    #[serde(rename = "EventCategories")]
    pub event_categories: Option<Vec<String>>,
    /// Specifies the Amazon Redshift event severity to be published by the event notification subscription.
    #[serde(rename = "Severity")]
    pub severity: Option<String>,
    /// The type of source that will be generating the events.
    #[serde(rename = "SourceType")]
    pub source_type: Option<String>,
    /// A boolean value; set to true to activate the subscription, and set to false to create the subscription but not activate it.
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the Amazon Redshift event notification subscription
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_scheduled_action {

#[derive(serde::Serialize, Default)]
pub struct CfnScheduledAction {
    /// If true, the schedule is enabled. If false, the scheduled action does not trigger.
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,
    /// The start time in UTC of the scheduled action. Before this time, the scheduled action does not trigger.
    #[serde(rename = "StartTime")]
    pub start_time: Option<timestamp>,
    /// The description of the scheduled action.
    #[serde(rename = "ScheduledActionDescription")]
    pub scheduled_action_description: Option<String>,
    /// The IAM role to assume to run the target action.
    #[serde(rename = "IamRole")]
    pub iam_role: Option<String>,
    /// The end time in UTC of the scheduled action. After this time, the scheduled action does not trigger.
    #[serde(rename = "EndTime")]
    pub end_time: Option<timestamp>,
    /// A JSON format string of the Amazon Redshift API operation with input parameters.
    #[serde(rename = "TargetAction")]
    pub target_action: Option<ScheduledActionType>,
    /// The schedule in `at( )` or `cron( )` format.
    #[serde(rename = "Schedule")]
    pub schedule: Option<String>,
    /// The name of the scheduled action. The name must be unique within an account.
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct PauseClusterMessage {
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,

}
pub type timestamp = String;
#[derive(serde::Serialize, Default)]
pub struct ResizeClusterMessage {
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,
    #[serde(rename = "NumberOfNodes")]
    pub number_of_nodes: Option<usize>,
    #[serde(rename = "ClusterType")]
    pub cluster_type: Option<String>,
    #[serde(rename = "Classic")]
    pub classic: Option<bool>,
    #[serde(rename = "NodeType")]
    pub node_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ResumeClusterMessage {
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct ScheduledActionType {

}


}
