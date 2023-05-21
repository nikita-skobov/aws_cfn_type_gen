/// Creates a query definition for    CloudWatch Logs Insights. For more information, see         Analyzing Log Data with CloudWatch Logs Insights.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnQueryDefinition {
    /// Use this parameter if  you want the query to query only certain log groups.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupNames")]
    pub log_group_names: Option<Vec<String>>,

    /// A name for the query definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    /// The query string    to use for this query definition. For more information, see         CloudWatch Logs Insights Query Syntax.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    pub query_string: String,
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
