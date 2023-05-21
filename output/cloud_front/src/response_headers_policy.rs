

/// A response headers policy.
///
/// A response headers policy contains information about a set of HTTP response headers.
///
/// After you create a response headers policy, you can use its ID to attach it to one or more 			cache behaviors in a CloudFront distribution. When it's attached to a cache behavior, the 			response headers policy affects the HTTP headers that CloudFront includes in HTTP responses to 			requests that match the cache behavior. CloudFront adds or removes response headers according 			to the configuration of the response headers policy.
///
/// For more information, see Adding or removing HTTP headers in CloudFront responses in the 			Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResponseHeadersPolicy {


    /// 
    /// A response headers policy configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: ResponseHeadersPolicyConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseHeadersPolicyConfig")]
    pub response_headers_policy_config: ResponseHeadersPolicyConfig,

}



impl cfn_resources::CfnResource for CfnResponseHeadersPolicy {
    fn type_string() -> &'static str {
        "AWS::CloudFront::ResponseHeadersPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.response_headers_policy_config.validate()?;

        Ok(())
    }
}

/// A list of HTTP header names that CloudFront includes as values for the 				Access-Control-Allow-Headers HTTP response header.
///
/// For more information about the Access-Control-Allow-Headers HTTP response 			header, see Access-Control-Allow-Headers in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessControlAllowHeaders {


    /// 
    /// The list of HTTP header names. You can specify * to allow all 			headers.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<String>,

}



impl cfn_resources::CfnResource for AccessControlAllowHeaders {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A list of HTTP methods that CloudFront includes as values for the 				Access-Control-Allow-Methods HTTP response header.
///
/// For more information about the Access-Control-Allow-Methods HTTP response 			header, see Access-Control-Allow-Methods in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessControlAllowMethods {


    /// 
    /// The list of HTTP methods. Valid values are:
    /// 
    /// GET                                DELETE                                HEAD                                OPTIONS                                PATCH                                POST                                PUT                                ALL
    /// 
    /// ALL is a special value that includes all of the listed HTTP 			methods.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<String>,

}



impl cfn_resources::CfnResource for AccessControlAllowMethods {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A list of origins (domain names) that CloudFront can use as the value for the 				Access-Control-Allow-Origin HTTP response header.
///
/// For more information about the Access-Control-Allow-Origin HTTP response 			header, see Access-Control-Allow-Origin in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessControlAllowOrigins {


    /// 
    /// The list of origins (domain names). You can specify * to allow all 			origins.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<String>,

}



impl cfn_resources::CfnResource for AccessControlAllowOrigins {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A list of HTTP headers that CloudFront includes as values for the 				Access-Control-Expose-Headers HTTP response header.
///
/// For more information about the Access-Control-Expose-Headers HTTP 			response header, see Access-Control-Expose-Headers in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessControlExposeHeaders {


    /// 
    /// The list of HTTP headers. You can specify * to expose all headers.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<String>,

}



impl cfn_resources::CfnResource for AccessControlExposeHeaders {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The policy directives and their values that CloudFront includes as values for the 				Content-Security-Policy HTTP response header.
///
/// For more information about the Content-Security-Policy HTTP response 			header, see Content-Security-Policy in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContentSecurityPolicy {


    /// 
    /// The policy directives and their values that CloudFront includes as values for the 				Content-Security-Policy HTTP response header.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentSecurityPolicy")]
    pub content_security_policy: String,


    /// 
    /// A Boolean that determines whether CloudFront overrides the 				Content-Security-Policy HTTP response header received from the origin 			with the one specified in this response headers policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Override")]
    pub cfn_override: bool,

}



impl cfn_resources::CfnResource for ContentSecurityPolicy {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Determines whether CloudFront includes the X-Content-Type-Options HTTP response 			header with its value set to nosniff.
///
/// For more information about the X-Content-Type-Options HTTP response 			header, see X-Content-Type-Options in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContentTypeOptions {


    /// 
    /// A Boolean that determines whether CloudFront overrides the 				X-Content-Type-Options HTTP response header received from the origin 			with the one specified in this response headers policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Override")]
    pub cfn_override: bool,

}



