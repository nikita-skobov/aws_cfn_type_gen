

/// The AWS::IoT1Click::Placement resource creates a placement to be associated with an AWS IoT 1-Click project. A placement is an instance of a device in a location.      For more information, see Projects, Templates, and Placements in the AWS IoT 1-Click Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPlacement {


    /// 
    /// The devices to associate with the placement, as defined by a mapping of zero or more key-value pairs wherein the key is a template name and the value is a device ID.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "AssociatedDevices")]
    pub associated_devices: Option<serde_json::Value>,


    /// 
    /// The user-defined attributes associated with the placement.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Option<serde_json::Value>,


    /// 
    /// The name of the placement.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlacementName")]
    pub placement_name: Option<String>,


    /// 
    /// The name of the project containing the placement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectName")]
    pub project_name: String,

}



impl cfn_resources::CfnResource for CfnPlacement {
    fn type_string() -> &'static str {
        "AWS::IoT1Click::Placement"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
