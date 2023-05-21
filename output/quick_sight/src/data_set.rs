/// Creates a dataset. This operation doesn't support datasets that include uploaded files as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataSet {
    ///
    /// The AWS account ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,

    ///
    /// Groupings of columns that work together in certain Amazon QuickSight features. Currently, only geospatial hierarchy is supported.
    ///
    /// Required: No
    ///
    /// Type: List of ColumnGroup
    ///
    /// Maximum: 8
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnGroups")]
    pub column_groups: Option<Vec<ColumnGroup>>,

    ///
    /// A set of one or more definitions of a ColumnLevelPermissionRule .
    ///
    /// Required: No
    ///
    /// Type: List of ColumnLevelPermissionRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnLevelPermissionRules")]
    pub column_level_permission_rules: Option<Vec<ColumnLevelPermissionRule>>,

    ///
    /// An ID for the dataset that you want to create. This ID is unique per AWS Region for each AWS account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSetId")]
    pub data_set_id: Option<String>,

    ///
    /// The usage configuration to apply to child datasets that reference this dataset as a source.
    ///
    /// Required: No
    ///
    /// Type: DataSetUsageConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetUsageConfiguration")]
    pub data_set_usage_configuration: Option<DataSetUsageConfiguration>,

    ///
    /// The folder that contains fields and nested subfolders for your dataset.
    ///
    /// Required: No
    ///
    /// Type: Map of FieldFolder
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldFolders")]
    pub field_folders: Option<std::collections::HashMap<String, FieldFolder>>,

    ///
    /// Indicates whether you want to import the data into SPICE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DIRECT_QUERY | SPICE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImportMode")]
    pub import_mode: Option<DataSetImportModeEnum>,

    ///
    /// The wait policy to use when creating or updating a Dataset. The default is to wait for SPICE ingestion to finish  with timeout of 36 hours.
    ///
    /// Required: No
    ///
    /// Type: IngestionWaitPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngestionWaitPolicy")]
    pub ingestion_wait_policy: Option<IngestionWaitPolicy>,

    ///
    /// Configures the combination and transformation of the data from the physical tables.
    ///
    /// Required: No
    ///
    /// Type: Map of LogicalTable
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogicalTableMap")]
    pub logical_table_map: Option<std::collections::HashMap<String, LogicalTable>>,

    ///
    /// The display name for the dataset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// A list of resource permissions on the dataset.
    ///
    /// Required: No
    ///
    /// Type: List of ResourcePermission
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<ResourcePermission>>,

    ///
    /// Declares the physical tables that are available in the underlying data sources.
    ///
    /// Required: No
    ///
    /// Type: Map of PhysicalTable
    ///
    /// Update requires: No interruption
    #[serde(rename = "PhysicalTableMap")]
    pub physical_table_map: Option<std::collections::HashMap<String, PhysicalTable>>,

    ///
    /// The row-level security configuration for the data that you want to create.
    ///
    /// Required: No
    ///
    /// Type: RowLevelPermissionDataSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowLevelPermissionDataSet")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,

    ///
    /// Contains a map of the key-value pairs for the resource tag or tags assigned to the dataset.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DataSetImportModeEnum {
    /// DIRECT_QUERY
    #[serde(rename = "DIRECT_QUERY")]
    Directquery,

    /// SPICE
    #[serde(rename = "SPICE")]
    Spice,
}

impl Default for DataSetImportModeEnum {
    fn default() -> Self {
        DataSetImportModeEnum::Directquery
    }
}

