/// Associates the specified principal ARN with the specified portfolio.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub principal_arn: cfn_resources::StrVal,

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
    pub principal_type: PortfolioPrincipalAssociationPrincipalTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PortfolioPrincipalAssociationPrincipalTypeEnum {
    /// IAM
    #[serde(rename = "IAM")]
    Iam,
}

impl Default for PortfolioPrincipalAssociationPrincipalTypeEnum {
    fn default() -> Self {
        PortfolioPrincipalAssociationPrincipalTypeEnum::Iam
    }
}

impl cfn_resources::CfnResource for CfnPortfolioPrincipalAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::PortfolioPrincipalAssociation"
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

        let the_val = &self.principal_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'principal_arn'. {} is greater than 1000",
                    s.len()
                ));
            }
        }

        let the_val = &self.principal_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'principal_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
