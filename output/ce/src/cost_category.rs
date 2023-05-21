

/// The AWS::CE::CostCategory resource creates groupings of cost that you can use    across products in the AWS Billing and Cost Management console, such as Cost Explorer and AWS Budgets. For more information, see Managing Your Costs with     Cost Categories in the AWS Billing and Cost Management User    Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCostCategory {


    /// 
    /// The array of CostCategoryRule in JSON array format.
    /// 
    /// NoteRules are processed in order. If there are multiple rules that match the line item, then     the first rule to match is used to determine that Cost Category value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: String,


    /// 
    /// The unique name of the Cost Category.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The rule schema version in this particular Cost Category.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleVersion")]
    pub rule_version: String,


    /// 
    /// The split charge rules that are used to allocate your charges between your Cost       Category values.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "SplitChargeRules")]
    pub split_charge_rules: Option<String>,


    /// 
    /// The default value for the cost category.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,

}

impl cfn_resources::CfnResource for CfnCostCategory {
    fn type_string() -> &'static str {
        "AWS::CE::CostCategory"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
