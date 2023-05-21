

/// The AWS::EMR::Studio resource specifies an Amazon EMR Studio. An EMR Studio is a web-based, integrated development environment for fully managed Jupyter notebooks that run on Amazon EMR clusters. For more information, see the Amazon EMR Management Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnStudio {


    /// 
    /// The ID of the Workspace security group associated with the Amazon EMR Studio.     The Workspace security group allows outbound network traffic to resources in the Engine     security group and to the internet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkspaceSecurityGroupId")]
    pub workspace_security_group_id: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that will be assumed by the Amazon EMR Studio. The service role provides a     way for Amazon EMR Studio to interoperate with other AWS services.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceRole")]
    pub service_role: String,


    /// 
    /// The Amazon S3 location to back up EMR Studio Workspaces and notebook files.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultS3Location")]
    pub default_s3_location: String,


    /// 
    /// Specifies whether the Studio authenticates users using IAM Identity Center or IAM.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: IAM | SSO
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthMode")]
    pub auth_mode: String,


    /// 
    /// A list of subnet IDs to associate with the Amazon EMR Studio. A Studio can have     a maximum of 5 subnets. The subnets must belong to the VPC specified by VpcId.     Studio users can create a Workspace in any of the specified subnets.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


    /// 
    /// The ID of the Amazon Virtual Private Cloud (Amazon VPC) to associate with the     Studio.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,


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
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A detailed description of the Amazon EMR Studio.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of your identity provider's RelayState parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdpRelayStateParameterName")]
    pub idp_relay_state_parameter_name: Option<String>,


    /// 
    /// A descriptive name for the Amazon EMR Studio.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM user role that will be assumed by users and groups logged in to a Studio. The     permissions attached to this IAM role can be scoped down for each user or group using     session policies. You only need to specify UserRole when you set AuthMode to SSO.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserRole")]
    pub user_role: Option<String>,


    /// 
    /// The ID of the Amazon EMR Studio Engine security group. The Engine security group     allows inbound network traffic from the Workspace security group, and it must be in the     same VPC specified by VpcId.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineSecurityGroupId")]
    pub engine_security_group_id: String,


    /// 
    /// Your identity provider's authentication endpoint. Amazon EMR Studio redirects     federated users to this endpoint for authentication when logging in to a Studio with the     Studio URL.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdpAuthUrl")]
    pub idp_auth_url: Option<String>,

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
