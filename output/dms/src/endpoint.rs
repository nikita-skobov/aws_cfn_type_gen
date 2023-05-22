/// The AWS::DMS::Endpoint resource specifies an AWS DMS endpoint.
///
/// Currently, AWS CloudFormation supports all AWS DMS endpoint types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpoint {
    ///
    /// The Amazon Resource Name (ARN) for the certificate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the endpoint database. For a MySQL source or target endpoint, don't specify DatabaseName.       To migrate to a specific database, use this setting and targetDbType.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<cfn_resources::StrVal>,

    ///
    /// Settings in JSON format for the source and target DocumentDB endpoint.       For more information about other available settings, see               Using extra connections attributes with Amazon DocumentDB as a source and               Using Amazon DocumentDB as a target for AWS Database Migration Service       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: DocDbSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_db_settings: Option<DocDbSettings>,

    ///
    /// Settings in JSON format for the target Amazon DynamoDB endpoint.       For information about other available settings, see               Using object mapping to migrate data to DynamoDB       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: DynamoDbSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,

    ///
    /// Settings in JSON format for the target OpenSearch endpoint. For more information       about the available settings, see               Extra connection attributes when using OpenSearch as a target for AWS DMS       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: ElasticsearchSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,

    ///
    /// The database endpoint identifier. Identifiers must begin with a letter and must contain     only ASCII letters, digits, and hyphens. They can't end with a hyphen, or contain two     consecutive hyphens.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The type of endpoint. Valid values are source and target.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: source | target
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointType")]
    pub endpoint_type: EndpointEndpointTypeEnum,

    ///
    /// The type of engine for the endpoint, depending on the EndpointType value.
    ///
    /// Valid values: mysql | oracle |         postgres | mariadb | aurora |         aurora-postgresql | opensearch | redshift | s3 |         db2 | azuredb | sybase | dynamodb | mongodb |         kinesis | kafka | elasticsearch | docdb |         sqlserver | neptune
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineName")]
    pub engine_name: cfn_resources::StrVal,

    ///
    /// Additional attributes associated with the connection. Each attribute is specified as a       name-value pair associated by an equal sign (=). Multiple attributes are separated by a       semicolon (;) with no additional white space. For information on the attributes available       for connecting your source or target endpoint, see                Working with AWS DMS Endpoints       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<cfn_resources::StrVal>,

    ///
    /// Settings in JSON format for the source GCP MySQL endpoint. These settings are much the same as       the settings for any MySQL-compatible endpoint. For more information, see               Extra connection attributes when using MySQL as a source for AWS DMS       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: GcpMySQLSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "GcpMySQLSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_my_sqlsettings: Option<GcpMySQLSettings>,

    ///
    /// Settings in JSON format for the source IBM Db2 LUW endpoint. For information about other available settings, see              Extra connection attributes when using Db2 LUW as a source for AWS DMS      in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: IbmDb2Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "IbmDb2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibm_db2_settings: Option<IbmDb2Settings>,

    ///
    /// Settings in JSON format for the target Apache Kafka endpoint.       For more information about other available settings, see               Using object mapping to migrate data to a Kafka topic       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: KafkaSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KafkaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_settings: Option<KafkaSettings>,

    ///
    /// Settings in JSON format for the target endpoint for Amazon Kinesis Data Streams.       For more information about other available settings, see                Using object mapping to migrate data to a Kinesis data stream       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: KinesisSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,

    ///
    /// An AWS KMS key identifier that is used to encrypt the connection parameters for the endpoint.
    ///
    /// If you don't specify a value for the KmsKeyId parameter, AWS DMS uses your default encryption key.
    ///
    /// AWS KMS creates the default encryption key for your AWS account.       Your AWS account has a different default encryption key for each AWS Region.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Settings in JSON format for the source and target Microsoft SQL Server endpoint.       For information about other available settings, see               Extra connection attributes when using SQL Server as a source for AWS DMS and                Extra connection attributes when using SQL Server as a target for AWS DMS       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: MicrosoftSqlServerSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MicrosoftSqlServerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_sql_server_settings: Option<MicrosoftSqlServerSettings>,

    ///
    /// Settings in JSON format for the source MongoDB endpoint.       For more information about the available settings, see                 Using MongoDB as a target for AWS Database Migration Service       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: MongoDbSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MongoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,

    ///
    /// Settings in JSON format for the source and target MySQL endpoint. For information about other available settings, see                Extra connection attributes when using MySQL as a source for AWS DMS and                Extra connection attributes when using a MySQL-compatible database as a target for AWS DMS       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: MySqlSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MySqlSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_sql_settings: Option<MySqlSettings>,

    ///
    /// Settings in JSON format for the target Amazon Neptune endpoint.       For more information about the available settings, see                Specifying endpoint settings for Amazon Neptune as a target        in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: NeptuneSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "NeptuneSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neptune_settings: Option<NeptuneSettings>,

    ///
    /// Settings in JSON format for the source and target Oracle endpoint. For information about other available settings, see                Extra connection attributes when using Oracle as a source for AWS DMS and                Extra connection attributes when using Oracle as a target for AWS DMS       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: OracleSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "OracleSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_settings: Option<OracleSettings>,

    ///
    /// The password to be used to log in to the endpoint database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<cfn_resources::StrVal>,

    ///
    /// The port used by the endpoint database.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// Settings in JSON format for the source and target PostgreSQL endpoint.
    ///
    /// For information about other available settings, see                Extra connection attributes when using PostgreSQL as a source for AWS DMS and               Extra connection attributes when using PostgreSQL as a target for AWS DMS       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: PostgreSqlSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "PostgreSqlSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_sql_settings: Option<PostgreSqlSettings>,

    ///
    /// Settings in JSON format for the target Redis endpoint.       For information about other available settings, see               Specifying endpoint settings for Redis as a target       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: RedisSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_settings: Option<RedisSettings>,

    ///
    /// Settings in JSON format for the Amazon Redshift endpoint.
    ///
    /// For more information about other available settings, see               Extra connection attributes when using Amazon Redshift as a target for AWS DMS       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: RedshiftSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedshiftSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,

    ///
    /// A display name for the resource identifier at the end of the EndpointArn       response parameter that is returned in the created Endpoint object. The       value for this parameter can have up to 31 characters. It can contain only ASCII       letters, digits, and hyphen ('-'). Also, it can't end with a hyphen or contain two       consecutive hyphens, and can only begin with a letter, such as         Example-App-ARN1.
    ///
    /// For example, this value might result in the EndpointArn value         arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1. If you don't       specify a ResourceIdentifier value, AWS DMS generates a default       identifier value for the end of EndpointArn.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<cfn_resources::StrVal>,

    ///
    /// Settings in JSON format for the source and target Amazon S3 endpoint. For more information about other available settings, see               Extra connection attributes when using Amazon S3 as a source for AWS DMS and               Extra connection attributes when using Amazon S3 as a target for AWS DMS       in theAWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: S3Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,

    ///
    /// The name of the server where the endpoint database resides.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<cfn_resources::StrVal>,

    ///
    /// The Secure Sockets Layer (SSL) mode to use for the SSL connection. The default is none.
    ///
    /// NoteWhen engine_name is set to S3, the only allowed value is none.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: none | require | verify-ca | verify-full
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<EndpointSslModeEnum>,

    ///
    /// Settings in JSON format for the source and target SAP ASE endpoint. For information about other available settings, see                Extra connection attributes when using SAP ASE as a source for AWS DMS and                Extra connection attributes when using SAP ASE as a target for AWS DMS       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: SybaseSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SybaseSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sybase_settings: Option<SybaseSettings>,

    ///
    /// One or more tags to be assigned to the endpoint.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The user name to be used to log in to the endpoint database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_external_id: CfnEndpointexternalid,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EndpointEndpointTypeEnum {
    /// source
    #[serde(rename = "source")]
    Source,

    /// target
    #[serde(rename = "target")]
    Target,
}

