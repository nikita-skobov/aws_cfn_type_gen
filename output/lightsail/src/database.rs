/// The AWS::Lightsail::Database resource specifies an Amazon Lightsail database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDatabase {
    ///
    /// The Availability Zone for the database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,

    ///
    /// A Boolean value indicating whether automated backup retention is enabled for the     database.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "BackupRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention: Option<bool>,

    ///
    /// The certificate associated with the database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaCertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_identifier: Option<String>,

    ///
    /// The meaning of this parameter differs according to the database engine you use.
    ///
    /// MySQL
    ///
    /// The name of the database to create when the Lightsail database resource     is created. If this parameter isn't specified, no database is created in the database     resource.
    ///
    /// Constraints:
    ///
    /// Must contain 1-64 letters or numbers.            Must begin with a letter. Subsequent characters can be letters, underscores, or        numbers (0-9).            Can't be a word reserved by the specified database engine.       For more information about reserved words in MySQL, see the Keywords and Reserved        Words articles for MySQL 5.6, MySQL 5.7, and          MySQL          8.0.
    ///
    /// PostgreSQL
    ///
    /// The name of the database to create when the Lightsail database resource     is created. If this parameter isn't specified, a database named postgres is     created in the database resource.
    ///
    /// Constraints:
    ///
    /// Must contain 1-63 letters or numbers.            Must begin with a letter. Subsequent characters can be letters, underscores, or        numbers (0-9).            Can't be a word reserved by the specified database engine.       For more information about reserved words in PostgreSQL, see the SQL Key Words        articles for PostgreSQL          9.6, PostgreSQL          10, PostgreSQL          11, and PostgreSQL          12.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "MasterDatabaseName")]
    pub master_database_name: String,

    ///
    /// The password for the primary user of the database. The password can include any     printable ASCII character except the following: /, ", or @. It cannot contain     spaces.
    ///
    /// NoteThe MasterUserPassword and RotateMasterUserPassword       parameters cannot be used together in the same template.
    ///
    /// MySQL
    ///
    /// Constraints: Must contain 8-41 characters.
    ///
    /// PostgreSQL
    ///
    /// Constraints: Must contain 8-128 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,

    ///
    /// The name for the primary user.
    ///
    /// MySQL
    ///
    /// Constraints:
    ///
    /// Required for MySQL.            Must be 1-16 letters or numbers. Can contain underscores.            First character must be a letter.            Can't be a reserved word for the chosen database engine.       For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords        and Reserved Words articles for MySQL 5.6,          MySQL          5.7, or MySQL 8.0.
    ///
    /// PostgreSQL
    ///
    /// Constraints:
    ///
    /// Required for PostgreSQL.            Must be 1-63 letters or numbers. Can contain underscores.            First character must be a letter.            Can't be a reserved word for the chosen database engine.       For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords        and Reserved Words articles for PostgreSQL          9.6, PostgreSQL          10, PostgreSQL          11, and PostgreSQL          12.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "MasterUsername")]
    pub master_username: String,

    ///
    /// The daily time range during which automated backups are created for the database (for     example, 16:00-16:30).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,

    ///
    /// The weekly time range during which system maintenance can occur for the database,     formatted as follows: ddd:hh24:mi-ddd:hh24:mi. For example,       Tue:17:00-Tue:17:30.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,

    ///
    /// A Boolean value indicating whether the database is accessible to anyone on the     internet.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,

    ///
    /// The blueprint ID for the database (for example, mysql_8_0).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RelationalDatabaseBlueprintId")]
    pub relational_database_blueprint_id: String,

    ///
    /// The bundle ID for the database (for example, medium_1_0).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RelationalDatabaseBundleId")]
    pub relational_database_bundle_id: String,

    ///
    /// The name of the instance.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RelationalDatabaseName")]
    pub relational_database_name: String,

    ///
    /// An array of parameters for the database.
    ///
    /// Required: No
    ///
    /// Type: List of RelationalDatabaseParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelationalDatabaseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_parameters: Option<Vec<RelationalDatabaseParameter>>,

    ///
    /// A Boolean value indicating whether to change the primary user password to a new, strong     password generated by Lightsail.
    ///
    /// NoteThe RotateMasterUserPassword and MasterUserPassword       parameters cannot be used together in the same template.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotateMasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_master_user_password: Option<bool>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag     in the AWS CloudFormation User Guide.
    ///
    /// NoteThe Value of Tags is optional for Lightsail resources.
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

impl cfn_resources::CfnResource for CfnDatabase {
    fn type_string(&self) -> &'static str {
        "AWS::Lightsail::Database"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// RelationalDatabaseParameter is a property of the AWS::Lightsail::Database resource. It describes parameters for the     database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RelationalDatabaseParameter {
    ///
    /// The valid range of values for the parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,

    ///
    /// Indicates when parameter updates are applied.
    ///
    /// Can be immediate or pending-reboot.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplyMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_method: Option<String>,

    ///
    /// Specifies the engine-specific parameter type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_type: Option<String>,

    ///
    /// The valid data type of the parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,

    ///
    /// A description of the parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///
    /// A Boolean value indicating whether the parameter can be modified.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsModifiable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<bool>,

    ///
    /// The name of the parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,

    ///
    /// The value for the parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
}

impl cfn_resources::CfnResource for RelationalDatabaseParameter {
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
