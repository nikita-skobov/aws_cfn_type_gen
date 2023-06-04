/// The AWS::RoboMaker::SimulationApplication resource creates an AWS RoboMaker simulation application.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSimulationApplication {
    ///
    /// The current revision id.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrentRevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision_id: Option<cfn_resources::StrVal>,

    ///
    /// The environment of the simulation application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The rendering engine for the simulation application.
    ///
    /// Required: No
    ///
    /// Type: RenderingEngine
    ///
    /// Update requires: No interruption
    #[serde(rename = "RenderingEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_engine: Option<RenderingEngine>,

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
    /// The sources of the simulation application.
    ///
    /// Required: No
    ///
    /// Type: List of SourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SourceConfig>>,

    ///
    /// A map that contains tag keys and tag values that are attached to the simulation     application.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnSimulationApplicationarn,

    #[serde(skip_serializing)]
    pub att_current_revision_id: CfnSimulationApplicationcurrentrevisionid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSimulationApplicationarn;
impl CfnSimulationApplicationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSimulationApplicationcurrentrevisionid;
impl CfnSimulationApplicationcurrentrevisionid {
    pub fn att_name(&self) -> &'static str {
        r#"CurrentRevisionId"#
    }
}

impl cfn_resources::CfnResource for CfnSimulationApplication {
    fn type_string(&self) -> &'static str {
        "AWS::RoboMaker::SimulationApplication"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.rendering_engine
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.robot_software_suite.validate()?;

        self.simulation_software_suite.validate()?;

        Ok(())
    }
}

/// Information about a rendering engine.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RenderingEngine {
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
    pub name: RenderingEngineNameEnum,

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
    pub version: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RenderingEngineNameEnum {
    /// OGRE
    #[serde(rename = "OGRE")]
    Ogre,
}

impl Default for RenderingEngineNameEnum {
    fn default() -> Self {
        RenderingEngineNameEnum::Ogre
    }
}

impl cfn_resources::CfnResource for RenderingEngine {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.version;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 4 as _ {
                return Err(format!(
                    "Max validation failed on field 'version'. {} is greater than 4",
                    s.len()
                ));
            }
        }

        let the_val = &self.version;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'version'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Information about a robot software suite.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RobotSoftwareSuite {
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
    pub name: RobotSoftwareSuiteNameEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<RobotSoftwareSuiteVersionEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RobotSoftwareSuiteNameEnum {
    /// General
    #[serde(rename = "General")]
    General,

    /// ROS
    #[serde(rename = "ROS")]
    Ros,

    /// ROS2
    #[serde(rename = "ROS2")]
    Ros2,
}

impl Default for RobotSoftwareSuiteNameEnum {
    fn default() -> Self {
        RobotSoftwareSuiteNameEnum::General
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RobotSoftwareSuiteVersionEnum {
    /// Dashing
    #[serde(rename = "Dashing")]
    Dashing,

    /// Foxy
    #[serde(rename = "Foxy")]
    Foxy,

    /// Kinetic
    #[serde(rename = "Kinetic")]
    Kinetic,

    /// Melodic
    #[serde(rename = "Melodic")]
    Melodic,
}

impl Default for RobotSoftwareSuiteVersionEnum {
    fn default() -> Self {
        RobotSoftwareSuiteVersionEnum::Dashing
    }
}

impl cfn_resources::CfnResource for RobotSoftwareSuite {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Information about a simulation software suite.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub name: SimulationSoftwareSuiteNameEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SimulationSoftwareSuiteNameEnum {
    /// Gazebo
    #[serde(rename = "Gazebo")]
    Gazebo,

    /// RosbagPlay
    #[serde(rename = "RosbagPlay")]
    Rosbagplay,

    /// SimulationRuntime
    #[serde(rename = "SimulationRuntime")]
    Simulationruntime,
}

impl Default for SimulationSoftwareSuiteNameEnum {
    fn default() -> Self {
        SimulationSoftwareSuiteNameEnum::Gazebo
    }
}

impl cfn_resources::CfnResource for SimulationSoftwareSuite {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'version'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'version'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Information about a source configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceConfig {
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
    pub architecture: SourceConfigArchitectureEnum,

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
    pub s3_bucket: cfn_resources::StrVal,

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
    pub s3_key: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SourceConfigArchitectureEnum {
    /// ARM64
    #[serde(rename = "ARM64")]
    Arm64,

    /// ARMHF
    #[serde(rename = "ARMHF")]
    Armhf,

    /// X86_64
    #[serde(rename = "X86_64")]
    X8664,
}

impl Default for SourceConfigArchitectureEnum {
    fn default() -> Self {
        SourceConfigArchitectureEnum::Arm64
    }
}

impl cfn_resources::CfnResource for SourceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.s3_bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_bucket'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.s3_bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 's3_bucket'. {} is less than 3",
                    s.len()
                ));
            }
        }

        let the_val = &self.s3_key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_key'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        let the_val = &self.s3_key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 's3_key'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
