
pub mod cfn_rule_groups_namespace {

#[derive(serde::Serialize, Default)]
pub struct CfnRuleGroupsNamespace {
    /// Required to identify a specific APS Workspace associated with this RuleGroupsNamespace.
    #[serde(rename = "Workspace")]
    pub workspace: String,
    /// The RuleGroupsNamespace data.
    #[serde(rename = "Data")]
    pub data: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The RuleGroupsNamespace name.
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_workspace {

#[derive(serde::Serialize, Default)]
pub struct CfnWorkspace {
    /// Logging configuration
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: Option<LoggingConfiguration>,
    /// AMP Workspace alias.
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The AMP Workspace alert manager definition data
    #[serde(rename = "AlertManagerDefinition")]
    pub alert_manager_definition: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingConfiguration {
    #[serde(rename = "LogGroupArn")]
    pub log_group_arn: Option<String>,

}


}
