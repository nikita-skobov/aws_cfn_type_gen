

/// Specifies the default version of a module. The default version of the module will be used in CloudFormation operations for this account and Region.
///
/// To register a module version, use the AWS::CloudFormation::ModuleVersion resource.
///
/// For more information using modules, see Using modules to encapsulate and reuse resource   configurations and Registering extensions in the   AWS CloudFormation User Guide. For information on developing modules, see Developing modules in the  AWS CloudFormation CLI User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnModuleDefaultVersion {


    /// 
    /// The name of the module.
    /// 
    /// Conditional: You must specify either Arn, or ModuleName and  VersionId.
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
    /// Update requires: Replacement
    #[serde(rename = "ModuleName")]
    pub module_name: Option<String>,


    /// 
    /// The ID for the specific version of the module.
    /// 
    /// Conditional: You must specify either Arn, or ModuleName and  VersionId.
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
    /// Update requires: Replacement
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the module version to set as the default version.
    /// 
    /// Conditional: You must specify either Arn, or ModuleName and  VersionId.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:aws[A-Za-z0-9-]{0,64}:cloudformation:[A-Za-z0-9-]{1,64}:[0-9]{12}:type/.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}



impl cfn_resources::CfnResource for CfnModuleDefaultVersion {
    fn type_string() -> &'static str {
        "AWS::CloudFormation::ModuleDefaultVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
