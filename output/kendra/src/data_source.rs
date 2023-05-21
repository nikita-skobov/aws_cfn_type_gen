

/// Creates a data source connector that you want to use with an Amazon Kendra index.
///
/// You specify a name, data source connector type and description for your data source. You also specify configuration information for the data source connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataSource {


    /// 
    /// Configuration information for altering document metadata and content during the document ingestion process.
    /// 
    /// Required: No
    ///
    /// Type: CustomDocumentEnrichmentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomDocumentEnrichmentConfiguration")]
    pub custom_document_enrichment_configuration: Option<CustomDocumentEnrichmentConfiguration>,


    /// 
    /// The name of the data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: [a-zA-Z0-9][a-zA-Z0-9_-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The type of the data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALFRESCO | BOX | CONFLUENCE | CUSTOM | DATABASE | FSX | GITHUB | GOOGLEDRIVE | JIRA | ONEDRIVE | QUIP | S3 | SALESFORCE | SERVICENOW | SHAREPOINT | SLACK | TEMPLATE | WEBCRAWLER | WORKDOCS
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// Configuration information for an Amazon Kendra data source. The       contents of the configuration depend on the type of data source. You       can only specify one type of data source in the configuration.
    /// 
    /// You can't specify the Configuration parameter when       the Type parameter is set to       CUSTOM.
    /// 
    /// The Configuration parameter is required for all other       data sources.
    /// 
    /// Required: No
    ///
    /// Type: DataSourceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceConfiguration")]
    pub data_source_configuration: Option<DataSourceConfiguration>,


    /// 
    /// An array of key-value pairs to apply to this resource
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Sets the frequency that Amazon Kendra checks the documents in your       data source and updates the index. If you don't set a schedule,       Amazon Kendra doesn't periodically update the index.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Option<String>,


    /// 
    /// A description for the data source connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of a role with permission to access       the data source.
    /// 
    /// You can't specify the RoleArn parameter when the         Type parameter is set to       CUSTOM.
    /// 
    /// The RoleArn parameter is required for all other data       sources.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The identifier of the index you want to use with the data source connector.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexId")]
    pub index_id: String,

}

impl cfn_resources::CfnResource for CfnDataSource {
    fn type_string() -> &'static str {
        "AWS::Kendra::DataSource"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Provides the configuration information to connect to Confluence as your data       source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluenceConfiguration {


    /// 
    /// The URL of your Confluence instance. Use the full URL of the server. For example,         https://server.example.com:port/. You can also use an IP       address, for example, https://192.168.1.113/.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^(https?|ftp|file):\/\/([^\s]*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerUrl")]
    pub server_url: String,


    /// 
    /// Configuration information for indexing Confluence blogs.
    /// 
    /// Required: No
    ///
    /// Type: ConfluenceBlogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlogConfiguration")]
    pub blog_configuration: Option<ConfluenceBlogConfiguration>,


    /// 
    /// A list of regular expression patterns to exclude certain blog posts, pages, spaces, or       attachments in your Confluence. Content that matches the patterns are excluded from the       index. Content that doesn't match the patterns is included in the index. If content       matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence       and the content isn't included in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<Vec<String>>,


    /// 
    /// Configuration information for indexing Confluence pages.
    /// 
    /// Required: No
    ///
    /// Type: ConfluencePageConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PageConfiguration")]
    pub page_configuration: Option<ConfluencePageConfiguration>,


    /// 
    /// Configuration information for indexing attachments to Confluence blogs and       pages.
    /// 
    /// Required: No
    ///
    /// Type: ConfluenceAttachmentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachmentConfiguration")]
    pub attachment_configuration: Option<ConfluenceAttachmentConfiguration>,


    /// 
    /// A list of regular expression patterns to include certain blog posts, pages, spaces, or       attachments in your Confluence. Content that matches the patterns are included in the       index. Content that doesn't match the patterns is excluded from the index. If content       matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence       and the content isn't included in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<Vec<String>>,


    /// 
    /// Configuration information for indexing Confluence spaces.
    /// 
    /// Required: No
    ///
    /// Type: ConfluenceSpaceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpaceConfiguration")]
    pub space_configuration: Option<ConfluenceSpaceConfiguration>,


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Secrets Manager secret that contains the       user name and password required to connect to the Confluence instance. If you use       Confluence Cloud, you use a generated API token as the password.
    /// 
    /// You can also provide authentication credentials in the form of a personal access       token. For more information, see Using a Confluence data         source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,


    /// 
    /// The version or the type of Confluence installation to connect to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CLOUD | SERVER
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: String,


    /// 
    /// Configuration information for an Amazon Virtual Private Cloud to connect to your Confluence.       For more information, see Configuring a VPC.
    /// 
    /// Required: No
    ///
    /// Type: DataSourceVpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,

}


/// Configuration of attachment settings for the Confluence data source. Attachment       settings are optional, if you don't specify settings attachments, Amazon Kendra       won't index them.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluenceAttachmentConfiguration {


    /// 
    /// TRUE to index attachments of pages and blogs in Confluence.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,


    /// 
    /// Maps attributes or field names of Confluence attachments to Amazon Kendra index       field names. To create custom fields, use the UpdateIndex API before you       map to Confluence fields. For more information, see Mapping data source fields. The       Confluence data source field names must exist in your Confluence custom metadata.
    /// 
    /// If you specify the AttachentFieldMappings parameter, you must specify at       least one field mapping.
    /// 
    /// Required: No
    ///
    /// Type: List of ConfluenceAttachmentToIndexFieldMapping
    ///
    /// Maximum: 11
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachmentFieldMappings")]
    pub attachment_field_mappings: Option<Vec<ConfluenceAttachmentToIndexFieldMapping>>,

}


/// Maps a column or attribute in the data source to an index field.       You must first create the fields in the index using the UpdateIndex operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceToIndexFieldMapping {


    /// 
    /// The name of the column or attribute in the data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: String,


    /// 
    /// The name of the field in the index.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 30
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: String,


    /// 
    /// The type of data stored in the column or attribute.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Pattern: ^(?!\s).*(?<!\s)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<String>,

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


/// Provides the configuration information that's required to connect to a       database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectionConfiguration {


    /// 
    /// The name of the database containing the document data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// 
    /// The Amazon Resource Name (ARN) of credentials stored in AWS Secrets Manager. The       credentials should be a user/password pair. For more information, see Using a         Database Data Source. For more information about AWS Secrets Manager, see         What         Is AWS Secrets Manager in the         AWS Secrets Manager       user guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,


    /// 
    /// The port that the database uses for connections.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabasePort")]
    pub database_port: i64,


    /// 
    /// The name of the host for the database. Can be either a string       (host.subdomain.domain.tld) or an IPv4 or IPv6 address.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 253
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseHost")]
    pub database_host: String,


    /// 
    /// The name of the table that contains the document data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,

}


/// Provides the configuration information to connect to an Amazon VPC.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceVpcConfiguration {


    /// 
    /// A list of identifiers for subnets within your Amazon VPC. The subnets should be able to    connect to each other in the VPC, and they should have outgoing access to the Internet through    a NAT device.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 6
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


    /// 
    /// A list of identifiers of security groups within your Amazon VPC. The security groups    should enable Amazon Kendra to connect to the data source.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

}


