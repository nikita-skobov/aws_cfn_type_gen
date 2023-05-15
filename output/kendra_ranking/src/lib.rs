
pub mod cfn_execution_plan {

#[derive(serde::Serialize, Default)]
pub struct CfnExecutionPlan {
    /// Tags for labeling the execution plan
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// Name of kendra ranking rescore execution plan
    #[serde(rename = "Name")]
    pub name: Name,
    /// Capacity units
    #[serde(rename = "CapacityUnits")]
    pub capacity_units: Option<CapacityUnitsConfiguration>,
    /// A description for the execution plan
    #[serde(rename = "Description")]
    pub description: Option<Description>,

}

pub type Arn = String;pub type Name = String;pub type RescoreCapacityUnits = usize;pub type Description = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type Id = String;
#[derive(serde::Serialize, Default)]
pub struct CapacityUnitsConfiguration {
    #[serde(rename = "RescoreCapacityUnits")]
    pub rescore_capacity_units: RescoreCapacityUnits,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}
