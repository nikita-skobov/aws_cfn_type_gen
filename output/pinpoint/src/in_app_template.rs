

/// Creates a message template that you can use to send in-app messages. A message       template is a set of content and settings that you can define, save, and reuse in       messages for any of your Amazon Pinpoint applications.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnInAppTemplate {


    /// 
    /// The name of the in-app message template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TemplateName")]
    pub template_name: String,


    /// 
    /// An optional description of the in-app template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateDescription")]
    pub template_description: Option<String>,


    /// 
    /// An object that contains information about the content of an in-app message,           including its title and body text, text colors, background colors, images,           buttons, and behaviors.
    /// 
    /// Required: No
    ///
    /// Type: List of InAppMessageContent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: Option<Vec<InAppMessageContent>>,


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
    pub tags: Option<serde_json::Value>,


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
    pub layout: Option<String>,


    /// 
    /// Custom data, in the form of key-value pairs, that is included in an in-app messaging       payload.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConfig")]
    pub custom_config: Option<serde_json::Value>,

}



impl cfn_resources::CfnResource for CfnInAppTemplate {
    fn type_string() -> &'static str {
        "AWS::Pinpoint::InAppTemplate"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies the configuration and content of the header or title text of the in-app       message.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HeaderConfig {


    /// 
    /// The color of the title text, expressed as a hex color code (such as #000000 for       black).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    pub text_color: Option<String>,


    /// 
    /// The text alignment of the title of the message. Acceptable values: LEFT,         CENTER, RIGHT.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alignment")]
    pub alignment: Option<String>,


    /// 
    /// The title text of the in-app message.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Header")]
    pub header: Option<String>,

}




/// Specifies the behavior of buttons that appear in an in-app message template.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ButtonConfig {


    /// 
    /// Optional button configuration to use for in-app messages sent to iOS devices. This       button configuration overrides the default button configuration.
    /// 
    /// Required: No
    ///
    /// Type: OverrideButtonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IOS")]
    pub ios: Option<OverrideButtonConfiguration>,


    /// 
    /// Specifies the default behavior of a button that appears in an in-app message. You can       optionally add button configurations that specifically apply to iOS, Android, or web       browser users.
    /// 
    /// Required: No
    ///
    /// Type: DefaultButtonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultConfig")]
    pub default_config: Option<DefaultButtonConfiguration>,


    /// 
    /// Optional button configuration to use for in-app messages sent to Android devices. This       button configuration overrides the default button configuration.
    /// 
    /// Required: No
    ///
    /// Type: OverrideButtonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Android")]
    pub android: Option<OverrideButtonConfiguration>,


    /// 
    /// Optional button configuration to use for in-app messages sent to web applications.       This button configuration overrides the default button configuration.
    /// 
    /// Required: No
    ///
    /// Type: OverrideButtonConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Web")]
    pub web: Option<OverrideButtonConfiguration>,

}




/// Specifies the default behavior of a button that appears in an in-app message. You can       optionally add button configurations that specifically apply to iOS, Android, or web       browser users.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DefaultButtonConfiguration {


    /// 
    /// The text that appears on a button in an in-app message.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Text")]
    pub text: Option<String>,


    /// 
    /// The destination (such as a URL) for a button.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Link")]
    pub link: Option<String>,


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
    pub button_action: Option<String>,


    /// 
    /// The border radius of a button.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorderRadius")]
    pub border_radius: Option<i64>,


    /// 
    /// The color of the body text in a button, expressed as a hex color code (such as #000000       for black).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    pub text_color: Option<String>,


    /// 
    /// The background color of a button, expressed as a hex color code (such as #000000 for       black).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,

}




/// Specifies the configuration of the main body text of the in-app message.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BodyConfig {


    /// 
    /// The color of the body text, expressed as a hex color code (such as #000000 for       black).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextColor")]
    pub text_color: Option<String>,


    /// 
    /// The text alignment of the main body text of the message. Acceptable values:         LEFT, CENTER, RIGHT.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alignment")]
    pub alignment: Option<String>,


    /// 
    /// The main body text of the message.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Body")]
    pub body: Option<String>,

}




/// Specifies the configuration of a button with settings that are specific to a certain       device type.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub button_action: Option<String>,


    /// 
    /// The destination (such as a URL) for a button.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Link")]
    pub link: Option<String>,

}




/// Specifies the configuration of an in-app message, including its header, body, buttons,       colors, and images.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InAppMessageContent {


    /// 
    /// An object that contains configuration information about the header or title           text of the in-app message.
    /// 
    /// Required: No
    ///
    /// Type: HeaderConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderConfig")]
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
    pub image_url: Option<String>,


    /// 
    /// An object that contains configuration information about the primary button in           an in-app message.
    /// 
    /// Required: No
    ///
    /// Type: ButtonConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryBtn")]
    pub primary_btn: Option<ButtonConfig>,


    /// 
    /// The background color for an in-app message banner, expressed as a hex color code (such       as #000000 for black).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,


    /// 
    /// An object that contains configuration information about the header or title           text of the in-app message.
    /// 
    /// Required: No
    ///
    /// Type: BodyConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "BodyConfig")]
    pub body_config: Option<BodyConfig>,


    /// 
    /// An object that contains configuration information about the secondary button           in an in-app message.
    /// 
    /// Required: No
    ///
    /// Type: ButtonConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryBtn")]
    pub secondary_btn: Option<ButtonConfig>,

}


