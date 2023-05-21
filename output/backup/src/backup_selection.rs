/// Specifies a set of resources to assign to a backup plan.
///
/// For a sample AWS CloudFormation template, see the AWS Backup Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBackupSelection {
    ///
    /// Uniquely identifies a backup plan.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,

    ///
    /// Specifies the body of a request to assign a set of resources to a backup plan.
    ///
    /// It includes an array of resources, an optional array of patterns to exclude resources,     an optional role to provide access to the AWS service the resource belongs     to, and an optional array of tags used to identify a set of resources.
    ///
    /// Required: Yes
    ///
    /// Type: BackupSelectionResourceType
    ///
    /// Update requires: Replacement
    #[serde(rename = "BackupSelection")]
    pub backup_selection: BackupSelectionResourceType,
}

impl cfn_resources::CfnResource for CfnBackupSelection {
    fn type_string(&self) -> &'static str {
        "AWS::Backup::BackupSelection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.backup_selection.validate()?;

        Ok(())
    }
}

/// Specifies an object containing properties used to assign a set of resources to a backup     plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BackupSelectionResourceType {
    ///
    /// A list of conditions that you define to assign resources to your backup plans using     tags. For example, "StringEquals": { "ConditionKey": "aws:ResourceTag/CreatedByCryo",       "ConditionValue": "true" },. Condition operators are case sensitive.
    ///
    /// Conditions differs from ListOfTags as follows:
    ///
    /// When you specify more than one condition, you only assign the resources that match        ALL conditions (using AND logic).            Conditions supports StringEquals,          StringLike, StringNotEquals, and          StringNotLike. ListOfTags only supports          StringEquals.
    ///
    /// Required: No
    ///
    /// Type: Conditions
    ///
    /// Update requires: Replacement
    #[serde(rename = "Conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,

    ///
    /// The ARN of the IAM role that AWS Backup uses to authenticate when backing up     the target resource; for example,     arn:aws:iam::123456789012:role/S3Access.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,

    ///
    /// A list of conditions that you define to assign resources to your backup plans using     tags. For example, "StringEquals": { "ConditionKey": "aws:ResourceTag/CreatedByCryo",       "ConditionValue": "true" },. Condition operators are case sensitive.
    ///
    /// ListOfTags differs from Conditions as follows:
    ///
    /// When you specify more than one condition, you assign all resources that match AT        LEAST ONE condition (using OR logic).            ListOfTags only supports StringEquals.          Conditions supports StringEquals,          StringLike, StringNotEquals, and          StringNotLike.
    ///
    /// Required: No
    ///
    /// Type: List of ConditionResourceType
    ///
    /// Update requires: Replacement
    #[serde(rename = "ListOfTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_of_tags: Option<Vec<ConditionResourceType>>,

    ///
    /// A list of Amazon Resource Names (ARNs) to exclude from a backup plan. The maximum number     of ARNs is 500 without wildcards, or 30 ARNs with wildcards.
    ///
    /// If you need to exclude many resources from a backup plan, consider a different resource     selection strategy, such as assigning only one or a few resource types or refining your     resource selection using tags.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_resources: Option<Vec<String>>,

    ///
    /// An array of strings that contain Amazon Resource Names (ARNs) of resources to assign to     a backup plan.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,

    ///
    /// The display name of a resource selection document.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SelectionName")]
    pub selection_name: String,
}

impl cfn_resources::CfnResource for BackupSelectionResourceType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.conditions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Includes information about tags you define to assign tagged resources to a backup     plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConditionParameter {
    ///
    /// The key in a key-value pair. For example, in the tag Department:     Accounting, Department is the key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConditionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_key: Option<String>,

    ///
    /// The value in a key-value pair. For example, in the tag Department:       Accounting, Accounting is the value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConditionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_value: Option<String>,
}

impl cfn_resources::CfnResource for ConditionParameter {
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

/// Specifies an object that contains an array of triplets made up of a condition type (such     as STRINGEQUALS), a key, and a value. Conditions are used to filter resources     in a selection that is assigned to a backup plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConditionResourceType {
    ///
    /// The key in a key-value pair. For example, in "Department": "accounting",       "Department" is the key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConditionKey")]
    pub condition_key: String,

    ///
    /// An operation, such as STRINGEQUALS, that is applied to a key-value pair     used to filter resources in a selection.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConditionType")]
    pub condition_type: String,

    ///
    /// The value in a key-value pair. For example, in "Department": "accounting",       "accounting" is the value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConditionValue")]
    pub condition_value: String,
}

impl cfn_resources::CfnResource for ConditionResourceType {
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

/// Contains information about which resources to include or exclude from a backup plan     using their tags. Conditions are case sensitive.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Conditions {
    ///
    /// Filters the values of your tagged resources for only those resources that you tagged     with the same value. Also called "exact matching."
    ///
    /// Required: No
    ///
    /// Type: List of ConditionParameter
    ///
    /// Update requires: Replacement
    #[serde(rename = "StringEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_equals: Option<Vec<ConditionParameter>>,

    ///
    /// Filters the values of your tagged resources for matching tag values with the use of a     wildcard character (*) anywhere in the string. For example, "prod*" or "*rod*" matches the     tag value "production".
    ///
    /// Required: No
    ///
    /// Type: List of ConditionParameter
    ///
    /// Update requires: Replacement
    #[serde(rename = "StringLike")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_like: Option<Vec<ConditionParameter>>,

    ///
    /// Filters the values of your tagged resources for only those resources that you tagged     that do not have the same value. Also called "negated matching."
    ///
    /// Required: No
    ///
    /// Type: List of ConditionParameter
    ///
    /// Update requires: Replacement
    #[serde(rename = "StringNotEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_not_equals: Option<Vec<ConditionParameter>>,

    ///
    /// Filters the values of your tagged resources for non-matching tag values with the use of     a wildcard character (*) anywhere in the string.
    ///
    /// Required: No
    ///
    /// Type: List of ConditionParameter
    ///
    /// Update requires: Replacement
    #[serde(rename = "StringNotLike")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_not_like: Option<Vec<ConditionParameter>>,
}

impl cfn_resources::CfnResource for Conditions {
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
