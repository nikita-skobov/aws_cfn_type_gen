/// Adds a notification channel to DevOps Guru. A notification channel is used to notify you 			about important DevOps Guru events, such as when an insight is generated.
///
/// If you use an Amazon SNS topic in another account, you must attach a policy to it that grants DevOps Guru permission 				to send it notifications. DevOps Guru adds the required policy on your behalf to send notifications using Amazon SNS in your account. DevOps Guru only supports standard SNS topics. 				For more information, see Permissions 				for Amazon SNS topics.
///
/// If you use an Amazon SNS topic that is encrypted by an AWS Key Management Service customer-managed key (CMK), then you must add permissions 				to the CMK. For more information, see Permissions for 				AWS KMS–encrypted Amazon SNS topics.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnNotificationChannel {
    ///
    /// A NotificationChannelConfig object that contains information about 			configured notification channels.
    ///
    /// Required: Yes
    ///
    /// Type: NotificationChannelConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "Config")]
    pub config: NotificationChannelConfig,

    #[serde(skip_serializing)]
    pub att_id: CfnNotificationChannelid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNotificationChannelid;
impl CfnNotificationChannelid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnNotificationChannel {
    fn type_string(&self) -> &'static str {
        "AWS::DevOpsGuru::NotificationChannel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.config.validate()?;

        Ok(())
    }
}

/// Information about notification channels you have configured with DevOps Guru. 			The one    	supported notification channel is Amazon Simple Notification Service (Amazon SNS).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotificationChannelConfig {
    ///
    /// The filter configurations for the Amazon SNS notification topic you use with DevOps Guru. 			If you do not provide filter configurations, the default configurations are to receive notifications for all message types of High or Medium severity.
    ///
    /// Required: No
    ///
    /// Type: NotificationFilterConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<NotificationFilterConfig>,

    ///
    /// Information about a notification channel configured in DevOps Guru to send notifications 			when insights are created.
    ///
    /// If you use an Amazon SNS topic in another account, you must attach a policy to it that grants DevOps Guru permission 				to send it notifications. DevOps Guru adds the required policy on your behalf to send notifications using Amazon SNS in your account. DevOps Guru only supports standard SNS topics. 				For more information, see Permissions 				for Amazon SNS topics.
    ///
    /// If you use an Amazon SNS topic that is encrypted by an AWS Key Management Service customer-managed key (CMK), then you must add permissions 				to the CMK. For more information, see Permissions for 				AWS KMS–encrypted Amazon SNS topics.
    ///
    /// Required: No
    ///
    /// Type: SnsChannelConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "Sns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SnsChannelConfig>,
}

impl cfn_resources::CfnResource for NotificationChannelConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.filters.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sns.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The filter configurations for the Amazon SNS notification topic you use with DevOps Guru. You can choose to specify which events or message types to receive notifications for. 			You can also choose to specify which severity levels to receive notifications for.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotificationFilterConfig {
    ///
    /// The events that you want to receive notifications for. For example, you can choose to receive notifications only when the severity level is upgraded or a new insight is created.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "MessageTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_types: Option<Vec<String>>,

    ///
    /// The severity levels that you want to receive notifications for. For example, you can choose to receive notifications only for insights with HIGH and MEDIUM severity levels. 			For more information, see Understanding insight severities.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: Replacement
    #[serde(rename = "Severities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for NotificationFilterConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.message_types {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'message_types'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.severities {
            if the_val.len() > 3 as _ {
                return Err(format!(
                    "Max validation failed on field 'severities'. {} is greater than 3",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Contains the Amazon Resource Name (ARN) of an Amazon Simple Notification Service topic.
///
/// If you use an Amazon SNS topic in another account, you must attach a policy to it that grants DevOps Guru permission 				to send it notifications. DevOps Guru adds the required policy on your behalf to send notifications using Amazon SNS in your account. DevOps Guru only supports standard SNS topics. 				For more information, see Permissions 				for Amazon SNS topics.
///
/// If you use an Amazon SNS topic that is encrypted by an AWS Key Management Service customer-managed key (CMK), then you must add permissions 				to the CMK. For more information, see Permissions for 				AWS KMS–encrypted Amazon SNS topics.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SnsChannelConfig {
    ///
    /// The Amazon Resource Name (ARN) of an Amazon Simple Notification Service topic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 36
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^arn:aws[a-z0-9-]*:sns:[a-z0-9-]+:\d{12}:[^:]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "TopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SnsChannelConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.topic_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'topic_arn'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.topic_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 36 as _ {
                    return Err(format!(
                        "Min validation failed on field 'topic_arn'. {} is less than 36",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
