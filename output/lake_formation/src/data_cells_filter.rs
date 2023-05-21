

/// A structure that represents a data cell filter with column-level, row-level, and/or cell-level security. Data cell filters belong to a specific table in a Data Catalog. During a stack operation,       AWS CloudFormation calls the AWS Lake Formation CreateDataCellsFilter API operation to create     a DataCellsFilter resource, and calls the DeleteDataCellsFilter API operation to delete it.
#[derive(Default, serde::Serialize)]
pub struct CfnDataCellsFilter {


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
    /// A wildcard with exclusions. You must specify either a ColumnNames list or the ColumnWildCard.
    /// 
    /// Required: No
    ///
    /// Type: ColumnWildcard
    ///
    /// Update requires: Replacement
    #[serde(rename = "ColumnWildcard")]
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
    /// An array of UTF-8 strings. A list of column names.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ColumnNames")]
    pub column_names: Option<Vec<String>>,


    /// 
    /// A PartiQL predicate.
    /// 
    /// Required: No
    ///
    /// Type: RowFilter
    ///
    /// Update requires: Replacement
    #[serde(rename = "RowFilter")]
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


/// A PartiQL predicate.
#[derive(Default, serde::Serialize)]
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
    pub filter_expression: Option<String>,

}


/// A wildcard object, consisting of an optional list of excluded column names or indexes.
#[derive(Default, serde::Serialize)]
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
    pub excluded_column_names: Option<Vec<String>>,

}