/// Provides the configuration information to connect to an Amazon S3       bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3DataSourceConfiguration {


    /// 
    /// The name of the bucket that contains the documents.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// 
    /// A list of glob patterns for documents that should be indexed. If a document that       matches an inclusion pattern also matches an exclusion pattern, the document is not       indexed.
    /// 
    /// Some examples       are:
    /// 
    /// *.txt will include all text files in a directory (files           with the extension .txt).                        **/*.txt will include all text files in a directory and           its subdirectories.                        *tax* will include all files in a directory that contain           'tax' in the file name, such as 'tax', 'taxes', 'income_tax'.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<Vec<String>>,


    /// 
    /// Specifies document metadata files that contain information such as       the document access control information, source URI, document       author, and custom attributes. Each metadata file contains metadata       about a single document.
    /// 
    /// Required: No
    ///
    /// Type: DocumentsMetadataConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentsMetadataConfiguration")]
    pub documents_metadata_configuration: Option<DocumentsMetadataConfiguration>,


    /// 
    /// A list of glob patterns for documents that should not be indexed. If a document that       matches an inclusion prefix or inclusion pattern also matches an exclusion pattern, the       document is not indexed.
    /// 
    /// Some examples       are:
    /// 
    /// *.png , *.jpg will exclude all PNG and JPEG image files           in a directory (files with the extensions .png and .jpg).                        *internal* will exclude all files in a directory that           contain 'internal' in the file name, such as 'internal', 'internal_only',           'company_internal'.                        **/*internal* will exclude all internal-related files in           a directory and its subdirectories.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<Vec<String>>,


    /// 
    /// Provides the path to the S3 bucket that contains the user context filtering files for       the data source. For the format of the file, see Access control for S3 data       sources.
    /// 
    /// Required: No
    ///
    /// Type: AccessControlListConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlListConfiguration")]
    pub access_control_list_configuration: Option<AccessControlListConfiguration>,


    /// 
    /// A list of S3 prefixes for the documents that should be included in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "InclusionPrefixes")]
    pub inclusion_prefixes: Option<Vec<String>>,

}


/// Provides the configuration information for standard Salesforce knowledge       articles.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceStandardKnowledgeArticleTypeConfiguration {


    /// 
    /// The name of the field that contains the document data to index.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,


    /// 
    /// Maps attributes or field names of the knowledge article to Amazon Kendra index       field names. To create custom fields, use the UpdateIndex API before you       map to Salesforce fields. For more information, see Mapping data source fields. The       Salesforce data source field names must exist in your Salesforce custom metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// The name of the field that contains the document title.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<String>,

}


/// Provides the configuration information for a web proxy to connect to website       hosts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProxyConfiguration {


    /// 
    /// The name of the website host you want to connect to via a web proxy server.
    /// 
    /// For example, the host name of https://a.example.com/page1.html is       "a.example.com".
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 253
    ///
    /// Pattern: ([^\s]*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// Your secret ARN, which you can create in AWS Secrets Manager
    /// 
    /// The credentials are optional. You use a secret if web proxy credentials are required       to connect to a website host. Amazon Kendra currently support basic authentication       to connect to a web proxy server. The secret stores your credentials.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    pub credentials: Option<String>,


    /// 
    /// The port number of the website host you want to connect to via a web proxy server.
    /// 
    /// For example, the port for https://a.example.com/page1.html is 443, the standard port       for HTTPS.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: i64,

}


/// Provides the configuration information to connect to a index.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabaseConfiguration {


    /// 
    /// The type of database engine that runs the database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: RDS_AURORA_MYSQL | RDS_AURORA_POSTGRESQL | RDS_MYSQL | RDS_POSTGRESQL
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseEngineType")]
    pub database_engine_type: String,


    /// 
    /// Provides information about how Amazon Kendra uses quote marks       around SQL identifiers when querying a database data source.
    /// 
    /// Required: No
    ///
    /// Type: SqlConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlConfiguration")]
    pub sql_configuration: Option<SqlConfiguration>,


    /// 
    /// Information about the database column that provides information for user context       filtering.
    /// 
    /// Required: No
    ///
    /// Type: AclConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AclConfiguration")]
    pub acl_configuration: Option<AclConfiguration>,


    /// 
    /// Provides information for connecting to an Amazon VPC.
    /// 
    /// Required: No
    ///
    /// Type: DataSourceVpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,


    /// 
    /// Information about where the index should get the document information from the       database.
    /// 
    /// Required: Yes
    ///
    /// Type: ColumnConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnConfiguration")]
    pub column_configuration: ColumnConfiguration,


    /// 
    /// Configuration information that's required to connect to a database.
    /// 
    /// Required: Yes
    ///
    /// Type: ConnectionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionConfiguration")]
    pub connection_configuration: ConnectionConfiguration,

}


/// Provides the configuration information to connect to ServiceNow as your data       source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceNowConfiguration {


    /// 
    /// The type of authentication used to connect to the ServiceNow instance. If you choose         HTTP_BASIC, Amazon Kendra is authenticated using the user name and       password provided in the AWS Secrets Manager secret in the SecretArn       field. If you choose OAUTH2, Amazon Kendra is authenticated using the       credentials of client ID, client secret, user name and password.
    /// 
    /// When you use OAUTH2 authentication, you must generate a token and a       client secret using the ServiceNow console. For more information, see Using a         ServiceNow data source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HTTP_BASIC | OAUTH2
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: Option<String>,


    /// 
    /// The identifier of the release that the ServiceNow host is running. If the host is not       running the LONDON release, use OTHERS.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: LONDON | OTHERS
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNowBuildVersion")]
    pub service_now_build_version: String,


    /// 
    /// The ServiceNow instance that the data source connects to. The host endpoint should       look like the following: {instance}.service-now.com.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^(?!(^(https?|ftp|file):\/\/))[a-z0-9-]+(\.service-now\.com)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostUrl")]
    pub host_url: String,


    /// 
    /// Configuration information for crawling service catalogs in the ServiceNow site.
    /// 
    /// Required: No
    ///
    /// Type: ServiceNowServiceCatalogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceCatalogConfiguration")]
    pub service_catalog_configuration: Option<ServiceNowServiceCatalogConfiguration>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Secrets Manager secret that contains the       user name and password required to connect to the ServiceNow instance. You can also       provide OAuth authentication credentials of user name, password, client ID, and client       secret. For more information, see Using a ServiceNow data         source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,


    /// 
    /// Configuration information for crawling knowledge articles in the ServiceNow       site.
    /// 
    /// Required: No
    ///
    /// Type: ServiceNowKnowledgeArticleConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "KnowledgeArticleConfiguration")]
    pub knowledge_article_configuration: Option<ServiceNowKnowledgeArticleConfiguration>,

}


/// Provides the configuration information for applying basic logic to alter document       metadata and content when ingesting documents into Amazon Kendra. To apply advanced       logic, to go beyond what you can do with basic logic, see HookConfiguration.
///
/// For more information, see Customizing document metadata         during the ingestion process.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InlineCustomDocumentEnrichmentConfiguration {


    /// 
    /// TRUE to delete content if the condition used for the target attribute is       met.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentContentDeletion")]
    pub document_content_deletion: Option<bool>,


    /// 
    /// Configuration of the target document attribute or metadata field when ingesting       documents into Amazon Kendra. You can also include a value.
    /// 
    /// Required: No
    ///
    /// Type: DocumentAttributeTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: Option<DocumentAttributeTarget>,


    /// 
    /// Configuration of the condition used for the target document attribute or metadata       field when ingesting documents into Amazon Kendra.
    /// 
    /// Required: No
    ///
    /// Type: DocumentAttributeCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Condition")]
    pub condition: Option<DocumentAttributeCondition>,

}


