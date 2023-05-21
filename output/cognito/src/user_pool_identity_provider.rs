

/// The AWS::Cognito::UserPoolIdentityProvider resource creates an identity    provider for a user pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPoolIdentityProvider {


    /// 
    /// The IdP details. The following list describes the provider detail keys for each IdP       type.
    /// 
    /// For Google and Login with Amazon:                                                         client_id                     client_secret                     authorize_scopes                        For Facebook:                                                                   client_id                     client_secret                     authorize_scopes                     api_version                        For Sign in with Apple:                                                                             client_id                     team_id                     key_id                     private_key                     authorize_scopes                        For OpenID Connect (OIDC) providers:                                                                                                 client_id                     client_secret                     attributes_request_method                     oidc_issuer                     authorize_scopes                     The following keys are only present if Amazon Cognito didn't discover them at               the oidc_issuer URL.                                                                                        authorize_url                           token_url                           attributes_url                           jwks_uri                                  Amazon Cognito sets the value of the following keys automatically. They are               read-only.                                                 attributes_url_add_attributes                                     For SAML providers:                                               MetadataFile or MetadataURL                     IDPSignout optional
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProviderDetails")]
    pub provider_details: Option<serde_json::Value>,


    /// 
    /// The IdP type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Facebook | Google | LoginWithAmazon | OIDC | SAML | SignInWithApple
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProviderType")]
    pub provider_type: String,


    /// 
    /// The user pool ID.
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


    /// 
    /// A list of IdP identifiers.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdpIdentifiers")]
    pub idp_identifiers: Option<Vec<String>>,


    /// 
    /// The IdP name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 32
    ///
    /// Pattern: [^_][\p{L}\p{M}\p{S}\p{N}\p{P}][^_]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProviderName")]
    pub provider_name: String,


    /// 
    /// A mapping of IdP attributes to standard and custom user pool attributes.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeMapping")]
    pub attribute_mapping: Option<serde_json::Value>,

}

impl cfn_resources::CfnResource for CfnUserPoolIdentityProvider {
    fn type_string() -> &'static str {
        "AWS::Cognito::UserPoolIdentityProvider"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
