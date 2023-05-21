

/// The AWS::RDS::DBProxyTargetGroup resource represents a set of RDS DB       instances, Aurora DB clusters, or both that a proxy can connect to. Currently, each       target group is associated with exactly one RDS DB instance or Aurora DB cluster.
///
/// This data type is used as a response element in the         DescribeDBProxyTargetGroups action.
///
/// For information about RDS Proxy for Amazon RDS, see Managing Connections with Amazon         RDS Proxy in the Amazon RDS User Guide.
///
/// For information about RDS Proxy for Amazon Aurora, see Managing Connections with         Amazon RDS Proxy in the Amazon Aurora User Guide.
///
/// For a sample template that creates a DB proxy and registers a DB instance, see                Examples in AWS::RDS::DBProxy.
#[derive(Default, serde::Serialize)]
pub struct CfnDBProxyTargetGroup {


    /// 
    /// One or more DB instance identifiers.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBInstanceIdentifiers")]
    pub dbinstance_identifiers: Option<Vec<String>>,


    /// 
    /// One or more DB cluster identifiers.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBClusterIdentifiers")]
    pub dbcluster_identifiers: Option<Vec<String>>,


    /// 
    /// The identifier for the target group.
    /// 
    /// NoteCurrently, this property must be set to default.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetGroupName")]
    pub target_group_name: String,


    /// 
    /// The identifier of the DBProxy that is associated with the DBProxyTargetGroup.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBProxyName")]
    pub dbproxy_name: String,


    /// 
    /// Settings that control the size and behavior of the connection pool associated with a         DBProxyTargetGroup.
    /// 
    /// Required: No
    ///
    /// Type: ConnectionPoolConfigurationInfoFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionPoolConfigurationInfo")]
    pub connection_pool_configuration_info: Option<ConnectionPoolConfigurationInfoFormat>,

}


/// Specifies the settings that control the size and behavior of the connection pool       associated with a DBProxyTargetGroup.
#[derive(Default, serde::Serialize)]
pub struct ConnectionPoolConfigurationInfoFormat {


    /// 
    /// Controls how actively the proxy closes idle database connections in the connection pool.     The value is expressed as a percentage of the max_connections setting for the RDS DB instance or Aurora DB cluster used by the target group.     With a high value, the proxy leaves a high percentage of idle database connections open. A low value causes the proxy to close more idle connections and return them to the database.
    /// 
    /// If you specify this parameter, then you must also include a value for MaxConnectionsPercent.
    /// 
    /// Default: The default value is half of the value of MaxConnectionsPercent. For example, if MaxConnectionsPercent is 80, then the default value of     MaxIdleConnectionsPercent is 40. If the value of MaxConnectionsPercent isn't specified, then for SQL Server, MaxIdleConnectionsPercent is 5, and     for all other engines, the default is 50.
    /// 
    /// Constraints: Must be between 0 and the value of MaxConnectionsPercent.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxIdleConnectionsPercent")]
    pub max_idle_connections_percent: Option<i64>,


    /// 
    /// Each item in the list represents a class of SQL operations that normally cause all later statements     in a session using a proxy to be pinned to the same underlying database connection. Including an item     in the list exempts that class of SQL operations from the pinning behavior.
    /// 
    /// Default: no session pinning filters
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionPinningFilters")]
    pub session_pinning_filters: Option<Vec<String>>,


    /// 
    /// One or more SQL statements for the proxy to run when opening each new database connection.     Typically used with SET statements to make sure that each connection has identical     settings such as time zone and character set. For multiple statements, use semicolons as the separator.     You can also include multiple variables in a single SET statement, such as     SET x=1, y=2.
    /// 
    /// Default: no initialization query
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InitQuery")]
    pub init_query: Option<String>,


    /// 
    /// The maximum size of the connection pool for each target in a target group. The value is expressed as a percentage of the     max_connections setting for the RDS DB instance or Aurora DB cluster used by the target group.
    /// 
    /// If you specify MaxIdleConnectionsPercent, then you must also include a value for this parameter.
    /// 
    /// Default: 10 for RDS for Microsoft SQL Server, and 100 for all other engines
    /// 
    /// Constraints: Must be between 1 and 100.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConnectionsPercent")]
    pub max_connections_percent: Option<i64>,


    /// 
    /// The number of seconds for a proxy to wait for a connection to become available in the connection pool. Only applies when the     proxy has opened its maximum number of connections and all connections are busy with client sessions.
    /// 
    /// Default: 120
    /// 
    /// Constraints: between 1 and 3600, or 0 representing unlimited
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionBorrowTimeout")]
    pub connection_borrow_timeout: Option<i64>,

}