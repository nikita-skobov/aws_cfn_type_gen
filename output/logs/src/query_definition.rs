/// Creates a query definition for    CloudWatch Logs Insights. For more information, see         Analyzing Log Data with CloudWatch Logs Insights.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnQueryDefinition {
    /// Use this parameter if  you want the query to query only certain log groups.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_names: Option<Vec<String>>,

    /// A name for the query definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    /// The query string    to use for this query definition. For more information, see         CloudWatch Logs Insights Query Syntax.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    pub query_string: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_query_definition_id: CfnQueryDefinitionquerydefinitionid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnQueryDefinitionquerydefinitionid;
impl CfnQueryDefinitionquerydefinitionid {
    pub fn att_name(&self) -> &'static str {
        r#"QueryDefinitionId"#
    }
}

impl cfn_resources::CfnResource for CfnQueryDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::Logs::QueryDefinition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
