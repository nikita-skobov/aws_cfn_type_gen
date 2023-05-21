

/// Creates a continuous deployment policy that routes a subset of production traffic       from a primary distribution to a staging distribution.
///
/// After you create and update a staging distribution, you can use a continuous     deployment policy to incrementally move traffic to the staging distribution. This enables 			you to test changes to a distribution's configuration before moving all of your 			production traffic to the new configuration.
///
/// For more information, see Using       CloudFront continuous deployment to safely test CDN configuration changes       in the Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnContinuousDeploymentPolicy {


    /// 
    /// Contains the configuration for a continuous deployment policy.
    /// 
    /// Required: Yes
    ///
    /// Type: ContinuousDeploymentPolicyConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContinuousDeploymentPolicyConfig")]
    pub continuous_deployment_policy_config: ContinuousDeploymentPolicyConfig,

}

impl cfn_resources::CfnResource for CfnContinuousDeploymentPolicy {
    fn type_string() -> &'static str {
        "AWS::CloudFront::ContinuousDeploymentPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains the configuration for a continuous deployment policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContinuousDeploymentPolicyConfig {


    /// 
    /// The CloudFront domain name of the staging distribution. For example: 				d111111abcdef8.cloudfront.net.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StagingDistributionDnsNames")]
    pub staging_distribution_dns_names: Vec<String>,


    /// 
    /// Contains the parameters for routing production traffic from your primary to staging 			distributions.
    /// 
    /// Required: No
    ///
    /// Type: TrafficConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrafficConfig")]
    pub traffic_config: Option<TrafficConfig>,


    /// 
    /// A Boolean that indicates whether this continuous deployment policy is enabled (in 			effect). When this value is true, this policy is enabled and in effect. 			When this value is false, this policy is not enabled and has no 			effect.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}


/// This configuration determines the percentage of HTTP requests that are sent to the       staging distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SingleWeightConfig {


    /// 
    /// The percentage of traffic to send to a staging distribution, expressed as a decimal 			number between 0 and .15.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    pub weight: f64,


    /// 
    /// Session stickiness provides the ability to define multiple requests from a single 			viewer as a single session. This prevents the potentially inconsistent experience of 			sending some of a given user's requests to your staging distribution, while others are 			sent to your primary distribution. Define the session duration using TTL values.
    /// 
    /// Required: No
    ///
    /// Type: SessionStickinessConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionStickinessConfig")]
    pub session_stickiness_config: Option<SessionStickinessConfig>,

}


/// The traffic configuration of your continuous deployment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrafficConfig {


    /// 
    /// The type of traffic configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SingleHeader | SingleWeight
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// Determines which HTTP requests are sent to the staging distribution.
    /// 
    /// Required: No
    ///
    /// Type: SingleHeaderConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingleHeaderConfig")]
    pub single_header_config: Option<SingleHeaderConfig>,


    /// 
    /// Contains the percentage of traffic to send to the staging distribution.
    /// 
    /// Required: No
    ///
    /// Type: SingleWeightConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingleWeightConfig")]
    pub single_weight_config: Option<SingleWeightConfig>,

}


/// Determines which HTTP requests are sent to the staging distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SingleHeaderConfig {


    /// 
    /// The request header name that you want CloudFront to send to your staging 			distribution. The header must contain the prefix aws-cf-cd-.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Header")]
    pub header: String,


    /// 
    /// The request header value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


/// Session stickiness provides the ability to define multiple requests from a single 			viewer as a single session. This prevents the potentially inconsistent experience of 			sending some of a given user's requests to your staging distribution, while others are 			sent to your primary distribution. Define the session duration using TTL values.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SessionStickinessConfig {


    /// 
    /// The amount of time after which you want sessions to cease if no requests are 			received. Allowed values are 300–3600 seconds (5–60 minutes).
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdleTTL")]
    pub idle_ttl: i64,


    /// 
    /// The maximum amount of time to consider requests from the viewer as being part of the same 			session. Allowed values are 300–3600 seconds (5–60 minutes).
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumTTL")]
    pub maximum_ttl: i64,

}
