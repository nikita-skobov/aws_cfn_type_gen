

/// Associates an attribute group      with an application      to augment the application's metadata      with the group's attributes.      This feature enables applications      to be described      with user-defined details     that are machine-readable,      such as third-party integrations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAttributeGroupAssociation {


    /// 
    /// The name or ID      of the attribute group      that holds the attributes      to describe the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AttributeGroup")]
    pub attribute_group: String,


    /// 
    /// The name or ID     of the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Application")]
    pub application: String,

}

impl cfn_resources::CfnResource for CfnAttributeGroupAssociation {
    fn type_string() -> &'static str {
        "AWS::ServiceCatalogAppRegistry::AttributeGroupAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
