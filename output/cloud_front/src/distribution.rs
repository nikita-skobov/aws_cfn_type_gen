/// A distribution tells CloudFront where you want content to be delivered from, and the details 			about how to track and manage content delivery.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDistribution {
    ///
    /// The distribution's configuration.
    ///
    /// Required: Yes
    ///
    /// Type: DistributionConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DistributionConfig")]
    pub distribution_config: DistributionConfig,

    ///
    /// A complex type that contains zero or more Tag elements.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnDistribution {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::Distribution"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.distribution_config.validate()?;

        Ok(())
    }
}

/// A complex type that describes how CloudFront processes requests.
///
/// You must create at least as many cache behaviors (including the default cache 			behavior) as you have origins if you want CloudFront to serve objects from all of the origins. 			Each cache behavior specifies the one origin from which you want CloudFront to get objects. If 			you have two origins and only the default cache behavior, the default cache behavior 			will cause CloudFront to get objects from one of the origins, but the other origin is never 			used.
///
/// For the current quota (formerly known as limit) on the number of cache behaviors that 			you can add to a distribution, see Quotas in the 			Amazon CloudFront Developer Guide.
///
/// If you don't want to specify any cache behaviors, include only an empty 				CacheBehaviors element. Don't include an empty 				CacheBehavior element because this is invalid.
///
/// To delete all cache behaviors in an existing distribution, update the distribution 			configuration and include only an empty CacheBehaviors element.
///
/// To add, change, or remove one or more cache behaviors, update the distribution 			configuration and specify all of the cache behaviors that you want to include in the 			updated distribution.
///
/// For more information about cache behaviors, see Cache Behavior Settings in the Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CacheBehavior {
    ///
    /// A complex type that controls which HTTP methods CloudFront processes and forwards to your 			Amazon S3 bucket or your custom origin. There are three choices:
    ///
    /// CloudFront forwards only GET and HEAD requests.               CloudFront forwards only GET, HEAD, and 						OPTIONS requests.               CloudFront forwards GET, HEAD, OPTIONS, PUT, PATCH, POST, and 						DELETE requests.
    ///
    /// If you pick the third choice, you may need to restrict access to your Amazon S3 bucket or 			to your custom origin so users can't perform operations that you don't want them to. For 			example, you might not want users to have permissions to delete objects from your 			origin.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<Vec<String>>,

    ///
    /// The unique identifier of the cache policy that is attached to this cache behavior. For 			more information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// A CacheBehavior must include either a CachePolicyId or 				ForwardedValues. We recommend that you use a 			CachePolicyId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachePolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy_id: Option<cfn_resources::StrVal>,

    ///
    /// A complex type that controls whether CloudFront caches the response to requests using the 			specified HTTP methods. There are two choices:
    ///
    /// CloudFront caches responses to GET and HEAD 					requests.               CloudFront caches responses to GET, HEAD, and 						OPTIONS requests.
    ///
    /// If you pick the second choice for your Amazon S3 Origin, you may need to forward 			Access-Control-Request-Method, Access-Control-Request-Headers, and Origin headers for 			the responses to be cached correctly.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_methods: Option<Vec<String>>,

    ///
    /// Whether you want CloudFront to automatically compress certain files for this cache behavior. 			If so, specify true; if not, specify false. For more information, see Serving 				Compressed Files in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,

    ///
    /// This field is deprecated. We recommend that you use the DefaultTTL field 			in a cache policy instead of this field. For more information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// The default amount of time that you want objects to stay in CloudFront caches before CloudFront 			forwards another request to your origin to determine whether the object has been 			updated. The value that you specify applies only when your origin does not add HTTP 			headers such as Cache-Control max-age, Cache-Control s-maxage, 			and Expires to objects. For more information, see Managing How Long Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<f64>,

    ///
    /// The value of ID for the field-level encryption configuration that you 			want CloudFront to use for encrypting specific fields of data for this cache behavior.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldLevelEncryptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_id: Option<cfn_resources::StrVal>,

    ///
    /// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field. For more information, see Working with policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// If you want to include values in the cache key, use a cache policy. For more 			information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// If you want to send values to the origin but not include them in the cache key, use an 			origin request policy. For more information, see Creating origin request policies or Using the managed origin request policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// A CacheBehavior must include either a CachePolicyId or 				ForwardedValues. We recommend that you use a 			CachePolicyId.
    ///
    /// A complex type that specifies how CloudFront handles query strings, cookies, and HTTP 			headers.
    ///
    /// Required: Conditional
    ///
    /// Type: ForwardedValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForwardedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarded_values: Option<ForwardedValues>,

    ///
    /// A list of CloudFront functions that are associated with this cache behavior. CloudFront functions 			must be published to the LIVE stage to associate them with a cache 			behavior.
    ///
    /// Required: No
    ///
    /// Type: List of FunctionAssociation
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_associations: Option<Vec<FunctionAssociation>>,

    ///
    /// A complex type that contains zero or more Lambda@Edge function associations for a 			cache behavior.
    ///
    /// Required: No
    ///
    /// Type: List of LambdaFunctionAssociation
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaFunctionAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_associations: Option<Vec<LambdaFunctionAssociation>>,

    ///
    /// This field is deprecated. We recommend that you use the MaxTTL field in a 			cache policy instead of this field. For more information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// The maximum amount of time that you want objects to stay in CloudFront caches before CloudFront 			forwards another request to your origin to determine whether the object has been 			updated. The value that you specify applies only when your origin adds HTTP headers such 			as Cache-Control max-age, Cache-Control s-maxage, and 				Expires to objects. For more information, see Managing How Long Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ttl: Option<f64>,

    ///
    /// This field is deprecated. We recommend that you use the MinTTL field in a 			cache policy instead of this field. For more information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// The minimum amount of time that you want objects to stay in CloudFront caches before CloudFront 			forwards another request to your origin to determine whether the object has been 			updated. For more information, see Managing How Long 				Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// You must specify 0 for MinTTL if you configure CloudFront to 			forward all headers to your origin (under Headers, if you specify 				1 for Quantity and * for 			Name).
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ttl: Option<f64>,

    ///
    /// The unique identifier of the origin request policy that is attached to this cache 			behavior. For more information, see Creating origin request policies or Using the managed origin request policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginRequestPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy_id: Option<cfn_resources::StrVal>,

    ///
    /// The pattern (for example, images/*.jpg) that specifies which requests to 			apply the behavior to. When CloudFront receives a viewer request, the requested path is 			compared with path patterns in the order in which cache behaviors are listed in the 			distribution.
    ///
    /// NoteYou can optionally include a slash (/) at the beginning of the path 				pattern. For example, /images/*.jpg. CloudFront behavior is the same with or 				without the leading /.
    ///
    /// The path pattern for the default cache behavior is * and cannot be 			changed. If the request for an object does not match the path pattern for any cache 			behaviors, CloudFront applies the behavior in the default cache behavior.
    ///
    /// For more information, see Path Pattern in the Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathPattern")]
    pub path_pattern: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the real-time log configuration that is attached to 			this cache behavior. For more information, see Real-time logs in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RealtimeLogConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config_arn: Option<cfn_resources::StrVal>,

    ///
    /// The identifier for a response headers policy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseHeadersPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy_id: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether you want to distribute media files in the Microsoft Smooth Streaming 			format using the origin that is associated with this cache behavior. If so, specify 				true; if not, specify false. If you specify 				true for SmoothStreaming, you can still distribute other 			content using this cache behavior if the content matches the value of 				PathPattern.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmoothStreaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smooth_streaming: Option<bool>,

    ///
    /// The value of ID for the origin that you want CloudFront to route requests to 			when they match this cache behavior.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetOriginId")]
    pub target_origin_id: cfn_resources::StrVal,

    ///
    /// A list of key groups that CloudFront can use to validate signed URLs or signed 			cookies.
    ///
    /// When a cache behavior contains trusted key groups, CloudFront requires signed URLs or signed 			cookies for all requests that match the cache behavior. The URLs or cookies must be 			signed with a private key whose corresponding public key is in the key group. The signed 			URL or cookie contains information about which public key CloudFront should use to verify the 			signature. For more information, see Serving private content in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrustedKeyGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_key_groups: Option<Vec<String>>,

    ///
    /// ImportantWe recommend using TrustedKeyGroups instead of 					TrustedSigners.
    ///
    /// A list of AWS account IDs whose public keys CloudFront can use to validate signed URLs or 			signed cookies.
    ///
    /// When a cache behavior contains trusted signers, CloudFront requires signed URLs or signed 			cookies for all requests that match the cache behavior. The URLs or cookies must be 			signed with the private key of a CloudFront key pair in the trusted signer's AWS account. 			The signed URL or cookie contains information about which public key CloudFront should use to 			verify the signature. For more information, see Serving private content in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrustedSigners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_signers: Option<Vec<String>>,

    ///
    /// The protocol that viewers can use to access the files in the origin specified by 				TargetOriginId when a request matches the path pattern in 				PathPattern. You can specify the following options:
    ///
    /// allow-all: Viewers can use HTTP or HTTPS.                        redirect-to-https: If a viewer submits an HTTP request, CloudFront 					returns an HTTP status code of 301 (Moved Permanently) to the viewer along with 					the HTTPS URL. The viewer then resubmits the request using the new URL.                        https-only: If a viewer sends an HTTP request, CloudFront returns an 					HTTP status code of 403 (Forbidden).
    ///
    /// For more information about requiring the HTTPS protocol, see Requiring HTTPS Between Viewers and CloudFront in the 				Amazon CloudFront Developer Guide.
    ///
    /// NoteThe only way to guarantee that viewers retrieve an object that was fetched from 				the origin using HTTPS is never to use any other protocol to fetch the object. If 				you have recently changed from HTTP to HTTPS, we recommend that you clear your 				objects' cache because cached objects are protocol agnostic. That means that an edge 				location will return an object from the cache regardless of whether the current 				request protocol matches the protocol used previously. For more information, see 					Managing Cache 					Expiration in the Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: allow-all | https-only | redirect-to-https
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViewerProtocolPolicy")]
    pub viewer_protocol_policy: CacheBehaviorViewerProtocolPolicyEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CacheBehaviorViewerProtocolPolicyEnum {
    /// allow-all
    #[serde(rename = "allow-all")]
    Allowall,

    /// https-only
    #[serde(rename = "https-only")]
    Httpsonly,

    /// redirect-to-https
    #[serde(rename = "redirect-to-https")]
    Redirecttohttps,
}

