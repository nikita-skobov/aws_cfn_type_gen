

/// Creates an Amazon EKS control plane.
///
/// The Amazon EKS control plane consists of control plane instances that run the       Kubernetes software, such as etcd and the API server. The control plane       runs in an account managed by AWS, and the Kubernetes API is exposed by       the Amazon EKS API server endpoint. Each Amazon EKS cluster control       plane is single tenant and unique. It runs on its own set of Amazon EC2       instances.
///
/// The cluster control plane is provisioned across multiple Availability Zones and       fronted by an Elastic Load Balancing       Network Load Balancer. Amazon EKS also provisions elastic network interfaces in       your VPC subnets to provide connectivity from the control plane instances to the nodes       (for example, to support kubectl exec, logs, and         proxy data flows).
///
/// Amazon EKS nodes run in your AWS account and connect to your       cluster's control plane over the Kubernetes API server endpoint and a certificate file       that is created for your cluster.
///
/// In most cases, it takes several minutes to create a cluster. After you create an         Amazon EKS cluster, you must configure your Kubernetes tooling to       communicate with the API server and launch nodes into your cluster. For more       information, see Managing Cluster Authentication and Launching           Amazon EKS nodes in the Amazon EKS User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCluster {


    /// 
    /// The encryption configuration for the cluster.
    /// 
    /// Required: No
    ///
    /// Type: List of EncryptionConfig
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionConfig")]
    pub encryption_config: Option<Vec<EncryptionConfig>>,


    /// 
    /// The Kubernetes network configuration for the cluster.
    /// 
    /// Required: No
    ///
    /// Type: KubernetesNetworkConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "KubernetesNetworkConfig")]
    pub kubernetes_network_config: Option<KubernetesNetworkConfig>,


    /// 
    /// The logging configuration for your cluster.
    /// 
    /// Required: No
    ///
    /// Type: Logging
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    pub logging: Option<Logging>,


    /// 
    /// The unique name to give to your cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[0-9A-Za-z][A-Za-z0-9\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// An object representing the configuration of your local Amazon EKS cluster on       an AWS Outpost. This object isn't available for clusters on the AWS cloud.
    /// 
    /// Required: No
    ///
    /// Type: OutpostConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostConfig")]
    pub outpost_config: Option<OutpostConfig>,


    /// 
    /// The VPC configuration that's used by the cluster control plane. Amazon EKS VPC       resources have specific requirements to work properly with Kubernetes. For more       information, see Cluster VPC Considerations and         Cluster         Security Group Considerations in the Amazon EKS User         Guide. You must specify at least two subnets. You can specify up to five       security groups, but we recommend that you use a dedicated security group for your       cluster control plane.
    /// 
    /// ImportantUpdates require replacement of the SecurityGroupIds and           SubnetIds sub-properties.
    /// 
    /// Required: Yes
    ///
    /// Type: ResourcesVpcConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcesVpcConfig")]
    pub resources_vpc_config: ResourcesVpcConfig,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that provides permissions for the       Kubernetes control plane to make calls to AWS API operations on your       behalf. For more information, see Amazon EKS Service IAM Role in the         Amazon EKS User Guide       .
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The metadata that you apply to the cluster to assist with categorization and       organization. Each tag consists of a key and an optional value, both of which you       define. Cluster tags don't propagate to any other resources associated with the       cluster.
    /// 
    /// NoteYou must have the eks:TagResource and eks:UntagResource         permissions for your IAM principal to manage the AWS CloudFormation stack. If         you don't have these permissions, there might be unexpected behavior with         stack-level tags propagating to the resource during resource creation and         update.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The desired Kubernetes version for your cluster. If you don't specify a value here,       the default version available in Amazon EKS is used.
    /// 
    /// NoteThe default version might not be the latest version available.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,

}



impl cfn_resources::CfnResource for CfnCluster {
    fn type_string() -> &'static str {
        "AWS::EKS::Cluster"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The cluster control plane logging configuration for your cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClusterLogging {


    /// 
    /// The enabled control plane logs for your cluster. All log types are disabled if the       array is empty.
    /// 
    /// ImportantWhen updating a resource, you must include this EnabledTypes property         if the previous CloudFormation template of the resource had it.
    /// 
    /// Required: No
    ///
    /// Type: List of LoggingTypeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnabledTypes")]
    pub enabled_types: Option<Vec<LoggingTypeConfig>>,

}




/// The placement configuration for all the control plane instances of your local Amazon EKS cluster on an AWS Outpost. For more information, see         Capacity           considerations in the Amazon EKS User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ControlPlanePlacement {


