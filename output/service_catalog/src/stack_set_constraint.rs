

/// Specifies a StackSet constraint.
#[derive(Default, serde::Serialize)]
pub struct CfnStackSetConstraint {


    /// 
    /// AdminRole ARN
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdminRole")]
    pub admin_role: String,


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
    /// ExecutionRole name
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRole")]
    pub execution_role: String,


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
    /// Permission to create, update, and delete stack instances. Choose from ALLOWED and NOT_ALLOWED.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackInstanceControl")]
    pub stack_instance_control: String,


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
    pub description: String,


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
