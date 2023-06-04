/// Create a scheduled query that will be run on your behalf at the configured schedule.    Timestream assumes the execution role provided as part of the     ScheduledQueryExecutionRoleArn parameter to run the query. You can use the     NotificationConfiguration parameter to configure notification for your    scheduled query operations.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnScheduledQuery {
    ///
    /// Using a ClientToken makes the call to CreateScheduledQuery idempotent, in other words,    making the same request repeatedly will produce the same result. Making multiple identical    CreateScheduledQuery requests has the same effect as making a single request.
    ///
    /// If CreateScheduledQuery is called without a ClientToken, the Query SDK      generates a ClientToken on your behalf.         After 8 hours, any request with the same ClientToken is treated as a new      request.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<cfn_resources::StrVal>,

    ///
    /// Configuration for error reporting. Error reports will be generated when a problem is    encountered when writing the query results.
    ///
    /// Required: Yes
    ///
    /// Type: ErrorReportConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "ErrorReportConfiguration")]
    pub error_report_configuration: ErrorReportConfiguration,

    ///
    /// The Amazon KMS key used to encrypt the scheduled query resource, at-rest. If the Amazon    KMS key is not specified, the scheduled query resource will be encrypted with a Timestream    owned Amazon KMS key. To specify a KMS key, use the key ID, key ARN, alias name, or alias ARN.    When using an alias name, prefix the name with alias/
    ///
    /// If ErrorReportConfiguration uses SSE_KMS as encryption type, the same    KmsKeyId is used to encrypt the error report at rest.
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
    /// Notification configuration for the scheduled query. A notification is sent by Timestream    when a query run finishes, when the state is updated or when you delete it.
    ///
    /// Required: Yes
    ///
    /// Type: NotificationConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: NotificationConfiguration,

    ///
    /// The query string to run. Parameter names can be specified in the query string     @ character followed by an identifier. The named Parameter     @scheduled_runtime is reserved and can be used in the query to get the time at    which the query is scheduled to run.
    ///
    /// The timestamp calculated according to the ScheduleConfiguration parameter, will be the    value of @scheduled_runtime paramater for each query run. For example, consider    an instance of a scheduled query executing on 2021-12-01 00:00:00. For this instance, the     @scheduled_runtime parameter is initialized to the timestamp 2021-12-01    00:00:00 when invoking the query.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "QueryString")]
    pub query_string: cfn_resources::StrVal,

    ///
    /// Schedule configuration.
    ///
    /// Required: Yes
    ///
    /// Type: ScheduleConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScheduleConfiguration")]
    pub schedule_configuration: ScheduleConfiguration,

    ///
    /// The ARN for the IAM role that Timestream will assume when running the scheduled query.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScheduledQueryExecutionRoleArn")]
    pub scheduled_query_execution_role_arn: cfn_resources::StrVal,

    ///
    /// A name for the query. Scheduled query names must be unique within each Region.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScheduledQueryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_query_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of key-value pairs to label the scheduled query.
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
    /// Scheduled query target store configuration.
    ///
    /// Required: No
    ///
    /// Type: TargetConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<TargetConfiguration>,

    #[serde(skip_serializing)]
    pub att_arn: CfnScheduledQueryarn,

    #[serde(skip_serializing)]
    pub att_sqerror_report_configuration: CfnScheduledQuerysqerrorreportconfiguration,

    #[serde(skip_serializing)]
    pub att_sqkms_key_id: CfnScheduledQuerysqkmskeyid,

    #[serde(skip_serializing)]
    pub att_sqname: CfnScheduledQuerysqname,

    #[serde(skip_serializing)]
    pub att_sqnotification_configuration: CfnScheduledQuerysqnotificationconfiguration,

    #[serde(skip_serializing)]
    pub att_sqquery_string: CfnScheduledQuerysqquerystring,

    #[serde(skip_serializing)]
    pub att_sqschedule_configuration: CfnScheduledQuerysqscheduleconfiguration,

    #[serde(skip_serializing)]
    pub att_sqscheduled_query_execution_role_arn: CfnScheduledQuerysqscheduledqueryexecutionrolearn,

    #[serde(skip_serializing)]
    pub att_sqtarget_configuration: CfnScheduledQuerysqtargetconfiguration,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQueryarn;
