/// Use the AWS::NetworkFirewall::FirewallPolicy to define the stateless and stateful network traffic filtering behavior for your AWS::NetworkFirewall::Firewall. You can use one firewall policy for multiple firewalls.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallPolicy {
    ///
    /// A description of the firewall policy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^.*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The traffic filtering behavior of a firewall policy, defined in a collection of stateless     and stateful rule groups and other settings.
    ///
    /// Required: Yes
    ///
    /// Type: FirewallPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirewallPolicy")]
    pub firewall_policy: Box<FirewallPolicy>,

    ///
    /// The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirewallPolicyName")]
    pub firewall_policy_name: cfn_resources::StrVal,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_firewall_policy_arn: CfnFirewallPolicyfirewallpolicyarn,

    #[serde(skip_serializing)]
    pub att_firewall_policy_id: CfnFirewallPolicyfirewallpolicyid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallPolicyfirewallpolicyarn;
impl CfnFirewallPolicyfirewallpolicyarn {
    pub fn att_name(&self) -> &'static str {
        r#"FirewallPolicyArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallPolicyfirewallpolicyid;
impl CfnFirewallPolicyfirewallpolicyid {
    pub fn att_name(&self) -> &'static str {
        r#"FirewallPolicyId"#
    }
}

impl cfn_resources::CfnResource for CfnFirewallPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkFirewall::FirewallPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        self.firewall_policy.validate()?;

        let the_val = &self.firewall_policy_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'firewall_policy_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.firewall_policy_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'firewall_policy_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A custom action to use in stateless rule actions settings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ActionDefinition {
    ///
    /// Stateless inspection criteria that publishes the specified metrics to Amazon CloudWatch for the     matching packet. This setting defines a CloudWatch dimension value to be published.
    ///
    /// You can pair this custom action with any of the standard stateless rule actions. For     example, you could pair this in a rule action with the standard action that forwards the     packet for stateful inspection. Then, when a packet matches the rule, Network Firewall     publishes metrics for the packet and forwards it.
    ///
    /// Required: No
    ///
    /// Type: PublishMetricAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublishMetricAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_metric_action: Option<PublishMetricAction>,
}

impl cfn_resources::CfnResource for ActionDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.publish_metric_action
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An optional, non-standard action to use for stateless packet handling. You can define     this in addition to the standard action that you must specify.
///
/// You define and name the custom actions that you want to be able to use, and then you     reference them by name in your actions settings.
///
/// You can use custom actions in the following places:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomAction {
    ///
    /// The custom action associated with the action name.
    ///
    /// Required: Yes
    ///
    /// Type: ActionDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionDefinition")]
    pub action_definition: ActionDefinition,

    ///
    /// The descriptive name of the custom action. You can't change the name of a custom action after you create it.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionName")]
    pub action_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CustomAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.action_definition.validate()?;

        let the_val = &self.action_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'action_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.action_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'action_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The value to use in an Amazon CloudWatch custom metric dimension. This is used in the       PublishMetrics custom action. A CloudWatch custom metric dimension is a name/value pair that's     part of the identity of a metric.
