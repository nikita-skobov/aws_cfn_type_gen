
pub mod cfn_asset {

#[derive(serde::Serialize, Default)]
pub struct CfnAsset {
    /// ARN of the source object in S3.
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// The resource ID to include in SPEKE key requests.
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    /// The IAM role_arn used to access the source S3 bucket.
    #[serde(rename = "SourceRoleArn")]
    pub source_role_arn: String,
    /// The ID of the PackagingGroup for the Asset.
    #[serde(rename = "PackagingGroupId")]
    pub packaging_group_id: String,
    /// The list of egress endpoints available for the Asset.
    #[serde(rename = "EgressEndpoints")]
    pub egress_endpoints: Option<Vec<EgressEndpoint>>,
    /// A collection of tags associated with a resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct EgressEndpoint {
    #[serde(rename = "PackagingConfigurationId")]
    pub packaging_configuration_id: String,
    #[serde(rename = "Url")]
    pub url: String,

}


}

pub mod cfn_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnChannel {
    /// A collection of tags associated with a resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// An HTTP Live Streaming (HLS) ingest resource configuration.
    #[serde(rename = "HlsIngest")]
    pub hls_ingest: Option<HlsIngest>,
    /// A short text description of the Channel.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The configuration parameters for egress access logging.
    #[serde(rename = "EgressAccessLogs")]
    pub egress_access_logs: Option<LogConfiguration>,
    /// The configuration parameters for egress access logging.
    #[serde(rename = "IngressAccessLogs")]
    pub ingress_access_logs: Option<LogConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct IngestEndpoint {
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Password")]
    pub password: String,

}

#[derive(serde::Serialize, Default)]
pub struct HlsIngest {
    #[serde(rename = "ingestEndpoints")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct LogConfiguration {
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,

}


}

pub mod cfn_origin_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnOriginEndpoint {
    /// A short string appended to the end of the OriginEndpoint URL.
    #[serde(rename = "ManifestName")]
    pub manifest_name: Option<String>,
    /// A Common Media Application Format (CMAF) packaging configuration.
    #[serde(rename = "CmafPackage")]
    pub cmaf_package: Option<CmafPackage>,
    /// CDN Authorization credentials
    #[serde(rename = "Authorization")]
    pub authorization: Option<Authorization>,
    /// The ID of the Channel the OriginEndpoint is associated with.
    #[serde(rename = "ChannelId")]
    pub channel_id: String,
    /// Maximum duration (seconds) of content to retain for startover playback. If not specified, startover playback will be disabled for the OriginEndpoint.
    #[serde(rename = "StartoverWindowSeconds")]
    pub startover_window_seconds: Option<usize>,
    /// A short text description of the OriginEndpoint.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination
    #[serde(rename = "Origination")]
    pub origination: Option<String>,
    /// A Microsoft Smooth Streaming (MSS) packaging configuration.
    #[serde(rename = "MssPackage")]
    pub mss_package: Option<MssPackage>,
    /// Amount of delay (seconds) to enforce on the playback of live content. If not specified, there will be no time delay in effect for the OriginEndpoint.
    #[serde(rename = "TimeDelaySeconds")]
    pub time_delay_seconds: Option<usize>,
    /// The ID of the OriginEndpoint.
    #[serde(rename = "Id")]
    pub id: String,
    /// A collection of tags associated with a resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.
    #[serde(rename = "Whitelist")]
    pub whitelist: Option<Vec<String>>,
    /// A Dynamic Adaptive Streaming over HTTP (DASH) packaging configuration.
    #[serde(rename = "DashPackage")]
    pub dash_package: Option<DashPackage>,
    /// An HTTP Live Streaming (HLS) packaging configuration.
    #[serde(rename = "HlsPackage")]
    pub hls_package: Option<HlsPackage>,

}

pub type AdsOnDeliveryRestrictions = String;
#[derive(serde::Serialize, Default)]
pub struct CmafEncryption {
    #[serde(rename = "ConstantInitializationVector")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "KeyRotationIntervalSeconds")]
    pub key_rotation_interval_seconds: Option<usize>,
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<String>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}

#[derive(serde::Serialize, Default)]
pub struct StreamSelection {
    #[serde(rename = "StreamOrder")]
    pub stream_order: Option<String>,
    #[serde(rename = "MinVideoBitsPerSecond")]
    pub min_video_bits_per_second: Option<usize>,
    #[serde(rename = "MaxVideoBitsPerSecond")]
    pub max_video_bits_per_second: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SpekeKeyProvider {
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "SystemIds")]
    pub system_ids: Vec<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "EncryptionContractConfiguration")]
    pub encryption_contract_configuration: Option<EncryptionContractConfiguration>,
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CmafPackage {
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<usize>,
    #[serde(rename = "SegmentPrefix")]
    pub segment_prefix: Option<String>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<CmafEncryption>,
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,
    #[serde(rename = "HlsManifests")]
    pub hls_manifests: Option<Vec<HlsManifest>>,

}

