/// The AWS::AppStream::ImageBuilder resource creates an image builder for Amazon AppStream 2.0. An image builder is a virtual machine that is used to create an image.
///
/// The initial state of the image builder is PENDING. When it is ready, the state is RUNNING.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnImageBuilder {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstream_agent_version: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain.
    ///
    /// Required: No
    ///
    /// Type: DomainJoinInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainJoinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_info: Option<DomainJoinInfo>,

    ///
    /// Enables or disables default internet access for the image builder.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDefaultInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_internet_access: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<cfn_resources::StrVal>,

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
    pub instance_type: cfn_resources::StrVal,

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
    pub name: cfn_resources::StrVal,

    ///
    /// An array of key-value pairs.
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
    /// The VPC configuration for the image builder. You can specify only one subnet.
    ///
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,

    #[serde(skip_serializing)]
    pub att_streaming_url: CfnImageBuilderstreamingurl,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnImageBuilderstreamingurl;
impl CfnImageBuilderstreamingurl {
    pub fn att_name(&self) -> &'static str {
        r#"StreamingUrl"#
    }
}

impl cfn_resources::CfnResource for CfnImageBuilder {
    fn type_string(&self) -> &'static str {
        "AWS::AppStream::ImageBuilder"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_endpoints {
            if the_val.len() > 4 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_endpoints'. {} is greater than 4",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.appstream_agent_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!("Max validation failed on field 'appstream_agent_version'. {} is greater than 100", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.appstream_agent_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'appstream_agent_version'. {} is less than 1", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.display_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'display_name'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        self.domain_join_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.image_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'image_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.instance_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'instance_type'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.vpc_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an interface VPC endpoint (interface endpoint) that lets you create a private connection between the virtual private cloud (VPC) that you specify and AppStream 2.0. When you specify an interface endpoint for a stack, users of the stack can connect to AppStream 2.0 only through that endpoint. When you specify an interface endpoint for an image builder, administrators can connect to the image builder only through that endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessEndpoint {
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
    pub endpoint_type: AccessEndpointEndpointTypeEnum,

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
    pub vpce_id: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AccessEndpointEndpointTypeEnum {
    /// STREAMING
    #[serde(rename = "STREAMING")]
    Streaming,
}

impl Default for AccessEndpointEndpointTypeEnum {
    fn default() -> Self {
        AccessEndpointEndpointTypeEnum::Streaming
    }
}

impl cfn_resources::CfnResource for AccessEndpoint {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.vpce_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'vpce_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DomainJoinInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.organizational_unit_distinguished_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2000 as _ {
                    return Err(format!("Max validation failed on field 'organizational_unit_distinguished_name'. {} is greater than 2000", s.len()));
                }
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

/// The VPC configuration for the image builder.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for VpcConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.security_group_ids {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'security_group_ids'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
