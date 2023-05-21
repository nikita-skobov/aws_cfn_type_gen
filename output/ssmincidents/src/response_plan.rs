/// The AWS::SSMIncidents::ResponsePlan resource specifies the details of the       response plan that are used when creating an incident.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResponsePlan {
    ///
    /// The actions that the response plan starts at the beginning of an incident.
    ///
    /// Required: No
    ///
    /// Type: List of Action
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,

    ///
    /// The AWS Chatbot chat channel used for collaboration during an       incident.
    ///
    /// Required: No
    ///
    /// Type: ChatChannel
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChatChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_channel: Option<ChatChannel>,

    ///
    /// The human readable name of the response plan.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) for the contacts and escalation plans that the response       plan engages during an incident.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Engagements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagements: Option<Vec<String>>,

    ///
    /// Details used to create an incident when using this response plan.
    ///
    /// Required: Yes
    ///
    /// Type: IncidentTemplate
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncidentTemplate")]
    pub incident_template: IncidentTemplate,

    ///
    /// Information about third-party services integrated into the response plan.
    ///
    /// Required: No
    ///
    /// Type: List of Integration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Integrations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<Integration>>,

    ///
    /// The name of the response plan.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^[a-zA-Z0-9-_]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnResponsePlan {
    fn type_string(&self) -> &'static str {
        "AWS::SSMIncidents::ResponsePlan"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.actions {
            if the_val.len() > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'actions'. {} is greater than 1",
                    the_val.len()
                ));
            }
        }

        self.chat_channel
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.display_name {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'display_name'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.display_name {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'display_name'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.engagements {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'engagements'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        self.incident_template.validate()?;

        let the_val = &self.name;

        if the_val.len() > 200 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 200",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The Action property type specifies the configuration to launch.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {
    ///
    /// Details about the Systems Manager automation document that will be used as a       runbook during an incident.
    ///
    /// Required: No
    ///
    /// Type: SsmAutomation
    ///
    /// Update requires: No interruption
    #[serde(rename = "SsmAutomation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_automation: Option<SsmAutomation>,
}

impl cfn_resources::CfnResource for Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ssm_automation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AWS Chatbot chat channel used for collaboration during an       incident.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ChatChannel {
    ///
    /// The SNS targets that AWS Chatbot uses to notify the chat channel of updates       to an incident. You can also make updates to the incident through the chat channel by       using the SNS topics
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChatbotSns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatbot_sns: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for ChatChannel {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.chatbot_sns {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'chatbot_sns'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// When you add a runbook to a response plan, you can specify the parameters the runbook       should use at runtime. Response plans support parameters with both static and dynamic       values. For static values, you enter the value when you define the parameter in the       response plan. For dynamic values, the system determines the correct parameter value by       collecting information from the incident. Incident Manager supports the       following dynamic parameters:
///
/// Incident ARN
///
/// When Incident Manager creates an incident, the system captures the Amazon       Resource Name (ARN) of the corresponding incident record and enters it for this       parameter in the runbook.
///
/// Involved resources
///
/// When Incident Manager creates an incident, the system captures the ARNs of       the resources involved in the incident. These resource ARNs are then assigned to this       parameter in the runbook.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynamicSsmParameter {
    ///
    /// The key parameter to use when running the Systems Manager Automation       runbook.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The dynamic parameter value.
    ///
    /// Required: Yes
    ///
    /// Type: DynamicSsmParameterValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: DynamicSsmParameterValue,
}

impl cfn_resources::CfnResource for DynamicSsmParameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.value.validate()?;

        Ok(())
    }
}

/// The dynamic parameter value.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynamicSsmParameterValue {
    ///
    /// Variable dynamic parameters. A parameter value is determined when an incident is       created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: INCIDENT_RECORD_ARN | INVOLVED_RESOURCES
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<DynamicSsmParameterValueVariableEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DynamicSsmParameterValueVariableEnum {
    /// INCIDENT_RECORD_ARN
    #[serde(rename = "INCIDENT_RECORD_ARN")]
    Incidentrecordarn,

    /// INVOLVED_RESOURCES
    #[serde(rename = "INVOLVED_RESOURCES")]
    Involvedresources,
}

impl Default for DynamicSsmParameterValueVariableEnum {
    fn default() -> Self {
        DynamicSsmParameterValueVariableEnum::Incidentrecordarn
    }
}

impl cfn_resources::CfnResource for DynamicSsmParameterValue {
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

/// The IncidentTemplate property type specifies details used to create an       incident when using this response plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IncidentTemplate {
    ///
    /// Used to create only one incident record for an incident.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "DedupeString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedupe_string: Option<String>,

    ///
    /// Defines the impact to the customers. Providing an impact overwrites the impact       provided by a response plan.
    ///
    /// Possible impacts:                                                1 - Critical impact, this typically relates to full application           failure that impacts many to all customers.                2 - High impact, partial application failure with impact to many           customers.               3 - Medium impact, the application is providing reduced service           to customers.               4 - Low impact, customer might aren't impacted by the problem           yet.               5 - No impact, customers aren't currently impacted but urgent           action is needed to avoid impact.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Impact")]
    pub impact: i64,

    ///
    /// Tags to assign to the template. When the StartIncident API action is       called, Incident Manager assigns the tags specified in the template to the       incident.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncidentTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_tags: Option<Vec<Tag>>,

    ///
    /// The SNS targets that AWS Chatbot uses to notify the chat channel of updates       to an incident. You can also make updates to the incident through the chat channel using       the SNS topics.
    ///
    /// Required: No
    ///
    /// Type: List of NotificationTargetItem
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_targets: Option<Vec<NotificationTargetItem>>,

    ///
    /// The summary describes what has happened during the incident.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 8000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    ///
    /// The title of the incident is a brief and easily recognizable.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: String,
}

impl cfn_resources::CfnResource for IncidentTemplate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.dedupe_string {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'dedupe_string'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.dedupe_string {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'dedupe_string'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.impact;

        if *the_val > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'impact'. {} is greater than 5",
                the_val
            ));
        }

        let the_val = &self.impact;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'impact'. {} is less than 1",
                the_val
            ));
        }

        if let Some(the_val) = &self.notification_targets {
            if the_val.len() > 10 as _ {
                return Err(format!(
                    "Max validation failed on field 'notification_targets'. {} is greater than 10",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.summary {
            if the_val.len() > 8000 as _ {
                return Err(format!(
                    "Max validation failed on field 'summary'. {} is greater than 8000",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.summary {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'summary'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.title;

        if the_val.len() > 200 as _ {
            return Err(format!(
                "Max validation failed on field 'title'. {} is greater than 200",
                the_val.len()
            ));
        }

        let the_val = &self.title;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'title'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Information about third-party services integrated into a response plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Integration {
    ///
    /// Information about the PagerDuty service where the response plan creates an       incident.
    ///
    /// Required: Yes
    ///
    /// Type: PagerDutyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PagerDutyConfiguration")]
    pub pager_duty_configuration: PagerDutyConfiguration,
}

impl cfn_resources::CfnResource for Integration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.pager_duty_configuration.validate()?;

        Ok(())
    }
}

/// The SNS topic that's used by AWS Chatbot to notify the incidents chat       channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NotificationTargetItem {
    ///
    /// The Amazon Resource Name (ARN) of the SNS topic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^arn:aws(-cn|-us-gov)?:[a-z0-9-]*:[a-z0-9-]*:([0-9]{12})?:.+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

impl cfn_resources::CfnResource for NotificationTargetItem {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.sns_topic_arn {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'sns_topic_arn'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.sns_topic_arn {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'sns_topic_arn'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Details about the PagerDuty configuration for a response plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PagerDutyConfiguration {
    ///
    /// The name of the PagerDuty configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Details about the PagerDuty service associated with the configuration.
    ///
    /// Required: Yes
    ///
    /// Type: PagerDutyIncidentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PagerDutyIncidentConfiguration")]
    pub pager_duty_incident_configuration: PagerDutyIncidentConfiguration,

    ///
    /// The ID of the AWS Secrets Manager secret that stores your PagerDuty key, either a       General Access REST API Key or User Token REST API Key, and other user       credentials.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

impl cfn_resources::CfnResource for PagerDutyConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.pager_duty_incident_configuration.validate()?;

        Ok(())
    }
}

/// Details about the PagerDuty service where the response plan creates an       incident.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PagerDutyIncidentConfiguration {
    ///
    /// The ID of the PagerDuty service that the response plan associates with an incident       when it launches.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceId")]
    pub service_id: String,
}

impl cfn_resources::CfnResource for PagerDutyIncidentConfiguration {
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

/// The SsmAutomation property type specifies details about the Systems       Manager automation document that will be used as a runbook during an incident.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SsmAutomation {
    ///
    /// The automation document's name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.:/]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentName")]
    pub document_name: String,

    ///
    /// The automation document's version to use when running.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,

    ///
    /// The key-value pairs to resolve dynamic parameter values when processing a Systems Manager Automation runbook.
    ///
    /// Required: No
    ///
    /// Type: List of DynamicSsmParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamicParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_parameters: Option<Vec<DynamicSsmParameter>>,

    ///
    /// The key-value pair parameters to use when running the automation document.
    ///
    /// Required: No
    ///
    /// Type: List of SsmParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<SsmParameter>>,

    ///
    /// The Amazon Resource Name (ARN) of the role that the automation document will assume       when running commands.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^arn:aws(-cn|-us-gov)?:iam::([0-9]{12})?:role/.+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

    ///
    /// The account that the automation document will be run in. This can be in either the       management account or an application account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: IMPACTED_ACCOUNT | RESPONSE_PLAN_OWNER_ACCOUNT
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account: Option<SsmAutomationTargetAccountEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SsmAutomationTargetAccountEnum {
    /// IMPACTED_ACCOUNT
    #[serde(rename = "IMPACTED_ACCOUNT")]
    Impactedaccount,

    /// RESPONSE_PLAN_OWNER_ACCOUNT
    #[serde(rename = "RESPONSE_PLAN_OWNER_ACCOUNT")]
    Responseplanowneraccount,
}

impl Default for SsmAutomationTargetAccountEnum {
    fn default() -> Self {
        SsmAutomationTargetAccountEnum::Impactedaccount
    }
}

impl cfn_resources::CfnResource for SsmAutomation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.document_version {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'document_version'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.document_version {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'document_version'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if the_val.len() > 1000 as _ {
            return Err(format!(
                "Max validation failed on field 'role_arn'. {} is greater than 1000",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'role_arn'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The key-value pair parameters to use when running the automation document.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SsmParameter {
    ///
    /// The key parameter to use when running the automation document.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value parameter to use when running the automation document.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

impl cfn_resources::CfnResource for SsmParameter {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
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
