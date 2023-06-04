/// Creates a message template that you can use to send in-app messages. A message       template is a set of content and settings that you can define, save, and reuse in       messages for any of your Amazon Pinpoint applications.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnInAppTemplate {
    ///
    /// An object that contains information about the content of an in-app message,           including its title and body text, text colors, background colors, images,           buttons, and behaviors.
    ///
    /// Required: No
    ///
    /// Type: List of InAppMessageContent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub content: Option<Vec<InAppMessageContent>>,

    ///
    /// Custom data, in the form of key-value pairs, that is included in an in-app messaging       payload.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub custom_config: Option<serde_json::Value>,

    ///
    /// A string that determines the appearance of the in-app message. You can specify           one of the following:
    ///
    /// BOTTOM_BANNER – a message that appears as a banner at the               bottom of the page.                                                        TOP_BANNER – a message that appears as a banner at the               top of the page.                                                        OVERLAYS – a message that covers entire screen.                                                        MOBILE_FEED – a message that appears in a window in front               of the page.                                                        MIDDLE_BANNER – a message that appears as a banner in the               middle of the page.                                                        CAROUSEL – a scrollable layout of up to five unique               messages.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Layout")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub layout: Option<cfn_resources::StrVal>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<serde_json::Value>,

    ///
    /// An optional description of the in-app template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub template_description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the in-app message template.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TemplateName")]
    pub template_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnInAppTemplatearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInAppTemplatearn;
impl CfnInAppTemplatearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnInAppTemplate {
    fn type_string(&self) -> &'static str {
        "AWS::Pinpoint::InAppTemplate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the configuration of the main body text of the in-app message.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BodyConfig {
    ///
    /// The text alignment of the main body text of the message. Acceptable values:         LEFT, CENTER, RIGHT.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alignment")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub alignment: Option<cfn_resources::StrVal>,

    ///
    /// The main body text of the message.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub body: Option<cfn_resources::StrVal>,

    ///
    /// The color of the body text, expressed as a hex color code (such as #000000 for       black).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub text_color: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for BodyConfig {
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

/// Specifies the behavior of buttons that appear in an in-app message template.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ButtonConfig {
    ///
    /// Optional button configuration to use for in-app messages sent to Android devices. This       button configuration overrides the default button configuration.
    ///
    /// Required: No
    ///
    /// Type: OverrideButtonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Android")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub android: Option<OverrideButtonConfiguration>,

    ///
    /// Specifies the default behavior of a button that appears in an in-app message. You can       optionally add button configurations that specifically apply to iOS, Android, or web       browser users.
    ///
    /// Required: No
    ///
    /// Type: DefaultButtonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub default_config: Option<DefaultButtonConfiguration>,

    ///
    /// Optional button configuration to use for in-app messages sent to iOS devices. This       button configuration overrides the default button configuration.
    ///
    /// Required: No
    ///
    /// Type: OverrideButtonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IOS")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub ios: Option<OverrideButtonConfiguration>,

    ///
    /// Optional button configuration to use for in-app messages sent to web applications.       This button configuration overrides the default button configuration.
    ///
    /// Required: No
    ///
    /// Type: OverrideButtonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Web")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub web: Option<OverrideButtonConfiguration>,
}

impl cfn_resources::CfnResource for ButtonConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.android.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.default_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ios.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.web.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the default behavior of a button that appears in an in-app message. You can       optionally add button configurations that specifically apply to iOS, Android, or web       browser users.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DefaultButtonConfiguration {
    ///
    /// The background color of a button, expressed as a hex color code (such as #000000 for       black).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub background_color: Option<cfn_resources::StrVal>,

    ///
    /// The border radius of a button.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorderRadius")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub border_radius: Option<i64>,

    ///
    /// The action that occurs when a recipient chooses a button in an in-app message.           You can specify one of the following:
    ///
    /// LINK – A link to a web destination.                                      DEEP_LINK – A link to a specific page in an               application.                                      CLOSE – Dismisses the message.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ButtonAction")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub button_action: Option<cfn_resources::StrVal>,

    ///
    /// The destination (such as a URL) for a button.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Link")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub link: Option<cfn_resources::StrVal>,

    ///
    /// The text that appears on a button in an in-app message.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub text: Option<cfn_resources::StrVal>,

    ///
    /// The color of the body text in a button, expressed as a hex color code (such as #000000       for black).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub text_color: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DefaultButtonConfiguration {
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

/// Specifies the configuration and content of the header or title text of the in-app       message.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HeaderConfig {
    ///
    /// The text alignment of the title of the message. Acceptable values: LEFT,         CENTER, RIGHT.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alignment")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub alignment: Option<cfn_resources::StrVal>,

    ///
    /// The title text of the in-app message.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Header")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub header: Option<cfn_resources::StrVal>,

    ///
    /// The color of the title text, expressed as a hex color code (such as #000000 for       black).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub text_color: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HeaderConfig {
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

/// Specifies the configuration of an in-app message, including its header, body, buttons,       colors, and images.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageContent {
    ///
    /// The background color for an in-app message banner, expressed as a hex color code (such       as #000000 for black).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub background_color: Option<cfn_resources::StrVal>,

    ///
    /// An object that contains configuration information about the header or title           text of the in-app message.
    ///
    /// Required: No
    ///
    /// Type: BodyConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "BodyConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub body_config: Option<BodyConfig>,

    ///
    /// An object that contains configuration information about the header or title           text of the in-app message.
    ///
    /// Required: No
    ///
    /// Type: HeaderConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub header_config: Option<HeaderConfig>,

    ///
    /// The URL of the image that appears on an in-app message banner.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub image_url: Option<cfn_resources::StrVal>,

    ///
    /// An object that contains configuration information about the primary button in           an in-app message.
    ///
    /// Required: No
    ///
    /// Type: ButtonConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryBtn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub primary_btn: Option<ButtonConfig>,

    ///
    /// An object that contains configuration information about the secondary button           in an in-app message.
    ///
    /// Required: No
    ///
    /// Type: ButtonConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryBtn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub secondary_btn: Option<ButtonConfig>,
}

impl cfn_resources::CfnResource for InAppMessageContent {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.body_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.header_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.primary_btn
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.secondary_btn
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the configuration of a button with settings that are specific to a certain       device type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverrideButtonConfiguration {
    ///
    /// The action that occurs when a recipient chooses a button in an in-app message.           You can specify one of the following:
    ///
    /// LINK – A link to a web destination.                                      DEEP_LINK – A link to a specific page in an               application.                                      CLOSE – Dismisses the message.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ButtonAction")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub button_action: Option<cfn_resources::StrVal>,

    ///
    /// The destination (such as a URL) for a button.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Link")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub link: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for OverrideButtonConfiguration {
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
