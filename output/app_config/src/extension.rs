/// Creates an AWS AppConfig extension. An extension augments your ability to inject     logic or behavior at different points during the AWS AppConfig workflow of creating     or deploying a configuration.
///
/// You can create your own extensions or use the AWS authored extensions provided by       AWS AppConfig. For an AWS AppConfig extension that uses       AWS Lambda, you must create a Lambda function to perform any     computation and processing defined in the extension. If you plan to create custom versions     of the AWS authored notification extensions, you only need to specify an Amazon Resource     Name (ARN) in the Uri field for the new extension version.
///
/// For more information about extensions, see Working with        AWS AppConfig extensions in the             AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExtension {
    ///
    /// The actions defined in the extension.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: serde_json::Value,

    ///
    /// Information about the extension.
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
    /// You can omit this field when you create an extension. When you create a new version,     specify the most recent current version number. For example, you create version 3, enter 2     for this field.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "LatestVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i64>,

    ///
    /// A name for the extension. Each extension name in your account must be unique. Extension     versions use the same name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The parameters accepted by the extension. You specify parameter values when you     associate the extension to an AWS AppConfig resource by using the       CreateExtensionAssociation API action. For AWS Lambda extension     actions, these parameters are included in the Lambda request object.
    ///
    /// Required: No
    ///
    /// Type: Map of Parameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Parameter>>,

    ///
    /// Adds one or more tags for the specified extension. Tags are metadata that help you     categorize resources in different ways, for example, by purpose, owner, or environment.     Each tag consists of a key and an optional value, both of which you define.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnExtensionarn,

    #[serde(skip_serializing)]
    pub att_id: CfnExtensionid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExtensionarn;
impl CfnExtensionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExtensionid;
impl CfnExtensionid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnExtension {
    fn type_string(&self) -> &'static str {
        "AWS::AppConfig::Extension"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// A value such as an Amazon Resource Name (ARN) or an Amazon Simple Notification Service topic entered     in an extension when invoked. Parameter values are specified in an extension association.     For more information about extensions, see Working with        AWS AppConfig extensions in the             AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
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
    /// A parameter value must be specified in the extension association.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Required")]
    pub required: bool,
}

impl cfn_resources::CfnResource for Parameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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
