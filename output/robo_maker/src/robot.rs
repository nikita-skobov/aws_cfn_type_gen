

/// The AWS::RoboMaker::RobotApplication resource creates an AWS     RoboMaker robot.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub architecture: String,


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
    /// The Amazon Resource Name (ARN) of the fleet to which the robot will be     registered.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Fleet")]
    pub fleet: Option<String>,

}

impl cfn_resources::CfnResource for CfnRobot {
    fn type_string() -> &'static str {
        "AWS::RoboMaker::Robot"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
