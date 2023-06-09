/// Configuration sets let you create groups of rules that you can apply to the emails you       send using Amazon SES. For more information about using configuration sets, see Using Amazon         SES Configuration Sets in the Amazon SES Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnConfigurationSet {
    ///
    /// Specifies whether messages that use the configuration set are required to use       Transport Layer Security (TLS).
    ///
    /// Required: No
    ///
    /// Type: DeliveryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,

    ///
    /// The name of the configuration set. The name must meet the following       requirements:
    ///
    /// Contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes           (-).               Contain 64 characters or fewer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// An object that represents the reputation settings for the configuration set.
    ///
    /// Required: No
    ///
    /// Type: ReputationOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReputationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_options: Option<ReputationOptions>,

    ///
    /// An object that defines whether or not Amazon SES can send email that you send using       the configuration set.
    ///
    /// Required: No
    ///
    /// Type: SendingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SendingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_options: Option<SendingOptions>,

    ///
    /// An object that contains information about the suppression list preferences for your       account.
    ///
    /// Required: No
    ///
    /// Type: SuppressionOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuppressionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_options: Option<SuppressionOptions>,

    ///
    /// The name of the custom open and click tracking domain associated with the       configuration set.
    ///
    /// Required: No
    ///
    /// Type: TrackingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrackingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_options: Option<TrackingOptions>,

    ///
    /// The Virtual Deliverability Manager (VDM) options that apply to the configuration       set.
    ///
    /// Required: No
    ///
    /// Type: VdmOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "VdmOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdm_options: Option<VdmOptions>,
}

impl cfn_resources::CfnResource for CfnConfigurationSet {
    fn type_string(&self) -> &'static str {
        "AWS::SES::ConfigurationSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.delivery_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.reputation_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sending_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.suppression_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.tracking_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vdm_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for your VDM configuration as applicable to the Dashboard.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DashboardOptions {
    ///
    /// Specifies the status of your VDM engagement metrics collection. Can be one of the       following:
    ///
    /// ENABLED – Amazon SES enables engagement metrics for the           configuration set.               DISABLED – Amazon SES disables engagement metrics for           the configuration set.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngagementMetrics")]
    pub engagement_metrics: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for DashboardOptions {
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

/// Specifies whether messages that use the configuration set are required to use       Transport Layer Security (TLS).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeliveryOptions {
    ///
    /// The name of the dedicated IP pool to associate with the configuration set.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SendingPoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_pool_name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether messages that use the configuration set are required to use       Transport Layer Security (TLS). If the value is REQUIRE, messages are only       delivered if a TLS connection can be established. If the value is OPTIONAL,       messages can be delivered in plain text if a TLS connection can't be established.
    ///
    /// Valid Values: REQUIRE | OPTIONAL
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TlsPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<DeliveryOptionsTlsPolicyEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DeliveryOptionsTlsPolicyEnum {
    /// REQUIRE
    #[serde(rename = "REQUIRE")]
    Require,

    /// OPTIONAL
    #[serde(rename = "OPTIONAL")]
    Optional,
}

impl Default for DeliveryOptionsTlsPolicyEnum {
    fn default() -> Self {
        DeliveryOptionsTlsPolicyEnum::Require
    }
}

impl cfn_resources::CfnResource for DeliveryOptions {
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

/// Settings for your VDM configuration as applicable to the Guardian.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GuardianOptions {
    ///
    /// Specifies the status of your VDM optimized shared delivery. Can be one of the       following:
    ///
    /// ENABLED – Amazon SES enables optimized shared delivery           for the configuration set.               DISABLED – Amazon SES disables optimized shared           delivery for the configuration set.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptimizedSharedDelivery")]
    pub optimized_shared_delivery: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for GuardianOptions {
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

/// Contains information about the reputation settings for a configuration set.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReputationOptions {
    ///
    /// Describes whether or not Amazon SES publishes reputation metrics for the configuration set,       such as bounce and complaint rates, to Amazon CloudWatch.
    ///
    /// If the value is true, reputation metrics are published. If the value is         false, reputation metrics are not published. The default value is         false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReputationMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_metrics_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for ReputationOptions {
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

/// Used to enable or disable email sending for messages that use this configuration set       in the current AWS Region.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SendingOptions {
    ///
    /// If true, email sending is enabled for the configuration set. If         false, email sending is disabled for the configuration set.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for SendingOptions {
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

/// An object that contains information about the suppression list preferences for your       account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SuppressionOptions {
    ///
    /// A list that contains the reasons that email addresses are automatically added to the       suppression list for your account. This list can contain any or all of the       following:
    ///
    /// COMPLAINT – Amazon SES adds an email address to the suppression           list for your account when a message sent to that address results in a           complaint.               BOUNCE – Amazon SES adds an email address to the suppression list           for your account when a message sent to that address results in a hard           bounce.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuppressedReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for SuppressionOptions {
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

/// A domain that is used to redirect email recipients to an Amazon SES-operated domain. This       domain captures open and click events generated by Amazon SES emails.
///
/// For more information, see Configuring Custom         Domains to Handle Open and Click Tracking in the Amazon SES Developer         Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TrackingOptions {
    ///
    /// The custom subdomain that is used to redirect email recipients to the Amazon SES event       tracking domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomRedirectDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_redirect_domain: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TrackingOptions {
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

/// The Virtual Deliverability Manager (VDM) options that apply to a configuration       set.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VdmOptions {
    ///
    /// Settings for your VDM configuration as applicable to the Dashboard.
    ///
    /// Required: No
    ///
    /// Type: DashboardOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashboardOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_options: Option<DashboardOptions>,

    ///
    /// Settings for your VDM configuration as applicable to the Guardian.
    ///
    /// Required: No
    ///
    /// Type: GuardianOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "GuardianOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardian_options: Option<GuardianOptions>,
}

impl cfn_resources::CfnResource for VdmOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dashboard_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.guardian_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