///
/// AWS Network Firewall sets the dimension name to CustomAction and you provide the     dimension value.
///
/// For more information about CloudWatch custom metric dimensions, see      Publishing Custom Metrics in the Amazon CloudWatch User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Dimension {
    ///
    /// The value to use in the custom metric dimension.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9-_ ]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Dimension {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'value'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The traffic filtering behavior of a firewall policy, defined in a collection of stateless     and stateful rule groups and other settings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FirewallPolicy {
    ///
    /// Contains variables that you can use to override default Suricata settings in your firewall policy.
    ///
    /// Required: No
    ///
    /// Type: PolicyVariables
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_variables: Option<PolicyVariables>,

    ///
    /// The default actions to take on a packet that doesn't match any stateful rules. The stateful default action is optional,      and is only valid when using the strict rule order.
    ///
    /// Valid values of the stateful default action:
    ///
    /// aws:drop_strict               aws:drop_established               aws:alert_strict               aws:alert_established
    ///
    /// For more information, see      Strict evaluation order in the AWS Network Firewall Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatefulDefaultActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_default_actions: Option<Vec<String>>,

    ///
    /// Additional options governing how Network Firewall handles stateful rules. The stateful    rule groups that you use in your policy must have stateful rule options settings that are compatible with these settings.
    ///
    /// Required: No
    ///
    /// Type: StatefulEngineOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatefulEngineOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_engine_options: Option<StatefulEngineOptions>,

    ///
    /// References to the stateful rule groups that are used in the policy. These define the     inspection criteria in stateful rules.
    ///
    /// Required: No
    ///
    /// Type: List of StatefulRuleGroupReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatefulRuleGroupReferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_rule_group_references: Option<Vec<StatefulRuleGroupReference>>,

    ///
    /// The custom action definitions that are available for use in the firewall policy's       StatelessDefaultActions setting. You name each custom action that you     define, and then you can use it by name in your default actions specifications.
    ///
    /// Required: No
    ///
    /// Type: List of CustomAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatelessCustomActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_custom_actions: Option<Vec<CustomAction>>,

    ///
    /// The actions to take on a packet if it doesn't match any of the stateless rules in the     policy. If you want non-matching packets to be forwarded for stateful inspection, specify       aws:forward_to_sfe.
    ///
    /// You must specify one of the standard actions: aws:pass,       aws:drop, or aws:forward_to_sfe. In addition, you can specify     custom actions that are compatible with your standard section choice.
    ///
    /// For example, you could specify ["aws:pass"] or you could specify       ["aws:pass", “customActionName”]. For information about compatibility, see     the custom action descriptions.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatelessDefaultActions")]
    pub stateless_default_actions: Vec<String>,

    ///
    /// The actions to take on a fragmented packet if it doesn't match any of the stateless     rules in the policy. If you want non-matching fragmented packets to be forwarded for     stateful inspection, specify aws:forward_to_sfe.
    ///
    /// You must specify one of the standard actions: aws:pass,       aws:drop, or aws:forward_to_sfe. In addition, you can specify     custom actions that are compatible with your standard section choice.
    ///
    /// For example, you could specify ["aws:pass"] or you could specify       ["aws:pass", “customActionName”]. For information about compatibility, see     the custom action descriptions.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatelessFragmentDefaultActions")]
    pub stateless_fragment_default_actions: Vec<String>,

    ///
    /// References to the stateless rule groups that are used in the policy. These define the     matching criteria in stateless rules.
    ///
    /// Required: No
    ///
    /// Type: List of StatelessRuleGroupReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatelessRuleGroupReferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_rule_group_references: Option<Vec<StatelessRuleGroupReference>>,
}

impl cfn_resources::CfnResource for FirewallPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.policy_variables
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.stateful_engine_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A list of IP addresses and address ranges, in CIDR notation. This is part of a RuleVariables.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IPSet {
    ///
    /// The list of IP addresses and address ranges, in CIDR notation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for IPSet {
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

/// Contains variables that you can use to override default Suricata settings in your firewall policy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PolicyVariables {
    ///
    /// The IPv4 or IPv6 addresses in CIDR notation to use for the Suricata HOME_NET variable. If your firewall uses an inspection VPC, you might want to override the HOME_NET variable with the CIDRs of your home networks. If you don't override HOME_NET with your own CIDRs, Network Firewall by default uses the CIDR of your inspection VPC.
    ///
    /// Required: No
    ///
    /// Type: Map of IPSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_variables: Option<std::collections::HashMap<String, IPSet>>,
}

impl cfn_resources::CfnResource for PolicyVariables {
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

/// Stateless inspection criteria that publishes the specified metrics to Amazon CloudWatch for the     matching packet. This setting defines a CloudWatch dimension value to be published.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PublishMetricAction {
    ///
    ///
    ///
    /// Required: Yes
    ///
    /// Type: List of Dimension
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Vec<Dimension>,
}