    /// 
    /// The name of the placement group for the Kubernetes control plane instances.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

}




/// The encryption configuration for the cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionConfig {


    /// 
    /// The encryption provider for the cluster.
    /// 
    /// Required: No
    ///
    /// Type: Provider
    ///
    /// Update requires: Replacement
    #[serde(rename = "Provider")]
    pub provider: Option<Provider>,


    /// 
    /// Specifies the resources to be encrypted. The only supported value is "secrets".
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,

}




/// The Kubernetes network configuration for the cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KubernetesNetworkConfig {


    /// 
    /// Specify which IP family is used to assign Kubernetes pod and service IP addresses. If       you don't specify a value, ipv4 is used by default. You can only specify an       IP family when you create a cluster and can't change this value once the cluster is       created. If you specify ipv6, the VPC and subnets that you specify for       cluster creation must have both IPv4 and IPv6 CIDR blocks assigned to them. You can't       specify ipv6 for clusters in China Regions.
    /// 
    /// You can only specify ipv6 for 1.21 and later clusters that use version       1.10.1 or later of the Amazon VPC CNI add-on. If you specify ipv6, then ensure       that your VPC meets the requirements listed in the considerations listed in Assigning IPv6         addresses to pods and services in the Amazon EKS User Guide.       Kubernetes assigns services IPv6 addresses from the unique local address range       (fc00::/7). You can't specify a custom IPv6 CIDR block. Pod addresses are assigned from       the subnet's IPv6 CIDR.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ipv4 | ipv6
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpFamily")]
    pub ip_family: Option<KubernetesNetworkConfigIpFamilyEnum>,


    /// 
    /// Don't specify a value if you select ipv6 for ipFamily. The CIDR block to assign Kubernetes service IP addresses from.       If you don't specify a block, Kubernetes assigns addresses from either the 10.100.0.0/16       or 172.20.0.0/16 CIDR blocks. We recommend that you specify a block that does not       overlap with resources in other networks that are peered or connected to your VPC. The       block must meet the following requirements:
    /// 
    /// Within one of the following private IP address blocks: 10.0.0.0/8,           172.16.0.0/12, or 192.168.0.0/16.               Doesn't overlap with any CIDR block assigned to the VPC that you selected for           VPC.               Between /24 and /12.
    /// 
    /// ImportantYou can only specify a custom CIDR block when you create a cluster and can't         change this value once the cluster is created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceIpv4Cidr")]
    pub service_ipv4_cidr: Option<String>,


    /// 
    /// The CIDR block that Kubernetes pod and service IP addresses are assigned from if you       created a 1.21 or later cluster with version 1.10.1 or later of the Amazon VPC CNI add-on and       specified ipv6 for ipFamily when you       created the cluster. Kubernetes assigns service addresses from the unique local address       range (fc00::/7) because you can't specify a custom IPv6 CIDR block when       you create the cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceIpv6Cidr")]
    pub service_ipv6_cidr: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum KubernetesNetworkConfigIpFamilyEnum {

    /// ipv4
    #[serde(rename = "ipv4")]
    Ipv4,

    /// ipv6
    #[serde(rename = "ipv6")]
    Ipv6,

}

impl Default for KubernetesNetworkConfigIpFamilyEnum {
    fn default() -> Self {
        KubernetesNetworkConfigIpFamilyEnum::Ipv4
    }
}



/// Enable or disable exporting the Kubernetes control plane logs for your cluster to       CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch       Logs. For more information, see Amazon EKS Cluster control plane         logs in the         Amazon EKS User Guide       .
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Logging {


    /// 
    /// The cluster control plane logging configuration for your cluster.
    /// 
    /// Required: No
    ///
    /// Type: ClusterLogging
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterLogging")]
    pub cluster_logging: Option<ClusterLogging>,

}




/// The enabled logging type. For a list of the valid logging types, see the types property of LogSetup in the           Amazon EKS API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingTypeConfig {


    /// 
    /// The name of the log type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}




