
pub mod cfn_job_template {

#[derive(serde::Serialize, Default)]
pub struct CfnJobTemplate {
    /// No documentation provided by AWS
    #[serde(rename = "SettingsJson")]
    pub settings_json: (),
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "StatusUpdateInterval")]
    pub status_update_interval: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Category")]
    pub category: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AccelerationSettings")]
    pub acceleration_settings: Option<AccelerationSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "Priority")]
    pub priority: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of HopDestination
    #[serde(rename = "HopDestinations")]
    pub hop_destinations: Option<Vec<HopDestination>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Queue")]
    pub queue: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct HopDestination {
    #[serde(rename = "Priority")]
    pub priority: Option<usize>,
    #[serde(rename = "Queue")]
    pub queue: Option<String>,
    #[serde(rename = "WaitMinutes")]
    pub wait_minutes: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AccelerationSettings {
    #[serde(rename = "Mode")]
    pub mode: String,

}


}

pub mod cfn_preset {

#[derive(serde::Serialize, Default)]
pub struct CfnPreset {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Category")]
    pub category: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "SettingsJson")]
    pub settings_json: (),
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



}

pub mod cfn_queue {

#[derive(serde::Serialize, Default)]
pub struct CfnQueue {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,

}



}
