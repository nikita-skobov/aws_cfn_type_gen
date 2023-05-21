/// Creates a new component that can be used to build, validate, test, and assess your 			image. The component is based on a YAML document that you specify using exactly one of 			the following methods:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnComponent {
    ///
    /// The change description of the component. Describes what change has been made in this 			version, or what makes this version different from other versions of this 			component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "ChangeDescription")]
    pub change_description: Option<String>,

    ///
    /// Component data contains inline YAML document content for the component. 			Alternatively, you can specify the uri of a YAML document file stored in 			Amazon S3. However, you cannot specify both properties.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16000
    ///
    /// Pattern: [^\x00]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Data")]
    pub data: Option<String>,

    ///
    /// Describes the contents of the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The ID of the KMS key that is used to encrypt this component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

    ///
    /// The name of the component.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[-_A-Za-z-0-9][-_A-Za-z0-9 ]{1,126}[-_A-Za-z-0-9]$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The operating system platform of the component.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Linux | Windows
    ///
    /// Update requires: Replacement
    #[serde(rename = "Platform")]
    pub platform: ComponentPlatformEnum,

    ///
    /// The operating system (OS) version supported by the component. If the OS information is 			available, a prefix match is performed against the base image OS version during image 			recipe creation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 25
    ///
    /// Update requires: Replacement
    #[serde(rename = "SupportedOsVersions")]
    pub supported_os_versions: Option<Vec<String>>,

    ///
    /// The tags that apply to the component.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// The uri of a YAML component document file. This must be an S3 URL 				(s3://bucket/key), and the requester must have permission to access the 			S3 bucket it points to. If you use Amazon S3, you can specify component content up to your 			service quota.
    ///
    /// Alternatively, you can specify the YAML document inline, using the component 				data property. You cannot specify both properties.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Uri")]
    pub uri: Option<String>,

    ///
    /// The component version. For example, 1.0.0.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ComponentPlatformEnum {
    /// Linux
    #[serde(rename = "Linux")]
    Linux,

    /// Windows
    #[serde(rename = "Windows")]
    Windows,
}

impl Default for ComponentPlatformEnum {
    fn default() -> Self {
        ComponentPlatformEnum::Linux
    }
}

impl cfn_resources::CfnResource for CfnComponent {
    fn type_string(&self) -> &'static str {
        "AWS::ImageBuilder::Component"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.change_description {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'change_description'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.change_description {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'change_description'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.data {
            if the_val.len() > 16000 as _ {
                return Err(format!(
                    "Max validation failed on field 'data'. {} is greater than 16000",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.data {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'data'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'description'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'kms_key_id'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'kms_key_id'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.supported_os_versions {
            if the_val.len() > 25 as _ {
                return Err(format!(
                    "Max validation failed on field 'supported_os_versions'. {} is greater than 25",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
