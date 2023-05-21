

/// Creates a new thing group. A dynamic thing group is created if the resource template     contains the QueryString attribute. A dynamic thing group will not contain the       ParentGroupName attribute. A static thing group and dynamic thing group     can't be converted to each other via the addition or removal of the       QueryString attribute.
///
/// Requires permission to access the CreateThingGroup action.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub parent_group_name: Option<String>,


    /// 
    /// Thing group properties.
    /// 
    /// Required: No
    ///
    /// Type: ThingGroupProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThingGroupProperties")]
    pub thing_group_properties: Option<ThingGroupProperties>,


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
    pub query_string: Option<String>,


    /// 
    /// The thing group name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingGroupName")]
    pub thing_group_name: Option<String>,


    /// 
    /// Metadata which can be used to manage the thing group or dynamic thing group.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnThingGroup {
    fn type_string() -> &'static str {
        "AWS::IoT::ThingGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The attribute payload.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub attributes: Option<std::collections::HashMap<String, String>>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}




/// Thing group properties.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub thing_group_description: Option<String>,

}


