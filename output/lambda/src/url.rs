

/// The AWS::Lambda::Url resource creates a function URL with the specified configuration parameters. A function URL is a dedicated HTTP(S) endpoint that   you can use to invoke your function.
#[derive(Default, serde::Serialize)]
pub struct CfnUrl {


    /// 
    /// The type of authentication that your function URL uses. Set to AWS_IAM if you want to restrict access to authenticated  users only. Set to NONE if you want to bypass IAM authentication to create a public endpoint. For more information,  see Security and auth model for Lambda function URLs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AWS_IAM | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthType")]
    pub auth_type: String,


    /// 
    /// The Cross-Origin Resource Sharing (CORS) settings    for your function URL.
    /// 
    /// Required: No
    ///
    /// Type: Cors
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cors")]
    pub cors: Option<Cors>,


    /// 
    /// The name of the Lambda function.
    /// 
    /// Name formats                   Function name - my-function.        Function ARN - arn:aws:lambda:us-west-2:123456789012:function:my-function.        Partial ARN - 123456789012:function:my-function.
    /// 
    /// The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64    characters in length.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetFunctionArn")]
    pub target_function_arn: String,


    /// 
    /// Use one of the following options:
    /// 
    /// BUFFERED – This is the default option. Lambda invokes your function using the Invoke API operation. Invocation results     are available when the payload is complete. The maximum payload size is 6 MB.                        RESPONSE_STREAM – Your function streams payload results as they become available. Lambda invokes your function using     the InvokeWithResponseStream API operation. The maximum response payload size is 20 MB, however, you can request a quota increase.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BUFFERED | RESPONSE_STREAM
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvokeMode")]
    pub invoke_mode: Option<String>,


    /// 
    /// The alias name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Qualifier")]
    pub qualifier: Option<String>,

}


/// The Cross-Origin Resource Sharing (CORS)    settings for your function URL. Use CORS to grant access to your function URL from any origin. You can also use CORS    to control access for specific HTTP headers and methods in requests to your function URL.
#[derive(Default, serde::Serialize)]
pub struct Cors {


    /// 
    /// The HTTP headers in your function response that you want to expose to origins that call your function URL. For example:    Date, Keep-Alive, X-Custom-Header.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExposeHeaders")]
    pub expose_headers: Option<Vec<String>>,


    /// 
    /// The HTTP headers that origins can include in requests to your function URL. For example: Date, Keep-Alive,    X-Custom-Header.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowHeaders")]
    pub allow_headers: Option<Vec<String>>,


    /// 
    /// The HTTP methods that are allowed when calling your function URL. For example: GET, POST, DELETE,    or the wildcard character (*).
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 6
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowMethods")]
    pub allow_methods: Option<Vec<String>>,


    /// 
    /// The maximum amount of time, in seconds, that browsers can cache results of a preflight request. By default, this is set to 0,    which means the browser will not cache results.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 86400
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxAge")]
    pub max_age: Option<i64>,


    /// 
    /// The origins that can access your function URL. You can list any number of specific origins, separated by a comma. For example:    https://www.example.com, http://localhost:60905.
    /// 
    /// Alternatively, you can grant access to all origins with the wildcard character (*).
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowOrigins")]
    pub allow_origins: Option<Vec<String>>,


    /// 
    /// Whether you want to allow cookies or other credentials in requests to your function URL. The default is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowCredentials")]
    pub allow_credentials: Option<bool>,

}