impl CfnScheduledQueryarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQuerysqerrorreportconfiguration;
impl CfnScheduledQuerysqerrorreportconfiguration {
    pub fn att_name(&self) -> &'static str {
        r#"SQErrorReportConfiguration"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQuerysqkmskeyid;
impl CfnScheduledQuerysqkmskeyid {
    pub fn att_name(&self) -> &'static str {
        r#"SQKmsKeyId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQuerysqname;
impl CfnScheduledQuerysqname {
    pub fn att_name(&self) -> &'static str {
        r#"SQName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQuerysqnotificationconfiguration;
impl CfnScheduledQuerysqnotificationconfiguration {
    pub fn att_name(&self) -> &'static str {
        r#"SQNotificationConfiguration"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQuerysqquerystring;
impl CfnScheduledQuerysqquerystring {
    pub fn att_name(&self) -> &'static str {
        r#"SQQueryString"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQuerysqscheduleconfiguration;
impl CfnScheduledQuerysqscheduleconfiguration {
    pub fn att_name(&self) -> &'static str {
        r#"SQScheduleConfiguration"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQuerysqscheduledqueryexecutionrolearn;
impl CfnScheduledQuerysqscheduledqueryexecutionrolearn {
    pub fn att_name(&self) -> &'static str {
        r#"SQScheduledQueryExecutionRoleArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnScheduledQuerysqtargetconfiguration;
impl CfnScheduledQuerysqtargetconfiguration {
    pub fn att_name(&self) -> &'static str {
        r#"SQTargetConfiguration"#
    }
}

impl cfn_resources::CfnResource for CfnScheduledQuery {
    fn type_string(&self) -> &'static str {
        "AWS::Timestream::ScheduledQuery"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.error_report_configuration.validate()?;

        self.notification_configuration.validate()?;

        self.schedule_configuration.validate()?;

        self.target_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This type is used to map column(s) from the query result to a dimension in the destination    table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DimensionMapping {
    ///
    /// Type for the dimension: VARCHAR
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DimensionValueType")]
    pub dimension_value_type: cfn_resources::StrVal,

    ///
    /// Column name from query result.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for DimensionMapping {
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

/// Configuration required for error reporting.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ErrorReportConfiguration {
    ///
    /// The S3 configuration for the error reports.
    ///
    /// Required: Yes
    ///
    /// Type: S3Configuration
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3Configuration,
}

impl cfn_resources::CfnResource for ErrorReportConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_configuration.validate()?;

        Ok(())
    }
}

/// MixedMeasureMappings are mappings that can be used to ingest data into a mixture of narrow    and multi measures in the derived table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MixedMeasureMapping {
    ///
    /// Refers to the value of measure_name in a result row. This field is required if    MeasureNameColumn is provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MeasureName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_name: Option<cfn_resources::StrVal>,

    ///
    /// Type of the value that is to be read from sourceColumn. If the mapping is for MULTI, use    MeasureValueType.MULTI.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MeasureValueType")]
    pub measure_value_type: cfn_resources::StrVal,

    ///
    /// Required when measureValueType is MULTI. Attribute mappings for MULTI value    measures.
    ///
    /// Required: No
    ///
    /// Type: List of MultiMeasureAttributeMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "MultiMeasureAttributeMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_measure_attribute_mappings: Option<Vec<MultiMeasureAttributeMapping>>,

    ///
    /// This field refers to the source column from which measure-value is to be read for result    materialization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column: Option<cfn_resources::StrVal>,

    ///
    /// Target measure name to be used. If not provided, the target measure name by default would    be measure-name if provided, or sourceColumn otherwise.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetMeasureName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_measure_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MixedMeasureMapping {
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

/// Attribute mapping for MULTI value measures.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MultiMeasureAttributeMapping {
    ///
    /// Type of the attribute to be read from the source column.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MeasureValueType")]
    pub measure_value_type: cfn_resources::StrVal,

    ///
    /// Source column from where the attribute value is to be read.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceColumn")]
    pub source_column: cfn_resources::StrVal,

    ///
    /// Custom name to be used for attribute name in derived table. If not provided, source column    name would be used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetMultiMeasureAttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_multi_measure_attribute_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MultiMeasureAttributeMapping {
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

/// Only one of MixedMeasureMappings or MultiMeasureMappings is to be provided.    MultiMeasureMappings can be used to ingest data as multi measures in the derived table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MultiMeasureMappings {
    ///
    /// Required. Attribute mappings to be used for mapping query results to ingest data for    multi-measure attributes.
    ///
    /// Required: Yes
    ///
    /// Type: List of MultiMeasureAttributeMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "MultiMeasureAttributeMappings")]
    pub multi_measure_attribute_mappings: Vec<MultiMeasureAttributeMapping>,

