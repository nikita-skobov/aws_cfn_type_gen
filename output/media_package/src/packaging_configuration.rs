

/// Creates a packaging configuration in a packaging group.
///
/// The packaging configuration represents a single delivery point for an asset. It determines the format and setting for the egressing content. Specify only one package format per configuration, such as HlsPackage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPackagingConfiguration {


    /// 
    /// Parameters for Microsoft Smooth Streaming packaging.
    /// 
    /// Required: No
    ///
    /// Type: MssPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "MssPackage")]
    pub mss_package: Option<MssPackage>,


    /// 
    /// Parameters for CMAF packaging.
    /// 
    /// Required: No
    ///
    /// Type: CmafPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "CmafPackage")]
    pub cmaf_package: Option<CmafPackage>,


    /// 
    /// The tags to assign to the packaging configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Unique identifier that you assign to the packaging configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// Parameters for Apple HLS packaging.
    /// 
    /// Required: No
    ///
    /// Type: HlsPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsPackage")]
    pub hls_package: Option<HlsPackage>,


    /// 
    /// Parameters for DASH-ISO packaging.
    /// 
    /// Required: No
    ///
    /// Type: DashPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashPackage")]
    pub dash_package: Option<DashPackage>,


    /// 
    /// The ID of the packaging group associated with this packaging configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PackagingGroupId")]
    pub packaging_group_id: String,

}

impl cfn_resources::CfnResource for CfnPackagingConfiguration {
    fn type_string() -> &'static str {
        "AWS::MediaPackage::PackagingConfiguration"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Parameters for a packaging configuration that uses Microsoft Smooth Streaming (MSS) packaging.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MssPackage {


    /// 
    /// A list of Microsoft Smooth manifest configurations that are available from this endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: List of MssManifest
    ///
    /// Update requires: No interruption
    #[serde(rename = "MssManifests")]
    pub mss_manifests: Vec<MssManifest>,


    /// 
    /// Duration (in seconds) of each fragment. Actual fragments are rounded to the nearest multiple of the source fragment duration.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<i64>,


    /// 
    /// Parameters for encrypting content.
    /// 
    /// Required: No
    ///
    /// Type: MssEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    pub encryption: Option<MssEncryption>,

}


/// A configuration for accessing an external Secure Packager and Encoder Key Exchange     (SPEKE) service that provides encryption keys.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SpekeKeyProvider {


    /// 
    /// The ARN for the IAM role that's granted by the key provider to provide access     to the key provider API. Valid format: arn:aws:iam::{accountID}:role/{name}
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// Use encryptionContractConfiguration to configure one or more content encryption keys for your         endpoints that use SPEKE Version 2.0. The encryption contract defines which content keys are used to encrypt the         audio and video tracks in your stream. To configure the encryption contract, specify which audio and video encryption presets to use.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionContractConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionContractConfiguration")]
    pub encryption_contract_configuration: Option<EncryptionContractConfiguration>,


    /// 
    /// URL for the key provider's key retrieval API endpoint. Must start with https://.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: String,


    /// 
    /// List of unique identifiers for the DRM systems to use, as defined in the CPIX specification.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SystemIds")]
    pub system_ids: Vec<String>,

}


/// Parameters for a packaging configuration that uses HTTP Live Streaming (HLS) packaging.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsPackage {


    /// 
    /// Parameters for encrypting content.
    /// 
    /// Required: No
    ///
    /// Type: HlsEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    pub encryption: Option<HlsEncryption>,


    /// 
    /// When true, AWS Elemental MediaPackage bundles all audio tracks in a rendition group. All other tracks in the stream can be used with any audio rendition from the group.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseAudioRenditionGroup")]
    pub use_audio_rendition_group: Option<bool>,


    /// 
    /// When enabled, MediaPackage passes through digital video broadcasting (DVB) subtitles into the output.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeDvbSubtitles")]
    pub include_dvb_subtitles: Option<bool>,


    /// 
    /// A list of HLS manifest configurations that are available from this endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: List of HlsManifest
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsManifests")]
    pub hls_manifests: Vec<HlsManifest>,


    /// 
    /// Duration (in seconds) of each fragment. Actual fragments are rounded to the nearest multiple of the source fragment duration.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<i64>,

}


