/// Modifies which task set in a service is the primary task set. Any parameters that are 			updated on the primary task set in a service will transition to the service. This is 			used when a service uses the EXTERNAL deployment controller type. For more 			information, see Amazon ECS Deployment 				Types in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPrimaryTaskSet {
    ///
    /// The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task 			set exists in.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cluster")]
    pub cluster: cfn_resources::StrVal,

    ///
    /// The short name or full Amazon Resource Name (ARN) of the service that the task set exists in.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Service")]
    pub service: cfn_resources::StrVal,

    ///
    /// The short name or full Amazon Resource Name (ARN) of the task set to set as the primary task set in the 			deployment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskSetId")]
    pub task_set_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnPrimaryTaskSet {
    fn type_string(&self) -> &'static str {
        "AWS::ECS::PrimaryTaskSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
