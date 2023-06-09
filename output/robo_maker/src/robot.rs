/// The AWS::RoboMaker::RobotApplication resource creates an AWS     RoboMaker robot.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnRobot {
    ///
    /// The architecture of the robot.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ARM64 | ARMHF | X86_64
    ///
    /// Update requires: Replacement
    #[serde(rename = "Architecture")]
    pub architecture: RobotArchitectureEnum,

    ///
    /// The Amazon Resource Name (ARN) of the fleet to which the robot will be     registered.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Fleet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<cfn_resources::StrVal>,

    ///
    /// The Greengrass group associated with the robot.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1224
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "GreengrassGroupId")]
    pub greengrass_group_id: cfn_resources::StrVal,

    ///
    /// The name of the robot.
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
    /// A map that contains tag keys and tag values that are attached to the robot.
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
    pub att_arn: CfnRobotarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RobotArchitectureEnum {
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

impl Default for RobotArchitectureEnum {
    fn default() -> Self {
        RobotArchitectureEnum::Arm64
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRobotarn;
impl CfnRobotarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnRobot {
    fn type_string(&self) -> &'static str {
        "AWS::RoboMaker::Robot"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.greengrass_group_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1224 as _ {
                return Err(format!(
                    "Max validation failed on field 'greengrass_group_id'. {} is greater than 1224",
                    s.len()
                ));
            }
        }

        let the_val = &self.greengrass_group_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'greengrass_group_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

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

        Ok(())
    }
}
