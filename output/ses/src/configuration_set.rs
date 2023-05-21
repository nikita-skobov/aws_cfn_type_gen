

/// Configuration sets let you create groups of rules that you can apply to the emails you       send using Amazon SES. For more information about using configuration sets, see Using Amazon         SES Configuration Sets in the Amazon SES Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnConfigurationSet {


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
    pub name: Option<String>,


    /// 
    /// An object that represents the reputation settings for the configuration set.
    /// 
    /// Required: No
    ///
    /// Type: ReputationOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReputationOptions")]
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
    pub sending_options: Option<SendingOptions>,


    /// 
    /// The Virtual Deliverability Manager (VDM) options that apply to the configuration       set.
    /// 
    /// Required: No
    ///
    /// Type: VdmOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "VdmOptions")]
    pub vdm_options: Option<VdmOptions>,


    /// 
    /// The name of the custom open and click tracking domain associated with the       configuration set.
    /// 
    /// Required: No
    ///
    /// Type: TrackingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrackingOptions")]
    pub tracking_options: Option<TrackingOptions>,


    /// 
    /// An object that contains information about the suppression list preferences for your       account.
    /// 
    /// Required: No
    ///
    /// Type: SuppressionOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuppressionOptions")]
    pub suppression_options: Option<SuppressionOptions>,


    /// 
    /// Specifies whether messages that use the configuration set are required to use       Transport Layer Security (TLS).
    /// 
    /// Required: No
    ///
    /// Type: DeliveryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryOptions")]
    pub delivery_options: Option<DeliveryOptions>,

}


/// The Virtual Deliverability Manager (VDM) options that apply to a configuration       set.
#[derive(Default, serde::Serialize)]
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
    pub guardian_options: Option<GuardianOptions>,

}


/// Specifies whether messages that use the configuration set are required to use       Transport Layer Security (TLS).
#[derive(Default, serde::Serialize)]
pub struct DeliveryOptions {


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
    pub tls_policy: Option<String>,


    /// 
    /// The name of the dedicated IP pool to associate with the configuration set.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SendingPoolName")]
    pub sending_pool_name: Option<String>,

}


/// Settings for your VDM configuration as applicable to the Dashboard.
#[derive(Default, serde::Serialize)]
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
    pub engagement_metrics: String,

}


/// An object that contains information about the suppression list preferences for your       account.
#[derive(Default, serde::Serialize)]
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
    pub suppressed_reasons: Option<Vec<String>>,

}


/// Settings for your VDM configuration as applicable to the Guardian.
#[derive(Default, serde::Serialize)]
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
    pub optimized_shared_delivery: String,

}


/// Used to enable or disable email sending for messages that use this configuration set       in the current AWS Region.
#[derive(Default, serde::Serialize)]
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
    pub sending_enabled: Option<bool>,

}


/// Contains information about the reputation settings for a configuration set.
#[derive(Default, serde::Serialize)]
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
    pub reputation_metrics_enabled: Option<bool>,

}


/// A domain that is used to redirect email recipients to an Amazon SES-operated domain. This       domain captures open and click events generated by Amazon SES emails.
///
/// For more information, see Configuring Custom         Domains to Handle Open and Click Tracking in the Amazon SES Developer         Guide.
#[derive(Default, serde::Serialize)]
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
    pub custom_redirect_domain: Option<String>,

}