

/// The AWS::AppStream::ImageBuilder resource creates an image builder for Amazon AppStream 2.0. An image builder is a virtual machine that is used to create an image.
///
/// The initial state of the image builder is PENDING. When it is ready, the state is RUNNING.
#[derive(Default, serde::Serialize)]
pub struct CfnImageBuilder {


    /// 
    /// The description to display.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The ARN of the public, private, or shared image to use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws(?:\-cn|\-iso\-b|\-iso|\-us\-gov)?:[A-Za-z0-9][A-Za-z0-9_/.-]{0,62}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9][A-Za-z0-9:_/+=,@.\\-]{0,1023}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageArn")]
    pub image_arn: Option<String>,


    /// 
    /// The image builder name to display.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,


    /// 
    /// Enables or disables default internet access for the image builder.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDefaultInternetAccess")]
    pub enable_default_internet_access: Option<bool>,


    /// 
    /// An array of key-value pairs.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The name of the image used to create the image builder.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageName")]
    pub image_name: Option<String>,


    /// 
    /// The name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain.
    /// 
    /// Required: No
    ///
    /// Type: DomainJoinInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainJoinInfo")]
    pub domain_join_info: Option<DomainJoinInfo>,


    /// 
    /// The list of virtual private cloud (VPC) interface endpoint objects. Administrators can connect to the image builder only through the specified endpoints.
    /// 
    /// Required: No
    ///
    /// Type: List of AccessEndpoint
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessEndpoints")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,


    /// 
    /// The version of the AppStream 2.0 agent to use for this image builder. To use the latest version of the AppStream 2.0 agent, specify [LATEST].
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppstreamAgentVersion")]
    pub appstream_agent_version: Option<String>,


    /// 
    /// The VPC configuration for the image builder. You can specify only one subnet.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,


    /// 
    /// The instance type to use when launching the image builder. The following instance types are available:
    /// 
    /// stream.standard.small               stream.standard.medium               stream.standard.large               stream.compute.large               stream.compute.xlarge               stream.compute.2xlarge               stream.compute.4xlarge               stream.compute.8xlarge               stream.memory.large               stream.memory.xlarge               stream.memory.2xlarge               stream.memory.4xlarge               stream.memory.8xlarge               stream.memory.z1d.large               stream.memory.z1d.xlarge               stream.memory.z1d.2xlarge               stream.memory.z1d.3xlarge               stream.memory.z1d.6xlarge               stream.memory.z1d.12xlarge               stream.graphics-design.large               stream.graphics-design.xlarge               stream.graphics-design.2xlarge               stream.graphics-design.4xlarge               stream.graphics-desktop.2xlarge               stream.graphics.g4dn.xlarge               stream.graphics.g4dn.2xlarge               stream.graphics.g4dn.4xlarge               stream.graphics.g4dn.8xlarge               stream.graphics.g4dn.12xlarge               stream.graphics.g4dn.16xlarge               stream.graphics-pro.4xlarge               stream.graphics-pro.8xlarge               stream.graphics-pro.16xlarge
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: String,


    /// 
    /// The ARN of the IAM role that is applied to the image builder. To assume a role, the image builder calls the AWS Security Token Service AssumeRole API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the appstream_machine_role credential profile on the instance.
    /// 
    /// For more information, see Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances in the Amazon AppStream 2.0 Administration Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws(?:\-cn|\-iso\-b|\-iso|\-us\-gov)?:[A-Za-z0-9][A-Za-z0-9_/.-]{0,62}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9][A-Za-z0-9:_/+=,@.\\-]{0,1023}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: Option<String>,


    /// 
    /// A unique name for the image builder.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,100}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// The name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain.
#[derive(Default, serde::Serialize)]
pub struct DomainJoinInfo {


    /// 
    /// The fully qualified name of the directory (for example, corp.example.com).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<String>,


    /// 
    /// The distinguished name of the organizational unit for computer accounts.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2000
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    pub organizational_unit_distinguished_name: Option<String>,

}


/// The VPC configuration for the image builder.
#[derive(Default, serde::Serialize)]
pub struct VpcConfig {


    /// 
    /// The identifiers of the security groups for the image builder.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The identifier of the subnet to which a network interface is attached from the image builder instance. An image builder instance can use one subnet.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

}


/// Describes an interface VPC endpoint (interface endpoint) that lets you create a private connection between the virtual private cloud (VPC) that you specify and AppStream 2.0. When you specify an interface endpoint for a stack, users of the stack can connect to AppStream 2.0 only through that endpoint. When you specify an interface endpoint for an image builder, administrators can connect to the image builder only through that endpoint.
#[derive(Default, serde::Serialize)]
pub struct AccessEndpoint {


    /// 
    /// The identifier (ID) of the VPC in which the interface endpoint is used.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpceId")]
    pub vpce_id: String,


    /// 
    /// The type of interface endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: STREAMING
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}
