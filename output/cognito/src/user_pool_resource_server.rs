

/// The AWS::Cognito::UserPoolResourceServer resource creates a new OAuth2.0    resource server and defines custom scopes in it.
#[derive(Default, serde::Serialize)]
pub struct CfnUserPoolResourceServer {


    /// 
    /// A unique resource server identifier for the resource server. This could be an HTTPS    endpoint where the resource server is located. For example:     https://my-weather-api.example.com.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\x21\x23-\x5B\x5D-\x7E]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Identifier")]
    pub identifier: String,


    /// 
    /// A friendly name for the resource server.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\w\s+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A list of scopes. Each scope is a map with keys ScopeName and     ScopeDescription.
    /// 
    /// Required: No
    ///
    /// Type: List of ResourceServerScopeType
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scopes")]
    pub scopes: Option<Vec<ResourceServerScopeType>>,


    /// 
    /// The user pool ID for the user pool.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 55
    ///
    /// Pattern: [\w-]+_[0-9a-zA-Z]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,

}


/// A resource server scope.
#[derive(Default, serde::Serialize)]
pub struct ResourceServerScopeType {


    /// 
    /// The name of the scope.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\x21\x23-\x2E\x30-\x5B\x5D-\x7E]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScopeName")]
    pub scope_name: String,


    /// 
    /// A description of the scope.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScopeDescription")]
    pub scope_description: String,

}
