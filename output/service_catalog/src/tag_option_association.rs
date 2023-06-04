/// Associate the specified TagOption with the specified portfolio or product.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTagOptionAssociation {
    ///
    /// The resource identifier.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    pub resource_id: cfn_resources::StrVal,

    ///
    /// The TagOption identifier.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnTagOptionAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::TagOptionAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