impl cfn_resources::CfnResource for ContentTypeOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A configuration for a set of HTTP response headers that are used for cross-origin 			resource sharing (CORS). CloudFront adds these headers to HTTP responses that it sends for 			CORS requests that match a cache behavior associated with this response headers 			policy.
///
/// For more information about CORS, see Cross-Origin Resource 				Sharing (CORS) in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CorsConfig {


    /// 
    /// A Boolean that CloudFront uses as the value for the 				Access-Control-Allow-Credentials HTTP response header.
    /// 
    /// For more information about the Access-Control-Allow-Credentials HTTP 			response header, see Access-Control-Allow-Credentials in the MDN Web Docs.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlAllowCredentials")]
    pub access_control_allow_credentials: bool,


    /// 
    /// A list of HTTP header names that CloudFront includes as values for the 				Access-Control-Allow-Headers HTTP response header.
    /// 
    /// For more information about the Access-Control-Allow-Headers HTTP response 			header, see Access-Control-Allow-Headers in the MDN Web Docs.
    /// 
    /// Required: Yes
    ///
    /// Type: AccessControlAllowHeaders
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlAllowHeaders")]
    pub access_control_allow_headers: AccessControlAllowHeaders,


    /// 
    /// A list of HTTP methods that CloudFront includes as values for the 				Access-Control-Allow-Methods HTTP response header.
    /// 
    /// For more information about the Access-Control-Allow-Methods HTTP response 			header, see Access-Control-Allow-Methods in the MDN Web Docs.
    /// 
    /// Required: Yes
    ///
    /// Type: AccessControlAllowMethods
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlAllowMethods")]
    pub access_control_allow_methods: AccessControlAllowMethods,


    /// 
    /// A list of origins (domain names) that CloudFront can use as the value for the 				Access-Control-Allow-Origin HTTP response header.
    /// 
    /// For more information about the Access-Control-Allow-Origin HTTP response 			header, see Access-Control-Allow-Origin in the MDN Web Docs.
    /// 
    /// Required: Yes
    ///
    /// Type: AccessControlAllowOrigins
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlAllowOrigins")]
    pub access_control_allow_origins: AccessControlAllowOrigins,


    /// 
    /// A list of HTTP headers that CloudFront includes as values for the 				Access-Control-Expose-Headers HTTP response header.
    /// 
    /// For more information about the Access-Control-Expose-Headers HTTP 			response header, see Access-Control-Expose-Headers in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: AccessControlExposeHeaders
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlExposeHeaders")]
    pub access_control_expose_headers: Option<AccessControlExposeHeaders>,


    /// 
    /// A number that CloudFront uses as the value for the Access-Control-Max-Age HTTP 			response header.
    /// 
    /// For more information about the Access-Control-Max-Age HTTP response 			header, see Access-Control-Max-Age in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlMaxAgeSec")]
    pub access_control_max_age_sec: Option<i64>,


    /// 
    /// A Boolean that determines whether CloudFront overrides HTTP response headers received from 			the origin with the ones specified in this response headers policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginOverride")]
    pub origin_override: bool,

}



impl cfn_resources::CfnResource for CorsConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.access_control_allow_headers.validate()?;

        self.access_control_allow_methods.validate()?;

        self.access_control_allow_origins.validate()?;

        self.access_control_expose_headers.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An HTTP response header name and its value. CloudFront includes this header in HTTP 			responses that it sends for requests that match a cache behavior that's associated with 			this response headers policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomHeader {


    /// 
    /// The HTTP response header name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Header")]
    pub header: String,


    /// 
    /// A Boolean that determines whether CloudFront overrides a response header with the same name 			received from the origin with the header specified here.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Override")]
    pub cfn_override: bool,


    /// 
    /// The value for the HTTP response header.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for CustomHeader {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A list of HTTP response header names and their values. CloudFront includes these headers in 			HTTP responses that it sends for requests that match a cache behavior that's associated 			with this response headers policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomHeadersConfig {


    /// 
    /// The list of HTTP response headers and their values.
    /// 
    /// Required: Yes
    ///
    /// Type: List of CustomHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<CustomHeader>,

}



