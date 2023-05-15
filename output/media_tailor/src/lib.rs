
pub mod cfn_playback_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnPlaybackConfiguration {
    /// The URL for a high-quality video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID configurations. For VPAID, the slate is required because MediaTailor provides it in the slots that are designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video.
    #[serde(rename = "SlateAdUrl")]
    pub slate_ad_url: Option<String>,
    /// The configuration for bumpers. Bumpers are short audio or video clips that play at the start or before the end of an ad break. To learn more about bumpers, see Bumpers (https://docs.aws.amazon.com/mediatailor/latest/ug/bumpers.html).
    #[serde(rename = "Bumper")]
    pub bumper: Option<Bumper>,
    /// The name that is used to associate this playback configuration with a custom transcode profile. This overrides the dynamic transcoding defaults of MediaTailor. Use this only if you have already set up custom profiles with the help of AWS Support.
    #[serde(rename = "TranscodeProfileName")]
    pub transcode_profile_name: Option<String>,
    /// The configuration for avail suppression, also known as ad suppression. For more information about ad suppression, see Ad Suppression (https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html).
    #[serde(rename = "AvailSuppression")]
    pub avail_suppression: Option<AvailSuppression>,
    /// The tags to assign to the playback configuration.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management.
    #[serde(rename = "CdnConfiguration")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// The identifier for the playback configuration.
    #[serde(rename = "Name")]
    pub name: String,
    /// The configuration for manifest processing rules. Manifest processing rules enable customization of the personalized manifests created by MediaTailor.
    #[serde(rename = "ManifestProcessingRules")]
    pub manifest_processing_rules: Option<ManifestProcessingRules>,
    /// The configuration for DASH content.
    #[serde(rename = "DashConfiguration")]
    pub dash_configuration: Option<DashConfiguration>,
    /// The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing you can provide a static VAST URL. The maximum length is 25,000 characters.
    #[serde(rename = "AdDecisionServerUrl")]
    pub ad_decision_server_url: String,
    /// The configuration for HLS content.
    #[serde(rename = "HlsConfiguration")]
    pub hls_configuration: Option<HlsConfiguration>,
    /// The player parameters and aliases used as dynamic variables during session initialization. For more information, see Domain Variables.
    #[serde(rename = "ConfigurationAliases")]
    pub configuration_aliases: Option<ConfigurationAliases>,
    /// Defines the maximum duration of underfilled ad time (in seconds) allowed in an ad break. If the duration of underfilled ad time exceeds the personalization threshold, then the personalization of the ad break is abandoned and the underlying content is shown. This feature applies to ad replacement in live and VOD streams, rather than ad insertion, because it relies on an underlying content stream. For more information about ad break behavior, including ad replacement and insertion, see Ad Behavior in AWS Elemental MediaTailor (https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html).
    #[serde(rename = "PersonalizationThresholdSeconds")]
    pub personalization_threshold_seconds: Option<usize>,
    /// The configuration for pre-roll ad insertion.
    #[serde(rename = "LivePreRollConfiguration")]
    pub live_pre_roll_configuration: Option<LivePreRollConfiguration>,
    /// The URL prefix for the parent manifest for the stream, minus the asset ID. The maximum length is 512 characters.
    #[serde(rename = "VideoContentSourceUrl")]
    pub video_content_source_url: String,

}


#[derive(serde::Serialize, Default)]
pub struct AvailSuppression {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Mode")]
    pub mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Bumper {
    #[serde(rename = "EndUrl")]
    pub end_url: Option<String>,
    #[serde(rename = "StartUrl")]
    pub start_url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LivePreRollConfiguration {
    #[serde(rename = "AdDecisionServerUrl")]
    pub ad_decision_server_url: Option<String>,
    #[serde(rename = "MaxDurationSeconds")]
    pub max_duration_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsConfiguration {
    #[serde(rename = "ManifestEndpointPrefix")]
    pub manifest_endpoint_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DashConfiguration {
    #[serde(rename = "ManifestEndpointPrefix")]
    pub manifest_endpoint_prefix: Option<String>,
    #[serde(rename = "MpdLocation")]
    pub mpd_location: Option<String>,
    #[serde(rename = "OriginManifestType")]
    pub origin_manifest_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ManifestProcessingRules {
    #[serde(rename = "AdMarkerPassthrough")]
    pub ad_marker_passthrough: Option<AdMarkerPassthrough>,

}

#[derive(serde::Serialize, Default)]
pub struct AdMarkerPassthrough {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigurationAliases {

}

#[derive(serde::Serialize, Default)]
pub struct CdnConfiguration {
    #[serde(rename = "AdSegmentUrlPrefix")]
    pub ad_segment_url_prefix: Option<String>,
    #[serde(rename = "ContentSegmentUrlPrefix")]
    pub content_segment_url_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
