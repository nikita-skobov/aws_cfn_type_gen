
pub mod cfn_data_source {

#[derive(serde::Serialize, Default)]
pub struct CfnDataSource {
    /// Description of data source
    #[serde(rename = "Description")]
    pub description: Option<Description>,
    /// Role ARN
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<RoleArn>,
    /// Tags for labeling the data source
    #[serde(rename = "Tags")]
    pub tags: Option<TagList>,
    /// Schedule
    #[serde(rename = "Schedule")]
    pub schedule: Option<Schedule>,
    /// Data source type
    #[serde(rename = "Type")]
    pub ty: Type,
    /// ID of Index
    #[serde(rename = "IndexId")]
    pub index_id: IndexId,
    /// No documentation provided by AWS
    #[serde(rename = "DataSourceConfiguration")]
    pub data_source_configuration: Option<DataSourceConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomDocumentEnrichmentConfiguration")]
    pub custom_document_enrichment_configuration: Option<CustomDocumentEnrichmentConfiguration>,
    /// Name of data source
    #[serde(rename = "Name")]
    pub name: Name,

}


#[derive(serde::Serialize, Default)]
pub struct ExcludeSharedDrivesList {

}
pub type S3ObjectKey = String;
#[derive(serde::Serialize, Default)]
pub struct SharePointConfiguration {
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: SecretArn,
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<DataSourceFieldName>,
    #[serde(rename = "SslCertificateS3Path")]
    pub ssl_certificate_s3_path: Option<S3Path>,
    #[serde(rename = "UseChangeLog")]
    pub use_change_log: Option<bool>,
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,
    #[serde(rename = "Urls")]
    pub urls: Vec<Url>,
    #[serde(rename = "SharePointVersion")]
    pub share_point_version: String,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "DisableLocalGroups")]
    pub disable_local_groups: Option<DisableLocalGroups>,
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,

}

#[derive(serde::Serialize, Default)]
pub struct DatabaseConfiguration {
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,
    #[serde(rename = "ConnectionConfiguration")]
    pub connection_configuration: ConnectionConfiguration,
    #[serde(rename = "ColumnConfiguration")]
    pub column_configuration: ColumnConfiguration,
    #[serde(rename = "DatabaseEngineType")]
    pub database_engine_type: DatabaseEngineType,
    #[serde(rename = "AclConfiguration")]
    pub acl_configuration: Option<AclConfiguration>,
    #[serde(rename = "SqlConfiguration")]
    pub sql_configuration: Option<SqlConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct S3DataSourceConfiguration {
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "AccessControlListConfiguration")]
    pub access_control_list_configuration: Option<AccessControlListConfiguration>,
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "DocumentsMetadataConfiguration")]
    pub documents_metadata_configuration: Option<DocumentsMetadataConfiguration>,
    #[serde(rename = "BucketName")]
    pub bucket_name: S3BucketName,
    #[serde(rename = "InclusionPrefixes")]
    pub inclusion_prefixes: Option<DataSourceInclusionsExclusionsStrings>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSourceToIndexFieldMappingList {

}
pub type DatabaseEngineType = String;pub type S3BucketName = String;pub type DatabasePort = usize;pub type TableName = String;
#[derive(serde::Serialize, Default)]
pub struct DataSourceVpcConfiguration {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

}
pub type DisableLocalGroups = bool;
#[derive(serde::Serialize, Default)]
pub struct ColumnConfiguration {
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "DocumentTitleColumnName")]
    pub document_title_column_name: Option<ColumnName>,
    #[serde(rename = "DocumentIdColumnName")]
    pub document_id_column_name: ColumnName,
    #[serde(rename = "ChangeDetectingColumns")]
    pub change_detecting_columns: ChangeDetectingColumns,
    #[serde(rename = "DocumentDataColumnName")]
    pub document_data_column_name: ColumnName,

}
pub type TenantDomain = String;
#[derive(serde::Serialize, Default)]
pub struct ServiceNowServiceCatalogConfiguration {
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: DataSourceFieldName,
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<DataSourceFieldName>,
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    pub exclude_attachment_file_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    pub include_attachment_file_patterns: Option<DataSourceInclusionsExclusionsStrings>,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceConfiguration {
    #[serde(rename = "StandardObjectConfigurations")]
    pub standard_object_configurations: Option<SalesforceStandardObjectConfigurationList>,
    #[serde(rename = "KnowledgeArticleConfiguration")]
    pub knowledge_article_configuration: Option<SalesforceKnowledgeArticleConfiguration>,
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    pub include_attachment_file_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: SecretArn,
    #[serde(rename = "ServerUrl")]
    pub server_url: Url,
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    pub exclude_attachment_file_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "ChatterFeedConfiguration")]
    pub chatter_feed_configuration: Option<SalesforceChatterFeedConfiguration>,
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,
    #[serde(rename = "StandardObjectAttachmentConfiguration")]
    pub standard_object_attachment_configuration: Option<SalesforceStandardObjectAttachmentConfiguration>,

}
pub type SalesforceChatterFeedIncludeFilterType = String;pub type Type = String;pub type SharedDriveId = String;
#[derive(serde::Serialize, Default)]
pub struct SalesforceChatterFeedConfiguration {
    #[serde(rename = "IncludeFilterTypes")]
    pub include_filter_types: Option<SalesforceChatterFeedIncludeFilterTypes>,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<DataSourceFieldName>,
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: DataSourceFieldName,

}

