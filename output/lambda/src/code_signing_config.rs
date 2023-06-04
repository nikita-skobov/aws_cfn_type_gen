/// Details about a Code signing configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCodeSigningConfig {
    ///
    /// List of allowed publishers.
    ///
    /// Required: Yes
    ///
    /// Type: AllowedPublishers
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedPublishers")]
    pub allowed_publishers: AllowedPublishers,

    ///
    /// The code signing policy controls the validation failure action for signature mismatch or expiry.
    ///
    /// Required: No
    ///
    /// Type: CodeSigningPolicies
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeSigningPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_policies: Option<CodeSigningPolicies>,

    ///
    /// Code signing configuration description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_code_signing_config_arn: CfnCodeSigningConfigcodesigningconfigarn,

    #[serde(skip_serializing)]
    pub att_code_signing_config_id: CfnCodeSigningConfigcodesigningconfigid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCodeSigningConfigcodesigningconfigarn;
impl CfnCodeSigningConfigcodesigningconfigarn {
    pub fn att_name(&self) -> &'static str {
        r#"CodeSigningConfigArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCodeSigningConfigcodesigningconfigid;
impl CfnCodeSigningConfigcodesigningconfigid {
    pub fn att_name(&self) -> &'static str {
        r#"CodeSigningConfigId"#
    }
}

impl cfn_resources::CfnResource for CfnCodeSigningConfig {
    fn type_string(&self) -> &'static str {
        "AWS::Lambda::CodeSigningConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.allowed_publishers.validate()?;

        self.code_signing_policies
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 256",
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

/// List of signing profiles that can sign a code package.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AllowedPublishers {
    ///
    /// The Amazon Resource Name (ARN) for each of the signing profiles. A signing profile defines a trusted user    who can sign a code package.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningProfileVersionArns")]
    pub signing_profile_version_arns: Vec<String>,
}

impl cfn_resources::CfnResource for AllowedPublishers {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.signing_profile_version_arns;

        if the_val.len() > 20 as _ {
            return Err(format!("Max validation failed on field 'signing_profile_version_arns'. {} is greater than 20", the_val.len()));
        }

        Ok(())
    }
}

/// Code signing configuration policies specify the validation failure action for signature mismatch or    expiry.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CodeSigningPolicies {
    ///
    /// Code signing configuration policy for deployment validation failure. If you set the policy to    Enforce, Lambda blocks the deployment request if signature validation checks fail. If you set the    policy to Warn, Lambda allows the deployment and creates a CloudWatch log.
    ///
    /// Default value: Warn
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Enforce | Warn
    ///
    /// Update requires: No interruption
    #[serde(rename = "UntrustedArtifactOnDeployment")]
    pub untrusted_artifact_on_deployment: CodeSigningPoliciesUntrustedArtifactOnDeploymentEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CodeSigningPoliciesUntrustedArtifactOnDeploymentEnum {
    /// Enforce
    #[serde(rename = "Enforce")]
    Enforce,

    /// Warn
    #[serde(rename = "Warn")]
    Warn,
}

impl Default for CodeSigningPoliciesUntrustedArtifactOnDeploymentEnum {
    fn default() -> Self {
        CodeSigningPoliciesUntrustedArtifactOnDeploymentEnum::Enforce
    }
}

impl cfn_resources::CfnResource for CodeSigningPolicies {
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
