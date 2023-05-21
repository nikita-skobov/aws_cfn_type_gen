

/// The AWS::RoboMaker::SimulationApplicationVersion resource creates a version     of an AWS RoboMaker simulation application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSimulationApplicationVersion {


    /// 
    /// The application information for the simulation application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1224
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Application")]
    pub application: String,


    /// 
    /// The current revision id for the simulation application. If you provide a value and it     matches the latest revision ID, a new version will be created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 40
    ///
    /// Pattern: [a-zA-Z0-9_.\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CurrentRevisionId")]
    pub current_revision_id: Option<String>,

}

impl cfn_resources::CfnResource for CfnSimulationApplicationVersion {
    fn type_string() -> &'static str {
        "AWS::RoboMaker::SimulationApplicationVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