#[derive(serde::Serialize, Default)]
pub struct DocumentAttributeCondition {
    #[serde(rename = "ConditionDocumentAttributeKey")]
    pub condition_document_attribute_key: DocumentAttributeKey,
    #[serde(rename = "ConditionOnValue")]
    pub condition_on_value: Option<DocumentAttributeValue>,
    #[serde(rename = "Operator")]
    pub operator: ConditionOperator,

}

#[derive(serde::Serialize, Default)]
pub struct ConfluenceConfiguration {
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,
    #[serde(rename = "ServerUrl")]
    pub server_url: Url,
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "Version")]
    pub version: ConfluenceVersion,
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "AttachmentConfiguration")]
    pub attachment_configuration: Option<ConfluenceAttachmentConfiguration>,
    #[serde(rename = "SpaceConfiguration")]
    pub space_configuration: Option<ConfluenceSpaceConfiguration>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: SecretArn,
    #[serde(rename = "BlogConfiguration")]
    pub blog_configuration: Option<ConfluenceBlogConfiguration>,
    #[serde(rename = "PageConfiguration")]
    pub page_configuration: Option<ConfluencePageConfiguration>,

}
pub type DataSourceFieldName = String;
#[derive(serde::Serialize, Default)]
pub struct ConfluencePageFieldMappingsList {

}

#[derive(serde::Serialize, Default)]
pub struct OneDriveConfiguration {
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "TenantDomain")]
    pub tenant_domain: TenantDomain,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "DisableLocalGroups")]
    pub disable_local_groups: Option<DisableLocalGroups>,
    #[serde(rename = "OneDriveUsers")]
    pub one_drive_users: OneDriveUsers,
    #[serde(rename = "SecretArn")]
    pub secret_arn: SecretArn,
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,

}
pub type ConfluenceVersion = String;
#[derive(serde::Serialize, Default)]
pub struct ConfluenceSpaceConfiguration {
    #[serde(rename = "CrawlArchivedSpaces")]
    pub crawl_archived_spaces: Option<bool>,
    #[serde(rename = "CrawlPersonalSpaces")]
    pub crawl_personal_spaces: Option<bool>,
    #[serde(rename = "ExcludeSpaces")]
    pub exclude_spaces: Option<ConfluenceSpaceList>,
    #[serde(rename = "IncludeSpaces")]
    pub include_spaces: Option<ConfluenceSpaceList>,
    #[serde(rename = "SpaceFieldMappings")]
    pub space_field_mappings: Option<ConfluenceSpaceFieldMappingsList>,

}
pub type WebCrawlerSeedUrl = String;
#[derive(serde::Serialize, Default)]
pub struct ConfluencePageConfiguration {
    #[serde(rename = "PageFieldMappings")]
    pub page_field_mappings: Option<ConfluencePageFieldMappingsList>,

}
pub type ServiceNowHostUrl = String;pub type ConfluenceAttachmentFieldName = String;
#[derive(serde::Serialize, Default)]
pub struct WebCrawlerSeedUrlList {

}

