

/// Specifies a launch constraint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLaunchRoleConstraint {


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
    /// You are required to specify either the RoleArn or the LocalRoleName but can't use both.
    /// 
    /// If you specify the LocalRoleName property, when an account uses the launch constraint, the IAM role with that name in the account will be used. This allows launch-role constraints to be account-agnostic so the administrator can create fewer resources per shared account.
    /// 
    /// The given role name must exist in the account used to create the launch constraint and the account of the user who launches a product with this launch constraint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalRoleName")]
    pub local_role_name: Option<String>,


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
    /// The ARN of the launch role.
    /// 
    /// You are required to specify RoleArn or LocalRoleName but can't use both.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,

}



impl cfn_resources::CfnResource for CfnLaunchRoleConstraint {
    fn type_string() -> &'static str {
        "AWS::ServiceCatalog::LaunchRoleConstraint"
    }

    fn properties(self) -> serde_json::Value {
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