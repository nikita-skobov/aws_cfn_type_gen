/// Associates the specified product with the specified portfolio.
///
/// A delegated admin is authorized to invoke this command.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPortfolioProductAssociation {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<cfn_resources::StrVal>,

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
    pub portfolio_id: cfn_resources::StrVal,

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
    pub product_id: cfn_resources::StrVal,

    ///
    /// The identifier of the source portfolio.
    ///
    /// Required: No
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
    #[serde(rename = "SourcePortfolioId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_portfolio_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnPortfolioProductAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::PortfolioProductAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.accept_language {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'accept_language'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.portfolio_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'portfolio_id'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.portfolio_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'portfolio_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.product_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'product_id'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.product_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'product_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.source_portfolio_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!("Max validation failed on field 'source_portfolio_id'. {} is greater than 100", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.source_portfolio_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'source_portfolio_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
