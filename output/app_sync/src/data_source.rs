

/// The AWS::AppSync::DataSource resource creates data sources for resolvers in AWS AppSync to connect to, such as Amazon DynamoDB, AWS Lambda, and Amazon OpenSearch Service. Resolvers use these data sources to fetch data when clients make GraphQL calls.
#[derive(Default, serde::Serialize)]
pub struct CfnDataSource {


    /// 
    /// AWS Region and TableName for an Amazon DynamoDB table in your account.
    /// 
    /// Required: No
    ///
    /// Type: DynamoDBConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDBConfig")]
    pub dynamo_dbconfig: Option<DynamoDBConfig>,


    /// 
    /// Unique AWS AppSync GraphQL API identifier where this data source will be created.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: String,


    /// 
    /// AWS Region and Endpoints for an Amazon OpenSearch Service domain in your account.
    /// 
    /// As of September 2021, Amazon Elasticsearch Service is Amazon OpenSearch Service. This property is deprecated.     For new data sources, use OpenSearchServiceConfig to specify an OpenSearch Service data     source.
    /// 
    /// Required: No
    ///
    /// Type: ElasticsearchConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticsearchConfig")]
    pub elasticsearch_config: Option<ElasticsearchConfig>,


    /// 
    /// AWS Region and Endpoints for an Amazon OpenSearch Service domain in your account.
    /// 
    /// Required: No
    ///
    /// Type: OpenSearchServiceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenSearchServiceConfig")]
    pub open_search_service_config: Option<OpenSearchServiceConfig>,


    /// 
    /// An EventBridge configuration that contains a valid ARN of an event bus.
    /// 
    /// Required: No
    ///
    /// Type: EventBridgeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBridgeConfig")]
    pub event_bridge_config: Option<EventBridgeConfig>,


    /// 
    /// The type of the data source.
    /// 
    /// AWS_LAMBDA: The data source is an AWS Lambda        function.                    AMAZON_DYNAMODB: The data source is an Amazon DynamoDB        table.                    AMAZON_ELASTICSEARCH: The data source is an Amazon OpenSearch Service        domain.                    AMAZON_EVENTBRIDGE: The data source is an Amazon EventBridge event        bus.                    AMAZON_OPENSEARCH_SERVICE: The data source is an Amazon OpenSearch Service        domain.                    NONE: There is no data source. This type is used when you wish to invoke        a GraphQL operation without connecting to a data source, such as performing data transformation with        resolvers or triggering a subscription to be invoked from a mutation.                    HTTP: The data source is an HTTP endpoint.                    RELATIONAL_DATABASE: The data source is a relational database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// Endpoints for an HTTP data source.
    /// 
    /// Required: No
    ///
    /// Type: HttpConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpConfig")]
    pub http_config: Option<HttpConfig>,


    /// 
    /// The description of the data source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The AWS Identity and Access Management service role ARN for the data source. The system assumes this role when     accessing the data source.
    /// 
    /// Required if Type is specified as AWS_LAMBDA, AMAZON_DYNAMODB,       AMAZON_ELASTICSEARCH, AMAZON_EVENTBRIDGE, or       AMAZON_OPENSEARCH_SERVICE.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: Option<String>,


    /// 
    /// An ARN of a Lambda function in valid ARN format. This can be the ARN of a Lambda function that exists in the     current account or in another account.
    /// 
    /// Required: No
    ///
    /// Type: LambdaConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaConfig")]
    pub lambda_config: Option<LambdaConfig>,


    /// 
    /// Relational Database configuration of the relational database data source.
    /// 
    /// Required: No
    ///
    /// Type: RelationalDatabaseConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelationalDatabaseConfig")]
    pub relational_database_config: Option<RelationalDatabaseConfig>,


    /// 
    /// Friendly name for you to identify your AppSync data source after creation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}


/// Use the HttpConfig property type to specify HttpConfig for an AWS AppSync data source.
///
/// HttpConfig is a property of the AWS::AppSync::DataSource resource.
#[derive(Default, serde::Serialize)]
pub struct HttpConfig {


    /// 
    /// The authorization configuration.
    /// 
    /// Required: No
    ///
    /// Type: AuthorizationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationConfig")]
    pub authorization_config: Option<AuthorizationConfig>,


    /// 
    /// The endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: String,

}


/// The DynamoDBConfig property type specifies the AwsRegion and       TableName for an Amazon DynamoDB table in your account for an AWS AppSync data source.
///
/// DynamoDBConfig is a property of the AWS::AppSync::DataSource property type.
#[derive(Default, serde::Serialize)]
pub struct DynamoDBConfig {


    /// 
    /// The DeltaSyncConfig for a versioned datasource.
    /// 
    /// Required: No
    ///
    /// Type: DeltaSyncConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaSyncConfig")]
    pub delta_sync_config: Option<DeltaSyncConfig>,


    /// 
    /// Set to TRUE to use AWS Identity and Access Management with this data source.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseCallerCredentials")]
    pub use_caller_credentials: Option<bool>,


    /// 
    /// The table name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,


    /// 
    /// Set to TRUE to use Conflict Detection and Resolution with this data source.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Versioned")]
    pub versioned: Option<bool>,


