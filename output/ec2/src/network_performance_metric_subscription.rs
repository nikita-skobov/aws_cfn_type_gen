/// Describes Infrastructure Performance subscriptions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnNetworkPerformanceMetricSubscription {
    ///
    /// The Region or Availability Zone that's the target for the subscription. For example, eu-west-1.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Destination")]
    pub destination: cfn_resources::StrVal,

    ///
    /// The metric used for the subscription.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: aggregate-latency
    ///
    /// Update requires: Replacement
    #[serde(rename = "Metric")]
    pub metric: NetworkPerformanceMetricSubscriptionMetricEnum,

    ///
    /// The Region or Availability Zone that's the source for the subscription. For example, us-east-1.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Source")]
    pub source: cfn_resources::StrVal,

    ///
    /// The statistic used for the subscription.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: p50
    ///
    /// Update requires: Replacement
    #[serde(rename = "Statistic")]
    pub statistic: NetworkPerformanceMetricSubscriptionStatisticEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NetworkPerformanceMetricSubscriptionMetricEnum {
    /// aggregate-latency
    #[serde(rename = "aggregate-latency")]
    Aggregatelatency,
}

impl Default for NetworkPerformanceMetricSubscriptionMetricEnum {
    fn default() -> Self {
        NetworkPerformanceMetricSubscriptionMetricEnum::Aggregatelatency
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NetworkPerformanceMetricSubscriptionStatisticEnum {
    /// p50
    #[serde(rename = "p50")]
    P50,
}

impl Default for NetworkPerformanceMetricSubscriptionStatisticEnum {
    fn default() -> Self {
        NetworkPerformanceMetricSubscriptionStatisticEnum::P50
    }
}

impl cfn_resources::CfnResource for CfnNetworkPerformanceMetricSubscription {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::NetworkPerformanceMetricSubscription"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
