/// The CreateTable operation adds a new table to an existing database in your account. In an     AWS account, table names must be at least unique within each Region if they    are in the same database. You may have identical table names in the same Region if the tables    are in separate databases. While creating the table, you must specify the table name, database    name, and the retention properties. Service quotas apply. See     code sample    for details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTable {
    ///
    /// The name of the Timestream database that contains this table.
    ///
    /// Length Constraints: Minimum length of 3 bytes. Maximum length of 256    bytes.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    pub database_name: cfn_resources::StrVal,

    ///
    /// Contains properties to set on the table when enabling magnetic store writes.
    ///
    /// This object has the following attributes:
    ///
    /// EnableMagneticStoreWrites: A boolean flag to enable      magnetic store writes.        MagneticStoreRejectedDataLocation: The location to write error      reports for records rejected, asynchronously, during magnetic store writes. Only       S3Configuration objects are allowed. The S3Configuration      object has the following attributes:                                         BucketName: The name of the S3 bucket.            EncryptionOption: The encryption option for the S3 location.        Valid values are S3 server-side encryption with an S3 managed key        (SSE_S3) or AWS managed key (        SSE_KMS).            KmsKeyId: The AWS KMS key ID to use when        encrypting with an AWS managed key.            ObjectKeyPrefix: The prefix to use option for the objects        stored in S3.          Both BucketName and EncryptionOption are required when S3Configuration is specified. If you      specify SSE_KMS as your EncryptionOption then       KmsKeyId is required.
    ///
    /// EnableMagneticStoreWrites attribute is required    when MagneticStoreWriteProperties is specified.     MagneticStoreRejectedDataLocation attribute is required when EnableMagneticStoreWrites is set to    true.
    ///
    /// See the following examples:
    ///
    /// JSON
    ///
    /// {  "Type" : AWS::Timestream::Table",   "Properties":{    "DatabaseName":"TestDatabase",    "TableName":"TestTable",    "MagneticStoreWriteProperties":{     "EnableMagneticStoreWrites":true,     "MagneticStoreRejectedDataLocation":{       "S3Configuration":{        "BucketName":"testbucket",        "EncryptionOption":"SSE_KMS",        "KmsKeyId":"1234abcd-12ab-34cd-56ef-1234567890ab",        "ObjectKeyPrefix":"prefix"       }     }    }  } }
    ///
    /// YAML
    ///
    /// Type: AWS::Timestream::Table DependsOn: TestDatabase Properties:  TableName: "TestTable"  DatabaseName: "TestDatabase"  MagneticStoreWriteProperties:   EnableMagneticStoreWrites: true   MagneticStoreRejectedDataLocation:    S3Configuration:     BucketName: "testbucket"     EncryptionOption: "SSE_KMS"     KmsKeyId: "1234abcd-12ab-34cd-56ef-1234567890ab"     ObjectKeyPrefix: "prefix"
    ///
    /// Required: No
    ///
    /// Type: MagneticStoreWriteProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "MagneticStoreWriteProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetic_store_write_properties: Option<MagneticStoreWriteProperties>,

    ///
    /// The retention duration for the memory store and magnetic store. This object has the    following attributes:
    ///
    /// MemoryStoreRetentionPeriodInHours: Retention duration for memory      store, in hours.        MagneticStoreRetentionPeriodInDays: Retention duration for      magnetic store, in days.
    ///
    /// Both attributes are of type string. Both attributes are required when RetentionProperties is specified.
    ///
    /// See the following examples:
    ///
    /// JSON
    ///
    /// {   "Type" : AWS::Timestream::Table",   "Properties" : {     "DatabaseName" : "TestDatabase",     "TableName" : "TestTable",     "RetentionProperties" : {       "MemoryStoreRetentionPeriodInHours": "24",       "MagneticStoreRetentionPeriodInDays": "7"     }   } }
    ///
    /// YAML
    ///
    /// Type: AWS::Timestream::Table DependsOn: TestDatabase Properties:   TableName: "TestTable"   DatabaseName: "TestDatabase"   RetentionProperties:     MemoryStoreRetentionPeriodInHours: "24"     MagneticStoreRetentionPeriodInDays: "7"
    ///
    /// Required: No
    ///
    /// Type: RetentionProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_properties: Option<RetentionProperties>,

    ///
    /// The name of the Timestream table.
    ///
    /// Length Constraints: Minimum length of 3 bytes. Maximum length of 256    bytes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<cfn_resources::StrVal>,

    ///
    /// The tags to add to the table
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnTablearn,

    #[serde(skip_serializing)]
    pub att_name: CfnTablename,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTablearn;
impl CfnTablearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTablename;
impl CfnTablename {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnTable {
    fn type_string(&self) -> &'static str {
        "AWS::Timestream::Table"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.magnetic_store_write_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.retention_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The location to write error reports for records rejected, asynchronously, during     magnetic store writes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MagneticStoreRejectedDataLocation {
    ///
    /// Configuration of an S3 location to write error reports for records rejected,     asynchronously, during magnetic store writes.
    ///
    /// Required: No
    ///
    /// Type: S3Configuration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3Configuration>,
}

impl cfn_resources::CfnResource for MagneticStoreRejectedDataLocation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The set of properties on a table for configuring magnetic store writes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MagneticStoreWriteProperties {
    ///
    /// A flag to enable magnetic store writes.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableMagneticStoreWrites")]
    pub enable_magnetic_store_writes: bool,

    ///
    /// The location to write error reports for records rejected asynchronously during magnetic     store writes.
    ///
    /// Required: No
    ///
    /// Type: MagneticStoreRejectedDataLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "MagneticStoreRejectedDataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetic_store_rejected_data_location: Option<MagneticStoreRejectedDataLocation>,
}

impl cfn_resources::CfnResource for MagneticStoreWriteProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.magnetic_store_rejected_data_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Retention properties contain the duration for which your time-series data must be stored     in the magnetic store and the memory store.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RetentionProperties {
    ///
    /// The duration for which data must be stored in the magnetic store.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MagneticStoreRetentionPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnetic_store_retention_period_in_days: Option<cfn_resources::StrVal>,

    ///
    /// The duration for which data must be stored in the memory store.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemoryStoreRetentionPeriodInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_store_retention_period_in_hours: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for RetentionProperties {
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

/// The configuration that specifies an S3 location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Configuration {
    ///
    /// The bucket name of the customer S3 bucket.
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
    pub bucket_name: cfn_resources::StrVal,

    ///
    /// The encryption option for the customer S3 location. Options are S3 server-side     encryption with an S3 managed key or AWS managed key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SSE_KMS | SSE_S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionOption")]
    pub encryption_option: S3ConfigurationEncryptionOptionEnum,

    ///
    /// The AWS KMS key ID for the customer S3 location when encrypting with an       AWS managed key.
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
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The object key preview for the customer S3 location.
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
    /// Update requires: No interruption
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
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

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_id'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kms_key_id'. {} is less than 1",
                        s.len()
                    ));
                }
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
