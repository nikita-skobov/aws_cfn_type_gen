

/// The HookDefaultVersion resource specifies the default version of the hook. The default version of  the hook is used in CloudFormation operations for this AWS account and AWS Region.
#[derive(Default, serde::Serialize)]
pub struct CfnHookDefaultVersion {


    /// 
    /// The version ID of the type configuration.
    /// 
    /// You must specify either TypeVersionArn, or TypeName and VersionId.
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
    #[serde(rename = "TypeVersionArn")]
    pub type_version_arn: Option<String>,


    /// 
    /// The version ID of the type specified.
    /// 
    /// You must specify either TypeVersionArn, or TypeName and VersionId.
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
    pub version_id: Option<String>,


    /// 
    /// The name of the hook.
    /// 
    /// You must specify either TypeVersionArn, or TypeName and VersionId.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,

}
