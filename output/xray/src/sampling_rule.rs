/// Use the AWS::XRay::SamplingRule resource to specify a sampling rule, which controls sampling behavior for instrumented applications.    Include a SamplingRule entity to create or update a sampling rule.
///
/// Services retrieve rules with GetSamplingRules, and evaluate each rule in ascending    order of priority for each request. If a rule matches, the service records a trace, borrowing it from the reservoir size. After 10 seconds, the service    reports back to X-Ray with GetSamplingTargets to get updated versions of    each in-use rule. The updated rule contains a trace quota that the service can use instead of borrowing from the reservoir.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSamplingRule {
    ///
    /// The sampling rule to be created or updated.
    ///
    /// Required: No
    ///
    /// Type: SamplingRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamplingRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule: Option<Box<SamplingRule>>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_rule_arn: CfnSamplingRulerulearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSamplingRulerulearn;
impl CfnSamplingRulerulearn {
    pub fn att_name(&self) -> &'static str {
        r#"RuleARN"#
    }
}

impl cfn_resources::CfnResource for CfnSamplingRule {
    fn type_string(&self) -> &'static str {
        "AWS::XRay::SamplingRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.sampling_rule
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A sampling rule that services use to decide whether to instrument a request. Rule    fields can match properties of the service, or properties of a request. The service can ignore    rules that don't match its properties.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SamplingRule {
    ///
    /// Matches attributes derived from the request.
    ///
    /// Map Entries: Maximum number of 5 items.
    ///
    /// Key Length Constraints: Minimum length of 1. Maximum length of 32.
    ///
    /// Value Length Constraints: Minimum length of 1. Maximum length of 32.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,

    ///
    /// The percentage of matching requests to instrument, after the reservoir is    exhausted.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedRate")]
    pub fixed_rate: f64,

    ///
    /// Matches the HTTP method of a request.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTPMethod")]
    pub httpmethod: cfn_resources::StrVal,

    ///
    /// Matches the hostname from a request URL.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: cfn_resources::StrVal,

    ///
    /// The priority of the sampling rule.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 9999
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,

    ///
    /// A fixed number of matching requests to instrument per second, prior to applying the    fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReservoirSize")]
    pub reservoir_size: i64,

    ///
    /// Matches the ARN of the AWS resource on which the service runs.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceARN")]
    pub resource_arn: cfn_resources::StrVal,

    ///
    /// The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.
    ///
    /// NoteSpecifying a sampling rule by name is recommended, as specifying by      ARN will be deprecated in future.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the sampling rule. Specify a rule by either name or ARN, but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<cfn_resources::StrVal>,

    ///
    /// Matches the name that the service uses to identify itself in segments.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: cfn_resources::StrVal,

    ///
    /// Matches the origin that the service uses to identify its type in segments.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceType")]
    pub service_type: cfn_resources::StrVal,

    ///
    /// Matches the path from a request URL.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "URLPath")]
    pub urlpath: cfn_resources::StrVal,

    ///
    /// The version of the sampling rule. Version can only be set when creating a new sampling rule.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl cfn_resources::CfnResource for SamplingRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.httpmethod;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 10 as _ {
                return Err(format!(
                    "Max validation failed on field 'httpmethod'. {} is greater than 10",
                    s.len()
                ));
            }
        }

        let the_val = &self.host;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'host'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.priority;

        if *the_val > 9999 as _ {
            return Err(format!(
                "Max validation failed on field 'priority'. {} is greater than 9999",
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

        let the_val = &self.reservoir_size;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'reservoir_size'. {} is less than 0",
                the_val
            ));
        }

        let the_val = &self.resource_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 500 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_arn'. {} is greater than 500",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.rule_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 32 as _ {
                    return Err(format!(
                        "Max validation failed on field 'rule_name'. {} is greater than 32",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.rule_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'rule_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.service_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'service_name'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.service_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'service_type'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.urlpath;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'urlpath'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.version {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'version'. {} is less than 1",
                    the_val
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
#[serde(default)]
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
