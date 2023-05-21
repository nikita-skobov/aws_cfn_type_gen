

/// Shares the specified portfolio with the specified account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPortfolioShare {


    /// 
    /// The AWS account ID. For example, 123456789012.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccountId")]
    pub account_id: String,


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
    /// Indicates whether TagOptions sharing is enabled or disabled for the portfolio share.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShareTagOptions")]
    pub share_tag_options: Option<bool>,


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

}

impl cfn_resources::CfnResource for CfnPortfolioShare {
    fn type_string() -> &'static str {
        "AWS::ServiceCatalog::PortfolioShare"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
