

/// The AWS::LookoutVision::Project type creates an Amazon Lookout for Vision     project. A project is a grouping of the resources needed to create and manage an Amazon     Lookout for Vision model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProject {


    /// 
    /// The name of the project.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9][a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectName")]
    pub project_name: String,

}



impl cfn_resources::CfnResource for CfnProject {
    fn type_string() -> &'static str {
        "AWS::LookoutVision::Project"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
