/// Detailed data of an AWS Proton environment account connection resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentAccountConnection {
    ///
    /// The Amazon Resource Name (ARN) of an IAM service role in the environment account. AWS Proton uses this role to provision infrastructure resources    using CodeBuild-based provisioning in the associated environment account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov):iam::\d{12}:role/([\w+=,.@-]{1,512}[/:])*([\w+=,.@-]{1,64})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodebuildRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codebuild_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM service role that AWS Proton uses when provisioning directly defined components in the associated    environment account. It determines the scope of infrastructure that a component can provision in the account.
    ///
    /// The environment account connection must have a componentRoleArn to allow directly defined components to be associated with any    environments running in the account.
    ///
    /// For more information about components, see  AWS Proton components in the  AWS Proton User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov):iam::\d{12}:role/([\w+=,.@-]{1,512}[/:])*([\w+=,.@-]{1,64})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The environment account that's connected to the environment account connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^\d{12}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_account_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the environment that's associated with the environment account connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[0-9A-Za-z]+[0-9A-Za-z_\-]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the management account that's connected to the environment account connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^\d{12}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagementAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_account_id: Option<cfn_resources::StrVal>,

    ///
    /// The IAM service role that's associated with the environment account connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov):[a-zA-Z0-9-]+:[a-zA-Z0-9-]*:\d{12}:([\w+=,.@-]+[/:])*[\w+=,.@-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// An optional list of metadata items that you can associate with the AWS Proton environment account connection. A tag is a key-value pair.
    ///
    /// For more information, see AWS Proton resources and tagging in the     AWS Proton User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnEnvironmentAccountConnectionarn,

    #[serde(skip_serializing)]
    pub att_id: CfnEnvironmentAccountConnectionid,

    #[serde(skip_serializing)]
    pub att_status: CfnEnvironmentAccountConnectionstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentAccountConnectionarn;
impl CfnEnvironmentAccountConnectionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentAccountConnectionid;
impl CfnEnvironmentAccountConnectionid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironmentAccountConnectionstatus;
impl CfnEnvironmentAccountConnectionstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnEnvironmentAccountConnection {
    fn type_string(&self) -> &'static str {
        "AWS::Proton::EnvironmentAccountConnection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.codebuild_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'codebuild_role_arn'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.codebuild_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'codebuild_role_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.component_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'component_role_arn'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.component_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'component_role_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.environment_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'environment_name'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.environment_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'environment_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 200 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 200",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'role_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
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
