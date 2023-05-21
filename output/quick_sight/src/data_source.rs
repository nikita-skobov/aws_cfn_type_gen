

/// Creates a data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataSource {


    /// 
    /// Error information from the last update or the creation of the data source.
    /// 
    /// Required: No
    ///
    /// Type: DataSourceErrorInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<DataSourceErrorInfo>,


    /// 
    /// An ID for the data source. This ID is unique per AWS Region for each AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSourceId")]
    pub data_source_id: Option<String>,


    /// 
    /// A set of alternate data source parameters that you want to share for the credentials       stored with this data source. The credentials are applied in tandem with the data source       parameters when you copy a data source by using a create or update request. The API       operation compares the DataSourceParameters structure that's in the request       with the structures in the AlternateDataSourceParameters allow list. If the       structures are an exact match, the request is allowed to use the credentials from this       existing data source. If the AlternateDataSourceParameters list is null,       the Credentials originally used with this DataSourceParameters       are automatically allowed.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceParameters
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlternateDataSourceParameters")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,


    /// 
    /// A list of resource permissions on the data source.
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
    /// Contains a map of the key-value pairs for the resource tag or tags assigned to the data source.
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


    /// 
    /// The credentials Amazon QuickSight that uses to connect to your underlying source. Currently, only 			credentials based on user name and password are supported.
    /// 
    /// Required: No
    ///
    /// Type: DataSourceCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    pub credentials: Option<DataSourceCredentials>,


    /// 
    /// The type of the data source. To return a 			list of all data sources, use ListDataSources.
    /// 
    /// Use AMAZON_ELASTICSEARCH for Amazon OpenSearch Service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADOBE_ANALYTICS | AMAZON_ELASTICSEARCH | AMAZON_OPENSEARCH | ATHENA | AURORA | AURORA_POSTGRESQL | AWS_IOT_ANALYTICS | DATABRICKS | EXASOL | GITHUB | JIRA | MARIADB | MYSQL | ORACLE | POSTGRESQL | PRESTO | REDSHIFT | S3 | SALESFORCE | SERVICENOW | SNOWFLAKE | SPARK | SQLSERVER | TERADATA | TIMESTREAM | TWITTER
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// A display name for the data source.
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
    /// The parameters that Amazon QuickSight uses to connect to your underlying source.
    /// 
    /// Required: No
    ///
    /// Type: DataSourceParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceParameters")]
    pub data_source_parameters: Option<DataSourceParameters>,


    /// 
    /// Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source.
    /// 
    /// Required: No
    ///
    /// Type: SslProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslProperties")]
    pub ssl_properties: Option<SslProperties>,


    /// 
    /// Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to 			your underlying source.
    /// 
    /// Required: No
    ///
    /// Type: VpcConnectionProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConnectionProperties")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,

}

impl cfn_resources::CfnResource for CfnDataSource {
    fn type_string() -> &'static str {
        "AWS::QuickSight::DataSource"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The parameters for MariaDB.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MariaDbParameters {


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,

}


/// Oracle parameters.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OracleParameters {


    /// 
    /// Database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,

}


