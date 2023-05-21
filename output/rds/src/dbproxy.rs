

/// The AWS::RDS::DBProxy resource creates or updates a DB proxy.
///
/// For information about RDS Proxy for Amazon RDS, see Managing Connections with Amazon         RDS Proxy in the Amazon RDS User Guide.
///
/// For information about RDS Proxy for Amazon Aurora, see Managing Connections with         Amazon RDS Proxy in the Amazon Aurora User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBProxy {


    /// 
    /// The authorization mechanism that the proxy uses.
    /// 
    /// Required: Yes
    ///
    /// Type: List of AuthFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Auth")]
    pub auth: Vec<AuthFormat>,


    /// 
    /// One or more VPC security group IDs to associate with the new proxy.
    /// 
    /// If you plan to update the resource, don't specify VPC security groups in a shared VPC.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,


    /// 
    /// The number of seconds that a connection to the proxy can be inactive before the proxy disconnects it. You can set this     value higher or lower than the connection timeout limit for the associated database.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdleClientTimeout")]
    pub idle_client_timeout: Option<i64>,


    /// 
    /// The identifier for the proxy. This name must be unique for all proxies owned by your AWS account in the specified AWS Region. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBProxyName")]
    pub dbproxy_name: String,


    /// 
    /// The kinds of databases that the proxy can connect to. This value determines which       database network protocol the proxy recognizes when it interprets network traffic to and       from the database. For Aurora MySQL, RDS for MariaDB, and RDS for MySQL databases, specify MYSQL.       For Aurora PostgreSQL and RDS for PostgreSQL databases, specify POSTGRESQL.       For RDS for Microsoft SQL Server, specify SQLSERVER.
    /// 
    /// Valid values: MYSQL | POSTGRESQL | SQLSERVER
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineFamily")]
    pub engine_family: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that the proxy uses to access secrets in AWS Secrets Manager.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// A Boolean parameter that specifies whether Transport Layer Security (TLS) encryption is required for connections to the proxy.     By enabling this setting, you can enforce encrypted TLS connections to the proxy.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireTLS")]
    pub require_tls: Option<bool>,


    /// 
    /// One or more VPC subnet IDs to associate with the new proxy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcSubnetIds")]
    pub vpc_subnet_ids: Vec<String>,


    /// 
    /// An optional set of key-value pairs to associate arbitrary data of your choosing with the proxy.
    /// 
    /// Required: No
    ///
    /// Type: List of TagFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagFormat>>,


    /// 
    /// Whether the proxy includes detailed information about SQL statements in its logs.     This information helps you to debug issues involving SQL behavior or the performance     and scalability of the proxy connections. The debug information includes the text of     SQL statements that you submit through the proxy. Thus, only enable this setting     when needed for debugging, and only when you have security measures in place to     safeguard any sensitive information that appears in the logs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DebugLogging")]
    pub debug_logging: Option<bool>,

}

impl cfn_resources::CfnResource for CfnDBProxy {
    fn type_string() -> &'static str {
        "AWS::RDS::DBProxy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies the details of authentication used by a proxy to log in as a specific       database user.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuthFormat {


    /// 
    /// A user-specified description about the authentication used by a proxy to log in as a specific database user.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Whether to require or disallow AWS Identity and Access Management (IAM) authentication       for connections to the proxy. The ENABLED value is valid only for proxies with RDS for Microsoft SQL Server.
    /// 
    /// Valid Values: ENABLED | DISABLED | REQUIRED
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IAMAuth")]
    pub iamauth: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) representing the secret that the proxy uses to authenticate     to the RDS DB instance or Aurora DB cluster. These secrets are stored within Amazon Secrets Manager.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,


    /// 
    /// Specifies the details of authentication used by a proxy to log in as a specific database user.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientPasswordAuthType")]
    pub client_password_auth_type: Option<String>,


    /// 
    /// The type of authentication that the proxy uses for connections from the proxy to the       underlying database.
    /// 
    /// Valid Values: SECRETS
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthScheme")]
    pub auth_scheme: Option<String>,

}


/// Metadata assigned to a DB proxy consisting of a key-value pair.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagFormat {


    /// 
    /// A value is the optional value of the tag. The string value can be 1-256 Unicode       characters in length and can't be prefixed with aws:. The string can contain only the       set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex:       "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// A key is the required name of the tag. The string value can be 1-128 Unicode       characters in length and can't be prefixed with aws:. The string can contain only the       set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex:       "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,

}
