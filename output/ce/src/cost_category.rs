/// The AWS::CE::CostCategory resource creates groupings of cost that you can use    across products in the AWS Billing and Cost Management console, such as Cost Explorer and AWS Budgets. For more information, see Managing Your Costs with     Cost Categories in the AWS Billing and Cost Management User    Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCostCategory {
    ///
    /// The default value for the cost category.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<cfn_resources::StrVal>,

    ///
    /// The unique name of the Cost Category.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The rule schema version in this particular Cost Category.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleVersion")]
    pub rule_version: cfn_resources::StrVal,

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
    pub rules: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_charge_rules: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnCostCategoryarn,

    #[serde(skip_serializing)]
    pub att_effective_start: CfnCostCategoryeffectivestart,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCostCategoryarn;
impl CfnCostCategoryarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCostCategoryeffectivestart;
impl CfnCostCategoryeffectivestart {
    pub fn att_name(&self) -> &'static str {
        r#"EffectiveStart"#
    }
}

impl cfn_resources::CfnResource for CfnCostCategory {
    fn type_string(&self) -> &'static str {
        "AWS::CE::CostCategory"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.rules;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 500 as _ {
                return Err(format!(
                    "Max validation failed on field 'rules'. {} is greater than 500",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.split_charge_rules {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10 as _ {
                    return Err(format!("Max validation failed on field 'split_charge_rules'. {} is greater than 10", s.len()));
                }
            }
        }

        Ok(())
    }
}