/// The parameters that Amazon QuickSight uses to connect to your underlying data source.       This is a variant type structure. For this structure to be valid, only one of the       attributes can be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceParameters {


    /// 
    /// The parameters for SQL Server.
    /// 
    /// Required: No
    ///
    /// Type: SqlServerParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlServerParameters")]
    pub sql_server_parameters: Option<SqlServerParameters>,


    /// 
    /// The parameters for MariaDB.
    /// 
    /// Required: No
    ///
    /// Type: MariaDbParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MariaDbParameters")]
    pub maria_db_parameters: Option<MariaDbParameters>,


    /// 
    /// The parameters for Spark.
    /// 
    /// Required: No
    ///
    /// Type: SparkParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SparkParameters")]
    pub spark_parameters: Option<SparkParameters>,


    /// 
    /// The parameters for Amazon Aurora MySQL.
    /// 
    /// Required: No
    ///
    /// Type: AuroraParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuroraParameters")]
    pub aurora_parameters: Option<AuroraParameters>,


    /// 
    /// The required parameters that are needed to connect to a Databricks data source.
    /// 
    /// Required: No
    ///
    /// Type: DatabricksParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabricksParameters")]
    pub databricks_parameters: Option<DatabricksParameters>,


    /// 
    /// The parameters for Amazon RDS.
    /// 
    /// Required: No
    ///
    /// Type: RdsParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "RdsParameters")]
    pub rds_parameters: Option<RdsParameters>,


    /// 
    /// The parameters for Presto.
    /// 
    /// Required: No
    ///
    /// Type: PrestoParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrestoParameters")]
    pub presto_parameters: Option<PrestoParameters>,


    /// 
    /// The parameters for Amazon Athena.
    /// 
    /// Required: No
    ///
    /// Type: AthenaParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AthenaParameters")]
    pub athena_parameters: Option<AthenaParameters>,


    /// 
    /// The parameters for OpenSearch.
    /// 
    /// Required: No
    ///
    /// Type: AmazonOpenSearchParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmazonOpenSearchParameters")]
    pub amazon_open_search_parameters: Option<AmazonOpenSearchParameters>,


    /// 
    /// Oracle parameters.
    /// 
    /// Required: No
    ///
    /// Type: OracleParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "OracleParameters")]
    pub oracle_parameters: Option<OracleParameters>,


    /// 
    /// The parameters for Amazon Aurora.
    /// 
    /// Required: No
    ///
    /// Type: AuroraPostgreSqlParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuroraPostgreSqlParameters")]
    pub aurora_postgre_sql_parameters: Option<AuroraPostgreSqlParameters>,


    /// 
    /// The parameters for S3.
    /// 
    /// Required: No
    ///
    /// Type: S3Parameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Parameters")]
    pub s3_parameters: Option<S3Parameters>,


    /// 
    /// The parameters for Snowflake.
    /// 
    /// Required: No
    ///
    /// Type: SnowflakeParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnowflakeParameters")]
    pub snowflake_parameters: Option<SnowflakeParameters>,


    /// 
    /// The parameters for Amazon Redshift.
    /// 
    /// Required: No
    ///
    /// Type: RedshiftParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedshiftParameters")]
    pub redshift_parameters: Option<RedshiftParameters>,


    /// 
    /// The parameters for Teradata.
    /// 
    /// Required: No
    ///
    /// Type: TeradataParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "TeradataParameters")]
    pub teradata_parameters: Option<TeradataParameters>,


    /// 
    /// The parameters for PostgreSQL.
    /// 
    /// Required: No
    ///
    /// Type: PostgreSqlParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "PostgreSqlParameters")]
    pub postgre_sql_parameters: Option<PostgreSqlParameters>,


    /// 
    /// The parameters for OpenSearch.
    /// 
    /// Required: No
    ///
    /// Type: AmazonElasticsearchParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmazonElasticsearchParameters")]
    pub amazon_elasticsearch_parameters: Option<AmazonElasticsearchParameters>,


    /// 
    /// The parameters for MySQL.
    /// 
    /// Required: No
    ///
    /// Type: MySqlParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MySqlParameters")]
    pub my_sql_parameters: Option<MySqlParameters>,

}


/// The parameters for PostgreSQL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PostgreSqlParameters {


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,

}


/// Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your       underlying data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SslProperties {


    /// 
    /// A Boolean option to control whether SSL should be disabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableSsl")]
    pub disable_ssl: Option<bool>,

}


/// The parameters for OpenSearch.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonElasticsearchParameters {


    /// 
    /// The OpenSearch domain.
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
    #[serde(rename = "Domain")]
    pub domain: String,

}


/// VPC connection properties.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConnectionProperties {


    /// 
    /// The Amazon Resource Name (ARN) for the VPC connection.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConnectionArn")]
    pub vpc_connection_arn: String,

}


/// The parameters for Amazon Redshift. The ClusterId field can be blank if       Host and Port are both set. The Host and       Port fields can be blank if the ClusterId field is set.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedshiftParameters {


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// Host. This field can be blank if ClusterId is provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: Option<String>,


    /// 
    /// Port. This field can be blank if the ClusterId is provided.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<f64>,


    /// 
    /// Cluster ID. This field can be blank if the Host and Port are       provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterId")]
    pub cluster_id: Option<String>,

}


