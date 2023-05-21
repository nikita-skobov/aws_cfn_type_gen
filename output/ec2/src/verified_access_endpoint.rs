

/// An AWS Verified Access endpoint specifies the application that AWS Verified Access provides access to. It must be     attached to an AWS Verified Access group. An AWS Verified Access endpoint must also have an attached access policy     before you attached it to a group.
#[derive(Default, serde::Serialize)]
pub struct CfnVerifiedAccessEndpoint {


    /// 
    /// The options for network-interface type endpoint.
    /// 
    /// Required: No
    ///
    /// Type: NetworkInterfaceOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceOptions")]
    pub network_interface_options: Option<NetworkInterfaceOptions>,


    /// 
    /// The ARN of a public TLS/SSL certificate imported into or created with ACM.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainCertificateArn")]
    pub domain_certificate_arn: String,


    /// 
    /// The status of the Verified Access policy.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyEnabled")]
    pub policy_enabled: Option<bool>,


    /// 
    /// A custom identifier that is prepended to the DNS name that is generated for the     endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointDomainPrefix")]
    pub endpoint_domain_prefix: String,


    /// 
    /// The type of attachment used to provide connectivity between the AWS Verified Access endpoint and the     application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: vpc
    ///
    /// Update requires: Replacement
    #[serde(rename = "AttachmentType")]
    pub attachment_type: String,


    /// 
    /// The Verified Access policy document.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: Option<String>,


    /// 
    /// The load balancer details if creating the AWS Verified Access endpoint as     load-balancertype.
    /// 
    /// Required: No
    ///
    /// Type: LoadBalancerOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerOptions")]
    pub load_balancer_options: Option<LoadBalancerOptions>,


    /// 
    /// The tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The DNS name for users to reach your application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationDomain")]
    pub application_domain: String,


    /// 
    /// A description for the AWS Verified Access endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The ID of the AWS Verified Access group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerifiedAccessGroupId")]
    pub verified_access_group_id: String,


    /// 
    /// The type of AWS Verified Access endpoint. Incoming application requests will be sent to an IP     address, load balancer or a network interface depending on the endpoint type     specified.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: load-balancer | network-interface
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,


    /// 
    /// The IDs of the security groups for the endpoint.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}


/// Describes the load balancer options when creating an AWS Verified Access endpoint using the       load-balancer type.
#[derive(Default, serde::Serialize)]
pub struct LoadBalancerOptions {


    /// 
    /// The IDs of the subnets.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,


    /// 
    /// The IP protocol.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: http | https
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,


    /// 
    /// The ARN of the load balancer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoadBalancerArn")]
    pub load_balancer_arn: Option<String>,


    /// 
    /// The IP port number.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,

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


/// Describes the network interface options when creating an AWS Verified Access endpoint using the       network-interface type.
#[derive(Default, serde::Serialize)]
pub struct NetworkInterfaceOptions {


    /// 
    /// The IP port number.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The ID of the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,


    /// 
    /// The IP protocol.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: http | https
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

}