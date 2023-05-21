/// The AWS::SSMContacts::Contact resource specifies a contact or escalation       plan. Incident Manager contacts are a subset of actions and data types that       you can use for managing responder engagement and interaction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnContact {
    ///
    /// The unique and identifiable alias of the contact or escalation plan.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-z0-9_\-]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Alias")]
    pub alias: String,

    ///
    /// The full name of the contact or escalation plan.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[\p{L}\p{Z}\p{N}_.\-]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    pub display_name: String,

    ///
    /// A list of stages. A contact has an engagement plan with stages that contact specified     contact channels. An escalation plan uses stages that contact specified contacts.
    ///
    /// Required: No
    ///
    /// Type: List of Stage
    ///
    /// Update requires: No interruption
    #[serde(rename = "Plan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Vec<Stage>>,

    ///
    /// Refers to the type of contact:
    ///
    /// PERSONAL: A single, individual contact.                        ESCALATION: An escalation plan.                        ONCALL_SCHEDULE: An on-call schedule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ESCALATION | ONCALL_SCHEDULE | PERSONAL
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: ContactTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ContactTypeEnum {
    /// ESCALATION
    #[serde(rename = "ESCALATION")]
    Escalation,

    /// ONCALL_SCHEDULE
    #[serde(rename = "ONCALL_SCHEDULE")]
    Oncallschedule,

    /// PERSONAL
    #[serde(rename = "PERSONAL")]
    Personal,
}

impl Default for ContactTypeEnum {
    fn default() -> Self {
        ContactTypeEnum::Escalation
    }
}

impl cfn_resources::CfnResource for CfnContact {
    fn type_string(&self) -> &'static str {
        "AWS::SSMContacts::Contact"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.alias;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'alias'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.alias;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'alias'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.display_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'display_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.display_name;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'display_name'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Information about the contact channel that Incident Manager uses to engage the     contact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ChannelTargetInfo {
    ///
    /// The Amazon Resource Name (ARN) of the contact channel.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:(aws|aws-cn|aws-us-gov):ssm-contacts:[-\w+=\/,.@]*:[0-9]+:([\w+=\/,.@:-]+)*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelId")]
    pub channel_id: String,

    ///
    /// The number of minutes to wait before retrying to send engagement if the engagement     initially failed.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 60
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryIntervalInMinutes")]
    pub retry_interval_in_minutes: i64,
}

impl cfn_resources::CfnResource for ChannelTargetInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.channel_id;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'channel_id'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.channel_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'channel_id'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.retry_interval_in_minutes;

        if *the_val > 60 as _ {
            return Err(format!(
                "Max validation failed on field 'retry_interval_in_minutes'. {} is greater than 60",
                the_val
            ));
        }

        let the_val = &self.retry_interval_in_minutes;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'retry_interval_in_minutes'. {} is less than 0",
                the_val
            ));
        }

        Ok(())
    }
}

/// The contact that Incident Manager is engaging during an incident.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContactTargetInfo {
    ///
    /// The Amazon Resource Name (ARN) of the contact.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:(aws|aws-cn|aws-us-gov):ssm-contacts:[-\w+=\/,.@]*:[0-9]+:([\w+=\/,.@:-]+)*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactId")]
    pub contact_id: String,

    ///
    /// A Boolean value determining if the contact's acknowledgement stops the progress of     stages in the plan.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsEssential")]
    pub is_essential: bool,
}

impl cfn_resources::CfnResource for ContactTargetInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.contact_id;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'contact_id'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.contact_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'contact_id'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The Stage property type specifies a set amount of time that an escalation       plan or engagement plan engages the specified contacts or contact methods.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Stage {
    ///
    /// The time to wait until beginning the next stage. The duration can only be set to 0 if a     target is specified.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_minutes: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_ids: Option<Vec<String>>,

    ///
    /// The contacts or contact methods that the escalation plan or engagement plan is     engaging.
    ///
    /// Required: No
    ///
    /// Type: List of Targets
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Targets>>,
}

impl cfn_resources::CfnResource for Stage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.duration_in_minutes {
            if *the_val > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'duration_in_minutes'. {} is greater than 30",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.duration_in_minutes {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'duration_in_minutes'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// The contact or contact channel that's being engaged.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Targets {
    ///
    /// Information about the contact channel that Incident Manager engages.
    ///
    /// Required: No
    ///
    /// Type: ChannelTargetInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelTargetInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_target_info: Option<ChannelTargetInfo>,

    ///
    /// The contact that Incident Manager is engaging during an incident.
    ///
    /// Required: No
    ///
    /// Type: ContactTargetInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactTargetInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_target_info: Option<ContactTargetInfo>,
}

impl cfn_resources::CfnResource for Targets {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.channel_target_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.contact_target_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
