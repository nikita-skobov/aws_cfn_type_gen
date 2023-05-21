/// The AWS::Lambda::LayerVersion resource creates a Lambda layer from a ZIP archive.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLayerVersion {
    ///
    /// A list of compatible  instruction set architectures.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 2
    ///
    /// Update requires: Replacement
    #[serde(rename = "CompatibleArchitectures")]
    pub compatible_architectures: Option<Vec<String>>,

    ///
    /// A list of compatible function     runtimes. Used for filtering with ListLayers and ListLayerVersions.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 15
    ///
    /// Update requires: Replacement
    #[serde(rename = "CompatibleRuntimes")]
    pub compatible_runtimes: Option<Vec<String>>,

    ///
    /// The function layer archive.
    ///
    /// Required: Yes
    ///
    /// Type: Content
    ///
    /// Update requires: Replacement
    #[serde(rename = "Content")]
    pub content: Content,

    ///
    /// The description of the version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The name or Amazon Resource Name (ARN) of the layer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Pattern: (arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+)|[a-zA-Z0-9-_]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LayerName")]
    pub layer_name: Option<String>,

    ///
    /// The layer's software license. It can be any of the following:
    ///
    /// An SPDX license identifier. For example,      MIT.               The URL of a license hosted on the internet. For example,      https://opensource.org/licenses/MIT.               The full text of the license.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Update requires: Replacement
    #[serde(rename = "LicenseInfo")]
    pub license_info: Option<String>,
}

impl cfn_resources::CfnResource for CfnLayerVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Lambda::LayerVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.compatible_architectures {
            if the_val.len() > 2 as _ {
                return Err(format!("Max validation failed on field 'compatible_architectures'. {} is greater than 2", the_val.len()));
            }
        }

        if let Some(the_val) = &self.compatible_runtimes {
            if the_val.len() > 15 as _ {
                return Err(format!(
                    "Max validation failed on field 'compatible_runtimes'. {} is greater than 15",
                    the_val.len()
                ));
            }
        }

        self.content.validate()?;

        if let Some(the_val) = &self.description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'description'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.layer_name {
            if the_val.len() > 140 as _ {
                return Err(format!(
                    "Max validation failed on field 'layer_name'. {} is greater than 140",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.layer_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'layer_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.license_info {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'license_info'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A ZIP archive that contains the contents of an Lambda layer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Content {
    ///
    /// The Amazon S3 bucket of the layer archive.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[0-9A-Za-z\.\-_]*(?<!\.)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,

    ///
    /// The Amazon S3 key of the layer archive.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Key")]
    pub s3_key: String,

    ///
    /// For versioned objects, the version of the layer archive object to use.
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
    #[serde(rename = "S3ObjectVersion")]
    pub s3_object_version: Option<String>,
}

impl cfn_resources::CfnResource for Content {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.s3_bucket;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 's3_bucket'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.s3_bucket;

        if the_val.len() < 3 as _ {
            return Err(format!(
                "Min validation failed on field 's3_bucket'. {} is less than 3",
                the_val.len()
            ));
        }

        let the_val = &self.s3_key;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 's3_key'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.s3_key;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 's3_key'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.s3_object_version {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_object_version'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.s3_object_version {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 's3_object_version'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