/// The combination of user name and password that are used as credentials.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CredentialPair {


    /// 
    /// A set of alternate data source parameters that you want to share for these       credentials. The credentials are applied in tandem with the data source parameters when       you copy a data source by using a create or update request. The API operation compares       the DataSourceParameters structure that's in the request with the       structures in the AlternateDataSourceParameters allow list. If the       structures are an exact match, the request is allowed to use the new data source with       the existing credentials. If the AlternateDataSourceParameters list is       null, the DataSourceParameters originally used with these         Credentials is automatically allowed.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceParameters
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlternateDataSourceParameters")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,


    /// 
    /// Password.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,


    /// 
    /// User name.
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
    #[serde(rename = "Username")]
    pub username: String,

}


/// The parameters for Snowflake.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SnowflakeParameters {


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// Warehouse.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Warehouse")]
    pub warehouse: String,

}


/// The parameters for Teradata.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TeradataParameters {


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,

}


/// Parameters for Amazon Aurora PostgreSQL-Compatible Edition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuroraPostgreSqlParameters {


    /// 
    /// The Amazon Aurora PostgreSQL-Compatible host to connect to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// The port that Amazon Aurora PostgreSQL is listening on.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// The Amazon Aurora PostgreSQL database to connect to.
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
    #[serde(rename = "Database")]
    pub database: String,

}


/// Error information for the data source creation or update.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceErrorInfo {


    /// 
    /// Error message.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Message")]
    pub message: Option<String>,


    /// 
    /// Error type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACCESS_DENIED | CONFLICT | COPY_SOURCE_NOT_FOUND | ENGINE_VERSION_NOT_SUPPORTED | GENERIC_SQL_FAILURE | TIMEOUT | UNKNOWN | UNKNOWN_HOST
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}


/// Parameters for Amazon Aurora.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuroraParameters {


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,

}


/// Data source credentials. This is a variant type structure. For this structure to be       valid, only one of the attributes can be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceCredentials {


    /// 
    /// Credential pair. For more information, see               CredentialPair       .
    /// 
    /// Required: No
    ///
    /// Type: CredentialPair
    ///
    /// Update requires: No interruption
    #[serde(rename = "CredentialPair")]
    pub credential_pair: Option<CredentialPair>,


    /// 
    /// The Amazon Resource Name (ARN) of a data source that has the credential pair that you       want to use. When CopySourceArn is not null, the credential pair from the       data source in the ARN is used as the credentials for the       DataSourceCredentials structure.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:[-a-z0-9]*:quicksight:[-a-z0-9]*:[0-9]{12}:datasource/.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopySourceArn")]
    pub copy_source_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the secret associated with the data source in AWS Secrets Manager.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,

}


/// The parameters for S3.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Parameters {


    /// 
    /// Location of the Amazon S3 manifest file. This is NULL if the manifest file was       uploaded into Amazon QuickSight.
    /// 
    /// Required: Yes
    ///
    /// Type: ManifestFileLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestFileLocation")]
    pub manifest_file_location: ManifestFileLocation,

}


/// The parameters for SQL Server.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SqlServerParameters {


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,

}


/// Permission for the resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourcePermission {


    /// 
    /// The IAM action to grant or revoke permissions on.
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


/// The parameters for Presto.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrestoParameters {


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// Catalog.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Catalog")]
    pub catalog: String,

}


/// The parameters for MySQL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MySqlParameters {


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// Parameters for Amazon Athena.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AthenaParameters {


    /// 
    /// The workgroup that Amazon Athena uses.
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
    #[serde(rename = "WorkGroup")]
    pub work_group: Option<String>,

}


/// The parameters for Amazon RDS.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RdsParameters {


    /// 
    /// Instance ID.
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
    #[serde(rename = "InstanceId")]
    pub instance_id: String,


    /// 
    /// Database.
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
    #[serde(rename = "Database")]
    pub database: String,

}


/// The parameters for Spark.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SparkParameters {


    /// 
    /// Port.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,

}


/// Amazon S3 manifest file location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ManifestFileLocation {


    /// 
    /// Amazon S3 key that identifies an object.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// Amazon S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,

}


/// The required parameters that are needed to connect to a Databricks data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabricksParameters {


    /// 
    /// The port for the Databricks data source.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,


    /// 
    /// The host name of the Databricks data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// The HTTP path of the Databricks data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlEndpointPath")]
    pub sql_endpoint_path: String,

}


/// The parameters for OpenSearch.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonOpenSearchParameters {


    /// 
    /// The OpenSearch domain.
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
    #[serde(rename = "Domain")]
    pub domain: String,

}
