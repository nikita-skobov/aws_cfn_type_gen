

/// Specifies a notification constraint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLaunchNotificationConstraint {


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
    /// The notification ARNs.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationArns")]
    pub notification_arns: Vec<String>,


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

}



impl cfn_resources::CfnResource for CfnLaunchNotificationConstraint {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::LaunchNotificationConstraint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.accept_language {

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'accept_language'. {} is greater than 100", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() > 2000 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 2000", the_val.len()));
        }

        }
        
        let the_val = &self.portfolio_id;

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'portfolio_id'. {} is greater than 100", the_val.len()));
        }

        
        let the_val = &self.portfolio_id;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'portfolio_id'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.product_id;

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'product_id'. {} is greater than 100", the_val.len()));
        }

        
        let the_val = &self.product_id;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'product_id'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}