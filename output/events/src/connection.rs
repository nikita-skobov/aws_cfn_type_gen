/// Creates a connection. A connection defines the authorization type and credentials to use    for authorization with an API destination HTTP endpoint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub authorization_type: ConnectionAuthorizationTypeEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnConnectionarn,

    #[serde(skip_serializing)]
    pub att_secret_arn: CfnConnectionsecretarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ConnectionAuthorizationTypeEnum {
    /// API_KEY
    #[serde(rename = "API_KEY")]
    Apikey,

    /// BASIC
    #[serde(rename = "BASIC")]
    Basic,

    /// OAUTH_CLIENT_CREDENTIALS
    #[serde(rename = "OAUTH_CLIENT_CREDENTIALS")]
    Oauthclientcredentials,
}

impl Default for ConnectionAuthorizationTypeEnum {
    fn default() -> Self {
        ConnectionAuthorizationTypeEnum::Apikey
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectionarn;
impl CfnConnectionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectionsecretarn;
impl CfnConnectionsecretarn {
    pub fn att_name(&self) -> &'static str {
        r#"SecretArn"#
    }
}

impl cfn_resources::CfnResource for CfnConnection {
    fn type_string(&self) -> &'static str {
        "AWS::Events::Connection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.auth_parameters.validate()?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Contains the API key authorization parameters for the connection.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ApiKeyAuthParameters {
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
    pub api_key_name: cfn_resources::StrVal,

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
    pub api_key_value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ApiKeyAuthParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.api_key_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'api_key_name'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.api_key_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'api_key_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.api_key_value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'api_key_value'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.api_key_value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'api_key_value'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Contains the authorization parameters to use for the connection.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AuthParameters {
    ///
    /// The API Key parameters to use for authorization.
    ///
    /// Required: No
    ///
    /// Type: ApiKeyAuthParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeyAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_auth_parameters: Option<ApiKeyAuthParameters>,

    ///
    /// The authorization parameters for Basic authorization.
    ///
    /// Required: No
    ///
    /// Type: BasicAuthParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_parameters: Option<BasicAuthParameters>,

    ///
    /// Additional parameters for the connection that are passed through with every invocation to    the HTTP endpoint.
    ///
    /// Required: No
    ///
    /// Type: ConnectionHttpParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvocationHttpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,

    ///
    /// The OAuth parameters to use for authorization.
    ///
    /// Required: No
    ///
    /// Type: OAuthParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_parameters: Option<OAuthParameters>,
}

impl cfn_resources::CfnResource for AuthParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.api_key_auth_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.basic_auth_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.invocation_http_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.oauth_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains the Basic authorization parameters for the connection.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BasicAuthParameters {
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
    pub password: cfn_resources::StrVal,

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
    pub username: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for BasicAuthParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.password;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'password'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.password;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'password'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.username;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'username'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.username;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'username'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Contains the OAuth authorization parameters to use for the connection.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ClientParameters {
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
    pub client_id: cfn_resources::StrVal,

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
    pub client_secret: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ClientParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.client_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'client_id'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.client_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'client_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.client_secret;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'client_secret'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.client_secret;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'client_secret'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Contains additional parameters for the connection.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_parameters: Option<Vec<Parameter>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_parameters: Option<Vec<Parameter>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_parameters: Option<Vec<Parameter>>,
}

impl cfn_resources::CfnResource for ConnectionHttpParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.body_parameters {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'body_parameters'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.header_parameters {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'header_parameters'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.query_string_parameters {
            if the_val.len() > 100 as _ {
                return Err(format!("Max validation failed on field 'query_string_parameters'. {} is greater than 100", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Contains the OAuth authorization parameters to use for the connection.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct OAuthParameters {
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
    pub authorization_endpoint: cfn_resources::StrVal,

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
    pub http_method: OAuthParametersHttpMethodEnum,

    ///
    /// A ConnectionHttpParameters object that contains details about the additional    parameters to use for the connection.
    ///
    /// Required: No
    ///
    /// Type: ConnectionHttpParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "OAuthHttpParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_http_parameters: Option<ConnectionHttpParameters>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OAuthParametersHttpMethodEnum {
    /// GET
    #[serde(rename = "GET")]
    Get,

    /// POST
    #[serde(rename = "POST")]
    Post,

    /// PUT
    #[serde(rename = "PUT")]
    Put,
}

impl Default for OAuthParametersHttpMethodEnum {
    fn default() -> Self {
        OAuthParametersHttpMethodEnum::Get
    }
}

impl cfn_resources::CfnResource for OAuthParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.authorization_endpoint;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'authorization_endpoint'. {} is greater than 2048", s.len()));
            }
        }

        let the_val = &self.authorization_endpoint;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'authorization_endpoint'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.client_parameters.validate()?;

        self.oauth_http_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Additional query string parameter for the connection. You can include up to 100 additional    query string parameters per request. Each additional parameter counts towards the event    payload size, which cannot exceed 64 KB.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    ///
    /// Specifies whether the value is secret.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsValueSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_secret: Option<bool>,

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
    pub key: cfn_resources::StrVal,

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
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Parameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'key'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
