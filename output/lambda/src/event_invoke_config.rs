

/// The AWS::Lambda::EventInvokeConfig resource configures options for asynchronous invocation on a version or an alias.
///
/// By default, Lambda retries an asynchronous invocation twice if the function returns an error. It retains    events in a queue for up to six hours. When an event fails all processing attempts or stays in the asynchronous    invocation queue for too long, Lambda discards it.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub destination_config: Option<DestinationConfig>,


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
    pub maximum_retry_attempts: Option<i64>,


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
    pub function_name: String,


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
    pub qualifier: String,

}



impl cfn_resources::CfnResource for CfnEventInvokeConfig {
    fn type_string() -> &'static str {
        "AWS::Lambda::EventInvokeConfig"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A destination for events that were processed successfully.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub destination: String,

}




/// A destination for events that failed processing.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub destination: String,

}




/// A configuration object that specifies the destination of an event after Lambda processes it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DestinationConfig {


    /// 
    /// The destination configuration for successful invocations.
    /// 
    /// Required: No
    ///
    /// Type: OnSuccess
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnSuccess")]
    pub on_success: Option<OnSuccess>,


    /// 
    /// The destination configuration for failed invocations.
    /// 
    /// Required: No
    ///
    /// Type: OnFailure
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnFailure")]
    pub on_failure: Option<OnFailure>,

}


