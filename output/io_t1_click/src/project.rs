

/// The AWS::IoT1Click::Project resource creates an empty project with a placement template. A project contains zero or more placements that      adhere to the placement template defined in the project. For more information, see CreateProject     in the AWS IoT 1-Click Projects API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub description: Option<String>,


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
    pub project_name: Option<String>,

}



impl cfn_resources::CfnResource for CfnProject {
    fn type_string(&self) -> &'static str {
        "AWS::IoT1Click::Project"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.placement_template.validate()?;

        Ok(())
    }
}

/// In AWS CloudFormation, use the DeviceTemplate property type to define the template for an AWS IoT 1-Click project.
///
/// DeviceTemplate is a property of the AWS::IoT1Click::Project resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub device_type: Option<String>,

}



impl cfn_resources::CfnResource for DeviceTemplate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// In AWS CloudFormation, use the PlacementTemplate property type to define the template for an AWS IoT 1-Click project.
///
/// PlacementTemplate is a property of the AWS::IoT1Click::Project resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub device_templates: Option<serde_json::Value>,

}



impl cfn_resources::CfnResource for PlacementTemplate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}