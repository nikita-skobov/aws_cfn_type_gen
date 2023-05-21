

/// Creates a managed node group for an Amazon EKS cluster. You can only create a       node group for your cluster that is equal to the current Kubernetes version for the       cluster. All node groups are created with the latest AMI release version for the       respective minor Kubernetes version of the cluster, unless you deploy a custom AMI using       a launch template. For more information about using launch templates, see Launch         template support.
///
/// An Amazon EKS managed node group is an Amazon EC2       Auto Scaling group and associated Amazon EC2 instances that are managed by         AWS for an Amazon EKS cluster. For more information, see         Managed node groups in the Amazon EKS User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNodegroup {


    /// 
    /// The AMI type for your node group. If you specify launchTemplate, and your launch template uses a custom AMI,         then don't specify amiType, or the node group deployment       will fail. If your launch template uses a Windows custom AMI, then add         eks:kube-proxy-windows to your Windows nodes rolearn in       the aws-auth       ConfigMap. For more information about using launch templates with Amazon EKS, see Launch template support in the Amazon EKS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AL2_ARM_64 | AL2_x86_64 | AL2_x86_64_GPU | BOTTLEROCKET_ARM_64 | BOTTLEROCKET_ARM_64_NVIDIA | BOTTLEROCKET_x86_64 | BOTTLEROCKET_x86_64_NVIDIA | CUSTOM | WINDOWS_CORE_2019_x86_64 | WINDOWS_CORE_2022_x86_64 | WINDOWS_FULL_2019_x86_64 | WINDOWS_FULL_2022_x86_64
    ///
    /// Update requires: Replacement
    #[serde(rename = "AmiType")]
    pub ami_type: Option<NodegroupAmiTypeEnum>,


    /// 
    /// The capacity type of your managed node group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ON_DEMAND | SPOT
    ///
    /// Update requires: Replacement
    #[serde(rename = "CapacityType")]
    pub capacity_type: Option<NodegroupCapacityTypeEnum>,


    /// 
    /// The name of the cluster to create the node group in.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,


    /// 
    /// The root device disk size (in GiB) for your node group instances. The default disk       size is 20 GiB for Linux and Bottlerocket. The default disk size is 50 GiB for Windows.       If you specify launchTemplate, then don't specify diskSize, or the node group       deployment will fail. For more information about using launch templates with Amazon EKS, see Launch template support in the Amazon EKS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "DiskSize")]
    pub disk_size: Option<i64>,


    /// 
    /// Force the update if the existing node group's pods are unable to be drained due to a       pod disruption budget issue. If an update fails because pods could not be drained, you       can force the update after it fails to terminate the old node whether or not any pods       are running on the node.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForceUpdateEnabled")]
    pub force_update_enabled: Option<bool>,


    /// 
    /// Specify the instance types for a node group. If you specify a GPU instance type, make       sure to also specify an applicable GPU AMI type with the amiType parameter.       If you specify launchTemplate, then you can specify zero or one instance       type in your launch template or you can specify 0-20 instance types       for instanceTypes. If however, you specify an instance type in your launch       template and specify any instanceTypes, the node group       deployment will fail. If you don't specify an instance type in a launch template or for         instanceTypes, then t3.medium is used, by default. If you       specify Spot for capacityType, then we recommend specifying       multiple values for instanceTypes. For more information, see Managed node group capacity types and Launch template support in       the Amazon EKS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceTypes")]
    pub instance_types: Option<Vec<String>>,


    /// 
    /// The Kubernetes labels applied to the nodes in the node group.
    /// 
    /// NoteOnly labels that are applied with the Amazon EKS API are shown here. There         may be other Kubernetes labels applied to the nodes in this group.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,


    /// 
    /// An object representing a node group's launch template specification. If specified,       then do not specify instanceTypes, diskSize, or         remoteAccess and make sure that the launch template meets the       requirements in launchTemplateSpecification.
    /// 
    /// Required: No
    ///
    /// Type: LaunchTemplateSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplateSpecification>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role to associate with your node group. The         Amazon EKS worker node kubelet daemon makes calls to AWS APIs on your behalf. Nodes receive permissions for these API calls       through an IAM instance profile and associated policies. Before you can       launch nodes and register them into a cluster, you must create an IAM       role for those nodes to use when they are launched. For more information, see Amazon EKS node IAM role in the                 Amazon EKS User Guide       . If you specify launchTemplate, then don't specify         IamInstanceProfile in your launch template,       or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see Launch template support in the Amazon EKS User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NodeRole")]
    pub node_role: String,


    /// 
    /// The unique name to give your node group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NodegroupName")]
    pub nodegroup_name: Option<String>,


    /// 
    /// The AMI version of the Amazon EKS optimized AMI to use with your node group       (for example, 1.14.7-YYYYMMDD). By default, the latest       available AMI version for the node group's current Kubernetes version is used. For more       information, see Amazon EKS optimized         Linux AMI Versions in the Amazon EKS User       Guide.
    /// 
    /// NoteChanging this value triggers an update of the node group if one is available. You         can't update other properties at the same time as updating Release           Version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReleaseVersion")]
    pub release_version: Option<String>,


    /// 
    /// The remote access configuration to use with your node group.       For Linux, the protocol is SSH. For Windows, the protocol is RDP.       If you specify launchTemplate, then don't specify         remoteAccess, or the node group deployment will fail.       For more information about using launch templates with Amazon EKS, see Launch template support in the Amazon EKS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: RemoteAccess
    ///
    /// Update requires: Replacement
    #[serde(rename = "RemoteAccess")]
    pub remote_access: Option<RemoteAccess>,


    /// 
    /// The scaling configuration details for the Auto Scaling group that is created for your       node group.
    /// 
    /// Required: No
    ///
    /// Type: ScalingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingConfig")]
    pub scaling_config: Option<ScalingConfig>,


    /// 
    /// The subnets to use for the Auto Scaling group that is created for your node group.       If you specify launchTemplate, then don't specify SubnetId in your launch template, or the node group       deployment will fail. For more information about using launch templates with Amazon EKS, see Launch template support in the Amazon EKS User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,


    /// 
    /// The metadata applied to the node group to assist with categorization and organization.       Each tag consists of a key and an optional value. You define both. Node group tags do       not propagate to any other resources associated with the node group, such as the Amazon EC2 instances or subnets.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The Kubernetes taints to be applied to the nodes in the node group when they are       created. Effect is one of No_Schedule, Prefer_No_Schedule, or         No_Execute. Kubernetes taints can be used together with tolerations to       control how workloads are scheduled to your nodes. For more information, see Node taints on managed node groups.
    /// 
    /// Required: No
    ///
    /// Type: List of Taint
    ///
    /// Update requires: No interruption
    #[serde(rename = "Taints")]
    pub taints: Option<Vec<Taint>>,


    /// 
    /// The node group update configuration.
    /// 
    /// Required: No
    ///
    /// Type: UpdateConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateConfig")]
    pub update_config: Option<UpdateConfig>,


    /// 
    /// The Kubernetes version to use for your managed nodes. By default, the Kubernetes       version of the cluster is used, and this is the only accepted specified value. If you       specify launchTemplate, and your launch template uses a custom AMI, then       don't specify version, or the node group deployment will fail. For more       information about using launch templates with Amazon EKS, see Launch         template support in the Amazon EKS User Guide.
    /// 
    /// NoteYou can't update other properties at the same time as updating           Version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum NodegroupCapacityTypeEnum {

    /// ON_DEMAND
    #[serde(rename = "ON_DEMAND")]
    Ondemand,

    /// SPOT
    #[serde(rename = "SPOT")]
    Spot,

}

