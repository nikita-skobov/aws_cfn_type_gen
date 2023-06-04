/// The AWS::IoT1Click::Project resource creates an empty project with a placement template. A project contains zero or more placements that      adhere to the placement template defined in the project. For more information, see CreateProject     in the AWS IoT 1-Click Projects API Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProject {
    ///
    /// The description of the project.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// An object describing the project's placement specifications.
    ///
    /// Required: Yes
    ///
    /// Type: PlacementTemplate
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlacementTemplate")]
    pub placement_template: PlacementTemplate,

    ///
    /// The name of the project from which to obtain information.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnProjectarn,

    #[serde(skip_serializing)]
    pub att_project_name: CfnProjectprojectname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectarn;
impl CfnProjectarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectprojectname;
impl CfnProjectprojectname {
    pub fn att_name(&self) -> &'static str {
        r#"ProjectName"#
    }
}

impl cfn_resources::CfnResource for CfnProject {
    fn type_string(&self) -> &'static str {
        "AWS::IoT1Click::Project"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.placement_template.validate()?;

        Ok(())
    }
}

/// In AWS CloudFormation, use the DeviceTemplate property type to define the template for an AWS IoT 1-Click project.
///
/// DeviceTemplate is a property of the AWS::IoT1Click::Project resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DeviceTemplate {
    ///
    /// An optional AWS Lambda function to invoke instead of the default AWS Lambda function provided by    the placement template.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "CallbackOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_overrides: Option<serde_json::Value>,

    ///
    /// The device type, which currently must be "button".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DeviceTemplate {
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

/// In AWS CloudFormation, use the PlacementTemplate property type to define the template for an AWS IoT 1-Click project.
///
/// PlacementTemplate is a property of the AWS::IoT1Click::Project resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PlacementTemplate {
    ///
    /// The default attributes (key-value pairs) to be applied to all placements using this    template.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_attributes: Option<serde_json::Value>,

    ///
    /// An object specifying the DeviceTemplate for all placements using this     (PlacementTemplate) template.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_templates: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for PlacementTemplate {
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
