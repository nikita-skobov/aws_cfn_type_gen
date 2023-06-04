/// The AWS::Logs::SubscriptionFilter resource specifies a subscription filter and associates it with the specified log    group. Subscription filters allow you to subscribe to a real-time stream of log events    and have them delivered to a specific    destination. Currently, the supported destinations are:
///
/// There can be as many as two subscription filters associated with a log group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSubscriptionFilter {
    ///
    /// The Amazon Resource Name (ARN) of the destination.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: cfn_resources::StrVal,

    ///
    /// The method used to distribute log data to the destination, which can be either    random or grouped by log stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Distribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<cfn_resources::StrVal>,

    ///
    /// The name of the subscription filter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [^:*]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "FilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<cfn_resources::StrVal>,

    ///
    /// The filtering expressions that restrict what gets delivered to the destination AWS resource.      For more information about the filter pattern syntax, see      Filter and Pattern Syntax.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterPattern")]
    pub filter_pattern: cfn_resources::StrVal,

    ///
    /// The log group to associate with the subscription filter. All log events that are     uploaded to this log group are filtered and delivered to the specified AWS resource if the filter pattern matches the log events.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\.\-_/#A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogGroupName")]
    pub log_group_name: cfn_resources::StrVal,

    ///
    /// The ARN of an IAM role that grants CloudWatch Logs permissions to deliver ingested log events to the destination      stream. You don't need to provide the ARN when you are working with a logical destination for cross-account delivery.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnSubscriptionFilter {
    fn type_string(&self) -> &'static str {
        "AWS::Logs::SubscriptionFilter"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.destination_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'destination_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.filter_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'filter_name'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.filter_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'filter_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.log_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'log_group_name'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.log_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'log_group_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'role_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
