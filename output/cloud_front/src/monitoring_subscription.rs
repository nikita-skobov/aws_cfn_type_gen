/// A monitoring subscription. This structure contains information about whether 			additional CloudWatch metrics are enabled for a given CloudFront distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMonitoringSubscription {
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
}

impl cfn_resources::CfnResource for CfnMonitoringSubscription {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::MonitoringSubscription"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.monitoring_subscription.validate()?;

        Ok(())
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

impl cfn_resources::CfnResource for MonitoringSubscription {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.realtime_metrics_subscription_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    pub realtime_metrics_subscription_status:
        RealtimeMetricsSubscriptionConfigRealtimeMetricsSubscriptionStatusEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RealtimeMetricsSubscriptionConfigRealtimeMetricsSubscriptionStatusEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for RealtimeMetricsSubscriptionConfigRealtimeMetricsSubscriptionStatusEnum {
    fn default() -> Self {
        RealtimeMetricsSubscriptionConfigRealtimeMetricsSubscriptionStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for RealtimeMetricsSubscriptionConfig {
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
