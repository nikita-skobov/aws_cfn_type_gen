/// Creates a framework with one or more controls. A framework is a collection of controls     that you can use to evaluate your backup practices. By using pre-built customizable     controls to define your policies, you can evaluate whether your backup practices comply     with your policies and which resources are not yet in compliance.
///
/// For a sample AWS CloudFormation template, see the AWS Backup Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFramework {
    ///
    /// Contains detailed information about all of the controls of a framework. Each framework     must contain at least one control.
    ///
    /// Required: Yes
    ///
    /// Type: List of FrameworkControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameworkControls")]
    pub framework_controls: Vec<FrameworkControl>,

    ///
    /// An optional description of the framework with a maximum 1,024 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameworkDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_description: Option<cfn_resources::StrVal>,

    ///
    /// The unique name of a framework. This name is between 1 and 256 characters, starting with     a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and underscores (_).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z][_a-zA-Z0-9]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "FrameworkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of tags with which to tag your framework.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameworkTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnFrameworkcreationtime,

    #[serde(skip_serializing)]
    pub att_deployment_status: CfnFrameworkdeploymentstatus,

    #[serde(skip_serializing)]
    pub att_framework_arn: CfnFrameworkframeworkarn,

    #[serde(skip_serializing)]
    pub att_framework_status: CfnFrameworkframeworkstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFrameworkcreationtime;
impl CfnFrameworkcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFrameworkdeploymentstatus;
impl CfnFrameworkdeploymentstatus {
    pub fn att_name(&self) -> &'static str {
        r#"DeploymentStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFrameworkframeworkarn;
impl CfnFrameworkframeworkarn {
    pub fn att_name(&self) -> &'static str {
        r#"FrameworkArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFrameworkframeworkstatus;
impl CfnFrameworkframeworkstatus {
    pub fn att_name(&self) -> &'static str {
        r#"FrameworkStatus"#
    }
}

impl cfn_resources::CfnResource for CfnFramework {
    fn type_string(&self) -> &'static str {
        "AWS::Backup::Framework"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.framework_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'framework_description'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.framework_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'framework_description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.framework_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'framework_name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.framework_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'framework_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A list of parameters for a control. A control can have zero, one, or more than one     parameter. An example of a control with two parameters is: "backup plan frequency is at     least daily and the retention period is at least 1 year". The     first parameter is daily. The second parameter is 1 year.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ControlInputParameter {
    ///
    /// The name of a parameter, for example, BackupPlanFrequency.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: cfn_resources::StrVal,

    ///
    /// The value of parameter, for example, hourly.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    pub parameter_value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ControlInputParameter {
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

/// A framework consists of one or more controls. Each control has its own control scope.     The control scope can include one or more resource types, a combination of a tag key and     value, or a combination of one resource type and one resource ID. If no scope is specified,     evaluations for the rule are triggered when any resource in your recording group changes in     configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ControlScope {
    ///
    /// The ID of the only AWS resource that you want your control scope to     contain.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComplianceResourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_ids: Option<Vec<String>>,

    ///
    /// Describes whether the control scope includes one or more types of resources, such as       EFS or RDS.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComplianceResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_types: Option<Vec<String>>,

    ///
    /// The tag key-value pair applied to those AWS resources that you want to     trigger an evaluation for a rule. A maximum of one key-value pair can be provided. The tag     value is optional, but it cannot be an empty string. The structure to assign a tag is:       [{"Key":"string","Value":"string"}].
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for ControlScope {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.compliance_resource_ids {
            if the_val.len() > 100 as _ {
                return Err(format!("Max validation failed on field 'compliance_resource_ids'. {} is greater than 100", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Contains detailed information about all of the controls of a framework. Each framework     must contain at least one control.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrameworkControl {
    ///
    /// A list of ParameterName and ParameterValue pairs.
    ///
    /// Required: No
    ///
    /// Type: List of ControlInputParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlInputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_input_parameters: Option<Vec<ControlInputParameter>>,

    ///
    /// The name of a control. This name is between 1 and 256 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlName")]
    pub control_name: cfn_resources::StrVal,

    ///
    /// The scope of a control. The control scope defines what the control will evaluate. Three     examples of control scopes are: a specific backup plan, all backup plans with a specific     tag, or all backup plans. For more information, see ControlScope.
    ///
    /// Required: No
    ///
    /// Type: ControlScope
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_scope: Option<ControlScope>,
}

impl cfn_resources::CfnResource for FrameworkControl {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.control_scope
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