/// Configuration information for indexing Confluence spaces.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluenceSpaceConfiguration {


    /// 
    /// A list of space keys of Confluence spaces. If you include a key, the blogs, documents,       and attachments in the space are not indexed. If a space is in both the         ExcludeSpaces and the IncludeSpaces list, the space is       excluded.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeSpaces")]
    pub exclude_spaces: Option<Vec<String>>,


    /// 
    /// TRUE to index archived spaces.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlArchivedSpaces")]
    pub crawl_archived_spaces: Option<bool>,


    /// 
    /// TRUE to index personal spaces. You can add restrictions to items in       personal spaces. If personal spaces are indexed, queries without user context       information may return restricted items from a personal space in their results. For more       information, see Filtering on user       context.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlPersonalSpaces")]
    pub crawl_personal_spaces: Option<bool>,


    /// 
    /// Maps attributes or field names of Confluence spaces to Amazon Kendra index field       names. To create custom fields, use the UpdateIndex API before you map to       Confluence fields. For more information, see Mapping data source fields. The       Confluence data source field names must exist in your Confluence custom metadata.
    /// 
    /// If you specify the SpaceFieldMappings parameter, you must specify at       least one field mapping.
    /// 
    /// Required: No
    ///
    /// Type: List of ConfluenceSpaceToIndexFieldMapping
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpaceFieldMappings")]
    pub space_field_mappings: Option<Vec<ConfluenceSpaceToIndexFieldMapping>>,


    /// 
    /// A list of space keys for Confluence spaces. If you include a key, the blogs,       documents, and attachments in the space are indexed. Spaces that aren't in the list       aren't indexed. A space in the list must exist. Otherwise, Amazon Kendra logs an       error when the data source is synchronized. If a space is in both the         IncludeSpaces and the ExcludeSpaces list, the space is       excluded.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeSpaces")]
    pub include_spaces: Option<Vec<String>>,

}


/// Provides the configuration information for crawling knowledge articles in the       ServiceNow site.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceNowKnowledgeArticleConfiguration {


    /// 
    /// A query that selects the knowledge articles to index. The query can return articles       from multiple knowledge bases, and the knowledge bases can be public or private.
    /// 
    /// The query string must be one generated by the ServiceNow console. For more       information, see Specifying documents to index with a         query.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterQuery")]
    pub filter_query: Option<String>,


    /// 
    /// TRUE to index attachments to knowledge articles.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,


    /// 
    /// Maps attributes or field names of knoweldge articles to Amazon Kendra index field       names. To create custom fields, use the UpdateIndex API before you map to       ServiceNow fields. For more information, see Mapping data source fields. The       ServiceNow data source field names must exist in your ServiceNow custom metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// A list of regular expression patterns to exclude certain attachments of knowledge       articles in your ServiceNow. Item that match the patterns are excluded from the index.       Items that don't match the patterns are included in the index. If an item matches both       an inclusion and exclusion pattern, the exclusion pattern takes precedence and the item       isn't included in the index.
    /// 
    /// The regex is applied to the field specified in the       PatternTargetField.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    pub exclude_attachment_file_patterns: Option<Vec<String>>,


    /// 
    /// The name of the ServiceNow field that is mapped to the index document title       field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<String>,


    /// 
    /// The name of the ServiceNow field that is mapped to the index document contents field       in the Amazon Kendra index.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,


    /// 
    /// A list of regular expression patterns to include certain attachments of knowledge       articles in your ServiceNow. Item that match the patterns are included in the index.       Items that don't match the patterns are excluded from the index. If an item matches both       an inclusion and exclusion pattern, the exclusion pattern takes precedence and the item       isn't included in the index.
    /// 
    /// The regex is applied to the field specified in the       PatternTargetField.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    pub include_attachment_file_patterns: Option<Vec<String>>,

}


/// Provides the configuration information to connect to websites that require basic user       authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebCrawlerBasicAuthentication {


    /// 
    /// The name of the website host you want to connect to using authentication       credentials.
    /// 
    /// For example, the host name of https://a.example.com/page1.html is       "a.example.com".
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 253
    ///
    /// Pattern: ([^\s]*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// Your secret ARN, which you can create in AWS Secrets Manager
    /// 
    /// You use a secret if basic authentication credentials are required to connect to a       website. The secret stores your credentials of user name and password.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    pub credentials: String,


    /// 
    /// The port number of the website host you want to connect to using authentication       credentials.
    /// 
    /// For example, the port for https://a.example.com/page1.html is 443, the standard port       for HTTPS.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: i64,

}


/// Configuration of the page settings for the Confluence data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluencePageConfiguration {


    /// 
    /// Maps attributes or field names of Confluence pages to Amazon Kendra index field       names. To create custom fields, use the UpdateIndex API before you map to       Confluence fields. For more information, see Mapping data source fields. The       Confluence data source field names must exist in your Confluence custom metadata.
    /// 
    /// If you specify the PageFieldMappings parameter, you must specify at least       one field mapping.
    /// 
    /// Required: No
    ///
    /// Type: List of ConfluencePageToIndexFieldMapping
    ///
    /// Maximum: 12
    ///
    /// Update requires: No interruption
    #[serde(rename = "PageFieldMappings")]
    pub page_field_mappings: Option<Vec<ConfluencePageToIndexFieldMapping>>,

}


/// Provides the configuration information for processing attachments to Salesforce       standard objects.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceStandardObjectAttachmentConfiguration {


    /// 
    /// One or more objects that map fields in attachments to Amazon Kendra index       fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// The name of the field used for the document title.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<String>,

}


/// Maps attributes or field names of Confluence blog to Amazon Kendra index field       names. To create custom fields, use the UpdateIndex API before you map to       Confluence fields. For more information, see Mapping data source fields. The       Confluence data source field names must exist in your Confluence custom metadata.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluenceBlogToIndexFieldMapping {


    /// 
    /// The name of the field in the data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AUTHOR | DISPLAY_URL | ITEM_TYPE | LABELS | PUBLISH_DATE | SPACE_KEY | SPACE_NAME | URL | VERSION
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: String,


    /// 
    /// The format for date fields in the data source. If the field specified in         DataSourceFieldName is a date field you must specify the date format.       If the field is not a date field, an exception is thrown.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Pattern: ^(?!\s).*(?<!\s)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<String>,


    /// 
    /// The name of the index field to map to the Confluence data source field. The index       field type must match the Confluence field type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 30
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: String,

}


/// Provides the configuration information for the knowledge article types that Amazon Kendra indexes. Amazon Kendra indexes standard knowledge articles and the       standard fields of knowledge articles, or the custom fields of custom knowledge       articles, but not both
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceKnowledgeArticleConfiguration {


    /// 
    /// Configuration information for standard Salesforce knowledge articles.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceStandardKnowledgeArticleTypeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StandardKnowledgeArticleTypeConfiguration")]
    pub standard_knowledge_article_type_configuration: Option<SalesforceStandardKnowledgeArticleTypeConfiguration>,


    /// 
    /// Configuration information for custom Salesforce knowledge articles.
    /// 
    /// Required: No
    ///
    /// Type: List of SalesforceCustomKnowledgeArticleTypeConfiguration
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomKnowledgeArticleTypeConfigurations")]
    pub custom_knowledge_article_type_configurations: Option<Vec<SalesforceCustomKnowledgeArticleTypeConfiguration>>,


    /// 
    /// Specifies the document states that should be included when Amazon Kendra indexes       knowledge articles. You must specify at least one state.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedStates")]
    pub included_states: Vec<String>,

}


/// The value of a document attribute. You can only provide one value for a document       attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentAttributeValue {


    /// 
    /// A date expressed as an ISO 8601 string.
    /// 
    /// It is important for the time zone to be included in the ISO 8601 date-time format. For       example, 2012-03-25T12:30:10+01:00 is the ISO 8601 date-time format for March 25th 2012       at 12:30PM (plus 10 seconds) in Central European Time.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateValue")]
    pub date_value: Option<String>,


    /// 
    /// A long integer value.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "LongValue")]
    pub long_value: Option<i64>,


    /// 
    /// A list of strings. The default maximum length or number of strings is 10.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringListValue")]
    pub string_list_value: Option<Vec<String>>,


    /// 
    /// A string, such as "department".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,

}


/// Provides information about how Amazon Kendra should use the columns of a database       in an index.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColumnConfiguration {


    /// 
    /// The column that provides the document's identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentIdColumnName")]
    pub document_id_column_name: String,


    /// 
    /// An array of objects that map database column names to the       corresponding fields in an index. You must first create the fields       in the index using the UpdateIndex       operation.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// One to five columns that indicate when a document in the database has changed.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChangeDetectingColumns")]
    pub change_detecting_columns: Vec<String>,


    /// 
    /// The column that contains the title of the document.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleColumnName")]
    pub document_title_column_name: Option<String>,


    /// 
    /// The column that contains the contents of the document.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentDataColumnName")]
    pub document_data_column_name: String,

}