impl Default for NodegroupCapacityTypeEnum {
    fn default() -> Self {
        NodegroupCapacityTypeEnum::Ondemand
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NodegroupAmiTypeEnum {

    /// AL2_ARM_64
    #[serde(rename = "AL2_ARM_64")]
    Al2arm64,

    /// AL2_x86_64
    #[serde(rename = "AL2_x86_64")]
    Al2x8664,

    /// AL2_x86_64_GPU
    #[serde(rename = "AL2_x86_64_GPU")]
    Al2x8664gpu,

    /// BOTTLEROCKET_ARM_64
    #[serde(rename = "BOTTLEROCKET_ARM_64")]
    Bottlerocketarm64,

    /// BOTTLEROCKET_ARM_64_NVIDIA
    #[serde(rename = "BOTTLEROCKET_ARM_64_NVIDIA")]
    Bottlerocketarm64nvidia,

    /// BOTTLEROCKET_x86_64
    #[serde(rename = "BOTTLEROCKET_x86_64")]
    Bottlerocketx8664,

    /// BOTTLEROCKET_x86_64_NVIDIA
    #[serde(rename = "BOTTLEROCKET_x86_64_NVIDIA")]
    Bottlerocketx8664nvidia,

    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// WINDOWS_CORE_2019_x86_64
    #[serde(rename = "WINDOWS_CORE_2019_x86_64")]
    Windowscore2019x8664,

    /// WINDOWS_CORE_2022_x86_64
    #[serde(rename = "WINDOWS_CORE_2022_x86_64")]
    Windowscore2022x8664,

    /// WINDOWS_FULL_2019_x86_64
    #[serde(rename = "WINDOWS_FULL_2019_x86_64")]
    Windowsfull2019x8664,

    /// WINDOWS_FULL_2022_x86_64
    #[serde(rename = "WINDOWS_FULL_2022_x86_64")]
    Windowsfull2022x8664,

}

impl Default for NodegroupAmiTypeEnum {
    fn default() -> Self {
        NodegroupAmiTypeEnum::Al2arm64
    }
}


impl cfn_resources::CfnResource for CfnNodegroup {
    fn type_string() -> &'static str {
        "AWS::EKS::Nodegroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// An object representing a node group launch template specification. The launch template       can't include SubnetId, IamInstanceProfile, RequestSpotInstances, HibernationOptions, or TerminateInstances, or the node group deployment or       update will fail. For more information about launch templates, see CreateLaunchTemplate in the Amazon EC2 API       Reference. For more information about using launch templates with Amazon EKS, see Launch template support in the Amazon EKS User Guide.
///
/// You must specify either the launch template ID or the launch template name in the       request, but not both.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchTemplateSpecification {


    /// 
    /// The ID of the launch template.
    /// 
    /// You must specify either the launch template ID or the launch template name in the       request, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: Option<String>,


    /// 
    /// The name of the launch template.
    /// 
    /// You must specify either the launch template name or the launch template ID in the       request, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The version number of the launch template to use. If no version is specified, then the       template's default version is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,

}




/// An object representing the remote access configuration for the managed node       group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RemoteAccess {


    /// 
    /// The Amazon EC2 SSH key name that provides access for SSH communication with       the nodes in the managed node group. For more information, see Amazon EC2 key pairs and Linux instances in the Amazon Elastic Compute Cloud User Guide for Linux Instances. For       Windows, an Amazon EC2 SSH key is used to obtain the RDP password. For more       information, see Amazon EC2 key pairs and Windows instances in       the Amazon Elastic Compute Cloud User Guide for Windows Instances.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ec2SshKey")]
    pub ec2_ssh_key: String,


    /// 
    /// The security group IDs that are allowed SSH access (port 22) to the nodes. For       Windows, the port is 3389. If you specify an Amazon EC2 SSH key but don't       specify a source security group when you create a managed node group, then the port on       the nodes is opened to the internet (0.0.0.0/0). For more information, see         Security Groups for Your VPC in the Amazon Virtual Private Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceSecurityGroups")]
    pub source_security_groups: Option<Vec<String>>,

}




/// An object representing the scaling configuration details for the Auto Scaling group       that is associated with your node group. When creating a node group, you must specify       all or none of the properties. When updating a node group, you can specify any or none       of the properties.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScalingConfig {


