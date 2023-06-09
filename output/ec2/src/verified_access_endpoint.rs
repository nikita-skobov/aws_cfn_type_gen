/// An AWS Verified Access endpoint specifies the application that AWS Verified Access provides access to. It must be     attached to an AWS Verified Access group. An AWS Verified Access endpoint must also have an attached access policy     before you attached it to a group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnVerifiedAccessEndpoint {
    ///
    /// The DNS name for users to reach your application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationDomain")]
    pub application_domain: cfn_resources::StrVal,

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
    pub attachment_type: VerifiedAccessEndpointAttachmentTypeEnum,

    ///
    /// A description for the AWS Verified Access endpoint.
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
    /// The ARN of a public TLS/SSL certificate imported into or created with ACM.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainCertificateArn")]
    pub domain_certificate_arn: cfn_resources::StrVal,

    ///
    /// A custom identifier that is prepended to the DNS name that is generated for the     endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointDomainPrefix")]
    pub endpoint_domain_prefix: cfn_resources::StrVal,

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
    pub endpoint_type: VerifiedAccessEndpointEndpointTypeEnum,

    ///
    /// The load balancer details if creating the AWS Verified Access endpoint as     load-balancertype.
    ///
    /// Required: No
    ///
    /// Type: LoadBalancerOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_options: Option<LoadBalancerOptions>,

    ///
    /// The options for network-interface type endpoint.
    ///
    /// Required: No
    ///
    /// Type: NetworkInterfaceOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_options: Option<NetworkInterfaceOptions>,

    ///
    /// The Verified Access policy document.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<cfn_resources::StrVal>,

    ///
    /// The status of the Verified Access policy.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_enabled: Option<bool>,

    ///
    /// The IDs of the security groups for the endpoint.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// The tags.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the AWS Verified Access group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerifiedAccessGroupId")]
    pub verified_access_group_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnVerifiedAccessEndpointcreationtime,

    #[serde(skip_serializing)]
    pub att_device_validation_domain: CfnVerifiedAccessEndpointdevicevalidationdomain,

    #[serde(skip_serializing)]
    pub att_endpoint_domain: CfnVerifiedAccessEndpointendpointdomain,

    #[serde(skip_serializing)]
    pub att_last_updated_time: CfnVerifiedAccessEndpointlastupdatedtime,

    #[serde(skip_serializing)]
    pub att_status: CfnVerifiedAccessEndpointstatus,

    #[serde(skip_serializing)]
    pub att_verified_access_endpoint_id: CfnVerifiedAccessEndpointverifiedaccessendpointid,

    #[serde(skip_serializing)]
    pub att_verified_access_instance_id: CfnVerifiedAccessEndpointverifiedaccessinstanceid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VerifiedAccessEndpointAttachmentTypeEnum {
    /// vpc
    #[serde(rename = "vpc")]
    Vpc,
}

impl Default for VerifiedAccessEndpointAttachmentTypeEnum {
    fn default() -> Self {
        VerifiedAccessEndpointAttachmentTypeEnum::Vpc
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VerifiedAccessEndpointEndpointTypeEnum {
    /// load-balancer
    #[serde(rename = "load-balancer")]
    Loadbalancer,

    /// network-interface
    #[serde(rename = "network-interface")]
    Networkinterface,
}

impl Default for VerifiedAccessEndpointEndpointTypeEnum {
    fn default() -> Self {
        VerifiedAccessEndpointEndpointTypeEnum::Loadbalancer
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessEndpointcreationtime;
impl CfnVerifiedAccessEndpointcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessEndpointdevicevalidationdomain;
impl CfnVerifiedAccessEndpointdevicevalidationdomain {
    pub fn att_name(&self) -> &'static str {
        r#"DeviceValidationDomain"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessEndpointendpointdomain;
impl CfnVerifiedAccessEndpointendpointdomain {
    pub fn att_name(&self) -> &'static str {
        r#"EndpointDomain"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessEndpointlastupdatedtime;
impl CfnVerifiedAccessEndpointlastupdatedtime {
    pub fn att_name(&self) -> &'static str {
        r#"LastUpdatedTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessEndpointstatus;
impl CfnVerifiedAccessEndpointstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessEndpointverifiedaccessendpointid;
impl CfnVerifiedAccessEndpointverifiedaccessendpointid {
    pub fn att_name(&self) -> &'static str {
        r#"VerifiedAccessEndpointId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVerifiedAccessEndpointverifiedaccessinstanceid;
impl CfnVerifiedAccessEndpointverifiedaccessinstanceid {
    pub fn att_name(&self) -> &'static str {
        r#"VerifiedAccessInstanceId"#
    }
}

impl cfn_resources::CfnResource for CfnVerifiedAccessEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VerifiedAccessEndpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.load_balancer_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.network_interface_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the load balancer options when creating an AWS Verified Access endpoint using the       load-balancer type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LoadBalancerOptions {
    ///
    /// The ARN of the load balancer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoadBalancerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<LoadBalancerOptionsProtocolEnum>,

    ///
    /// The IDs of the subnets.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum LoadBalancerOptionsProtocolEnum {
    /// http
    #[serde(rename = "http")]
    Http,

    /// https
    #[serde(rename = "https")]
    Https,
}

impl Default for LoadBalancerOptionsProtocolEnum {
    fn default() -> Self {
        LoadBalancerOptionsProtocolEnum::Http
    }
}

impl cfn_resources::CfnResource for LoadBalancerOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.port {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'port'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Describes the network interface options when creating an AWS Verified Access endpoint using the       network-interface type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NetworkInterfaceOptions {
    ///
    /// The ID of the network interface.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<NetworkInterfaceOptionsProtocolEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NetworkInterfaceOptionsProtocolEnum {
    /// http
    #[serde(rename = "http")]
    Http,

    /// https
    #[serde(rename = "https")]
    Https,
}

impl Default for NetworkInterfaceOptionsProtocolEnum {
    fn default() -> Self {
        NetworkInterfaceOptionsProtocolEnum::Http
    }
}

impl cfn_resources::CfnResource for NetworkInterfaceOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.port {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'port'. {} is less than 1",
                    the_val
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
