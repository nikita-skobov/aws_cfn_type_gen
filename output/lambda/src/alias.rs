/// The AWS::Lambda::Alias resource creates an alias for a Lambda function version. Use aliases to    provide clients with a function identifier that you can update to invoke a different version.
///
/// You can also map an alias to split invocation requests between two versions. Use the     RoutingConfig parameter to specify a second version and the percentage of invocation requests that    it receives.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAlias {
    ///
    /// A description of the alias.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the Lambda function.
    ///
    /// Name formats                                            Function name - MyFunction.                        Function ARN - arn:aws:lambda:us-west-2:123456789012:function:MyFunction.                        Partial ARN - 123456789012:function:MyFunction.
    ///
    /// The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64    characters in length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Pattern: (arn:(aws[a-zA-Z-]*)?:lambda:)?([a-z]{2}(-gov)?-[a-z]+-\d{1}:)?(\d{12}:)?(function:)?([a-zA-Z0-9-_]+)(:(\$LATEST|[a-zA-Z0-9-_]+))?
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionName")]
    pub function_name: cfn_resources::StrVal,

    ///
    /// The function version that the alias invokes.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: (\$LATEST|[0-9]+)
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionVersion")]
    pub function_version: cfn_resources::StrVal,

    ///
    /// The name of the alias.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (?!^[0-9]+$)([a-zA-Z0-9-_]+)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Specifies a provisioned concurrency configuration for a function's alias.
    ///
    /// Required: No
    ///
    /// Type: ProvisionedConcurrencyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedConcurrencyConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,

    ///
    /// The routing     configuration of the alias.
    ///
    /// Required: No
    ///
    /// Type: AliasRoutingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

impl cfn_resources::CfnResource for CfnAlias {
    fn type_string(&self) -> &'static str {
        "AWS::Lambda::Alias"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.function_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 140 as _ {
                return Err(format!(
                    "Max validation failed on field 'function_name'. {} is greater than 140",
                    s.len()
                ));
            }
        }

        let the_val = &self.function_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'function_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.function_version;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'function_version'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        let the_val = &self.function_version;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'function_version'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
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

        self.provisioned_concurrency_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.routing_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The traffic-shifting configuration of a Lambda function alias.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AliasRoutingConfiguration {
    ///
    /// The second version, and the percentage of traffic that's routed to it.
    ///
    /// Required: Yes
    ///
    /// Type: List of VersionWeight
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalVersionWeights")]
    pub additional_version_weights: Vec<VersionWeight>,
}

impl cfn_resources::CfnResource for AliasRoutingConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A provisioned concurrency configuration for a function's alias.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ProvisionedConcurrencyConfiguration {
    ///
    /// The amount of provisioned concurrency to allocate for the alias.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedConcurrentExecutions")]
    pub provisioned_concurrent_executions: i64,
}

impl cfn_resources::CfnResource for ProvisionedConcurrencyConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The traffic-shifting configuration of a Lambda function alias.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct VersionWeight {
    ///
    /// The qualifier of the second version.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionVersion")]
    pub function_version: cfn_resources::StrVal,

    ///
    /// The percentage of traffic that the alias routes to the second version.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionWeight")]
    pub function_weight: f64,
}

impl cfn_resources::CfnResource for VersionWeight {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
