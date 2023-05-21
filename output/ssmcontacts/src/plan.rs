/// Information about the stages and on-call rotation teams associated with an escalation     plan or engagement plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPlan {
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
    /// Update requires: Replacement
    #[serde(rename = "ContactId")]
    pub contact_id: String,

    ///
    /// The Amazon Resource Names (ARNs) of the on-call rotations associated with the plan.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotationIds")]
    pub rotation_ids: Option<Vec<String>>,

    ///
    /// A list of stages that the escalation plan or engagement plan uses to engage contacts and     contact methods.
    ///
    /// Required: No
    ///
    /// Type: List of Stage
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stages")]
    pub stages: Option<Vec<Stage>>,
}

impl cfn_resources::CfnResource for CfnPlan {
    fn type_string(&self) -> &'static str {
        "AWS::SSMContacts::Plan"
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

/// A set amount of time that an escalation plan or engagement plan engages the specified     contacts or contact methods.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Stage {
    ///
    /// The time to wait until beginning the next stage. The duration can only be set to 0 if a     target is specified.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: i64,

    ///
    /// The contacts or contact methods that the escalation plan or engagement plan is     engaging.
    ///
    /// Required: No
    ///
    /// Type: List of Targets
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
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
        let the_val = &self.duration_in_minutes;

        if *the_val > 30 as _ {
            return Err(format!(
                "Max validation failed on field 'duration_in_minutes'. {} is greater than 30",
                the_val
            ));
        }

        let the_val = &self.duration_in_minutes;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'duration_in_minutes'. {} is less than 0",
                the_val
            ));
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
    pub channel_target_info: Option<ChannelTargetInfo>,

    ///
    /// Information about the contact that Incident Manager engages.
    ///
    /// Required: No
    ///
    /// Type: ContactTargetInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactTargetInfo")]
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
