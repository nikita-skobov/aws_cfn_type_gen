/// A structure that represents a data cell filter with column-level, row-level, and/or cell-level security. Data cell filters belong to a specific table in a Data Catalog. During a stack operation,       AWS CloudFormation calls the AWS Lake Formation CreateDataCellsFilter API operation to create     a DataCellsFilter resource, and calls the DeleteDataCellsFilter API operation to delete it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataCellsFilter {
    ///
    /// An array of UTF-8 strings. A list of column names.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,

    ///
    /// A wildcard with exclusions. You must specify either a ColumnNames list or the ColumnWildCard.
    ///
    /// Required: No
    ///
    /// Type: ColumnWildcard
    ///
    /// Update requires: Replacement
    #[serde(rename = "ColumnWildcard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_wildcard: Option<ColumnWildcard>,

    ///
    /// UTF-8 string, not less than 1 or more than 255 bytes long, matching the single-line string pattern.
    ///
    /// A database in the Data Catalog.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    pub database_name: String,

    ///
    /// UTF-8 string, not less than 1 or more than 255 bytes long, matching the single-line string pattern.
    ///
    /// The name given by the user to the data filter cell.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// A PartiQL predicate.
    ///
    /// Required: No
    ///
    /// Type: RowFilter
    ///
    /// Update requires: Replacement
    #[serde(rename = "RowFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_filter: Option<RowFilter>,

    ///
    /// Catalog id string, not less than 1 or more than 255 bytes long, matching the single-line string pattern.
    ///
    /// The ID of the catalog to which the table belongs.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableCatalogId")]
    pub table_catalog_id: String,

    ///
    /// UTF-8 string, not less than 1 or more than 255 bytes long, matching the single-line string pattern.
    ///
    /// A table in the database.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableName")]
    pub table_name: String,
}

impl cfn_resources::CfnResource for CfnDataCellsFilter {
    fn type_string(&self) -> &'static str {
        "AWS::LakeFormation::DataCellsFilter"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.column_wildcard
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.row_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A wildcard object, consisting of an optional list of excluded column names or indexes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColumnWildcard {
    ///
    /// Excludes column names. Any column with this name will be excluded.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExcludedColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_column_names: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for ColumnWildcard {
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

/// A PartiQL predicate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RowFilter {
    ///
    /// A wildcard for all rows.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "AllRowsWildcard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_rows_wildcard: Option<serde_json::Value>,

    ///
    /// A filter expression.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
}

impl cfn_resources::CfnResource for RowFilter {
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
