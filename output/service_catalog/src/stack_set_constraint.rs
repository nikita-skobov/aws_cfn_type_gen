/// Specifies a StackSet constraint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnStackSetConstraint {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<cfn_resources::StrVal>,

    ///
    /// One or more AWS accounts that will have access to the provisioned product.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountList")]
    pub account_list: Vec<String>,

    ///
    /// AdminRole ARN
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminRole")]
    pub admin_role: cfn_resources::StrVal,

    ///
    /// The description of the constraint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: cfn_resources::StrVal,

    ///
    /// ExecutionRole name
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRole")]
    pub execution_role: cfn_resources::StrVal,

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
    /// One or more AWS Regions where the provisioned product will be available.
    ///
    /// Applicable only to a CFN_STACKSET provisioned product type.
    ///
    /// The specified Regions should be within the list of Regions from the STACKSET constraint. To get the list of Regions in the STACKSET constraint, use the DescribeProvisioningParameters operation.
    ///
    /// If no values are specified, the default value is all Regions from the STACKSET constraint.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionList")]
    pub region_list: Vec<String>,

    ///
    /// Permission to create, update, and delete stack instances. Choose from ALLOWED and NOT_ALLOWED.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackInstanceControl")]
    pub stack_instance_control: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnStackSetConstraint {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::StackSetConstraint"
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

        let the_val = &self.description;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2000 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 2000",
                    s.len()
                ));
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

        Ok(())
    }
}
