/// Specifies a new application with given parameters. Requires an existing runtime     environment and application definition file.
///
/// For information about application definitions, see the AWS Mainframe Modernization User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplication {
    ///
    /// The application definition for a particular application. You can specify either inline     JSON or an Amazon S3 bucket location.
    ///
    /// For information about application definitions, see the AWS Mainframe Modernization User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Definition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Definition,

    ///
    /// The description of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The type of the target platform for this application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: bluage | microfocus
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineType")]
    pub engine_type: ApplicationEngineTypeEnum,

    ///
    /// The identifier of a customer managed key.
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
    /// The name of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [A-Za-z0-9][A-Za-z0-9_\-]{1,59}
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing)]
    pub att_application_arn: CfnApplicationapplicationarn,

    #[serde(skip_serializing)]
    pub att_application_id: CfnApplicationapplicationid,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ApplicationEngineTypeEnum {
    /// bluage
    #[serde(rename = "bluage")]
    Bluage,

    /// microfocus
    #[serde(rename = "microfocus")]
    Microfocus,
}

impl Default for ApplicationEngineTypeEnum {
    fn default() -> Self {
        ApplicationEngineTypeEnum::Bluage
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplicationapplicationarn;
impl CfnApplicationapplicationarn {
    pub fn att_name(&self) -> &'static str {
        r#"ApplicationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplicationapplicationid;
impl CfnApplicationapplicationid {
    pub fn att_name(&self) -> &'static str {
        r#"ApplicationId"#
    }
}

impl cfn_resources::CfnResource for CfnApplication {
    fn type_string(&self) -> &'static str {
        "AWS::M2::Application"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.definition.validate()?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 500 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 500",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The application definition for a particular application. You can specify either inline     JSON or an Amazon S3 bucket location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Definition {
    ///
    /// The content of the application definition. This is a JSON object that contains the     resource configuration/definitions that identify an application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<cfn_resources::StrVal>,

    ///
    /// The S3 bucket that contains the application definition.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: \S{1,2000}
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Definition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.content {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 65000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'content'. {} is greater than 65000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.content {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'content'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