    /// 
    /// The current number of nodes that the managed node group should maintain.
    /// 
    /// ImportantIf you use Cluster Autoscaler, you shouldn't change the desiredSize value         directly, as this can cause the Cluster Autoscaler to suddenly scale up or scale         down.
    /// 
    /// Whenever this parameter changes, the number of worker nodes in the node group is       updated to the specified size. If this parameter is given a value that is smaller than       the current number of running worker nodes, the necessary number of worker nodes are       terminated to match the given value.             When using CloudFormation, no action occurs if you remove this parameter from your CFN       template.
    /// 
    /// This parameter can be different from minSize in some cases, such as when starting with       extra hosts for testing. This parameter can also be different when you want to start       with an estimated number of needed hosts, but let Cluster Autoscaler reduce the number       if there are too many. When Cluster Autoscaler is used, the desiredSize parameter is       altered by Cluster Autoscaler (but can be out-of-date for short periods of time).       Cluster Autoscaler doesn't scale a managed node group lower than minSize or higher than       maxSize.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredSize")]
    pub desired_size: Option<i64>,


    /// 
    /// The maximum number of nodes that the managed node group can scale out to. For       information about the maximum number that you can specify, see Amazon EKS service quotas in the Amazon EKS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSize")]
    pub max_size: Option<i64>,


    /// 
    /// The minimum number of nodes that the managed node group can scale in to.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinSize")]
    pub min_size: Option<i64>,

}




/// A property that allows a node to repel a set of pods. For more information, see Node taints on managed node groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Taint {


    /// 
    /// The effect of the taint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NO_EXECUTE | NO_SCHEDULE | PREFER_NO_SCHEDULE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Effect")]
    pub effect: Option<TaintEffectEnum>,


    /// 
    /// The key of the taint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The value of the taint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 63
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TaintEffectEnum {

    /// NO_EXECUTE
    #[serde(rename = "NO_EXECUTE")]
    Noexecute,

    /// NO_SCHEDULE
    #[serde(rename = "NO_SCHEDULE")]
    Noschedule,

    /// PREFER_NO_SCHEDULE
    #[serde(rename = "PREFER_NO_SCHEDULE")]
    Prefernoschedule,

}

impl Default for TaintEffectEnum {
    fn default() -> Self {
        TaintEffectEnum::Noexecute
    }
}



/// The update configuration for the node group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfig {


    /// 
    /// The maximum number of nodes unavailable at once during a version update. Nodes will be       updated in parallel. This value or maxUnavailablePercentage is required to       have a value.The maximum number is 100.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxUnavailable")]
    pub max_unavailable: Option<f64>,


    /// 
    /// The maximum percentage of nodes unavailable during a version update. This percentage       of nodes will be updated in parallel, up to 100 nodes at once. This value or         maxUnavailable is required to have a value.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxUnavailablePercentage")]
    pub max_unavailable_percentage: Option<f64>,

}


