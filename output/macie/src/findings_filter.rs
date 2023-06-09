/// The AWS::Macie::FindingsFilter resource specifies a findings filter. In Amazon Macie, a         findings filter, also referred to as a filter         rule, is a set of custom criteria that specifies which findings to       include or exclude from the results of a query for findings. The criteria can help you       identify and focus on findings that have specific characteristics, such as severity,       type, or the name of an affected AWS resource. You can also configure a       findings filter to suppress (automatically archive) findings that match the filter's       criteria. For more information, see Filtering findings in       the Amazon Macie User Guide.
///
/// An AWS::Macie::Session resource must exist for an AWS account before you can create an         AWS::Macie::FindingsFilter resource for the account. Use a DependsOn         attribute to ensure that an AWS::Macie::Session resource is       created before other Macie resources are created for an account. For       example, "DependsOn": "Session".
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnFindingsFilter {
    ///
    /// The action to perform on findings that match the filter criteria         (FindingCriteria). Valid values are:
    ///
    /// ARCHIVE - Suppress (automatically archive) the findings.               NOOP - Don't perform any action on the findings.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<cfn_resources::StrVal>,

    ///
    /// A custom description of the findings filter. The description can contain 1-512       characters.
    ///
    /// Avoid including sensitive data in the description. Users of the account might be able       to see the description, depending on the actions that they're allowed to perform in         Amazon Macie.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The criteria to use to filter findings.
    ///
    /// Required: Yes
    ///
    /// Type: FindingCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "FindingCriteria")]
    pub finding_criteria: FindingCriteria,

    ///
    /// A custom name for the findings filter. The name can contain 3-64 characters.
    ///
    /// Avoid including sensitive data in the name. Users of the account might be able to see       the name, depending on the actions that they're allowed to perform in Amazon Macie.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The position of the findings filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter       is applied to findings, relative to other filters that are also applied to       findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,

    #[serde(skip_serializing)]
    pub att_arn: CfnFindingsFilterarn,

    #[serde(skip_serializing)]
    pub att_id: CfnFindingsFilterid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFindingsFilterarn;
impl CfnFindingsFilterarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFindingsFilterid;
impl CfnFindingsFilterid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnFindingsFilter {
    fn type_string(&self) -> &'static str {
        "AWS::Macie::FindingsFilter"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.finding_criteria.validate()?;

        Ok(())
    }
}

/// Specifies a condition that defines the property, operator, and one or more values to       use in a findings filter. A findings filter, also referred to as a filter rule, is a       set of custom criteria that specifies which findings to include or exclude from the results of a query for findings. You can also       configure a findings filter to suppress (automatically archive) findings that match the filter's criteria. For more information,       see Filtering findings in       the Amazon Macie User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CriterionAdditionalProperties {
    ///
    /// The value for the specified property matches (equals) the specified value. If you specify       multiple values, Amazon Macie uses OR logic to join the values.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "eq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,

    ///
    /// The value for the specified property is greater than the specified value.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "gt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<i64>,

    ///
    /// The value for the specified property is greater than or equal to the specified value.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<i64>,

    ///
    /// The value for the specified property is less than the specified value.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "lt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<i64>,

    ///
    /// The value for the specified property is less than or equal to the specified value.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<i64>,

    ///
    /// The value for the specified property doesn't match (doesn't equal) the specified       value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "neq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neq: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CriterionAdditionalProperties {
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

/// Specifies, as a map, one or more property-based conditions for a findings filter. A findings filter, also referred       to as a filter rule, is a set of custom criteria that specifies which findings to include or exclude       from the results of a query for findings. You can also configure a findings filter to suppress (automatically archive) findings that       match the filter's criteria. For more information,       see Filtering findings in       the Amazon Macie User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FindingCriteria {
    ///
    /// Specifies a condition that defines the property, operator, and one or more values to       use to filter the results.
    ///
    /// Required: No
    ///
    /// Type: Map of CriterionAdditionalProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Criterion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion: Option<std::collections::HashMap<String, CriterionAdditionalProperties>>,
}

impl cfn_resources::CfnResource for FindingCriteria {
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
