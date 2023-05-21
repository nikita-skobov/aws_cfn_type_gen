

/// Creates a connection. A connection defines the authorization type and credentials to use    for authorization with an API destination HTTP endpoint.
#[derive(Default, serde::Serialize)]
pub struct CfnConnection {


    /// 
    /// A CreateConnectionAuthRequestParameters object that contains the    authorization parameters to use to authorize with the endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: AuthParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthParameters")]
    pub auth_parameters: AuthParameters,


    /// 
    /// The name for the connection to create.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [\.\-_A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// A description for the connection to create.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The type of authorization to use for the connection.
    /// 
    /// NoteOAUTH tokens are refreshed when a 401 or 407 response is returned.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: API_KEY | BASIC | OAUTH_CLIENT_CREDENTIALS
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationType")]
    pub authorization_type: String,

}


/// Contains the authorization parameters to use for the connection.
#[derive(Default, serde::Serialize)]
pub struct AuthParameters {


    /// 
    /// The authorization parameters for Basic authorization.
    /// 
    /// Required: No
    ///
    /// Type: BasicAuthParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuthParameters")]
    pub basic_auth_parameters: Option<BasicAuthParameters>,


    /// 
    /// The API Key parameters to use for authorization.
    /// 
    /// Required: No
    ///
    /// Type: ApiKeyAuthParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeyAuthParameters")]
    pub api_key_auth_parameters: Option<ApiKeyAuthParameters>,


    /// 
    /// The OAuth parameters to use for authorization.
    /// 
    /// Required: No
    ///
    /// Type: OAuthParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthParameters")]
    pub oauth_parameters: Option<OAuthParameters>,


    /// 
    /// Additional parameters for the connection that are passed through with every invocation to    the HTTP endpoint.
    /// 
    /// Required: No
    ///
    /// Type: ConnectionHttpParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvocationHttpParameters")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,

}


/// Contains the Basic authorization parameters for the connection.
#[derive(Default, serde::Serialize)]
pub struct BasicAuthParameters {


    /// 
    /// The user name to use for Basic authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^[ \t]*[^\x00-\x1F:\x7F]+([ \t]+[^\x00-\x1F:\x7F]+)*[ \t]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: String,


    /// 
    /// The password associated with the user name to use for Basic authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^[ \t]*[^\x00-\x1F:\x7F]+([ \t]+[^\x00-\x1F:\x7F]+)*[ \t]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,

}


/// Contains the API key authorization parameters for the connection.
#[derive(Default, serde::Serialize)]
pub struct ApiKeyAuthParameters {


    /// 
    /// The value for the API key to use for authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^[ \t]*[^\x00-\x1F:\x7F]+([ \t]+[^\x00-\x1F:\x7F]+)*[ \t]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeyValue")]
    pub api_key_value: String,


    /// 
    /// The name of the API key to use for authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^[ \t]*[^\x00-\x1F:\x7F]+([ \t]+[^\x00-\x1F:\x7F]+)*[ \t]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeyName")]
    pub api_key_name: String,

}


/// Additional query string parameter for the connection. You can include up to 100 additional    query string parameters per request. Each additional parameter counts towards the event    payload size, which cannot exceed 64 KB.
#[derive(Default, serde::Serialize)]
pub struct Parameter {


    /// 
    /// The key for a query string parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [^\x00-\x1F\x7F]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value associated with the key for the query string parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [^\x00-\x09\x0B\x0C\x0E-\x1F\x7F]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// Specifies whether the value is secret.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsValueSecret")]
    pub is_value_secret: Option<bool>,

}


/// Contains additional parameters for the connection.
#[derive(Default, serde::Serialize)]
pub struct ConnectionHttpParameters {


    /// 
    /// Contains additional body string parameters for the connection.
    /// 
    /// Required: No
    ///
    /// Type: List of Parameter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "BodyParameters")]
    pub body_parameters: Option<Vec<Parameter>>,


    /// 
    /// Contains additional query string parameters for the connection.
    /// 
    /// Required: No
    ///
    /// Type: List of Parameter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStringParameters")]
    pub query_string_parameters: Option<Vec<Parameter>>,


    /// 
    /// Contains additional header parameters for the connection.
    /// 
    /// Required: No
    ///
    /// Type: List of Parameter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderParameters")]
    pub header_parameters: Option<Vec<Parameter>>,

}


/// Contains the OAuth authorization parameters to use for the connection.
#[derive(Default, serde::Serialize)]
pub struct OAuthParameters {


    /// 
    /// The method to use for the authorization request.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GET | POST | PUT
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpMethod")]
    pub http_method: String,


    /// 
    /// A ConnectionHttpParameters object that contains details about the additional    parameters to use for the connection.
    /// 
    /// Required: No
    ///
    /// Type: ConnectionHttpParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthHttpParameters")]
    pub oauth_http_parameters: Option<ConnectionHttpParameters>,


    /// 
    /// A CreateConnectionOAuthClientRequestParameters object that contains the    client parameters for OAuth authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: ClientParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientParameters")]
    pub client_parameters: ClientParameters,


    /// 
    /// The URL to the authorization endpoint when OAuth is specified as the authorization    type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^((%[0-9A-Fa-f]{2}|[-()_.!~*';/?:@\x26=+$,A-Za-z0-9])+)([).!';/?:,])?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationEndpoint")]
    pub authorization_endpoint: String,

}


/// Contains the OAuth authorization parameters to use for the connection.
#[derive(Default, serde::Serialize)]
pub struct ClientParameters {


    /// 
    /// The client secret assciated with the client ID to use for OAuth authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^[ \t]*[^\x00-\x1F:\x7F]+([ \t]+[^\x00-\x1F:\x7F]+)*[ \t]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,


    /// 
    /// The client ID to use for OAuth authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^[ \t]*[^\x00-\x1F:\x7F]+([ \t]+[^\x00-\x1F:\x7F]+)*[ \t]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientID")]
    pub client_id: String,

}