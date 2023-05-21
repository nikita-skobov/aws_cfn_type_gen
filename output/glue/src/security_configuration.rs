

/// Creates a new security configuration. A security configuration is a set of security properties that can be used by AWS Glue. You can use a security configuration to encrypt data at rest. For information about using security configurations in AWS Glue, see Encrypting Data Written by Crawlers, Jobs, and Development Endpoints.
#[derive(Default, serde::Serialize)]
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
    pub name: String,

}


/// Specifies how Amazon Simple Storage Service (Amazon S3) data should be encrypted.
#[derive(Default, serde::Serialize)]
pub struct S3Encryption {


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
    pub s3_encryption_mode: Option<String>,


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
    pub kms_key_arn: Option<String>,

}


/// Specifies how job bookmark data should be encrypted.
#[derive(Default, serde::Serialize)]
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
    pub job_bookmarks_encryption_mode: Option<String>,


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
    pub kms_key_arn: Option<String>,

}


/// Specifies how Amazon CloudWatch data should be encrypted.
#[derive(Default, serde::Serialize)]
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
    pub cloud_watch_encryption_mode: Option<String>,


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
    pub kms_key_arn: Option<String>,

}


/// The S3Encryptions property type specifies the encyption configuration for       Amazon Simple Storage Service (Amazon S3) data for a security configuration.
#[derive(Default, serde::Serialize)]
pub struct S3Encryptions {

}


/// Specifies an encryption configuration.
#[derive(Default, serde::Serialize)]
pub struct EncryptionConfiguration {


    /// 
    /// The encryption configuration for job bookmarks.
    /// 
    /// Required: No
    ///
    /// Type: JobBookmarksEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobBookmarksEncryption")]
    pub job_bookmarks_encryption: Option<JobBookmarksEncryption>,


    /// 
    /// The encryption configuration for Amazon CloudWatch.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchEncryption")]
    pub cloud_watch_encryption: Option<CloudWatchEncryption>,


    /// 
    /// The encyption configuration for Amazon Simple Storage Service (Amazon S3) data.
    /// 
    /// Required: No
    ///
    /// Type: S3Encryptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Encryptions")]
    pub s3_encryptions: Option<S3Encryptions>,

}
