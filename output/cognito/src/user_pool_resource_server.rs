/// The AWS::Cognito::UserPoolResourceServer resource creates a new OAuth2.0    resource server and defines custom scopes in it.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub identifier: cfn_resources::StrVal,

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
    pub name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub user_pool_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnUserPoolResourceServer {
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::UserPoolResourceServer"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.identifier;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'identifier'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.identifier;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'identifier'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.scopes {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'scopes'. {} is greater than 100",
                    the_val.len()
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

/// A resource server scope.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceServerScopeType {
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
    pub scope_description: cfn_resources::StrVal,

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
    pub scope_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ResourceServerScopeType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.scope_description;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'scope_description'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.scope_description;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'scope_description'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.scope_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'scope_name'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.scope_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'scope_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
