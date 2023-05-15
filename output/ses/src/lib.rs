
pub mod cfn_configuration_set {

#[derive(serde::Serialize, Default)]
pub struct CfnConfigurationSet {
    /// An object that defines whether or not Amazon SES can send email that you send using the configuration set.
    #[serde(rename = "SendingOptions")]
    pub sending_options: Option<SendingOptions>,
    /// An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.
    #[serde(rename = "DeliveryOptions")]
    pub delivery_options: Option<DeliveryOptions>,
    /// An object that defines the open and click tracking options for emails that you send using the configuration set.
    #[serde(rename = "TrackingOptions")]
    pub tracking_options: Option<TrackingOptions>,
    /// An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set.
    #[serde(rename = "ReputationOptions")]
    pub reputation_options: Option<ReputationOptions>,
    /// An object that contains information about the suppression list preferences for your account.
    #[serde(rename = "SuppressionOptions")]
    pub suppression_options: Option<SuppressionOptions>,
    /// An object that contains Virtual Deliverability Manager (VDM) settings for this configuration set.
    #[serde(rename = "VdmOptions")]
    pub vdm_options: Option<VdmOptions>,
    /// The name of the configuration set.
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ReputationOptions {
    #[serde(rename = "ReputationMetricsEnabled")]
    pub reputation_metrics_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct TrackingOptions {
    #[serde(rename = "CustomRedirectDomain")]
    pub custom_redirect_domain: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VdmOptions {
    #[serde(rename = "DashboardOptions")]
    pub dashboard_options: Option<DashboardOptions>,
    #[serde(rename = "GuardianOptions")]
    pub guardian_options: Option<GuardianOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DeliveryOptions {
    #[serde(rename = "SendingPoolName")]
    pub sending_pool_name: Option<String>,
    #[serde(rename = "TlsPolicy")]
    pub tls_policy: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SuppressionOptions {
    #[serde(rename = "SuppressedReasons")]
    pub suppressed_reasons: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct SendingOptions {
    #[serde(rename = "SendingEnabled")]
    pub sending_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct GuardianOptions {
    #[serde(rename = "OptimizedSharedDelivery")]
    pub optimized_shared_delivery: String,

}

#[derive(serde::Serialize, Default)]
pub struct DashboardOptions {
    #[serde(rename = "EngagementMetrics")]
    pub engagement_metrics: String,

}


}

pub mod cfn_configuration_set_event_destination {

#[derive(serde::Serialize, Default)]
pub struct CfnConfigurationSetEventDestination {
    /// The name of the configuration set that contains the event destination.
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// The event destination object.
    #[serde(rename = "EventDestination")]
    pub event_destination: EventDestination,

}


#[derive(serde::Serialize, Default)]
pub struct DimensionConfiguration {
    #[serde(rename = "DefaultDimensionValue")]
    pub default_dimension_value: String,
    #[serde(rename = "DimensionValueSource")]
    pub dimension_value_source: String,
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct EventDestination {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "MatchingEventTypes")]
    pub matching_event_types: Vec<String>,
    #[serde(rename = "CloudWatchDestination")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    #[serde(rename = "KinesisFirehoseDestination")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    #[serde(rename = "SnsDestination")]
    pub sns_destination: Option<SnsDestination>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct SnsDestination {
    #[serde(rename = "TopicARN")]
    pub topic_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchDestination {
    #[serde(rename = "DimensionConfigurations")]
    pub dimension_configurations: Option<Vec<DimensionConfiguration>>,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisFirehoseDestination {
    #[serde(rename = "IAMRoleARN")]
    pub iamrole_arn: String,
    #[serde(rename = "DeliveryStreamARN")]
    pub delivery_stream_arn: String,

}


}

pub mod cfn_contact_list {

#[derive(serde::Serialize, Default)]
pub struct CfnContactList {
    /// The description of the contact list.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The name of the contact list.
    #[serde(rename = "ContactListName")]
    pub contact_list_name: Option<String>,
    /// The tags (keys and values) associated with the contact list.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The topics associated with the contact list.
    #[serde(rename = "Topics")]
    pub topics: Option<Vec<Topic>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Topic {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "TopicName")]
    pub topic_name: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "DefaultSubscriptionStatus")]
    pub default_subscription_status: String,

}


}

pub mod cfn_dedicated_ip_pool {

#[derive(serde::Serialize, Default)]
pub struct CfnDedicatedIpPool {
    /// Specifies whether the dedicated IP pool is managed or not. The default value is STANDARD.
    #[serde(rename = "ScalingMode")]
    pub scaling_mode: Option<String>,
    /// The name of the dedicated IP pool.
    #[serde(rename = "PoolName")]
    pub pool_name: Option<String>,

}



}

pub mod cfn_email_identity {

#[derive(serde::Serialize, Default)]
pub struct CfnEmailIdentity {
    /// If your request includes this object, Amazon SES configures the identity to use Bring Your Own DKIM (BYODKIM) for DKIM authentication purposes, or, configures the key length to be used for Easy DKIM.
    #[serde(rename = "DkimSigningAttributes")]
    pub dkim_signing_attributes: Option<DkimSigningAttributes>,
    /// Used to associate a configuration set with an email identity.
    #[serde(rename = "ConfigurationSetAttributes")]
    pub configuration_set_attributes: Option<ConfigurationSetAttributes>,
    /// Used to enable or disable DKIM authentication for an email identity.
    #[serde(rename = "DkimAttributes")]
    pub dkim_attributes: Option<DkimAttributes>,
    /// Used to enable or disable the custom Mail-From domain configuration for an email identity.
    #[serde(rename = "MailFromAttributes")]
    pub mail_from_attributes: Option<MailFromAttributes>,
    /// Used to enable or disable feedback forwarding for an identity.
    #[serde(rename = "FeedbackAttributes")]
    pub feedback_attributes: Option<FeedbackAttributes>,
    /// The email address or domain to verify.
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,

}


#[derive(serde::Serialize, Default)]
pub struct DkimAttributes {
    #[serde(rename = "SigningEnabled")]
    pub signing_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct DkimSigningAttributes {
    #[serde(rename = "DomainSigningSelector")]
    pub domain_signing_selector: Option<String>,
    #[serde(rename = "NextSigningKeyLength")]
    pub next_signing_key_length: Option<String>,
    #[serde(rename = "DomainSigningPrivateKey")]
    pub domain_signing_private_key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MailFromAttributes {
    #[serde(rename = "BehaviorOnMxFailure")]
    pub behavior_on_mx_failure: Option<String>,
    #[serde(rename = "MailFromDomain")]
    pub mail_from_domain: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FeedbackAttributes {
    #[serde(rename = "EmailForwardingEnabled")]
    pub email_forwarding_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigurationSetAttributes {
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: Option<String>,

}


}

pub mod cfn_template {

#[derive(serde::Serialize, Default)]
pub struct CfnTemplate {
    /// The content of the email, composed of a subject line, an HTML part, and a text-only part
    #[serde(rename = "Template")]
    pub template: Option<Template>,

}


#[derive(serde::Serialize, Default)]
pub struct Template {
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,
    #[serde(rename = "TextPart")]
    pub text_part: Option<String>,
    #[serde(rename = "SubjectPart")]
    pub subject_part: String,
    #[serde(rename = "HtmlPart")]
    pub html_part: Option<String>,

}


}

pub mod cfn_vdm_attributes {

#[derive(serde::Serialize, Default)]
pub struct CfnVdmAttributes {
    /// Preferences regarding the Guardian feature.
    #[serde(rename = "GuardianAttributes")]
    pub guardian_attributes: Option<GuardianAttributes>,
    /// Preferences regarding the Dashboard feature.
    #[serde(rename = "DashboardAttributes")]
    pub dashboard_attributes: Option<DashboardAttributes>,

}


#[derive(serde::Serialize, Default)]
pub struct GuardianAttributes {
    #[serde(rename = "OptimizedSharedDelivery")]
    pub optimized_shared_delivery: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DashboardAttributes {
    #[serde(rename = "EngagementMetrics")]
    pub engagement_metrics: Option<String>,

}


}
