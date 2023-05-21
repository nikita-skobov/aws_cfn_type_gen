

/// The AWS::Athena::NamedQuery resource specifies an Amazon Athena saved query, where QueryString contains the SQL query statements that       make up the query.
#[derive(Default, serde::Serialize)]
pub struct CfnNamedQuery {


    /// 
    /// The database to which the query belongs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// The query description.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the workgroup that contains the named query.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [a-zA-Z0-9._-]{1,128}
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkGroup")]
    pub work_group: Option<String>,


    /// 
    /// The query name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The SQL statements that make up the query.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 262144
    ///
    /// Update requires: Replacement
    #[serde(rename = "QueryString")]
    pub query_string: String,

}