#[derive(serde::Serialize, Default)]
pub struct ProxyConfiguration {
    #[serde(rename = "Port")]
    pub port: usize,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Credentials")]
    pub credentials: Option<SecretArn>,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceKnowledgeArticleConfiguration {
    #[serde(rename = "CustomKnowledgeArticleTypeConfigurations")]
    pub custom_knowledge_article_type_configurations: Option<SalesforceCustomKnowledgeArticleTypeConfigurationList>,
    #[serde(rename = "StandardKnowledgeArticleTypeConfiguration")]
    pub standard_knowledge_article_type_configuration: Option<SalesforceStandardKnowledgeArticleTypeConfiguration>,
    #[serde(rename = "IncludedStates")]
    pub included_states: SalesforceKnowledgeArticleStateList,

}
pub type ConfluenceBlogFieldName = String;pub type Url = String;
#[derive(serde::Serialize, Default)]
pub struct ChangeDetectingColumns {

}

#[derive(serde::Serialize, Default)]
pub struct WebCrawlerSeedUrlConfiguration {
    #[serde(rename = "WebCrawlerMode")]
    pub web_crawler_mode: Option<String>,
    #[serde(rename = "SeedUrls")]
    pub seed_urls: WebCrawlerSeedUrlList,

}

#[derive(serde::Serialize, Default)]
pub struct DocumentAttributeTarget {
    #[serde(rename = "TargetDocumentAttributeKey")]
    pub target_document_attribute_key: DocumentAttributeKey,
    #[serde(rename = "TargetDocumentAttributeValue")]
    pub target_document_attribute_value: Option<DocumentAttributeValue>,
    #[serde(rename = "TargetDocumentAttributeValueDeletion")]
    pub target_document_attribute_value_deletion: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfluenceSpaceList {

}

#[derive(serde::Serialize, Default)]
pub struct TagList {

}
pub type DocumentAttributeKey = String;pub type QueryIdentifiersEnclosingOption = String;pub type Id = String;pub type ConditionOperator = String;pub type ServiceNowAuthenticationType = String;
#[derive(serde::Serialize, Default)]
pub struct ConfluenceBlogToIndexFieldMapping {
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<DateFieldFormat>,
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: ConfluenceBlogFieldName,
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: IndexFieldName,

}
pub type ColumnName = String;pub type Name = String;
#[derive(serde::Serialize, Default)]
pub struct DocumentAttributeValue {
    #[serde(rename = "StringListValue")]
    pub string_list_value: Option<Vec<String>>,
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,
    #[serde(rename = "DateValue")]
    pub date_value: Option<Timestamp>,
    #[serde(rename = "LongValue")]
    pub long_value: Option<Long>,

}

#[derive(serde::Serialize, Default)]
pub struct DataSourceToIndexFieldMapping {
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: IndexFieldName,
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: DataSourceFieldName,
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<DateFieldFormat>,

}
pub type SalesforceStandardObjectName = String;pub type DatabaseHost = String;
#[derive(serde::Serialize, Default)]
pub struct SalesforceStandardObjectConfigurationList {

}

#[derive(serde::Serialize, Default)]
pub struct WebCrawlerSiteMaps {

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceKnowledgeArticleStateList {

}
pub type Description = String;
#[derive(serde::Serialize, Default)]
pub struct WebCrawlerConfiguration {
    #[serde(rename = "AuthenticationConfiguration")]
    pub authentication_configuration: Option<WebCrawlerAuthenticationConfiguration>,
    #[serde(rename = "Urls")]
    pub urls: WebCrawlerUrls,
    #[serde(rename = "UrlInclusionPatterns")]
    pub url_inclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "MaxUrlsPerMinuteCrawlRate")]
    pub max_urls_per_minute_crawl_rate: Option<usize>,
    #[serde(rename = "ProxyConfiguration")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    #[serde(rename = "CrawlDepth")]
    pub crawl_depth: Option<usize>,
    #[serde(rename = "MaxLinksPerPage")]
    pub max_links_per_page: Option<usize>,
    #[serde(rename = "UrlExclusionPatterns")]
    pub url_exclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "MaxContentSizePerPageInMegaBytes")]
    pub max_content_size_per_page_in_mega_bytes: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct HookConfiguration {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: S3BucketName,
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: LambdaArn,
    #[serde(rename = "InvocationCondition")]
    pub invocation_condition: Option<DocumentAttributeCondition>,

}

#[derive(serde::Serialize, Default)]
pub struct WebCrawlerAuthenticationConfiguration {
    #[serde(rename = "BasicAuthentication")]
    pub basic_authentication: Option<WebCrawlerBasicAuthenticationList>,

}
pub type ServiceNowKnowledgeArticleFilterQuery = String;
#[derive(serde::Serialize, Default)]
pub struct OneDriveUserList {

}
pub type Timestamp = String;
#[derive(serde::Serialize, Default)]
pub struct ConfluenceSpaceFieldMappingsList {

}

