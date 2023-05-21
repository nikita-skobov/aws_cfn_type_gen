

/// The AWS::Glue::Workflow is an AWS Glue resource type that manages AWS Glue workflows. A workflow is a container for a set of related jobs, crawlers, and triggers in AWS Glue. Using a workflow, you can design a complex multi-job extract, transform, and load (ETL) activity that AWS Glue can execute and track as single entity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWorkflow {


    /// A collection of properties to be used as part of each execution of the workflow
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRunProperties")]
    pub default_run_properties: Option<serde_json::Value>,


    /// A description of the workflow
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// You can use this parameter to prevent unwanted multiple updates to data, to control costs, or in some cases, to prevent exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConcurrentRuns")]
    pub max_concurrent_runs: Option<i64>,


    /// The name of the workflow representing the flow
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// The tags to use with this workflow.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,

}



impl cfn_resources::CfnResource for CfnWorkflow {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::Workflow"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}