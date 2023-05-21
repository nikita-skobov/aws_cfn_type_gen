

/// You can use the AWS::SupportApp::SlackWorkspaceConfiguration resource to       specify your Slack workspace configuration. This resource configures your AWS account so that you can use the specified Slack workspace in the         AWS Support App. This resource includes the following information:
///
/// For more information, see the following topics in the AWS Support User Guide:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSlackWorkspaceConfiguration {


    /// 
    /// The team ID in Slack. This ID uniquely identifies a Slack workspace, such as         T012ABCDEFG.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TeamId")]
    pub team_id: String,


    /// 
    /// An identifier used to update an existing Slack workspace configuration in AWS CloudFormation, such as 100.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,

}



impl cfn_resources::CfnResource for CfnSlackWorkspaceConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::SupportApp::SlackWorkspaceConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}