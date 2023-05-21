

/// Associates the specified principal ARN with the specified portfolio.
#[derive(Default, serde::Serialize)]
pub struct CfnPortfolioPrincipalAssociation {


    /// 
    /// The language code.
    /// 
    /// jp - Japanese                        zh - Chinese
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,


    /// 
    /// The principal type. The supported value is IAM.
    /// 
    /// Allowed Values: IAM
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrincipalType")]
    pub principal_type: String,


    /// 
    /// The portfolio identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,


    /// 
    /// The ARN of the principal (IAM user, role, or group).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrincipalARN")]
    pub principal_arn: String,

}
