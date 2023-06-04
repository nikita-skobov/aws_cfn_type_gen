/// You can use the AWS::SupportApp::AccountAlias resource to specify your         AWS account when you configure the AWS Support App in       Slack. Your alias name appears on the AWS Support App page in the Support Center Console and in messages from the AWS Support App. You       can use this alias to identify the account you've configured with the AWS Support App.
///
/// For more information, see AWS Support App in Slack in the AWS Support User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccountAlias {
    ///
    /// An alias or short name for an AWS account.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountAlias")]
    pub account_alias: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_account_alias_resource_id: CfnAccountAliasaccountaliasresourceid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccountAliasaccountaliasresourceid;
impl CfnAccountAliasaccountaliasresourceid {
    pub fn att_name(&self) -> &'static str {
        r#"AccountAliasResourceId"#
    }
}

impl cfn_resources::CfnResource for CfnAccountAlias {
    fn type_string(&self) -> &'static str {
        "AWS::SupportApp::AccountAlias"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
