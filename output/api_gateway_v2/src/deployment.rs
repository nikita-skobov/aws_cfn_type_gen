/// The AWS::ApiGatewayV2::Deployment resource creates a deployment for          an API.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub api_id: cfn_resources::StrVal,

    ///
    /// The description for the deployment resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of an existing stage to associate with the deployment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub stage_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_deployment_id: CfnDeploymentdeploymentid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDeploymentdeploymentid;
impl CfnDeploymentdeploymentid {
    pub fn att_name(&self) -> &'static str {
        r#"DeploymentId"#
    }
}

impl cfn_resources::CfnResource for CfnDeployment {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::Deployment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
