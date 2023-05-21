

/// Specifies the default version of a resource. The default version of a resource will be used in CloudFormation operations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourceDefaultVersion {


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
    pub type_version_arn: Option<String>,


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
    pub version_id: Option<String>,


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
    pub type_name: Option<String>,

}

impl cfn_resources::CfnResource for CfnResourceDefaultVersion {
    fn type_string() -> &'static str {
        "AWS::CloudFormation::ResourceDefaultVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