/// Maps attributes or field names of Confluence pages to Amazon Kendra index field       names. To create custom fields, use the UpdateIndex API before you map to       Confluence fields. For more information, see Mapping data source fields. The       Confluence data source field names must exist in your Confluence custom metadata.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluencePageToIndexFieldMapping {


    /// 
    /// The name of the index field to map to the Confluence data source field. The index       field type must match the Confluence field type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 30
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: String,


    /// 
    /// The name of the field in the data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AUTHOR | CONTENT_STATUS | CREATED_DATE | DISPLAY_URL | ITEM_TYPE | LABELS | MODIFIED_DATE | PARENT_ID | SPACE_KEY | SPACE_NAME | URL | VERSION
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: String,


    /// 
    /// The format for date fields in the data source. If the field specified in         DataSourceFieldName is a date field you must specify the date format.       If the field is not a date field, an exception is thrown.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Pattern: ^(?!\s).*(?<!\s)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<String>,

}


/// Specifies the seed or starting point URLs of the       websites or the sitemap URLs of the websites you want to crawl.
///
/// You can include website subdomains. You can list up to 100 seed       URLs and up to three sitemap URLs.
///
/// You can only crawl websites that use the secure communication protocol,       Hypertext Transfer Protocol Secure (HTTPS). If you receive an error when       crawling a website, it could be that the website is blocked from crawling.
///
/// When selecting websites to index, you must adhere to       the Amazon Acceptable Use Policy       and all other Amazon terms. Remember that you must only use the Amazon Kendra       web crawler to index your own webpages, or webpages that you have       authorization to index.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebCrawlerUrls {


    /// 
    /// Configuration of the seed or starting point URLs of the websites you want to       crawl.
    /// 
    /// You can choose to crawl only the website host names, or the website host names with       subdomains, or the website host names with subdomains and other domains that the       web pages link to.
    /// 
    /// You can list up to 100 seed URLs.
    /// 
    /// Required: No
    ///
    /// Type: WebCrawlerSeedUrlConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SeedUrlConfiguration")]
    pub seed_url_configuration: Option<WebCrawlerSeedUrlConfiguration>,


    /// 
    /// Configuration of the sitemap URLs of the websites you want to crawl.
    /// 
    /// Only URLs belonging to the same website host names are crawled. You can list up to       three sitemap URLs.
    /// 
    /// Required: No
    ///
    /// Type: WebCrawlerSiteMapsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SiteMapsConfiguration")]
    pub site_maps_configuration: Option<WebCrawlerSiteMapsConfiguration>,

}


/// Provides the configuration information to connect to Salesforce as your data       source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceConfiguration {


    /// 
    /// A list of regular expression patterns to exclude certain documents in your Salesforce.       Documents that match the patterns are excluded from the index. Documents that don't       match the patterns are included in the index. If a document matches both an inclusion       and exclusion pattern, the exclusion pattern takes precedence and the document isn't       included in the index.
    /// 
    /// The pattern is applied to the name of the attached file.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    pub exclude_attachment_file_patterns: Option<Vec<String>>,


    /// 
    /// Configuration information for processing attachments to Salesforce standard objects.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceStandardObjectAttachmentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StandardObjectAttachmentConfiguration")]
    pub standard_object_attachment_configuration: Option<SalesforceStandardObjectAttachmentConfiguration>,


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Secrets Managersecret that contains the       key/value pairs required to connect to your Salesforce instance. The secret must contain       a JSON structure with the following keys:
    /// 
    /// authenticationUrl - The OAUTH endpoint that Amazon Kendra connects to get           an OAUTH token.               consumerKey - The application public key generated when you created your           Salesforce application.               consumerSecret - The application private key generated when you created your           Salesforce application.               password - The password associated with the user logging in to the Salesforce           instance.               securityToken - The token associated with the user logging in to the           Salesforce instance.               username - The user name of the user logging in to the Salesforce           instance.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,


    /// 
    /// Configuration information for the knowledge article types that Amazon Kendra       indexes. Amazon Kendra indexes standard knowledge articles and the standard fields       of knowledge articles, or the custom fields of custom knowledge articles, but not       both.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceKnowledgeArticleConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "KnowledgeArticleConfiguration")]
    pub knowledge_article_configuration: Option<SalesforceKnowledgeArticleConfiguration>,


    /// 
    /// Indicates whether Amazon Kendra should index attachments to Salesforce       objects.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,


    /// 
    /// Configuration of the Salesforce standard objects that Amazon Kendra       indexes.
    /// 
    /// Required: No
    ///
    /// Type: List of SalesforceStandardObjectConfiguration
    ///
    /// Maximum: 17
    ///
    /// Update requires: No interruption
    #[serde(rename = "StandardObjectConfigurations")]
    pub standard_object_configurations: Option<Vec<SalesforceStandardObjectConfiguration>>,


    /// 
    /// The instance URL for the Salesforce site that you want to index.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^(https?|ftp|file):\/\/([^\s]*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerUrl")]
    pub server_url: String,


    /// 
    /// Configuration information for Salesforce chatter feeds.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceChatterFeedConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChatterFeedConfiguration")]
    pub chatter_feed_configuration: Option<SalesforceChatterFeedConfiguration>,


    /// 
    /// A list of regular expression patterns to include certain documents in your Salesforce.       Documents that match the patterns are included in the index. Documents that don't match       the patterns are excluded from the index. If a document matches both an inclusion and       exclusion pattern, the exclusion pattern takes precedence and the document isn't       included in the index.
    /// 
    /// The pattern is applied to the name of the attached file.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    pub include_attachment_file_patterns: Option<Vec<String>>,

}


/// The target document attribute or metadata field you want to alter when ingesting       documents into Amazon Kendra.
///
/// For example, you can delete customer identification numbers associated with the       documents, stored in the document metadata field called 'Customer_ID'. You set the       target key as 'Customer_ID' and the deletion flag to TRUE. This removes all       customer ID values in the field 'Customer_ID'. This would scrub personally identifiable       information from each document's metadata.
///
/// Amazon Kendra cannot create a target field if it has not already been created as       an index field. After you create your index field, you can create a document metadata       field using DocumentAttributeTarget. Amazon Kendra then will map your       newly created metadata field to your index field.
///
/// You can also use this with DocumentAttributeCondition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentAttributeTarget {


    /// 
    /// TRUE to delete the existing target value for your specified target       attribute key. You cannot create a target value and set this to TRUE. To       create a target value (TargetDocumentAttributeValue), set this to         FALSE.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetDocumentAttributeValueDeletion")]
    pub target_document_attribute_value_deletion: Option<bool>,


    /// 
    /// The target value you want to create for the target attribute.
    /// 
    /// For example, 'Finance' could be the target value for the target attribute key       'Department'.
    /// 
    /// Required: No
    ///
    /// Type: DocumentAttributeValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetDocumentAttributeValue")]
    pub target_document_attribute_value: Option<DocumentAttributeValue>,


    /// 
    /// The identifier of the target document attribute or metadata field.
    /// 
    /// For example, 'Department' could be an identifier for the target attribute or metadata       field that includes the department names associated with the documents.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: [a-zA-Z0-9_][a-zA-Z0-9_-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetDocumentAttributeKey")]
    pub target_document_attribute_key: String,

}


