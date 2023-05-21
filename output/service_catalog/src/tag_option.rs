/// Specifies a TagOption. A TagOption is a key-value pair managed by AWS Service Catalog     that serves as a template for creating an AWS tag.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTagOption {
    ///
    /// The TagOption active state.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Active")]
    pub active: Option<bool>,

    ///
    /// The TagOption key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The TagOption value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for CfnTagOption {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::TagOption"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
