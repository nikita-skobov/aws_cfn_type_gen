
pub mod cfn_in_app_template {

#[derive(serde::Serialize, Default)]
pub struct CfnInAppTemplate {
    /// List of InAppMessageContent
    #[serde(rename = "Content")]
    pub content: Option<Vec<InAppMessageContent>>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "TemplateDescription")]
    pub template_description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "CustomConfig")]
    pub custom_config: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Layout")]
    pub layout: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct BodyConfig {
    #[serde(rename = "TextColor")]
    pub text_color: Option<String>,
    #[serde(rename = "Alignment")]
    pub alignment: Option<Alignment>,
    #[serde(rename = "Body")]
    pub body: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OverrideButtonConfiguration {
    #[serde(rename = "Link")]
    pub link: Option<String>,
    #[serde(rename = "ButtonAction")]
    pub button_action: Option<ButtonAction>,

}
pub type ButtonAction = String;
#[derive(serde::Serialize, Default)]
pub struct HeaderConfig {
    #[serde(rename = "Header")]
    pub header: Option<String>,
    #[serde(rename = "TextColor")]
    pub text_color: Option<String>,
    #[serde(rename = "Alignment")]
    pub alignment: Option<Alignment>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultButtonConfiguration {
    #[serde(rename = "BorderRadius")]
    pub border_radius: Option<usize>,
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "TextColor")]
    pub text_color: Option<String>,
    #[serde(rename = "ButtonAction")]
    pub button_action: Option<ButtonAction>,
    #[serde(rename = "Link")]
    pub link: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InAppMessageContent {
    #[serde(rename = "ImageUrl")]
    pub image_url: Option<String>,
    #[serde(rename = "BodyConfig")]
    pub body_config: Option<BodyConfig>,
    #[serde(rename = "PrimaryBtn")]
    pub primary_btn: Option<ButtonConfig>,
    #[serde(rename = "SecondaryBtn")]
    pub secondary_btn: Option<ButtonConfig>,
    #[serde(rename = "HeaderConfig")]
    pub header_config: Option<HeaderConfig>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,

}
pub type Alignment = String;
#[derive(serde::Serialize, Default)]
pub struct ButtonConfig {
    #[serde(rename = "Web")]
    pub web: Option<OverrideButtonConfiguration>,
    #[serde(rename = "Android")]
    pub android: Option<OverrideButtonConfiguration>,
    #[serde(rename = "DefaultConfig")]
    pub default_config: Option<DefaultButtonConfiguration>,
    #[serde(rename = "IOS")]
    pub ios: Option<OverrideButtonConfiguration>,

}


}
