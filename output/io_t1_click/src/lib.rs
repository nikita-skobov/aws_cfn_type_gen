
pub mod cfn_device {

#[derive(serde::Serialize, Default)]
pub struct CfnDevice {
    /// No documentation provided by AWS
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}



}

pub mod cfn_placement {

#[derive(serde::Serialize, Default)]
pub struct CfnPlacement {
    /// No documentation provided by AWS
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "PlacementName")]
    pub placement_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProjectName")]
    pub project_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "AssociatedDevices")]
    pub associated_devices: Option<()>,

}



}

pub mod cfn_project {

#[derive(serde::Serialize, Default)]
pub struct CfnProject {
    /// No documentation provided by AWS
    #[serde(rename = "ProjectName")]
    pub project_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PlacementTemplate")]
    pub placement_template: PlacementTemplate,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct PlacementTemplate {
    #[serde(rename = "DeviceTemplates")]
    pub device_templates: Option<()>,
    #[serde(rename = "DefaultAttributes")]
    pub default_attributes: Option<()>,

}


}
