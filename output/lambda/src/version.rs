

/// The AWS::Lambda::Version resource creates a version from the current code and configuration of a    function. Use versions to create a snapshot of your function code and configuration that doesn't change.
#[derive(Default, serde::Serialize)]
pub struct CfnVersion {


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


    /// 
    /// A description for the version to override the description in the function configuration. Updates are not    supported for this property.
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
    /// Specifies a provisioned concurrency configuration for a function's version. Updates are not supported for this    property.
    /// 
    /// Required: No
    ///
    /// Type: ProvisionedConcurrencyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedConcurrencyConfig")]
    pub provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,


    /// 
    /// Only publish a version if the hash value matches the value that's specified. Use this option to avoid    publishing a version if the function code has changed since you last updated it. Updates are not supported for    this property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeSha256")]
    pub code_sha256: Option<String>,

}


/// A provisioned concurrency configuration for a function's version.
#[derive(Default, serde::Serialize)]
pub struct ProvisionedConcurrencyConfiguration {


    /// 
    /// The amount of provisioned concurrency to allocate for the version.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedConcurrentExecutions")]
    pub provisioned_concurrent_executions: i64,

}
