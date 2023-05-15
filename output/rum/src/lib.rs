
pub mod cfn_app_monitor {

#[derive(serde::Serialize, Default)]
pub struct CfnAppMonitor {
    /// AppMonitor custom events configuration
    #[serde(rename = "CustomEvents")]
    pub custom_events: Option<CustomEvents>,
    /// The top-level internet domain name for which your application has administrative authority.
    #[serde(rename = "Domain")]
    pub domain: String,
    /// Data collected by RUM is kept by RUM for 30 days and then deleted. This parameter specifies whether RUM sends a copy of this telemetry data to CWLlong in your account. This enables you to keep the telemetry data for more than 30 days, but it does incur CWLlong charges. If you omit this parameter, the default is false
    #[serde(rename = "CwLogEnabled")]
    pub cw_log_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<TagDef>,
    /// A name for the app monitor
    #[serde(rename = "Name")]
    pub name: String,
    /// AppMonitor configuration
    #[serde(rename = "AppMonitorConfiguration")]
    pub app_monitor_configuration: Option<AppMonitorConfiguration>,

}

pub type CustomEventsStatus = String;pub type Arn = String;pub type FavoritePages = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct AppMonitorConfiguration {
    #[serde(rename = "SessionSampleRate")]
    pub session_sample_rate: Option<f64>,
    #[serde(rename = "MetricDestinations")]
    pub metric_destinations: Option<Vec<MetricDestination>>,
    #[serde(rename = "IncludedPages")]
    pub included_pages: Option<Pages>,
    #[serde(rename = "GuestRoleArn")]
    pub guest_role_arn: Option<Arn>,
    #[serde(rename = "FavoritePages")]
    pub favorite_pages: Option<FavoritePages>,
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "ExcludedPages")]
    pub excluded_pages: Option<Pages>,
    #[serde(rename = "Telemetries")]
    pub telemetries: Option<Vec<Telemetry>>,
    #[serde(rename = "EnableXRay")]
    pub enable_xray: Option<bool>,
    #[serde(rename = "AllowCookies")]
    pub allow_cookies: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricDestination {
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "MetricDefinitions")]
    pub metric_definitions: Option<Vec<MetricDefinition>>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomEvents {
    #[serde(rename = "Status")]
    pub status: Option<CustomEventsStatus>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct TagDef {

}
pub type Telemetry = String;
#[derive(serde::Serialize, Default)]
pub struct MetricDefinition {
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "ValueKey")]
    pub value_key: Option<String>,
    #[serde(rename = "UnitLabel")]
    pub unit_label: Option<String>,
    #[serde(rename = "EventPattern")]
    pub event_pattern: Option<String>,
    #[serde(rename = "DimensionKeys")]
    pub dimension_keys: Option<()>,
    #[serde(rename = "Name")]
    pub name: String,

}
pub type Url = String;
#[derive(serde::Serialize, Default)]
pub struct Pages {

}


}
