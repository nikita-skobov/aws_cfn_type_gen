/// Creates a continuous deployment policy that routes a subset of production traffic       from a primary distribution to a staging distribution.
///
/// After you create and update a staging distribution, you can use a continuous     deployment policy to incrementally move traffic to the staging distribution. This enables 			you to test changes to a distribution's configuration before moving all of your 			production traffic to the new configuration.
///
/// For more information, see Using       CloudFront continuous deployment to safely test CDN configuration changes       in the Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

    #[serde(skip_serializing)]
    pub att_id: CfnContinuousDeploymentPolicyid,

    #[serde(skip_serializing)]
    pub att_last_modified_time: CfnContinuousDeploymentPolicylastmodifiedtime,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnContinuousDeploymentPolicyid;
impl CfnContinuousDeploymentPolicyid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnContinuousDeploymentPolicylastmodifiedtime;
impl CfnContinuousDeploymentPolicylastmodifiedtime {
    pub fn att_name(&self) -> &'static str {
        r#"LastModifiedTime"#
    }
}

impl cfn_resources::CfnResource for CfnContinuousDeploymentPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::ContinuousDeploymentPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.continuous_deployment_policy_config.validate()?;

        Ok(())
    }
}

/// Contains the configuration for a continuous deployment policy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ContinuousDeploymentPolicyConfig {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_config: Option<TrafficConfig>,
}

impl cfn_resources::CfnResource for ContinuousDeploymentPolicyConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.traffic_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Session stickiness provides the ability to define multiple requests from a single 			viewer as a single session. This prevents the potentially inconsistent experience of 			sending some of a given user's requests to your staging distribution, while others are 			sent to your primary distribution. Define the session duration using TTL values.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for SessionStickinessConfig {
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

/// Determines which HTTP requests are sent to the staging distribution.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub header: cfn_resources::StrVal,

    ///
    /// The request header value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SingleHeaderConfig {
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

/// This configuration determines the percentage of HTTP requests that are sent to the       staging distribution.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SingleWeightConfig {
    ///
    /// Session stickiness provides the ability to define multiple requests from a single 			viewer as a single session. This prevents the potentially inconsistent experience of 			sending some of a given user's requests to your staging distribution, while others are 			sent to your primary distribution. Define the session duration using TTL values.
    ///
    /// Required: No
    ///
    /// Type: SessionStickinessConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionStickinessConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_stickiness_config: Option<SessionStickinessConfig>,

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
}

impl cfn_resources::CfnResource for SingleWeightConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.session_stickiness_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The traffic configuration of your continuous deployment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TrafficConfig {
    ///
    /// Determines which HTTP requests are sent to the staging distribution.
    ///
    /// Required: No
    ///
    /// Type: SingleHeaderConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingleHeaderConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_weight_config: Option<SingleWeightConfig>,

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
    pub cfn_type: TrafficConfigTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TrafficConfigTypeEnum {
    /// SingleHeader
    #[serde(rename = "SingleHeader")]
    Singleheader,

    /// SingleWeight
    #[serde(rename = "SingleWeight")]
    Singleweight,
}

impl Default for TrafficConfigTypeEnum {
    fn default() -> Self {
        TrafficConfigTypeEnum::Singleheader
    }
}

impl cfn_resources::CfnResource for TrafficConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.single_header_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.single_weight_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
