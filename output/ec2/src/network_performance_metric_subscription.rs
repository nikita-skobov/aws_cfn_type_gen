

/// Describes Infrastructure Performance subscriptions.
#[derive(Default, serde::Serialize)]
pub struct CfnNetworkPerformanceMetricSubscription {


    /// 
    /// The Region or Availability Zone that's the source for the subscription. For example, us-east-1.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Source")]
    pub source: String,


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
    pub statistic: String,


    /// 
    /// The Region or Availability Zone that's the target for the subscription. For example, eu-west-1.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Destination")]
    pub destination: String,


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
    pub metric: String,

}