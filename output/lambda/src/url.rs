/// The AWS::Lambda::Url resource creates a function URL with the specified configuration parameters. A function URL is a dedicated HTTP(S) endpoint that   you can use to invoke your function.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub auth_type: UrlAuthTypeEnum,

    ///
    /// The Cross-Origin Resource Sharing (CORS) settings    for your function URL.
    ///
    /// Required: No
    ///
    /// Type: Cors
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_mode: Option<UrlInvokeModeEnum>,

    ///
    /// The alias name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<cfn_resources::StrVal>,

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
    pub target_function_arn: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_function_arn: CfnUrlfunctionarn,

    #[serde(skip_serializing)]
    pub att_function_url: CfnUrlfunctionurl,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UrlAuthTypeEnum {
    /// AWS_IAM
    #[serde(rename = "AWS_IAM")]
    Awsiam,

    /// NONE
    #[serde(rename = "NONE")]
    None,
}

impl Default for UrlAuthTypeEnum {
    fn default() -> Self {
        UrlAuthTypeEnum::Awsiam
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UrlInvokeModeEnum {
    /// BUFFERED
    #[serde(rename = "BUFFERED")]
    Buffered,

    /// RESPONSE_STREAM
    #[serde(rename = "RESPONSE_STREAM")]
    Responsestream,
}

impl Default for UrlInvokeModeEnum {
    fn default() -> Self {
        UrlInvokeModeEnum::Buffered
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUrlfunctionarn;
impl CfnUrlfunctionarn {
    pub fn att_name(&self) -> &'static str {
        r#"FunctionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUrlfunctionurl;
impl CfnUrlfunctionurl {
    pub fn att_name(&self) -> &'static str {
        r#"FunctionUrl"#
    }
}

impl cfn_resources::CfnResource for CfnUrl {
    fn type_string(&self) -> &'static str {
        "AWS::Lambda::Url"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cors.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The Cross-Origin Resource Sharing (CORS)    settings for your function URL. Use CORS to grant access to your function URL from any origin. You can also use CORS    to control access for specific HTTP headers and methods in requests to your function URL.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Cors {
    ///
    /// Whether you want to allow cookies or other credentials in requests to your function URL. The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
}

impl cfn_resources::CfnResource for Cors {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.allow_headers {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'allow_headers'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.allow_methods {
            if the_val.len() > 6 as _ {
                return Err(format!(
                    "Max validation failed on field 'allow_methods'. {} is greater than 6",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.allow_origins {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'allow_origins'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.expose_headers {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'expose_headers'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.max_age {
            if *the_val > 86400 as _ {
                return Err(format!(
                    "Max validation failed on field 'max_age'. {} is greater than 86400",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.max_age {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'max_age'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}
