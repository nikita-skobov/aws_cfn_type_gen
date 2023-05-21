/// The AWS::SSMContacts::ContactChannel resource specifies a contact channel       as the method that Incident Manager uses to engage your contact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnContactChannel {
    ///
    /// The details that Incident Manager uses when trying to engage the contact channel.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelAddress")]
    pub channel_address: String,

    ///
    /// The name of the contact channel.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[\p{L}\p{Z}\p{N}_.\-]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelName")]
    pub channel_name: String,

    ///
    /// The type of the contact channel. Incident Manager supports three contact methods:
    ///
    /// SMS               VOICE               EMAIL
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EMAIL | SMS | VOICE
    ///
    /// Update requires: Replacement
    #[serde(rename = "ChannelType")]
    pub channel_type: ContactChannelChannelTypeEnum,

    ///
    /// The Amazon Resource Name (ARN) of the contact you are adding the contact channel     to.
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
    /// If you want to activate the channel at a later time, you can choose to defer activation.     Incident Manager can't engage your contact channel until it has been activated.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeferActivation")]
    pub defer_activation: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ContactChannelChannelTypeEnum {
    /// EMAIL
    #[serde(rename = "EMAIL")]
    Email,

    /// SMS
    #[serde(rename = "SMS")]
    Sms,

    /// VOICE
    #[serde(rename = "VOICE")]
    Voice,
}

impl Default for ContactChannelChannelTypeEnum {
    fn default() -> Self {
        ContactChannelChannelTypeEnum::Email
    }
}

impl cfn_resources::CfnResource for CfnContactChannel {
    fn type_string(&self) -> &'static str {
        "AWS::SSMContacts::ContactChannel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.channel_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'channel_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.channel_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'channel_name'. {} is less than 1",
                the_val.len()
            ));
        }

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