impl Default for EndpointEndpointTypeEnum {
    fn default() -> Self {
        EndpointEndpointTypeEnum::Source
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EndpointSslModeEnum {
    /// none
    #[serde(rename = "none")]
    None,

    /// require
    #[serde(rename = "require")]
    Require,

    /// verify-ca
    #[serde(rename = "verify-ca")]
    Verifyca,

    /// verify-full
    #[serde(rename = "verify-full")]
    Verifyfull,
}

impl Default for EndpointSslModeEnum {
    fn default() -> Self {
        EndpointSslModeEnum::None
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointexternalid;
impl CfnEndpointexternalid {
    pub fn att_name(&self) -> &'static str {
        r#"ExternalId"#
    }
}

impl cfn_resources::CfnResource for CfnEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::DMS::Endpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.doc_db_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dynamo_db_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.elasticsearch_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.gcp_my_sqlsettings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ibm_db2_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kafka_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kinesis_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.microsoft_sql_server_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.mongo_db_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.my_sql_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.neptune_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.oracle_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.postgre_sql_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.redis_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.redshift_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sybase_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Provides information that defines a DocumentDB endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about other available settings, see               Using extra connections attributes with Amazon DocumentDB as a source and               Using Amazon DocumentDB as a target for AWS Database Migration Service       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocDbSettings {
    ///
    /// Indicates the number of documents to preview to determine the document organization.     Use this setting when NestingLevel is set to "one".
    ///
    /// Must be a positive value greater than 0. Default value is     1000.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocsToInvestigate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_to_investigate: Option<i64>,

    ///
    /// Specifies the document ID. Use this setting when NestingLevel is set to     "none".
    ///
    /// Default value is "false".
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExtractDocId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_doc_id: Option<bool>,

    ///
    /// Specifies either document or table mode.
    ///
    /// Default value is "none". Specify "none" to use document mode.     Specify "one" to use table mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: none | one
    ///
    /// Update requires: No interruption
    #[serde(rename = "NestingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<DocDbSettingsNestingLevelEnum>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager secret       that allows access to the DocumentDB endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,         ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                   Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret that contains the DocumentDB endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DocDbSettingsNestingLevelEnum {
    /// none
    #[serde(rename = "none")]
    None,

    /// one
    #[serde(rename = "one")]
    One,
}

impl Default for DocDbSettingsNestingLevelEnum {
    fn default() -> Self {
        DocDbSettingsNestingLevelEnum::None
    }
}

impl cfn_resources::CfnResource for DocDbSettings {
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

/// Provides information, including the Amazon Resource Name (ARN) of the IAM        role used to define an Amazon DynamoDB target endpoint. This       information also includes the output format of records applied to the endpoint and details of       transaction and control table data information. For information about other available settings, see                Using object mapping to migrate data to DynamoDB       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynamoDbSettings {
    ///
    /// The Amazon Resource Name (ARN) used by the service to access the IAM role. The role must allow the iam:PassRole action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DynamoDbSettings {
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

/// Provides information that defines an OpenSearch endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about the available settings, see               Extra connection attributes when using OpenSearch as a target for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticsearchSettings {
    ///
    /// The endpoint for the OpenSearch cluster. AWS DMS uses HTTPS if a transport      protocol (either HTTP or HTTPS) isn't specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<cfn_resources::StrVal>,

    ///
    /// The maximum number of seconds for which DMS retries failed API requests to the     OpenSearch cluster.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorRetryDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_retry_duration: Option<i64>,

    ///
    /// The maximum percentage of records that can fail to be written before a full load       operation stops.
    ///
    /// To avoid early failure, this counter is only effective after 1,000 records      are transferred. OpenSearch also has the concept of error monitoring during the      last 10 minutes of an Observation Window. If transfer of all records fail in the      last 10 minutes, the full load operation stops.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FullLoadErrorPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_error_percentage: Option<i64>,

    ///
    /// The Amazon Resource Name (ARN) used by the service to access the IAM role.     The role must allow the iam:PassRole action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ElasticsearchSettings {
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

/// Provides information that defines a GCP MySQL endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. These settings are much the same as       the settings for any MySQL-compatible endpoint. For more information, see               Extra connection attributes when using MySQL as a source for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GcpMySQLSettings {
    ///
    /// Specifies a script to run immediately after AWS DMS connects to the endpoint.      The migration task continues running regardless if the SQL statement succeeds or fails.
    ///
    /// For this parameter, provide the code of the script itself, not the name of a file containing the script.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AfterConnectScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<cfn_resources::StrVal>,

    ///
    /// Adjusts the behavior of AWS DMS when migrating from an SQL Server source database      that is hosted as part of an Always On availability group cluster. If you need AWS DMS     to poll all the nodes in the Always On cluster for transaction backups, set this attribute to false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CleanSourceMetadataOnMismatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_source_metadata_on_mismatch: Option<bool>,

    ///
    /// Database name for the endpoint. For a MySQL source or target endpoint, don't explicitly specify      the database using the DatabaseName request parameter on either the CreateEndpoint     or ModifyEndpoint API call. Specifying DatabaseName when you create or modify a      MySQL endpoint replicates all the task tables to this single database. For MySQL endpoints, you specify      the database only when you specify the schema in the table-mapping rules of the AWS DMS task.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies how often to check the binary log for new changes/events when the database is idle. The default is five seconds.
    ///
    /// Example: eventsPollInterval=5;
    ///
    /// In the example, AWS DMS checks for changes in the binary logs every five seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventsPollInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_poll_interval: Option<i64>,

    ///
    /// Specifies the maximum size (in KB) of any .csv file used to transfer data to a MySQL-compatible database.
    ///
    /// Example: maxFileSize=512
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,

    ///
    /// Improves performance when loading data into the MySQL-compatible target database. Specifies how many      threads to use to load the data into the MySQL-compatible target database. Setting a large number of      threads can have an adverse effect on database performance, because a separate connection is required      for each thread. The default is one.
    ///
    /// Example: parallelLoadThreads=1
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelLoadThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_load_threads: Option<i64>,

    ///
    /// Endpoint connection password.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<cfn_resources::StrVal>,

    ///
    /// The port used by the endpoint database.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS       as the trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager secret       that allows access to the MySQL endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify         the values for this setting and SecretsManagerSecretId. Or you can specify clear-text         values for UserName, Password, ServerName, and Port.         You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId required to         access it, see                    Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret     that contains the MySQL endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// Endpoint TCP port.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the time zone for the source MySQL database. Don't enclose time zones in single quotation marks.
    ///
    /// Example: serverTimezone=US/Pacific;
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timezone: Option<cfn_resources::StrVal>,

    ///
    /// Endpoint connection user name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for GcpMySQLSettings {
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

/// Provides information that defines an IBMDB2 endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about other available settings, see               Extra connection attributes when using Db2 LUW as a source for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IbmDb2Settings {
    ///
    /// For ongoing replication (CDC), use CurrentLSN to specify a     log sequence number (LSN) where you want the replication     to start.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrentLsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_lsn: Option<cfn_resources::StrVal>,

    ///
    /// Maximum number of bytes per read, as a NUMBER value.     The default is 64 KB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxKBytesPerRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_kbytes_per_read: Option<i64>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value ofthe AWS Secrets Manager secret       that allows access to the Db2 LUW endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,         ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                    Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret that contains the IBMDB2 endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// Enables ongoing replication (CDC) as a BOOLEAN value. The     default is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SetDataCaptureChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_data_capture_changes: Option<bool>,
}

impl cfn_resources::CfnResource for IbmDb2Settings {
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

/// Provides information that describes an Apache Kafka endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about other available settings, see               Using object mapping to migrate data to a Kafka topic       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KafkaSettings {
    ///
    /// A comma-separated list of one or more broker locations in your Kafka cluster that host your Kafka instance. Specify each broker location       in the form broker-hostname-or-ip:port.       For example, "ec2-12-345-678-901.compute-1.amazonaws.com:2345".       For more information and examples of specifying a list of broker locations, see                Using Apache Kafka as a target for AWS Database Migration Service       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Broker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker: Option<cfn_resources::StrVal>,

    ///
    /// Shows detailed control information for table definition, column definition, and table       and column changes in the Kafka message output. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeControlDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_control_details: Option<bool>,

    ///
    /// Include NULL and empty columns for records migrated to the endpoint. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeNullAndEmpty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_null_and_empty: Option<bool>,

    ///
    /// Shows the partition value within the Kafka message output unless the partition type is       schema-table-type. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludePartitionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_partition_value: Option<bool>,

    ///
    /// Includes any data definition language (DDL) operations that change the table in the       control data, such as rename-table, drop-table,       add-column, drop-column, and rename-column. The       default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeTableAlterOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_table_alter_operations: Option<bool>,

    ///
    /// Provides detailed transaction information from the source database. This information       includes a commit timestamp, a log position, and values for transaction_id,       previous transaction_id, and transaction_record_id (the record       offset within a transaction). The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeTransactionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transaction_details: Option<bool>,

    ///
    /// The output format for the records created on the endpoint. The message format is       JSON (default) or JSON_UNFORMATTED (a single line with no     tab).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: json | json-unformatted
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<KafkaSettingsMessageFormatEnum>,

    ///
    /// The maximum size in bytes for records created on the endpoint The default is 1,000,000.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageMaxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_max_bytes: Option<i64>,

    ///
    /// Set this optional parameter to true to avoid adding a '0x' prefix       to raw data in hexadecimal format. For example, by default, AWS DMS adds a '0x'       prefix to the LOB column type in hexadecimal format moving from an Oracle source to a Kafka       target. Use the NoHexPrefix endpoint setting to enable migration of RAW data       type columns without adding the '0x' prefix.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoHexPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_hex_prefix: Option<bool>,

    ///
    /// Prefixes schema and table names to partition values, when the partition type is       primary-key-type. Doing this increases data distribution among Kafka       partitions. For example, suppose that a SysBench schema has thousands of tables and each       table has only limited range for a primary key. In this case, the same primary key is sent       from thousands of tables to the same partition, which causes throttling. The default is       false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionIncludeSchemaTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_include_schema_table: Option<bool>,

    ///
    /// The secure password that you created when you first set up your Amazon MSK cluster to validate a client identity and       make an encrypted connection between server and client using SASL-SSL authentication.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SaslPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_password: Option<cfn_resources::StrVal>,

    ///
    /// The secure user name you created when you first set up your Amazon MSK cluster to validate a       client identity and make an encrypted connection between server and client using SASL-SSL       authentication.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SaslUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_user_name: Option<cfn_resources::StrVal>,

    ///
    /// Set secure connection to a Kafka target endpoint using Transport Layer Security (TLS). Options include       ssl-encryption, ssl-authentication, and sasl-ssl.       sasl-ssl requires SaslUsername and SaslPassword.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: plaintext | sasl-ssl | ssl-authentication | ssl-encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_protocol: Option<KafkaSettingsSecurityProtocolEnum>,

    ///
    /// The Amazon Resource Name (ARN) for the private certificate authority (CA) cert that AWS DMS uses       to securely connect to your Kafka target endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslCaCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ca_certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the client certificate used to securely connect to a Kafka target endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslClientCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_client_certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) for the client private key used to securely connect to a Kafka target endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslClientKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_client_key_arn: Option<cfn_resources::StrVal>,

    ///
    /// The password for the client private key used to securely connect to a Kafka target endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslClientKeyPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_client_key_password: Option<cfn_resources::StrVal>,

    ///
    /// The topic to which you migrate the data. If you don't specify a topic, AWS DMS       specifies "kafka-default-topic" as the migration topic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum KafkaSettingsMessageFormatEnum {
    /// json
    #[serde(rename = "json")]
    Json,

    /// json-unformatted
    #[serde(rename = "json-unformatted")]
    Jsonunformatted,
}

impl Default for KafkaSettingsMessageFormatEnum {
    fn default() -> Self {
        KafkaSettingsMessageFormatEnum::Json
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum KafkaSettingsSecurityProtocolEnum {
    /// plaintext
    #[serde(rename = "plaintext")]
    Plaintext,

    /// sasl-ssl
    #[serde(rename = "sasl-ssl")]
    Saslssl,

    /// ssl-authentication
    #[serde(rename = "ssl-authentication")]
    Sslauthentication,

    /// ssl-encryption
    #[serde(rename = "ssl-encryption")]
    Sslencryption,
}

impl Default for KafkaSettingsSecurityProtocolEnum {
    fn default() -> Self {
        KafkaSettingsSecurityProtocolEnum::Plaintext
    }
}

impl cfn_resources::CfnResource for KafkaSettings {
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

/// Provides information that describes an Amazon Kinesis Data Stream endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about other available settings, see                Using object mapping to migrate data to a Kinesis data stream       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisSettings {
    ///
    /// Shows detailed control information for table definition, column definition, and table       and column changes in the Kinesis message output. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeControlDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_control_details: Option<bool>,

    ///
    /// Include NULL and empty columns for records migrated to the endpoint. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeNullAndEmpty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_null_and_empty: Option<bool>,

    ///
    /// Shows the partition value within the Kinesis message output, unless the partition type     is schema-table-type. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludePartitionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_partition_value: Option<bool>,

    ///
    /// Includes any data definition language (DDL) operations that change the table in the       control data, such as rename-table, drop-table,       add-column, drop-column, and rename-column. The       default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeTableAlterOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_table_alter_operations: Option<bool>,

    ///
    /// Provides detailed transaction information from the source database. This information       includes a commit timestamp, a log position, and values for transaction_id,       previous transaction_id, and transaction_record_id (the record       offset within a transaction). The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeTransactionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transaction_details: Option<bool>,

    ///
    /// The output format for the records created on the endpoint. The message format is       JSON (default) or JSON_UNFORMATTED (a single line with no tab).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: json | json-unformatted
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<KinesisSettingsMessageFormatEnum>,

    ///
    /// Set this optional parameter to true to avoid adding a '0x' prefix       to raw data in hexadecimal format. For example, by default, AWS DMS adds a '0x'       prefix to the LOB column type in hexadecimal format moving from an Oracle source to an       Amazon Kinesis target. Use the NoHexPrefix endpoint setting to enable       migration of RAW data type columns without adding the '0x' prefix.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoHexPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_hex_prefix: Option<bool>,

    ///
    /// Prefixes schema and table names to partition values, when the partition type is       primary-key-type. Doing this increases data distribution among Kinesis       shards. For example, suppose that a SysBench schema has thousands of tables and each table       has only limited range for a primary key. In this case, the same primary key is sent from       thousands of tables to the same shard, which causes throttling. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionIncludeSchemaTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_include_schema_table: Option<bool>,

    ///
    /// The Amazon Resource Name (ARN) for the IAM role       that AWS DMS uses to write to the Kinesis data stream.       The role must allow the iam:PassRole action.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) for the Amazon Kinesis Data Streams endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum KinesisSettingsMessageFormatEnum {
    /// json
    #[serde(rename = "json")]
    Json,

    /// json-unformatted
    #[serde(rename = "json-unformatted")]
    Jsonunformatted,
}

impl Default for KinesisSettingsMessageFormatEnum {
    fn default() -> Self {
        KinesisSettingsMessageFormatEnum::Json
    }
}

impl cfn_resources::CfnResource for KinesisSettings {
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

/// Provides information that defines a Microsoft SQL Server endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For information about other available settings, see               Extra connection attributes when using SQL Server as a source for AWS DMS and                Extra connection attributes when using SQL Server as a target for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MicrosoftSqlServerSettings {
    ///
    /// The maximum size of the packets (in bytes) used to transfer     data using BCP.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BcpPacketSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcp_packet_size: Option<i64>,

    ///
    /// Specifies a file group for the AWS DMS internal tables. When the replication task     starts, all the internal AWS DMS control tables (awsdms_ apply_exception, awsdms_apply,     awsdms_changes) are created for the specified file group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlTablesFileGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_tables_file_group: Option<cfn_resources::StrVal>,

    ///
    /// Cleans and recreates table metadata information on the replication instance when      a mismatch occurs. An example is a situation where running an alter DDL statement on      a table might result in different information about the table cached in the replication      instance.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "QuerySingleAlwaysOnNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_single_always_on_node: Option<bool>,

    ///
    /// When this attribute is set to Y, AWS DMS only reads changes     from transaction log backups and doesn't read from the     active transaction log file during ongoing replication. Setting     this parameter to Y enables you to control active transaction     log file growth during full load and ongoing replication     tasks. However, it can add some source latency to ongoing     replication.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadBackupOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_backup_only: Option<bool>,

    ///
    /// Use this attribute to minimize the need to access the     backup log and enable AWS DMS to prevent truncation using one of the     following two methods.
    ///
    /// Start transactions in the database: This is the default     method. When this method is used, AWS DMS prevents     TLOG truncation by mimicking a transaction in the database.     As long as such a transaction is open, changes that appear     after the transaction started aren't truncated. If you need     Microsoft Replication to be enabled in your database, then     you must choose this method.
    ///
    /// Exclusively use sp_repldone within a single task: When     this method is used, AWS DMS reads the changes and then     uses sp_repldone to mark the TLOG transactions as ready     for truncation. Although this method doesn't involve any     transactional activities, it can only be used when Microsoft     Replication isn't running. Also, when using this method, only     one AWS DMS task can access the database at any given     time. Therefore, if you need to run parallel AWS DMS tasks     against the same database, use the default method.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SafeguardPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safeguard_policy: Option<cfn_resources::StrVal>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager       secret that allows access to the SQL Server endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,         ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                    Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret that contains the MicrosoftSQLServer endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// Use this to attribute to transfer data for full-load operations     using BCP. When the target table contains an identity     column that does not exist in the source table, you must     disable the use BCP for loading table option.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseBcpFullLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_bcp_full_load: Option<bool>,

    ///
    /// When this attribute is set to Y, DMS processes third-party      transaction log backups if they are created in native format.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseThirdPartyBackupDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_third_party_backup_device: Option<bool>,
}

impl cfn_resources::CfnResource for MicrosoftSqlServerSettings {
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

/// Provides information that defines a MongoDB endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about other available settings, see                Endpoint configuration settings when using MongoDB as a source for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MongoDbSettings {
    ///
    /// The authentication mechanism you use to access the MongoDB source endpoint.
    ///
    /// For the default value, in MongoDB version 2.x, "default" is       "mongodb_cr". For MongoDB version 3.x or later, "default" is       "scram_sha_1". This setting isn't used when AuthType is set to "no".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: default | mongodb_cr | scram_sha_1
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthMechanism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mechanism: Option<MongoDbSettingsAuthMechanismEnum>,

    ///
    /// The MongoDB database name. This setting isn't used when AuthType is set to "no".
    ///
    /// The default is "admin".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source: Option<cfn_resources::StrVal>,

    ///
    /// The authentication type you use to access the MongoDB source endpoint.
    ///
    /// When set to "no", user name and password parameters are not used and can be empty.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: no | password
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<MongoDbSettingsAuthTypeEnum>,

    ///
    /// The database name on the MongoDB source endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<cfn_resources::StrVal>,

    ///
    /// Indicates the number of documents to preview to determine the document organization.       Use this setting when NestingLevel is set to "one".
    ///
    /// Must be a positive value greater than 0. Default value is 1000.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocsToInvestigate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_to_investigate: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the document ID. Use this setting when NestingLevel is set to "none".
    ///
    /// Default value is "false".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExtractDocId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_doc_id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies either document or table mode.
    ///
    /// Default value is "none". Specify "none" to use document mode.       Specify "one" to use table mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: none | one
    ///
    /// Update requires: No interruption
    #[serde(rename = "NestingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<MongoDbSettingsNestingLevelEnum>,

    ///
    /// The password for the user account you use to access the MongoDB source endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<cfn_resources::StrVal>,

    ///
    /// The port value for the MongoDB source endpoint.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager secret       that allows access to the MongoDB endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,         ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                    Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret that contains the MongoDB endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the server on the MongoDB source endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<cfn_resources::StrVal>,

    ///
    /// The user name you use to access the MongoDB source endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MongoDbSettingsAuthMechanismEnum {
    /// default
    #[serde(rename = "default")]
    Default,

    /// mongodb_cr
    #[serde(rename = "mongodb_cr")]
    Mongodbcr,

    /// scram_sha_1
    #[serde(rename = "scram_sha_1")]
    Scramsha1,
}

impl Default for MongoDbSettingsAuthMechanismEnum {
    fn default() -> Self {
        MongoDbSettingsAuthMechanismEnum::Default
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MongoDbSettingsAuthTypeEnum {
    /// no
    #[serde(rename = "no")]
    No,

    /// password
    #[serde(rename = "password")]
    Password,
}

impl Default for MongoDbSettingsAuthTypeEnum {
    fn default() -> Self {
        MongoDbSettingsAuthTypeEnum::No
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MongoDbSettingsNestingLevelEnum {
    /// none
    #[serde(rename = "none")]
    None,

    /// one
    #[serde(rename = "one")]
    One,
}

impl Default for MongoDbSettingsNestingLevelEnum {
    fn default() -> Self {
        MongoDbSettingsNestingLevelEnum::None
    }
}

impl cfn_resources::CfnResource for MongoDbSettings {
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

/// Provides information that defines a MySQL endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For information about other available settings, see               Extra connection attributes when using MySQL as a source for AWS DMS and               Extra connection attributes when using a MySQL-compatible database as a target for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MySqlSettings {
    ///
    /// Specifies a script to run immediately after AWS DMS     connects to the endpoint. The migration task continues     running regardless if the SQL statement succeeds or fails.
    ///
    /// For this parameter, provide the code of the script itself, not the name of a file     containing the script.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AfterConnectScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<cfn_resources::StrVal>,

    ///
    /// Cleans and recreates table metadata information on the replication instance      when a mismatch occurs. For example, in a situation where running an alter DDL      on the table could result in different information about the table cached in the      replication instance.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CleanSourceMetadataOnMismatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_source_metadata_on_mismatch: Option<bool>,

    ///
    /// Specifies how often to check the binary log for new     changes/events when the database is idle. The default is five seconds.
    ///
    /// Example: eventsPollInterval=5;
    ///
    /// In the example, AWS DMS checks for changes in the binary     logs every five seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventsPollInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_poll_interval: Option<i64>,

    ///
    /// Specifies the maximum size (in KB) of any .csv file used to     transfer data to a MySQL-compatible database.
    ///
    /// Example: maxFileSize=512
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,

    ///
    /// Improves performance when loading data into the MySQL-compatible target database.     Specifies how many threads to use to load the data into the MySQL-compatible target     database. Setting a large number of threads can have an adverse effect on database     performance, because a separate connection is required for each thread. The default is one.
    ///
    /// Example: parallelLoadThreads=1
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelLoadThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_load_threads: Option<i64>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager secret       that allows access to the MySQL endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,        ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                    Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret       that contains the MySQL endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the time zone for the source MySQL database.
    ///
    /// Example: serverTimezone=US/Pacific;
    ///
    /// Note: Do not enclose time zones in single quotes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timezone: Option<cfn_resources::StrVal>,

    ///
    /// Specifies where to migrate source tables on the target, either     to a single database or multiple databases. If you specify    SPECIFIC_DATABASE, specify the database name using the DatabaseName    parameter of the Endpoint object.
    ///
    /// Example: targetDbType=MULTIPLE_DATABASES
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetDbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_db_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MySqlSettings {
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

/// Provides information that defines an Amazon Neptune endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about the available settings, see               Specifying endpoint settings for Amazon Neptune as a target       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NeptuneSettings {
    ///
    /// The number of milliseconds for AWS DMS to wait to retry a bulk-load of migrated graph     data to the Neptune target database before raising an error. The default is 250.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorRetryDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_retry_duration: Option<i64>,

    ///
    /// If you want IAM authorization enabled for this     endpoint, set this parameter to true. Then attach the appropriate IAM policy     document to your service role specified by ServiceAccessRoleArn. The default     is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamAuthEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_auth_enabled: Option<bool>,

    ///
    /// The maximum size in kilobytes of migrated graph data stored in a .csv file before AWS DMS     bulk-loads the data to the Neptune target database. The default is 1,048,576 KB. If the     bulk load is successful, AWS DMS clears the bucket, ready to store the next batch of     migrated graph data.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,

    ///
    /// The number of times for AWS DMS to retry a bulk load of migrated graph data to the     Neptune target database before raising an error. The default is 5.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retry_count: Option<i64>,

    ///
    /// A folder path where you want AWS DMS to store migrated graph data in the S3 bucket     specified by S3BucketName
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_folder: Option<cfn_resources::StrVal>,

    ///
    /// The name of the Amazon S3 bucket where AWS DMS can temporarily store migrated graph data     in .csv files before bulk-loading it to the Neptune target database. AWS DMS maps the SQL     source data to graph data before storing it in these .csv files.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the service role that you created for the Neptune     target endpoint. The role must allow the iam:PassRole action.
    ///
    /// For more information, see                Creating an IAM Service Role for Accessing Amazon Neptune as a Target       in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NeptuneSettings {
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

/// Provides information that defines an Oracle endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For information about other available settings, see                Extra connection attributes when using Oracle as a source for AWS DMS and                Extra connection attributes when using Oracle as a target for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OracleSettings {
    ///
    /// Set this attribute to false in order to use the Binary Reader     to capture change data for an Amazon RDS for Oracle as the     source. This tells the DMS instance to not access redo logs     through any specified path prefix replacement using direct     file access.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessAlternateDirectly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_alternate_directly: Option<bool>,

    ///
    /// Set this attribute to set up table-level supplemental logging     for the Oracle database. This attribute enables PRIMARY KEY     supplemental logging on all tables selected for a migration     task.
    ///
    /// If you use this option, you still need to enable     database-level supplemental logging.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddSupplementalLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_supplemental_logging: Option<bool>,

    ///
    /// Set this attribute with ArchivedLogDestId in a primary/     standby setup. This attribute is useful in the case of a     switchover. In this case, AWS DMS needs to know which     destination to get archive redo logs from to read changes.     This need arises because the previous primary instance is     now a standby instance after switchover.
    ///
    /// Although AWS DMS supports the use of the Oracle     RESETLOGS option to open the database, never     use RESETLOGS unless necessary. For additional     information about RESETLOGS, see RMAN Data Repair Concepts in the     Oracle Database Backup and Recovery User's Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalArchivedLogDestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_archived_log_dest_id: Option<i64>,

    ///
    /// Set this attribute to true to enable replication of Oracle     tables containing columns that are nested tables or defined     types.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowSelectNestedTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_select_nested_tables: Option<bool>,

    ///
    /// Specifies the ID of the destination for the archived redo logs. This value     should be the same as a number in the dest_id column of the v$archived_log     view. If you work with an additional redo log destination, use the     AdditionalArchivedLogDestId option to specify the additional     destination ID. Doing this improves performance by ensuring that the correct     logs are accessed from the outset.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArchivedLogDestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_log_dest_id: Option<i64>,

    ///
    /// When this field is set to Y, AWS DMS only accesses the     archived redo logs. If the archived redo logs are stored on     Automatic Storage Management (ASM) only, the AWS DMS user account needs to be     granted ASM privileges.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArchivedLogsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_logs_only: Option<bool>,

    ///
    /// For an Oracle source endpoint, your Oracle Automatic Storage Management (ASM) password.     You can set this value from the         asm_user_password       value.     You set this value as part of the comma-separated value that you set to the       Password request parameter when you create the endpoint to access     transaction logs using Binary Reader. For more information, see Configuration for change data capture (CDC) on an Oracle source       database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AsmPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_password: Option<cfn_resources::StrVal>,

    ///
    /// For an Oracle source endpoint, your ASM server address. You can set this value from the       asm_server value. You set asm_server as part of the extra     connection attribute string to access an Oracle server with Binary Reader that uses ASM.     For more information, see Configuration for change data capture (CDC) on an Oracle source       database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AsmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_server: Option<cfn_resources::StrVal>,

    ///
    /// For an Oracle source endpoint, your ASM user name. You can set this value from the       asm_user value. You set asm_user as part of the extra     connection attribute string to access an Oracle server with Binary Reader that uses ASM.     For more information, see Configuration for change data capture (CDC) on an Oracle source       database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AsmUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_user: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether the length of a character column is in     bytes or in characters. To indicate that the character column     length is in characters, set this attribute to CHAR. Otherwise,     the character column length is in bytes.
    ///
    /// Example: charLengthSemantics=CHAR;
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: byte | char | default
    ///
    /// Update requires: No interruption
    #[serde(rename = "CharLengthSemantics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_length_semantics: Option<OracleSettingsCharLengthSemanticsEnum>,

    ///
    /// When set to true, this attribute helps to increase the     commit rate on the Oracle target database by writing     directly to tables and not writing a trail to database logs.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DirectPathNoLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_path_no_log: Option<bool>,

    ///
    /// When set to true, this attribute specifies a parallel load     when useDirectPathFullLoad is set to Y. This attribute     also only applies when you use the AWS DMS parallel load     feature. Note that the target table cannot have any constraints or indexes.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DirectPathParallelLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_path_parallel_load: Option<bool>,

    ///
    /// Set this attribute to enable homogenous tablespace     replication and create existing tables or indexes under the     same tablespace on the target.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableHomogenousTablespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_homogenous_tablespace: Option<bool>,

    ///
    /// Specifies the IDs of one more destinations for one or more archived redo logs. These IDs     are the values of the dest_id column in the v$archived_log view.     Use this setting with the archivedLogDestId extra connection attribute in a     primary-to-single setup or a primary-to-multiple-standby setup.
    ///
    /// This setting is useful in a switchover when you use an Oracle Data Guard database as a     source. In this case, AWS DMS needs information about what destination to get archive redo     logs from to read changes. AWS DMS needs this because after the switchover the previous     primary is a standby instance. For example, in a primary-to-single standby setup you might     apply the following settings.
    ///
    /// archivedLogDestId=1; ExtraArchivedLogDestIds=[2]
    ///
    /// In a primary-to-multiple-standby setup, you might apply the following settings.
    ///
    /// archivedLogDestId=1; ExtraArchivedLogDestIds=[2,3,4]
    ///
    /// Although AWS DMS supports the use of the Oracle RESETLOGS option to open the     database, never use RESETLOGS unless it's necessary. For more information     about RESETLOGS, see RMAN Data Repair Concepts in the Oracle Database Backup and Recovery       User's Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExtraArchivedLogDestIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_archived_log_dest_ids: Option<Vec<i64>>,

    ///
    /// When set to true, this attribute causes a task to fail if the     actual size of an LOB column is greater than the specified     LobMaxSize.
    ///
    /// If a task is set to limited LOB mode and this option is set to     true, the task fails instead of truncating the LOB data.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailTasksOnLobTruncation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_tasks_on_lob_truncation: Option<bool>,

    ///
    /// Specifies the number scale. You can select a scale up to 38,     or you can select FLOAT. By default, the NUMBER data type     is converted to precision 38, scale 10.
    ///
    /// Example: numberDataTypeScale=12
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberDatatypeScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_datatype_scale: Option<i64>,

    ///
    /// Set this string attribute to the required value in order to use     the Binary Reader to capture change data for an Amazon     RDS for Oracle as the source. This value specifies the     default Oracle root used to access the redo logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OraclePathPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_path_prefix: Option<cfn_resources::StrVal>,

    ///
    /// Set this attribute to change the number of threads that DMS configures to perform a     change data capture (CDC) load using Oracle Automatic Storage Management (ASM). You can     specify an integer value between 2 (the default) and 8 (the maximum). Use this attribute     together with the readAheadBlocks attribute.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelAsmReadThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_asm_read_threads: Option<i64>,

    ///
    /// Set this attribute to change the number of read-ahead blocks that DMS configures to     perform a change data capture (CDC) load using Oracle Automatic Storage Management (ASM).     You can specify an integer value between 1000 (the default) and 200,000 (the     maximum).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadAheadBlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_ahead_blocks: Option<i64>,

    ///
    /// When set to true, this attribute supports tablespace     replication.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadTableSpaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_table_space_name: Option<bool>,

    ///
    /// Set this attribute to true in order to use the Binary Reader     to capture change data for an Amazon RDS for Oracle as the     source. This setting tells DMS instance to replace the default     Oracle root with the specified usePathPrefix setting to     access the redo logs.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplacePathPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_path_prefix: Option<bool>,

    ///
    /// Specifies the number of seconds that the system waits     before resending a query.
    ///
    /// Example: retryInterval=6;
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager secret       that allows access to the Oracle endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,         ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                   Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// Required only if your Oracle endpoint uses Advanced Storage Manager (ASM). The full ARN       of the IAM role that specifies AWS DMS as the trusted entity and grants the required       permissions to access the SecretsManagerOracleAsmSecret. This       SecretsManagerOracleAsmSecret has the secret value that allows access to       the Oracle ASM of the endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerOracleAsmSecretId. Or you can         specify clear-text values for AsmUserName, AsmPassword, and         AsmServerName. You can't specify both.For more information on creating this SecretsManagerOracleAsmSecret, the corresponding         SecretsManagerOracleAsmAccessRoleArn, and the SecretsManagerOracleAsmSecretId         that is required to access it, see                    Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerOracleAsmAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_oracle_asm_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// Required only if your Oracle endpoint uses Advanced Storage Manager (ASM). The full ARN, partial ARN, or display name of the       SecretsManagerOracleAsmSecret that contains the Oracle ASM connection details for the Oracle endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerOracleAsmSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_oracle_asm_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret that contains       the Oracle endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// For an Oracle source endpoint, the transparent data encryption (TDE) password required     by AWM DMS to access Oracle redo logs encrypted by TDE using Binary Reader. It is also the                TDE_Password       part of the comma-separated value you     set to the Password request parameter when you create the endpoint. The       SecurityDbEncryptian setting is related to this       SecurityDbEncryptionName setting. For more information, see Supported encryption methods for using Oracle as a source for AWS DMS in the         AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityDbEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_db_encryption: Option<cfn_resources::StrVal>,

    ///
    /// For an Oracle source endpoint, the name of a key used for the transparent data     encryption (TDE) of the columns and tablespaces in an Oracle source database that is     encrypted using TDE. The key value is the value of the SecurityDbEncryption     setting. For more information on setting the key name value of       SecurityDbEncryptionName, see the information and example for setting the       securityDbEncryptionName extra connection attribute in Supported encryption methods for using Oracle as a source for AWS DMS in the         AWS Database Migration Service User     Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityDbEncryptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_db_encryption_name: Option<cfn_resources::StrVal>,

    ///
    /// Use this attribute to convert SDO_GEOMETRY to      GEOJSON format. By default, DMS calls the      SDO2GEOJSON custom function if present and accessible.      Or you can create your own custom function that mimics the operation of      SDOGEOJSON and set      SpatialDataOptionToGeoJsonFunctionName to call it instead.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpatialDataOptionToGeoJsonFunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_data_option_to_geo_json_function_name: Option<cfn_resources::StrVal>,

    ///
    /// Use this attribute to specify a time in minutes for the delay in standby sync. If the     source is an Oracle Active Data Guard standby database, use this attribute to specify the     time lag between primary and standby databases.
    ///
    /// In AWS DMS, you can create an Oracle CDC task that uses an Active Data Guard standby     instance as a source for replicating ongoing changes. Doing this eliminates the need to connect     to an active database that might be in production.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "StandbyDelayTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_delay_time: Option<i64>,

    ///
    /// Set this attribute to true in order to use the Binary Reader     to capture change data for an Amazon RDS for Oracle as     the source. This tells the DMS instance to use any specified     prefix replacement to access all online redo logs.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseAlternateFolderForOnline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_alternate_folder_for_online: Option<bool>,

    ///
    /// Set this attribute to Y to capture change data using the Binary Reader utility. Set       UseLogminerReader to N to set this attribute to Y. To use Binary Reader     with Amazon RDS for Oracle as the source, you set additional attributes. For more information     about using this setting with Oracle Automatic Storage Management (ASM), see Using Oracle LogMiner or AWS DMS Binary Reader for     CDC.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseBFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_bfile: Option<bool>,

    ///
    /// Set this attribute to Y to have AWS DMS use a direct path full load.      Specify this value to use the direct path protocol in the Oracle Call Interface (OCI).      By using this OCI protocol, you can bulk-load Oracle target tables during a full load.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseDirectPathFullLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_direct_path_full_load: Option<bool>,

    ///
    /// Set this attribute to Y to capture change data using the Oracle LogMiner utility (the     default). Set this attribute to N if you want to access the redo logs as a binary file.     When you set UseLogminerReader to N, also set UseBfile to Y. For     more information on this setting and using Oracle ASM, see Using Oracle LogMiner or AWS DMS Binary Reader for CDC in     the         AWS DMS User Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseLogminerReader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_logminer_reader: Option<bool>,

    ///
    /// Set this string attribute to the required value in order to use     the Binary Reader to capture change data for an Amazon     RDS for Oracle as the source. This value specifies the path     prefix used to replace the default Oracle root to access the     redo logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UsePathPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_path_prefix: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OracleSettingsCharLengthSemanticsEnum {
    /// byte
    #[serde(rename = "byte")]
    Byte,

    /// char
    #[serde(rename = "char")]
    Char,

    /// default
    #[serde(rename = "default")]
    Default,
}

impl Default for OracleSettingsCharLengthSemanticsEnum {
    fn default() -> Self {
        OracleSettingsCharLengthSemanticsEnum::Byte
    }
}

impl cfn_resources::CfnResource for OracleSettings {
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

/// Provides information that defines a PostgreSQL endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For information about other available settings, see                Extra connection attributes when using PostgreSQL as a source for AWS DMS and               Extra connection attributes when using PostgreSQL as a target for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PostgreSqlSettings {
    ///
    /// For use with change data capture (CDC) only, this attribute     has AWS DMS bypass foreign keys and user triggers to     reduce the time it takes to bulk load data.
    ///
    /// Example: afterConnectScript=SET     session_replication_role='replica'
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AfterConnectScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<cfn_resources::StrVal>,

    ///
    /// To capture DDL events, AWS DMS creates various artifacts in     the PostgreSQL database when the task starts. You can later     remove these artifacts.
    ///
    /// If this value is set to N, you don't have to create tables or     triggers on the source database.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptureDdls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_ddls: Option<bool>,

    ///
    /// The schema in which the operational DDL database artifacts     are created.
    ///
    /// Example: ddlArtifactsSchema=xyzddlschema;
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DdlArtifactsSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddl_artifacts_schema: Option<cfn_resources::StrVal>,

    ///
    /// Sets the client statement timeout for the PostgreSQL     instance, in seconds. The default value is 60 seconds.
    ///
    /// Example: executeTimeout=100;
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecuteTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_timeout: Option<i64>,

    ///
    /// When set to true, this value causes a task to fail if the     actual size of a LOB column is greater than the specified     LobMaxSize.
    ///
    /// If task is set to Limited LOB mode and this option is set to     true, the task fails instead of truncating the LOB data.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailTasksOnLobTruncation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_tasks_on_lob_truncation: Option<bool>,

    ///
    /// The write-ahead log (WAL) heartbeat feature mimics a dummy transaction. By doing this,     it prevents idle logical replication slots from holding onto old WAL logs, which can result in     storage full situations on the source. This heartbeat keeps restart_lsn moving     and prevents storage full scenarios.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeartbeatEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_enable: Option<bool>,

    ///
    /// Sets the WAL heartbeat frequency (in minutes).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeartbeatFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_frequency: Option<i64>,

    ///
    /// Sets the schema in which the heartbeat artifacts are created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeartbeatSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_schema: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MapBooleanAsBoolean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_boolean_as_boolean: Option<bool>,

    ///
    /// Specifies the maximum size (in KB) of any .csv file used to     transfer data to PostgreSQL.
    ///
    /// Example: maxFileSize=512
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,

    ///
    /// Specifies the plugin to use to create a replication slot.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PluginName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<cfn_resources::StrVal>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager secret       that allows access to the PostgreSQL endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,         ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                   Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret that contains the PostgreSQL endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// Sets the name of a previously created logical replication slot     for a change data capture (CDC) load of the PostgreSQL source instance.
    ///
    /// When used with the CdcStartPosition     request parameter for the AWS DMS API , this attribute also makes it possible to use native CDC     start points. DMS verifies that the specified logical     replication slot exists before starting the CDC load task. It     also verifies that the task was created with a valid setting of     CdcStartPosition. If the specified slot     doesn't exist or the task doesn't have a valid     CdcStartPosition setting, DMS raises an     error.
    ///
    /// For more information about setting the CdcStartPosition request parameter,     see Determining a CDC native start point in the         AWS Database Migration Service User       Guide. For more information about using CdcStartPosition, see       CreateReplicationTask, StartReplicationTask, and ModifyReplicationTask.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SlotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for PostgreSqlSettings {
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

/// Provides information that defines a Redis target endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For information about other available settings, see               Specifying endpoint settings for Redis as a target       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedisSettings {
    ///
    /// The password provided with the auth-role and      auth-token options of the AuthType setting for a Redis      target endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_password: Option<cfn_resources::StrVal>,

    ///
    /// The type of authentication to perform when connecting to a Redis target. Options include       none, auth-token, and auth-role. The       auth-token option requires an AuthPassword value to be provided. The     auth-role option requires AuthUserName and AuthPassword values     to be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: auth-role | auth-token | none
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<RedisSettingsAuthTypeEnum>,

    ///
    /// The user name provided with the auth-role option of the      AuthType setting for a Redis target endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user_name: Option<cfn_resources::StrVal>,

    ///
    /// Transmission Control Protocol (TCP) port for the endpoint.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<f64>,

    ///
    /// Fully qualified domain name of the endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) for the certificate authority (CA) that DMS uses to     connect to your Redis target endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslCaCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ca_certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// The connection to a Redis target endpoint using Transport Layer Security (TLS). Valid     values include plaintext and ssl-encryption. The default is       ssl-encryption. The ssl-encryption option makes an encrypted     connection. Optionally, you can identify an Amazon Resource Name (ARN) for an SSL certificate authority (CA)      using the SslCaCertificateArn setting. If an ARN isn't given for a CA, DMS     uses the Amazon root CA.
    ///
    /// The plaintext option doesn't provide Transport Layer Security (TLS)      encryption for traffic between endpoint and database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: plaintext | ssl-encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslSecurityProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_security_protocol: Option<RedisSettingsSslSecurityProtocolEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RedisSettingsAuthTypeEnum {
    /// auth-role
    #[serde(rename = "auth-role")]
    Authrole,

    /// auth-token
    #[serde(rename = "auth-token")]
    Authtoken,

    /// none
    #[serde(rename = "none")]
    None,
}

impl Default for RedisSettingsAuthTypeEnum {
    fn default() -> Self {
        RedisSettingsAuthTypeEnum::Authrole
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RedisSettingsSslSecurityProtocolEnum {
    /// plaintext
    #[serde(rename = "plaintext")]
    Plaintext,

    /// ssl-encryption
    #[serde(rename = "ssl-encryption")]
    Sslencryption,
}

impl Default for RedisSettingsSslSecurityProtocolEnum {
    fn default() -> Self {
        RedisSettingsSslSecurityProtocolEnum::Plaintext
    }
}

impl cfn_resources::CfnResource for RedisSettings {
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

/// Provides information that defines an Amazon Redshift endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about other available settings, see               Extra connection attributes when using Amazon Redshift as a target for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedshiftSettings {
    ///
    /// A value that indicates to allow any date format, including invalid formats such as     00/00/00 00:00:00, to be loaded without generating an error. You can choose       true or false (the default).
    ///
    /// This parameter applies only to TIMESTAMP and DATE columns. Always use ACCEPTANYDATE with     the DATEFORMAT parameter. If the date format for the data doesn't match the DATEFORMAT     specification, Amazon Redshift inserts a NULL value into that field.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptAnyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_any_date: Option<bool>,

    ///
    /// Code to run after connecting. This parameter should contain the code itself, not the     name of a file containing the code.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AfterConnectScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<cfn_resources::StrVal>,

    ///
    /// An S3 folder where the comma-separated-value (.csv) files are stored before being      uploaded to the target Redshift cluster.
    ///
    /// For full load mode, AWS DMS converts source records into .csv files and loads them to     the BucketFolder/TableID path. AWS DMS uses the Redshift       COPY command to upload the .csv files to the target table. The files are     deleted once the COPY operation has finished. For more information, see COPY in the       Amazon Redshift Database Developer Guide.
    ///
    /// For change-data-capture (CDC) mode, AWS DMS creates a NetChanges table,      and loads the .csv files to this BucketFolder/NetChangesTableID path.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_folder: Option<cfn_resources::StrVal>,

    ///
    /// The name of the intermediate S3 bucket used to store .csv files before uploading data to Redshift.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<cfn_resources::StrVal>,

    ///
    /// If Amazon Redshift is configured to support case sensitive schema names, set       CaseSensitiveNames to true. The default is     false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaseSensitiveNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive_names: Option<bool>,

    ///
    /// If you set CompUpdate to true Amazon Redshift applies     automatic compression if the table is empty. This applies even if the table columns already     have encodings other than RAW. If you set CompUpdate to       false, automatic compression is disabled and existing column encodings     aren't changed. The default is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comp_update: Option<bool>,

    ///
    /// A value that sets the amount of time to wait (in milliseconds) before timing out,     beginning from when you initially establish a connection.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i64>,

    ///
    /// The date format that you are using. Valid values are auto (case-sensitive),     your date format string enclosed in quotes, or NULL. If this parameter is left unset     (NULL), it defaults to a format of 'YYYY-MM-DD'. Using auto recognizes most     strings, even some that aren't supported when you use a date format string.
    ///
    /// If your date and time values use formats different from each other, set this to     auto.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<cfn_resources::StrVal>,

    ///
    /// A value that specifies whether AWS DMS should migrate empty CHAR and VARCHAR fields as     NULL. A value of true sets empty CHAR and VARCHAR fields to null. The default     is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmptyAsNull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_as_null: Option<bool>,

    ///
    /// The type of server-side encryption that you want to use for your data. This encryption     type is part of the endpoint settings or the extra connections attributes for Amazon S3.     You can choose either SSE_S3 (the default) or SSE_KMS.
    ///
    /// NoteFor the ModifyEndpoint operation, you can change the existing value of the        EncryptionMode parameter from SSE_KMS to        SSE_S3. But you can’t change the existing value from SSE_S3       to SSE_KMS.
    ///
    /// To use SSE_S3, create an AWS Identity and Access Management (IAM) role with     a policy that allows "arn:aws:s3:::*" to use the following actions:       "s3:PutObject", "s3:ListBucket"
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: sse-kms | sse-s3
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<RedshiftSettingsEncryptionModeEnum>,

    ///
    /// This setting is only valid for a full-load migration task. Set ExplicitIds     to true to have tables with IDENTITY columns override their     auto-generated values with explicit values loaded from the source data files used to     populate the tables. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExplicitIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_ids: Option<bool>,

    ///
    /// The number of threads used to upload a single file. This parameter accepts a value from     1 through 64. It defaults to 10.
    ///
    /// The number of parallel streams used to upload a single .csv file to an S3 bucket using     S3 Multipart Upload. For more information, see Multipart upload       overview.
    ///
    /// FileTransferUploadStreams accepts a value from 1 through 64. It     defaults to 10.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileTransferUploadStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_transfer_upload_streams: Option<i64>,

    ///
    /// The amount of time to wait (in milliseconds) before timing out of operations performed       by AWS DMS on a Redshift cluster, such as Redshift COPY, INSERT, DELETE, and UPDATE.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_timeout: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MapBooleanAsBoolean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_boolean_as_boolean: Option<bool>,

    ///
    /// The maximum size (in KB) of any .csv file used to load data on an S3 bucket and transfer       data to Amazon Redshift. It defaults to 1048576KB (1 GB).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,

    ///
    /// A value that specifies to remove surrounding quotation marks from strings in the     incoming data. All characters within the quotation marks, including delimiters, are     retained. Choose true to remove quotation marks. The default is       false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveQuotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_quotes: Option<bool>,

    ///
    /// A value that specifies to replaces the invalid characters specified in     ReplaceInvalidChars, substituting the specified characters instead. The     default is "?".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplaceChars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_chars: Option<cfn_resources::StrVal>,

    ///
    /// A list of characters that you want to replace. Use with     ReplaceChars.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplaceInvalidChars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_invalid_chars: Option<cfn_resources::StrVal>,

    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager secret       that allows access to the Amazon Redshift endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,         ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                   Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret that contains the Amazon Redshift endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,

    ///
    /// The AWS KMS key ID. If you are using SSE_KMS for the EncryptionMode,     provide this key ID. The key that you use needs an attached policy that enables IAM user     permissions and allows use of the key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role that has access to the Amazon Redshift     service. The role must allow the iam:PassRole action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The time format that you want to use. Valid values are auto     (case-sensitive), 'timeformat_string', 'epochsecs', or     'epochmillisecs'. It defaults to 10. Using auto recognizes     most strings, even some that aren't supported when you use a time format string.
    ///
    /// If your date and time values use formats different from each other, set this parameter     to auto.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<cfn_resources::StrVal>,

    ///
    /// A value that specifies to remove the trailing white space characters from a VARCHAR     string. This parameter applies only to columns with a VARCHAR data type. Choose       true to remove unneeded white space. The default is     false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrimBlanks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_blanks: Option<bool>,

    ///
    /// A value that specifies to truncate data in columns to the appropriate number of     characters, so that the data fits in the column. This parameter applies only to columns     with a VARCHAR or CHAR data type, and rows with a size of 4 MB or less. Choose       true to truncate data. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TruncateColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate_columns: Option<bool>,

    ///
    /// The size (in KB) of the in-memory file write buffer used when generating .csv files      on the local disk at the DMS replication instance. The default value is 1000      (buffer size is 1000KB).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_buffer_size: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RedshiftSettingsEncryptionModeEnum {
    /// sse-kms
    #[serde(rename = "sse-kms")]
    Ssekms,

    /// sse-s3
    #[serde(rename = "sse-s3")]
    Sses3,
}

impl Default for RedshiftSettingsEncryptionModeEnum {
    fn default() -> Self {
        RedshiftSettingsEncryptionModeEnum::Ssekms
    }
}

impl cfn_resources::CfnResource for RedshiftSettings {
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

/// Provides information that defines an Amazon S3 endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For more information about the available settings, see               Extra connection attributes when using Amazon S3 as a source for AWS DMS and               Extra connection attributes when using Amazon S3 as a target for AWS DMS       in theAWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Settings {
    ///
    /// An optional parameter that, when set to true or y, you can use     to add column name information to the .csv output file.
    ///
    /// The default value is false. Valid values are true, false,     y, and n.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddColumnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_column_name: Option<bool>,

    ///
    /// An optional parameter to set a folder name in the S3 bucket. If provided, tables are         created in the path bucketFolder/schema_name/table_name/.         If this parameter isn't specified, the path used is schema_name/table_name/.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_folder: Option<cfn_resources::StrVal>,

    ///
    /// The name of the S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<cfn_resources::StrVal>,

    ///
    /// A value that enables AWS DMS to specify a predefined (canned) access control list (ACL) for       objects created in an Amazon S3 bucket as .csv or .parquet files. For more information       about Amazon S3 canned ACLs, see                Canned ACL       in the Amazon S3 Developer Guide.
    ///
    /// The default value is NONE. Valid values include NONE, PRIVATE,     PUBLIC_READ, PUBLIC_READ_WRITE, AUTHENTICATED_READ,     AWS_EXEC_READ, BUCKET_OWNER_READ, and     BUCKET_OWNER_FULL_CONTROL.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: authenticated-read | aws-exec-read | bucket-owner-full-control | bucket-owner-read | none | private | public-read | public-read-write
    ///
    /// Update requires: No interruption
    #[serde(rename = "CannedAclForObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl_for_objects: Option<S3SettingsCannedAclForObjectsEnum>,

    ///
    /// A value that enables a change data capture (CDC) load to write INSERT and UPDATE     operations to .csv or .parquet (columnar storage) output files. The default setting is       false, but when CdcInsertsAndUpdates is set to       true or y, only INSERTs and UPDATEs from the source database     are migrated to the .csv or .parquet file.
    ///
    /// For .csv file format only, how these INSERTs and UPDATEs are recorded depends on the     value of the IncludeOpForFullLoad parameter. If       IncludeOpForFullLoad is set to true, the first field of every     CDC record is set to either I or U to indicate INSERT and UPDATE     operations at the source. But if IncludeOpForFullLoad is set to       false, CDC records are written without an indication of INSERT or UPDATE     operations at the source. For more information about how these settings work together, see               Indicating Source DB Operations in Migrated S3 Data       in the AWS Database Migration Service User Guide.
    ///
    /// Note        AWS DMS supports the use of the CdcInsertsAndUpdates parameter in       versions 3.3.1 and later.        CdcInsertsOnly and CdcInsertsAndUpdates can't       both be set to true for the same endpoint. Set either       CdcInsertsOnly or CdcInsertsAndUpdates to true       for the same endpoint, but not both.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdcInsertsAndUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_inserts_and_updates: Option<bool>,

    ///
    /// A value that enables a change data capture (CDC) load to write only INSERT operations to       .csv or columnar storage (.parquet) output files. By default       (the false setting), the first field in a .csv or .parquet record contains the       letter I (INSERT), U (UPDATE), or D (DELETE). These values indicate whether the row was       inserted, updated, or deleted at the source database for a CDC load to the target.
    ///
    /// If CdcInsertsOnly is set to true or y, only       INSERTs from the source database are migrated to the .csv or .parquet file. For .csv format       only, how these INSERTs are recorded depends on the value of       IncludeOpForFullLoad. If IncludeOpForFullLoad is set to       true, the first field of every CDC record is set to I to indicate the       INSERT operation at the source. If IncludeOpForFullLoad is set to       false, every CDC record is written without a first field to indicate the       INSERT operation at the source. For more information about how these settings work       together, see               Indicating Source DB Operations in Migrated S3 Data       in the AWS Database Migration Service User Guide.
    ///
    /// NoteAWS DMS supports the interaction described preceding between the         CdcInsertsOnly and IncludeOpForFullLoad parameters in         versions 3.1.4 and later.         CdcInsertsOnly and CdcInsertsAndUpdates can't         both be set to true for the same endpoint. Set either         CdcInsertsOnly or CdcInsertsAndUpdates to true         for the same endpoint, but not both.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdcInsertsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_inserts_only: Option<bool>,

    ///
    /// Maximum length of the interval, defined in seconds, after which to output a file to Amazon S3.
    ///
    /// When CdcMaxBatchInterval and CdcMinFileSize are both specified, the     file write is triggered by whichever parameter condition is met first within an AWS DMS     CloudFormation template.
    ///
    /// The default value is 60 seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdcMaxBatchInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_max_batch_interval: Option<i64>,

    ///
    /// Minimum file size, defined in kilobytes, to reach for a file output to Amazon S3.
    ///
    /// When CdcMinFileSize and CdcMaxBatchInterval are both specified, the file      write is triggered by whichever parameter condition is met first within an AWS DMS      CloudFormation template.
    ///
    /// The default value is 32 MB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdcMinFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_min_file_size: Option<i64>,

    ///
    /// Specifies the folder path of CDC files. For an S3 source, this setting is required if a       task captures change data; otherwise, it's optional. If CdcPath is set, AWS DMS       reads CDC files from this path and replicates the data changes to the target endpoint.       For an S3 target if you set       PreserveTransactions       to true, AWS DMS       verifies that you have set this parameter to a folder path on your S3 target where AWS DMS       can save the transaction order for the CDC load. AWS DMS creates this CDC folder path       in either your S3 target working directory or the S3 target location specified by       BucketFolder and       BucketName.
    ///
    /// For example, if you specify CdcPath as MyChangedData, and you       specify BucketName as MyTargetBucket but do not specify       BucketFolder, AWS DMS creates the CDC folder path following:       MyTargetBucket/MyChangedData.
    ///
    /// If you specify the same CdcPath, and you specify BucketName as       MyTargetBucket and BucketFolder as MyTargetData,       AWS DMS creates the CDC folder path following:       MyTargetBucket/MyTargetData/MyChangedData.
    ///
    /// For more information on CDC including transaction order on an S3 target, see               Capturing data changes (CDC) including transaction order on the S3 target.
    ///
    /// NoteThis setting is supported in AWS DMS versions 3.4.2 and later.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdcPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_path: Option<cfn_resources::StrVal>,

    ///
    /// An optional parameter. When set to GZIP it enables the service to compress the target files.       To allow the service to write the target files uncompressed, either set this parameter to NONE (the default) or don't specify       the parameter at all. This parameter applies to both .csv and .parquet file formats.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: gzip | none
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<S3SettingsCompressionTypeEnum>,

    ///
    /// The delimiter used to separate columns in the .csv file for both source and target. The default is a comma.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<cfn_resources::StrVal>,

    ///
    /// This setting only applies if your Amazon S3 output files during a change data capture       (CDC) load are written in .csv format. If       UseCsvNoSupValue       is set to true, specify a string value that you want AWS DMS to use for all columns not included       in the supplemental log. If you do not specify a string value, AWS DMS uses the null value for       these columns regardless of the UseCsvNoSupValue setting.
    ///
    /// NoteThis setting is supported in AWS DMS versions 3.4.1 and later.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvNoSupValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_no_sup_value: Option<cfn_resources::StrVal>,

    ///
    /// An optional parameter that specifies how AWS DMS treats null     values. While handling the null value, you can use this     parameter to pass a user-defined string as null when writing to     the target. For example, when target columns are not nullable,     you can use this option to differentiate between the empty     string value and the null value. So, if you set this parameter     value to the empty string ("" or ''), AWS DMS treats the empty     string as the null value instead of NULL.
    ///
    /// The default value is NULL. Valid values include any valid string.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvNullValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_null_value: Option<cfn_resources::StrVal>,

    ///
    /// The delimiter used to separate rows in the .csv file for both source and target.
    ///
    /// The default is a carriage return (\n).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvRowDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_row_delimiter: Option<cfn_resources::StrVal>,

    ///
    /// The format of the data that you want to use for output. You can choose one of the     following:
    ///
    /// csv : This is a row-based file format with comma-separated values        (.csv).                         parquet : Apache Parquet (.parquet) is a columnar storage file format        that features efficient compression and provides faster query response.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: csv | parquet
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<S3SettingsDataFormatEnum>,

    ///
    /// The size of one data page in bytes. This parameter defaults to 1024 * 1024 bytes (1 MiB).     This number is used for .parquet file format only.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_page_size: Option<i64>,

    ///
    /// Specifies a date separating delimiter to use during folder partitioning. The default value is      SLASH. Use this parameter when DatePartitionedEnabled is set to true.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DASH | NONE | SLASH | UNDERSCORE
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatePartitionDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_delimiter: Option<S3SettingsDatePartitionDelimiterEnum>,

    ///
    /// When set to true, this parameter partitions S3 bucket folders based on       transaction commit dates. The default value is false. For more information       about date-based folder partitioning, see                Using date-based folder partitioning.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatePartitionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_enabled: Option<bool>,

    ///
    /// Identifies the sequence of the date format to use during folder partitioning. The default value is      YYYYMMDD. Use this parameter when DatePartitionedEnabled is set to true.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DDMMYYYY | MMYYYYDD | YYYYMM | YYYYMMDD | YYYYMMDDHH
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatePartitionSequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_sequence: Option<S3SettingsDatePartitionSequenceEnum>,

    ///
    /// When creating an S3 target endpoint, set DatePartitionTimezone to       convert the current UTC time into a specified time zone. The conversion occurs when a       date partition folder is created and a change data capture (CDC) file name is generated. The time zone format       is Area/Location. Use this parameter when DatePartitionedEnabled is set to         true, as shown in the following example.
    ///
    /// s3-settings='{"DatePartitionEnabled": true, "DatePartitionSequence": "YYYYMMDDHH",     "DatePartitionDelimiter": "SLASH",     "DatePartitionTimezone":"Asia/Seoul", "BucketName":     "dms-nattarat-test"}'
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatePartitionTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_timezone: Option<cfn_resources::StrVal>,

    ///
    /// The maximum size of an encoded dictionary page of a column. If the dictionary page     exceeds this, this column is stored using an encoding type of PLAIN. This     parameter defaults to 1024 * 1024 bytes (1 MiB), the maximum size of a dictionary page     before it reverts to PLAIN encoding. This size is used for      .parquet file format only.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DictPageSizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dict_page_size_limit: Option<i64>,

    ///
    /// A value that enables statistics for Parquet pages and row groups. Choose       true to enable statistics, false to disable. Statistics     include NULL, DISTINCT, MAX, and MIN     values. This parameter defaults to true. This value is used for       .parquet file format only.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_statistics: Option<bool>,

    ///
    /// The type of encoding that you're using:
    ///
    /// RLE_DICTIONARY uses a combination of bit-packing and run-length        encoding to store repeated values more efficiently. This is the default.                        PLAIN doesn't use encoding at all. Values are stored as they        are.                        PLAIN_DICTIONARY builds a dictionary of the values encountered in a        given column. The dictionary is stored in a dictionary page for each column        chunk.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: plain | plain-dictionary | rle-dictionary
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<S3SettingsEncodingTypeEnum>,

    ///
    /// The type of server-side encryption that you want to use for your data. This encryption     type is part of the endpoint settings or the extra connections attributes for Amazon S3.     You can choose either SSE_S3 (the default) or SSE_KMS.
    ///
    /// NoteFor the ModifyEndpoint operation, you can change the existing value of       the EncryptionMode parameter from SSE_KMS to        SSE_S3. But you can’t change the existing value from SSE_S3       to SSE_KMS.
    ///
    /// To use SSE_S3, you need an IAM role     with permission to allow "arn:aws:s3:::dms-*" to use the following     actions:
    ///
    /// s3:CreateBucket                                s3:ListBucket                                s3:DeleteBucket                                s3:GetBucketLocation                                s3:GetObject                                s3:PutObject                                s3:DeleteObject                                s3:GetObjectVersion                                s3:GetBucketPolicy                                s3:PutBucketPolicy                                s3:DeleteBucketPolicy
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: sse-kms | sse-s3
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<S3SettingsEncryptionModeEnum>,

    ///
    /// The external table definition.
    ///
    /// Conditional: If S3 is used as a source then ExternalTableDefinition is required.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<cfn_resources::StrVal>,

    ///
    /// When this value is set to 1, AWS DMS ignores the first row header in a .csv file. A value     of 1 turns on the feature; a value of 0 turns off the feature.
    ///
    /// The default is 0.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnoreHeaderRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_header_rows: Option<i64>,

    ///
    /// A value that enables a full load to write INSERT operations to the comma-separated value     (.csv) output files only to indicate how the rows were added to the source database.
    ///
    /// Note        AWS DMS supports the IncludeOpForFullLoad parameter in versions 3.1.4 and       later.
    ///
    /// For full load, records can only be inserted. By default (the false     setting), no information is recorded in these output files for a full load to indicate that     the rows were inserted at the source database. If IncludeOpForFullLoad is set     to true or y, the INSERT is recorded as an I annotation in the     first field of the .csv file. This allows the format of your target records from a full     load to be consistent with the target records from a CDC load.
    ///
    /// NoteThis setting works together with the CdcInsertsOnly and the         CdcInsertsAndUpdates parameters for output to .csv files only. For more         information about how these settings work together, see                   Indicating Source DB Operations in Migrated S3 Data         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeOpForFullLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_op_for_full_load: Option<bool>,

    ///
    /// A value that specifies the maximum size (in KB) of any .csv     file to be created while migrating to an S3 target during full     load.
    ///
    /// The default value is 1,048,576 KB (1 GB). Valid values include 1 to 1,048,576.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,

    ///
    /// A value that specifies the precision of any TIMESTAMP column values that     are written to an Amazon S3 object file in .parquet format.
    ///
    /// Note        AWS DMS supports the ParquetTimestampInMillisecond parameter in versions       3.1.4 and later.
    ///
    /// When ParquetTimestampInMillisecond is set to true or       y, AWS DMS writes all TIMESTAMP columns in a .parquet     formatted file with millisecond precision. Otherwise, DMS writes them with microsecond     precision.
    ///
    /// Currently, Amazon Athena and AWS Glue can handle only     millisecond precision for TIMESTAMP values. Set     this parameter to true for S3 endpoint object     files that are .parquet formatted only if you plan to query or process the data with Athena or AWS Glue.
    ///
    /// Note        AWS DMS writes any TIMESTAMP column          values written to an S3 file in .csv format with          microsecond precision.Setting ParquetTimestampInMillisecond has no effect on the string       format of the timestamp column value that is inserted by setting the        TimestampColumnName parameter.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParquetTimestampInMillisecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_timestamp_in_millisecond: Option<bool>,

    ///
    /// The version of the Apache Parquet format that you want to use: parquet_1_0     (the default) or parquet_2_0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: parquet-1-0 | parquet-2-0
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParquetVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_version: Option<S3SettingsParquetVersionEnum>,

    ///
    /// If this setting is set to true, AWS DMS saves the transaction order for a change data       capture (CDC) load on the Amazon S3 target specified by       CdcPath.       For more information, see                Capturing data changes (CDC) including transaction order on the S3 target.
    ///
    /// NoteThis setting is supported in AWS DMS versions 3.4.2 and later.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreserveTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_transactions: Option<bool>,

    ///
    /// For an S3 source, when this value is set to true or y,     each leading double quotation mark has to be followed by an     ending double quotation mark. This formatting complies with RFC     4180. When this value is set to false or     n, string literals are copied to the target as     is. In this case, a delimiter (row or column) signals the end of     the field. Thus, you can't use a delimiter as part of the     string, because it signals the end of the value.
    ///
    /// For an S3 target, an optional parameter used to set behavior to comply with RFC     4180 for data migrated to Amazon S3 using .csv file format only. When this     value is set to true or y using Amazon     S3 as a target, if the data has quotation marks or newline     characters in it, AWS DMS encloses the entire column with an     additional pair of double quotation marks ("). Every quotation     mark within the data is repeated twice.
    ///
    /// The default value is true. Valid values include true, false,     y, and n.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rfc4180")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfc4180: Option<bool>,

    ///
    /// The number of rows in a row group. A smaller row group size provides faster reads. But     as the number of row groups grows, the slower writes become. This parameter defaults to     10,000 rows. This number is used for .parquet file format only.
    ///
    /// If you choose a value larger than the maximum, RowGroupLength is set to the     max row group length in bytes (64 * 1024 * 1024).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowGroupLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_length: Option<i64>,

    ///
    /// If you are using SSE_KMS for the EncryptionMode, provide the       AWS KMS key ID. The key that you use needs an attached policy that enables        IAM user permissions and allows use of the key.
    ///
    /// Here is a CLI example: aws dms create-endpoint --endpoint-identifier        value --endpoint-type target --engine-name s3 --s3-settings        ServiceAccessRoleArn=value,BucketFolder=value,BucketName=value,EncryptionMode=SSE_KMS,ServerSideEncryptionKmsKeyId=value
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// A required parameter that specifies the Amazon Resource Name (ARN) used by the service to access the IAM role.       The role must allow the iam:PassRole action. It enables AWS DMS to read and write objects from an S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// A value that when nonblank causes AWS DMS to add a column with timestamp information to     the endpoint data for an Amazon S3 target.
    ///
    /// Note        AWS DMS supports the TimestampColumnName parameter in versions 3.1.4 and later.
    ///
    /// AWS DMS includes an additional STRING column in the     .csv or .parquet object files of your migrated data when you set     TimestampColumnName to a nonblank value.
    ///
    /// For a full load, each row of this timestamp column contains a     timestamp for when the data was transferred from the source to     the target by DMS.
    ///
    /// For a change data capture (CDC) load, each row of the timestamp column contains the     timestamp for the commit of that row in the source     database.
    ///
    /// The string format for this timestamp column value is     yyyy-MM-dd HH:mm:ss.SSSSSS. By default, the     precision of this value is in microseconds. For a CDC load, the     rounding of the precision depends on the commit timestamp     supported by DMS for the source database.
    ///
    /// When the AddColumnName parameter is set to true, DMS also     includes a name for the timestamp column that you set with     TimestampColumnName.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampColumnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_column_name: Option<cfn_resources::StrVal>,

    ///
    /// This setting applies if the S3 output files during a change data capture (CDC) load are       written in .csv format. If this setting is set to true for columns not included in the       supplemental log, AWS DMS uses the value specified by       CsvNoSupValue.       If this setting isn't set or is set to false, AWS DMS uses the null value for these columns.
    ///
    /// NoteThis setting is supported in AWS DMS versions 3.4.1 and later.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseCsvNoSupValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_csv_no_sup_value: Option<bool>,

    ///
    /// When set to true, this parameter uses the task start time as the timestamp column value instead of      the time data is written to target. For full load, when useTaskStartTimeForFullLoadTimestamp     is set to true, each row of the timestamp column contains the task start time. For CDC loads,      each row of the timestamp column contains the transaction commit time.
    ///
    /// When useTaskStartTimeForFullLoadTimestamp is set to false, the full load timestamp      in the timestamp column increments with the time data arrives at the target.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseTaskStartTimeForFullLoadTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_task_start_time_for_full_load_timestamp: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3SettingsCannedAclForObjectsEnum {
    /// authenticated-read
    #[serde(rename = "authenticated-read")]
    Authenticatedread,

    /// aws-exec-read
    #[serde(rename = "aws-exec-read")]
    Awsexecread,

    /// bucket-owner-full-control
    #[serde(rename = "bucket-owner-full-control")]
    Bucketownerfullcontrol,

    /// bucket-owner-read
    #[serde(rename = "bucket-owner-read")]
    Bucketownerread,

    /// none
    #[serde(rename = "none")]
    None,

    /// private
    #[serde(rename = "private")]
    Private,

    /// public-read
    #[serde(rename = "public-read")]
    Publicread,

    /// public-read-write
    #[serde(rename = "public-read-write")]
    Publicreadwrite,
}

impl Default for S3SettingsCannedAclForObjectsEnum {
    fn default() -> Self {
        S3SettingsCannedAclForObjectsEnum::Authenticatedread
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3SettingsCompressionTypeEnum {
    /// gzip
    #[serde(rename = "gzip")]
    Gzip,

    /// none
    #[serde(rename = "none")]
    None,
}

impl Default for S3SettingsCompressionTypeEnum {
    fn default() -> Self {
        S3SettingsCompressionTypeEnum::Gzip
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3SettingsDataFormatEnum {
    /// csv
    #[serde(rename = "csv")]
    Csv,

    /// parquet
    #[serde(rename = "parquet")]
    Parquet,
}

impl Default for S3SettingsDataFormatEnum {
    fn default() -> Self {
        S3SettingsDataFormatEnum::Csv
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3SettingsDatePartitionDelimiterEnum {
    /// DASH
    #[serde(rename = "DASH")]
    Dash,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SLASH
    #[serde(rename = "SLASH")]
    Slash,

    /// UNDERSCORE
    #[serde(rename = "UNDERSCORE")]
    Underscore,
}

impl Default for S3SettingsDatePartitionDelimiterEnum {
    fn default() -> Self {
        S3SettingsDatePartitionDelimiterEnum::Dash
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3SettingsDatePartitionSequenceEnum {
    /// DDMMYYYY
    #[serde(rename = "DDMMYYYY")]
    Ddmmyyyy,

    /// MMYYYYDD
    #[serde(rename = "MMYYYYDD")]
    Mmyyyydd,

    /// YYYYMM
    #[serde(rename = "YYYYMM")]
    Yyyymm,

    /// YYYYMMDD
    #[serde(rename = "YYYYMMDD")]
    Yyyymmdd,

    /// YYYYMMDDHH
    #[serde(rename = "YYYYMMDDHH")]
    Yyyymmddhh,
}

impl Default for S3SettingsDatePartitionSequenceEnum {
    fn default() -> Self {
        S3SettingsDatePartitionSequenceEnum::Ddmmyyyy
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3SettingsEncodingTypeEnum {
    /// plain
    #[serde(rename = "plain")]
    Plain,

    /// plain-dictionary
    #[serde(rename = "plain-dictionary")]
    Plaindictionary,

    /// rle-dictionary
    #[serde(rename = "rle-dictionary")]
    Rledictionary,
}

impl Default for S3SettingsEncodingTypeEnum {
    fn default() -> Self {
        S3SettingsEncodingTypeEnum::Plain
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3SettingsEncryptionModeEnum {
    /// sse-kms
    #[serde(rename = "sse-kms")]
    Ssekms,

    /// sse-s3
    #[serde(rename = "sse-s3")]
    Sses3,
}

impl Default for S3SettingsEncryptionModeEnum {
    fn default() -> Self {
        S3SettingsEncryptionModeEnum::Ssekms
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3SettingsParquetVersionEnum {
    /// parquet-1-0
    #[serde(rename = "parquet-1-0")]
    Parquet10,

    /// parquet-2-0
    #[serde(rename = "parquet-2-0")]
    Parquet20,
}

impl Default for S3SettingsParquetVersionEnum {
    fn default() -> Self {
        S3SettingsParquetVersionEnum::Parquet10
    }
}

impl cfn_resources::CfnResource for S3Settings {
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

/// Provides information that defines a SAP ASE endpoint. This       information includes the output format of records applied to the endpoint and details of       transaction and control table data information. For information about other available settings, see               Extra connection attributes when using SAP ASE as a source for AWS DMS and               Extra connection attributes when using SAP ASE as a target for AWS DMS       in the AWS Database Migration Service User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SybaseSettings {
    ///
    /// The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the       trusted entity and grants the required permissions to access the value in       SecretsManagerSecret. The role must allow the iam:PassRole action.       SecretsManagerSecret has the value of the AWS Secrets Manager       secret that allows access to the SAP ASE endpoint.
    ///
    /// NoteYou can specify one of two sets of values for these permissions. You can specify the         values for this setting and SecretsManagerSecretId. Or you can specify         clear-text values for UserName, Password,         ServerName, and Port. You can't specify both.For more information on creating this SecretsManagerSecret, the corresponding         SecretsManagerAccessRoleArn, and the SecretsManagerSecretId         that is required to access it, see                   Using secrets to access AWS Database Migration Service resources         in the AWS Database Migration Service User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The full ARN, partial ARN, or display name of the SecretsManagerSecret that contains the SAP SAE endpoint connection details.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SybaseSettings {
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
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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
