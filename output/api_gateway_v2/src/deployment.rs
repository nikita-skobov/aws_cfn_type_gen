

/// The AWS::ApiGatewayV2::Deployment resource creates a deployment for          an API.
#[derive(Default, serde::Serialize)]
pub struct CfnDeployment {


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

}