/// Creates an API destination, which is an HTTP invocation endpoint configured as a target    for events.
///
/// When using ApiDesinations with OAuth authentication we recommend these best practices:
///
/// When the Connection resource is created the secret will be passed to EventBridge and stored in the customer account using “Service Linked Secrets,”    effectively creating two secrets. This will minimize the cost because the original secret is only accessed when a CloudFormation template is created or updated,    not every time an event is sent to the ApiDestination. The secret stored in the customer account by EventBridge is the one used for each event sent to the    ApiDestination and AWS is responsible for the fees.
///
/// For examples of CloudFormation templates that use secrets, see Examples.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub connection_arn: String,

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
    pub description: Option<String>,

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
    pub invocation_endpoint: String,

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
    pub name: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
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

impl cfn_resources::CfnResource for CfnApiDestination {
    fn type_string(&self) -> &'static str {
        "AWS::Events::ApiDestination"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.connection_arn;

        if the_val.len() > 1600 as _ {
            return Err(format!(
                "Max validation failed on field 'connection_arn'. {} is greater than 1600",
                the_val.len()
            ));
        }

        let the_val = &self.connection_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'connection_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.invocation_endpoint;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'invocation_endpoint'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.invocation_endpoint;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'invocation_endpoint'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.invocation_rate_limit_per_second {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'invocation_rate_limit_per_second'. {} is less than 1", the_val));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
