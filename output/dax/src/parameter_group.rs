

/// A named set of parameters that are applied to all of the nodes in a DAX cluster.
#[derive(Default, serde::Serialize)]
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
    pub description: Option<String>,


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
    pub parameter_name_values: Option<serde_json::Value>,


    /// 
    /// The name of the parameter group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: Option<String>,

}
