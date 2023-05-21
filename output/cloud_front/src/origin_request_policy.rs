/// An origin request policy.
///
/// When it's attached to a cache behavior, the origin request policy determines the 			values that CloudFront includes in requests that it sends to the origin. Each request that 			CloudFront sends to the origin includes the following:
///
/// CloudFront sends a request when it can't find an object in its cache that matches the 			request. If you want to send values to the origin and also include them in the cache 			key, use CachePolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnOriginRequestPolicy {
    ///
    /// The origin request policy configuration.
    ///
    /// Required: Yes
    ///
    /// Type: OriginRequestPolicyConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginRequestPolicyConfig")]
    pub origin_request_policy_config: OriginRequestPolicyConfig,
}

impl cfn_resources::CfnResource for CfnOriginRequestPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::OriginRequestPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.origin_request_policy_config.validate()?;

        Ok(())
    }
}

/// An object that determines whether any cookies in viewer requests (and if so, which 			cookies) are included in requests that CloudFront sends to the origin.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CookiesConfig {
    ///
    /// Determines whether cookies in viewer requests are included in requests that CloudFront sends 			to the origin. Valid values are:
    ///
    /// none – No cookies in viewer requests are included in requests that CloudFront sends 					to the origin. Even when this field is set to none, any cookies 					that are listed in a CachePolicy          are included 					in origin requests.                        whitelist – Only the cookies in viewer requests that are listed in the 					CookieNames type are included in requests that CloudFront sends to the 					origin.                        all – All cookies in viewer requests are included in requests 					that CloudFront sends to the origin.                        allExcept – All cookies in viewer requests are included in 					requests that CloudFront sends to the origin,            except          for those listed in the CookieNames 					type, which are not included.
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
    pub cookies: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// An object that determines whether any HTTP headers (and if so, which headers) are 			included in requests that CloudFront sends to the origin.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HeadersConfig {
    ///
    /// Determines whether any HTTP headers are included in requests that CloudFront sends to the 			origin. Valid values are:
    ///
    /// none – No HTTP headers in viewer requests are included in requests that CloudFront 					sends to the origin. Even when this field is set to none, any 					headers that are listed in a CachePolicy          are 					included in origin requests.                        whitelist – Only the HTTP headers that are listed in the Headers 					type are included in requests that CloudFront sends to the origin.                        allViewer – All HTTP headers in viewer requests are included in 					requests that CloudFront sends to the origin.                        allViewerAndWhitelistCloudFront – All HTTP headers in viewer 					requests and the additional CloudFront headers that are listed in the 						Headers type are included in requests that CloudFront sends to the 					origin. The additional headers are added by CloudFront.                        allExcept – All HTTP headers in viewer requests are included in 					requests that CloudFront sends to the origin,            except          for those listed in the Headers type, 					which are not included.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: allExcept | allViewer | allViewerAndWhitelistCloudFront | none | whitelist
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
    pub headers: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum HeadersConfigHeaderBehaviorEnum {
    /// allExcept
    #[serde(rename = "allExcept")]
    Allexcept,

    /// allViewer
    #[serde(rename = "allViewer")]
    Allviewer,

    /// allViewerAndWhitelistCloudFront
    #[serde(rename = "allViewerAndWhitelistCloudFront")]
    Allviewerandwhitelistcloudfront,

    /// none
    #[serde(rename = "none")]
    None,

    /// whitelist
    #[serde(rename = "whitelist")]
    Whitelist,
}

impl Default for HeadersConfigHeaderBehaviorEnum {
    fn default() -> Self {
        HeadersConfigHeaderBehaviorEnum::Allexcept
    }
}

impl cfn_resources::CfnResource for HeadersConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// An origin request policy configuration.
///
/// This configuration determines the values that CloudFront includes in requests that it sends 			to the origin. Each request that CloudFront sends to the origin includes the following:
///
/// CloudFront sends a request when it can't find an object in its cache that matches the 			request. If you want to send values to the origin and also include them in the cache 			key, use CachePolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginRequestPolicyConfig {
    ///
    /// A comment to describe the origin request policy. The comment cannot be longer than 128 			characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

    ///
    /// The cookies from viewer requests to include in origin requests.
    ///
    /// Required: Yes
    ///
    /// Type: CookiesConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CookiesConfig")]
    pub cookies_config: CookiesConfig,

    ///
    /// The HTTP headers to include in origin requests. These can include headers from viewer 			requests and additional headers added by CloudFront.
    ///
    /// Required: Yes
    ///
    /// Type: HeadersConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeadersConfig")]
    pub headers_config: HeadersConfig,

    ///
    /// A unique name to identify the origin request policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The URL query strings from viewer requests to include in origin requests.
    ///
    /// Required: Yes
    ///
    /// Type: QueryStringsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStringsConfig")]
    pub query_strings_config: QueryStringsConfig,
}

impl cfn_resources::CfnResource for OriginRequestPolicyConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.cookies_config.validate()?;

        self.headers_config.validate()?;

        self.query_strings_config.validate()?;

        Ok(())
    }
}

/// An object that determines whether any URL query strings in viewer requests (and if so, 			which query strings) are included in requests that CloudFront sends to the origin.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct QueryStringsConfig {
    ///
    /// Determines whether any URL query strings in viewer requests are included in requests 			that CloudFront sends to the origin. Valid values are:
    ///
    /// none – No query strings in viewer requests are included in requests that CloudFront 					sends to the origin. Even when this field is set to none, any query 					strings that are listed in a CachePolicy          are 					included in origin requests.                        whitelist – Only the query strings in viewer requests that are listed in the 					QueryStringNames type are included in requests that CloudFront sends to 					the origin.                        all – All query strings in viewer requests are included in requests that CloudFront 					sends to the origin.                        allExcept – All query strings in viewer requests are included in 					requests that CloudFront sends to the origin,            except          for those listed in the 					QueryStringNames type, which are not included.
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
    pub query_strings: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