impl Default for CacheBehaviorViewerProtocolPolicyEnum {
    fn default() -> Self {
        CacheBehaviorViewerProtocolPolicyEnum::Allowall
    }
}

impl cfn_resources::CfnResource for CacheBehavior {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.forwarded_values
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field.
///
/// If you want to include cookies in the cache key, use a cache policy. For more 			information, see Creating cache policies in the Amazon CloudFront Developer Guide.
///
/// If you want to send cookies to the origin but not include them in the cache key, use 			an origin request policy. For more information, see Creating origin request policies in the 			Amazon CloudFront Developer Guide.
///
/// A complex type that specifies whether you want CloudFront to forward cookies to the origin 			and, if so, which ones. For more information about forwarding cookies to the origin, see 				How CloudFront Forwards, Caches, 				and Logs Cookies in the Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Cookies {
    ///
    /// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field.
    ///
    /// If you want to include cookies in the cache key, use a cache policy. For more 			information, see Creating cache policies in the Amazon CloudFront Developer Guide.
    ///
    /// If you want to send cookies to the origin but not include them in the cache key, use 			origin request policy. For more information, see Creating origin request policies in the 			Amazon CloudFront Developer Guide.
    ///
    /// Specifies which cookies to forward to the origin for this cache behavior: all, none, 			or the list of cookies specified in the WhitelistedNames complex 			type.
    ///
    /// Amazon S3 doesn't process cookies. When the cache behavior is forwarding requests to an 			Amazon S3 origin, specify none for the Forward element.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: all | none | whitelist
    ///
    /// Update requires: No interruption
    #[serde(rename = "Forward")]
    pub forward: CookiesForwardEnum,

    ///
    /// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field.
    ///
    /// If you want to include cookies in the cache key, use a cache policy. For more 			information, see Creating cache policies in the Amazon CloudFront Developer Guide.
    ///
    /// If you want to send cookies to the origin but not include them in the cache key, use 			an origin request policy. For more information, see Creating origin request policies in the 			Amazon CloudFront Developer Guide.
    ///
    /// Required if you specify whitelist for the value of Forward. 			A complex type that specifies how many different cookies you want CloudFront to forward to the 			origin for this cache behavior and, if you want to forward selected cookies, the names 			of those cookies.
    ///
    /// If you specify all or none for the value of 				Forward, omit WhitelistedNames. If you change the value of 				Forward from whitelist to all or 				none and you don't delete the WhitelistedNames element and 			its child elements, CloudFront deletes them automatically.
    ///
    /// For the current limit on the number of cookie names that you can whitelist for each 			cache behavior, see CloudFront 				Limits in the         AWS General Reference.
    ///
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WhitelistedNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelisted_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CookiesForwardEnum {
    /// all
    #[serde(rename = "all")]
    All,

    /// none
    #[serde(rename = "none")]
    None,

    /// whitelist
    #[serde(rename = "whitelist")]
    Whitelist,
}

impl Default for CookiesForwardEnum {
    fn default() -> Self {
        CookiesForwardEnum::All
    }
}

impl cfn_resources::CfnResource for Cookies {
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

/// A complex type that controls:
///
/// For more information about custom error pages, see Customizing 				Error Responses in the Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomErrorResponse {
    ///
    /// The minimum amount of time, in seconds, that you want CloudFront to cache the HTTP status 			code specified in ErrorCode. When this time period has elapsed, CloudFront 			queries your origin to see whether the problem that caused the error has been resolved 			and the requested object is now available.
    ///
    /// For more information, see Customizing 				Error Responses in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorCachingMinTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_caching_min_ttl: Option<f64>,

    ///
    /// The HTTP status code for which you want to specify a custom error page and/or a 			caching duration.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorCode")]
    pub error_code: i64,

    ///
    /// The HTTP status code that you want CloudFront to return to the viewer along with the custom 			error page. There are a variety of reasons that you might want CloudFront to return a status 			code different from the status code that your origin returned to CloudFront, for 			example:
    ///
    /// Some Internet devices (some firewalls and corporate proxies, for example) 					intercept HTTP 4xx and 5xx and prevent the response from being returned to the 					viewer. If you substitute 200, the response typically won't be 					intercepted.               If you don't care about distinguishing among different client errors or server 					errors, you can specify 400 or 500 as the 						ResponseCode for all 4xx or 5xx errors.               You might want to return a 200 status code (OK) and static 					website so your customers don't know that your website is down.
    ///
    /// If you specify a value for ResponseCode, you must also specify a value 			for ResponsePagePath.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,

    ///
    /// The path to the custom error page that you want CloudFront to return to a viewer when your 			origin returns the HTTP status code specified by ErrorCode, for example, 				/4xx-errors/403-forbidden.html. If you want to store your objects and 			your custom error pages in different locations, your distribution must include a cache 			behavior for which the following is true:
    ///
    /// The value of PathPattern matches the path to your custom error 					messages. For example, suppose you saved custom error pages for 4xx errors in an 					Amazon S3 bucket in a directory named /4xx-errors. Your distribution 					must include a cache behavior for which the path pattern routes requests for 					your custom error pages to that location, for example, 						/4xx-errors/*.               The value of TargetOriginId specifies the value of the 						ID element for the origin that contains your custom error 					pages.
    ///
    /// If you specify a value for ResponsePagePath, you must also specify a 			value for ResponseCode.
    ///
    /// We recommend that you store custom error pages in an Amazon S3 bucket. If you store custom 			error pages on an HTTP server and the server starts to return 5xx errors, CloudFront can't get 			the files that you want to return to viewers because the origin server is 			unavailable.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponsePagePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_page_path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CustomErrorResponse {
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

/// A custom origin. A custom origin is any origin that is not an 			Amazon S3 bucket, with one exception. An Amazon S3 bucket that is configured with 				static website hosting       is a custom origin.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomOriginConfig {
    ///
    /// The HTTP port that CloudFront uses to connect to the origin. Specify the HTTP port that the 			origin listens on.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTPPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpport: Option<i64>,

