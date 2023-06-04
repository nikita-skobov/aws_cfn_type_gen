/// A cache policy.
///
/// When it's attached to a cache behavior, the cache policy determines the 			following:
///
/// The headers, cookies, and query strings that are included in the cache key are also included 			in requests that CloudFront sends to the origin. CloudFront sends a request when it can't find a 			valid object in its cache that matches the request's cache key. If you want to send 			values to the origin but not include them in the cache key, use 			OriginRequestPolicy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnCachePolicy {
    ///
    /// The cache policy configuration.
    ///
    /// Required: Yes
    ///
    /// Type: CachePolicyConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachePolicyConfig")]
    pub cache_policy_config: CachePolicyConfig,

    #[serde(skip_serializing)]
    pub att_id: CfnCachePolicyid,

    #[serde(skip_serializing)]
    pub att_last_modified_time: CfnCachePolicylastmodifiedtime,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCachePolicyid;
impl CfnCachePolicyid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCachePolicylastmodifiedtime;
impl CfnCachePolicylastmodifiedtime {
    pub fn att_name(&self) -> &'static str {
        r#"LastModifiedTime"#
    }
}

impl cfn_resources::CfnResource for CfnCachePolicy {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::CachePolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cache_policy_config.validate()?;

        Ok(())
    }
}

/// A cache policy configuration.
///
/// This configuration determines the following:
///
/// The headers, cookies, and query strings that are included in the cache key are also included 			in requests that CloudFront sends to the origin. CloudFront sends a request when it can't find a 			valid object in its cache that matches the request's cache key. If you want to send 			values to the origin but not include them in the cache key, use 			OriginRequestPolicy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CachePolicyConfig {
    ///
    /// A comment to describe the cache policy. The comment cannot be longer than 128 			characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub comment: Option<cfn_resources::StrVal>,

    ///
    /// The default amount of time, in seconds, that you want objects to stay in the CloudFront 			cache before CloudFront sends another request to the origin to see if the object has been 			updated. CloudFront uses this value as the object's time to live (TTL) only when the origin 			does not send Cache-Control or Expires 			headers with the object. For more information, see Managing How Long Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// The default value for this field is 86400 seconds (one day). If the value of 				MinTTL is more than 86400 seconds, then the default value for this 			field is the same as the value of MinTTL.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultTTL")]
    pub default_ttl: f64,

    ///
    /// The maximum amount of time, in seconds, that objects stay in the CloudFront cache before 			CloudFront sends another request to the origin to see if the object has been updated. CloudFront 			uses this value only when the origin sends Cache-Control or 				Expires headers with the object. For more information, see Managing How Long Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// The default value for this field is 31536000 seconds (one year). If the value of 				MinTTL or DefaultTTL is more than 31536000 seconds, then 			the default value for this field is the same as the value of 			DefaultTTL.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxTTL")]
    pub max_ttl: f64,

    ///
    /// The minimum amount of time, in seconds, that you want objects to stay in the CloudFront 			cache before CloudFront sends another request to the origin to see if the object has been 			updated. For more information, see Managing How Long Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinTTL")]
    pub min_ttl: f64,

    ///
    /// A unique name to identify the cache policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The HTTP headers, cookies, and URL query strings to include in the cache key. The values 			included in the cache key are also included in requests that CloudFront sends to the 			origin.
    ///
    /// Required: Yes
    ///
    /// Type: ParametersInCacheKeyAndForwardedToOrigin
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParametersInCacheKeyAndForwardedToOrigin")]
    pub parameters_in_cache_key_and_forwarded_to_origin: ParametersInCacheKeyAndForwardedToOrigin,
}

impl cfn_resources::CfnResource for CachePolicyConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.parameters_in_cache_key_and_forwarded_to_origin
            .validate()?;

        Ok(())
    }
}

