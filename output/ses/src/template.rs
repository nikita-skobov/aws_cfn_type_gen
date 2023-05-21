/// Specifies an email template. Email templates enable you to send personalized email to       one or more destinations in a single API operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<Template>>,
}

impl cfn_resources::CfnResource for CfnTemplate {
    fn type_string(&self) -> &'static str {
        "AWS::SES::Template"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.template
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The content of the email, composed of a subject line and either an HTML part or a       text-only part.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Template {
    ///
    /// The HTML body of the email.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HtmlPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<cfn_resources::StrVal>,

    ///
    /// The subject line of the email.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectPart")]
    pub subject_part: cfn_resources::StrVal,

    ///
    /// The name of the template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<cfn_resources::StrVal>,

    ///
    /// The email body that is visible to recipients whose email clients do not display HTML       content.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Template {
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
