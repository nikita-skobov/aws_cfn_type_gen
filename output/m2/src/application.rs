

/// Specifies a new application with given parameters. Requires an existing runtime     environment and application definition file.
///
/// For information about application definitions, see the AWS Mainframe Modernization User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnApplication {


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
    pub engine_type: String,


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

}


/// The application definition for a particular application. You can specify either inline     JSON or an Amazon S3 bucket location.
#[derive(Default, serde::Serialize)]
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
