
pub mod cfn_firewall {

#[derive(serde::Serialize, Default)]
pub struct CfnFirewall {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FirewallName")]
    pub firewall_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DeleteProtection")]
    pub delete_protection: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetChangeProtection")]
    pub subnet_change_protection: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "FirewallPolicyChangeProtection")]
    pub firewall_policy_change_protection: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "FirewallPolicyArn")]
    pub firewall_policy_arn: ResourceArn,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of SubnetMapping
    #[serde(rename = "SubnetMappings")]
    pub subnet_mappings: Vec<SubnetMapping>,

}

pub type EndpointId = String;
#[derive(serde::Serialize, Default)]
pub struct SubnetMapping {
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    #[serde(rename = "IPAddressType")]
    pub ipaddress_type: Option<String>,

}
pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_firewall_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnFirewallPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "FirewallPolicy")]
    pub firewall_policy: FirewallPolicy,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "FirewallPolicyName")]
    pub firewall_policy_name: String,

}

pub type OverrideAction = String;
#[derive(serde::Serialize, Default)]
pub struct Dimension {
    #[serde(rename = "Value")]
    pub value: String,

}
pub type RuleOrder = String;pub type StreamExceptionPolicy = String;
#[derive(serde::Serialize, Default)]
pub struct FirewallPolicy {
    #[serde(rename = "StatelessFragmentDefaultActions")]
    pub stateless_fragment_default_actions: Vec<String>,
    #[serde(rename = "StatefulDefaultActions")]
    pub stateful_default_actions: Option<Vec<String>>,
    #[serde(rename = "StatelessDefaultActions")]
    pub stateless_default_actions: Vec<String>,
    #[serde(rename = "StatelessCustomActions")]
    pub stateless_custom_actions: Option<Vec<CustomAction>>,
    #[serde(rename = "StatefulRuleGroupReferences")]
    pub stateful_rule_group_references: Option<Vec<StatefulRuleGroupReference>>,
    #[serde(rename = "StatelessRuleGroupReferences")]
    pub stateless_rule_group_references: Option<Vec<StatelessRuleGroupReference>>,
    #[serde(rename = "StatefulEngineOptions")]
    pub stateful_engine_options: Option<StatefulEngineOptions>,
    #[serde(rename = "PolicyVariables")]
    pub policy_variables: Option<()>,

}
pub type Priority = usize;
#[derive(serde::Serialize, Default)]
pub struct StatefulEngineOptions {
    #[serde(rename = "RuleOrder")]
    pub rule_order: Option<RuleOrder>,
    #[serde(rename = "StreamExceptionPolicy")]
    pub stream_exception_policy: Option<StreamExceptionPolicy>,

}

#[derive(serde::Serialize, Default)]
pub struct PublishMetricAction {
    #[serde(rename = "Dimensions")]
    pub dimensions: Vec<Dimension>,

}
pub type VariableDefinition = String;
#[derive(serde::Serialize, Default)]
pub struct IPSet {
    #[serde(rename = "Definition")]
    pub definition: Option<Vec<VariableDefinition>>,

}

#[derive(serde::Serialize, Default)]
pub struct StatefulRuleGroupOverride {
    #[serde(rename = "Action")]
    pub action: Option<OverrideAction>,

}

#[derive(serde::Serialize, Default)]
pub struct StatelessRuleGroupReference {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: ResourceArn,
    #[serde(rename = "Priority")]
    pub priority: Priority,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ActionDefinition {
    #[serde(rename = "PublishMetricAction")]
    pub publish_metric_action: Option<PublishMetricAction>,

}

#[derive(serde::Serialize, Default)]
pub struct RuleVariables {

}

#[derive(serde::Serialize, Default)]
pub struct CustomAction {
    #[serde(rename = "ActionDefinition")]
    pub action_definition: ActionDefinition,
    #[serde(rename = "ActionName")]
    pub action_name: String,

}
pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct StatefulRuleGroupReference {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: ResourceArn,
    #[serde(rename = "Override")]
    pub overide: Option<StatefulRuleGroupOverride>,
    #[serde(rename = "Priority")]
    pub priority: Option<Priority>,

}


}

pub mod cfn_logging_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnLoggingConfiguration {
    /// No documentation provided by AWS
    #[serde(rename = "FirewallName")]
    pub firewall_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FirewallArn")]
    pub firewall_arn: ResourceArn,
    /// No documentation provided by AWS
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: LoggingConfiguration,

}


#[derive(serde::Serialize, Default)]
pub struct LogDestinationConfig {
    #[serde(rename = "LogDestinationType")]
    pub log_destination_type: String,
    #[serde(rename = "LogDestination")]
    pub log_destination: (),
    #[serde(rename = "LogType")]
    pub log_type: String,

}
pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct LoggingConfiguration {
    #[serde(rename = "LogDestinationConfigs")]
    pub log_destination_configs: Vec<LogDestinationConfig>,

}


}

pub mod cfn_rule_group {

#[derive(serde::Serialize, Default)]
pub struct CfnRuleGroup {
    /// No documentation provided by AWS
    #[serde(rename = "RuleGroupName")]
    pub rule_group_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "RuleGroup")]
    pub rule_group: Option<RuleGroup>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Capacity")]
    pub capacity: usize,

}


#[derive(serde::Serialize, Default)]
pub struct RuleDefinition {
    #[serde(rename = "MatchAttributes")]
    pub match_attributes: MatchAttributes,
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomAction {
    #[serde(rename = "ActionName")]
    pub action_name: String,
    #[serde(rename = "ActionDefinition")]
    pub action_definition: ActionDefinition,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct StatefulRule {
    #[serde(rename = "Header")]
    pub header: Header,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "RuleOptions")]
    pub rule_options: Vec<RuleOption>,

}

