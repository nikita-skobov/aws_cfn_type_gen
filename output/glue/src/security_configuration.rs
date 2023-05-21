/// Creates a new security configuration. A security configuration is a set of security properties that can be used by AWS Glue. You can use a security configuration to encrypt data at rest. For information about using security configurations in AWS Glue, see Encrypting Data Written by Crawlers, Jobs, and Development Endpoints.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSecurityConfiguration {
    ///
    /// The encryption configuration associated with this security configuration.
    ///
    /// Required: Yes
    ///
    /// Type: EncryptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,

    ///
    /// The name of the security configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnSecurityConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::SecurityConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption_configuration.validate()?;

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies how Amazon CloudWatch data should be encrypted.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchEncryption {
    ///
    /// The encryption mode to use for CloudWatch data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | SSE-KMS
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption_mode: Option<CloudWatchEncryptionCloudWatchEncryptionModeEnum>,

    ///
    /// The Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: arn:aws:kms:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CloudWatchEncryptionCloudWatchEncryptionModeEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// SSE-KMS
    #[serde(rename = "SSE-KMS")]
    Ssekms,
}

impl Default for CloudWatchEncryptionCloudWatchEncryptionModeEnum {
    fn default() -> Self {
        CloudWatchEncryptionCloudWatchEncryptionModeEnum::Disabled
    }
}

impl cfn_resources::CfnResource for CloudWatchEncryption {
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

/// Specifies an encryption configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionConfiguration {
    ///
    /// The encryption configuration for Amazon CloudWatch.
    ///
    /// Required: No
    ///
    /// Type: CloudWatchEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption: Option<CloudWatchEncryption>,

    ///
    /// The encryption configuration for job bookmarks.
    ///
    /// Required: No
    ///
    /// Type: JobBookmarksEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobBookmarksEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmarks_encryption: Option<JobBookmarksEncryption>,

    ///
    /// The encyption configuration for Amazon Simple Storage Service (Amazon S3) data.
    ///
    /// Required: No
    ///
    /// Type: S3Encryptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Encryptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryptions: Option<S3Encryptions>,
}

impl cfn_resources::CfnResource for EncryptionConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_watch_encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.job_bookmarks_encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_encryptions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies how job bookmark data should be encrypted.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JobBookmarksEncryption {
    ///
    /// The encryption mode to use for job bookmarks data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSE-KMS | DISABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobBookmarksEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmarks_encryption_mode: Option<JobBookmarksEncryptionJobBookmarksEncryptionModeEnum>,

    ///
    /// The Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: arn:aws:kms:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum JobBookmarksEncryptionJobBookmarksEncryptionModeEnum {
    /// CSE-KMS
    #[serde(rename = "CSE-KMS")]
    Csekms,

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,
}

impl Default for JobBookmarksEncryptionJobBookmarksEncryptionModeEnum {
    fn default() -> Self {
        JobBookmarksEncryptionJobBookmarksEncryptionModeEnum::Csekms
    }
}

impl cfn_resources::CfnResource for JobBookmarksEncryption {
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

/// Specifies how Amazon Simple Storage Service (Amazon S3) data should be encrypted.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Encryption {
    ///
    /// The Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: arn:aws:kms:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<cfn_resources::StrVal>,

    ///
    /// The encryption mode to use for Amazon S3 data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | SSE-KMS | SSE-S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption_mode: Option<S3EncryptionS3EncryptionModeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum S3EncryptionS3EncryptionModeEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// SSE-KMS
    #[serde(rename = "SSE-KMS")]
    Ssekms,

    /// SSE-S3
    #[serde(rename = "SSE-S3")]
    Sses3,
}

impl Default for S3EncryptionS3EncryptionModeEnum {
    fn default() -> Self {
        S3EncryptionS3EncryptionModeEnum::Disabled
    }
}

impl cfn_resources::CfnResource for S3Encryption {
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

/// The S3Encryptions property type specifies the encyption configuration for       Amazon Simple Storage Service (Amazon S3) data for a security configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Encryptions {}

impl cfn_resources::CfnResource for S3Encryptions {
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
