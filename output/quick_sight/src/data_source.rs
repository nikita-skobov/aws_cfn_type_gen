/// Creates a data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataSource {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,

    ///
    /// The credentials Amazon QuickSight that uses to connect to your underlying source. Currently, only 			credentials based on user name and password are supported.
    ///
    /// Required: No
    ///
    /// Type: DataSourceCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<DataSourceCredentials>,

    ///
    /// An ID for the data source. This ID is unique per AWS Region for each AWS account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,

    ///
    /// The parameters that Amazon QuickSight uses to connect to your underlying source.
    ///
    /// Required: No
    ///
    /// Type: DataSourceParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_parameters: Option<DataSourceParameters>,

    ///
    /// Error information from the last update or the creation of the data source.
    ///
    /// Required: No
    ///
    /// Type: DataSourceErrorInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<DataSourceErrorInfo>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,

    ///
    /// Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source.
    ///
    /// Required: No
    ///
    /// Type: SslProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_properties: Option<SslProperties>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<DataSourceTypeEnum>,

    ///
    /// Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to 			your underlying source.
    ///
    /// Required: No
    ///
    /// Type: VpcConnectionProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConnectionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DataSourceTypeEnum {
    /// ADOBE_ANALYTICS
    #[serde(rename = "ADOBE_ANALYTICS")]
    Adobeanalytics,

    /// AMAZON_ELASTICSEARCH
    #[serde(rename = "AMAZON_ELASTICSEARCH")]
    Amazonelasticsearch,

    /// AMAZON_OPENSEARCH
    #[serde(rename = "AMAZON_OPENSEARCH")]
    Amazonopensearch,

    /// ATHENA
    #[serde(rename = "ATHENA")]
    Athena,

    /// AURORA
    #[serde(rename = "AURORA")]
    Aurora,

    /// AURORA_POSTGRESQL
    #[serde(rename = "AURORA_POSTGRESQL")]
    Aurorapostgresql,

    /// AWS_IOT_ANALYTICS
    #[serde(rename = "AWS_IOT_ANALYTICS")]
    Awsiotanalytics,

    /// DATABRICKS
    #[serde(rename = "DATABRICKS")]
    Databricks,

    /// EXASOL
    #[serde(rename = "EXASOL")]
    Exasol,

    /// GITHUB
    #[serde(rename = "GITHUB")]
    Github,

    /// JIRA
    #[serde(rename = "JIRA")]
    Jira,

    /// MARIADB
    #[serde(rename = "MARIADB")]
    Mariadb,

    /// MYSQL
    #[serde(rename = "MYSQL")]
    Mysql,

    /// ORACLE
    #[serde(rename = "ORACLE")]
    Oracle,

    /// POSTGRESQL
    #[serde(rename = "POSTGRESQL")]
    Postgresql,

    /// PRESTO
    #[serde(rename = "PRESTO")]
    Presto,

    /// REDSHIFT
    #[serde(rename = "REDSHIFT")]
    Redshift,

    /// S3
    #[serde(rename = "S3")]
    S3,

    /// SALESFORCE
    #[serde(rename = "SALESFORCE")]
    Salesforce,

    /// SERVICENOW
    #[serde(rename = "SERVICENOW")]
    Servicenow,

    /// SNOWFLAKE
    #[serde(rename = "SNOWFLAKE")]
    Snowflake,

    /// SPARK
    #[serde(rename = "SPARK")]
    Spark,

    /// SQLSERVER
    #[serde(rename = "SQLSERVER")]
    Sqlserver,

    /// TERADATA
    #[serde(rename = "TERADATA")]
    Teradata,

    /// TIMESTREAM
    #[serde(rename = "TIMESTREAM")]
    Timestream,

    /// TWITTER
    #[serde(rename = "TWITTER")]
    Twitter,
}

impl Default for DataSourceTypeEnum {
    fn default() -> Self {
        DataSourceTypeEnum::Adobeanalytics
    }
}

impl cfn_resources::CfnResource for CfnDataSource {
    fn type_string(&self) -> &'static str {
        "AWS::QuickSight::DataSource"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.alternate_data_source_parameters {
            if the_val.len() > 50 as _ {
                return Err(format!("Max validation failed on field 'alternate_data_source_parameters'. {} is greater than 50", the_val.len()));
            }
        }

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

        self.credentials
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.data_source_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.error_info
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

        self.ssl_properties
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

