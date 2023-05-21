

/// Specifies an email template. Email templates enable you to send personalized email to       one or more destinations in a single API operation.
#[derive(Default, serde::Serialize)]
pub struct CfnTemplate {


    /// 
    /// The content of the email, composed of a subject line and either an HTML part or a       text-only part.
    /// 
    /// Required: No
    ///
    /// Type: Template
    ///
    /// Update requires: No interruption
    #[serde(rename = "Template")]
    pub template: Option<Box<Template>>,

}


/// The content of the email, composed of a subject line and either an HTML part or a       text-only part.
#[derive(Default, serde::Serialize)]
pub struct Template {


    /// 
    /// The name of the template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,


    /// 
    /// The HTML body of the email.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HtmlPart")]
    pub html_part: Option<String>,


    /// 
    /// The subject line of the email.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectPart")]
    pub subject_part: String,


    /// 
    /// The email body that is visible to recipients whose email clients do not display HTML       content.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextPart")]
    pub text_part: Option<String>,

}