/// Information about the credentials for a GitHub, GitHub Enterprise, or Bitbucket repository. We strongly recommend that you      use AWS Secrets Manager to store your credentials. If you use      Secrets Manager, you must have secrets in your secrets manager. For more       information, see         Using Dynamic References to Specify Template Values.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSourceCredential {
    ///
    /// The type of authentication used by the credentials. Valid options are OAUTH,       BASIC_AUTH, or PERSONAL_ACCESS_TOKEN.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BASIC_AUTH | OAUTH | PERSONAL_ACCESS_TOKEN
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthType")]
    pub auth_type: SourceCredentialAuthTypeEnum,

    ///
    /// The type of source provider. The valid options are GITHUB, GITHUB_ENTERPRISE, or       BITBUCKET.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BITBUCKET | GITHUB | GITHUB_ENTERPRISE
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerType")]
    pub server_type: SourceCredentialServerTypeEnum,

    ///
    /// For GitHub or GitHub Enterprise, this is the personal access token. For Bitbucket,       this is the app password.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Token")]
    pub token: String,

    ///
    /// The Bitbucket username when the authType is BASIC_AUTH. This parameter       is not valid for other types of source providers or connections.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SourceCredentialAuthTypeEnum {
    /// BASIC_AUTH
    #[serde(rename = "BASIC_AUTH")]
    Basicauth,

    /// OAUTH
    #[serde(rename = "OAUTH")]
    Oauth,

    /// PERSONAL_ACCESS_TOKEN
    #[serde(rename = "PERSONAL_ACCESS_TOKEN")]
    Personalaccesstoken,
}

impl Default for SourceCredentialAuthTypeEnum {
    fn default() -> Self {
        SourceCredentialAuthTypeEnum::Basicauth
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SourceCredentialServerTypeEnum {
    /// BITBUCKET
    #[serde(rename = "BITBUCKET")]
    Bitbucket,

    /// GITHUB
    #[serde(rename = "GITHUB")]
    Github,

    /// GITHUB_ENTERPRISE
    #[serde(rename = "GITHUB_ENTERPRISE")]
    Githubenterprise,
}

impl Default for SourceCredentialServerTypeEnum {
    fn default() -> Self {
        SourceCredentialServerTypeEnum::Bitbucket
    }
}

impl cfn_resources::CfnResource for CfnSourceCredential {
    fn type_string(&self) -> &'static str {
        "AWS::CodeBuild::SourceCredential"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.token;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'token'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.username {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'username'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