impl cfn_resources::CfnResource for CustomHeadersConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Determines whether CloudFront includes the X-Frame-Options HTTP response header 			and the header's value.
///
/// For more information about the X-Frame-Options HTTP response header, see 				X-Frame-Options in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrameOptions {


    /// 
    /// The value of the X-Frame-Options HTTP response header. Valid values are 				DENY and SAMEORIGIN.
    /// 
    /// For more information about these values, see X-Frame-Options in the MDN Web Docs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DENY | SAMEORIGIN
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameOption")]
    pub frame_option: FrameOptionsFrameOptionEnum,


    /// 
    /// A Boolean that determines whether CloudFront overrides the X-Frame-Options HTTP 			response header received from the origin with the one specified in this response headers 			policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Override")]
    pub cfn_override: bool,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FrameOptionsFrameOptionEnum {

    /// DENY
    #[serde(rename = "DENY")]
    Deny,

    /// SAMEORIGIN
    #[serde(rename = "SAMEORIGIN")]
    Sameorigin,

}

impl Default for FrameOptionsFrameOptionEnum {
    fn default() -> Self {
        FrameOptionsFrameOptionEnum::Deny
    }
}


impl cfn_resources::CfnResource for FrameOptions {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Determines whether CloudFront includes the Referrer-Policy HTTP response header 			and the header's value.
///
/// For more information about the Referrer-Policy HTTP response header, see 				Referrer-Policy in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReferrerPolicy {


    /// 
    /// A Boolean that determines whether CloudFront overrides the Referrer-Policy HTTP 			response header received from the origin with the one specified in this response headers 			policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Override")]
    pub cfn_override: bool,


    /// 
    /// The value of the Referrer-Policy HTTP response header. Valid values 			are:
    /// 
    /// no-referrer                                no-referrer-when-downgrade                                origin                                origin-when-cross-origin                                same-origin                                strict-origin                                strict-origin-when-cross-origin                                unsafe-url
    /// 
    /// For more information about these values, see Referrer-Policy in the MDN Web Docs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: no-referrer | no-referrer-when-downgrade | origin | origin-when-cross-origin | same-origin | strict-origin | strict-origin-when-cross-origin | unsafe-url
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferrerPolicy")]
    pub referrer_policy: ReferrerPolicyReferrerPolicyEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ReferrerPolicyReferrerPolicyEnum {

    /// no-referrer
    #[serde(rename = "no-referrer")]
    Noreferrer,

    /// no-referrer-when-downgrade
    #[serde(rename = "no-referrer-when-downgrade")]
    Noreferrerwhendowngrade,

    /// origin
    #[serde(rename = "origin")]
    Origin,

    /// origin-when-cross-origin
    #[serde(rename = "origin-when-cross-origin")]
    Originwhencrossorigin,

    /// same-origin
    #[serde(rename = "same-origin")]
    Sameorigin,

    /// strict-origin
    #[serde(rename = "strict-origin")]
    Strictorigin,

    /// strict-origin-when-cross-origin
    #[serde(rename = "strict-origin-when-cross-origin")]
    Strictoriginwhencrossorigin,

    /// unsafe-url
    #[serde(rename = "unsafe-url")]
    Unsafeurl,

}

impl Default for ReferrerPolicyReferrerPolicyEnum {
    fn default() -> Self {
        ReferrerPolicyReferrerPolicyEnum::Noreferrer
    }
}


impl cfn_resources::CfnResource for ReferrerPolicy {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The name of an HTTP header that CloudFront removes from HTTP responses to requests that match the 			cache behavior that this response headers policy is attached to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RemoveHeader {


    /// 
    /// The HTTP header name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Header")]
    pub header: String,

}



impl cfn_resources::CfnResource for RemoveHeader {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A list of HTTP header names that CloudFront removes from HTTP responses to requests that match the 			cache behavior that this response headers policy is attached to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RemoveHeadersConfig {


