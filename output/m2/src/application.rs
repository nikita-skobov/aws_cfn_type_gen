

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
    pub description: Option<String>,


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
    pub kms_key_id: Option<String>,


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
    pub name: String,


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
    pub tags: Option<std::collections::HashMap<String, String>>,

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


impl cfn_resources::CfnResource for CfnApplication {
    fn type_string() -> &'static str {
        "AWS::M2::Application"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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
    pub content: Option<String>,


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
    pub s3_location: Option<String>,

}