/// Parameters for an HLS manifest.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsManifest {


    /// 
    /// Repeat the EXT-X-KEY directive for every media segment. This might result in an increase in client requests to the DRM server.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepeatExtXKey")]
    pub repeat_ext_xkey: Option<bool>,


    /// 
    /// This setting controls ad markers in the packaged content.
    /// 
    /// Valid values:
    /// 
    /// NONE - Omits all SCTE-35 ad markers from the output.                  PASSTHROUGH - Creates a copy in the output of the SCTE-35 ad markers (comments) taken directly from the input manifest.                  SCTE35_ENHANCED - Generates ad markers and blackout tags in the output based on the SCTE-35 messages from the input manifest.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdMarkers")]
    pub ad_markers: Option<String>,


    /// 
    /// Video bitrate limitations for outputs from this packaging configuration.
    /// 
    /// Required: No
    ///
    /// Type: StreamSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,


    /// 
    /// A short string that's appended to the end of the endpoint URL to create a unique path to this packaging configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestName")]
    pub manifest_name: Option<String>,


    /// 
    /// Inserts EXT-X-PROGRAM-DATE-TIME tags in the output manifest at the interval that you specify. Additionally, ID3Timed metadata messages are generated every 5     seconds starting when the content was ingested.
    ///
    /// Irrespective of this parameter, if any ID3Timed metadata is in the HLS input, it is passed through to the HLS output.
    ///
    /// Omit this attribute or enter 0 to indicate that the       EXT-X-PROGRAM-DATE-TIME tags are not included in the manifest.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    pub program_date_time_interval_seconds: Option<i64>,


    /// 
    /// Applies to stream sets with a single video track only. When enabled, the output includes an additional I-frame only stream, along with the other tracks.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeIframeOnlyStream")]
    pub include_iframe_only_stream: Option<bool>,

}


/// Parameters for a DASH manifest.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DashManifest {


    /// 
    /// Determines the position of some tags in the Media Presentation Description (MPD). When set to FULL, elements like SegmentTemplate and       ContentProtection are included in each Representation. When set to COMPACT, duplicate elements are combined and presented at the     AdaptationSet level.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestLayout")]
    pub manifest_layout: Option<String>,


    /// 
    /// Minimum amount of content (measured in seconds) that a player must keep available in the buffer.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinBufferTimeSeconds")]
    pub min_buffer_time_seconds: Option<i64>,


    /// 
    /// A short string that's appended to the end of the endpoint URL to create a unique path to this packaging configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestName")]
    pub manifest_name: Option<String>,


    /// 
    /// The source of scte markers used.
    /// 
    /// Value description:
    /// 
    /// SEGMENTS - The scte markers are sourced from the segments of the ingested content.                                      MANIFEST - the scte markers are sourced from the manifest of the ingested content.            The MANIFEST value is compatible with source HLS playlists using the SCTE-35 Enhanced syntax (EXT-OATCLS-SCTE35 tags).            SCTE-35 Elemental and SCTE-35 Daterange syntaxes are not supported with this option.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScteMarkersSource")]
    pub scte_markers_source: Option<String>,


    /// 
    /// Limitations for outputs from the endpoint, based on the video bitrate.
    /// 
    /// Required: No
    ///
    /// Type: StreamSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,


    /// 
    /// The DASH profile type. When set to HBBTV_1_5, the content is compliant with HbbTV 1.5.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Profile")]
    pub profile: Option<String>,

}


/// Limitations for outputs from the endpoint, based on the video bitrate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StreamSelection {


    /// 
    /// The upper limit of the bitrates that this endpoint serves. If the video track exceeds this threshold, then AWS Elemental MediaPackage excludes it from output. If you don't specify a value, it defaults to 2147483647 bits per second.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxVideoBitsPerSecond")]
    pub max_video_bits_per_second: Option<i64>,


    /// 
    /// The lower limit of the bitrates that this endpoint serves. If the video track is below this threshold, then AWS Elemental MediaPackage excludes it from output. If you don't specify a value, it defaults to 0 bits per second.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinVideoBitsPerSecond")]
    pub min_video_bits_per_second: Option<i64>,


    /// 
    /// Order in which the different video bitrates are presented to the player.
    /// 
    /// Valid values: ORIGINAL, VIDEO_BITRATE_ASCENDING, VIDEO_BITRATE_DESCENDING.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamOrder")]
    pub stream_order: Option<String>,

}


/// Holds encryption information so that access to the content can be controlled by a DRM solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CmafEncryption {


    /// 
    /// Parameters for the SPEKE key provider.
    /// 
    /// Required: Yes
    ///
    /// Type: SpekeKeyProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}


