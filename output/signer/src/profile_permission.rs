/// Adds cross-account permissions to a signing profile.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProfilePermission {
    ///
    /// The AWS Signer action permitted as part of cross-account             permissions.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Action")]
    pub action: cfn_resources::StrVal,

    ///
    /// The AWS principal receiving cross-account permissions. This             may be an IAM role or another AWS account ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: cfn_resources::StrVal,

    ///
    /// The human-readable name of the signing profile.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProfileName")]
    pub profile_name: cfn_resources::StrVal,

    ///
    /// The version of the signing profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProfileVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<cfn_resources::StrVal>,

    ///
    /// A unique identifier for the cross-account permission statement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StatementId")]
    pub statement_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnProfilePermission {
    fn type_string(&self) -> &'static str {
        "AWS::Signer::ProfilePermission"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
