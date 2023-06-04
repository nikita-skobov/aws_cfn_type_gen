/// Creates an API destination, which is an HTTP invocation endpoint configured as a target    for events.
///
/// When using ApiDesinations with OAuth authentication we recommend these best practices:
///
/// When the Connection resource is created the secret will be passed to EventBridge and stored in the customer account using “Service Linked Secrets,”    effectively creating two secrets. This will minimize the cost because the original secret is only accessed when a CloudFormation template is created or updated,    not every time an event is sent to the ApiDestination. The secret stored in the customer account by EventBridge is the one used for each event sent to the    ApiDestination and AWS is responsible for the fees.
///
/// For examples of CloudFormation templates that use secrets, see Examples.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnApiDestination {
    ///
    /// The ARN of the connection to use for the API destination. The destination endpoint must    support the authorization type specified for the connection.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1600
    ///
    /// Pattern: ^arn:aws([a-z]|\-)*:events:([a-z]|\d|\-)*:([0-9]{12})?:connection\/[\.\-_A-Za-z0-9]+\/[\-A-Za-z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: cfn_resources::StrVal,

    ///
    /// A description for the API destination to create.
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The method to use for the request to the HTTP invocation endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DELETE | GET | HEAD | OPTIONS | PATCH | POST | PUT
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpMethod")]
    pub http_method: ApiDestinationHttpMethodEnum,

    ///
    /// The URL to the HTTP invocation endpoint for the API destination.
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
    #[serde(rename = "InvocationEndpoint")]
    pub invocation_endpoint: cfn_resources::StrVal,

    ///
    /// The maximum number of requests per second to send to the HTTP invocation endpoint.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvocationRateLimitPerSecond")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub invocation_rate_limit_per_second: Option<i64>,

    ///
    /// The name for the API destination to create.
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnApiDestinationarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ApiDestinationHttpMethodEnum {
    /// DELETE
    #[serde(rename = "DELETE")]
    Delete,

    /// GET
    #[serde(rename = "GET")]
    Get,

    /// HEAD
    #[serde(rename = "HEAD")]
    Head,

    /// OPTIONS
    #[serde(rename = "OPTIONS")]
    Options,

    /// PATCH
    #[serde(rename = "PATCH")]
    Patch,

    /// POST
    #[serde(rename = "POST")]
    Post,

    /// PUT
    #[serde(rename = "PUT")]
    Put,
}

impl Default for ApiDestinationHttpMethodEnum {
    fn default() -> Self {
        ApiDestinationHttpMethodEnum::Delete
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnApiDestinationarn;
impl CfnApiDestinationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnApiDestination {
    fn type_string(&self) -> &'static str {
        "AWS::Events::ApiDestination"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.connection_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1600 as _ {
                return Err(format!(
                    "Max validation failed on field 'connection_arn'. {} is greater than 1600",
                    s.len()
                ));
            }
        }

        let the_val = &self.connection_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'connection_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

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

        let the_val = &self.invocation_endpoint;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'invocation_endpoint'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.invocation_endpoint;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'invocation_endpoint'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.invocation_rate_limit_per_second {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'invocation_rate_limit_per_second'. {} is less than 1", the_val));
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
