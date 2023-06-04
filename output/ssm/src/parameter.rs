/// The AWS::SSM::Parameter resource creates an SSM parameter in AWS Systems Manager Parameter Store.
///
/// For information about valid values for parameters, see Requirements     and Constraints for Parameter Names in the AWS Systems Manager User     Guide and PutParameter in the     AWS Systems Manager API Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnParameter {
    ///
    /// A regular expression used to validate the parameter value. For example, for String types    with values restricted to numbers, you can specify the following:     AllowedPattern=^\d+$
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<cfn_resources::StrVal>,

    ///
    /// The data type of the parameter, such as text or aws:ec2:image. The  default is text.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<cfn_resources::StrVal>,

    ///
    /// Information about the parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the parameter.
    ///
    /// NoteThe maximum length constraint listed below includes capacity for additional system   attributes that aren't part of the name. The maximum length for a parameter name, including the   full length of the parameter ARN, is 1011 characters. For example, the length of the following   parameter name is 65 characters, not 20 characters: arn:aws:ssm:us-east-2:111222333444:parameter/ExampleParameterName
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Information about the policies assigned to a parameter.
    ///
    /// Assigning parameter   policies in the         AWS Systems Manager User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<cfn_resources::StrVal>,

    ///
    /// Optional metadata that you assign to a resource in the form of an arbitrary set of tags    (key-value pairs). Tags enable you to categorize a resource in different ways, such as by    purpose, owner, or environment. For example, you might want to tag a Systems Manager parameter    to identify the type of resource to which it applies, the environment, or the type of    configuration data referenced by the parameter.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    ///
    /// The parameter tier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Advanced | Intelligent-Tiering | Standard
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<ParameterTierEnum>,

    ///
    /// The type of parameter.
    ///
    /// NoteAWS CloudFormation doesn't support creating a SecureString parameter     type.
    ///
    /// Allowed Values: String | StringList
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: ParameterTypeEnum,

    ///
    /// The parameter value.
    ///
    /// NoteIf type is StringList, the system returns a comma-separated string with no   spaces between commas in the Value field.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_cfn_type: CfnParametercfntype,

    #[serde(skip_serializing)]
    pub att_value: CfnParametervalue,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ParameterTierEnum {
    /// Advanced
    #[serde(rename = "Advanced")]
    Advanced,

    /// Intelligent-Tiering
    #[serde(rename = "Intelligent-Tiering")]
    Intelligenttiering,

    /// Standard
    #[serde(rename = "Standard")]
    Standard,
}

impl Default for ParameterTierEnum {
    fn default() -> Self {
        ParameterTierEnum::Advanced
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ParameterTypeEnum {
    /// String
    #[serde(rename = "String")]
    String,

    /// StringList
    #[serde(rename = "StringList")]
    Stringlist,
}

impl Default for ParameterTypeEnum {
    fn default() -> Self {
        ParameterTypeEnum::String
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnParametercfntype;
impl CfnParametercfntype {
    pub fn att_name(&self) -> &'static str {
        r#"Type"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnParametervalue;
impl CfnParametervalue {
    pub fn att_name(&self) -> &'static str {
        r#"Value"#
    }
}

impl cfn_resources::CfnResource for CfnParameter {
    fn type_string(&self) -> &'static str {
        "AWS::SSM::Parameter"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.allowed_pattern {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'allowed_pattern'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.allowed_pattern {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'allowed_pattern'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.data_type {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'data_type'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.data_type {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'data_type'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
