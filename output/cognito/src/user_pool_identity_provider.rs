/// The AWS::Cognito::UserPoolIdentityProvider resource creates an identity    provider for a user pool.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnUserPoolIdentityProvider {
    ///
    /// A mapping of IdP attributes to standard and custom user pool attributes.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mapping: Option<serde_json::Value>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_details: Option<serde_json::Value>,

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
    pub provider_name: cfn_resources::StrVal,

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
    pub provider_type: UserPoolIdentityProviderProviderTypeEnum,

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
    pub user_pool_id: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserPoolIdentityProviderProviderTypeEnum {
    /// Facebook
    #[serde(rename = "Facebook")]
    Facebook,

    /// Google
    #[serde(rename = "Google")]
    Google,

    /// LoginWithAmazon
    #[serde(rename = "LoginWithAmazon")]
    Loginwithamazon,

    /// OIDC
    #[serde(rename = "OIDC")]
    Oidc,

    /// SAML
    #[serde(rename = "SAML")]
    Saml,

    /// SignInWithApple
    #[serde(rename = "SignInWithApple")]
    Signinwithapple,
}

impl Default for UserPoolIdentityProviderProviderTypeEnum {
    fn default() -> Self {
        UserPoolIdentityProviderProviderTypeEnum::Facebook
    }
}

impl cfn_resources::CfnResource for CfnUserPoolIdentityProvider {
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::UserPoolIdentityProvider"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.idp_identifiers {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'idp_identifiers'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.provider_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'provider_name'. {} is greater than 32",
                    s.len()
                ));
            }
        }

        let the_val = &self.provider_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'provider_name'. {} is less than 3",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 55 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_pool_id'. {} is greater than 55",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_pool_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