/// Provides the configuration information for invoking a Lambda function in AWS Lambda to alter document metadata and content when ingesting documents into         Amazon Kendra. You can configure your Lambda function using PreExtractionHookConfiguration if you want to apply advanced alterations on       the original or raw documents. If you want to apply advanced alterations on the Amazon Kendra structured documents, you must configure your Lambda function using         PostExtractionHookConfiguration. You can only invoke one Lambda function.       However, this function can invoke other functions it requires.
///
/// For more information, see Customizing document metadata         during the ingestion process.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HookConfiguration {


    /// 
    /// The condition used for when a Lambda function should be invoked.
    /// 
    /// For example, you can specify a condition that if there are empty date-time values,       then Amazon Kendra should invoke a function that inserts the current       date-time.
    /// 
    /// Required: No
    ///
    /// Type: DocumentAttributeCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvocationCondition")]
    pub invocation_condition: Option<DocumentAttributeCondition>,


    /// 
    /// Stores the original, raw documents or the structured, parsed documents before and       after altering them. For more information, see Data contracts for Lambda functions.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,


    /// 
    /// The Amazon Resource Name (ARN) of a role with permission to run a Lambda function       during ingestion. For more information, see IAM roles for Amazon Kendra.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: /arn:aws[a-zA-Z-]*:lambda:[a-z]+-[a-z]+-[0-9]:[0-9]{12}:function:[a-zA-Z0-9-_]+(\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})?(:[a-zA-Z0-9-_]+)?/
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: String,

}


/// Provides the configuration information to connect to OneDrive as your data       source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OneDriveConfiguration {


    /// 
    /// A list of user accounts whose documents should be indexed.
    /// 
    /// Required: Yes
    ///
    /// Type: OneDriveUsers
    ///
    /// Update requires: No interruption
    #[serde(rename = "OneDriveUsers")]
    pub one_drive_users: OneDriveUsers,


    /// 
    /// A list of regular expression patterns to exclude certain documents in your OneDrive.       Documents that match the patterns are excluded from the index. Documents that don't       match the patterns are included in the index. If a document matches both an inclusion       and exclusion pattern, the exclusion pattern takes precedence and the document isn't       included in the index.
    /// 
    /// The pattern is applied to the file name.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<Vec<String>>,


    /// 
    /// A list of DataSourceToIndexFieldMapping objects that map OneDrive data       source attributes or field names to Amazon Kendra index field names. To create       custom fields, use the UpdateIndex API before you map to OneDrive fields.       For more information, see Mapping data source fields. The       OneDrive data source field names must exist in your OneDrive custom metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// The Azure Active Directory domain of the organization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-z]{2,}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "TenantDomain")]
    pub tenant_domain: String,


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Secrets Manager secret       that contains the user name and password to connect to OneDrive. The       user name should be the application ID for the OneDrive       application, and the password is the application key for the       OneDrive application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,


    /// 
    /// A list of regular expression patterns to include certain documents in your OneDrive.       Documents that match the patterns are included in the index. Documents that don't match       the patterns are excluded from the index. If a document matches both an inclusion and       exclusion pattern, the exclusion pattern takes precedence and the document isn't       included in the index.
    /// 
    /// The pattern is applied to the file name.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<Vec<String>>,


    /// 
    /// TRUE to disable local groups information.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableLocalGroups")]
    pub disable_local_groups: Option<bool>,

}


/// Document metadata files that contain information such as the document access control       information, source URI, document author, and custom attributes. Each metadata file       contains metadata about a single document.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentsMetadataConfiguration {


    /// 
    /// A prefix used to filter metadata configuration files in the AWS S3       bucket. The S3 bucket might contain multiple metadata files. Use S3Prefix       to include only the desired metadata files.
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
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: Option<String>,

}


/// Specifies configuration information for indexing a single standard       object.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceStandardObjectConfiguration {


    /// 
    /// The name of the field in the standard object table that contains the document       contents.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,


    /// 
    /// The name of the field in the standard object table that contains       the document title.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<String>,


    /// 
    /// The name of the standard object.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ACCOUNT | CAMPAIGN | CASE | CONTACT | CONTRACT | DOCUMENT | GROUP | IDEA | LEAD | OPPORTUNITY | PARTNER | PRICEBOOK | PRODUCT | PROFILE | SOLUTION | TASK | USER
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Maps attributes or field names of the standard object to Amazon Kendra index       field names. To create custom fields, use the UpdateIndex API before you       map to Salesforce fields. For more information, see Mapping data source fields. The       Salesforce data source field names must exist in your Salesforce custom metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,

}


/// The configuration information for syncing a Salesforce chatter feed. The contents of       the object comes from the Salesforce FeedItem table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceChatterFeedConfiguration {


    /// 
    /// The name of the column in the Salesforce FeedItem table that contains the title of the       document. This is typically the Title column.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<String>,


    /// 
    /// The name of the column in the Salesforce FeedItem table that contains the content to       index. Typically this is the Body column.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,


    /// 
    /// Maps fields from a Salesforce chatter feed into Amazon Kendra index       fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// Filters the documents in the feed based on status of the user. When you specify         ACTIVE_USERS only documents from users who have an active account are       indexed. When you specify STANDARD_USER only documents for Salesforce       standard users are documented. You can specify both.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeFilterTypes")]
    pub include_filter_types: Option<Vec<String>>,

}


/// Provides information about the column that should be used for filtering the query       response by groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AclConfiguration {


    /// 
    /// A list of groups, separated by semi-colons, that filters a query       response based on user context. The document is only returned to       users that are in one of the groups specified in the         UserContext field of the Query operation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedGroupsColumnName")]
    pub allowed_groups_column_name: String,

}


/// The condition used for the target document attribute or metadata field when ingesting       documents into Amazon Kendra. You use this with DocumentAttributeTarget to         apply the condition.
///
/// For example, you can create the 'Department' target field and have it prefill       department names associated with the documents based on information in the 'Source_URI'       field. Set the condition that if the 'Source_URI' field contains 'financial' in its URI       value, then prefill the target field 'Department' with the target value 'Finance' for       the document.
///
/// Amazon Kendra cannot create a target field if it has not already been created as       an index field. After you create your index field, you can create a document metadata       field using DocumentAttributeTarget. Amazon Kendra then will map your       newly created metadata field to your index field.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentAttributeCondition {


    /// 
    /// The identifier of the document attribute used for the condition.
    /// 
    /// For example, 'Source_URI' could be an identifier for the attribute or metadata field       that contains source URIs associated with the documents.
    /// 
    /// Amazon Kendra currently does not support _document_body as an       attribute key used for the condition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: [a-zA-Z0-9_][a-zA-Z0-9_-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionDocumentAttributeKey")]
    pub condition_document_attribute_key: String,


    /// 
    /// The value used by the operator.
    /// 
    /// For example, you can specify the value 'financial' for strings in the 'Source_URI'       field that partially match or contain this value.
    /// 
    /// Required: No
    ///
    /// Type: DocumentAttributeValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionOnValue")]
    pub condition_on_value: Option<DocumentAttributeValue>,


    /// 
    /// The condition operator.
    /// 
    /// For example, you can use 'Contains' to partially match a string.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BeginsWith | Contains | Equals | Exists | GreaterThan | GreaterThanOrEquals | LessThan | LessThanOrEquals | NotContains | NotEquals | NotExists
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operator")]
    pub operator: String,

}


/// Provides the configuration information of the sitemap URLs to crawl.
///
/// When selecting websites to index, you must adhere to       the Amazon Acceptable Use Policy       and all other Amazon terms. Remember that you must only use the Amazon Kendra web       crawler to index your own webpages, or webpages that you have authorization       to index.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebCrawlerSiteMapsConfiguration {


    /// 
    /// The list of sitemap URLs of the websites you want to crawl.
    /// 
    /// The list can include a maximum of three sitemap URLs.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "SiteMaps")]
    pub site_maps: Vec<String>,

}


