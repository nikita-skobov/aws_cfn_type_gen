/// Creates a new thing group. A dynamic thing group is created if the resource template     contains the QueryString attribute. A dynamic thing group will not contain the       ParentGroupName attribute. A static thing group and dynamic thing group     can't be converted to each other via the addition or removal of the       QueryString attribute.
///
/// Requires permission to access the CreateThingGroup action.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnThingGroup {
    ///
    /// The parent thing group name.
    ///
    /// A Dynamic Thing Group does not have parentGroupName defined.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The dynamic thing group search query string.
    ///
    /// The queryString attribute is required for       CreateDynamicThingGroup. The queryString attribute       is not required for CreateThingGroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<cfn_resources::StrVal>,

    ///
    /// Metadata which can be used to manage the thing group or dynamic thing group.
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
    /// The thing group name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<cfn_resources::StrVal>,

    ///
    /// Thing group properties.
    ///
    /// Required: No
    ///
    /// Type: ThingGroupProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThingGroupProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_properties: Option<ThingGroupProperties>,

    #[serde(skip_serializing)]
    pub att_arn: CfnThingGrouparn,

    #[serde(skip_serializing)]
    pub att_id: CfnThingGroupid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnThingGrouparn;
impl CfnThingGrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnThingGroupid;
impl CfnThingGroupid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnThingGroup {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::ThingGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.thing_group_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The attribute payload.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AttributePayload {
    ///
    /// A JSON string containing up to three key-value pair in JSON format. For example:
    ///
    /// {\"attributes\":{\"string1\":\"string2\"}}
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for AttributePayload {
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

/// Thing group properties.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ThingGroupProperties {
    ///
    /// The thing group attributes in JSON format.
    ///
    /// Required: No
    ///
    /// Type: AttributePayload
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,

    ///
    /// The thing group description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThingGroupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_description: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ThingGroupProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.attribute_payload
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