    /// 
    /// The AWS Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,

}


/// Use the AwsIamConfig property type to specify AwsIamConfig for a AWS AppSync authorizaton.
///
/// AwsIamConfig is a property of the AWS AppSync DataSource AuthorizationConfig resource.
#[derive(Default, serde::Serialize)]
pub struct AwsIamConfig {


    /// 
    /// The signing service name for AWS Identity and Access Management authorization.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningServiceName")]
    pub signing_service_name: Option<String>,


    /// 
    /// The signing Region for AWS Identity and Access Management authorization.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningRegion")]
    pub signing_region: Option<String>,

}


/// Use the RelationalDatabaseConfig property type to specify RelationalDatabaseConfig     for an AWS AppSync data source.
///
/// RelationalDatabaseConfig is a property of the AWS::AppSync::DataSource property type.
#[derive(Default, serde::Serialize)]
pub struct RelationalDatabaseConfig {


    /// 
    /// Information about the Amazon RDS resource.
    /// 
    /// Required: No
    ///
    /// Type: RdsHttpEndpointConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RdsHttpEndpointConfig")]
    pub rds_http_endpoint_config: Option<RdsHttpEndpointConfig>,


    /// 
    /// The type of relational data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelationalDatabaseSourceType")]
    pub relational_database_source_type: String,

}


/// Describes a Delta Sync configuration.
#[derive(Default, serde::Serialize)]
pub struct DeltaSyncConfig {


    /// 
    /// The number of minutes that a Delta Sync log entry is stored in the Delta Sync     table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaSyncTableTTL")]
    pub delta_sync_table_ttl: String,


    /// 
    /// The Delta Sync table name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaSyncTableName")]
    pub delta_sync_table_name: String,


    /// 
    /// The number of minutes that an Item is stored in the data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseTableTTL")]
    pub base_table_ttl: String,

}


/// Use the RdsHttpEndpointConfig property type to specify the RdsHttpEndpoint for an       AWS AppSync relational database.
///
/// RdsHttpEndpointConfig is a property of the AWS AppSync DataSource RelationalDatabaseConfig resource.
#[derive(Default, serde::Serialize)]
pub struct RdsHttpEndpointConfig {


    /// 
    /// AWS Region for RDS HTTP endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,


    /// 
    /// Logical schema name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schema")]
    pub schema: Option<String>,


    /// 
    /// The ARN for database credentials stored in AWS Secrets Manager.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsSecretStoreArn")]
    pub aws_secret_store_arn: String,


    /// 
    /// Logical database name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


    /// 
    /// Amazon RDS cluster Amazon Resource Name (ARN).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DbClusterIdentifier")]
    pub db_cluster_identifier: String,

}


/// The LambdaConfig property type specifies the Lambda function ARN for an AWS AppSync data source.
///
/// LambdaConfig is a property of the AWS::AppSync::DataSource property type.
#[derive(Default, serde::Serialize)]
pub struct LambdaConfig {


    /// 
    /// The ARN for the Lambda function.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: String,

}


/// The AuthorizationConfig property type specifies the authorization type and configuration for an       AWS AppSync http data source.
///
/// AuthorizationConfig is a property of the AWS AppSync DataSource HttpConfig property type.
#[derive(Default, serde::Serialize)]
pub struct AuthorizationConfig {


    /// 
    /// The AWS Identity and Access Management settings.
    /// 
    /// Required: No
    ///
    /// Type: AwsIamConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsIamConfig")]
    pub aws_iam_config: Option<AwsIamConfig>,


    /// 
    /// The authorization type that the HTTP endpoint requires.
    /// 
    /// AWS_IAM: The authorization type is Signature        Version 4 (SigV4).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationType")]
    pub authorization_type: String,

}


/// The OpenSearchServiceConfig property type specifies the AwsRegion and       Endpoints for an Amazon OpenSearch Service domain in your account for an AWS AppSync data source.
///
/// OpenSearchServiceConfig is a property of the AWS::AppSync::DataSource property type.
#[derive(Default, serde::Serialize)]
pub struct OpenSearchServiceConfig {


    /// 
    /// The AWS Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,


    /// 
    /// The endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: String,

}


/// The data source. This can be an API destination, resource, or AWS service.
#[derive(Default, serde::Serialize)]
pub struct EventBridgeConfig {


    /// 
    /// The event bus pipeline's ARN. For more information about event buses, see EventBridge event buses.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBusArn")]
    pub event_bus_arn: String,

}


/// The ElasticsearchConfig property type specifies the AwsRegion and       Endpoints for an Amazon OpenSearch Service domain in your account for an AWS AppSync data source.
///
/// ElasticsearchConfig is a property of the AWS::AppSync::DataSource property type.
///
/// As of September 2021, Amazon Elasticsearch Service is Amazon OpenSearch Service. This property is deprecated.     For new data sources, use OpenSearchServiceConfig to specify an OpenSearch Service data     source.
#[derive(Default, serde::Serialize)]
pub struct ElasticsearchConfig {


    /// 
    /// The AWS Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,


    /// 
    /// The endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: String,

}
