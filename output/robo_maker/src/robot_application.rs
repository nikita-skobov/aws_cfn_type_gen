/// The AWS::RoboMaker::RobotApplication resource creates an AWS     RoboMaker robot application.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnRobotApplication {
    ///
    /// The current revision id.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrentRevisionId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub current_revision_id: Option<cfn_resources::StrVal>,

    ///
    /// The environment of the robot application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub environment: Option<cfn_resources::StrVal>,

    ///
    /// The name of the robot application.
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The robot software suite used by the robot application.
    ///
    /// Required: Yes
    ///
    /// Type: RobotSoftwareSuite
    ///
    /// Update requires: No interruption
    #[serde(rename = "RobotSoftwareSuite")]
    pub robot_software_suite: RobotSoftwareSuite,

    ///
    /// The sources of the robot application.
    ///
    /// Required: No
    ///
    /// Type: List of SourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sources: Option<Vec<SourceConfig>>,

    ///
    /// A map that contains tag keys and tag values that are attached to the robot     application.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnRobotApplicationarn,

    #[serde(skip_serializing)]
    pub att_current_revision_id: CfnRobotApplicationcurrentrevisionid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRobotApplicationarn;
impl CfnRobotApplicationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRobotApplicationcurrentrevisionid;
impl CfnRobotApplicationcurrentrevisionid {
    pub fn att_name(&self) -> &'static str {
        r#"CurrentRevisionId"#
    }
}

impl cfn_resources::CfnResource for CfnRobotApplication {
    fn type_string(&self) -> &'static str {
        "AWS::RoboMaker::RobotApplication"
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

        self.robot_software_suite.validate()?;

        Ok(())
    }
}

/// Information about a robot software suite.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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

/// Information about a source configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
