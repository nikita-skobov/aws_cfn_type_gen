

/// The AWS::ApiGateway::DocumentationPart resource creates a documentation part for an API. For more information, see Representation of API Documentation in API Gateway in the API Gateway Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDocumentationPart {


    /// 
    /// The location of the targeted API entity of the to-be-created documentation part.
    /// 
    /// Required: Yes
    ///
    /// Type: Location
    ///
    /// Update requires: Replacement
    #[serde(rename = "Location")]
    pub location: Location,


    /// 
    /// The new documentation content map of the targeted API entity. Enclosed key-value pairs are API-specific, but only OpenAPI-compliant key-value pairs can be exported and, hence, published.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Properties")]
    pub properties: String,


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

}

impl cfn_resources::CfnResource for CfnDocumentationPart {
    fn type_string() -> &'static str {
        "AWS::ApiGateway::DocumentationPart"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The Location property specifies the location of the Amazon API Gateway API entity that the documentation applies to. Location is a property of the AWS::ApiGateway::DocumentationPart resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Location {


    /// 
    /// The URL path of the target. It is a valid field for the API entity types of RESOURCE, METHOD, PATH_PARAMETER, QUERY_PARAMETER, REQUEST_HEADER, REQUEST_BODY, RESPONSE, RESPONSE_HEADER, and RESPONSE_BODY. The default value is / for the root resource. When an applicable child entity inherits the content of another entity of the same type with more general specifications of the other location attributes, the child entity's path attribute must match that of the parent entity as a prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    pub path: Option<String>,


    /// 
    /// The HTTP status code of a response. It is a valid field for the API entity types of RESPONSE, RESPONSE_HEADER, and RESPONSE_BODY. The default value is * for any status code. When an applicable child entity inherits the content of an entity of the same type with more general specifications of the other location attributes, the child entity's statusCode attribute must match that of the parent entity exactly.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^([1-5]\d\d|\*|\s*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "StatusCode")]
    pub status_code: Option<String>,


    /// 
    /// The HTTP verb of a method. It is a valid field for the API entity types of METHOD, PATH_PARAMETER, QUERY_PARAMETER, REQUEST_HEADER, REQUEST_BODY, RESPONSE, RESPONSE_HEADER, and RESPONSE_BODY. The default value is * for any method. When an applicable child entity inherits the content of an entity of the same type with more general specifications of the other location attributes, the child entity's method attribute must match that of the parent entity exactly.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Method")]
    pub method: Option<String>,


    /// 
    /// The name of the targeted API entity. It is a valid and required field for the API entity types of AUTHORIZER, MODEL, PATH_PARAMETER, QUERY_PARAMETER, REQUEST_HEADER, REQUEST_BODY and RESPONSE_HEADER. It is an invalid field for any other entity type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The type of API entity to which the documentation content applies. Valid values are API, AUTHORIZER, MODEL, RESOURCE, METHOD, PATH_PARAMETER, QUERY_PARAMETER, REQUEST_HEADER, REQUEST_BODY, RESPONSE, RESPONSE_HEADER, and RESPONSE_BODY. Content inheritance does not apply to any entity of the API, AUTHORIZER, METHOD, MODEL, REQUEST_BODY, or RESOURCE type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: API | AUTHORIZER | METHOD | MODEL | PATH_PARAMETER | QUERY_PARAMETER | REQUEST_BODY | REQUEST_HEADER | RESOURCE | RESPONSE | RESPONSE_BODY | RESPONSE_HEADER
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}
