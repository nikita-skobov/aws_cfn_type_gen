
pub mod cfn_geofence_collection {

#[derive(serde::Serialize, Default)]
pub struct CfnGeofenceCollection {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CollectionName")]
    pub collection_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "PricingPlanDataSource")]
    pub pricing_plan_data_source: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: Option<PricingPlan>,

}

pub type iso8601UTC = String;pub type PricingPlan = String;

}

pub mod cfn_map {

#[derive(serde::Serialize, Default)]
pub struct CfnMap {
    /// No documentation provided by AWS
    #[serde(rename = "Configuration")]
    pub configuration: MapConfiguration,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: Option<PricingPlan>,
    /// No documentation provided by AWS
    #[serde(rename = "MapName")]
    pub map_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct MapConfiguration {
    #[serde(rename = "Style")]
    pub style: String,

}
pub type iso8601UTC = String;pub type PricingPlan = String;

}

pub mod cfn_place_index {

#[derive(serde::Serialize, Default)]
pub struct CfnPlaceIndex {
    /// No documentation provided by AWS
    #[serde(rename = "DataSource")]
    pub data_source: String,
    /// No documentation provided by AWS
    #[serde(rename = "DataSourceConfiguration")]
    pub data_source_configuration: Option<DataSourceConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "IndexName")]
    pub index_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: Option<PricingPlan>,

}

pub type IntendedUse = String;
#[derive(serde::Serialize, Default)]
pub struct DataSourceConfiguration {
    #[serde(rename = "IntendedUse")]
    pub intended_use: Option<IntendedUse>,

}
pub type PricingPlan = String;pub type iso8601UTC = String;

}

pub mod cfn_route_calculator {

#[derive(serde::Serialize, Default)]
pub struct CfnRouteCalculator {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CalculatorName")]
    pub calculator_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: Option<PricingPlan>,
    /// No documentation provided by AWS
    #[serde(rename = "DataSource")]
    pub data_source: String,

}

pub type iso8601UTC = String;pub type PricingPlan = String;

}

pub mod cfn_tracker {

#[derive(serde::Serialize, Default)]
pub struct CfnTracker {
    /// No documentation provided by AWS
    #[serde(rename = "PricingPlanDataSource")]
    pub pricing_plan_data_source: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PositionFiltering")]
    pub position_filtering: Option<PositionFiltering>,
    /// No documentation provided by AWS
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: Option<PricingPlan>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TrackerName")]
    pub tracker_name: String,

}

pub type iso8601UTC = String;pub type PositionFiltering = String;pub type PricingPlan = String;

}

pub mod cfn_tracker_consumer {

#[derive(serde::Serialize, Default)]
pub struct CfnTrackerConsumer {
    /// No documentation provided by AWS
    #[serde(rename = "TrackerName")]
    pub tracker_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConsumerArn")]
    pub consumer_arn: String,

}



}
