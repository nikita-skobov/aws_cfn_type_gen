/// Creates a notification rule for a resource. The rule specifies the events you want       notifications about and the targets (such as AWS Chatbot topics or AWS Chatbot clients configured for Slack) where you want to receive       them.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnNotificationRule {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub created_by: Option<cfn_resources::StrVal>,

    ///
    /// The level of detail to include in the notifications for this resource. BASIC will include only the     contents of the event as it would appear in Amazon CloudWatch. FULL will include any supplemental information     provided by AWS CodeStar Notifications and/or the service for the resource for which the notification is created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BASIC | FULL
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetailType")]
    pub detail_type: NotificationRuleDetailTypeEnum,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventTypeId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub event_type_id: Option<cfn_resources::StrVal>,

    ///
    /// A list of event types associated with this notification rule. For a complete list of event types and IDs, see      Notification concepts     in the Developer Tools Console User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventTypeIds")]
    pub event_type_ids: Vec<String>,

    ///
    /// The name for the notification rule. Notification rule names must be unique in your AWS account.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [A-Za-z0-9\-_ ]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the resource to associate with the notification rule. Supported resources include pipelines in AWS CodePipeline,    repositories in AWS CodeCommit, and build projects in AWS CodeBuild.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws[^:\s]*:[^:\s]*:[^:\s]*:[0-9]{12}:[^\s]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resource")]
    pub resource: cfn_resources::StrVal,

    ///
    /// The status of the notification rule. The default value is ENABLED. If the status is       set to DISABLED, notifications aren't sent for the notification rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub status: Option<NotificationRuleStatusEnum>,

    ///
    /// A list of tags to apply to this notification rule. Key names cannot start with "aws".
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetAddress")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_address: Option<cfn_resources::StrVal>,

    ///
    /// A list of Amazon Resource Names (ARNs) of Amazon Simple Notification Service topics and AWS Chatbot clients to associate with the    notification rule.
    ///
    /// Required: Yes
    ///
    /// Type: List of Target
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,

    #[serde(skip_serializing)]
    pub att_arn: CfnNotificationRulearn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NotificationRuleDetailTypeEnum {
    /// BASIC
    #[serde(rename = "BASIC")]
    Basic,

    /// FULL
    #[serde(rename = "FULL")]
    Full,
}

impl Default for NotificationRuleDetailTypeEnum {
    fn default() -> Self {
        NotificationRuleDetailTypeEnum::Basic
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NotificationRuleStatusEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for NotificationRuleStatusEnum {
    fn default() -> Self {
        NotificationRuleStatusEnum::Disabled
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNotificationRulearn;
impl CfnNotificationRulearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnNotificationRule {
    fn type_string(&self) -> &'static str {
        "AWS::CodeStarNotifications::NotificationRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.targets;

        if the_val.len() > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'targets'. {} is greater than 10",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Information about the AWS Chatbot topics or AWS Chatbot clients associated with a notification rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Target {
    ///
    /// The Amazon Resource Name (ARN) of the AWS Chatbot topic or AWS Chatbot client.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 320
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetAddress")]
    pub target_address: cfn_resources::StrVal,

    ///
    /// The target type. Can be an Amazon Simple Notification Service topic or AWS Chatbot client.
    ///
    /// Amazon Simple Notification Service topics are specified as SNS.AWS Chatbot clients are specified as AWSChatbotSlack.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[A-Za-z]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetType")]
    pub target_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Target {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.target_address;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 320 as _ {
                return Err(format!(
                    "Max validation failed on field 'target_address'. {} is greater than 320",
                    s.len()
                ));
            }
        }

        let the_val = &self.target_address;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_address'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
