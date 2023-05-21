

/// Registers the specified version of the module with the CloudFormation service. Registering a module  makes it available for use in CloudFormation templates in your AWS account and  Region.
///
/// To specify a module version as the default version, use the AWS::CloudFormation::ModuleDefaultVersion resource.
///
/// For more information using modules, see Using modules to encapsulate and reuse resource   configurations and Registering extensions in the   CloudFormation User Guide. For information on developing modules, see Developing modules in the   CloudFormation CLI User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnModuleVersion {


    /// 
    /// A URL to the S3 bucket containing the package that contains the template fragment and schema files for the  module version to register.
    /// 
    /// NoteThe user registering the module version must be able to access the module package in the S3 bucket. That's, the   user needs to have GetObject   permissions for the package. For more information, see Actions, Resources, and Condition Keys for Amazon S3 in the    AWS Identity and Access Management User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModulePackage")]
    pub module_package: String,


    /// 
    /// The name of the module being registered.
    /// 
    /// Required: Yes
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
    pub module_name: String,

}



impl cfn_resources::CfnResource for CfnModuleVersion {
    fn type_string() -> &'static str {
        "AWS::CloudFormation::ModuleVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
