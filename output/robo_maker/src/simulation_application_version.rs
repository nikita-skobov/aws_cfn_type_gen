

/// The AWS::RoboMaker::SimulationApplicationVersion resource creates a version     of an AWS RoboMaker simulation application.
#[derive(Default, serde::Serialize)]
pub struct CfnSimulationApplicationVersion {


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

}
