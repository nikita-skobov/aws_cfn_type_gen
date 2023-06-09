/// When you create an extension or configure an AWS authored extension, you     associate the extension with an AWS AppConfig application, environment, or     configuration profile. For example, you can choose to run the         AWS AppConfig       deployment events to Amazon SNS       AWS authored extension and receive notifications on an Amazon SNS     topic anytime a configuration deployment is started for a specific application. Defining     which extension to associate with an AWS AppConfig resource is called an       extension association. An extension association is a specified     relationship between an extension and an AWS AppConfig resource, such as an     application or a configuration profile. For more information about extensions and     associations, see Working with        AWS AppConfig extensions in the             AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnExtensionAssociation {
    ///
    /// The name, the ID, or the Amazon Resource Name (ARN) of the extension.
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
    #[serde(rename = "ExtensionIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The version number of the extension. If not specified, AWS AppConfig uses the     maximum version of the extension.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExtensionVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_version_number: Option<i64>,

    ///
    /// The parameter names and values defined in the extensions. Extension parameters marked       Required must be entered for this field.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,

    ///
    /// The ARN of an application, configuration profile, or environment.
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
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<cfn_resources::StrVal>,

    ///
    /// Adds one or more tags for the specified extension association. Tags are metadata that     help you categorize resources in different ways, for example, by purpose, owner, or     environment. Each tag consists of a key and an optional value, both of which you define.
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
    pub att_arn: CfnExtensionAssociationarn,

    #[serde(skip_serializing)]
    pub att_extension_arn: CfnExtensionAssociationextensionarn,

    #[serde(skip_serializing)]
    pub att_id: CfnExtensionAssociationid,

    #[serde(skip_serializing)]
    pub att_resource_arn: CfnExtensionAssociationresourcearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExtensionAssociationarn;
impl CfnExtensionAssociationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExtensionAssociationextensionarn;
impl CfnExtensionAssociationextensionarn {
    pub fn att_name(&self) -> &'static str {
        r#"ExtensionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExtensionAssociationid;
impl CfnExtensionAssociationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExtensionAssociationresourcearn;
impl CfnExtensionAssociationresourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceArn"#
    }
}

impl cfn_resources::CfnResource for CfnExtensionAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::AppConfig::ExtensionAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.extension_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'extension_identifier'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.extension_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'extension_identifier'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.resource_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'resource_identifier'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.resource_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'resource_identifier'. {} is less than 1",
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
