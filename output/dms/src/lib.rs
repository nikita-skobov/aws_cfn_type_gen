
pub mod cfn_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnCertificate {
    /// No documentation provided by AWS
    #[serde(rename = "CertificatePem")]
    pub certificate_pem: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateIdentifier")]
    pub certificate_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateWallet")]
    pub certificate_wallet: Option<String>,

}



}

pub mod cfn_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnEndpoint {
    /// No documentation provided by AWS
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RedshiftSettings")]
    pub redshift_settings: Option<RedshiftSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Password")]
    pub password: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OracleSettings")]
    pub oracle_settings: Option<OracleSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "RedisSettings")]
    pub redis_settings: Option<RedisSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "GcpMySQLSettings")]
    pub gcp_my_sqlsettings: Option<GcpMySQLSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "SybaseSettings")]
    pub sybase_settings: Option<SybaseSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "DynamoDbSettings")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineName")]
    pub engine_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "MySqlSettings")]
    pub my_sql_settings: Option<MySqlSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NeptuneSettings")]
    pub neptune_settings: Option<NeptuneSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "KinesisSettings")]
    pub kinesis_settings: Option<KinesisSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "IbmDb2Settings")]
    pub ibm_db2_settings: Option<IbmDb2Settings>,
    /// No documentation provided by AWS
    #[serde(rename = "DocDbSettings")]
    pub doc_db_settings: Option<DocDbSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "MongoDbSettings")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "ElasticsearchSettings")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "MicrosoftSqlServerSettings")]
    pub microsoft_sql_server_settings: Option<MicrosoftSqlServerSettings>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ExtraConnectionAttributes")]
    pub extra_connection_attributes: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SslMode")]
    pub ssl_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Username")]
    pub username: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointIdentifier")]
    pub endpoint_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "S3Settings")]
    pub s3_settings: Option<S3Settings>,
    /// No documentation provided by AWS
    #[serde(rename = "KafkaSettings")]
    pub kafka_settings: Option<KafkaSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "PostgreSqlSettings")]
    pub postgre_sql_settings: Option<PostgreSqlSettings>,

}


