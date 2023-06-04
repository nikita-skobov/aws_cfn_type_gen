/// The AWS::Lambda::EventInvokeConfig resource configures options for asynchronous invocation on a version or an alias.
///
/// By default, Lambda retries an asynchronous invocation twice if the function returns an error. It retains    events in a queue for up to six hours. When an event fails all processing attempts or stays in the asynchronous    invocation queue for too long, Lambda discards it.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnEventInvokeConfig {
    ///
    /// A destination for events after they have been sent to a function for processing.
    ///
    /// Destinations                                                   Function - The Amazon Resource Name (ARN) of a Lambda function.                        Queue - The ARN of a standard SQS queue.                        Topic - The ARN of a standard SNS topic.                        Event Bus - The ARN of an Amazon EventBridge event bus.
    ///
    /// Required: No
    ///
    /// Type: DestinationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub destination_config: Option<DestinationConfig>,

    ///
    /// The name of the Lambda function.
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ([a-zA-Z0-9-_]+)
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionName")]
    pub function_name: cfn_resources::StrVal,

    ///
    /// The maximum age of a request that Lambda sends to a function for processing.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 60
    ///
    /// Maximum: 21600
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub maximum_event_age_in_seconds: Option<i64>,

    ///
    /// The maximum number of times to retry when the function returns an error.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub maximum_retry_attempts: Option<i64>,

    ///
    /// The identifier of a version or alias.
    ///
    /// Version - A version number.        Alias - An alias name.        Latest - To specify the unpublished version, use      $LATEST.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (|[a-zA-Z0-9$_-]+)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Qualifier")]
    pub qualifier: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnEventInvokeConfig {
    fn type_string(&self) -> &'static str {
        "AWS::Lambda::EventInvokeConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.function_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'function_name'. {} is greater than 64",
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

        if let Some(the_val) = &self.maximum_event_age_in_seconds {
            if *the_val > 21600 as _ {
                return Err(format!("Max validation failed on field 'maximum_event_age_in_seconds'. {} is greater than 21600", the_val));
            }
        }

        if let Some(the_val) = &self.maximum_event_age_in_seconds {
            if *the_val < 60 as _ {
                return Err(format!("Min validation failed on field 'maximum_event_age_in_seconds'. {} is less than 60", the_val));
            }
        }

        if let Some(the_val) = &self.maximum_retry_attempts {
            if *the_val > 2 as _ {
                return Err(format!(
                    "Max validation failed on field 'maximum_retry_attempts'. {} is greater than 2",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.maximum_retry_attempts {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'maximum_retry_attempts'. {} is less than 0",
                    the_val
                ));
            }
        }

        let the_val = &self.qualifier;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'qualifier'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.qualifier;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'qualifier'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// A configuration object that specifies the destination of an event after Lambda processes it.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DestinationConfig {
    ///
    /// The destination configuration for failed invocations.
    ///
    /// Required: No
    ///
    /// Type: OnFailure
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnFailure")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub on_failure: Option<OnFailure>,

    ///
    /// The destination configuration for successful invocations.
    ///
    /// Required: No
    ///
    /// Type: OnSuccess
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnSuccess")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub on_success: Option<OnSuccess>,
}

impl cfn_resources::CfnResource for DestinationConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.on_failure
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.on_success
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A destination for events that failed processing.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OnFailure {
    ///
    /// The Amazon Resource Name (ARN) of the destination resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 350
    ///
    /// Pattern: ^$|arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for OnFailure {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.destination;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 350 as _ {
                return Err(format!(
                    "Max validation failed on field 'destination'. {} is greater than 350",
                    s.len()
                ));
            }
        }

        let the_val = &self.destination;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'destination'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// A destination for events that were processed successfully.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OnSuccess {
    ///
    /// The Amazon Resource Name (ARN) of the destination resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 350
    ///
    /// Pattern: ^$|arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for OnSuccess {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.destination;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 350 as _ {
                return Err(format!(
                    "Max validation failed on field 'destination'. {} is greater than 350",
                    s.len()
                ));
            }
        }

        let the_val = &self.destination;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'destination'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
