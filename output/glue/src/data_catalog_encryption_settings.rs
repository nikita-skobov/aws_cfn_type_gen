/// Sets the security configuration for a specified catalog. After the configuration has been    set, the specified encryption is applied to every catalog write thereafter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDataCatalogEncryptionSettings {
    ///
    /// The ID of the Data Catalog in which the settings are created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: cfn_resources::StrVal,

    ///
    /// Contains configuration information for maintaining Data Catalog security.
    ///
    /// Required: Yes
    ///
    /// Type: DataCatalogEncryptionSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCatalogEncryptionSettings")]
    pub data_catalog_encryption_settings: Box<DataCatalogEncryptionSettings>,
}

impl cfn_resources::CfnResource for CfnDataCatalogEncryptionSettings {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::DataCatalogEncryptionSettings"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_catalog_encryption_settings.validate()?;

        Ok(())
    }
}

/// The data structure used by the Data Catalog to encrypt the password as part of     CreateConnection or UpdateConnection and store it in the     ENCRYPTED_PASSWORD field in the connection properties. You can enable catalog    encryption or only password encryption.
///
/// When a CreationConnection request arrives containing a password, the Data    Catalog first encrypts the password using your AWS KMS key. It then encrypts the whole    connection object again if catalog encryption is also enabled.
///
/// This encryption requires that you set AWS KMS key permissions to enable or restrict access    on the password key according to your security requirements. For example, you might want only    administrators to have decrypt permission on the password key.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConnectionPasswordEncryption {
    ///
    /// An AWS KMS key that is used to encrypt the connection password.
    ///
    /// If connection password protection is enabled, the caller of         CreateConnection and UpdateConnection needs at least         kms:Encrypt permission on the specified AWS KMS key, to encrypt       passwords before storing them in the Data Catalog. You can set the decrypt permission to       enable or restrict access on the password key according to your security       requirements.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// When the ReturnConnectionPasswordEncrypted flag is set to "true", passwords remain encrypted in the responses of GetConnection and GetConnections. This encryption takes effect independently from catalog encryption.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReturnConnectionPasswordEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_connection_password_encrypted: Option<bool>,
}

impl cfn_resources::CfnResource for ConnectionPasswordEncryption {
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

/// Contains configuration information for maintaining Data Catalog security.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataCatalogEncryptionSettings {
    ///
    /// When connection password protection is enabled, the Data Catalog uses a customer-provided    key to encrypt the password as part of CreateConnection or     UpdateConnection and store it in the ENCRYPTED_PASSWORD field in    the connection properties. You can enable catalog encryption or only password    encryption.
    ///
    /// Required: No
    ///
    /// Type: ConnectionPasswordEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionPasswordEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_password_encryption: Option<ConnectionPasswordEncryption>,

    ///
    /// Specifies the encryption-at-rest configuration for the Data Catalog.
    ///
    /// Required: No
    ///
    /// Type: EncryptionAtRest
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionAtRest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
}

impl cfn_resources::CfnResource for DataCatalogEncryptionSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.connection_password_encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.encryption_at_rest
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the encryption-at-rest configuration for the Data Catalog.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncryptionAtRest {
    ///
    /// The encryption-at-rest mode for encrypting Data Catalog data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | SSE-KMS
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_encryption_mode: Option<EncryptionAtRestCatalogEncryptionModeEnum>,

    ///
    /// The ID of the AWS KMS key to use for encryption at rest.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SseAwsKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_aws_kms_key_id: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum EncryptionAtRestCatalogEncryptionModeEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// SSE-KMS
    #[serde(rename = "SSE-KMS")]
    Ssekms,
}

impl Default for EncryptionAtRestCatalogEncryptionModeEnum {
    fn default() -> Self {
        EncryptionAtRestCatalogEncryptionModeEnum::Disabled
    }
}

impl cfn_resources::CfnResource for EncryptionAtRest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.sse_aws_kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'sse_aws_kms_key_id'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.sse_aws_kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'sse_aws_kms_key_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