    ///
    /// The name of the target multi-measure name in the derived table. This input is required    when measureNameColumn is not provided. If MeasureNameColumn is provided, then value from that    column will be used as multi-measure name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetMultiMeasureName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_multi_measure_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MultiMeasureMappings {
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

/// Notification configuration for a scheduled query. A notification is sent by Timestream    when a scheduled query is created, its state is updated or when it is deleted.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotificationConfiguration {
    ///
    /// Details on SNS configuration.
    ///
    /// Required: Yes
    ///
    /// Type: SnsConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnsConfiguration")]
    pub sns_configuration: SnsConfiguration,
}

impl cfn_resources::CfnResource for NotificationConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.sns_configuration.validate()?;

        Ok(())
    }
}

/// Details on S3 location for error reports that result from running a query.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct S3Configuration {
    ///
    /// Name of the S3 bucket under which error reports will be created.
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
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    pub bucket_name: cfn_resources::StrVal,

    ///
    /// Encryption at rest options for the error reports. If no encryption option is specified,    Timestream will choose SSE_S3 as default.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: SSE_KMS | SSE_S3
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_option: Option<S3ConfigurationEncryptionOptionEnum>,

    ///
    /// Prefix for the error report key. Timestream by default adds the following prefix to the    error report path.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 928
    ///
    /// Pattern: [a-zA-Z0-9|!\-_*'\(\)]([a-zA-Z0-9]|[!\-_*'\(\)\/.])+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum S3ConfigurationEncryptionOptionEnum {
    /// SSE_KMS
    #[serde(rename = "SSE_KMS")]
    Ssekms,

    /// SSE_S3
    #[serde(rename = "SSE_S3")]
    Sses3,
}

impl Default for S3ConfigurationEncryptionOptionEnum {
    fn default() -> Self {
        S3ConfigurationEncryptionOptionEnum::Ssekms
    }
}

impl cfn_resources::CfnResource for S3Configuration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'bucket_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.bucket_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'bucket_name'. {} is less than 3",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.object_key_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 928 as _ {
                    return Err(format!("Max validation failed on field 'object_key_prefix'. {} is greater than 928", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.object_key_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'object_key_prefix'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Configuration of the schedule of the query.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScheduleConfiguration {
    ///
    /// An expression that denotes when to trigger the scheduled query run. This can be a cron    expression or a rate expression.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ScheduleConfiguration {
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

/// Details on SNS that are required to send the notification.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SnsConfiguration {
    ///
    /// SNS topic ARN that the scheduled query status notifications will be sent to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TopicArn")]
    pub topic_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SnsConfiguration {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

/// Configuration used for writing the output of a query.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TargetConfiguration {
    ///
    /// Configuration needed to write data into the Timestream database and table.
    ///
    /// Required: Yes
    ///
    /// Type: TimestreamConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimestreamConfiguration")]
    pub timestream_configuration: TimestreamConfiguration,
}

impl cfn_resources::CfnResource for TargetConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.timestream_configuration.validate()?;

        Ok(())
    }
}

/// Configuration to write data into Timestream database and table. This configuration allows    the user to map the query result select columns into the destination table columns.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TimestreamConfiguration {
    ///
    /// Name of Timestream database to which the query result will be written.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    pub database_name: cfn_resources::StrVal,

    ///
    /// This is to allow mapping column(s) from the query result to the dimension in the    destination table.
    ///
    /// Required: Yes
    ///
    /// Type: List of DimensionMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "DimensionMappings")]
    pub dimension_mappings: Vec<DimensionMapping>,

    ///
    /// Name of the measure column. Also see MultiMeasureMappings and     MixedMeasureMappings for how measure name properties on those relate to     MeasureNameColumn.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MeasureNameColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_name_column: Option<cfn_resources::StrVal>,

    ///
    /// Specifies how to map measures to multi-measure records.
    ///
    /// Required: No
    ///
    /// Type: List of MixedMeasureMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "MixedMeasureMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_measure_mappings: Option<Vec<MixedMeasureMapping>>,

    ///
    /// Multi-measure mappings.
    ///
    /// Required: No
    ///
    /// Type: MultiMeasureMappings
    ///
    /// Update requires: Replacement
    #[serde(rename = "MultiMeasureMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_measure_mappings: Option<MultiMeasureMappings>,

    ///
    /// Name of Timestream table that the query result will be written to. The table should be    within the same database that is provided in Timestream configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableName")]
    pub table_name: cfn_resources::StrVal,

    ///
    /// Column from query result that should be used as the time column in destination table.    Column type for this should be TIMESTAMP.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeColumn")]
    pub time_column: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TimestreamConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.multi_measure_mappings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
