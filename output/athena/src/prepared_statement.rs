

/// Specifies a prepared statement for use with SQL queries in Athena.
#[derive(Default, serde::Serialize)]
pub struct CfnPreparedStatement {


    /// 
    /// The workgroup to which the prepared statement belongs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkGroup")]
    pub work_group: String,


    /// 
    /// The name of the prepared statement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z_][a-zA-Z0-9_@:]{1,256}
    ///
    /// Update requires: Replacement
    #[serde(rename = "StatementName")]
    pub statement_name: String,


    /// 
    /// The description of the prepared statement.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The query string for the prepared statement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 262144
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStatement")]
    pub query_statement: String,

}
