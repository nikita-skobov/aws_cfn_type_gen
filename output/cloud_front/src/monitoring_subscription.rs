

/// A monitoring subscription. This structure contains information about whether 			additional CloudWatch metrics are enabled for a given CloudFront distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMonitoringSubscription {


    /// 
    /// A subscription configuration for additional CloudWatch metrics.
    /// 
    /// Required: Yes
    ///
    /// Type: MonitoringSubscription
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringSubscription")]
    pub monitoring_subscription: Box<MonitoringSubscription>,


    /// 
    /// The ID of the distribution that you are enabling metrics for.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DistributionId")]
    pub distribution_id: String,

}

impl cfn_resources::CfnResource for CfnMonitoringSubscription {
    fn type_string() -> &'static str {
        "AWS::CloudFront::MonitoringSubscription"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A monitoring subscription. This structure contains information about whether 			additional CloudWatch metrics are enabled for a given CloudFront distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringSubscription {


    /// 
    /// A subscription configuration for additional CloudWatch metrics.
    /// 
    /// Required: No
    ///
    /// Type: RealtimeMetricsSubscriptionConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RealtimeMetricsSubscriptionConfig")]
    pub realtime_metrics_subscription_config: Option<RealtimeMetricsSubscriptionConfig>,

}


/// A subscription configuration for additional CloudWatch metrics.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RealtimeMetricsSubscriptionConfig {


    /// 
    /// A flag that indicates whether additional CloudWatch metrics are enabled for a given 			CloudFront distribution.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "RealtimeMetricsSubscriptionStatus")]
    pub realtime_metrics_subscription_status: String,

}