    ///
    /// The HTTPS port that CloudFront uses to connect to the origin. Specify the HTTPS port that 			the origin listens on.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTPSPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpsport: Option<i64>,

    ///
    /// Specifies how long, in seconds, CloudFront persists its connection to the origin. The 			minimum timeout is 1 second, the maximum is 60 seconds, and the default (if you don't 			specify otherwise) is 5 seconds.
    ///
    /// For more information, see Origin Keep-alive Timeout in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginKeepaliveTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_keepalive_timeout: Option<i64>,

    ///
    /// Specifies the protocol (HTTP or HTTPS) that CloudFront uses to connect to the origin. Valid 			values are:
    ///
    /// http-only – CloudFront always uses HTTP to connect to the 					origin.                        match-viewer – CloudFront connects to the origin using the same 					protocol that the viewer used to connect to CloudFront.                        https-only – CloudFront always uses HTTPS to connect to the 					origin.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: http-only | https-only | match-viewer
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginProtocolPolicy")]
    pub origin_protocol_policy: CustomOriginConfigOriginProtocolPolicyEnum,

    ///
    /// Specifies how long, in seconds, CloudFront waits for a response from the origin. This is 			also known as the origin response timeout. The minimum timeout is 1 			second, the maximum is 60 seconds, and the default (if you don't specify otherwise) is 			30 seconds.
    ///
    /// For more information, see Origin Response Timeout in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginReadTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_read_timeout: Option<i64>,

    ///
    /// Specifies the minimum SSL/TLS protocol that CloudFront uses when connecting to your origin 			over HTTPS. Valid values include SSLv3, TLSv1, 				TLSv1.1, and TLSv1.2.
    ///
    /// For more information, see Minimum Origin SSL Protocol in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginSSLProtocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_sslprotocols: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CustomOriginConfigOriginProtocolPolicyEnum {
    /// http-only
    #[serde(rename = "http-only")]
    Httponly,

    /// https-only
    #[serde(rename = "https-only")]
    Httpsonly,

    /// match-viewer
    #[serde(rename = "match-viewer")]
    Matchviewer,
}

impl Default for CustomOriginConfigOriginProtocolPolicyEnum {
    fn default() -> Self {
        CustomOriginConfigOriginProtocolPolicyEnum::Httponly
    }
}

impl cfn_resources::CfnResource for CustomOriginConfig {
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

/// A complex type that describes the default cache behavior if you don't specify a 				CacheBehavior element or if request URLs don't match any of the values 			of PathPattern in CacheBehavior elements. You must create 			exactly one default cache behavior.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DefaultCacheBehavior {
    ///
    /// A complex type that controls which HTTP methods CloudFront processes and forwards to your 			Amazon S3 bucket or your custom origin. There are three choices:
    ///
    /// CloudFront forwards only GET and HEAD requests.               CloudFront forwards only GET, HEAD, and 						OPTIONS requests.               CloudFront forwards GET, HEAD, OPTIONS, PUT, PATCH, POST, and 						DELETE requests.
    ///
    /// If you pick the third choice, you may need to restrict access to your Amazon S3 bucket or 			to your custom origin so users can't perform operations that you don't want them to. For 			example, you might not want users to have permissions to delete objects from your 			origin.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<Vec<String>>,

    ///
    /// The unique identifier of the cache policy that is attached to the default cache 			behavior. For more information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// A DefaultCacheBehavior must include either a CachePolicyId 			or ForwardedValues. We recommend that you use a 			CachePolicyId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachePolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy_id: Option<cfn_resources::StrVal>,

    ///
    /// A complex type that controls whether CloudFront caches the response to requests using the 			specified HTTP methods. There are two choices:
    ///
    /// CloudFront caches responses to GET and HEAD 					requests.               CloudFront caches responses to GET, HEAD, and 						OPTIONS requests.
    ///
    /// If you pick the second choice for your Amazon S3 Origin, you may need to forward 			Access-Control-Request-Method, Access-Control-Request-Headers, and Origin headers for 			the responses to be cached correctly.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_methods: Option<Vec<String>>,

    ///
    /// Whether you want CloudFront to automatically compress certain files for this cache behavior. 			If so, specify true; if not, specify false. For more 			information, see Serving 				Compressed Files in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,

    ///
    /// This field is deprecated. We recommend that you use the DefaultTTL field 			in a cache policy instead of this field. For more information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// The default amount of time that you want objects to stay in CloudFront caches before CloudFront 			forwards another request to your origin to determine whether the object has been 			updated. The value that you specify applies only when your origin does not add HTTP 			headers such as Cache-Control max-age, Cache-Control s-maxage, 			and Expires to objects. For more information, see Managing How Long Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<f64>,

    ///
    /// The value of ID for the field-level encryption configuration that you 			want CloudFront to use for encrypting specific fields of data for the default cache 			behavior.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldLevelEncryptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_id: Option<cfn_resources::StrVal>,

    ///
    /// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field. For more information, see Working with policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// If you want to include values in the cache key, use a cache policy. For more 			information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// If you want to send values to the origin but not include them in the cache key, use an 			origin request policy. For more information, see Creating origin request policies or Using the managed origin request policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// A DefaultCacheBehavior must include either a CachePolicyId 			or ForwardedValues. We recommend that you use a 			CachePolicyId.
    ///
    /// A complex type that specifies how CloudFront handles query strings, cookies, and HTTP 			headers.
    ///
    /// Required: Conditional
    ///
    /// Type: ForwardedValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForwardedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarded_values: Option<ForwardedValues>,

    ///
    /// A list of CloudFront functions that are associated with this cache behavior. CloudFront functions 			must be published to the LIVE stage to associate them with a cache 			behavior.
    ///
    /// Required: No
    ///
    /// Type: List of FunctionAssociation
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_associations: Option<Vec<FunctionAssociation>>,

    ///
    /// A complex type that contains zero or more Lambda@Edge function associations for a 			cache behavior.
    ///
    /// Required: No
    ///
    /// Type: List of LambdaFunctionAssociation
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaFunctionAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_associations: Option<Vec<LambdaFunctionAssociation>>,

    ///
    /// This field is deprecated. We recommend that you use the MaxTTL field in a 			cache policy instead of this field. For more information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// The maximum amount of time that you want objects to stay in CloudFront caches before CloudFront 			forwards another request to your origin to determine whether the object has been 			updated. The value that you specify applies only when your origin adds HTTP headers such 			as Cache-Control max-age, Cache-Control s-maxage, and 				Expires to objects. For more information, see Managing How Long Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ttl: Option<f64>,

    ///
    /// This field is deprecated. We recommend that you use the MinTTL field in a 			cache policy instead of this field. For more information, see Creating cache policies or Using the managed cache policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// The minimum amount of time that you want objects to stay in CloudFront caches before CloudFront 			forwards another request to your origin to determine whether the object has been 			updated. For more information, see Managing How Long 				Content Stays in an Edge Cache (Expiration) in the 				Amazon CloudFront Developer Guide.
    ///
    /// You must specify 0 for MinTTL if you configure CloudFront to 			forward all headers to your origin (under Headers, if you specify 				1 for Quantity and * for 			Name).
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ttl: Option<f64>,

    ///
    /// The unique identifier of the origin request policy that is attached to the default 			cache behavior. For more information, see Creating origin request policies or Using the managed origin request policies in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginRequestPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy_id: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the real-time log configuration that is attached to 			this cache behavior. For more information, see Real-time logs in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RealtimeLogConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config_arn: Option<cfn_resources::StrVal>,

