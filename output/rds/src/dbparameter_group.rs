/// The AWS::RDS::DBParameterGroup resource creates a custom parameter group       for an RDS database family.
///
/// This type can be declared in a template and referenced in the         DBParameterGroupName property of an         AWS::RDS::DBInstance resource.
///
/// For information about configuring parameters for Amazon RDS DB instances, see       Working with parameter groups       in the Amazon RDS User Guide.
///
/// For information about configuring parameters for Amazon Aurora DB instances, see       Working with parameter         groups in the Amazon Aurora User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBParameterGroup {
    ///
    /// The name of the DB parameter group.
    ///
    /// Constraints:
    ///
    /// Must be 1 to 255 letters, numbers, or hyphens.               First character must be a letter               Can't end with a hyphen or contain two consecutive hyphens
    ///
    /// If you don't specify a value for DBParameterGroupName property, a name is automatically created for the DB parameter group.
    ///
    /// NoteThis value is stored as a lowercase string.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbparameter_group_name: Option<String>,

    ///
    /// Provides the customer-specified description for this DB parameter group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: String,

    ///
    /// The DB parameter group family name. A DB parameter group can be associated with one       and only one DB parameter group family, and can be applied only to a DB instance running       a DB engine and engine version compatible with that DB parameter group family.
    ///
    /// NoteThe DB parameter group family can't be changed when updating a DB parameter         group.
    ///
    /// To list all of the available parameter group families, use the following       command:
    ///
    /// aws rds describe-db-engine-versions --query         "DBEngineVersions[].DBParameterGroupFamily"
    ///
    /// The output contains duplicates.
    ///
    /// For more information, see CreateDBParameterGroup.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Family")]
    pub family: String,

    ///
    /// An array of parameter names and values for the parameter update. At least one       parameter name and value must be supplied. Subsequent arguments are optional.
    ///
    /// For more information about DB parameters and DB parameter groups for Amazon RDS DB       engines, see Working with DB         Parameter Groups in the Amazon RDS User Guide.
    ///
    /// For more information about DB cluster and DB instance parameters and parameter groups       for Amazon Aurora DB engines, see Working         with DB Parameter Groups and DB Cluster Parameter Groups in the         Amazon Aurora User Guide.
    ///
    /// NoteAWS CloudFormation doesn't support specifying an apply method for each individual parameter. The default         apply method for each parameter is used.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,

    ///
    /// An optional array of key-value pairs to apply to this DB parameter group.
    ///
    /// NoteCurrently, this is the only property that supports drift detection.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnDBParameterGroup {
    fn type_string(&self) -> &'static str {
        "AWS::RDS::DBParameterGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
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
