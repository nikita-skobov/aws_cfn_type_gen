/// The AWS::AppSync::ApiCache resource represents the input of a CreateApiCache     operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnApiCache {
    ///
    /// Caching behavior.
    ///
    /// FULL_REQUEST_CACHING: All requests are fully        cached.                        PER_RESOLVER_CACHING: Individual resolvers        that you specify are cached.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiCachingBehavior")]
    pub api_caching_behavior: cfn_resources::StrVal,

    ///
    /// The GraphQL API ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: cfn_resources::StrVal,

    ///
    /// At-rest encryption flag for cache. You cannot update this setting after creation.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AtRestEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,

    ///
    /// Transit encryption flag when connecting to cache. You cannot update this setting after     creation.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,

    ///
    /// TTL in seconds for cache entries.
    ///
    /// Valid values are 1–3,600 seconds.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ttl")]
    pub ttl: f64,

    ///
    /// The cache instance type. Valid values are
    ///
    /// SMALL                                MEDIUM                                LARGE                                XLARGE                                LARGE_2X                                LARGE_4X                                LARGE_8X (not available in all regions)                        LARGE_12X
    ///
    /// Historically, instance types were identified by an EC2-style value. As of July 2020, this is deprecated, and the generic identifiers above should be used.
    ///
    /// The following legacy instance types are available, but their use is discouraged:
    ///
    /// T2_SMALL: A t2.small instance type.                        T2_MEDIUM: A t2.medium instance type.                        R4_LARGE: A r4.large instance type.                        R4_XLARGE: A r4.xlarge instance type.                        R4_2XLARGE: A r4.2xlarge instance type.                        R4_4XLARGE: A r4.4xlarge instance type.                        R4_8XLARGE: A r4.8xlarge instance type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnApiCache {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::ApiCache"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
