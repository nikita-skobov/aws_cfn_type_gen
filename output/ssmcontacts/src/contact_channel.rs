/// The AWS::SSMContacts::ContactChannel resource specifies a contact channel       as the method that Incident Manager uses to engage your contact.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub channel_address: cfn_resources::StrVal,

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
    pub channel_name: cfn_resources::StrVal,

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
    pub contact_id: cfn_resources::StrVal,

    ///
    /// If you want to activate the channel at a later time, you can choose to defer activation.     Incident Manager can't engage your contact channel until it has been activated.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeferActivation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_activation: Option<bool>,

    #[serde(skip_serializing)]
    pub att_arn: CfnContactChannelarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnContactChannelarn;
impl CfnContactChannelarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
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

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'channel_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.channel_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'channel_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.contact_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'contact_id'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.contact_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'contact_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