#[derive(serde::Serialize, Default)]
pub struct StatelessRulesAndCustomActions {
    #[serde(rename = "CustomActions")]
    pub custom_actions: Option<Vec<CustomAction>>,
    #[serde(rename = "StatelessRules")]
    pub stateless_rules: Vec<StatelessRule>,

}

#[derive(serde::Serialize, Default)]
pub struct StatelessRule {
    #[serde(rename = "RuleDefinition")]
    pub rule_definition: RuleDefinition,
    #[serde(rename = "Priority")]
    pub priority: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Dimension {
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct PortSet {
    #[serde(rename = "Definition")]
    pub definition: Option<Vec<VariableDefinition>>,

}

#[derive(serde::Serialize, Default)]
pub struct RuleOption {
    #[serde(rename = "Settings")]
    pub settings: Option<Vec<Setting>>,
    #[serde(rename = "Keyword")]
    pub keyword: String,

}

#[derive(serde::Serialize, Default)]
pub struct RuleVariables {
    #[serde(rename = "PortSets")]
    pub port_sets: Option<()>,
    #[serde(rename = "IPSets")]
    pub ipsets: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct ActionDefinition {
    #[serde(rename = "PublishMetricAction")]
    pub publish_metric_action: Option<PublishMetricAction>,

}
pub type RulesString = String;
#[derive(serde::Serialize, Default)]
pub struct RulesSource {
    #[serde(rename = "StatefulRules")]
    pub stateful_rules: Option<Vec<StatefulRule>>,
    #[serde(rename = "RulesString")]
    pub rules_string: Option<RulesString>,
    #[serde(rename = "RulesSourceList")]
    pub rules_source_list: Option<RulesSourceList>,
    #[serde(rename = "StatelessRulesAndCustomActions")]
    pub stateless_rules_and_custom_actions: Option<StatelessRulesAndCustomActions>,

}
pub type GeneratedRulesType = String;pub type VariableDefinition = String;pub type TargetType = String;
#[derive(serde::Serialize, Default)]
pub struct Address {
    #[serde(rename = "AddressDefinition")]
    pub address_definition: String,

}

#[derive(serde::Serialize, Default)]
pub struct StatefulRuleOptions {
    #[serde(rename = "RuleOrder")]
    pub rule_order: Option<RuleOrder>,

}

#[derive(serde::Serialize, Default)]
pub struct PublishMetricAction {
    #[serde(rename = "Dimensions")]
    pub dimensions: Vec<Dimension>,

}

#[derive(serde::Serialize, Default)]
pub struct MatchAttributes {
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<Address>>,
    #[serde(rename = "DestinationPorts")]
    pub destination_ports: Option<Vec<PortRange>>,
    #[serde(rename = "TCPFlags")]
    pub tcpflags: Option<Vec<TCPFlagField>>,
    #[serde(rename = "Protocols")]
    pub protocols: Option<Vec<ProtocolNumber>>,
    #[serde(rename = "SourcePorts")]
    pub source_ports: Option<Vec<PortRange>>,
    #[serde(rename = "Sources")]
    pub sources: Option<Vec<Address>>,

}

#[derive(serde::Serialize, Default)]
pub struct IPSet {
    #[serde(rename = "Definition")]
    pub definition: Option<Vec<VariableDefinition>>,

}

#[derive(serde::Serialize, Default)]
pub struct Header {
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "DestinationPort")]
    pub destination_port: Port,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Direction")]
    pub direction: String,
    #[serde(rename = "SourcePort")]
    pub source_port: Port,

}

#[derive(serde::Serialize, Default)]
pub struct IPSetReference {
    #[serde(rename = "ReferenceArn")]
    pub reference_arn: Option<ResourceArn>,

}
pub type RuleOrder = String;
#[derive(serde::Serialize, Default)]
pub struct RulesSourceList {
    #[serde(rename = "TargetTypes")]
    pub target_types: Vec<TargetType>,
    #[serde(rename = "Targets")]
    pub targets: Vec<String>,
    #[serde(rename = "GeneratedRulesType")]
    pub generated_rules_type: GeneratedRulesType,

}
pub type ResourceArn = String;pub type TCPFlag = String;pub type Setting = String;pub type ProtocolNumber = usize;
#[derive(serde::Serialize, Default)]
pub struct PortRange {
    #[serde(rename = "ToPort")]
    pub to_port: PortRangeBound,
    #[serde(rename = "FromPort")]
    pub from_port: PortRangeBound,

}
pub type Port = String;
#[derive(serde::Serialize, Default)]
pub struct TCPFlagField {
    #[serde(rename = "Masks")]
    pub masks: Option<Vec<TCPFlag>>,
    #[serde(rename = "Flags")]
    pub flags: Vec<TCPFlag>,

}
pub type PortRangeBound = usize;
#[derive(serde::Serialize, Default)]
pub struct ReferenceSets {
    #[serde(rename = "IPSetReferences")]
    pub ipset_references: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct RuleGroup {
    #[serde(rename = "RulesSource")]
    pub rules_source: RulesSource,
    #[serde(rename = "RuleVariables")]
    pub rule_variables: Option<RuleVariables>,
    #[serde(rename = "StatefulRuleOptions")]
    pub stateful_rule_options: Option<StatefulRuleOptions>,
    #[serde(rename = "ReferenceSets")]
    pub reference_sets: Option<ReferenceSets>,

}


}