#[derive(serde::Serialize, Default)]
pub struct SqlConfiguration {
    #[serde(rename = "QueryIdentifiersEnclosingOption")]
    pub query_identifiers_enclosing_option: Option<QueryIdentifiersEnclosingOption>,

}
pub type ConfluenceSpaceIdentifier = String;
#[derive(serde::Serialize, Default)]
pub struct AccessControlListConfiguration {
    #[serde(rename = "KeyPath")]
    pub key_path: Option<S3ObjectKey>,

}
pub type ConfluencePageFieldName = String;pub type LambdaArn = String;pub type IndexId = String;pub type DatabaseName = String;
#[derive(serde::Serialize, Default)]
pub struct DocumentsMetadataConfiguration {
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: Option<S3ObjectKey>,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceCustomKnowledgeArticleTypeConfigurationList {

}
pub type Long = usize;pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct ServiceNowKnowledgeArticleConfiguration {
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,
    #[serde(rename = "ExcludeAttachmentFilePatterns")]
    pub exclude_attachment_file_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "IncludeAttachmentFilePatterns")]
    pub include_attachment_file_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: DataSourceFieldName,
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<DataSourceFieldName>,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "FilterQuery")]
    pub filter_query: Option<ServiceNowKnowledgeArticleFilterQuery>,

}

#[derive(serde::Serialize, Default)]
pub struct ExcludeUserAccountsList {

}
pub type OneDriveUser = String;pub type ConfluenceSpaceFieldName = String;
#[derive(serde::Serialize, Default)]
pub struct GoogleDriveConfiguration {
    #[serde(rename = "ExcludeMimeTypes")]
    pub exclude_mime_types: Option<ExcludeMimeTypesList>,
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: SecretArn,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "ExcludeUserAccounts")]
    pub exclude_user_accounts: Option<ExcludeUserAccountsList>,
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "ExcludeSharedDrives")]
    pub exclude_shared_drives: Option<ExcludeSharedDrivesList>,

}

#[derive(serde::Serialize, Default)]
pub struct InlineConfigurations {

}

#[derive(serde::Serialize, Default)]
pub struct ConfluenceSpaceToIndexFieldMapping {
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<DateFieldFormat>,
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: IndexFieldName,
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: ConfluenceSpaceFieldName,

}
pub type SecretArn = String;pub type SalesforceKnowledgeArticleState = String;pub type MimeType = String;pub type Schedule = String;
#[derive(serde::Serialize, Default)]
pub struct SalesforceCustomKnowledgeArticleTypeConfiguration {
    #[serde(rename = "Name")]
    pub name: SalesforceCustomKnowledgeArticleTypeName,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<DataSourceFieldName>,
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: DataSourceFieldName,

}

