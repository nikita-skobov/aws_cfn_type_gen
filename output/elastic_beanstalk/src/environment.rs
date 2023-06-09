/// The AWS::ElasticBeanstalk::Environment resource is an AWS Elastic Beanstalk resource     type that specifies an Elastic Beanstalk environment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnEnvironment {
    ///
    /// The name of the application that is associated with this environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: cfn_resources::StrVal,

    ///
    /// If specified, the environment attempts to use this value as the prefix for the CNAME in    your Elastic Beanstalk environment URL. If not specified, the CNAME is generated automatically by    appending a random alphanumeric string to the environment name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 63
    ///
    /// Update requires: Replacement
    #[serde(rename = "CNAMEPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnameprefix: Option<cfn_resources::StrVal>,

    ///
    /// Your description for this environment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// A unique name for the environment.
    ///
    /// Constraint: Must be from 4 to 40 characters in length. The name can contain only    letters, numbers, and hyphens. It can't start or end with a hyphen. This name must be unique    within a region in your account.
    ///
    /// If you don't specify the CNAMEPrefix parameter, the environment name becomes part of    the CNAME, and therefore part of the visible URL for your application.
    ///
    /// If you don't specify an environment name, AWS CloudFormation generates a unique physical    ID and uses that ID for the environment name. For more information, see Name     Type.
    ///
    /// ImportantIf you specify a name, you cannot perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you must replace     the resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<cfn_resources::StrVal>,

    ///
    /// ImportantThe operations role feature of AWS Elastic Beanstalk is in beta release and is subject to change.
    ///
    /// The Amazon Resource Name (ARN) of an existing IAM role to be used as the environment's    operations role. If specified, Elastic Beanstalk uses the operations role for permissions to downstream    services during this call and during subsequent calls acting on this environment. To specify    an operations role, you must have the iam:PassRole permission for the role.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperationsRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations_role: Option<cfn_resources::StrVal>,

    ///
    /// Key-value pairs defining configuration options for this environment, such as the instance    type. These options override the values that are defined in the solution stack or the configuration template.    If you remove any options during a stack update, the removed options retain their current values.
    ///
    /// Required: No
    ///
    /// Type: List of OptionSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<Vec<OptionSetting>>,

    ///
    /// The Amazon Resource Name (ARN) of the custom platform to use with the environment. For    more information, see Custom Platforms in the             AWS Elastic Beanstalk Developer Guide.
    ///
    /// NoteIf you specify PlatformArn, don't specify     SolutionStackName.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlatformArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of an Elastic Beanstalk solution stack (platform version) to use with the environment. If    specified, Elastic Beanstalk sets the configuration values to the default values associated with the    specified solution stack. For a list of current solution stacks, see Elastic Beanstalk Supported Platforms in the         AWS Elastic Beanstalk     Platforms guide.
    ///
    /// NoteIf you specify SolutionStackName, don't specify PlatformArn or      TemplateName.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SolutionStackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the tags applied to resources in the environment.
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
    /// The name of the Elastic Beanstalk configuration template to use with the environment.
    ///
    /// NoteIf you specify TemplateName, then don't specify      SolutionStackName.
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
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the tier to use in creating this environment. The environment tier that you    choose determines whether Elastic Beanstalk provisions resources to support a web application that handles    HTTP(S) requests or a web application that handles background-processing tasks.
    ///
    /// Required: No
    ///
    /// Type: Tier
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,

    ///
    /// The name of the application version to deploy.
    ///
    /// Default: If not specified, Elastic Beanstalk attempts to deploy the sample application.
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
    #[serde(rename = "VersionLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_endpoint_url: CfnEnvironmentendpointurl,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEnvironmentendpointurl;
impl CfnEnvironmentendpointurl {
    pub fn att_name(&self) -> &'static str {
        r#"EndpointURL"#
    }
}

impl cfn_resources::CfnResource for CfnEnvironment {
    fn type_string(&self) -> &'static str {
        "AWS::ElasticBeanstalk::Environment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.application_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'application_name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.application_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'application_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.cnameprefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!(
                        "Max validation failed on field 'cnameprefix'. {} is greater than 63",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.cnameprefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 4 as _ {
                    return Err(format!(
                        "Min validation failed on field 'cnameprefix'. {} is less than 4",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 200 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 200",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.environment_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 40 as _ {
                    return Err(format!(
                        "Max validation failed on field 'environment_name'. {} is greater than 40",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.environment_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 4 as _ {
                    return Err(format!(
                        "Min validation failed on field 'environment_name'. {} is less than 4",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.operations_role {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'operations_role'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.operations_role {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'operations_role'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.template_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'template_name'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.template_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'template_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.tier.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.version_label {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'version_label'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.version_label {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'version_label'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The OptionSetting property type specifies an option for an AWS Elastic Beanstalk environment.
///
/// The OptionSettings property of the AWS::ElasticBeanstalk::Environment resource contains a list of     OptionSetting property types.
///
/// For a list of possible namespaces and option values, see Option Values in the     AWS Elastic Beanstalk Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OptionSetting {
    ///
    /// A unique namespace that identifies the option's associated AWS resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: cfn_resources::StrVal,

    ///
    /// The name of the configuration option.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionName")]
    pub option_name: cfn_resources::StrVal,

    ///
    /// A unique resource name for the option setting. Use it for a time–based scaling configuration option.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<cfn_resources::StrVal>,

    ///
    /// The current value for the configuration option.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for OptionSetting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.resource_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'resource_name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.resource_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'resource_name'. {} is less than 1",
                        s.len()
                    ));
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

/// Describes the environment tier for an AWS::ElasticBeanstalk::Environment resource. For more information, see Environment Tiers in the AWS Elastic Beanstalk Developer    Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tier {
    ///
    /// The name of this environment tier.
    ///
    /// Valid values:
    ///
    /// For Web server tier – WebServer                       For Worker tier – Worker
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The type of this environment tier.
    ///
    /// Valid values:
    ///
    /// For Web server tier – Standard                       For Worker tier – SQS/HTTP
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,

    ///
    /// The version of this environment tier. When you don't set a value to it, Elastic Beanstalk uses the    latest compatible worker tier version.
    ///
    /// NoteThis member is deprecated. Any specific version that you set may become out of date.     We recommend leaving it unspecified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Tier {
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