    ///
    /// The identifier for a response headers policy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseHeadersPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy_id: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether you want to distribute media files in the Microsoft Smooth Streaming 			format using the origin that is associated with this cache behavior. If so, specify 				true; if not, specify false. If you specify 				true for SmoothStreaming, you can still distribute other 			content using this cache behavior if the content matches the value of 				PathPattern.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmoothStreaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smooth_streaming: Option<bool>,

    ///
    /// The value of ID for the origin that you want CloudFront to route requests to 			when they use the default cache behavior.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetOriginId")]
    pub target_origin_id: cfn_resources::StrVal,

    ///
    /// A list of key groups that CloudFront can use to validate signed URLs or signed 			cookies.
    ///
    /// When a cache behavior contains trusted key groups, CloudFront requires signed URLs or signed 			cookies for all requests that match the cache behavior. The URLs or cookies must be 			signed with a private key whose corresponding public key is in the key group. The signed 			URL or cookie contains information about which public key CloudFront should use to verify the 			signature. For more information, see Serving private content in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrustedKeyGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_key_groups: Option<Vec<String>>,

    ///
    /// ImportantWe recommend using TrustedKeyGroups instead of 					TrustedSigners.
    ///
    /// A list of AWS account IDs whose public keys CloudFront can use to validate signed URLs or 			signed cookies.
    ///
    /// When a cache behavior contains trusted signers, CloudFront requires signed URLs or signed 			cookies for all requests that match the cache behavior. The URLs or cookies must be 			signed with the private key of a CloudFront key pair in a trusted signer's AWS account. The 			signed URL or cookie contains information about which public key CloudFront should use to 			verify the signature. For more information, see Serving private content in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrustedSigners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_signers: Option<Vec<String>>,