/// Provides the configuration information for crawling service catalog items in the       ServiceNow site
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceNowServiceCatalogConfiguration {


    /// 
    /// TRUE to index attachments to service catalog items.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,


    /// 
    /// The name of the ServiceNow field that is mapped to the index document title       field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<String>,


    /// 
    /// A list of regular expression patterns to exclude certain attachments of catalogs in       your ServiceNow. Item that match the patterns are excluded from the index. Items that       don't match the patterns are included in the index. If an item matches both an inclusion       and exclusion pattern, the exclusion pattern takes precedence and the item isn't       included in the index.
    /// 
    /// The regex is applied to the file name of the attachment.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    pub exclude_attachment_file_patterns: Option<Vec<String>>,


    /// 
    /// The name of the ServiceNow field that is mapped to the index document contents field       in the Amazon Kendra index.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,


    /// 
    /// A list of regular expression patterns to include certain attachments of catalogs in       your ServiceNow. Item that match the patterns are included in the index. Items that       don't match the patterns are excluded from the index. If an item matches both an       inclusion and exclusion pattern, the exclusion pattern takes precedence and the item       isn't included in the index.
    /// 
    /// The regex is applied to the file name of the attachment.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    pub include_attachment_file_patterns: Option<Vec<String>>,


    /// 
    /// Maps attributes or field names of catalogs to Amazon Kendra index field names. To       create custom fields, use the UpdateIndex API before you map to ServiceNow       fields. For more information, see Mapping data source fields. The       ServiceNow data source field names must exist in your ServiceNow custom metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,

}


/// Provides the configuration information to connect to Amazon WorkDocs       as your data source.
///
/// Amazon WorkDocs connector is available in Oregon, North Virginia, Sydney, Singapore and Ireland       regions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WorkDocsConfiguration {


    /// 
    /// A list of regular expression patterns to exclude certain files       in your Amazon WorkDocs site repository. Files that match the patterns       are excluded from the index. Files that don’t match the patterns       are included in the index. If a file matches both an inclusion and exclusion       pattern, the exclusion pattern takes precedence and the file isn't included       in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<Vec<String>>,


    /// 
    /// A list of regular expression patterns to include certain files       in your Amazon WorkDocs site repository. Files that match the patterns       are included in the index. Files that don't match the patterns are       excluded from the index. If a file matches both an inclusion and exclusion       pattern, the exclusion pattern takes precedence and the file isn't included       in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<Vec<String>>,


    /// 
    /// TRUE to include comments on documents       in your index. Including comments in your index means each comment       is a document that can be searched on.
    /// 
    /// The default is set to FALSE.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlComments")]
    pub crawl_comments: Option<bool>,


    /// 
    /// The identifier of the directory corresponding to your       Amazon WorkDocs site repository.
    /// 
    /// You can find the organization ID in the       AWS Directory Service by going to       Active Directory, then       Directories. Your Amazon WorkDocs site directory has an       ID, which is the organization ID. You can also set up a new Amazon WorkDocs       directory in the AWS Directory Service console and enable a Amazon WorkDocs site       for the directory in the Amazon WorkDocs console.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: d-[0-9a-fA-F]{10}
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,


    /// 
    /// A list of DataSourceToIndexFieldMapping objects that       map Amazon WorkDocs data source attributes or field names to Amazon Kendra       index field names. To create custom fields, use the       UpdateIndex API before you map to Amazon WorkDocs fields.       For more information, see Mapping         data source fields. The Amazon WorkDocs data source field names       must exist in your Amazon WorkDocs custom metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// TRUE to use the Amazon WorkDocs change log to determine       which documents require updating in the index. Depending on the change log's       size, it may take longer for Amazon Kendra to use the change log than to       scan all of your documents in Amazon WorkDocs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseChangeLog")]
    pub use_change_log: Option<bool>,

}


/// Specifies access control list files for the documents in a data       source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessControlListConfiguration {


    /// 
    /// Path to the AWS S3 bucket that contains the access control list       files.
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
    #[serde(rename = "KeyPath")]
    pub key_path: Option<String>,

}


/// Configuration of blog settings for the Confluence data source. Blogs are always       indexed unless filtered from the index by the ExclusionPatterns or         InclusionPatterns fields in the ConfluenceConfiguration       object.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluenceBlogConfiguration {


    /// 
    /// Maps attributes or field names of Confluence blogs to Amazon Kendra index field       names. To create custom fields, use the UpdateIndex API before you map to       Confluence fields. For more information, see Mapping data source fields. The       Confluence data source field names must exist in your Confluence custom metadata.
    /// 
    /// If you specify the BlogFieldMappings parameter, you must specify at least       one field mapping.
    /// 
    /// Required: No
    ///
    /// Type: List of ConfluenceBlogToIndexFieldMapping
    ///
    /// Maximum: 9
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlogFieldMappings")]
    pub blog_field_mappings: Option<Vec<ConfluenceBlogToIndexFieldMapping>>,

}


/// Information required to find a specific file in an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Path {


    /// 
    /// The name of the file.
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
    /// The name of the S3 bucket that contains the file.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,

}


/// Provides information that configures Amazon Kendra to use a SQL       database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SqlConfiguration {


    /// 
    /// Determines whether Amazon Kendra encloses SQL identifiers for       tables and column names in double quotes (") when making a database       query. You can set the value to DOUBLE_QUOTES or         NONE.
    /// 
    /// By default, Amazon Kendra passes SQL identifiers the way that       they are entered into the data source configuration. It does not       change the case of identifiers or enclose them in quotes.
    /// 
    /// PostgreSQL internally converts uppercase characters to lower case       characters in identifiers unless they are quoted. Choosing this       option encloses identifiers in quotes so that PostgreSQL does not       convert the character's case.
    /// 
    /// For MySQL databases, you must enable the ansi_quotes option when       you set this field to DOUBLE_QUOTES.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryIdentifiersEnclosingOption")]
    pub query_identifiers_enclosing_option: Option<String>,

}


/// Provides the configuration information for an Amazon Kendra data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceConfiguration {


    /// 
    /// Provides the configuration information to connect to Amazon WorkDocs as your data    source.
    /// 
    /// Required: No
    ///
    /// Type: WorkDocsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkDocsConfiguration")]
    pub work_docs_configuration: Option<WorkDocsConfiguration>,


    /// 
    /// Provides the configuration information to connect to an Amazon S3 bucket as your    data source.
    /// 
    /// Required: No
    ///
    /// Type: S3DataSourceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: Option<S3DataSourceConfiguration>,


    /// 
    /// Provides the configuration information to connect to Salesforce as your data    source.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SalesforceConfiguration")]
    pub salesforce_configuration: Option<SalesforceConfiguration>,


    /// 
    /// Provides the configuration information required for Amazon Kendra       Web Crawler.
    /// 
    /// Required: No
    ///
    /// Type: WebCrawlerConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebCrawlerConfiguration")]
    pub web_crawler_configuration: Option<WebCrawlerConfiguration>,


    /// 
    /// Provides the configuration information to connect to Microsoft OneDrive as your data    source.
    /// 
    /// Required: No
    ///
    /// Type: OneDriveConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "OneDriveConfiguration")]
    pub one_drive_configuration: Option<OneDriveConfiguration>,


    /// 
    /// Provides the configuration information to connect to Google Drive as your data    source.
    /// 
    /// Required: No
    ///
    /// Type: GoogleDriveConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "GoogleDriveConfiguration")]
    pub google_drive_configuration: Option<GoogleDriveConfiguration>,


    /// 
    /// Provides the configuration information to connect to Confluence as your data    source.
    /// 
    /// Required: No
    ///
    /// Type: ConfluenceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfluenceConfiguration")]
    pub confluence_configuration: Option<ConfluenceConfiguration>,


    /// 
    /// Provides the configuration information to connect to Microsoft SharePoint as your data    source.
    /// 
    /// Required: No
    ///
    /// Type: SharePointConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SharePointConfiguration")]
    pub share_point_configuration: Option<SharePointConfiguration>,


    /// 
    /// Provides the configuration information to connect to a database as your data    source.
    /// 
    /// Required: No
    ///
    /// Type: DatabaseConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseConfiguration")]
    pub database_configuration: Option<DatabaseConfiguration>,


    /// 
    /// Provides the configuration information to connect to ServiceNow as your data    source.
    /// 
    /// Required: No
    ///
    /// Type: ServiceNowConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNowConfiguration")]
    pub service_now_configuration: Option<ServiceNowConfiguration>,

}


