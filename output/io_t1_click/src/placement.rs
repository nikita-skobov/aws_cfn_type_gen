/// The AWS::IoT1Click::Placement resource creates a placement to be associated with an AWS IoT 1-Click project. A placement is an instance of a device in a location.      For more information, see Projects, Templates, and Placements in the AWS IoT 1-Click Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub placement_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the project containing the placement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectName")]
    pub project_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_placement_name: CfnPlacementplacementname,

    #[serde(skip_serializing)]
    pub att_project_name: CfnPlacementprojectname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlacementplacementname;
impl CfnPlacementplacementname {
    pub fn att_name(&self) -> &'static str {
        r#"PlacementName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlacementprojectname;
impl CfnPlacementprojectname {
    pub fn att_name(&self) -> &'static str {
        r#"ProjectName"#
    }
}

impl cfn_resources::CfnResource for CfnPlacement {
    fn type_string(&self) -> &'static str {
        "AWS::IoT1Click::Placement"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