impl cfn_resources::CfnResource for CfnDataSet {
    fn type_string(&self) -> &'static str {
        "AWS::QuickSight::DataSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.aws_account_id {
            if the_val.len() > 12 as _ {
                return Err(format!(
                    "Max validation failed on field 'aws_account_id'. {} is greater than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.aws_account_id {
            if the_val.len() < 12 as _ {
                return Err(format!(
                    "Min validation failed on field 'aws_account_id'. {} is less than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.column_groups {
            if the_val.len() > 8 as _ {
                return Err(format!(
                    "Max validation failed on field 'column_groups'. {} is greater than 8",
                    the_val.len()
                ));
            }
        }

        self.data_set_usage_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ingestion_wait_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.permissions {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'permissions'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        self.row_level_permission_data_set
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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

/// A calculated column for a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CalculatedColumn {
    ///
    /// A unique ID to identify a calculated column. During a dataset update, if the column ID       of a calculated column matches that of an existing calculated column, Amazon QuickSight       preserves the existing calculated column.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnId")]
    pub column_id: String,

    ///
    /// Column name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnName")]
    pub column_name: String,

    ///
    /// An expression that defines the calculated column.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,
}

impl cfn_resources::CfnResource for CalculatedColumn {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.column_id;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'column_id'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.column_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'column_id'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.column_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'column_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.column_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'column_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.expression;

        if the_val.len() > 4096 as _ {
            return Err(format!(
                "Max validation failed on field 'expression'. {} is greater than 4096",
                the_val.len()
            ));
        }

        let the_val = &self.expression;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'expression'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// A transform operation that casts a column to a different type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CastColumnTypeOperation {
    ///
    /// Column name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnName")]
    pub column_name: String,

    ///
    /// When casting a column from string to datetime type, you can supply a string in a       format supported by Amazon QuickSight to denote the source data format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<String>,

    ///
    /// New column data type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DATETIME | DECIMAL | INTEGER | STRING
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewColumnType")]
    pub new_column_type: CastColumnTypeOperationNewColumnTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CastColumnTypeOperationNewColumnTypeEnum {
    /// DATETIME
    #[serde(rename = "DATETIME")]
    Datetime,

    /// DECIMAL
    #[serde(rename = "DECIMAL")]
    Decimal,

    /// INTEGER
    #[serde(rename = "INTEGER")]
    Integer,

    /// STRING
    #[serde(rename = "STRING")]
    String,
}

impl Default for CastColumnTypeOperationNewColumnTypeEnum {
    fn default() -> Self {
        CastColumnTypeOperationNewColumnTypeEnum::Datetime
    }
}

impl cfn_resources::CfnResource for CastColumnTypeOperation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.column_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'column_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.column_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'column_name'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.format {
            if the_val.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'format'. {} is greater than 32",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Metadata that contains a description for a column.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColumnDescription {
    ///
    /// The text of a description for a column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "Text")]
    pub text: Option<String>,
}

impl cfn_resources::CfnResource for ColumnDescription {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.text {
            if the_val.len() > 500 as _ {
                return Err(format!(
                    "Max validation failed on field 'text'. {} is greater than 500",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.text {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'text'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Groupings of columns that work together in certain Amazon QuickSight features. This is       a variant type structure. For this structure to be valid, only one of the attributes can       be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColumnGroup {
    ///
    /// Geospatial column group that denotes a hierarchy.
    ///
    /// Required: No
    ///
    /// Type: GeoSpatialColumnGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeoSpatialColumnGroup")]
    pub geo_spatial_column_group: Option<GeoSpatialColumnGroup>,
}

impl cfn_resources::CfnResource for ColumnGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.geo_spatial_column_group
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A rule defined to grant access on one or more restricted columns. Each dataset can have multiple rules. To  create a restricted column, you add it to one or more rules. Each rule must contain at least one column and at least  one user or group. To be able to see a restricted column, a user or group needs to be added to a rule for that  column.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColumnLevelPermissionRule {
    ///
    /// An array of column names.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnNames")]
    pub column_names: Option<Vec<String>>,

    ///
    /// An array of Amazon Resource Names (ARNs) for Amazon QuickSight users or groups.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Principals")]
    pub principals: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for ColumnLevelPermissionRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A tag for a column in a               TagColumnOperation             structure. This is a       variant type structure. For this structure to be valid, only one of the attributes can       be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColumnTag {
    ///
    /// A description for a column.
    ///
    /// Required: No
    ///
    /// Type: ColumnDescription
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnDescription")]
    pub column_description: Option<ColumnDescription>,

    ///
    /// A geospatial role for a column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CITY | COUNTRY | COUNTY | LATITUDE | LONGITUDE | POSTCODE | STATE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnGeographicRole")]
    pub column_geographic_role: Option<ColumnTagColumnGeographicRoleEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ColumnTagColumnGeographicRoleEnum {
    /// CITY
    #[serde(rename = "CITY")]
    City,

    /// COUNTRY
    #[serde(rename = "COUNTRY")]
    Country,

    /// COUNTY
    #[serde(rename = "COUNTY")]
    County,

    /// LATITUDE
    #[serde(rename = "LATITUDE")]
    Latitude,

    /// LONGITUDE
    #[serde(rename = "LONGITUDE")]
    Longitude,

    /// POSTCODE
    #[serde(rename = "POSTCODE")]
    Postcode,

    /// STATE
    #[serde(rename = "STATE")]
    State,
}

impl Default for ColumnTagColumnGeographicRoleEnum {
    fn default() -> Self {
        ColumnTagColumnGeographicRoleEnum::City
    }
}

impl cfn_resources::CfnResource for ColumnTag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.column_description
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A transform operation that creates calculated columns. Columns created in one such       operation form a lexical closure.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreateColumnsOperation {
    ///
    /// Calculated columns to create.
    ///
    /// Required: Yes
    ///
    /// Type: List of CalculatedColumn
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Vec<CalculatedColumn>,
}

impl cfn_resources::CfnResource for CreateColumnsOperation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.columns;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'columns'. {} is greater than 128",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// A physical table type built from the results of the custom SQL query.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomSql {
    ///
    /// The column schema from the SQL query result set.
    ///
    /// Required: Yes
    ///
    /// Type: List of InputColumn
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Vec<InputColumn>,

    ///
    /// The Amazon Resource Name (ARN) of the data source.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,

    ///
    /// A display name for the SQL query result.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The SQL query.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65536
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlQuery")]
    pub sql_query: String,
}

impl cfn_resources::CfnResource for CustomSql {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.columns;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'columns'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.sql_query;

        if the_val.len() > 65536 as _ {
            return Err(format!(
                "Max validation failed on field 'sql_query'. {} is greater than 65536",
                the_val.len()
            ));
        }

        let the_val = &self.sql_query;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'sql_query'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The usage configuration to apply to child datasets that reference this dataset as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSetUsageConfiguration {
    ///
    /// An option that controls whether a child dataset of a direct query can use this dataset as a source.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableUseAsDirectQuerySource")]
    pub disable_use_as_direct_query_source: Option<bool>,

    ///
    /// An option that controls whether a child dataset that's stored in QuickSight can use this dataset as a source.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableUseAsImportedSource")]
    pub disable_use_as_imported_source: Option<bool>,
}

impl cfn_resources::CfnResource for DataSetUsageConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A FieldFolder element is a folder that contains fields and nested subfolders.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FieldFolder {
    ///
    /// A folder has a list of columns. A column can only be in one folder.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<String>>,

    ///
    /// The description for a field folder.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

impl cfn_resources::CfnResource for FieldFolder {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A transform operation that filters rows based on a condition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterOperation {
    ///
    /// An expression that must evaluate to a Boolean value. Rows for which the expression       evaluates to true are kept in the dataset.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: String,
}

impl cfn_resources::CfnResource for FilterOperation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.condition_expression;

        if the_val.len() > 4096 as _ {
            return Err(format!(
                "Max validation failed on field 'condition_expression'. {} is greater than 4096",
                the_val.len()
            ));
        }

        let the_val = &self.condition_expression;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'condition_expression'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Geospatial column group that denotes a hierarchy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GeoSpatialColumnGroup {
    ///
    /// Columns in this hierarchy.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Vec<String>,

    ///
    /// Country code.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: US
    ///
    /// Update requires: No interruption
    #[serde(rename = "CountryCode")]
    pub country_code: Option<GeoSpatialColumnGroupCountryCodeEnum>,

    ///
    /// A display name for the hierarchy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum GeoSpatialColumnGroupCountryCodeEnum {
    /// US
    #[serde(rename = "US")]
    Us,
}

impl Default for GeoSpatialColumnGroupCountryCodeEnum {
    fn default() -> Self {
        GeoSpatialColumnGroupCountryCodeEnum::Us
    }
}

impl cfn_resources::CfnResource for GeoSpatialColumnGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.columns;

        if the_val.len() > 16 as _ {
            return Err(format!(
                "Max validation failed on field 'columns'. {} is greater than 16",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The wait policy to use when creating or updating a Dataset. The default is to wait for SPICE ingestion to finish  with timeout of 36 hours.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IngestionWaitPolicy {
    ///
    /// The maximum time (in hours) to wait for Ingestion to complete. Default timeout is 36 hours. Applicable only when  DataSetImportMode mode is set to SPICE and WaitForSpiceIngestion is set to true.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngestionWaitTimeInHours")]
    pub ingestion_wait_time_in_hours: Option<f64>,

    ///
    /// Wait for SPICE ingestion to finish to mark dataset creation or update as successful. Default (true). Applicable  only when DataSetImportMode mode is set to SPICE.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WaitForSpiceIngestion")]
    pub wait_for_spice_ingestion: Option<bool>,
}

impl cfn_resources::CfnResource for IngestionWaitPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Metadata for a column that is used as the input of a transform operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputColumn {
    ///
    /// The name of this column in the underlying data source.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The data type of the column.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BIT | BOOLEAN | DATETIME | DECIMAL | INTEGER | JSON | STRING
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: InputColumnTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InputColumnTypeEnum {
    /// BIT
    #[serde(rename = "BIT")]
    Bit,

    /// BOOLEAN
    #[serde(rename = "BOOLEAN")]
    Boolean,

    /// DATETIME
    #[serde(rename = "DATETIME")]
    Datetime,

    /// DECIMAL
    #[serde(rename = "DECIMAL")]
    Decimal,

    /// INTEGER
    #[serde(rename = "INTEGER")]
    Integer,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

    /// STRING
    #[serde(rename = "STRING")]
    String,
}

impl Default for InputColumnTypeEnum {
    fn default() -> Self {
        InputColumnTypeEnum::Bit
    }
}

impl cfn_resources::CfnResource for InputColumn {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The instructions associated with a join.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JoinInstruction {
    ///
    /// Join key properties of the left operand.
    ///
    /// Required: No
    ///
    /// Type: JoinKeyProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "LeftJoinKeyProperties")]
    pub left_join_key_properties: Option<JoinKeyProperties>,

    ///
    /// The operand on the left side of a join.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LeftOperand")]
    pub left_operand: String,

    ///
    /// The join instructions provided in the ON clause of a join.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnClause")]
    pub on_clause: String,

    ///
    /// Join key properties of the right operand.
    ///
    /// Required: No
    ///
    /// Type: JoinKeyProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "RightJoinKeyProperties")]
    pub right_join_key_properties: Option<JoinKeyProperties>,

    ///
    /// The operand on the right side of a join.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RightOperand")]
    pub right_operand: String,

    ///
    /// The type of join that it is.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: INNER | LEFT | OUTER | RIGHT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: JoinInstructionTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum JoinInstructionTypeEnum {
    /// INNER
    #[serde(rename = "INNER")]
    Inner,

    /// LEFT
    #[serde(rename = "LEFT")]
    Left,

    /// OUTER
    #[serde(rename = "OUTER")]
    Outer,

    /// RIGHT
    #[serde(rename = "RIGHT")]
    Right,
}

impl Default for JoinInstructionTypeEnum {
    fn default() -> Self {
        JoinInstructionTypeEnum::Inner
    }
}

impl cfn_resources::CfnResource for JoinInstruction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.left_join_key_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.on_clause;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'on_clause'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.on_clause;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'on_clause'. {} is less than 1",
                the_val.len()
            ));
        }

        self.right_join_key_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Properties associated with the columns participating in a join.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JoinKeyProperties {
    ///
    /// A value that indicates that a row in a table is uniquely identified by the columns in a join key. This is used  by Amazon QuickSight to optimize query performance.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UniqueKey")]
    pub unique_key: Option<bool>,
}

impl cfn_resources::CfnResource for JoinKeyProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A logical table is a unit that joins and that data       transformations operate on. A logical table has a source, which can be either a physical       table or result of a join. When a logical table points to a physical table, the logical       table acts as a mutable copy of that physical table through transform operations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogicalTable {
    ///
    /// A display name for the logical table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alias")]
    pub alias: String,

    ///
    /// Transform operations that act on this logical table. For this structure to be valid, only one of the attributes can be non-null.
    ///
    /// Required: No
    ///
    /// Type: List of TransformOperation
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTransforms")]
    pub data_transforms: Option<Vec<TransformOperation>>,

    ///
    /// Source of this logical table.
    ///
    /// Required: Yes
    ///
    /// Type: LogicalTableSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: LogicalTableSource,
}

impl cfn_resources::CfnResource for LogicalTable {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.alias;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'alias'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.alias;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'alias'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.data_transforms {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'data_transforms'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        self.source.validate()?;

        Ok(())
    }
}

/// Information about the source of a logical table. This is a variant type structure. For       this structure to be valid, only one of the attributes can be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogicalTableSource {
    ///
    /// The Amazon Resource Number (ARN) of the parent dataset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSetArn")]
    pub data_set_arn: Option<String>,

    ///
    /// Specifies the result of a join of two logical tables.
    ///
    /// Required: No
    ///
    /// Type: JoinInstruction
    ///
    /// Update requires: No interruption
    #[serde(rename = "JoinInstruction")]
    pub join_instruction: Option<JoinInstruction>,

    ///
    /// Physical table ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [0-9a-zA-Z-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PhysicalTableId")]
    pub physical_table_id: Option<String>,
}

impl cfn_resources::CfnResource for LogicalTableSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.join_instruction
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.physical_table_id {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'physical_table_id'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.physical_table_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'physical_table_id'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Output column.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputColumn {
    ///
    /// A description for a column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// A display name for the dataset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// The type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DATETIME | DECIMAL | INTEGER | STRING
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<OutputColumnTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OutputColumnTypeEnum {
    /// DATETIME
    #[serde(rename = "DATETIME")]
    Datetime,

    /// DECIMAL
    #[serde(rename = "DECIMAL")]
    Decimal,

    /// INTEGER
    #[serde(rename = "INTEGER")]
    Integer,

    /// STRING
    #[serde(rename = "STRING")]
    String,
}

impl Default for OutputColumnTypeEnum {
    fn default() -> Self {
        OutputColumnTypeEnum::Datetime
    }
}

impl cfn_resources::CfnResource for OutputColumn {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 500 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 500",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'description'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A view of a data source that contains information about the shape of the data in the       underlying source. This is a variant type structure. For this structure to be valid,       only one of the attributes can be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PhysicalTable {
    ///
    /// A physical table type built from the results of the custom SQL query.
    ///
    /// Required: No
    ///
    /// Type: CustomSql
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomSql")]
    pub custom_sql: Option<CustomSql>,

    ///
    /// A physical table type for relational data sources.
    ///
    /// Required: No
    ///
    /// Type: RelationalTable
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelationalTable")]
    pub relational_table: Option<RelationalTable>,

    ///
    /// A physical table type for as S3 data source.
    ///
    /// Required: No
    ///
    /// Type: S3Source
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Source")]
    pub s3_source: Option<S3Source>,
}

impl cfn_resources::CfnResource for PhysicalTable {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.custom_sql
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.relational_table
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_source
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A transform operation that projects columns. Operations that come after a projection       can only refer to projected columns.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProjectOperation {
    ///
    /// Projected columns.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 2000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProjectedColumns")]
    pub projected_columns: Vec<String>,
}

impl cfn_resources::CfnResource for ProjectOperation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.projected_columns;

        if the_val.len() > 2000 as _ {
            return Err(format!(
                "Max validation failed on field 'projected_columns'. {} is greater than 2000",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// A physical table type for relational data sources.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RelationalTable {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Catalog")]
    pub catalog: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) for the data source.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,

    ///
    /// The column schema of the table.
    ///
    /// Required: Yes
    ///
    /// Type: List of InputColumn
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputColumns")]
    pub input_columns: Vec<InputColumn>,

    ///
    /// The name of the relational table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The schema name. This name applies to certain relational database engines.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schema")]
    pub schema: Option<String>,
}

impl cfn_resources::CfnResource for RelationalTable {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.input_columns;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'input_columns'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.schema {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'schema'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A transform operation that renames a column.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RenameColumnOperation {
    ///
    /// The name of the column to be renamed.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnName")]
    pub column_name: String,

    ///
    /// The new name for the column.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewColumnName")]
    pub new_column_name: String,
}

impl cfn_resources::CfnResource for RenameColumnOperation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.column_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'column_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.column_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'column_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.new_column_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'new_column_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.new_column_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'new_column_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Permission for the resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourcePermission {
    ///
    /// The IAM action to grant or revoke permisions on
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,

    ///
    /// The Amazon Resource Name (ARN) of the principal. This can be one of the following:
    ///
    /// The ARN of an Amazon QuickSight user or group associated with a data source or dataset. (This is   common.)     The ARN of an Amazon QuickSight user, group, or namespace associated with an analysis, dashboard,   template, or theme. (This is common.)     The ARN of an AWS account root: This is an IAM ARN rather than a Amazon QuickSight ARN. Use this option only to share resources (templates) across AWS accounts. (This is   less common.)
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Principal")]
    pub principal: String,
}

impl cfn_resources::CfnResource for ResourcePermission {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Information about a dataset that contains permissions for row-level security (RLS).       The permissions dataset maps fields to users or groups. For more information, see       Using Row-Level Security (RLS) to Restrict Access to a Dataset in the Amazon QuickSight User         Guide.
///
/// The option to deny permissions by setting PermissionPolicy to DENY_ACCESS is       not supported for new RLS datasets.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RowLevelPermissionDataSet {
    ///
    /// The Amazon Resource Name (ARN) of the dataset that contains permissions for RLS.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: String,

    ///
    /// The user or group rules associated with the dataset that contains permissions for RLS.
    ///
    /// By default, FormatVersion is VERSION_1. When FormatVersion is VERSION_1, UserName and GroupName are required. When FormatVersion is VERSION_2, UserARN and GroupARN are required, and Namespace must not exist.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: VERSION_1 | VERSION_2
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatVersion")]
    pub format_version: Option<RowLevelPermissionDataSetFormatVersionEnum>,

    ///
    /// The namespace associated with the dataset that contains permissions for RLS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9._-]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

    ///
    /// The type of permissions to use when interpreting the permissions for RLS. DENY_ACCESS     is included for backward compatibility only.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DENY_ACCESS | GRANT_ACCESS
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionPolicy")]
    pub permission_policy: RowLevelPermissionDataSetPermissionPolicyEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RowLevelPermissionDataSetFormatVersionEnum {
    /// VERSION_1
    #[serde(rename = "VERSION_1")]
    Version1,

    /// VERSION_2
    #[serde(rename = "VERSION_2")]
    Version2,
}

impl Default for RowLevelPermissionDataSetFormatVersionEnum {
    fn default() -> Self {
        RowLevelPermissionDataSetFormatVersionEnum::Version1
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RowLevelPermissionDataSetPermissionPolicyEnum {
    /// DENY_ACCESS
    #[serde(rename = "DENY_ACCESS")]
    Denyaccess,

    /// GRANT_ACCESS
    #[serde(rename = "GRANT_ACCESS")]
    Grantaccess,
}

impl Default for RowLevelPermissionDataSetPermissionPolicyEnum {
    fn default() -> Self {
        RowLevelPermissionDataSetPermissionPolicyEnum::Denyaccess
    }
}

impl cfn_resources::CfnResource for RowLevelPermissionDataSet {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.namespace {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'namespace'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A physical table type for an S3 data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Source {
    ///
    /// The Amazon Resource Name (ARN) for the data source.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceArn")]
    pub data_source_arn: String,

    ///
    /// A physical table type for an S3 data source.
    ///
    /// NoteFor files that aren't JSON, only STRING data types are supported in input columns.
    ///
    /// Required: Yes
    ///
    /// Type: List of InputColumn
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputColumns")]
    pub input_columns: Vec<InputColumn>,

    ///
    /// Information about the format for the S3 source file or files.
    ///
    /// Required: No
    ///
    /// Type: UploadSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "UploadSettings")]
    pub upload_settings: Option<UploadSettings>,
}

impl cfn_resources::CfnResource for S3Source {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.input_columns;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'input_columns'. {} is greater than 2048",
                the_val.len()
            ));
        }

        self.upload_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A transform operation that tags a column with additional information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagColumnOperation {
    ///
    /// The column that this operation acts on.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnName")]
    pub column_name: String,

    ///
    /// The dataset column tag, currently only used for geospatial type tagging.
    ///
    /// NoteThis is not tags for the AWS tagging feature.
    ///
    /// Required: Yes
    ///
    /// Type: List of ColumnTag
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Vec<ColumnTag>,
}

impl cfn_resources::CfnResource for TagColumnOperation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.column_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'column_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.column_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'column_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.tags;

        if the_val.len() > 16 as _ {
            return Err(format!(
                "Max validation failed on field 'tags'. {} is greater than 16",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// A data transformation on a logical table. This is a variant type structure. For this       structure to be valid, only one of the attributes can be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransformOperation {
    ///
    /// A transform operation that casts a column to a different type.
    ///
    /// Required: No
    ///
    /// Type: CastColumnTypeOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "CastColumnTypeOperation")]
    pub cast_column_type_operation: Option<CastColumnTypeOperation>,

    ///
    /// An operation that creates calculated columns. Columns created in one such operation       form a lexical closure.
    ///
    /// Required: No
    ///
    /// Type: CreateColumnsOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateColumnsOperation")]
    pub create_columns_operation: Option<CreateColumnsOperation>,

    ///
    /// An operation that filters rows based on some condition.
    ///
    /// Required: No
    ///
    /// Type: FilterOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterOperation")]
    pub filter_operation: Option<FilterOperation>,

    ///
    /// An operation that projects columns. Operations that come after a projection can only       refer to projected columns.
    ///
    /// Required: No
    ///
    /// Type: ProjectOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProjectOperation")]
    pub project_operation: Option<ProjectOperation>,

    ///
    /// An operation that renames a column.
    ///
    /// Required: No
    ///
    /// Type: RenameColumnOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "RenameColumnOperation")]
    pub rename_column_operation: Option<RenameColumnOperation>,

    ///
    /// An operation that tags a column with additional information.
    ///
    /// Required: No
    ///
    /// Type: TagColumnOperation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagColumnOperation")]
    pub tag_column_operation: Option<TagColumnOperation>,
}

impl cfn_resources::CfnResource for TransformOperation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.cast_column_type_operation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.create_columns_operation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.filter_operation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.project_operation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rename_column_operation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.tag_column_operation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the format for a source file or files.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UploadSettings {
    ///
    /// Whether the file has a header row, or the files each have a header row.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainsHeader")]
    pub contains_header: Option<bool>,

    ///
    /// The delimiter between values in the file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

    ///
    /// File format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CLF | CSV | ELF | JSON | TSV | XLSX
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<UploadSettingsFormatEnum>,

    ///
    /// A row number to start reading data from.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartFromRow")]
    pub start_from_row: Option<f64>,

    ///
    /// Text qualifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DOUBLE_QUOTE | SINGLE_QUOTE
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextQualifier")]
    pub text_qualifier: Option<UploadSettingsTextQualifierEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum UploadSettingsFormatEnum {
    /// CLF
    #[serde(rename = "CLF")]
    Clf,

    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// ELF
    #[serde(rename = "ELF")]
    Elf,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

    /// TSV
    #[serde(rename = "TSV")]
    Tsv,

    /// XLSX
    #[serde(rename = "XLSX")]
    Xlsx,
}

impl Default for UploadSettingsFormatEnum {
    fn default() -> Self {
        UploadSettingsFormatEnum::Clf
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum UploadSettingsTextQualifierEnum {
    /// DOUBLE_QUOTE
    #[serde(rename = "DOUBLE_QUOTE")]
    Doublequote,

    /// SINGLE_QUOTE
    #[serde(rename = "SINGLE_QUOTE")]
    Singlequote,
}

impl Default for UploadSettingsTextQualifierEnum {
    fn default() -> Self {
        UploadSettingsTextQualifierEnum::Doublequote
    }
}

impl cfn_resources::CfnResource for UploadSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.delimiter {
            if the_val.len() > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'delimiter'. {} is greater than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.delimiter {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'delimiter'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.start_from_row {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'start_from_row'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}