/// Provides the configuration information required for Amazon Kendra       Web Crawler.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebCrawlerConfiguration {


    /// 
    /// A list of regular expression patterns to include certain URLs to crawl. URLs that       match the patterns are included in the index. URLs that don't match the patterns are       excluded from the index. If a URL matches both an inclusion and exclusion pattern, the       exclusion pattern takes precedence and the URL file isn't included in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "UrlInclusionPatterns")]
    pub url_inclusion_patterns: Option<Vec<String>>,


    /// 
    /// The maximum size (in MB) of a web page or attachment to crawl.
    /// 
    /// Files larger than this size (in MB) are skipped/not crawled.
    /// 
    /// The default maximum size of a web page or attachment is set to 50 MB.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxContentSizePerPageInMegaBytes")]
    pub max_content_size_per_page_in_mega_bytes: Option<f64>,


    /// 
    /// Configuration information required to connect to websites using authentication.
    /// 
    /// You can connect to websites using basic authentication of user name and password. You       use a secret in AWS Secrets Manager to       store your authentication credentials.
    /// 
    /// You must provide the website host name and port number. For example, the host name of       https://a.example.com/page1.html is "a.example.com" and the port is 443, the standard       port for HTTPS.
    /// 
    /// Required: No
    ///
    /// Type: WebCrawlerAuthenticationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationConfiguration")]
    pub authentication_configuration: Option<WebCrawlerAuthenticationConfiguration>,


    /// 
    /// Specifies the number of levels in a website that you want to crawl.
    /// 
    /// The first level begins from the website seed or starting point URL. For example, if a       website has three levels—index level (the seed in this example), sections level, and       subsections level—and you are only interested in crawling information up to the       sections level (levels 0-1), you can set your depth to 1.
    /// 
    /// The default crawl depth is set to 2.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlDepth")]
    pub crawl_depth: Option<i64>,


    /// 
    /// The maximum number of URLs on a web page to include when crawling a website. This       number is per web page.
    /// 
    /// As a website’s web pages are crawled, any URLs the web pages link to are also crawled.       URLs on a web page are crawled in order of appearance.
    /// 
    /// The default maximum links per page is 100.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxLinksPerPage")]
    pub max_links_per_page: Option<i64>,


    /// 
    /// A list of regular expression patterns to exclude certain URLs to crawl. URLs that       match the patterns are excluded from the index. URLs that don't match the patterns are       included in the index. If a URL matches both an inclusion and exclusion pattern, the       exclusion pattern takes precedence and the URL file isn't included in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "UrlExclusionPatterns")]
    pub url_exclusion_patterns: Option<Vec<String>>,


    /// 
    /// The maximum number of URLs crawled per website host per minute.
    /// 
    /// A minimum of one URL is required.
    /// 
    /// The default maximum number of URLs crawled per website host per minute is 300.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxUrlsPerMinuteCrawlRate")]
    pub max_urls_per_minute_crawl_rate: Option<i64>,


    /// 
    /// Configuration information required to connect to your internal websites via a web       proxy.
    /// 
    /// You must provide the website host name and port number. For example, the host name of       https://a.example.com/page1.html is "a.example.com" and the port is 443, the standard       port for HTTPS.
    /// 
    /// Web proxy credentials are optional and you can use them to connect to a web proxy       server that requires basic authentication. To store web proxy credentials, you use a       secret in AWS Secrets Manager.
    /// 
    /// Required: No
    ///
    /// Type: ProxyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProxyConfiguration")]
    pub proxy_configuration: Option<ProxyConfiguration>,


    /// 
    /// Specifies the seed or starting point URLs of the       websites or the sitemap URLs of the websites you want to crawl.
    /// 
    /// You can include website subdomains. You can list up to 100 seed       URLs and up to three sitemap URLs.
    /// 
    /// You can only crawl websites that use the secure communication protocol,       Hypertext Transfer Protocol Secure (HTTPS). If you receive an error when       crawling a website, it could be that the website is blocked from crawling.
    /// 
    /// When selecting websites to index, you must adhere to       the Amazon Acceptable Use Policy       and all other Amazon terms. Remember that you must only use Amazon Kendra       Web Crawler to index your own webpages, or webpages that you have       authorization to index.
    /// 
    /// Required: Yes
    ///
    /// Type: WebCrawlerUrls
    ///
    /// Update requires: No interruption
    #[serde(rename = "Urls")]
    pub urls: WebCrawlerUrls,

}


/// Provides the configuration information to connect to Google Drive as your data       source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GoogleDriveConfiguration {


    /// 
    /// Maps Google Drive data source attributes or field names to Amazon Kendra index       field names. To create custom fields, use the UpdateIndex API before you       map to Google Drive fields. For more information, see Mapping data source fields. The       Google Drive data source field names must exist in your Google Drive custom       metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// A list of email addresses of the users. Documents owned by these users are excluded       from the index. Documents shared with excluded users are indexed unless they are       excluded in another way.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeUserAccounts")]
    pub exclude_user_accounts: Option<Vec<String>>,


    /// 
    /// The Amazon Resource Name (ARN) of a AWS Secrets Managersecret that contains the       credentials required to connect to Google Drive. For more information, see Using a         Google Workspace Drive data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,


    /// 
    /// A list of MIME types to exclude from the index. All documents matching the specified       MIME type are excluded.
    /// 
    /// For a list of MIME types, see Using a         Google Workspace Drive data source.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeMimeTypes")]
    pub exclude_mime_types: Option<Vec<String>>,


    /// 
    /// A list of identifiers or shared drives to exclude from the index. All files and       folders stored on the shared drive are excluded.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeSharedDrives")]
    pub exclude_shared_drives: Option<Vec<String>>,


    /// 
    /// A list of regular expression patterns to exclude certain items in your Google Drive,       including shared drives and users' My Drives. Items that match the patterns are excluded       from the index. Items that don't match the patterns are included in the index. If an       item matches both an inclusion and exclusion pattern, the exclusion pattern takes       precedence and the item isn't included in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<Vec<String>>,


    /// 
    /// A list of regular expression patterns to include certain items in your Google Drive,       including shared drives and users' My Drives. Items that match the patterns are included       in the index. Items that don't match the patterns are excluded from the index. If an       item matches both an inclusion and exclusion pattern, the exclusion pattern takes       precedence and the item isn't included in the index.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<Vec<String>>,

}


/// Provides the configuration information of the seed or starting point URLs to crawl.
///
/// When selecting websites to index, you must adhere to       the Amazon Acceptable Use Policy       and all other Amazon terms. Remember that you must only use the Amazon Kendra web       crawler to index your own webpages, or webpages that you have authorization       to index.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebCrawlerSeedUrlConfiguration {


    /// 
    /// You can choose one of the following modes:
    /// 
    /// HOST_ONLY – crawl only the website host names. For           example, if the seed URL is "abc.example.com", then only URLs with host name           "abc.example.com" are crawled.                        SUBDOMAINS – crawl the website host names with subdomains.           For example, if the seed URL is "abc.example.com", then "a.abc.example.com" and           "b.abc.example.com" are also crawled.                        EVERYTHING – crawl the website host names with subdomains           and other domains that the web pages link to.
    /// 
    /// The default mode is set to HOST_ONLY.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EVERYTHING | HOST_ONLY | SUBDOMAINS
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebCrawlerMode")]
    pub web_crawler_mode: Option<String>,


    /// 
    /// The list of seed or starting point URLs of the websites you want to crawl.
    /// 
    /// The list can include a maximum of 100 seed URLs.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SeedUrls")]
    pub seed_urls: Vec<String>,

}


