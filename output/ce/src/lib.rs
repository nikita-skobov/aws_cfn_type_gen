
pub mod cfn_cost_category {

#[derive(serde::Serialize, Default)]
pub struct CfnCostCategory {
    /// No documentation provided by AWS
    #[serde(rename = "RuleVersion")]
    pub rule_version: String,
    /// JSON array format of Expression in Billing and Cost Management API
    #[serde(rename = "Rules")]
    pub rules: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// Json array format of CostCategorySplitChargeRule in Billing and Cost Management API
    #[serde(rename = "SplitChargeRules")]
    pub split_charge_rules: Option<String>,
    /// The default value for the cost category
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,

}

pub type ZonedDateTime = String;

}
