/// A group object, which contains a specified group’s metadata and attributes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGroup {
    ///
    /// A string containing the description of the group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    pub display_name: String,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityStoreId")]
    pub identity_store_id: String,
}

impl cfn_resources::CfnResource for CfnGroup {
    fn type_string(&self) -> &'static str {
        "AWS::IdentityStore::Group"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
