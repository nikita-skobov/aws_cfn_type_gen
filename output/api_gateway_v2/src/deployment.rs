

/// The AWS::ApiGatewayV2::Deployment resource creates a deployment for          an API.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeployment {


    /// 
    /// The API identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: String,


    /// 
    /// The description for the deployment resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of an existing stage to associate with the deployment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,

}



impl cfn_resources::CfnResource for CfnDeployment {
    fn type_string() -> &'static str {
        "AWS::ApiGatewayV2::Deployment"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}