/// Specifies the default version of a resource. The default version of a resource will be used in CloudFormation operations.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnResourceDefaultVersion {
    ///
    /// The name of the resource.
    ///
    /// Conditional: You must specify either TypeVersionArn, or TypeName and   VersionId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 204
    ///
    /// Pattern: [A-Za-z0-9]{2,64}::[A-Za-z0-9]{2,64}::[A-Za-z0-9]{2,64}(::MODULE){0,1}
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub type_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the resource version.
    ///
    /// Conditional: You must specify either TypeVersionArn, or TypeName and   VersionId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:aws[A-Za-z0-9-]{0,64}:cloudformation:[A-Za-z0-9-]{1,64}:[0-9]{12}:type/.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeVersionArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub type_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ID of a specific version of the resource. The version ID is the value at the end of the Amazon Resource Name  (ARN) assigned to the resource version when it's registered.
    ///
    /// Conditional: You must specify either TypeVersionArn, or TypeName and   VersionId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [A-Za-z0-9-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub version_id: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnResourceDefaultVersionarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceDefaultVersionarn;
impl CfnResourceDefaultVersionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnResourceDefaultVersion {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::ResourceDefaultVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.type_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 204 as _ {
                    return Err(format!(
                        "Max validation failed on field 'type_name'. {} is greater than 204",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.type_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 10 as _ {
                    return Err(format!(
                        "Min validation failed on field 'type_name'. {} is less than 10",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.type_version_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'type_version_arn'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.version_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'version_id'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.version_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'version_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
