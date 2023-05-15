
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// The SNS topic provided to Application Insights that is associated to the created opsItem.
    #[serde(rename = "OpsItemSNSTopicArn")]
    pub ops_item_snstopic_arn: Option<String>,
    /// The name of the resource group.
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
    /// Indicates whether Application Insights can listen to CloudWatch events for the application resources.
    #[serde(rename = "CWEMonitorEnabled")]
    pub cwemonitor_enabled: Option<bool>,
    /// When set to true, creates opsItems for any problems detected on an application.
    #[serde(rename = "OpsCenterEnabled")]
    pub ops_center_enabled: Option<bool>,
    /// The log pattern sets.
    #[serde(rename = "LogPatternSets")]
    pub log_pattern_sets: Option<Vec<LogPatternSet>>,
    /// If set to true, application will be configured with recommended monitoring configuration.
    #[serde(rename = "AutoConfigurationEnabled")]
    pub auto_configuration_enabled: Option<bool>,
    /// The monitoring settings of the components.
    #[serde(rename = "ComponentMonitoringSettings")]
    pub component_monitoring_settings: Option<Vec<ComponentMonitoringSetting>>,
    /// The grouping type of the application
    #[serde(rename = "GroupingType")]
    pub grouping_type: Option<String>,
    /// The tags of Application Insights application.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The custom grouped components.
    #[serde(rename = "CustomComponents")]
    pub custom_components: Option<Vec<CustomComponent>>,

}


#[derive(serde::Serialize, Default)]
pub struct SubComponentTypeConfiguration {
    #[serde(rename = "SubComponentType")]
    pub sub_component_type: String,
    #[serde(rename = "SubComponentConfigurationDetails")]
    pub sub_component_configuration_details: SubComponentConfigurationDetails,

}

#[derive(serde::Serialize, Default)]
pub struct LogPattern {
    #[serde(rename = "PatternName")]
    pub pattern_name: String,
    #[serde(rename = "Pattern")]
    pub pattern: String,
    #[serde(rename = "Rank")]
    pub rank: usize,

}

#[derive(serde::Serialize, Default)]
pub struct HAClusterPrometheusExporter {
    #[serde(rename = "PrometheusPort")]
    pub prometheus_port: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct JMXPrometheusExporter {
    #[serde(rename = "PrometheusPort")]
    pub prometheus_port: Option<String>,
    #[serde(rename = "JMXURL")]
    pub jmxurl: Option<String>,
    #[serde(rename = "HostPort")]
    pub host_port: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigurationDetails {
    #[serde(rename = "HANAPrometheusExporter")]
    pub hanaprometheus_exporter: Option<HANAPrometheusExporter>,
    #[serde(rename = "HAClusterPrometheusExporter")]
    pub hacluster_prometheus_exporter: Option<HAClusterPrometheusExporter>,
    #[serde(rename = "JMXPrometheusExporter")]
    pub jmxprometheus_exporter: Option<JMXPrometheusExporter>,
    #[serde(rename = "AlarmMetrics")]
    pub alarm_metrics: Option<Vec<AlarmMetric>>,
    #[serde(rename = "Logs")]
    pub logs: Option<Vec<Log>>,
    #[serde(rename = "WindowsEvents")]
    pub windows_events: Option<Vec<WindowsEvent>>,
    #[serde(rename = "Alarms")]
    pub alarms: Option<Vec<Alarm>>,

}

#[derive(serde::Serialize, Default)]
pub struct HANAPrometheusExporter {
    #[serde(rename = "AgreeToInstallHANADBClient")]
    pub agree_to_install_hanadbclient: bool,
    #[serde(rename = "HANASecretName")]
    pub hanasecret_name: String,
    #[serde(rename = "PrometheusPort")]
    pub prometheus_port: Option<String>,
    #[serde(rename = "HANAPort")]
    pub hanaport: String,
    #[serde(rename = "HANASID")]
    pub hanasid: String,

}

#[derive(serde::Serialize, Default)]
pub struct Log {
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,
    #[serde(rename = "LogType")]
    pub log_type: String,
    #[serde(rename = "LogPath")]
    pub log_path: Option<String>,
    #[serde(rename = "Encoding")]
    pub encoding: Option<String>,
    #[serde(rename = "PatternSet")]
    pub pattern_set: Option<String>,

}
pub type EventLevel = String;
#[derive(serde::Serialize, Default)]
pub struct LogPatternSet {
    #[serde(rename = "LogPatterns")]
    pub log_patterns: Vec<LogPattern>,
    #[serde(rename = "PatternSetName")]
    pub pattern_set_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Alarm {
    #[serde(rename = "AlarmName")]
    pub alarm_name: String,
    #[serde(rename = "Severity")]
    pub severity: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentMonitoringSetting {
    #[serde(rename = "CustomComponentConfiguration")]
    pub custom_component_configuration: Option<ComponentConfiguration>,
    #[serde(rename = "ComponentName")]
    pub component_name: Option<String>,
    #[serde(rename = "Tier")]
    pub tier: String,
    #[serde(rename = "ComponentARN")]
    pub component_arn: Option<String>,
    #[serde(rename = "ComponentConfigurationMode")]
    pub component_configuration_mode: String,
    #[serde(rename = "DefaultOverwriteComponentConfiguration")]
    pub default_overwrite_component_configuration: Option<ComponentConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomComponent {
    #[serde(rename = "ResourceList")]
    pub resource_list: Vec<String>,
    #[serde(rename = "ComponentName")]
    pub component_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentConfiguration {
    #[serde(rename = "ConfigurationDetails")]
    pub configuration_details: Option<ConfigurationDetails>,
    #[serde(rename = "SubComponentTypeConfigurations")]
    pub sub_component_type_configurations: Option<Vec<SubComponentTypeConfiguration>>,

}

#[derive(serde::Serialize, Default)]
pub struct AlarmMetric {
    #[serde(rename = "AlarmMetricName")]
    pub alarm_metric_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct WindowsEvent {
    #[serde(rename = "PatternSet")]
    pub pattern_set: Option<String>,
    #[serde(rename = "EventLevels")]
    pub event_levels: Vec<EventLevel>,
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,
    #[serde(rename = "EventName")]
    pub event_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SubComponentConfigurationDetails {
    #[serde(rename = "AlarmMetrics")]
    pub alarm_metrics: Option<Vec<AlarmMetric>>,
    #[serde(rename = "Logs")]
    pub logs: Option<Vec<Log>>,
    #[serde(rename = "WindowsEvents")]
    pub windows_events: Option<Vec<WindowsEvent>>,

}


}
