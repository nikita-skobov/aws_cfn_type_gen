

/// The AWS::Lambda::Alias resource creates an alias for a Lambda function version. Use aliases to    provide clients with a function identifier that you can update to invoke a different version.
///
/// You can also map an alias to split invocation requests between two versions. Use the     RoutingConfig parameter to specify a second version and the percentage of invocation requests that    it receives.
#[derive(Default, serde::Serialize)]
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
    pub description: Option<String>,


    /// 
    /// Specifies a provisioned concurrency configuration for a function's alias.
    /// 
    /// Required: No
    ///
    /// Type: ProvisionedConcurrencyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedConcurrencyConfig")]
    pub provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,


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
    pub name: String,


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
    pub function_version: String,


    /// 
    /// The routing     configuration of the alias.
    /// 
    /// Required: No
    ///
    /// Type: AliasRoutingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoutingConfig")]
    pub routing_config: Option<AliasRoutingConfiguration>,


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
    pub function_name: String,

}


/// The traffic-shifting configuration of a Lambda function alias.
#[derive(Default, serde::Serialize)]
pub struct VersionWeight {


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


    /// 
    /// The qualifier of the second version.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionVersion")]
    pub function_version: String,

}


/// The traffic-shifting configuration of a Lambda function alias.
#[derive(Default, serde::Serialize)]
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


/// A provisioned concurrency configuration for a function's alias.
#[derive(Default, serde::Serialize)]
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