/// An object that determines whether any cookies in viewer requests (and if so, which cookies) 			are included in the cache key and in requests that CloudFront sends to the origin.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CookiesConfig {
    ///
    /// Determines whether any cookies in viewer requests are included in the cache key and in 			requests that CloudFront sends to the origin. Valid values are:
    ///
    /// none – No cookies in viewer requests are included in the cache key or in 					requests that CloudFront sends to the origin. Even when this field is set to 					none, any cookies that are listed in an 					OriginRequestPolicy          are included in origin 					requests.                        whitelist – Only the cookies in viewer requests that are listed in the 					CookieNames type are included in the cache key and in requests that 					CloudFront sends to the origin.                        allExcept – All cookies in viewer requests are included in the cache key and 					in requests that CloudFront sends to the origin,            except          for those that are listed in the 					CookieNames type, which are not included.                        all – All cookies in viewer requests are included in the cache key and in 					requests that CloudFront sends to the origin.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: all | allExcept | none | whitelist
    ///
    /// Update requires: No interruption
    #[serde(rename = "CookieBehavior")]
    pub cookie_behavior: CookiesConfigCookieBehaviorEnum,

    ///
    /// Contains a list of cookie names.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cookies")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cookies: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CookiesConfigCookieBehaviorEnum {
    /// all
    #[serde(rename = "all")]
    All,

    /// allExcept
    #[serde(rename = "allExcept")]
    Allexcept,

    /// none
    #[serde(rename = "none")]
    None,

    /// whitelist
    #[serde(rename = "whitelist")]
    Whitelist,
}

impl Default for CookiesConfigCookieBehaviorEnum {
    fn default() -> Self {
        CookiesConfigCookieBehaviorEnum::All
    }
}

impl cfn_resources::CfnResource for CookiesConfig {
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

/// An object that determines whether any HTTP headers (and if so, which headers) are included 			in the cache key and in requests that CloudFront sends to the origin.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HeadersConfig {
    ///
    /// Determines whether any HTTP headers are included in the cache key and in requests that CloudFront 			sends to the origin. Valid values are:
    ///
    /// none – No HTTP headers are included in the cache key or in requests that CloudFront 					sends to the origin. Even when this field is set to none, any 					headers that are listed in an OriginRequestPolicy          are included in origin requests.                        whitelist – Only the HTTP headers that are listed in the Headers 					type are included in the cache key and in requests that CloudFront sends to the 					origin.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: none | whitelist
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderBehavior")]
    pub header_behavior: HeadersConfigHeaderBehaviorEnum,

    ///
    /// Contains a list of HTTP header names.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub headers: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum HeadersConfigHeaderBehaviorEnum {
    /// none
    #[serde(rename = "none")]
    None,

    /// whitelist
    #[serde(rename = "whitelist")]
    Whitelist,
}

impl Default for HeadersConfigHeaderBehaviorEnum {
    fn default() -> Self {
        HeadersConfigHeaderBehaviorEnum::None
    }
}

impl cfn_resources::CfnResource for HeadersConfig {
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

/// This object determines the values that CloudFront includes in the cache key. These values 			can include HTTP headers, cookies, and URL query strings. CloudFront uses the cache key to 			find an object in its cache that it can return to the viewer.
///
/// The headers, cookies, and query strings that are included in the cache key are also included 			in requests that CloudFront sends to the origin. CloudFront sends a request when it can't find an 			object in its cache that matches the request's cache key. If you want to send values to 			the origin but not include them in the cache key, use 			OriginRequestPolicy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ParametersInCacheKeyAndForwardedToOrigin {
    ///
    /// An object that determines whether any cookies in viewer requests (and if so, which cookies) 			are included in the cache key and in requests that CloudFront sends to the origin.
    ///
    /// Required: Yes
    ///
    /// Type: CookiesConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CookiesConfig")]
    pub cookies_config: CookiesConfig,

    ///
    /// A flag that can affect whether the Accept-Encoding HTTP header is 			included in the cache key and included in requests that CloudFront sends to the origin.
    ///
    /// This field is related to the EnableAcceptEncodingGzip field. If one or 			both of these fields is true       and the viewer request includes the Accept-Encoding 			header, then CloudFront does the following:
    ///
    /// Normalizes the value of the viewer's Accept-Encoding 					header               Includes the normalized header in the cache key               Includes the normalized header in the request to the origin, if a request is 					necessary
    ///
    /// For more information, see Compression support in the 				Amazon CloudFront Developer Guide.
    ///
    /// If you set this value to true, and this cache behavior also has an origin 			request policy attached, do not include the Accept-Encoding header in the 			origin request policy. CloudFront always includes the Accept-Encoding header in 			origin requests when the value of this field is true, so including this 			header in an origin request policy has no effect.
    ///
    /// If both of these fields are false, then CloudFront treats the 				Accept-Encoding header the same as any other HTTP header in the viewer 			request. By default, it's not included in the cache key and it's not included in origin 			requests. In this case, you can manually add Accept-Encoding to the headers 			whitelist like any other HTTP header.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableAcceptEncodingBrotli")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub enable_accept_encoding_brotli: Option<bool>,

