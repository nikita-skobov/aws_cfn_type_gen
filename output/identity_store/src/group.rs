/// A group object, which contains a specified group’s metadata and attributes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    pub display_name: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityStoreId")]
    pub identity_store_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_group_id: CfnGroupgroupid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGroupgroupid;
impl CfnGroupgroupid {
    pub fn att_name(&self) -> &'static str {
        r#"GroupId"#
    }
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
