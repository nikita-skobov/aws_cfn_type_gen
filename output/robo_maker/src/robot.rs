

/// The AWS::RoboMaker::RobotApplication resource creates an AWS     RoboMaker robot.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRobot {


    /// 
    /// The Amazon Resource Name (ARN) of the fleet to which the robot will be     registered.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Fleet")]
    pub fleet: Option<String>,


    /// 
    /// A map that contains tag keys and tag values that are attached to the robot.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


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
    pub name: Option<String>,


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
    pub greengrass_group_id: String,


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

}


#[derive(Clone, Debug, serde::Serialize)]
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


impl cfn_resources::CfnResource for CfnRobot {
    fn type_string() -> &'static str {
        "AWS::RoboMaker::Robot"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
