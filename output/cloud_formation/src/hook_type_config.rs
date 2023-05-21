

/// The HookTypeConfig resource specifies the configuration of a hook.
#[derive(Default, serde::Serialize)]
pub struct CfnHookTypeConfig {


    /// 
    /// The unique name for your hook. Specifies a three-part namespace for your hook, with a recommended pattern of   Organization::Service::Hook.
    /// 
    /// You must specify either TypeName and Configuration or TypeARN and   Configuration.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 196
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,


    /// 
    /// The Amazon Resource Number (ARN) for the hook to set Configuration for.
    /// 
    /// You must specify either TypeName and Configuration or TypeARN and   Configuration.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeArn")]
    pub type_arn: Option<String>,


    /// 
    /// Specifies the activated hook type configuration, in this AWS account and AWS Region.
    /// 
    /// You must specify either TypeName and Configuration or TypeARN and   Configuration.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: String,


    /// 
    /// Specifies the activated hook type configuration, in this AWS account and AWS Region.
    /// 
    /// Defaults to default alias. Hook types currently support default configuration alias.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationAlias")]
    pub configuration_alias: Option<String>,

}