/// Maps attributes or field names of Confluence attachments to Amazon Kendra index       field names. To create custom fields, use the UpdateIndex API before you       map to Confluence fields. For more information, see Mapping data source fields. The       Confuence data source field names must exist in your Confluence custom metadata.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluenceAttachmentToIndexFieldMapping {


    /// 
    /// The name of the field in the data source.
    /// 
    /// You must first create the index field using the UpdateIndex API.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AUTHOR | CONTENT_TYPE | CREATED_DATE | DISPLAY_URL | FILE_SIZE | ITEM_TYPE | PARENT_ID | SPACE_KEY | SPACE_NAME | URL | VERSION
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: String,


    /// 
    /// The format for date fields in the data source. If the field specified in         DataSourceFieldName is a date field you must specify the date format.       If the field is not a date field, an exception is thrown.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Pattern: ^(?!\s).*(?<!\s)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<String>,


    /// 
    /// The name of the index field to map to the Confluence data source field. The index       field type must match the Confluence field type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 30
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: String,

}


/// User accounts whose documents should be indexed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OneDriveUsers {


    /// 
    /// A list of users whose documents should be indexed. Specify the user names in email       format, for example, username@tenantdomain. If you need to index the       documents of more than 100 users, use the OneDriveUserS3Path field to       specify the location of a file containing a list of users.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "OneDriveUserList")]
    pub one_drive_user_list: Option<Vec<String>>,


    /// 
    /// The S3 bucket location of a file containing a list of users whose documents should be       indexed.
    /// 
    /// Required: No
    ///
    /// Type: S3Path
    ///
    /// Update requires: No interruption
    #[serde(rename = "OneDriveUserS3Path")]
    pub one_drive_user_s3_path: Option<S3Path>,

}


/// Provides the configuration information to connect to websites that require 		  user authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebCrawlerAuthenticationConfiguration {


    /// 
    /// The list of configuration information that's required to connect to and crawl a       website host using basic authentication credentials.
    /// 
    /// The list includes the name and port number of the website host.
    /// 
    /// Required: No
    ///
    /// Type: List of WebCrawlerBasicAuthentication
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuthentication")]
    pub basic_authentication: Option<Vec<WebCrawlerBasicAuthentication>>,

}


/// Provides the configuration information to connect to Microsoft SharePoint as your data       source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SharePointConfiguration {


    /// 
    /// The version of Microsoft SharePoint that you use.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SHAREPOINT_2013 | SHAREPOINT_2016 | SHAREPOINT_2019 | SHAREPOINT_ONLINE
    ///
    /// Update requires: No interruption
    #[serde(rename = "SharePointVersion")]
    pub share_point_version: String,


    /// 
    /// Information required to find a specific file in an Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: S3Path
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslCertificateS3Path")]
    pub ssl_certificate_s3_path: Option<S3Path>,


    /// 
    /// A list of regular expression patterns. Documents that match the       patterns are excluded from the index. Documents that don't match the       patterns are included in the index. If a document matches both an       exclusion pattern and an inclusion pattern, the document is not       included in the index.
    /// 
    /// The regex is applied to the display URL of the SharePoint       document.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<Vec<String>>,


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Secrets Manager secret that contains the       user name and password required to connect to the SharePoint instance. For more       information, see Microsoft       SharePoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,


    /// 
    /// TRUE to index document attachments.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,


    /// 
    /// The Microsoft SharePoint site URLs for the documents you want to index.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Urls")]
    pub urls: Vec<String>,


    /// 
    /// A list of regular expression patterns to include certain documents in your SharePoint.       Documents that match the patterns are included in the index. Documents that don't match       the patterns are excluded from the index. If a document matches both an inclusion and       exclusion pattern, the exclusion pattern takes precedence and the document isn't       included in the index.
    /// 
    /// The regex applies to the display URL of the SharePoint document.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<Vec<String>>,


    /// 
    /// TRUE to disable local groups information.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableLocalGroups")]
    pub disable_local_groups: Option<bool>,


    /// Provides information for connecting to an Amazon VPC.
    ///
    /// Required: No
    ///
    /// Type: DataSourceVpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,


    /// 
    /// A list of DataSourceToIndexFieldMapping objects that       map Microsoft SharePoint attributes or fields to Amazon       Kendra index fields. You must first create the index fields using the         UpdateIndex       operation before you map SharePoint attributes. For more       information, see Mapping Data Source         Fields.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// TRUE to use the SharePoint change log to determine which documents       require updating in the index. Depending on the change log's size, it may take longer       for Amazon Kendra to use the change log than to scan all of your documents in       SharePoint.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseChangeLog")]
    pub use_change_log: Option<bool>,


    /// 
    /// The Microsoft SharePoint attribute field that contains the title of the       document.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<String>,

}


/// Provides the configuration information for indexing Salesforce custom articles.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceCustomKnowledgeArticleTypeConfiguration {


    /// 
    /// Maps attributes or field names of the custom knowledge article to Amazon Kendra       index field names. To create custom fields, use the UpdateIndex API before       you map to Salesforce fields. For more information, see Mapping data source fields. The       Salesforce data source field names must exist in your Salesforce custom metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of DataSourceToIndexFieldMapping
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,


    /// 
    /// The name of the configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The name of the field in the custom knowledge article that contains the document       title.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<String>,


    /// 
    /// The name of the field in the custom knowledge article that contains the document data       to index.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: String,

}


/// Maps attributes or field names of Confluence spaces to Amazon Kendra index field       names. To create custom fields, use the UpdateIndex API before you map to       Confluence fields. For more information, see Mapping data source fields. The       Confluence data source field names must exist in your Confluence custom metadata.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfluenceSpaceToIndexFieldMapping {


    /// 
    /// The name of the field in the data source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISPLAY_URL | ITEM_TYPE | SPACE_KEY | URL
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: String,


    /// 
    /// The format for date fields in the data source. If the field specified in         DataSourceFieldName is a date field you must specify the date format.       If the field is not a date field, an exception is thrown.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Pattern: ^(?!\s).*(?<!\s)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<String>,


    /// 
    /// The name of the index field to map to the Confluence data source field. The index       field type must match the Confluence field type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 30
    ///
    /// Pattern: ^\P{C}*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: String,

}


/// Provides the configuration information for altering document metadata and content       during the document ingestion process.
///
/// For more information, see Customizing document metadata         during the ingestion process.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomDocumentEnrichmentConfiguration {


    /// 
    /// Configuration information for invoking a Lambda function in AWS Lambda on       the structured documents with their metadata and text extracted. You can use a Lambda       function to apply advanced logic for creating, modifying, or deleting document metadata       and content. For more information, see Advanced data manipulation.
    /// 
    /// Required: No
    ///
    /// Type: HookConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PostExtractionHookConfiguration")]
    pub post_extraction_hook_configuration: Option<HookConfiguration>,


    /// 
    /// Configuration information to alter document attributes or metadata fields and content       when ingesting documents into Amazon Kendra.
    /// 
    /// Required: No
    ///
    /// Type: List of InlineCustomDocumentEnrichmentConfiguration
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "InlineConfigurations")]
    pub inline_configurations: Option<Vec<InlineCustomDocumentEnrichmentConfiguration>>,


    /// 
    /// The Amazon Resource Name (ARN) of a role with permission to run         PreExtractionHookConfiguration and         PostExtractionHookConfiguration for altering document metadata and       content during the document ingestion process. For more information, see IAM roles for Amazon Kendra.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1284
    ///
    /// Pattern: arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// Configuration information for invoking a Lambda function in AWS Lambda on       the original or raw documents before extracting their metadata and text. You can use a       Lambda function to apply advanced logic for creating, modifying, or deleting document       metadata and content. For more information, see Advanced data manipulation.
    /// 
    /// Required: No
    ///
    /// Type: HookConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreExtractionHookConfiguration")]
    pub pre_extraction_hook_configuration: Option<HookConfiguration>,

}