        self.vpc_connection_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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

impl cfn_resources::CfnResource for AmazonElasticsearchParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.domain;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'domain'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.domain;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'domain'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for AmazonOpenSearchParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.domain;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'domain'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.domain;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'domain'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

impl cfn_resources::CfnResource for AthenaParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.work_group {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'work_group'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.work_group {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'work_group'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Parameters for Amazon Aurora.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuroraParameters {
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

impl cfn_resources::CfnResource for AuroraParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// Parameters for Amazon Aurora PostgreSQL-Compatible Edition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuroraPostgreSqlParameters {
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
}

impl cfn_resources::CfnResource for AuroraPostgreSqlParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl cfn_resources::CfnResource for CredentialPair {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.alternate_data_source_parameters {
            if the_val.len() > 50 as _ {
                return Err(format!("Max validation failed on field 'alternate_data_source_parameters'. {} is greater than 50", the_val.len()));
            }
        }

        let the_val = &self.password;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'password'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.password;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'password'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.username;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'username'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.username;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'username'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Data source credentials. This is a variant type structure. For this structure to be       valid, only one of the attributes can be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceCredentials {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_arn: Option<String>,

    ///
    /// Credential pair. For more information, see               CredentialPair       .
    ///
    /// Required: No
    ///
    /// Type: CredentialPair
    ///
    /// Update requires: No interruption
    #[serde(rename = "CredentialPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_pair: Option<CredentialPair>,

    ///
    /// The Amazon Resource Name (ARN) of the secret associated with the data source in AWS Secrets Manager.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

impl cfn_resources::CfnResource for DataSourceCredentials {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.credential_pair
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<DataSourceErrorInfoTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DataSourceErrorInfoTypeEnum {
    /// ACCESS_DENIED
    #[serde(rename = "ACCESS_DENIED")]
    Accessdenied,

    /// CONFLICT
    #[serde(rename = "CONFLICT")]
    Conflict,

    /// COPY_SOURCE_NOT_FOUND
    #[serde(rename = "COPY_SOURCE_NOT_FOUND")]
    Copysourcenotfound,

    /// ENGINE_VERSION_NOT_SUPPORTED
    #[serde(rename = "ENGINE_VERSION_NOT_SUPPORTED")]
    Engineversionnotsupported,

    /// GENERIC_SQL_FAILURE
    #[serde(rename = "GENERIC_SQL_FAILURE")]
    Genericsqlfailure,

    /// TIMEOUT
    #[serde(rename = "TIMEOUT")]
    Timeout,

    /// UNKNOWN
    #[serde(rename = "UNKNOWN")]
    Unknown,

    /// UNKNOWN_HOST
    #[serde(rename = "UNKNOWN_HOST")]
    Unknownhost,
}

impl Default for DataSourceErrorInfoTypeEnum {
    fn default() -> Self {
        DataSourceErrorInfoTypeEnum::Accessdenied
    }
}

impl cfn_resources::CfnResource for DataSourceErrorInfo {
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

/// The parameters that Amazon QuickSight uses to connect to your underlying data source.       This is a variant type structure. For this structure to be valid, only one of the       attributes can be non-null.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceParameters {
    ///
    /// The parameters for OpenSearch.
    ///
    /// Required: No
    ///
    /// Type: AmazonElasticsearchParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmazonElasticsearchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_elasticsearch_parameters: Option<AmazonElasticsearchParameters>,

    ///
    /// The parameters for OpenSearch.
    ///
    /// Required: No
    ///
    /// Type: AmazonOpenSearchParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmazonOpenSearchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_open_search_parameters: Option<AmazonOpenSearchParameters>,

    ///
    /// The parameters for Amazon Athena.
    ///
    /// Required: No
    ///
    /// Type: AthenaParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AthenaParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_parameters: Option<AthenaParameters>,

    ///
    /// The parameters for Amazon Aurora MySQL.
    ///
    /// Required: No
    ///
    /// Type: AuroraParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuroraParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aurora_parameters: Option<AuroraParameters>,

    ///
    /// The parameters for Amazon Aurora.
    ///
    /// Required: No
    ///
    /// Type: AuroraPostgreSqlParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuroraPostgreSqlParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aurora_postgre_sql_parameters: Option<AuroraPostgreSqlParameters>,

    ///
    /// The required parameters that are needed to connect to a Databricks data source.
    ///
    /// Required: No
    ///
    /// Type: DatabricksParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabricksParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databricks_parameters: Option<DatabricksParameters>,

    ///
    /// The parameters for MariaDB.
    ///
    /// Required: No
    ///
    /// Type: MariaDbParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MariaDbParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maria_db_parameters: Option<MariaDbParameters>,

    ///
    /// The parameters for MySQL.
    ///
    /// Required: No
    ///
    /// Type: MySqlParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MySqlParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_sql_parameters: Option<MySqlParameters>,

    ///
    /// Oracle parameters.
    ///
    /// Required: No
    ///
    /// Type: OracleParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "OracleParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_parameters: Option<OracleParameters>,

    ///
    /// The parameters for PostgreSQL.
    ///
    /// Required: No
    ///
    /// Type: PostgreSqlParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "PostgreSqlParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_sql_parameters: Option<PostgreSqlParameters>,

    ///
    /// The parameters for Presto.
    ///
    /// Required: No
    ///
    /// Type: PrestoParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrestoParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presto_parameters: Option<PrestoParameters>,

    ///
    /// The parameters for Amazon RDS.
    ///
    /// Required: No
    ///
    /// Type: RdsParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "RdsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_parameters: Option<RdsParameters>,

    ///
    /// The parameters for Amazon Redshift.
    ///
    /// Required: No
    ///
    /// Type: RedshiftParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedshiftParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_parameters: Option<RedshiftParameters>,

    ///
    /// The parameters for S3.
    ///
    /// Required: No
    ///
    /// Type: S3Parameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_parameters: Option<SnowflakeParameters>,

    ///
    /// The parameters for Spark.
    ///
    /// Required: No
    ///
    /// Type: SparkParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SparkParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_parameters: Option<SparkParameters>,

    ///
    /// The parameters for SQL Server.
    ///
    /// Required: No
    ///
    /// Type: SqlServerParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlServerParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_server_parameters: Option<SqlServerParameters>,

    ///
    /// The parameters for Teradata.
    ///
    /// Required: No
    ///
    /// Type: TeradataParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "TeradataParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teradata_parameters: Option<TeradataParameters>,
}

impl cfn_resources::CfnResource for DataSourceParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.amazon_elasticsearch_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.amazon_open_search_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.athena_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.aurora_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.aurora_postgre_sql_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.databricks_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.maria_db_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.my_sql_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.oracle_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.postgre_sql_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.presto_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rds_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.redshift_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.snowflake_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.spark_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sql_server_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.teradata_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The required parameters that are needed to connect to a Databricks data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabricksParameters {
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

impl cfn_resources::CfnResource for DatabricksParameters {
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

/// Amazon S3 manifest file location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ManifestFileLocation {
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
}

impl cfn_resources::CfnResource for ManifestFileLocation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'bucket'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.bucket;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'bucket'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.key;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'key'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.key;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'key'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
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

impl cfn_resources::CfnResource for MariaDbParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// The parameters for MySQL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MySqlParameters {
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

impl cfn_resources::CfnResource for MySqlParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
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
    /// Host.
    ///
    /// Required: Yes
    ///
    /// Type: String
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
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: f64,
}

impl cfn_resources::CfnResource for OracleParameters {
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

/// The parameters for PostgreSQL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PostgreSqlParameters {
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

impl cfn_resources::CfnResource for PostgreSqlParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// The parameters for Presto.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrestoParameters {
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

impl cfn_resources::CfnResource for PrestoParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.catalog;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'catalog'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// The parameters for Amazon RDS.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RdsParameters {
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
}

impl cfn_resources::CfnResource for RdsParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.instance_id;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_id'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.instance_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'instance_id'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The parameters for Amazon Redshift. The ClusterId field can be blank if       Host and Port are both set. The Host and       Port fields can be blank if the ClusterId field is set.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedshiftParameters {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<f64>,
}

impl cfn_resources::CfnResource for RedshiftParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.cluster_id {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'cluster_id'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.cluster_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'cluster_id'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.host {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'host'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.host {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'host'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.port {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'port'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for ResourcePermission {
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

impl cfn_resources::CfnResource for S3Parameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.manifest_file_location.validate()?;

        Ok(())
    }
}

/// The parameters for Snowflake.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SnowflakeParameters {
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

impl cfn_resources::CfnResource for SnowflakeParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.warehouse;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'warehouse'. {} is greater than 128",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The parameters for Spark.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SparkParameters {
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

impl cfn_resources::CfnResource for SparkParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for SqlServerParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_ssl: Option<bool>,
}

impl cfn_resources::CfnResource for SslProperties {
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

impl cfn_resources::CfnResource for TeradataParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.database;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'database'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'database'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'host'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.host;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'host'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for VpcConnectionProperties {
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
