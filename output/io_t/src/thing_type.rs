/// Creates a new thing type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnThingType {
    ///
    /// Deprecates a thing type. You can not associate new things with deprecated thing 			type.
    ///
    /// Requires permission to access the DeprecateThingType action.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeprecateThingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecate_thing_type: Option<bool>,

    ///
    /// Metadata which can be used to manage the thing type.
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
    /// The name of the thing type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<cfn_resources::StrVal>,

    ///
    /// The thing type properties for the thing type to create. It contains information about     the new thing type including a description, and a list of searchable thing attribute names.       ThingTypeProperties can't be updated after the initial creation of the       ThingType.
    ///
    /// Required: No
    ///
    /// Type: ThingTypeProperties
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingTypeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,

    #[serde(skip_serializing)]
    pub att_arn: CfnThingTypearn,

    #[serde(skip_serializing)]
    pub att_id: CfnThingTypeid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnThingTypearn;
impl CfnThingTypearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnThingTypeid;
impl CfnThingTypeid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnThingType {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::ThingType"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.thing_type_properties
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

/// The ThingTypeProperties contains information about the thing type including: a thing type description, 			and a list of searchable thing attribute names.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ThingTypeProperties {
    ///
    /// A list of searchable thing attribute names.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SearchableAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,

    ///
    /// The description of the thing type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingTypeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_description: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ThingTypeProperties {
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