#[derive(serde::Serialize, Default)]
pub struct MssPackage {
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<usize>,
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<MssEncryption>,
    #[serde(rename = "ManifestWindowSeconds")]
    pub manifest_window_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DashEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
    #[serde(rename = "KeyRotationIntervalSeconds")]
    pub key_rotation_interval_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsPackage {
    #[serde(rename = "IncludeIframeOnlyStream")]
    pub include_iframe_only_stream: Option<bool>,
    #[serde(rename = "AdTriggers")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "PlaylistWindowSeconds")]
    pub playlist_window_seconds: Option<usize>,
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<usize>,
    #[serde(rename = "PlaylistType")]
    pub playlist_type: Option<String>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    pub ads_on_delivery_restrictions: Option<AdsOnDeliveryRestrictions>,
    #[serde(rename = "IncludeDvbSubtitles")]
    pub include_dvb_subtitles: Option<bool>,
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    pub program_date_time_interval_seconds: Option<usize>,
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<HlsEncryption>,
    #[serde(rename = "UseAudioRenditionGroup")]
    pub use_audio_rendition_group: Option<bool>,
    #[serde(rename = "AdMarkers")]
    pub ad_markers: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MssEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}

#[derive(serde::Serialize, Default)]
pub struct HlsManifest {
    #[serde(rename = "IncludeIframeOnlyStream")]
    pub include_iframe_only_stream: Option<bool>,
    #[serde(rename = "ManifestName")]
    pub manifest_name: Option<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "AdTriggers")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    pub program_date_time_interval_seconds: Option<usize>,
    #[serde(rename = "AdMarkers")]
    pub ad_markers: Option<String>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    pub ads_on_delivery_restrictions: Option<AdsOnDeliveryRestrictions>,
    #[serde(rename = "PlaylistWindowSeconds")]
    pub playlist_window_seconds: Option<usize>,
    #[serde(rename = "PlaylistType")]
    pub playlist_type: Option<String>,
    #[serde(rename = "Url")]
    pub url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionContractConfiguration {
    #[serde(rename = "PresetSpeke20Audio")]
    pub preset_speke20_audio: String,
    #[serde(rename = "PresetSpeke20Video")]
    pub preset_speke20_video: String,

}

#[derive(serde::Serialize, Default)]
pub struct DashPackage {
    #[serde(rename = "PeriodTriggers")]
    pub period_triggers: Option<Vec<String>>,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
    #[serde(rename = "ManifestLayout")]
    pub manifest_layout: Option<String>,
    #[serde(rename = "MinBufferTimeSeconds")]
    pub min_buffer_time_seconds: Option<usize>,
    #[serde(rename = "MinUpdatePeriodSeconds")]
    pub min_update_period_seconds: Option<usize>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<DashEncryption>,
    #[serde(rename = "SuggestedPresentationDelaySeconds")]
    pub suggested_presentation_delay_seconds: Option<usize>,
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<usize>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    pub ads_on_delivery_restrictions: Option<AdsOnDeliveryRestrictions>,
    #[serde(rename = "UtcTiming")]
    pub utc_timing: Option<String>,
    #[serde(rename = "ManifestWindowSeconds")]
    pub manifest_window_seconds: Option<usize>,
    #[serde(rename = "AdTriggers")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "SegmentTemplateFormat")]
    pub segment_template_format: Option<String>,
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,
    #[serde(rename = "UtcTimingUri")]
    pub utc_timing_uri: Option<String>,
    #[serde(rename = "IncludeIframeOnlyStream")]
    pub include_iframe_only_stream: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Authorization {
    #[serde(rename = "CdnIdentifierSecret")]
    pub cdn_identifier_secret: String,
    #[serde(rename = "SecretsRoleArn")]
    pub secrets_role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct HlsEncryption {
    #[serde(rename = "ConstantInitializationVector")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<String>,
    #[serde(rename = "RepeatExtXKey")]
    pub repeat_ext_xkey: Option<bool>,
    #[serde(rename = "KeyRotationIntervalSeconds")]
    pub key_rotation_interval_seconds: Option<usize>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_packaging_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnPackagingConfiguration {
    /// The ID of a PackagingGroup.
    #[serde(rename = "PackagingGroupId")]
    pub packaging_group_id: String,
    /// A Microsoft Smooth Streaming (MSS) PackagingConfiguration.
    #[serde(rename = "MssPackage")]
    pub mss_package: Option<MssPackage>,
    /// A collection of tags associated with a resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A Dynamic Adaptive Streaming over HTTP (DASH) packaging configuration.
    #[serde(rename = "DashPackage")]
    pub dash_package: Option<DashPackage>,
    /// A CMAF packaging configuration.
    #[serde(rename = "CmafPackage")]
    pub cmaf_package: Option<CmafPackage>,
    /// An HTTP Live Streaming (HLS) packaging configuration.
    #[serde(rename = "HlsPackage")]
    pub hls_package: Option<HlsPackage>,
    /// The ID of the PackagingConfiguration.
    #[serde(rename = "Id")]
    pub id: String,

}

pub type SegmentDurationSeconds = usize;
#[derive(serde::Serialize, Default)]
pub struct CmafPackage {
    #[serde(rename = "Encryption")]
    pub encryption: Option<CmafEncryption>,
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<SegmentDurationSeconds>,
    #[serde(rename = "HlsManifests")]
    pub hls_manifests: Vec<HlsManifest>,
    #[serde(rename = "IncludeEncoderConfigurationInSegments")]
    pub include_encoder_configuration_in_segments: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsPackage {
    #[serde(rename = "Encryption")]
    pub encryption: Option<HlsEncryption>,
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<SegmentDurationSeconds>,
    #[serde(rename = "HlsManifests")]
    pub hls_manifests: Vec<HlsManifest>,
    #[serde(rename = "UseAudioRenditionGroup")]
    pub use_audio_rendition_group: Option<bool>,
    #[serde(rename = "IncludeDvbSubtitles")]
    pub include_dvb_subtitles: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionContractConfiguration {
    #[serde(rename = "PresetSpeke20Video")]
    pub preset_speke20_video: String,
    #[serde(rename = "PresetSpeke20Audio")]
    pub preset_speke20_audio: String,

}

#[derive(serde::Serialize, Default)]
pub struct DashEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}

#[derive(serde::Serialize, Default)]
pub struct SpekeKeyProvider {
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "EncryptionContractConfiguration")]
    pub encryption_contract_configuration: Option<EncryptionContractConfiguration>,
    #[serde(rename = "SystemIds")]
    pub system_ids: Vec<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: RoleArn,

}
pub type ManifestName = String;
#[derive(serde::Serialize, Default)]
pub struct MssManifest {
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,
    #[serde(rename = "ManifestName")]
    pub manifest_name: Option<ManifestName>,

}

#[derive(serde::Serialize, Default)]
pub struct CmafEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}

#[derive(serde::Serialize, Default)]
pub struct StreamSelection {
    #[serde(rename = "StreamOrder")]
    pub stream_order: Option<String>,
    #[serde(rename = "MaxVideoBitsPerSecond")]
    pub max_video_bits_per_second: Option<usize>,
    #[serde(rename = "MinVideoBitsPerSecond")]
    pub min_video_bits_per_second: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct MssEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct DashManifest {
    #[serde(rename = "ManifestLayout")]
    pub manifest_layout: Option<String>,
    #[serde(rename = "MinBufferTimeSeconds")]
    pub min_buffer_time_seconds: Option<usize>,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
    #[serde(rename = "ManifestName")]
    pub manifest_name: Option<ManifestName>,
    #[serde(rename = "ScteMarkersSource")]
    pub scte_markers_source: Option<String>,
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,

}

#[derive(serde::Serialize, Default)]
pub struct MssPackage {
    #[serde(rename = "MssManifests")]
    pub mss_manifests: Vec<MssManifest>,
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<SegmentDurationSeconds>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<MssEncryption>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsManifest {
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    pub program_date_time_interval_seconds: Option<usize>,
    #[serde(rename = "ManifestName")]
    pub manifest_name: Option<ManifestName>,
    #[serde(rename = "AdMarkers")]
    pub ad_markers: Option<String>,
    #[serde(rename = "RepeatExtXKey")]
    pub repeat_ext_xkey: Option<bool>,
    #[serde(rename = "IncludeIframeOnlyStream")]
    pub include_iframe_only_stream: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct DashPackage {
    #[serde(rename = "SegmentTemplateFormat")]
    pub segment_template_format: Option<String>,
    #[serde(rename = "IncludeEncoderConfigurationInSegments")]
    pub include_encoder_configuration_in_segments: Option<bool>,
    #[serde(rename = "IncludeIframeOnlyStream")]
    pub include_iframe_only_stream: Option<bool>,
    #[serde(rename = "DashManifests")]
    pub dash_manifests: Vec<DashManifest>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<DashEncryption>,
    #[serde(rename = "PeriodTriggers")]
    pub period_triggers: Option<Vec<String>>,
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<SegmentDurationSeconds>,

}
pub type RoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct HlsEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<String>,
    #[serde(rename = "ConstantInitializationVector")]
    pub constant_initialization_vector: Option<String>,

}


}

pub mod cfn_packaging_group {

#[derive(serde::Serialize, Default)]
pub struct CfnPackagingGroup {
    /// CDN Authorization
    #[serde(rename = "Authorization")]
    pub authorization: Option<Authorization>,
    /// The ID of the PackagingGroup.
    #[serde(rename = "Id")]
    pub id: String,
    /// A collection of tags associated with a resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The configuration parameters for egress access logging.
    #[serde(rename = "EgressAccessLogs")]
    pub egress_access_logs: Option<LogConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct Authorization {
    #[serde(rename = "CdnIdentifierSecret")]
    pub cdn_identifier_secret: String,
    #[serde(rename = "SecretsRoleArn")]
    pub secrets_role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct LogConfiguration {
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,

}


}
