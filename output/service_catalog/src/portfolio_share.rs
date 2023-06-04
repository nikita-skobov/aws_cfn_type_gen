/// Shares the specified portfolio with the specified account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPortfolioShare {
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
    pub account_id: cfn_resources::StrVal,

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
    /// Indicates whether TagOptions sharing is enabled or disabled for the portfolio share.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShareTagOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_tag_options: Option<bool>,
}

impl cfn_resources::CfnResource for CfnPortfolioShare {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::PortfolioShare"
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

        Ok(())
    }
}
