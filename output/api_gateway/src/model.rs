/// The AWS::ApiGateway::Model resource defines the structure of a request or response payload for an API method.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnModel {
    ///
    /// The content-type for the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,

    ///
    /// The description of the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// A name for the model. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the model name. For more information, see Name Type.
    ///
    /// ImportantIf you specify a name, you cannot perform updates that require replacement of this resource. You can perform updates that require no or some interruption. If you must replace the resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// The string identifier of the associated RestApi.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,

    ///
    /// The schema for the model. For application/json models, this should be JSON schema draft 4 model. Do not include "\*/" characters in the description of any properties because such "\*/" characters may be interpreted as the closing marker for comments in some languages, such as Java or JavaScript, causing the installation of your API's SDK generated by API Gateway to fail.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schema")]
    pub schema: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for CfnModel {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::Model"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