    ///
    /// The protocol that viewers can use to access the files in the origin specified by 				TargetOriginId when a request matches the path pattern in 				PathPattern. You can specify the following options:
    ///
    /// allow-all: Viewers can use HTTP or HTTPS.                        redirect-to-https: If a viewer submits an HTTP request, CloudFront 					returns an HTTP status code of 301 (Moved Permanently) to the viewer along with 					the HTTPS URL. The viewer then resubmits the request using the new URL.                        https-only: If a viewer sends an HTTP request, CloudFront returns an 					HTTP status code of 403 (Forbidden).
    ///
    /// For more information about requiring the HTTPS protocol, see Requiring HTTPS Between Viewers and CloudFront in the 				Amazon CloudFront Developer Guide.
    ///
    /// NoteThe only way to guarantee that viewers retrieve an object that was fetched from 				the origin using HTTPS is never to use any other protocol to fetch the object. If 				you have recently changed from HTTP to HTTPS, we recommend that you clear your 				objects' cache because cached objects are protocol agnostic. That means that an edge 				location will return an object from the cache regardless of whether the current 				request protocol matches the protocol used previously. For more information, see 					Managing Cache 					Expiration in the Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: allow-all | https-only | redirect-to-https
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViewerProtocolPolicy")]
    pub viewer_protocol_policy: DefaultCacheBehaviorViewerProtocolPolicyEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DefaultCacheBehaviorViewerProtocolPolicyEnum {
    /// allow-all
    #[serde(rename = "allow-all")]
    Allowall,

    /// https-only
    #[serde(rename = "https-only")]
    Httpsonly,

    /// redirect-to-https
    #[serde(rename = "redirect-to-https")]
    Redirecttohttps,
}

impl Default for DefaultCacheBehaviorViewerProtocolPolicyEnum {
    fn default() -> Self {
        DefaultCacheBehaviorViewerProtocolPolicyEnum::Allowall
    }
}

impl cfn_resources::CfnResource for DefaultCacheBehavior {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.forwarded_values
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A distribution configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DistributionConfig {
    ///
    /// A complex type that contains information about CNAMEs (alternate domain names), if 			any, for this distribution.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CNAMEs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnames: Option<Vec<String>>,

    ///
    /// A complex type that contains zero or more CacheBehavior elements.
    ///
    /// Required: No
    ///
    /// Type: List of CacheBehavior
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheBehaviors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_behaviors: Option<Vec<CacheBehavior>>,

    ///
    /// A comment to describe the distribution. The comment cannot be longer than 			128 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<cfn_resources::StrVal>,

    ///
    /// The identifier of a continuous deployment policy. For more information, see 				CreateContinuousDeploymentPolicy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContinuousDeploymentPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy_id: Option<cfn_resources::StrVal>,

    ///
    /// A complex type that controls the following:
    ///
    /// Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom 					error messages before returning the response to the viewer.               How long CloudFront caches HTTP status codes in the 4xx and 5xx range.
    ///
    /// For more information about custom error pages, see Customizing 				Error Responses in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of CustomErrorResponse
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomErrorResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_error_responses: Option<Vec<CustomErrorResponse>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: LegacyCustomOrigin
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_origin: Option<LegacyCustomOrigin>,

    ///
    /// A complex type that describes the default cache behavior if you don't specify a 				CacheBehavior element or if files don't match any of the values of 				PathPattern in CacheBehavior elements. You must create 			exactly one default cache behavior.
    ///
    /// Required: Yes
    ///
    /// Type: DefaultCacheBehavior
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultCacheBehavior")]
    pub default_cache_behavior: DefaultCacheBehavior,

    ///
    /// The object that you want CloudFront to request from your origin (for example, 				index.html) when a viewer requests the root URL for your distribution 				(https://www.example.com) instead of an object in your distribution 				(https://www.example.com/product-description.html). Specifying a 			default root object avoids exposing the contents of your distribution.
    ///
    /// Specify only the object name, for example, index.html. Don't add a 				/ before the object name.
    ///
    /// If you don't want to specify a default root object when you create a distribution, 			include an empty DefaultRootObject element.
    ///
    /// To delete the default root object from an existing distribution, update the 			distribution configuration and include an empty DefaultRootObject 			element.
    ///
    /// To replace the default root object, update the distribution configuration and specify 			the new object.
    ///
    /// For more information about the default root object, see Creating a 				Default Root Object in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRootObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_object: Option<cfn_resources::StrVal>,

    ///
    /// From this field, you can enable or disable the selected distribution.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// (Optional) Specify the maximum HTTP version(s) that you want viewers to use to 			communicate with CloudFront. The default value for new distributions is 			http1.1.
    ///
    /// For viewers and CloudFront to use HTTP/2, viewers must support TLSv1.2 or later, and  		must support Server Name Indication (SNI).
    ///
    /// For viewers and CloudFront to use HTTP/3, viewers must support TLSv1.3 and Server  		Name Indication (SNI). CloudFront supports HTTP/3 connection migration to allow the  		viewer to switch networks without losing connection. For more information  		about connection migration, see Connection Migration at RFC 9000. For more  		information about supported TLSv1.3 ciphers, see Supported protocols and ciphers between viewers and 				CloudFront.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: http1.1 | http2 | http2and3 | http3
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_version: Option<DistributionConfigHttpVersionEnum>,

    ///
    /// If you want CloudFront to respond to IPv6 DNS requests with an IPv6 address for your 			distribution, specify true. If you specify false, CloudFront 			responds to IPv6 DNS requests with the DNS response code NOERROR and with 			no IP addresses. This allows viewers to submit a second request, for an IPv4 address for 			your distribution.
    ///
    /// In general, you should enable IPv6 if you have users on IPv6 networks who want to 			access your content. However, if you're using signed URLs or signed cookies to restrict 			access to your content, and if you're using a custom policy that includes the 				IpAddress parameter to restrict the IP addresses that can access your 			content, don't enable IPv6. If you want to restrict access to some content by IP address 			and not restrict access to other content (or restrict access but not by IP address), you 			can create two distributions. For more information, see Creating a Signed URL Using a Custom Policy in the 				Amazon CloudFront Developer Guide.
    ///
    /// If you're using an Amazon Route 53 AWS Integration alias resource record set to route traffic to your CloudFront 			distribution, you need to create a second alias resource record set when both of the 			following are true:
    ///
    /// You enable IPv6 for the distribution               You're using alternate domain names in the URLs for your objects
    ///
    /// For more information, see Routing 				Traffic to an Amazon CloudFront Web Distribution by Using Your Domain Name in the 				        Amazon Route 53 AWS Integration Developer Guide.
    ///
    /// If you created a CNAME resource record set, either with Amazon Route 53 AWS Integration or with another DNS 			service, you don't need to make any changes. A CNAME record will route traffic to your 			distribution regardless of the IP address format of the viewer request.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPV6Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_enabled: Option<bool>,

    ///
    /// A complex type that controls whether access logs are written for the 			distribution.
    ///
    /// For more information about logging, see Access Logs in 			the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Logging
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    ///
    /// A complex type that contains information about origin groups for this 			distribution.
    ///
    /// Required: No
    ///
    /// Type: OriginGroups
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_groups: Option<OriginGroups>,

    ///
    /// A complex type that contains information about origins for this distribution.
    ///
    /// Required: No
    ///
    /// Type: List of Origin
    ///
    /// Update requires: No interruption
    #[serde(rename = "Origins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<Origin>>,

    ///
    /// The price class that corresponds with the maximum price that you want to pay for CloudFront 			service. If you specify PriceClass_All, CloudFront responds to requests for your 			objects from all CloudFront edge locations.
    ///
    /// If you specify a price class other than PriceClass_All, CloudFront serves your 			objects from the CloudFront edge location that has the lowest latency among the edge locations 			in your price class. Viewers who are in or near regions that are excluded from your 			specified price class may encounter slower performance.
    ///
    /// For more information about price classes, see Choosing the Price 				Class for a CloudFront Distribution in the Amazon CloudFront Developer Guide. 			For information about CloudFront pricing, including how price classes (such as Price Class 			100) map to CloudFront regions, see Amazon CloudFront 				Pricing.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PriceClass_100 | PriceClass_200 | PriceClass_All
    ///
    /// Update requires: No interruption
    #[serde(rename = "PriceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_class: Option<DistributionConfigPriceClassEnum>,

    ///
    /// A complex type that identifies ways in which you want to restrict distribution of your 			content.
    ///
    /// Required: No
    ///
    /// Type: Restrictions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Restrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: LegacyS3Origin
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_origin: Option<LegacyS3Origin>,

    ///
    /// A Boolean that indicates whether this is a staging distribution. When this value is 				true, this is a staging distribution. When this value is 				false, this is not a staging distribution.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Staging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging: Option<bool>,

    ///
    /// A complex type that determines the distribution's SSL/TLS configuration for 			communicating with viewers.
    ///
    /// Required: No
    ///
    /// Type: ViewerCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViewerCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_certificate: Option<ViewerCertificate>,

    ///
    /// A unique identifier that specifies the AWS WAF web ACL, if any, to associate with this 			distribution. To specify a web ACL created using the latest version of AWS WAF, use the 			ACL ARN, for example 				arn:aws:wafv2:us-east-1:123456789012:global/webacl/ExampleWebACL/473e64fd-f30b-4765-81a0-62ad96dd167a. 			To specify a web ACL created using AWS WAF Classic, use the ACL ID, for example 				473e64fd-f30b-4765-81a0-62ad96dd167a.
    ///
    /// AWS WAF is a web application firewall that lets you monitor the HTTP and HTTPS requests 			that are forwarded to CloudFront, and lets you control access to your content. Based on 			conditions that you specify, such as the IP addresses that requests originate from or 			the values of query strings, CloudFront responds to requests either with the requested content 			or with an HTTP 403 status code (Forbidden). You can also configure CloudFront to return a 			custom error page when a request is blocked. For more information about AWS WAF, see the 				AWS WAF Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebACLId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_aclid: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DistributionConfigHttpVersionEnum {
    /// http1.1
    #[serde(rename = "http1.1")]
    Http11,

    /// http2
    #[serde(rename = "http2")]
    Http2,

    /// http2and3
    #[serde(rename = "http2and3")]
    Http2and3,

    /// http3
    #[serde(rename = "http3")]
    Http3,
}

impl Default for DistributionConfigHttpVersionEnum {
    fn default() -> Self {
        DistributionConfigHttpVersionEnum::Http11
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DistributionConfigPriceClassEnum {
    /// PriceClass_100
    #[serde(rename = "PriceClass_100")]
    Priceclass100,

    /// PriceClass_200
    #[serde(rename = "PriceClass_200")]
    Priceclass200,

    /// PriceClass_All
    #[serde(rename = "PriceClass_All")]
    Priceclassall,
}

impl Default for DistributionConfigPriceClassEnum {
    fn default() -> Self {
        DistributionConfigPriceClassEnum::Priceclass100
    }
}

impl cfn_resources::CfnResource for DistributionConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.custom_origin
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.default_cache_behavior.validate()?;

        self.logging.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.origin_groups
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.restrictions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_origin
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.viewer_certificate
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field.
///
/// If you want to include values in the cache key, use a cache policy. For more 			information, see Creating cache policies in the Amazon CloudFront Developer Guide.
///
/// If you want to send values to the origin but not include them in the cache key, use an 			origin request policy. For more information, see Creating origin request policies in the 			Amazon CloudFront Developer Guide.
///
/// A complex type that specifies how CloudFront handles query strings, cookies, and HTTP 			headers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ForwardedValues {
    ///
    /// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field.
    ///
    /// If you want to include cookies in the cache key, use a cache policy. For more 			information, see Creating cache policies in the Amazon CloudFront Developer Guide.
    ///
    /// If you want to send cookies to the origin but not include them in the cache key, use 			an origin request policy. For more information, see Creating origin request policies in the 			Amazon CloudFront Developer Guide.
    ///
    /// A complex type that specifies whether you want CloudFront to forward cookies to the origin 			and, if so, which ones. For more information about forwarding cookies to the origin, see 				How CloudFront Forwards, Caches, 				and Logs Cookies in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Cookies
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Cookies>,

    ///
    /// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field.
    ///
    /// If you want to include headers in the cache key, use a cache policy. For more 			information, see Creating cache policies in the Amazon CloudFront Developer Guide.
    ///
    /// If you want to send headers to the origin but not include them in the cache key, use 			an origin request policy. For more information, see Creating origin request policies in the 			Amazon CloudFront Developer Guide.
    ///
    /// A complex type that specifies the Headers, if any, that you want CloudFront to 			forward to the origin for this cache behavior (whitelisted headers). For the headers 			that you specify, CloudFront also caches separate versions of a specified object that is based 			on the header values in viewer requests.
    ///
    /// For more information, see Caching Content 				Based on Request Headers in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,

    ///
    /// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field.
    ///
    /// If you want to include query strings in the cache key, use a cache policy. For more 			information, see Creating cache policies in the Amazon CloudFront Developer Guide.
    ///
    /// If you want to send query strings to the origin but not include them in the cache key, 			use an origin request policy. For more information, see Creating origin request policies in the 			Amazon CloudFront Developer Guide.
    ///
    /// Indicates whether you want CloudFront to forward query strings to the origin that is 			associated with this cache behavior and cache based on the query string parameters. CloudFront 			behavior depends on the value of QueryString and on the values that you 			specify for QueryStringCacheKeys, if any:
    ///
    /// If you specify true for QueryString and you don't specify any values for 				QueryStringCacheKeys, CloudFront forwards all query string parameters to the 			origin and caches based on all query string parameters. Depending on how many query 			string parameters and values you have, this can adversely affect performance because 			CloudFront must forward more requests to the origin.
    ///
    /// If you specify true for QueryString and you specify one or more values 			for QueryStringCacheKeys, CloudFront forwards all query string parameters to the 			origin, but it only caches based on the query string parameters that you specify.
    ///
    /// If you specify false for QueryString, CloudFront doesn't forward any query 			string parameters to the origin, and doesn't cache based on query string 			parameters.
    ///
    /// For more information, see Configuring 				CloudFront to Cache Based on Query String Parameters in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    pub query_string: bool,

    ///
    /// This field is deprecated. We recommend that you use a cache policy or an origin 			request policy instead of this field.
    ///
    /// If you want to include query strings in the cache key, use a cache policy. For more 			information, see Creating cache policies in the Amazon CloudFront Developer Guide.
    ///
    /// If you want to send query strings to the origin but not include them in the cache key, 			use an origin request policy. For more information, see Creating origin request policies in the 			Amazon CloudFront Developer Guide.
    ///
    /// A complex type that contains information about the query string parameters that you 			want CloudFront to use for caching for this cache behavior.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStringCacheKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_cache_keys: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for ForwardedValues {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cookies.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A CloudFront function that is associated with a cache behavior in a CloudFront 			distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FunctionAssociation {
    ///
    /// The event type of the function, either viewer-request or 				viewer-response. You cannot use origin-facing event types 				(origin-request and origin-response) with a CloudFront 			function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: origin-request | origin-response | viewer-request | viewer-response
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<FunctionAssociationEventTypeEnum>,

    ///
    /// The Amazon Resource Name (ARN) of the function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 108
    ///
    /// Pattern: arn:aws:cloudfront::[0-9]{12}:function\/[a-zA-Z0-9-_]{1,64}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FunctionAssociationEventTypeEnum {
    /// origin-request
    #[serde(rename = "origin-request")]
    Originrequest,

    /// origin-response
    #[serde(rename = "origin-response")]
    Originresponse,

    /// viewer-request
    #[serde(rename = "viewer-request")]
    Viewerrequest,

    /// viewer-response
    #[serde(rename = "viewer-response")]
    Viewerresponse,
}

impl Default for FunctionAssociationEventTypeEnum {
    fn default() -> Self {
        FunctionAssociationEventTypeEnum::Originrequest
    }
}

impl cfn_resources::CfnResource for FunctionAssociation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.function_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 108 as _ {
                    return Err(format!(
                        "Max validation failed on field 'function_arn'. {} is greater than 108",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A complex type that controls the countries in which your content is distributed. CloudFront 			determines the location of your users using MaxMind GeoIP databases. To disable geo restriction, remove the Restrictions property from your stack template.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GeoRestriction {
    ///
    /// A complex type that contains a Location element for each country in 			which you want CloudFront either to distribute your content (whitelist) or not 			distribute your content (blacklist).
    ///
    /// The Location element is a two-letter, uppercase country code for a 			country that you want to include in your blacklist or 				whitelist. Include one Location element for each 			country.
    ///
    /// CloudFront and MaxMind both use ISO 3166 country codes. For the 			current list of countries and the corresponding codes, see ISO 				3166-1-alpha-2 code on the International Organization for 				Standardization website. You can also refer to the country list on the 			CloudFront console, which includes both country names and codes.
    ///
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,

    ///
    /// The method that you want to use to restrict distribution of your content by 			country:
    ///
    /// none: No geo restriction is enabled, meaning access to content is 					not restricted by client geo location.                        blacklist: The Location elements specify the 					countries in which you don't want CloudFront to distribute your content.                        whitelist: The Location elements specify the 					countries in which you want CloudFront to distribute your content.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: blacklist | none | whitelist
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestrictionType")]
    pub restriction_type: GeoRestrictionRestrictionTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum GeoRestrictionRestrictionTypeEnum {
    /// blacklist
    #[serde(rename = "blacklist")]
    Blacklist,

    /// none
    #[serde(rename = "none")]
    None,

    /// whitelist
    #[serde(rename = "whitelist")]
    Whitelist,
}

impl Default for GeoRestrictionRestrictionTypeEnum {
    fn default() -> Self {
        GeoRestrictionRestrictionTypeEnum::Blacklist
    }
}

impl cfn_resources::CfnResource for GeoRestriction {
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

/// A complex type that contains a Lambda@Edge function association.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LambdaFunctionAssociation {
    ///
    /// Specifies the event type that triggers a Lambda@Edge function invocation. You can 			specify the following values:
    ///
    /// viewer-request: The function executes when CloudFront receives a 					request from a viewer and before it checks to see whether the requested object 					is in the edge cache.                        origin-request: The function executes only when CloudFront sends a 					request to your origin. When the requested object is in the edge cache, the 					function doesn't execute.                        origin-response: The function executes after CloudFront receives a 					response from the origin and before it caches the object in the response. When 					the requested object is in the edge cache, the function doesn't execute.                        viewer-response: The function executes before CloudFront returns the 					requested object to the viewer. The function executes regardless of whether the 					object was already in the edge cache.        If the origin returns an HTTP status code other than HTTP 200 (OK), the 					function doesn't execute.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: origin-request | origin-response | viewer-request | viewer-response
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<LambdaFunctionAssociationEventTypeEnum>,

    ///
    /// A flag that allows a Lambda@Edge function to have read access to the body content. For 			more information, see Accessing the Request Body by Choosing the Include Body Option in the 			Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_body: Option<bool>,

    ///
    /// The ARN of the Lambda@Edge function. You must specify the ARN of a function version; 			you can't specify an alias or $LATEST.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaFunctionARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_arn: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LambdaFunctionAssociationEventTypeEnum {
    /// origin-request
    #[serde(rename = "origin-request")]
    Originrequest,

    /// origin-response
    #[serde(rename = "origin-response")]
    Originresponse,

    /// viewer-request
    #[serde(rename = "viewer-request")]
    Viewerrequest,

    /// viewer-response
    #[serde(rename = "viewer-response")]
    Viewerresponse,
}

impl Default for LambdaFunctionAssociationEventTypeEnum {
    fn default() -> Self {
        LambdaFunctionAssociationEventTypeEnum::Originrequest
    }
}

impl cfn_resources::CfnResource for LambdaFunctionAssociation {
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

/// The LegacyCustomOrigin property type specifies Property description not available. for an AWS::CloudFront::Distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LegacyCustomOrigin {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DNSName")]
    pub dnsname: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTPPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpport: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTPSPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub httpsport: Option<i64>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginProtocolPolicy")]
    pub origin_protocol_policy: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginSSLProtocols")]
    pub origin_sslprotocols: Vec<String>,
}

impl cfn_resources::CfnResource for LegacyCustomOrigin {
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

/// The LegacyS3Origin property type specifies Property description not available. for an AWS::CloudFront::Distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LegacyS3Origin {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DNSName")]
    pub dnsname: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginAccessIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_identity: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LegacyS3Origin {
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

/// A complex type that controls whether access logs are written for the 			distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Logging {
    ///
    /// The Amazon S3 bucket to store the access logs in, for example, 				myawslogbucket.s3.amazonaws.com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: cfn_resources::StrVal,

    ///
    /// Specifies whether you want CloudFront to include cookies in access logs, specify 				true for IncludeCookies. If you choose to include cookies 			in logs, CloudFront logs all cookies regardless of how you configure the cache behaviors for 			this distribution. If you don't want to include cookies when you create a distribution 			or if you want to disable include cookies for an existing distribution, specify 				false for IncludeCookies.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeCookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cookies: Option<bool>,

    ///
    /// An optional string that you want CloudFront to prefix to the access log 				filenames for this distribution, for example, myprefix/. 			If you want to enable logging, but you don't want to specify a prefix, you still must 			include an empty Prefix element in the Logging element.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Logging {
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

/// An origin.
///
/// An origin is the location where content is stored, and from which CloudFront gets content to 			serve to viewers. To specify an origin:
///
/// For the current maximum number of origins that you can specify per distribution, see 				General Quotas on Web Distributions in the 				Amazon CloudFront Developer Guide (quotas were formerly referred to as 			limits).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Origin {
    ///
    /// The number of times that CloudFront attempts to connect to the origin. The minimum number is 			1, the maximum is 3, and the default (if you don't specify otherwise) is 3.
    ///
    /// For a custom origin (including an Amazon S3 bucket that's configured with static website 			hosting), this value also specifies the number of times that CloudFront attempts to get a 			response from the origin, in the case of an Origin Response Timeout.
    ///
    /// For more information, see Origin Connection Attempts in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_attempts: Option<i64>,

    ///
    /// The number of seconds that CloudFront waits when trying to establish a connection to the 			origin. The minimum timeout is 1 second, the maximum is 10 seconds, and the default (if 			you don't specify otherwise) is 10 seconds.
    ///
    /// For more information, see Origin Connection Timeout in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i64>,

    ///
    /// Use this type to specify an origin that is not an Amazon S3 bucket, with one exception. If 			the Amazon S3 bucket is configured with static website hosting, use this type. If the Amazon S3 			bucket is not configured with static website hosting, use the 				S3OriginConfig type instead.
    ///
    /// Required: Conditional
    ///
    /// Type: CustomOriginConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomOriginConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_origin_config: Option<CustomOriginConfig>,

    ///
    /// The domain name for the origin.
    ///
    /// For more information, see Origin Domain Name in the Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    ///
    /// A unique identifier for the origin. This value must be unique within the 			distribution.
    ///
    /// Use this value to specify the TargetOriginId in a 				CacheBehavior or DefaultCacheBehavior.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The unique identifier of an origin access control for this origin.
    ///
    /// For more information, see Restricting access to an Amazon S3 origin in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginAccessControlId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control_id: Option<cfn_resources::StrVal>,

    ///
    /// A list of HTTP header names and values that CloudFront adds to the requests that it sends to 			the origin.
    ///
    /// For more information, see Adding Custom Headers to Origin Requests in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of OriginCustomHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginCustomHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_custom_headers: Option<Vec<OriginCustomHeader>>,

    ///
    /// An optional path that CloudFront appends to the origin domain name when CloudFront requests 			content from the origin.
    ///
    /// For more information, see Origin Path in the 			Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<cfn_resources::StrVal>,

    ///
    /// CloudFront Origin Shield. Using Origin Shield can help reduce the load on your 			origin.
    ///
    /// For more information, see Using Origin Shield in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: OriginShield
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginShield")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_shield: Option<OriginShield>,

    ///
    /// Use this type to specify an origin that is an Amazon S3 bucket that is not configured with 			static website hosting. To specify any other type of origin, including an Amazon S3 bucket 			that is configured with static website hosting, use the CustomOriginConfig 			type instead.
    ///
    /// Required: Conditional
    ///
    /// Type: S3OriginConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3OriginConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_origin_config: Option<S3OriginConfig>,
}

impl cfn_resources::CfnResource for Origin {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.custom_origin_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.origin_shield
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_origin_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A complex type that contains HeaderName and HeaderValue 			elements, if any, for this distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginCustomHeader {
    ///
    /// The name of a header that you want CloudFront to send to your origin. For more information, 			see Adding 				Custom Headers to Origin Requests in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderName")]
    pub header_name: cfn_resources::StrVal,

    ///
    /// The value for the header that you specified in the HeaderName 			field.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderValue")]
    pub header_value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for OriginCustomHeader {
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

/// An origin group includes two origins (a primary origin and a second origin to failover 			to) and a failover criteria that you specify. You create an origin group to support 			origin failover in CloudFront. When you create or update a distribution, you can 			specifiy the origin group instead of a single origin, and CloudFront will failover from 			the primary origin to the second origin under the failover conditions that you've 			chosen.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginGroup {
    ///
    /// A complex type that contains information about the failover criteria for an origin 			group.
    ///
    /// Required: Yes
    ///
    /// Type: OriginGroupFailoverCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailoverCriteria")]
    pub failover_criteria: OriginGroupFailoverCriteria,

    ///
    /// The origin group's ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// A complex type that contains information about the origins in an origin group.
    ///
    /// Required: Yes
    ///
    /// Type: OriginGroupMembers
    ///
    /// Update requires: No interruption
    #[serde(rename = "Members")]
    pub members: OriginGroupMembers,
}

impl cfn_resources::CfnResource for OriginGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.failover_criteria.validate()?;

        self.members.validate()?;

        Ok(())
    }
}

/// A complex data type that includes information about the failover criteria for an 			origin group, including the status codes for which CloudFront will failover from the 			primary origin to the second origin.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginGroupFailoverCriteria {
    ///
    /// The status codes that, when returned from the primary origin, will trigger CloudFront 			to failover to the second origin.
    ///
    /// Required: Yes
    ///
    /// Type: StatusCodes
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusCodes")]
    pub status_codes: StatusCodes,
}

impl cfn_resources::CfnResource for OriginGroupFailoverCriteria {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.status_codes.validate()?;

        Ok(())
    }
}

/// An origin in an origin group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginGroupMember {
    ///
    /// The ID for an origin in an origin group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginId")]
    pub origin_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for OriginGroupMember {
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

/// A complex data type for the origins included in an origin group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginGroupMembers {
    ///
    /// Items (origins) in an origin group.
    ///
    /// Required: Yes
    ///
    /// Type: List of OriginGroupMember
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<OriginGroupMember>,

    ///
    /// The number of origins in an origin group.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Quantity")]
    pub quantity: i64,
}

impl cfn_resources::CfnResource for OriginGroupMembers {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.items;

        if the_val.len() > 2 as _ {
            return Err(format!(
                "Max validation failed on field 'items'. {} is greater than 2",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// A complex data type for the origin groups specified for a distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginGroups {
    ///
    /// The items (origin groups) in a distribution.
    ///
    /// Required: No
    ///
    /// Type: List of OriginGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OriginGroup>>,

    ///
    /// The number of origin groups.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Quantity")]
    pub quantity: i64,
}

impl cfn_resources::CfnResource for OriginGroups {
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

/// CloudFront Origin Shield.
///
/// Using Origin Shield can help reduce the load on your origin. For more information, see 				Using Origin Shield in the 				Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginShield {
    ///
    /// A flag that specifies whether Origin Shield is enabled.
    ///
    /// When it's enabled, CloudFront routes all requests through Origin Shield, which can help 			protect your origin. When it's disabled, CloudFront might send requests directly to your 			origin from multiple edge locations or regional edge caches.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    ///
    /// The AWS Region for Origin Shield.
    ///
    /// Specify the AWS Region that has the lowest latency to your origin. To specify a 			region, use the region code, not the region name. For example, specify the US East 			(Ohio) region as us-east-2.
    ///
    /// When you enable CloudFront Origin Shield, you must specify the AWS Region for Origin 			Shield. For the list of AWS Regions that you can specify, and for help choosing the 			best Region for your origin, see Choosing the AWS Region for Origin Shield in the 				Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [a-z]{2}-[a-z]+-\d
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginShieldRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_shield_region: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for OriginShield {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.origin_shield_region {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 32 as _ {
                    return Err(format!("Max validation failed on field 'origin_shield_region'. {} is greater than 32", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.origin_shield_region {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'origin_shield_region'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A complex type that identifies ways in which you want to restrict distribution of your 			content.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Restrictions {
    ///
    /// A complex type that controls the countries in which your content is distributed. CloudFront 			determines the location of your users using MaxMind GeoIP databases. To disable geo restriction, remove the Restrictions property from your stack template.
    ///
    /// Required: Yes
    ///
    /// Type: GeoRestriction
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeoRestriction")]
    pub geo_restriction: GeoRestriction,
}

impl cfn_resources::CfnResource for Restrictions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.geo_restriction.validate()?;

        Ok(())
    }
}

/// A complex type that contains information about the Amazon S3 origin. If the origin is a 			custom origin or an S3 bucket that is configured as a website endpoint, use the 				CustomOriginConfig element instead.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3OriginConfig {
    ///
    /// The CloudFront origin access identity to associate with the origin. Use an origin access 			identity to configure the origin so that viewers can only access 			objects in an Amazon S3 bucket through CloudFront. The format of the value is:
    ///
    /// origin-access-identity/cloudfront/ID-of-origin-access-identity
    ///
    /// where         ID-of-origin-access-identity       is the value that 			CloudFront returned in the ID element when you created the origin access 			identity.
    ///
    /// If you want viewers to be able to access objects using either the CloudFront URL or the Amazon S3 			URL, specify an empty OriginAccessIdentity element.
    ///
    /// To delete the origin access identity from an existing distribution, update the 			distribution configuration and include an empty OriginAccessIdentity 			element.
    ///
    /// To replace the origin access identity, update the distribution configuration and 			specify the new origin access identity.
    ///
    /// For more information about the origin access identity, see Serving Private 				Content through CloudFront in the Amazon CloudFront Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginAccessIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_identity: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3OriginConfig {
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

/// A complex data type for the status codes that you specify that, when returned by a 			primary origin, trigger CloudFront to failover to a second origin.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StatusCodes {
    ///
    /// The items (status codes) for an origin group.
    ///
    /// Required: Yes
    ///
    /// Type: List of Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Items")]
    pub items: Vec<i64>,

    ///
    /// The number of status codes.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Quantity")]
    pub quantity: i64,
}

impl cfn_resources::CfnResource for StatusCodes {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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

/// A complex type that determines the distribution's SSL/TLS configuration for 			communicating with viewers.
///
/// If the distribution doesn't use Aliases (also known as alternate domain 			names or CNAMEs)—that is, if the distribution uses the CloudFront domain name such as 				d111111abcdef8.cloudfront.net—set 				CloudFrontDefaultCertificate to true and leave all other 			fields empty.
///
/// If the distribution uses Aliases (alternate domain names or CNAMEs), use 			the fields in this type to specify the following settings:
///
/// All distributions support HTTPS connections from viewers. To require viewers to use 			HTTPS only, or to redirect them from HTTP to HTTPS, use 				ViewerProtocolPolicy in the CacheBehavior or 				DefaultCacheBehavior. To specify how CloudFront should use SSL/TLS to 			communicate with your custom origin, use CustomOriginConfig.
///
/// For more information, see Using HTTPS with CloudFront and Using Alternate Domain Names and HTTPS in the 				Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ViewerCertificate {
    ///
    /// NoteIn CloudFormation, this field name is AcmCertificateArn. Note the 				different capitalization.
    ///
    /// If the distribution uses Aliases (alternate domain names or CNAMEs) and 			the SSL/TLS certificate is stored in AWS Certificate Manager (ACM), provide the Amazon Resource Name 			(ARN) of the ACM certificate. CloudFront only supports ACM certificates in the US East 			(N. Virginia) Region (us-east-1).
    ///
    /// If you specify an ACM certificate ARN, you must also specify values for 				MinimumProtocolVersion and SSLSupportMethod. (In CloudFormation, the field name is SslSupportMethod. Note 				the different capitalization.)
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcmCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// If the distribution uses the CloudFront domain name such as 				d111111abcdef8.cloudfront.net, set this field to 			true.
    ///
    /// If the distribution uses Aliases (alternate domain names or CNAMEs), set 			this field to false and specify values for the following fields:
    ///
    /// ACMCertificateArn or IAMCertificateId (specify a 					value for one, not both)        In CloudFormation, these field names are 						AcmCertificateArn and IamCertificateId. Note the 					different capitalization.                        MinimumProtocolVersion                                SSLSupportMethod (In CloudFormation, this field 						name is SslSupportMethod. Note the different 						capitalization.)
    ///
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudFrontDefaultCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_default_certificate: Option<bool>,

    ///
    /// NoteIn CloudFormation, this field name is IamCertificateId. Note the 				different capitalization.
    ///
    /// If the distribution uses Aliases (alternate domain names or CNAMEs) and 			the SSL/TLS certificate is stored in AWS Identity and Access Management (IAM), provide the ID of the IAM certificate.
    ///
    /// If you specify an IAM certificate ID, you must also specify values for 				MinimumProtocolVersion and SSLSupportMethod. (In CloudFormation, the field name is SslSupportMethod. Note 				the different capitalization.)
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_certificate_id: Option<cfn_resources::StrVal>,

    ///
    /// If the distribution uses Aliases (alternate domain names or CNAMEs), 			specify the security policy that you want CloudFront to use for HTTPS connections with 			viewers. The security policy determines two settings:
    ///
    /// The minimum SSL/TLS protocol that CloudFront can use to communicate with 					viewers.               The ciphers that CloudFront can use to encrypt the content that it returns to 					viewers.
    ///
    /// For more information, see Security Policy and Supported Protocols and Ciphers Between Viewers and 				CloudFront in the Amazon CloudFront Developer Guide.
    ///
    /// NoteOn the CloudFront console, this setting is called Security 					Policy.
    ///
    /// When you're using SNI only (you set SSLSupportMethod to 				sni-only), you must specify TLSv1 or higher. (In CloudFormation, the field name is SslSupportMethod. Note 				the different capitalization.)
    ///
    /// If the distribution uses the CloudFront domain name such as 				d111111abcdef8.cloudfront.net (you set 				CloudFrontDefaultCertificate to true), CloudFront automatically 			sets the security policy to TLSv1 regardless of the value that you set 			here.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: SSLv3 | TLSv1 | TLSv1.1_2016 | TLSv1.2_2018 | TLSv1.2_2019 | TLSv1.2_2021 | TLSv1_2016
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumProtocolVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_protocol_version: Option<ViewerCertificateMinimumProtocolVersionEnum>,

    ///
    /// NoteIn CloudFormation, this field name is SslSupportMethod. Note the 				different capitalization.
    ///
    /// If the distribution uses Aliases (alternate domain names or CNAMEs), 			specify which viewers the distribution accepts HTTPS connections from.
    ///
    /// sni-only – The distribution accepts HTTPS connections from only 					viewers that support server name 						indication (SNI). This is recommended. Most browsers and clients 					support SNI.                        vip – The distribution accepts HTTPS connections from all viewers 					including those that don't support SNI. This is not recommended, and results in 					additional monthly charges from CloudFront.                        static-ip - Do not specify this value unless your distribution 					has been enabled for this feature by the CloudFront team. If you have a use case 					that requires static IP addresses for a distribution, contact CloudFront through 					the AWS Support Center.
    ///
    /// If the distribution uses the CloudFront domain name such as 				d111111abcdef8.cloudfront.net, don't set a value for this field.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: sni-only | static-ip | vip
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslSupportMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_support_method: Option<ViewerCertificateSslSupportMethodEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ViewerCertificateMinimumProtocolVersionEnum {
    /// SSLv3
    #[serde(rename = "SSLv3")]
    Sslv3,

    /// TLSv1
    #[serde(rename = "TLSv1")]
    Tlsv1,

    /// TLSv1.1_2016
    #[serde(rename = "TLSv1.1_2016")]
    Tlsv112016,

    /// TLSv1.2_2018
    #[serde(rename = "TLSv1.2_2018")]
    Tlsv122018,

    /// TLSv1.2_2019
    #[serde(rename = "TLSv1.2_2019")]
    Tlsv122019,

    /// TLSv1.2_2021
    #[serde(rename = "TLSv1.2_2021")]
    Tlsv122021,

    /// TLSv1_2016
    #[serde(rename = "TLSv1_2016")]
    Tlsv12016,
}

impl Default for ViewerCertificateMinimumProtocolVersionEnum {
    fn default() -> Self {
        ViewerCertificateMinimumProtocolVersionEnum::Sslv3
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ViewerCertificateSslSupportMethodEnum {
    /// sni-only
    #[serde(rename = "sni-only")]
    Snionly,

    /// static-ip
    #[serde(rename = "static-ip")]
    Staticip,

    /// vip
    #[serde(rename = "vip")]
    Vip,
}

impl Default for ViewerCertificateSslSupportMethodEnum {
    fn default() -> Self {
        ViewerCertificateSslSupportMethodEnum::Snionly
    }
}

impl cfn_resources::CfnResource for ViewerCertificate {
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