#[derive(serde::Serialize, Default)]
pub struct SybaseSettings {
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RedshiftSettings {
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "EmptyAsNull")]
    pub empty_as_null: Option<bool>,
    #[serde(rename = "LoadTimeout")]
    pub load_timeout: Option<usize>,
    #[serde(rename = "CaseSensitiveNames")]
    pub case_sensitive_names: Option<bool>,
    #[serde(rename = "FileTransferUploadStreams")]
    pub file_transfer_upload_streams: Option<usize>,
    #[serde(rename = "BucketFolder")]
    pub bucket_folder: Option<String>,
    #[serde(rename = "ConnectionTimeout")]
    pub connection_timeout: Option<usize>,
    #[serde(rename = "RemoveQuotes")]
    pub remove_quotes: Option<bool>,
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    pub server_side_encryption_kms_key_id: Option<String>,
    #[serde(rename = "TruncateColumns")]
    pub truncate_columns: Option<bool>,
    #[serde(rename = "ReplaceChars")]
    pub replace_chars: Option<String>,
    #[serde(rename = "TrimBlanks")]
    pub trim_blanks: Option<bool>,
    #[serde(rename = "AcceptAnyDate")]
    pub accept_any_date: Option<bool>,
    #[serde(rename = "EncryptionMode")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ReplaceInvalidChars")]
    pub replace_invalid_chars: Option<String>,
    #[serde(rename = "DateFormat")]
    pub date_format: Option<String>,
    #[serde(rename = "WriteBufferSize")]
    pub write_buffer_size: Option<usize>,
    #[serde(rename = "AfterConnectScript")]
    pub after_connect_script: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "TimeFormat")]
    pub time_format: Option<String>,
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<usize>,
    #[serde(rename = "CompUpdate")]
    pub comp_update: Option<bool>,
    #[serde(rename = "MapBooleanAsBoolean")]
    pub map_boolean_as_boolean: Option<bool>,
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,
    #[serde(rename = "ExplicitIds")]
    pub explicit_ids: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct DocDbSettings {
    #[serde(rename = "DocsToInvestigate")]
    pub docs_to_investigate: Option<usize>,
    #[serde(rename = "ExtractDocId")]
    pub extract_doc_id: Option<bool>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "NestingLevel")]
    pub nesting_level: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RedisSettings {
    #[serde(rename = "SslSecurityProtocol")]
    pub ssl_security_protocol: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<f64>,
    #[serde(rename = "SslCaCertificateArn")]
    pub ssl_ca_certificate_arn: Option<String>,
    #[serde(rename = "AuthPassword")]
    pub auth_password: Option<String>,
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,
    #[serde(rename = "AuthUserName")]
    pub auth_user_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OracleSettings {
    #[serde(rename = "AllowSelectNestedTables")]
    pub allow_select_nested_tables: Option<bool>,
    #[serde(rename = "SecretsManagerOracleAsmAccessRoleArn")]
    pub secrets_manager_oracle_asm_access_role_arn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "UseBFile")]
    pub use_bfile: Option<bool>,
    #[serde(rename = "ReadAheadBlocks")]
    pub read_ahead_blocks: Option<usize>,
    #[serde(rename = "ArchivedLogDestId")]
    pub archived_log_dest_id: Option<usize>,
    #[serde(rename = "SecurityDbEncryptionName")]
    pub security_db_encryption_name: Option<String>,
    #[serde(rename = "AsmPassword")]
    pub asm_password: Option<String>,
    #[serde(rename = "ReplacePathPrefix")]
    pub replace_path_prefix: Option<bool>,
    #[serde(rename = "NumberDatatypeScale")]
    pub number_datatype_scale: Option<usize>,
    #[serde(rename = "UseLogminerReader")]
    pub use_logminer_reader: Option<bool>,
    #[serde(rename = "ReadTableSpaceName")]
    pub read_table_space_name: Option<bool>,
    #[serde(rename = "UseAlternateFolderForOnline")]
    pub use_alternate_folder_for_online: Option<bool>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "StandbyDelayTime")]
    pub standby_delay_time: Option<usize>,
    #[serde(rename = "AsmServer")]
    pub asm_server: Option<String>,
    #[serde(rename = "FailTasksOnLobTruncation")]
    pub fail_tasks_on_lob_truncation: Option<bool>,
    #[serde(rename = "SecretsManagerOracleAsmSecretId")]
    pub secrets_manager_oracle_asm_secret_id: Option<String>,
    #[serde(rename = "RetryInterval")]
    pub retry_interval: Option<usize>,
    #[serde(rename = "EnableHomogenousTablespace")]
    pub enable_homogenous_tablespace: Option<bool>,
    #[serde(rename = "AdditionalArchivedLogDestId")]
    pub additional_archived_log_dest_id: Option<usize>,
    #[serde(rename = "AsmUser")]
    pub asm_user: Option<String>,
    #[serde(rename = "OraclePathPrefix")]
    pub oracle_path_prefix: Option<String>,
    #[serde(rename = "SpatialDataOptionToGeoJsonFunctionName")]
    pub spatial_data_option_to_geo_json_function_name: Option<String>,
    #[serde(rename = "ArchivedLogsOnly")]
    pub archived_logs_only: Option<bool>,
    #[serde(rename = "AddSupplementalLogging")]
    pub add_supplemental_logging: Option<bool>,
    #[serde(rename = "UseDirectPathFullLoad")]
    pub use_direct_path_full_load: Option<bool>,
    #[serde(rename = "ParallelAsmReadThreads")]
    pub parallel_asm_read_threads: Option<usize>,
    #[serde(rename = "ExtraArchivedLogDestIds")]
    pub extra_archived_log_dest_ids: Option<Vec<usize>>,
    #[serde(rename = "DirectPathParallelLoad")]
    pub direct_path_parallel_load: Option<bool>,
    #[serde(rename = "UsePathPrefix")]
    pub use_path_prefix: Option<String>,
    #[serde(rename = "SecurityDbEncryption")]
    pub security_db_encryption: Option<String>,
    #[serde(rename = "AccessAlternateDirectly")]
    pub access_alternate_directly: Option<bool>,
    #[serde(rename = "DirectPathNoLog")]
    pub direct_path_no_log: Option<bool>,
    #[serde(rename = "CharLengthSemantics")]
    pub char_length_semantics: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MicrosoftSqlServerSettings {
    #[serde(rename = "BcpPacketSize")]
    pub bcp_packet_size: Option<usize>,
    #[serde(rename = "SafeguardPolicy")]
    pub safeguard_policy: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ReadBackupOnly")]
    pub read_backup_only: Option<bool>,
    #[serde(rename = "UseThirdPartyBackupDevice")]
    pub use_third_party_backup_device: Option<bool>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "ControlTablesFileGroup")]
    pub control_tables_file_group: Option<String>,
    #[serde(rename = "QuerySingleAlwaysOnNode")]
    pub query_single_always_on_node: Option<bool>,
    #[serde(rename = "UseBcpFullLoad")]
    pub use_bcp_full_load: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct MongoDbSettings {
    #[serde(rename = "AuthMechanism")]
    pub auth_mechanism: Option<String>,
    #[serde(rename = "Password")]
    pub password: Option<String>,
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "ExtractDocId")]
    pub extract_doc_id: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "AuthSource")]
    pub auth_source: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "NestingLevel")]
    pub nesting_level: Option<String>,
    #[serde(rename = "DocsToInvestigate")]
    pub docs_to_investigate: Option<String>,
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDbSettings {
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GcpMySQLSettings {
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "CleanSourceMetadataOnMismatch")]
    pub clean_source_metadata_on_mismatch: Option<bool>,
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "Password")]
    pub password: Option<String>,
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<usize>,
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,
    #[serde(rename = "EventsPollInterval")]
    pub events_poll_interval: Option<usize>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "ParallelLoadThreads")]
    pub parallel_load_threads: Option<usize>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "ServerTimezone")]
    pub server_timezone: Option<String>,
    #[serde(rename = "AfterConnectScript")]
    pub after_connect_script: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct PostgreSqlSettings {
    #[serde(rename = "HeartbeatSchema")]
    pub heartbeat_schema: Option<String>,
    #[serde(rename = "MapBooleanAsBoolean")]
    pub map_boolean_as_boolean: Option<bool>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "PluginName")]
    pub plugin_name: Option<String>,
    #[serde(rename = "SlotName")]
    pub slot_name: Option<String>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "CaptureDdls")]
    pub capture_ddls: Option<bool>,
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<usize>,
    #[serde(rename = "ExecuteTimeout")]
    pub execute_timeout: Option<usize>,
    #[serde(rename = "HeartbeatEnable")]
    pub heartbeat_enable: Option<bool>,
    #[serde(rename = "HeartbeatFrequency")]
    pub heartbeat_frequency: Option<usize>,
    #[serde(rename = "FailTasksOnLobTruncation")]
    pub fail_tasks_on_lob_truncation: Option<bool>,
    #[serde(rename = "DdlArtifactsSchema")]
    pub ddl_artifacts_schema: Option<String>,
    #[serde(rename = "AfterConnectScript")]
    pub after_connect_script: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MySqlSettings {
    #[serde(rename = "ServerTimezone")]
    pub server_timezone: Option<String>,
    #[serde(rename = "AfterConnectScript")]
    pub after_connect_script: Option<String>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,
    #[serde(rename = "ParallelLoadThreads")]
    pub parallel_load_threads: Option<usize>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "TargetDbType")]
    pub target_db_type: Option<String>,
    #[serde(rename = "CleanSourceMetadataOnMismatch")]
    pub clean_source_metadata_on_mismatch: Option<bool>,
    #[serde(rename = "EventsPollInterval")]
    pub events_poll_interval: Option<usize>,
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct IbmDb2Settings {
    #[serde(rename = "SetDataCaptureChanges")]
    pub set_data_capture_changes: Option<bool>,
    #[serde(rename = "CurrentLsn")]
    pub current_lsn: Option<String>,
    #[serde(rename = "SecretsManagerSecretId")]
    pub secrets_manager_secret_id: Option<String>,
    #[serde(rename = "MaxKBytesPerRead")]
    pub max_kbytes_per_read: Option<usize>,
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    pub secrets_manager_access_role_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticsearchSettings {
    #[serde(rename = "ErrorRetryDuration")]
    pub error_retry_duration: Option<usize>,
    #[serde(rename = "EndpointUri")]
    pub endpoint_uri: Option<String>,
    #[serde(rename = "FullLoadErrorPercentage")]
    pub full_load_error_percentage: Option<usize>,
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Settings {
    #[serde(rename = "RowGroupLength")]
    pub row_group_length: Option<usize>,
    #[serde(rename = "UseCsvNoSupValue")]
    pub use_csv_no_sup_value: Option<bool>,
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,
    #[serde(rename = "CdcPath")]
    pub cdc_path: Option<String>,
    #[serde(rename = "PreserveTransactions")]
    pub preserve_transactions: Option<bool>,
    #[serde(rename = "CannedAclForObjects")]
    pub canned_acl_for_objects: Option<String>,
    #[serde(rename = "CdcMaxBatchInterval")]
    pub cdc_max_batch_interval: Option<usize>,
    #[serde(rename = "EncryptionMode")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "DatePartitionSequence")]
    pub date_partition_sequence: Option<String>,
    #[serde(rename = "CsvDelimiter")]
    pub csv_delimiter: Option<String>,
    #[serde(rename = "IncludeOpForFullLoad")]
    pub include_op_for_full_load: Option<bool>,
    #[serde(rename = "TimestampColumnName")]
    pub timestamp_column_name: Option<String>,
    #[serde(rename = "CompressionType")]
    pub compression_type: Option<String>,
    #[serde(rename = "DatePartitionTimezone")]
    pub date_partition_timezone: Option<String>,
    #[serde(rename = "UseTaskStartTimeForFullLoadTimestamp")]
    pub use_task_start_time_for_full_load_timestamp: Option<bool>,
    #[serde(rename = "IgnoreHeaderRows")]
    pub ignore_header_rows: Option<usize>,
    #[serde(rename = "BucketFolder")]
    pub bucket_folder: Option<String>,
    #[serde(rename = "CdcInsertsOnly")]
    pub cdc_inserts_only: Option<bool>,
    #[serde(rename = "CdcInsertsAndUpdates")]
    pub cdc_inserts_and_updates: Option<bool>,
    #[serde(rename = "DatePartitionEnabled")]
    pub date_partition_enabled: Option<bool>,
    #[serde(rename = "ExternalTableDefinition")]
    pub external_table_definition: Option<String>,
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    pub server_side_encryption_kms_key_id: Option<String>,
    #[serde(rename = "CsvNoSupValue")]
    pub csv_no_sup_value: Option<String>,
    #[serde(rename = "DatePartitionDelimiter")]
    pub date_partition_delimiter: Option<String>,
    #[serde(rename = "CdcMinFileSize")]
    pub cdc_min_file_size: Option<usize>,
    #[serde(rename = "AddColumnName")]
    pub add_column_name: Option<bool>,
    #[serde(rename = "Rfc4180")]
    pub rfc4180: Option<bool>,
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<usize>,
    #[serde(rename = "DataFormat")]
    pub data_format: Option<String>,
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "ParquetTimestampInMillisecond")]
    pub parquet_timestamp_in_millisecond: Option<bool>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,
    #[serde(rename = "CsvRowDelimiter")]
    pub csv_row_delimiter: Option<String>,
    #[serde(rename = "DataPageSize")]
    pub data_page_size: Option<usize>,
    #[serde(rename = "DictPageSizeLimit")]
    pub dict_page_size_limit: Option<usize>,
    #[serde(rename = "EnableStatistics")]
    pub enable_statistics: Option<bool>,
    #[serde(rename = "CsvNullValue")]
    pub csv_null_value: Option<String>,
    #[serde(rename = "ParquetVersion")]
    pub parquet_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KafkaSettings {
    #[serde(rename = "SaslPassword")]
    pub sasl_password: Option<String>,
    #[serde(rename = "SaslUserName")]
    pub sasl_user_name: Option<String>,
    #[serde(rename = "SslClientCertificateArn")]
    pub ssl_client_certificate_arn: Option<String>,
    #[serde(rename = "NoHexPrefix")]
    pub no_hex_prefix: Option<bool>,
    #[serde(rename = "IncludeNullAndEmpty")]
    pub include_null_and_empty: Option<bool>,
    #[serde(rename = "IncludeTableAlterOperations")]
    pub include_table_alter_operations: Option<bool>,
    #[serde(rename = "MessageFormat")]
    pub message_format: Option<String>,
    #[serde(rename = "IncludeControlDetails")]
    pub include_control_details: Option<bool>,
    #[serde(rename = "SslCaCertificateArn")]
    pub ssl_ca_certificate_arn: Option<String>,
    #[serde(rename = "SslClientKeyArn")]
    pub ssl_client_key_arn: Option<String>,
    #[serde(rename = "Broker")]
    pub broker: Option<String>,
    #[serde(rename = "SecurityProtocol")]
    pub security_protocol: Option<String>,
    #[serde(rename = "Topic")]
    pub topic: Option<String>,
    #[serde(rename = "SslClientKeyPassword")]
    pub ssl_client_key_password: Option<String>,
    #[serde(rename = "IncludeTransactionDetails")]
    pub include_transaction_details: Option<bool>,
    #[serde(rename = "PartitionIncludeSchemaTable")]
    pub partition_include_schema_table: Option<bool>,
    #[serde(rename = "MessageMaxBytes")]
    pub message_max_bytes: Option<usize>,
    #[serde(rename = "IncludePartitionValue")]
    pub include_partition_value: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisSettings {
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "PartitionIncludeSchemaTable")]
    pub partition_include_schema_table: Option<bool>,
    #[serde(rename = "IncludeControlDetails")]
    pub include_control_details: Option<bool>,
    #[serde(rename = "IncludePartitionValue")]
    pub include_partition_value: Option<bool>,
    #[serde(rename = "IncludeTransactionDetails")]
    pub include_transaction_details: Option<bool>,
    #[serde(rename = "MessageFormat")]
    pub message_format: Option<String>,
    #[serde(rename = "StreamArn")]
    pub stream_arn: Option<String>,
    #[serde(rename = "IncludeTableAlterOperations")]
    pub include_table_alter_operations: Option<bool>,
    #[serde(rename = "NoHexPrefix")]
    pub no_hex_prefix: Option<bool>,
    #[serde(rename = "IncludeNullAndEmpty")]
    pub include_null_and_empty: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct NeptuneSettings {
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<usize>,
    #[serde(rename = "MaxRetryCount")]
    pub max_retry_count: Option<usize>,
    #[serde(rename = "ErrorRetryDuration")]
    pub error_retry_duration: Option<usize>,
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: Option<String>,
    #[serde(rename = "S3BucketFolder")]
    pub s3_bucket_folder: Option<String>,
    #[serde(rename = "IamAuthEnabled")]
    pub iam_auth_enabled: Option<bool>,

}


}

