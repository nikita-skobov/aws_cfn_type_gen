

/// The AWS::Rekognition::Project type creates an Amazon Rekognition Custom Labels     project. A project is a group of resources needed to create and manage versions of an      Amazon Rekognition Custom Labels model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProject {


    /// 
    /// The name of the project to create.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_.\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectName")]
    pub project_name: String,

}

impl cfn_resources::CfnResource for CfnProject {
    fn type_string() -> &'static str {
        "AWS::Rekognition::Project"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