/// Parameters for a packaging configuration that uses Dynamic Adaptive Streaming over HTTP (DASH) packaging.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DashPackage {


    /// 
    /// A list of DASH manifest configurations that are available from this endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DashManifest
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashManifests")]
    pub dash_manifests: Vec<DashManifest>,


    /// 
    /// This applies only to stream sets with a single video track. When true, the stream set includes an         additional I-frame trick-play only stream, along with the other tracks. If false, this extra stream is not included.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeIframeOnlyStream")]
    pub include_iframe_only_stream: Option<bool>,


    /// 
    /// Controls whether AWS Elemental MediaPackage produces single-period or multi-period DASH manifests. For more information about periods, see Multi-period DASH in AWS Elemental MediaPackage.
    /// 
    /// Valid values:
    /// 
    /// ADS - AWS Elemental MediaPackage will produce multi-period DASH manifests. Periods are created based on the SCTE-35 ad markers present in the input manifest.                  No value - AWS Elemental MediaPackage will produce single-period DASH manifests. This is the default setting.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodTriggers")]
    pub period_triggers: Option<Vec<String>>,


    /// 
    /// Duration (in seconds) of each fragment. Actual fragments are rounded to the nearest multiple of the source segment duration.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<i64>,


    /// 
    /// Parameters for encrypting content.
    /// 
    /// Required: No
    ///
    /// Type: DashEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    pub encryption: Option<DashEncryption>,


    /// 
    /// Determines the type of SegmentTemplate included in the Media Presentation Description (MPD). When set to NUMBER_WITH_TIMELINE, a full timeline is     presented in each SegmentTemplate, with $Number$ media URLs. When set to TIME_WITH_TIMELINE, a full timeline is presented in each     SegmentTemplate, with $Time$ media URLs. When set to NUMBER_WITH_DURATION, only a duration is included in each     SegmentTemplate, with $Number$ media URLs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentTemplateFormat")]
    pub segment_template_format: Option<String>,


    /// 
    /// When includeEncoderConfigurationInSegments is set to true, AWS Elemental MediaPackage places your encoder's Sequence Parameter Set (SPS), Picture Parameter Set (PPS), and Video Parameter Set (VPS) metadata in every video segment instead of in the init fragment. This lets you use different SPS/PPS/VPS settings for your assets during content playback.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeEncoderConfigurationInSegments")]
    pub include_encoder_configuration_in_segments: Option<bool>,

}


/// Holds encryption information so that access to the content can be controlled by a DRM solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DashEncryption {


    /// 
    /// Parameters for the SPEKE key provider.
    /// 
    /// Required: Yes
    ///
    /// Type: SpekeKeyProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}


/// Parameters for a packaging configuration that uses Common Media Application Format (CMAF) packaging.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CmafPackage {


    /// 
    /// Parameters for encrypting content.
    /// 
    /// Required: No
    ///
    /// Type: CmafEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    pub encryption: Option<CmafEncryption>,


    /// 
    /// Duration (in seconds) of each segment. Actual segments are rounded to the nearest multiple of the source fragment duration.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentDurationSeconds")]
    pub segment_duration_seconds: Option<i64>,


    /// 
    /// When includeEncoderConfigurationInSegments is set to true, AWS Elemental MediaPackage places your encoder's Sequence Parameter Set (SPS), Picture Parameter Set (PPS), and Video Parameter Set (VPS) metadata in every video segment instead of in the init fragment. This lets you use different SPS/PPS/VPS settings for your assets during content playback.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeEncoderConfigurationInSegments")]
    pub include_encoder_configuration_in_segments: Option<bool>,


    /// 
    /// A list of HLS manifest configurations that are available from this endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: List of HlsManifest
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsManifests")]
    pub hls_manifests: Vec<HlsManifest>,

}


/// Holds encryption information so that access to the content can be controlled by a DRM solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MssEncryption {


    /// 
    /// Parameters for the SPEKE key provider.
    /// 
    /// Required: Yes
    ///
    /// Type: SpekeKeyProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// Use encryptionContractConfiguration to configure one or more content        encryption keys for your endpoints that use SPEKE Version 2.0. The encryption contract        defines the content keys used to encrypt the audio and video tracks in your stream.        To configure the encryption contract, specify which audio and video encryption        presets to use. For more information about these presets, see SPEKE Version 2.0 Presets.
///
/// Note the following considerations when using        encryptionContractConfiguration:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionContractConfiguration {

}


/// Holds encryption information so that access to the content can be controlled by a DRM solution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsEncryption {


    /// 
    /// A 128-bit, 16-byte hex value represented by a 32-character string, used with the key for encrypting blocks. If you don't specify a constant initialization vector (IV),       AWS Elemental MediaPackage periodically rotates the IV.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantInitializationVector")]
    pub constant_initialization_vector: Option<String>,


    /// 
    /// Parameters for the SPEKE key provider.
    /// 
    /// Required: Yes
    ///
    /// Type: SpekeKeyProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,


    /// 
    /// HLS encryption type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<String>,

}


/// Parameters for a Microsoft Smooth manifest.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MssManifest {


    /// 
    /// A short string that's appended to the end of the endpoint URL to create a unique path to this packaging configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestName")]
    pub manifest_name: Option<String>,


    /// 
    /// Video bitrate limitations for outputs from this packaging configuration.
    /// 
    /// Required: No
    ///
    /// Type: StreamSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSelection")]
    pub stream_selection: Option<StreamSelection>,

}
