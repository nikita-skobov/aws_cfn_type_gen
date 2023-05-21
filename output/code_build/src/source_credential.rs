

/// Information about the credentials for a GitHub, GitHub Enterprise, or Bitbucket repository. We strongly recommend that you      use AWS Secrets Manager to store your credentials. If you use      Secrets Manager, you must have secrets in your secrets manager. For more       information, see         Using Dynamic References to Specify Template Values.
#[derive(Default, serde::Serialize)]
pub struct CfnSourceCredential {


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
    pub server_type: String,


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
    pub username: Option<String>,


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
    pub auth_type: String,

}