pub mod cfn_event_subscription {

#[derive(serde::Serialize, Default)]
pub struct CfnEventSubscription {
    /// No documentation provided by AWS
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceType")]
    pub source_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceIds")]
    pub source_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "EventCategories")]
    pub event_categories: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_replication_instance {

#[derive(serde::Serialize, Default)]
pub struct CfnReplicationInstance {
    /// No documentation provided by AWS
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AllocatedStorage")]
    pub allocated_storage: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "MultiAZ")]
    pub multi_az: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationInstanceClass")]
    pub replication_instance_class: String,
    /// No documentation provided by AWS
    #[serde(rename = "AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationInstanceIdentifier")]
    pub replication_instance_identifier: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_replication_subnet_group {

#[derive(serde::Serialize, Default)]
pub struct CfnReplicationSubnetGroup {
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    pub replication_subnet_group_description: String,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_replication_task {

#[derive(serde::Serialize, Default)]
pub struct CfnReplicationTask {
    /// No documentation provided by AWS
    #[serde(rename = "TargetEndpointArn")]
    pub target_endpoint_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "CdcStopPosition")]
    pub cdc_stop_position: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CdcStartPosition")]
    pub cdc_start_position: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TaskData")]
    pub task_data: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationTaskIdentifier")]
    pub replication_task_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceEndpointArn")]
    pub source_endpoint_arn: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "CdcStartTime")]
    pub cdc_start_time: Option<f64>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationTaskSettings")]
    pub replication_task_settings: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MigrationType")]
    pub migration_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "TableMappings")]
    pub table_mappings: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
