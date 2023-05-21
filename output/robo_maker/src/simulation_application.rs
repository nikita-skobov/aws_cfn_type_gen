

/// The AWS::RoboMaker::SimulationApplication resource creates an AWS RoboMaker simulation application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSimulationApplication {


    /// 
    /// A map that contains tag keys and tag values that are attached to the simulation     application.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The sources of the simulation application.
    /// 
    /// Required: No
    ///
    /// Type: List of SourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sources")]
    pub sources: Option<Vec<SourceConfig>>,


    /// 
    /// The name of the simulation application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The robot software suite used by the simulation application.
    /// 
    /// Required: Yes
    ///
    /// Type: RobotSoftwareSuite
    ///
    /// Update requires: No interruption
    #[serde(rename = "RobotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,


    /// 
    /// The simulation software suite used by the simulation application.
    /// 
    /// Required: Yes
    ///
    /// Type: SimulationSoftwareSuite
    ///
    /// Update requires: No interruption
    #[serde(rename = "SimulationSoftwareSuite")]
    pub simulation_software_suite: SimulationSoftwareSuite,


    /// 
    /// The environment of the simulation application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    pub environment: Option<String>,


    /// 
    /// The current revision id.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrentRevisionId")]
    pub current_revision_id: Option<String>,


    /// 
    /// The rendering engine for the simulation application.
    /// 
    /// Required: No
    ///
    /// Type: RenderingEngine
    ///
    /// Update requires: No interruption
    #[serde(rename = "RenderingEngine")]
    pub rendering_engine: Option<RenderingEngine>,

}

impl cfn_resources::CfnResource for CfnSimulationApplication {
    fn type_string() -> &'static str {
        "AWS::RoboMaker::SimulationApplication"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Information about a rendering engine.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RenderingEngine {


    /// 
    /// The version of the rendering engine.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4
    ///
    /// Pattern: 1.x
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: String,


    /// 
    /// The name of the rendering engine.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: OGRE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// Information about a simulation software suite.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SimulationSoftwareSuite {


    /// 
    /// The name of the simulation software suite. SimulationRuntime is the only supported value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Gazebo | RosbagPlay | SimulationRuntime
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The version of the simulation software suite. Not applicable for SimulationRuntime.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: 7|9|11|Kinetic|Melodic|Dashing|Foxy
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,

}


/// Information about a robot software suite.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RobotSoftwareSuite {


    /// 
    /// The version of the robot software suite. Not applicable for General software suite.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Dashing | Foxy | Kinetic | Melodic
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// The name of the robot software suite. General is the only supported value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: General | ROS | ROS2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// Information about a source configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceConfig {


    /// 
    /// The Amazon S3 bucket name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [a-z0-9][a-z0-9.\-]*[a-z0-9]
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,


    /// 
    /// The target processor architecture for the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ARM64 | ARMHF | X86_64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Architecture")]
    pub architecture: String,


    /// 
    /// The s3 object key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}