#[derive(serde::Serialize, Default)]
pub struct WebCrawlerBasicAuthentication {
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Port")]
    pub port: usize,
    #[serde(rename = "Credentials")]
    pub credentials: SecretArn,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceStandardKnowledgeArticleTypeConfiguration {
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<DataSourceFieldName>,
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: DataSourceFieldName,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,

}
pub type DataSourceInclusionsExclusionsStrings = Vec<String>;pub type IndexFieldName = String;
#[derive(serde::Serialize, Default)]
pub struct WebCrawlerUrls {
    #[serde(rename = "SeedUrlConfiguration")]
    pub seed_url_configuration: Option<WebCrawlerSeedUrlConfiguration>,
    #[serde(rename = "SiteMapsConfiguration")]
    pub site_maps_configuration: Option<WebCrawlerSiteMapsConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AclConfiguration {
    #[serde(rename = "AllowedGroupsColumnName")]
    pub allowed_groups_column_name: ColumnName,

}

#[derive(serde::Serialize, Default)]
pub struct ConfluenceBlogFieldMappingsList {

}

#[derive(serde::Serialize, Default)]
pub struct InlineCustomDocumentEnrichmentConfiguration {
    #[serde(rename = "DocumentContentDeletion")]
    pub document_content_deletion: Option<bool>,
    #[serde(rename = "Condition")]
    pub condition: Option<DocumentAttributeCondition>,
    #[serde(rename = "Target")]
    pub target: Option<DocumentAttributeTarget>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfluenceAttachmentFieldMappingsList {

}

#[derive(serde::Serialize, Default)]
pub struct ConfluenceAttachmentConfiguration {
    #[serde(rename = "AttachmentFieldMappings")]
    pub attachment_field_mappings: Option<ConfluenceAttachmentFieldMappingsList>,
    #[serde(rename = "CrawlAttachments")]
    pub crawl_attachments: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceChatterFeedIncludeFilterTypes {

}

#[derive(serde::Serialize, Default)]
pub struct S3Path {
    #[serde(rename = "Bucket")]
    pub bucket: S3BucketName,
    #[serde(rename = "Key")]
    pub key: S3ObjectKey,

}
pub type UserAccount = String;
#[derive(serde::Serialize, Default)]
pub struct OneDriveUsers {
    #[serde(rename = "OneDriveUserList")]
    pub one_drive_user_list: Option<OneDriveUserList>,
    #[serde(rename = "OneDriveUserS3Path")]
    pub one_drive_user_s3_path: Option<S3Path>,

}

#[derive(serde::Serialize, Default)]
pub struct WebCrawlerSiteMapsConfiguration {
    #[serde(rename = "SiteMaps")]
    pub site_maps: WebCrawlerSiteMaps,

}

#[derive(serde::Serialize, Default)]
pub struct ConfluencePageToIndexFieldMapping {
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: ConfluencePageFieldName,
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<DateFieldFormat>,
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: IndexFieldName,

}

#[derive(serde::Serialize, Default)]
pub struct ExcludeMimeTypesList {

}
pub type SalesforceCustomKnowledgeArticleTypeName = String;
#[derive(serde::Serialize, Default)]
pub struct ConfluenceBlogConfiguration {
    #[serde(rename = "BlogFieldMappings")]
    pub blog_field_mappings: Option<ConfluenceBlogFieldMappingsList>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomDocumentEnrichmentConfiguration {
    #[serde(rename = "PostExtractionHookConfiguration")]
    pub post_extraction_hook_configuration: Option<HookConfiguration>,
    #[serde(rename = "PreExtractionHookConfiguration")]
    pub pre_extraction_hook_configuration: Option<HookConfiguration>,
    #[serde(rename = "InlineConfigurations")]
    pub inline_configurations: Option<InlineConfigurations>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<RoleArn>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceStandardObjectConfiguration {
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<DataSourceFieldName>,
    #[serde(rename = "DocumentDataFieldName")]
    pub document_data_field_name: DataSourceFieldName,
    #[serde(rename = "Name")]
    pub name: SalesforceStandardObjectName,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectionConfiguration {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "SecretArn")]
    pub secret_arn: SecretArn,
    #[serde(rename = "DatabaseHost")]
    pub database_host: DatabaseHost,
    #[serde(rename = "DatabasePort")]
    pub database_port: DatabasePort,
    #[serde(rename = "DatabaseName")]
    pub database_name: DatabaseName,

}
pub type ServiceNowBuildVersionType = String;pub type DateFieldFormat = String;
#[derive(serde::Serialize, Default)]
pub struct ConfluenceAttachmentToIndexFieldMapping {
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: ConfluenceAttachmentFieldName,
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: IndexFieldName,
    #[serde(rename = "DateFieldFormat")]
    pub date_field_format: Option<DateFieldFormat>,

}

#[derive(serde::Serialize, Default)]
pub struct WebCrawlerBasicAuthenticationList {

}

#[derive(serde::Serialize, Default)]
pub struct SalesforceStandardObjectAttachmentConfiguration {
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "DocumentTitleFieldName")]
    pub document_title_field_name: Option<DataSourceFieldName>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceNowConfiguration {
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: Option<ServiceNowAuthenticationType>,
    #[serde(rename = "ServiceCatalogConfiguration")]
    pub service_catalog_configuration: Option<ServiceNowServiceCatalogConfiguration>,
    #[serde(rename = "ServiceNowBuildVersion")]
    pub service_now_build_version: ServiceNowBuildVersionType,
    #[serde(rename = "KnowledgeArticleConfiguration")]
    pub knowledge_article_configuration: Option<ServiceNowKnowledgeArticleConfiguration>,
    #[serde(rename = "HostUrl")]
    pub host_url: ServiceNowHostUrl,
    #[serde(rename = "SecretArn")]
    pub secret_arn: SecretArn,

}

#[derive(serde::Serialize, Default)]
pub struct WorkDocsConfiguration {
    #[serde(rename = "OrganizationId")]
    pub organization_id: String,
    #[serde(rename = "FieldMappings")]
    pub field_mappings: Option<DataSourceToIndexFieldMappingList>,
    #[serde(rename = "CrawlComments")]
    pub crawl_comments: Option<bool>,
    #[serde(rename = "InclusionPatterns")]
    pub inclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "ExclusionPatterns")]
    pub exclusion_patterns: Option<DataSourceInclusionsExclusionsStrings>,
    #[serde(rename = "UseChangeLog")]
    pub use_change_log: Option<bool>,

}
pub type WebCrawlerSiteMap = String;pub type RoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct DataSourceConfiguration {
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: Option<S3DataSourceConfiguration>,
    #[serde(rename = "DatabaseConfiguration")]
    pub database_configuration: Option<DatabaseConfiguration>,
    #[serde(rename = "ServiceNowConfiguration")]
    pub service_now_configuration: Option<ServiceNowConfiguration>,
    #[serde(rename = "GoogleDriveConfiguration")]
    pub google_drive_configuration: Option<GoogleDriveConfiguration>,
    #[serde(rename = "WorkDocsConfiguration")]
    pub work_docs_configuration: Option<WorkDocsConfiguration>,
    #[serde(rename = "SalesforceConfiguration")]
    pub salesforce_configuration: Option<SalesforceConfiguration>,
    #[serde(rename = "ConfluenceConfiguration")]
    pub confluence_configuration: Option<ConfluenceConfiguration>,
    #[serde(rename = "OneDriveConfiguration")]
    pub one_drive_configuration: Option<OneDriveConfiguration>,
    #[serde(rename = "SharePointConfiguration")]
    pub share_point_configuration: Option<SharePointConfiguration>,
    #[serde(rename = "WebCrawlerConfiguration")]
    pub web_crawler_configuration: Option<WebCrawlerConfiguration>,

}


}

pub mod cfn_faq {

#[derive(serde::Serialize, Default)]
pub struct CfnFaq {
    /// FAQ file format
    #[serde(rename = "FileFormat")]
    pub file_format: Option<FileFormat>,
    /// FAQ role ARN
    #[serde(rename = "RoleArn")]
    pub role_arn: RoleArn,
    /// Index ID
    #[serde(rename = "IndexId")]
    pub index_id: IndexId,
    /// Tags for labeling the FAQ
    #[serde(rename = "Tags")]
    pub tags: Option<TagList>,
    /// FAQ description
    #[serde(rename = "Description")]
    pub description: Option<Description>,
    /// FAQ S3 path
    #[serde(rename = "S3Path")]
    pub s3_path: S3Path,
    /// FAQ name
    #[serde(rename = "Name")]
    pub name: FaqName,

}

pub type FileFormat = String;pub type S3BucketName = String;pub type IndexId = String;pub type RoleArn = String;pub type FaqName = String;pub type S3ObjectKey = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type Description = String;
#[derive(serde::Serialize, Default)]
pub struct S3Path {
    #[serde(rename = "Key")]
    pub key: S3ObjectKey,
    #[serde(rename = "Bucket")]
    pub bucket: S3BucketName,

}

#[derive(serde::Serialize, Default)]
pub struct TagList {

}
pub type Id = String;

}

pub mod cfn_index {

#[derive(serde::Serialize, Default)]
pub struct CfnIndex {
    /// Document metadata configurations
    #[serde(rename = "DocumentMetadataConfigurations")]
    pub document_metadata_configurations: Option<DocumentMetadataConfigurationList>,
    /// Edition of index
    #[serde(rename = "Edition")]
    pub edition: Edition,
    /// No documentation provided by AWS
    #[serde(rename = "UserTokenConfigurations")]
    pub user_token_configurations: Option<UserTokenConfigurationList>,
    /// A description for the index
    #[serde(rename = "Description")]
    pub description: Option<Description>,
    /// No documentation provided by AWS
    #[serde(rename = "UserContextPolicy")]
    pub user_context_policy: Option<UserContextPolicy>,
    /// Name of index
    #[serde(rename = "Name")]
    pub name: Name,
    /// Role Arn
    #[serde(rename = "RoleArn")]
    pub role_arn: RoleArn,
    /// Capacity units
    #[serde(rename = "CapacityUnits")]
    pub capacity_units: Option<CapacityUnitsConfiguration>,
    /// Tags for labeling the index
    #[serde(rename = "Tags")]
    pub tags: Option<TagList>,
    /// Server side encryption configuration
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct UserTokenConfigurationList {

}

#[derive(serde::Serialize, Default)]
pub struct DocumentMetadataConfigurationList {

}

#[derive(serde::Serialize, Default)]
pub struct JsonTokenTypeConfiguration {
    #[serde(rename = "GroupAttributeField")]
    pub group_attribute_field: GroupAttributeField,
    #[serde(rename = "UserNameAttributeField")]
    pub user_name_attribute_field: UserNameAttributeField,

}
pub type UserContextPolicy = String;pub type Arn = String;pub type KeyLocation = String;
#[derive(serde::Serialize, Default)]
pub struct DocumentMetadataConfiguration {
    #[serde(rename = "Name")]
    pub name: DocumentMetadataConfigurationName,
    #[serde(rename = "Type")]
    pub ty: DocumentAttributeValueType,
    #[serde(rename = "Relevance")]
    pub relevance: Option<Relevance>,
    #[serde(rename = "Search")]
    pub search: Option<Search>,

}

#[derive(serde::Serialize, Default)]
pub struct JwtTokenTypeConfiguration {
    #[serde(rename = "KeyLocation")]
    pub key_location: KeyLocation,
    #[serde(rename = "GroupAttributeField")]
    pub group_attribute_field: Option<GroupAttributeField>,
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: Option<RoleArn>,
    #[serde(rename = "URL")]
    pub url: Option<Url>,
    #[serde(rename = "ClaimRegex")]
    pub claim_regex: Option<ClaimRegex>,
    #[serde(rename = "Issuer")]
    pub issuer: Option<Issuer>,
    #[serde(rename = "UserNameAttributeField")]
    pub user_name_attribute_field: Option<UserNameAttributeField>,

}

#[derive(serde::Serialize, Default)]
pub struct ServerSideEncryptionConfiguration {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<KmsKeyId>,

}
pub type RoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct Search {
    #[serde(rename = "Facetable")]
    pub facetable: Option<bool>,
    #[serde(rename = "Sortable")]
    pub sortable: Option<bool>,
    #[serde(rename = "Searchable")]
    pub searchable: Option<bool>,
    #[serde(rename = "Displayable")]
    pub displayable: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ValueImportanceItems {

}

#[derive(serde::Serialize, Default)]
pub struct Relevance {
    #[serde(rename = "ValueImportanceItems")]
    pub value_importance_items: Option<ValueImportanceItems>,
    #[serde(rename = "Duration")]
    pub duration: Option<Duration>,
    #[serde(rename = "Importance")]
    pub importance: Option<Importance>,
    #[serde(rename = "Freshness")]
    pub freshness: Option<Freshness>,
    #[serde(rename = "RankOrder")]
    pub rank_order: Option<Order>,

}
pub type Duration = String;pub type Freshness = bool;pub type Issuer = String;
#[derive(serde::Serialize, Default)]
pub struct ValueImportanceItem {
    #[serde(rename = "Key")]
    pub key: Option<ValueImportanceItemKey>,
    #[serde(rename = "Value")]
    pub value: Option<Importance>,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityUnitsConfiguration {
    #[serde(rename = "QueryCapacityUnits")]
    pub query_capacity_units: QueryCapacityUnits,
    #[serde(rename = "StorageCapacityUnits")]
    pub storage_capacity_units: StorageCapacityUnits,

}
pub type Name = String;pub type UserNameAttributeField = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type DocumentMetadataConfigurationName = String;
#[derive(serde::Serialize, Default)]
pub struct UserTokenConfiguration {
    #[serde(rename = "JsonTokenTypeConfiguration")]
    pub json_token_type_configuration: Option<JsonTokenTypeConfiguration>,
    #[serde(rename = "JwtTokenTypeConfiguration")]
    pub jwt_token_type_configuration: Option<JwtTokenTypeConfiguration>,

}
pub type Order = String;pub type DocumentAttributeValueType = String;pub type StorageCapacityUnits = usize;pub type Edition = String;pub type KmsKeyId = String;pub type QueryCapacityUnits = usize;pub type Url = String;pub type Description = String;pub type GroupAttributeField = String;
#[derive(serde::Serialize, Default)]
pub struct TagList {

}
pub type ValueImportanceItemKey = String;pub type Importance = usize;pub type Id = String;pub type ClaimRegex = String;

}