    /// 
    /// The list of HTTP header names.
    /// 
    /// Required: Yes
    ///
    /// Type: List of RemoveHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<RemoveHeader>,

}



impl cfn_resources::CfnResource for RemoveHeadersConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A response headers policy configuration.
///
/// A response headers policy configuration contains metadata about the response headers policy, 			and configurations for sets of HTTP response headers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResponseHeadersPolicyConfig {


    /// 
    /// A comment to describe the response headers policy.
    /// 
    /// The comment cannot be longer than 128 characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: Option<String>,


    /// 
    /// A configuration for a set of HTTP response headers that are used for cross-origin 			resource sharing (CORS).
    /// 
    /// Required: No
    ///
    /// Type: CorsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CorsConfig")]
    pub cors_config: Option<CorsConfig>,


    /// 
    /// A configuration for a set of custom HTTP response headers.
    /// 
    /// Required: No
    ///
    /// Type: CustomHeadersConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomHeadersConfig")]
    pub custom_headers_config: Option<CustomHeadersConfig>,


    /// 
    /// A name to identify the response headers policy.
    /// 
    /// The name must be unique for response headers policies in this AWS account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A configuration for a set of HTTP headers to remove from the HTTP response.
    /// 
    /// Required: No
    ///
    /// Type: RemoveHeadersConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveHeadersConfig")]
    pub remove_headers_config: Option<RemoveHeadersConfig>,


    /// 
    /// A configuration for a set of security-related HTTP response headers.
    /// 
    /// Required: No
    ///
    /// Type: SecurityHeadersConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityHeadersConfig")]
    pub security_headers_config: Option<SecurityHeadersConfig>,


    /// 
    /// A configuration for enabling the Server-Timing header in HTTP responses 			sent from CloudFront.
    /// 
    /// Required: No
    ///
    /// Type: ServerTimingHeadersConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerTimingHeadersConfig")]
    pub server_timing_headers_config: Option<ServerTimingHeadersConfig>,

}



impl cfn_resources::CfnResource for ResponseHeadersPolicyConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.cors_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.custom_headers_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.remove_headers_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.security_headers_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.server_timing_headers_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A configuration for a set of security-related HTTP response headers. CloudFront adds these 			headers to HTTP responses that it sends for requests that match a cache behavior 			associated with this response headers policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SecurityHeadersConfig {


    /// 
    /// The policy directives and their values that CloudFront includes as values for the 				Content-Security-Policy HTTP response header.
    /// 
    /// For more information about the Content-Security-Policy HTTP response 			header, see Content-Security-Policy in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: ContentSecurityPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentSecurityPolicy")]
    pub content_security_policy: Option<ContentSecurityPolicy>,


    /// 
    /// Determines whether CloudFront includes the X-Content-Type-Options HTTP response 			header with its value set to nosniff.
    /// 
    /// For more information about the X-Content-Type-Options HTTP response 			header, see X-Content-Type-Options in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: ContentTypeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentTypeOptions")]
    pub content_type_options: Option<ContentTypeOptions>,


    /// 
    /// Determines whether CloudFront includes the X-Frame-Options HTTP response header 			and the header's value.
    /// 
    /// For more information about the X-Frame-Options HTTP response header, see 				X-Frame-Options in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: FrameOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameOptions")]
    pub frame_options: Option<FrameOptions>,


    /// 
    /// Determines whether CloudFront includes the Referrer-Policy HTTP response header 			and the header's value.
    /// 
    /// For more information about the Referrer-Policy HTTP response header, see 				Referrer-Policy in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: ReferrerPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferrerPolicy")]
    pub referrer_policy: Option<ReferrerPolicy>,


    /// 
    /// Determines whether CloudFront includes the Strict-Transport-Security HTTP 			response header and the header's value.
    /// 
    /// For more information about the Strict-Transport-Security HTTP response 			header, see Strict-Transport-Security in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: StrictTransportSecurity
    ///
    /// Update requires: No interruption
    #[serde(rename = "StrictTransportSecurity")]
    pub strict_transport_security: Option<StrictTransportSecurity>,


