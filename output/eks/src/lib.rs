
pub mod cfn_addon {

#[derive(serde::Serialize, Default)]
pub struct CfnAddon {
    /// PreserveOnDelete parameter value
    #[serde(rename = "PreserveOnDelete")]
    pub preserve_on_delete: Option<bool>,
    /// IAM role to bind to the add-on's service account
    #[serde(rename = "ServiceAccountRoleArn")]
    pub service_account_role_arn: Option<String>,
    /// Resolve parameter value conflicts
    #[serde(rename = "ResolveConflicts")]
    pub resolve_conflicts: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Version of Addon
    #[serde(rename = "AddonVersion")]
    pub addon_version: Option<String>,
    /// Name of Addon
    #[serde(rename = "AddonName")]
    pub addon_name: String,
    /// The configuration values to use with the add-on
    #[serde(rename = "ConfigurationValues")]
    pub configuration_values: Option<String>,
    /// Name of Cluster
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCluster {
    /// List of EncryptionConfig
    #[serde(rename = "EncryptionConfig")]
    pub encryption_config: Option<Vec<EncryptionConfig>>,
    /// An object representing the VPC configuration to use for an Amazon EKS cluster.
    #[serde(rename = "ResourcesVpcConfig")]
    pub resources_vpc_config: ResourcesVpcConfig,
    /// The unique name to give to your cluster.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The Kubernetes network configuration for the cluster.
    #[serde(rename = "KubernetesNetworkConfig")]
    pub kubernetes_network_config: Option<KubernetesNetworkConfig>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Enable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs based on log types. By default, cluster control plane logs aren't exported to CloudWatch Logs.
    #[serde(rename = "Logging")]
    pub logging: Option<Logging>,
    /// The desired Kubernetes version for your cluster. If you don't specify a value here, the latest version available in Amazon EKS is used.
    #[serde(rename = "Version")]
    pub version: Option<String>,
    /// An object representing the Outpost configuration to use for AWS EKS outpost cluster.
    #[serde(rename = "OutpostConfig")]
    pub outpost_config: Option<OutpostConfig>,
    /// The Amazon Resource Name (ARN) of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct OutpostConfig {
    #[serde(rename = "OutpostArns")]
    pub outpost_arns: Vec<String>,
    #[serde(rename = "ControlPlaneInstanceType")]
    pub control_plane_instance_type: String,
    #[serde(rename = "ControlPlanePlacement")]
    pub control_plane_placement: Option<ControlPlanePlacement>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Provider {
    #[serde(rename = "KeyArn")]
    pub key_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KubernetesNetworkConfig {
    #[serde(rename = "IpFamily")]
    pub ip_family: Option<String>,
    #[serde(rename = "ServiceIpv4Cidr")]
    pub service_ipv4_cidr: Option<String>,
    #[serde(rename = "ServiceIpv6Cidr")]
    pub service_ipv6_cidr: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EnabledTypes {

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionConfig {
    #[serde(rename = "Provider")]
    pub provider: Option<Provider>,
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourcesVpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "EndpointPrivateAccess")]
    pub endpoint_private_access: Option<bool>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "PublicAccessCidrs")]
    pub public_access_cidrs: Option<Vec<String>>,
    #[serde(rename = "EndpointPublicAccess")]
    pub endpoint_public_access: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingTypeConfig {
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterLogging {
    #[serde(rename = "EnabledTypes")]
    pub enabled_types: Option<EnabledTypes>,

}

#[derive(serde::Serialize, Default)]
pub struct Logging {
    #[serde(rename = "ClusterLogging")]
    pub cluster_logging: Option<ClusterLogging>,

}

#[derive(serde::Serialize, Default)]
pub struct ControlPlanePlacement {
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

}


}

pub mod cfn_fargate_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnFargateProfile {
    /// No documentation provided by AWS
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,
    /// Name of FargateProfile
    #[serde(rename = "FargateProfileName")]
    pub fargate_profile_name: Option<String>,
    /// The IAM policy arn for pods
    #[serde(rename = "PodExecutionRoleArn")]
    pub pod_execution_role_arn: String,
    /// Name of the Cluster
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// List of Selector
    #[serde(rename = "Selectors")]
    pub selectors: Vec<Selector>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Selector {
    #[serde(rename = "Labels")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "Namespace")]
    pub namespace: String,

}

#[derive(serde::Serialize, Default)]
pub struct Label {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_identity_provider_config {

#[derive(serde::Serialize, Default)]
pub struct CfnIdentityProviderConfig {
    /// No documentation provided by AWS
    #[serde(rename = "Oidc")]
    pub oidc: Option<OidcIdentityProviderConfig>,
    /// The name of the OIDC provider configuration.
    #[serde(rename = "IdentityProviderConfigName")]
    pub identity_provider_config_name: Option<String>,
    /// The name of the identity provider configuration.
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// The type of the identity provider configuration.
    #[serde(rename = "Type")]
    pub ty: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct OidcIdentityProviderConfig {
    #[serde(rename = "GroupsClaim")]
    pub groups_claim: Option<String>,
    #[serde(rename = "IssuerUrl")]
    pub issuer_url: String,
    #[serde(rename = "UsernamePrefix")]
    pub username_prefix: Option<String>,
    #[serde(rename = "ClientId")]
    pub client_id: String,
    #[serde(rename = "UsernameClaim")]
    pub username_claim: Option<String>,
    #[serde(rename = "GroupsPrefix")]
    pub groups_prefix: Option<String>,
    #[serde(rename = "RequiredClaims")]
    pub required_claims: Option<Vec<RequiredClaim>>,

}

#[derive(serde::Serialize, Default)]
pub struct RequiredClaim {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_nodegroup {

#[derive(serde::Serialize, Default)]
pub struct CfnNodegroup {
    /// The AMI type for your node group.
    #[serde(rename = "AmiType")]
    pub ami_type: Option<String>,
    /// Specify the instance types for a node group.
    #[serde(rename = "InstanceTypes")]
    pub instance_types: Option<Vec<String>>,
    /// The Kubernetes taints to be applied to the nodes in the node group when they are created.
    #[serde(rename = "Taints")]
    pub taints: Option<Vec<Taint>>,
    /// The node group update configuration.
    #[serde(rename = "UpdateConfig")]
    pub update_config: Option<UpdateConfig>,
    /// The AMI version of the Amazon EKS-optimized AMI to use with your node group.
    #[serde(rename = "ReleaseVersion")]
    pub release_version: Option<String>,
    /// The remote access (SSH) configuration to use with your node group.
    #[serde(rename = "RemoteAccess")]
    pub remote_access: Option<RemoteAccess>,
    /// The Amazon Resource Name (ARN) of the IAM role to associate with your node group.
    #[serde(rename = "NodeRole")]
    pub node_role: String,
    /// The capacity type of your managed node group.
    #[serde(rename = "CapacityType")]
    pub capacity_type: Option<String>,
    /// The Kubernetes labels to be applied to the nodes in the node group when they are created.
    #[serde(rename = "Labels")]
    pub labels: Option<()>,
    /// The Kubernetes version to use for your managed nodes.
    #[serde(rename = "Version")]
    pub version: Option<String>,
    /// An object representing a node group's launch template specification.
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// The metadata, as key-value pairs, to apply to the node group to assist with categorization and organization. Follows same schema as Labels for consistency.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// Force the update if the existing node group's pods are unable to be drained due to a pod disruption budget issue.
    #[serde(rename = "ForceUpdateEnabled")]
    pub force_update_enabled: Option<bool>,
    /// The root device disk size (in GiB) for your node group instances.
    #[serde(rename = "DiskSize")]
    pub disk_size: Option<usize>,
    /// The scaling configuration details for the Auto Scaling group that is created for your node group.
    #[serde(rename = "ScalingConfig")]
    pub scaling_config: Option<ScalingConfig>,
    /// The subnets to use for the Auto Scaling group that is created for your node group.
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
    /// Name of the cluster to create the node group in.
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// The unique name to give your node group.
    #[serde(rename = "NodegroupName")]
    pub nodegroup_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct UpdateConfig {
    #[serde(rename = "MaxUnavailablePercentage")]
    pub max_unavailable_percentage: Option<f64>,
    #[serde(rename = "MaxUnavailable")]
    pub max_unavailable: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct Taint {
    #[serde(rename = "Effect")]
    pub effect: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateSpecification {
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingConfig {
    #[serde(rename = "MinSize")]
    pub min_size: Option<usize>,
    #[serde(rename = "DesiredSize")]
    pub desired_size: Option<usize>,
    #[serde(rename = "MaxSize")]
    pub max_size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RemoteAccess {
    #[serde(rename = "SourceSecurityGroups")]
    pub source_security_groups: Option<Vec<String>>,
    #[serde(rename = "Ec2SshKey")]
    pub ec2_ssh_key: String,

}


}
