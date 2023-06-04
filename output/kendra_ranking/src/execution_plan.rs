/// Creates a rescore execution plan. A rescore execution       plan is an Amazon Kendra Intelligent Ranking resource       used for provisioning the Rescore API. You set       the number of capacity units that you require for       Amazon Kendra Intelligent Ranking to rescore or re-rank       a search service's results.
///
/// For an example of using the       CreateRescoreExecutionPlan API, including using       the Python and Java SDKs, see Semantically         ranking a search service's results.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnExecutionPlan {
    ///
    /// You can set additional capacity units to meet the       needs of your rescore execution plan. You are given a single       capacity unit by default. If you want to use the default       capacity, you don't set additional capacity units. For more       information on the default capacity and additional capacity       units, see Adjusting         capacity.
    ///
    /// Required: No
    ///
    /// Type: CapacityUnitsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<CapacityUnitsConfiguration>,

    ///
    /// A description for the rescore execution plan.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// A name for the rescore execution plan.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: [a-zA-Z0-9][a-zA-Z0-9_-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// A list of key-value pairs that identify or categorize your       rescore execution plan. You can also use tags to help control       access to the rescore execution plan. Tag keys and values can       consist of Unicode letters, digits, white space. They can also       consist of underscore, period, colon, equal, plus, and asperand.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnExecutionPlanarn,

    #[serde(skip_serializing)]
    pub att_id: CfnExecutionPlanid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExecutionPlanarn;
impl CfnExecutionPlanarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnExecutionPlanid;
impl CfnExecutionPlanid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnExecutionPlan {
    fn type_string(&self) -> &'static str {
        "AWS::KendraRanking::ExecutionPlan"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.capacity_units
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 1000",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Sets additional capacity units configured for your       rescore execution plan. A rescore execution plan is an       Amazon Kendra Intelligent Ranking resource used for       provisioning the Rescore API. You can add and       remove capacity units to fit your usage requirements.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CapacityUnitsConfiguration {
    ///
    /// The amount of extra capacity for your rescore execution       plan.
    ///
    /// A single extra capacity unit for a rescore execution       plan provides 0.01 rescore requests per second. You can add       up to 1000 extra capacity units.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "RescoreCapacityUnits")]
    pub rescore_capacity_units: i64,
}

impl cfn_resources::CfnResource for CapacityUnitsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.rescore_capacity_units;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'rescore_capacity_units'. {} is less than 0",
                the_val
            ));
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