    ///
    /// A flag that can affect whether the Accept-Encoding HTTP header is 			included in the cache key and included in requests that CloudFront sends to the origin.
    ///
    /// This field is related to the EnableAcceptEncodingBrotli field. If one or 			both of these fields is true       and the viewer request includes the Accept-Encoding 			header, then CloudFront does the following:
    ///
    /// Normalizes the value of the viewer's Accept-Encoding 					header               Includes the normalized header in the cache key               Includes the normalized header in the request to the origin, if a request is 					necessary
    ///
    /// For more information, see Compression support in the 				Amazon CloudFront Developer Guide.
    ///
    /// If you set this value to true, and this cache behavior also has an origin 			request policy attached, do not include the Accept-Encoding header in the 			origin request policy. CloudFront always includes the Accept-Encoding header in 			origin requests when the value of this field is true, so including this 			header in an origin request policy has no effect.
    ///
    /// If both of these fields are false, then CloudFront treats the 				Accept-Encoding header the same as any other HTTP header in the viewer 			request. By default, it's not included in the cache key and it's not included in origin 			requests. In this case, you can manually add Accept-Encoding to the headers 			whitelist like any other HTTP header.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableAcceptEncodingGzip")]
    pub enable_accept_encoding_gzip: bool,

    ///
    /// An object that determines whether any HTTP headers (and if so, which headers) are included 			in the cache key and in requests that CloudFront sends to the origin.
    ///
    /// Required: Yes
    ///
    /// Type: HeadersConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeadersConfig")]
    pub headers_config: HeadersConfig,

    ///
    /// An object that determines whether any URL query strings in viewer requests (and if so, which 			query strings) are included in the cache key and in requests that CloudFront sends to the 			origin.
    ///
    /// Required: Yes
    ///
    /// Type: QueryStringsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStringsConfig")]
    pub query_strings_config: QueryStringsConfig,
}

impl cfn_resources::CfnResource for ParametersInCacheKeyAndForwardedToOrigin {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cookies_config.validate()?;

        self.headers_config.validate()?;

        self.query_strings_config.validate()?;

        Ok(())
    }
}

/// An object that determines whether any URL query strings in viewer requests (and if so, which 			query strings) are included in the cache key and in requests that CloudFront sends to the 			origin.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QueryStringsConfig {
    ///
    /// Determines whether any URL query strings in viewer requests are included in the cache key 			and in requests that CloudFront sends to the origin. Valid values are:
    ///
    /// none – No query strings in viewer requests are included in the cache key or 					in requests that CloudFront sends to the origin. Even when this field is set to 					none, any query strings that are listed in an 					OriginRequestPolicy          are included in origin 					requests.                        whitelist – Only the query strings in viewer requests that are listed in the 					QueryStringNames type are included in the cache key and in requests 					that CloudFront sends to the origin.                        allExcept – All query strings in viewer requests are included in the cache 					key and in requests that CloudFront sends to the origin,            except          those that are listed in the 					QueryStringNames type, which are not included.                        all – All query strings in viewer requests are included in the cache key and 					in requests that CloudFront sends to the origin.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: all | allExcept | none | whitelist
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStringBehavior")]
    pub query_string_behavior: QueryStringsConfigQueryStringBehaviorEnum,

    ///
    /// Contains a list of query string names.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStrings")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub query_strings: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum QueryStringsConfigQueryStringBehaviorEnum {
    /// all
    #[serde(rename = "all")]
    All,

    /// allExcept
    #[serde(rename = "allExcept")]
    Allexcept,

    /// none
    #[serde(rename = "none")]
    None,

    /// whitelist
    #[serde(rename = "whitelist")]
    Whitelist,
}

impl Default for QueryStringsConfigQueryStringBehaviorEnum {
    fn default() -> Self {
        QueryStringsConfigQueryStringBehaviorEnum::All
    }
}

impl cfn_resources::CfnResource for QueryStringsConfig {
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
