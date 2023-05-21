

/// Specifies a notification constraint.
#[derive(Default, serde::Serialize)]
pub struct CfnLaunchNotificationConstraint {


    /// 
    /// The description of the constraint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


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
    /// Update requires: No interruption
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


    /// 
    /// The product identifier.
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
    #[serde(rename = "ProductId")]
    pub product_id: String,


    /// 
    /// The notification ARNs.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationArns")]
    pub notification_arns: Vec<String>,

}