    /// 
    /// Determines whether CloudFront includes the X-XSS-Protection HTTP response 			header and the header's value.
    /// 
    /// For more information about the X-XSS-Protection HTTP response header, see 				X-XSS-Protection in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: XSSProtection
    ///
    /// Update requires: No interruption
    #[serde(rename = "XSSProtection")]
    pub xssprotection: Option<XSSProtection>,

}



impl cfn_resources::CfnResource for SecurityHeadersConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.content_security_policy.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.content_type_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.frame_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.referrer_policy.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.strict_transport_security.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.xssprotection.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A configuration for enabling the Server-Timing header in HTTP responses 			sent from CloudFront.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerTimingHeadersConfig {


    /// 
    /// A Boolean that determines whether CloudFront adds the Server-Timing header to 			HTTP responses that it sends in response to requests that match a cache behavior that's 			associated with this response headers policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,


    /// 
    /// A number 0–100 (inclusive) that specifies the percentage of responses that you want 			CloudFront to add the Server-Timing header to. When you set the sampling rate to 			100, CloudFront adds the Server-Timing header to the HTTP response for every 			request that matches the cache behavior that this response headers policy is attached 			to. When you set it to 50, CloudFront adds the header to 50% of the responses for requests 			that match the cache behavior. You can set the sampling rate to any number 0–100 with up 			to four decimal places.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamplingRate")]
    pub sampling_rate: Option<f64>,

}



impl cfn_resources::CfnResource for ServerTimingHeadersConfig {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Determines whether CloudFront includes the Strict-Transport-Security HTTP 			response header and the header's value.
///
/// For more information about the Strict-Transport-Security HTTP response 			header, see Strict-Transport-Security in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StrictTransportSecurity {


    /// 
    /// A number that CloudFront uses as the value for the max-age directive in the 				Strict-Transport-Security HTTP response header.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlMaxAgeSec")]
    pub access_control_max_age_sec: i64,


    /// 
    /// A Boolean that determines whether CloudFront includes the includeSubDomains 			directive in the Strict-Transport-Security HTTP response header.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeSubdomains")]
    pub include_subdomains: Option<bool>,


    /// 
    /// A Boolean that determines whether CloudFront overrides the 				Strict-Transport-Security HTTP response header received from the origin 			with the one specified in this response headers policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Override")]
    pub cfn_override: bool,


    /// 
    /// A Boolean that determines whether CloudFront includes the preload directive in 			the Strict-Transport-Security HTTP response header.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Preload")]
    pub preload: Option<bool>,

}



impl cfn_resources::CfnResource for StrictTransportSecurity {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Determines whether CloudFront includes the X-XSS-Protection HTTP response 			header and the header's value.
///
/// For more information about the X-XSS-Protection HTTP response header, see 				X-XSS-Protection in the MDN Web Docs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct XSSProtection {


    /// 
    /// A Boolean that determines whether CloudFront includes the mode=block directive 			in the X-XSS-Protection header.
    /// 
    /// For more information about this directive, see X-XSS-Protection in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModeBlock")]
    pub mode_block: Option<bool>,


    /// 
    /// A Boolean that determines whether CloudFront overrides the X-XSS-Protection 			HTTP response header received from the origin with the one specified in this response 			headers policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Override")]
    pub cfn_override: bool,


    /// 
    /// A Boolean that determines the value of the X-XSS-Protection HTTP response 			header. When this setting is true, the value of the 				X-XSS-Protection header is 1. When this setting is 				false, the value of the X-XSS-Protection header is 				0.
    /// 
    /// For more information about these settings, see X-XSS-Protection in the MDN Web Docs.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protection")]
    pub protection: bool,


    /// 
    /// A reporting URI, which CloudFront uses as the value of the report directive in 			the X-XSS-Protection header.
    /// 
    /// You cannot specify a ReportUri when ModeBlock is 				true.
    /// 
    /// For more information about using a reporting URL, see X-XSS-Protection in the MDN Web Docs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReportUri")]
    pub report_uri: Option<String>,

}



impl cfn_resources::CfnResource for XSSProtection {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}