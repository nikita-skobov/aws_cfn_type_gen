
pub mod cfn_database {

#[derive(serde::Serialize, Default)]
pub struct CfnDatabase {
    /// The name for the database. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the database name.
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The KMS key for the database. If the KMS key is not specified, the database will be encrypted with a Timestream managed KMS key located in your account.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_scheduled_query {

#[derive(serde::Serialize, Default)]
pub struct CfnScheduledQuery {
    /// The name of the scheduled query. Scheduled query names must be unique within each Region.
    #[serde(rename = "ScheduledQueryName")]
    pub scheduled_query_name: Option<ScheduledQueryName>,
    /// A list of key-value pairs to label the scheduled query.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// Using a ClientToken makes the call to CreateScheduledQuery idempotent, in other words, making the same request repeatedly will produce the same result. Making multiple identical CreateScheduledQuery requests has the same effect as making a single request. If CreateScheduledQuery is called without a ClientToken, the Query SDK generates a ClientToken on your behalf. After 8 hours, any request with the same ClientToken is treated as a new request.
    #[serde(rename = "ClientToken")]
    pub client_token: Option<ClientToken>,

}


#[derive(serde::Serialize, Default)]
pub struct TimestreamConfiguration {
    #[serde(rename = "DimensionMappings")]
    pub dimension_mappings: DimensionMappings,
    #[serde(rename = "MultiMeasureMappings")]
    pub multi_measure_mappings: Option<MultiMeasureMappings>,
    #[serde(rename = "MeasureNameColumn")]
    pub measure_name_column: Option<MeasureNameColumn>,
    #[serde(rename = "TimeColumn")]
    pub time_column: TimeColumn,
    #[serde(rename = "DatabaseName")]
    pub database_name: DatabaseName,
    #[serde(rename = "MixedMeasureMappings")]
    pub mixed_measure_mappings: Option<MixedMeasureMappings>,
    #[serde(rename = "TableName")]
    pub table_name: TableName,

}
pub type TableName = String;
#[derive(serde::Serialize, Default)]
pub struct MultiMeasureMappings {
    #[serde(rename = "TargetMultiMeasureName")]
    pub target_multi_measure_name: Option<TargetMultiMeasureName>,
    #[serde(rename = "MultiMeasureAttributeMappings")]
    pub multi_measure_attribute_mappings: MultiMeasureAttributeMappingList,

}
pub type TimeColumn = String;
#[derive(serde::Serialize, Default)]
pub struct MixedMeasureMappings {

}

#[derive(serde::Serialize, Default)]
pub struct S3Configuration {
    #[serde(rename = "ObjectKeyPrefix")]
    pub object_key_prefix: Option<ObjectKeyPrefix>,
    #[serde(rename = "EncryptionOption")]
    pub encryption_option: Option<EncryptionOption>,
    #[serde(rename = "BucketName")]
    pub bucket_name: BucketName,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Value,
    #[serde(rename = "Key")]
    pub key: Key,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationConfiguration {
    #[serde(rename = "SnsConfiguration")]
    pub sns_configuration: SnsConfiguration,

}
pub type ScheduleExpression = String;
#[derive(serde::Serialize, Default)]
pub struct MultiMeasureAttributeMappingList {

}
pub type MixedMeasureMappingTargetMeasureName = String;pub type MixedMeasureMappingMeasureName = String;
#[derive(serde::Serialize, Default)]
pub struct ErrorReportConfiguration {
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3Configuration,

}
pub type ObjectKeyPrefix = String;pub type EncryptionOption = String;pub type Arn = String;pub type MultiMeasureAttributeMappingMeasureValueType = String;pub type Key = String;
#[derive(serde::Serialize, Default)]
pub struct Tags {

}

#[derive(serde::Serialize, Default)]
pub struct DimensionMappings {

}
pub type DatabaseName = String;pub type MultiMeasureAttributeMappingSourceColumn = String;pub type DimensionValueType = String;pub type BucketName = String;pub type TopicArn = String;
#[derive(serde::Serialize, Default)]
pub struct TargetConfiguration {
    #[serde(rename = "TimestreamConfiguration")]
    pub timestream_configuration: TimestreamConfiguration,

}
pub type QueryString = String;pub type MixedMeasureMappingMeasureValueType = String;pub type MeasureNameColumn = String;pub type ScheduledQueryExecutionRoleArn = String;pub type KmsKeyId = String;pub type DimensionMappingName = String;pub type ClientToken = String;
#[derive(serde::Serialize, Default)]
pub struct MixedMeasureMapping {
    #[serde(rename = "SourceColumn")]
    pub source_column: Option<MixedMeasureMappingSourceColumn>,
    #[serde(rename = "MeasureName")]
    pub measure_name: Option<MixedMeasureMappingMeasureName>,
    #[serde(rename = "MultiMeasureAttributeMappings")]
    pub multi_measure_attribute_mappings: Option<MultiMeasureAttributeMappingList>,
    #[serde(rename = "TargetMeasureName")]
    pub target_measure_name: Option<MixedMeasureMappingTargetMeasureName>,
    #[serde(rename = "MeasureValueType")]
    pub measure_value_type: MixedMeasureMappingMeasureValueType,

}

#[derive(serde::Serialize, Default)]
pub struct ScheduleConfiguration {
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: ScheduleExpression,

}
pub type MixedMeasureMappingSourceColumn = String;pub type Value = String;
#[derive(serde::Serialize, Default)]
pub struct MultiMeasureAttributeMapping {
    #[serde(rename = "TargetMultiMeasureAttributeName")]
    pub target_multi_measure_attribute_name: Option<TargetMultiMeasureAttributeName>,
    #[serde(rename = "MeasureValueType")]
    pub measure_value_type: MultiMeasureAttributeMappingMeasureValueType,
    #[serde(rename = "SourceColumn")]
    pub source_column: MultiMeasureAttributeMappingSourceColumn,

}

#[derive(serde::Serialize, Default)]
pub struct SnsConfiguration {
    #[serde(rename = "TopicArn")]
    pub topic_arn: TopicArn,

}
pub type ScheduledQueryName = String;pub type TargetMultiMeasureAttributeName = String;pub type TargetMultiMeasureName = String;
#[derive(serde::Serialize, Default)]
pub struct DimensionMapping {
    #[serde(rename = "Name")]
    pub name: DimensionMappingName,
    #[serde(rename = "DimensionValueType")]
    pub dimension_value_type: DimensionValueType,

}


}

pub mod cfn_table {

#[derive(serde::Serialize, Default)]
pub struct CfnTable {
    /// The properties that determine whether magnetic store writes are enabled.
    #[serde(rename = "MagneticStoreWriteProperties")]
    pub magnetic_store_write_properties: Option<()>,
    /// The name for the table. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the table name.
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The retention duration of the memory store and the magnetic store.
    #[serde(rename = "RetentionProperties")]
    pub retention_properties: Option<()>,
    /// The name for the database which the table to be created belongs to.
    #[serde(rename = "DatabaseName")]
    pub database_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}
