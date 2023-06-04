/// A named set of parameters that are applied to all of the nodes in a DAX cluster.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnParameterGroup {
    ///
    /// A description of the parameter group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the parameter group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<cfn_resources::StrVal>,

    ///
    /// An array of name-value pairs for the parameters in the group. Each element in the       array represents a single parameter.
    ///
    /// Note        record-ttl-millis and query-ttl-millis are the only         supported parameter names. For more details, see Configuring TTL Settings.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterNameValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name_values: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for CfnParameterGroup {
    fn type_string(&self) -> &'static str {
        "AWS::DAX::ParameterGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
