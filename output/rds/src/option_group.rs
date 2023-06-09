/// The AWS::RDS::OptionGroup resource creates or updates an option group, to enable and       configure features that are specific to a particular DB engine.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnOptionGroup {
    ///
    /// Specifies the name of the engine that this option group should be associated with.
    ///
    /// Valid Values:
    ///
    /// mariadbmysqloracle-eeoracle-ee-cdboracle-se2oracle-se2-cdbpostgressqlserver-eesqlserver-sesqlserver-exsqlserver-web
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineName")]
    pub engine_name: cfn_resources::StrVal,

    ///
    /// Specifies the major version of the engine that this option group should be associated with.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MajorEngineVersion")]
    pub major_engine_version: cfn_resources::StrVal,

    ///
    /// A list of options and the settings for each option.
    ///
    /// Required: Conditional
    ///
    /// Type: List of OptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_configurations: Option<Vec<OptionConfiguration>>,

    ///
    /// The description of the option group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OptionGroupDescription")]
    pub option_group_description: cfn_resources::StrVal,

    ///
    /// The name of the option group to be created.
    ///
    /// Constraints:
    ///
    /// Must be 1 to 255 letters, numbers, or hyphens               First character must be a letter               Can't end with a hyphen or contain two consecutive hyphens
    ///
    /// Example: myoptiongroup
    ///
    /// If you don't specify a value for OptionGroupName property, a name is automatically created for the option group.
    ///
    /// NoteThis value is stored as a lowercase string.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OptionGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<cfn_resources::StrVal>,

    ///
    /// An optional array of key-value pairs to apply to this option group.
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

impl cfn_resources::CfnResource for CfnOptionGroup {
    fn type_string(&self) -> &'static str {
        "AWS::RDS::OptionGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The OptionConfiguration property type specifies an individual option, and       its settings, within an AWS::RDS::OptionGroup resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OptionConfiguration {
    ///
    /// A list of DBSecurityGroupMembership name strings used for this option.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBSecurityGroupMemberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbsecurity_group_memberships: Option<Vec<String>>,

    ///
    /// The configuration of options to include in a group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionName")]
    pub option_name: cfn_resources::StrVal,

    ///
    /// The option settings to include in an option group.
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
    /// The version for the option.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_version: Option<cfn_resources::StrVal>,

    ///
    /// The optional port for the option.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// A list of VpcSecurityGroupMembership name strings used for this option.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupMemberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_memberships: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for OptionConfiguration {
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

/// The OptionSetting property type specifies the value for an option within       an OptionSetting property.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OptionSetting {
    ///
    /// The name of the option that has settings that you can set.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The current value of the option setting.
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
