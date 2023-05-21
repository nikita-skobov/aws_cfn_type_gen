

/// Creates a self-service action.
#[derive(Default, serde::Serialize)]
pub struct CfnServiceAction {


    /// The language code.
    /// 
    /// en - English (default)jp - Japanesezh - Chinese
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,


    /// 
    /// The self-service action definition type. For example, SSM_AUTOMATION.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SSM_AUTOMATION
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefinitionType")]
    pub definition_type: String,


    /// 
    /// The self-service action name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The self-service action description.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A map that defines the self-service action.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DefinitionParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Vec<DefinitionParameter>,

}


/// The list of parameters in JSON format.    For example: [{\"Name\":\"InstanceId\",\"Type\":\"TARGET\"}] or [{\"Name\":\"InstanceId\",\"Type\":\"TEXT_VALUE\"}].
#[derive(Default, serde::Serialize)]
pub struct DefinitionParameter {


    /// 
    /// The value of the parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The parameter key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}
