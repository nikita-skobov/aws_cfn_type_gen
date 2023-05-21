

/// Accepts an offer to share the specified portfolio.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAcceptedPortfolioShare {


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



impl cfn_resources::CfnResource for CfnAcceptedPortfolioShare {
    fn type_string() -> &'static str {
        "AWS::ServiceCatalog::AcceptedPortfolioShare"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
