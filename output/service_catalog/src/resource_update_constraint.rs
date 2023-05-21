

/// Specifies a RESOURCE_UPDATE constraint.
#[derive(Default, serde::Serialize)]
pub struct CfnResourceUpdateConstraint {


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
    /// If set to ALLOWED, lets users change tags in a CloudFormationProvisionedProduct resource.
    /// 
    /// If set to NOT_ALLOWED, prevents users from changing tags in a CloudFormationProvisionedProduct resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagUpdateOnProvisionedProduct")]
    pub tag_update_on_provisioned_product: String,


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

}