impl cfn_resources::CfnResource for PublishMetricAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.dimensions;

        if the_val.len() > 1 as _ {
            return Err(format!(
                "Max validation failed on field 'dimensions'. {} is greater than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Configuration settings for the handling of the stateful rule groups in a firewall policy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatefulEngineOptions {
    ///
    /// Indicates how to manage the order of stateful rule evaluation for the policy. DEFAULT_ACTION_ORDER is     the default behavior. Stateful rules are provided to the rule engine as Suricata compatible strings, and Suricata evaluates them     based on certain settings. For more information, see     Evaluation order for stateful rules in the         AWS Network Firewall Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEFAULT_ACTION_ORDER | STRICT_ORDER
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<StatefulEngineOptionsRuleOrderEnum>,

    ///
    /// Configures how Network Firewall processes traffic when a network connection breaks midstream. Network connections can break due to disruptions in external networks or within the firewall itself.
    ///
    /// DROP - Network Firewall fails closed and drops all subsequent traffic going to the firewall. This is the default behavior.                        CONTINUE - Network Firewall continues to apply rules to the subsequent traffic without context from traffic before the break. This impacts the behavior of rules that depend on this context. For example, if you have a stateful rule to drop http traffic, Network Firewall won't match the traffic for this rule because the service won't have the context from session initialization defining the application layer protocol as HTTP. However, this behavior is rule dependent—a TCP-layer rule using a flow:stateless rule would still match, as would the aws:drop_strict default action.                        REJECT - Network Firewall fails closed and drops all subsequent traffic going to the firewall. Network Firewall also sends a TCP reject packet back to your client so that the client can immediately establish a new session. Network Firewall will have context about the new session and will apply rules to the subsequent traffic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONTINUE | DROP | REJECT
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamExceptionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_exception_policy: Option<StatefulEngineOptionsStreamExceptionPolicyEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StatefulEngineOptionsRuleOrderEnum {
    /// DEFAULT_ACTION_ORDER
    #[serde(rename = "DEFAULT_ACTION_ORDER")]
    Defaultactionorder,

    /// STRICT_ORDER
    #[serde(rename = "STRICT_ORDER")]
    Strictorder,
}

impl Default for StatefulEngineOptionsRuleOrderEnum {
    fn default() -> Self {
        StatefulEngineOptionsRuleOrderEnum::Defaultactionorder
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StatefulEngineOptionsStreamExceptionPolicyEnum {
    /// CONTINUE
    #[serde(rename = "CONTINUE")]
    Continue,

    /// DROP
    #[serde(rename = "DROP")]
    Drop,

    /// REJECT
    #[serde(rename = "REJECT")]
    Reject,
}

impl Default for StatefulEngineOptionsStreamExceptionPolicyEnum {
    fn default() -> Self {
        StatefulEngineOptionsStreamExceptionPolicyEnum::Continue
    }
}

impl cfn_resources::CfnResource for StatefulEngineOptions {
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

/// The setting that allows the policy owner to change the behavior of the rule group within a policy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatefulRuleGroupOverride {
    ///
    /// The action that changes the rule group from DROP to ALERT. This only applies to    managed rule groups.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DROP_TO_ALERT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<StatefulRuleGroupOverrideActionEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StatefulRuleGroupOverrideActionEnum {
    /// DROP_TO_ALERT
    #[serde(rename = "DROP_TO_ALERT")]
    Droptoalert,
}

impl Default for StatefulRuleGroupOverrideActionEnum {
    fn default() -> Self {
        StatefulRuleGroupOverrideActionEnum::Droptoalert
    }
}

impl cfn_resources::CfnResource for StatefulRuleGroupOverride {
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

/// Identifier for a single stateful rule group, used in a firewall policy to refer to a     rule group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatefulRuleGroupReference {
    ///
    /// The action that allows the policy owner to override the behavior of the rule group within a policy.
    ///
    /// Required: No
    ///
    /// Type: StatefulRuleGroupOverride
    ///
    /// Update requires: No interruption
    #[serde(rename = "Override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_override: Option<StatefulRuleGroupOverride>,

    ///
    /// An integer setting that indicates the order in which to run the stateful rule groups in    a single AWS::NetworkFirewall::FirewallPolicy. This setting only applies to firewall policies    that specify the STRICT_ORDER rule order in the stateful engine options settings.
    ///
    /// Network Firewall evalutes each stateful rule group     against a packet starting with the group that has the lowest priority setting. You must ensure     that the priority settings are unique within each policy.
    ///
    /// You can change the priority settings of your rule groups at any time. To make it easier to     insert rule groups later, number them so there's a wide range in between, for example use 100,     200, and so on.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    ///
    /// The Amazon Resource Name (ARN) of the stateful rule group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceArn")]
    pub resource_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StatefulRuleGroupReference {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cfn_override
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.priority {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'priority'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.priority {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'priority'. {} is less than 1",
                    the_val
                ));
            }
        }

        let the_val = &self.resource_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_arn'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.resource_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'resource_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Identifier for a single stateless rule group, used in a firewall policy to refer to the     rule group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatelessRuleGroupReference {
    ///
    /// An integer setting that indicates the order in which to run the stateless rule groups in     a single AWS::NetworkFirewall::FirewallPolicy. Network Firewall applies each stateless rule group     to a packet starting with the group that has the lowest priority setting. You must ensure     that the priority settings are unique within each policy.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,

    ///
    /// The Amazon Resource Name (ARN) of the stateless rule group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceArn")]
    pub resource_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StatelessRuleGroupReference {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.priority;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'priority'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.priority;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'priority'. {} is less than 1",
                the_val
            ));
        }

        let the_val = &self.resource_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_arn'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.resource_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'resource_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
