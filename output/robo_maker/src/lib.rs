
pub mod cfn_fleet {

#[derive(serde::Serialize, Default)]
pub struct CfnFleet {
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The name of the fleet.
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {

}
pub type Arn = String;

}

pub mod cfn_robot {

#[derive(serde::Serialize, Default)]
pub struct CfnRobot {
    /// The Amazon Resource Name (ARN) of the fleet.
    #[serde(rename = "Fleet")]
    pub fleet: Option<String>,
    /// The name for the robot.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The target architecture of the robot.
    #[serde(rename = "Architecture")]
    pub architecture: String,
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The Greengrass group id.
    #[serde(rename = "GreengrassGroupId")]
    pub greengrass_group_id: String,

}

pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_robot_application {

#[derive(serde::Serialize, Default)]
pub struct CfnRobotApplication {
    /// The robot software suite used by the robot application.
    #[serde(rename = "RobotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,
    /// The sources of the robot application.
    #[serde(rename = "Sources")]
    pub sources: Option<Vec<SourceConfig>>,
    /// The URI of the Docker image for the robot application.
    #[serde(rename = "Environment")]
    pub environment: Option<String>,
    /// The name of the robot application.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The revision ID of robot application.
    #[serde(rename = "CurrentRevisionId")]
    pub current_revision_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {

}

#[derive(serde::Serialize, Default)]
pub struct RobotSoftwareSuite {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct SourceConfig {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "Architecture")]
    pub architecture: String,
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}


}

pub mod cfn_robot_application_version {

#[derive(serde::Serialize, Default)]
pub struct CfnRobotApplicationVersion {
    /// The revision ID of robot application.
    #[serde(rename = "CurrentRevisionId")]
    pub current_revision_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Application")]
    pub application: Arn,

}

pub type Arn = String;

}

pub mod cfn_simulation_application {

#[derive(serde::Serialize, Default)]
pub struct CfnSimulationApplication {
    /// The name of the simulation application.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The current revision id.
    #[serde(rename = "CurrentRevisionId")]
    pub current_revision_id: Option<String>,
    /// The robot software suite used by the simulation application.
    #[serde(rename = "RobotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,
    /// The URI of the Docker image for the robot application.
    #[serde(rename = "Environment")]
    pub environment: Option<String>,
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The rendering engine for the simulation application.
    #[serde(rename = "RenderingEngine")]
    pub rendering_engine: Option<RenderingEngine>,
    /// The simulation software suite used by the simulation application.
    #[serde(rename = "SimulationSoftwareSuite")]
    pub simulation_software_suite: SimulationSoftwareSuite,
    /// The sources of the simulation application.
    #[serde(rename = "Sources")]
    pub sources: Option<Vec<SourceConfig>>,

}


#[derive(serde::Serialize, Default)]
pub struct SourceConfig {
    #[serde(rename = "S3Key")]
    pub s3_key: String,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "Architecture")]
    pub architecture: String,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct SimulationSoftwareSuite {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}

#[derive(serde::Serialize, Default)]
pub struct RenderingEngine {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,

}

#[derive(serde::Serialize, Default)]
pub struct RobotSoftwareSuite {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}


}

pub mod cfn_simulation_application_version {

#[derive(serde::Serialize, Default)]
pub struct CfnSimulationApplicationVersion {
    /// No documentation provided by AWS
    #[serde(rename = "Application")]
    pub application: Arn,
    /// The revision ID of robot application.
    #[serde(rename = "CurrentRevisionId")]
    pub current_revision_id: Option<String>,

}

pub type Arn = String;

}
