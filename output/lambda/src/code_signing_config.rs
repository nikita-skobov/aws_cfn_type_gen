

/// Details about a Code signing configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCodeSigningConfig {


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
    pub description: Option<String>,


    /// 
    /// The code signing policy controls the validation failure action for signature mismatch or expiry.
    /// 
    /// Required: No
    ///
    /// Type: CodeSigningPolicies
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeSigningPolicies")]
    pub code_signing_policies: Option<CodeSigningPolicies>,


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

}

impl cfn_resources::CfnResource for CfnCodeSigningConfig {
    fn type_string() -> &'static str {
        "AWS::Lambda::CodeSigningConfig"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Code signing configuration policies specify the validation failure action for signature mismatch or    expiry.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub untrusted_artifact_on_deployment: String,

}


/// List of signing profiles that can sign a code package.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