/// The configuration of your local Amazon EKS cluster on an AWS       Outpost. Before creating a cluster on an Outpost, review Creating a local         cluster on an Outpost in the Amazon EKS User Guide. This API isn't available for         Amazon EKS clusters on the AWS cloud.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutpostConfig {


    /// 
    /// The Amazon EC2 instance type that you want to use for your local Amazon EKS cluster on Outposts. Choose an instance type based on the number of nodes       that your cluster will have. For more information, see Capacity         considerations in the Amazon EKS User Guide.
    /// 
    /// The instance type that you specify is used for all Kubernetes control plane instances. The       instance type can't be changed after cluster creation. The control plane is not       automatically scaled by Amazon EKS.
    /// 
    /// 
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ControlPlaneInstanceType")]
    pub control_plane_instance_type: String,


    /// 
    /// An object representing the placement configuration for all the control plane instances       of your local Amazon EKS cluster on an AWS Outpost. For more       information, see Capacity considerations in the Amazon EKS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: ControlPlanePlacement
    ///
    /// Update requires: Replacement
    #[serde(rename = "ControlPlanePlacement")]
    pub control_plane_placement: Option<ControlPlanePlacement>,


    /// 
    /// The ARN of the Outpost that you want to use for your local Amazon EKS       cluster on Outposts. Only a single Outpost ARN is supported.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostArns")]
    pub outpost_arns: Vec<String>,

}




/// Identifies the AWS Key Management Service (AWS KMS) key used to encrypt the       secrets.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Provider {


    /// 
    /// Amazon Resource Name (ARN) or alias of the KMS key. The KMS key must be       symmetric and created in the same AWS Region as the cluster. If the         KMS key was created in a different account, the IAM principal must       have access to the KMS key. For more information, see Allowing         users in other accounts to use a KMS key in the                   AWS Key Management Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyArn")]
    pub key_arn: Option<String>,

}




/// An object representing the VPC configuration to use for an Amazon EKS cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourcesVpcConfig {


    /// 
    /// Set this value to true to enable private access for your cluster's       Kubernetes API server endpoint. If you enable private access, Kubernetes API requests       from within your cluster's VPC use the private VPC endpoint. The default value for this       parameter is false, which disables private access for your Kubernetes API       server. If you disable private access and you have nodes or AWS Fargate       pods in the cluster, then ensure that publicAccessCidrs includes the       necessary CIDR blocks for communication with the nodes or Fargate pods.       For more information, see Amazon EKS cluster endpoint access control in       the         Amazon EKS User Guide       .
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointPrivateAccess")]
    pub endpoint_private_access: Option<bool>,


    /// 
    /// Set this value to false to disable public access to your cluster's       Kubernetes API server endpoint. If you disable public access, your cluster's Kubernetes       API server can only receive requests from within the cluster VPC. The default value for       this parameter is true, which enables public access for your Kubernetes API       server. For more information, see Amazon EKS cluster endpoint access control in the                 Amazon EKS User Guide       .
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointPublicAccess")]
    pub endpoint_public_access: Option<bool>,


    /// 
    /// The CIDR blocks that are allowed access to your cluster's public Kubernetes API server       endpoint. Communication to the endpoint from addresses outside of the CIDR blocks that       you specify is denied. The default value is 0.0.0.0/0. If you've disabled       private endpoint access and you have nodes or AWS Fargate pods in the       cluster, then ensure that you specify the necessary CIDR blocks. For more information,       see Amazon EKS cluster endpoint access control in the                 Amazon EKS User Guide       .
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicAccessCidrs")]
    pub public_access_cidrs: Option<Vec<String>>,


    /// 
    /// Specify one or more security groups for the cross-account elastic network interfaces       that Amazon EKS creates to use that allow communication between your nodes and       the Kubernetes control plane. If you don't specify any security groups, then familiarize       yourself with the difference between Amazon EKS defaults for clusters deployed       with Kubernetes. For more information, see Amazon EKS security group         considerations in the         Amazon EKS User Guide       .
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// Specify subnets for your Amazon EKS nodes. Amazon EKS creates       cross-account elastic network interfaces in these subnets to allow communication between       your nodes and the Kubernetes control plane.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

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


