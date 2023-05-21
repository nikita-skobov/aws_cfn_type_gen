

/// The AWS::RoboMaker::Fleet resource creates an AWS RoboMaker     fleet. Fleets contain robots and can receive deployments.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFleet {


    /// 
    /// The list of all tags added to the fleet.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The name of the fleet.
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

}



impl cfn_resources::CfnResource for CfnFleet {
    fn type_string() -> &'static str {
        "AWS::RoboMaker::Fleet"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
