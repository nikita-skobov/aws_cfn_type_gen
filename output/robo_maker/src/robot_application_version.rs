

/// The AWS::RoboMaker::RobotApplicationVersion resource creates an AWS RoboMaker robot version.
#[derive(Default, serde::Serialize)]
pub struct CfnRobotApplicationVersion {


    /// 
    /// The application information for the robot application.
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
    /// The current revision id for the robot application. If you provide a value and it matches     the latest revision ID, a new version will be created.
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
