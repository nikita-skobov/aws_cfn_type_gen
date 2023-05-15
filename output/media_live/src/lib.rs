
pub mod cfn_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnChannel {
    /// No documentation provided by AWS
    #[serde(rename = "InputSpecification")]
    pub input_specification: Option<InputSpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "ChannelClass")]
    pub channel_class: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Vpc")]
    pub vpc: Option<VpcOutputSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "EncoderSettings")]
    pub encoder_settings: Option<EncoderSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "LogLevel")]
    pub log_level: Option<String>,
    /// List of InputAttachment
    #[serde(rename = "InputAttachments")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// No documentation provided by AWS
    #[serde(rename = "Maintenance")]
    pub maintenance: Option<MaintenanceCreateSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// List of OutputDestination
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<OutputDestination>>,
    /// No documentation provided by AWS
    #[serde(rename = "CdiInputSpecification")]
    pub cdi_input_specification: Option<CdiInputSpecification>,

}


#[derive(serde::Serialize, Default)]
pub struct EncoderSettings {
    #[serde(rename = "GlobalConfiguration")]
    pub global_configuration: Option<GlobalConfiguration>,
    #[serde(rename = "AudioDescriptions")]
    pub audio_descriptions: Option<Vec<AudioDescription>>,
    #[serde(rename = "AvailConfiguration")]
    pub avail_configuration: Option<AvailConfiguration>,
    #[serde(rename = "CaptionDescriptions")]
    pub caption_descriptions: Option<Vec<CaptionDescription>>,
    #[serde(rename = "VideoDescriptions")]
    pub video_descriptions: Option<Vec<VideoDescription>>,
    #[serde(rename = "NielsenConfiguration")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    #[serde(rename = "MotionGraphicsConfiguration")]
    pub motion_graphics_configuration: Option<MotionGraphicsConfiguration>,
    #[serde(rename = "AvailBlanking")]
    pub avail_blanking: Option<AvailBlanking>,
    #[serde(rename = "BlackoutSlate")]
    pub blackout_slate: Option<BlackoutSlate>,
    #[serde(rename = "OutputGroups")]
    pub output_groups: Option<Vec<OutputGroup>>,
    #[serde(rename = "FeatureActivations")]
    pub feature_activations: Option<FeatureActivations>,
    #[serde(rename = "TimecodeConfig")]
    pub timecode_config: Option<TimecodeConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsInputSettings {
    #[serde(rename = "BufferSegments")]
    pub buffer_segments: Option<usize>,
    #[serde(rename = "RetryInterval")]
    pub retry_interval: Option<usize>,
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Option<usize>,
    #[serde(rename = "Retries")]
    pub retries: Option<usize>,
    #[serde(rename = "Scte35Source")]
    pub scte35_source: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TimecodeConfig {
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "SyncThreshold")]
    pub sync_threshold: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VideoDescription {
    #[serde(rename = "RespondToAfd")]
    pub respond_to_afd: Option<String>,
    #[serde(rename = "Height")]
    pub height: Option<usize>,
    #[serde(rename = "Width")]
    pub width: Option<usize>,
    #[serde(rename = "CodecSettings")]
    pub codec_settings: Option<VideoCodecSettings>,
    #[serde(rename = "ScalingBehavior")]
    pub scaling_behavior: Option<String>,
    #[serde(rename = "Sharpness")]
    pub sharpness: Option<usize>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BurnInDestinationSettings {
    #[serde(rename = "ShadowYOffset")]
    pub shadow_yoffset: Option<usize>,
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "TeletextGridControl")]
    pub teletext_grid_control: Option<String>,
    #[serde(rename = "BackgroundOpacity")]
    pub background_opacity: Option<usize>,
    #[serde(rename = "FontOpacity")]
    pub font_opacity: Option<usize>,
    #[serde(rename = "FontResolution")]
    pub font_resolution: Option<usize>,
    #[serde(rename = "Alignment")]
    pub alignment: Option<String>,
    #[serde(rename = "OutlineColor")]
    pub outline_color: Option<String>,
    #[serde(rename = "Font")]
    pub font: Option<InputLocation>,
    #[serde(rename = "ShadowXOffset")]
    pub shadow_xoffset: Option<usize>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "ShadowColor")]
    pub shadow_color: Option<String>,
    #[serde(rename = "FontSize")]
    pub font_size: Option<String>,
    #[serde(rename = "YPosition")]
    pub yposition: Option<usize>,
    #[serde(rename = "OutlineSize")]
    pub outline_size: Option<usize>,
    #[serde(rename = "ShadowOpacity")]
    pub shadow_opacity: Option<usize>,
    #[serde(rename = "XPosition")]
    pub xposition: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct BlackoutSlate {
    #[serde(rename = "NetworkId")]
    pub network_id: Option<String>,
    #[serde(rename = "NetworkEndBlackout")]
    pub network_end_blackout: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "BlackoutSlateImage")]
    pub blackout_slate_image: Option<InputLocation>,
    #[serde(rename = "NetworkEndBlackoutImage")]
    pub network_end_blackout_image: Option<InputLocation>,

}

#[derive(serde::Serialize, Default)]
pub struct M3u8Settings {
    #[serde(rename = "TimedMetadataPid")]
    pub timed_metadata_pid: Option<String>,
    #[serde(rename = "NielsenId3Behavior")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "PmtInterval")]
    pub pmt_interval: Option<usize>,
    #[serde(rename = "AudioFramesPerPes")]
    pub audio_frames_per_pes: Option<usize>,
    #[serde(rename = "TransportStreamId")]
    pub transport_stream_id: Option<usize>,
    #[serde(rename = "PcrPid")]
    pub pcr_pid: Option<String>,
    #[serde(rename = "PatInterval")]
    pub pat_interval: Option<usize>,
    #[serde(rename = "PcrControl")]
    pub pcr_control: Option<String>,
    #[serde(rename = "EcmPid")]
    pub ecm_pid: Option<String>,
    #[serde(rename = "Scte35Pid")]
    pub scte35_pid: Option<String>,
    #[serde(rename = "TimedMetadataBehavior")]
    pub timed_metadata_behavior: Option<String>,
    #[serde(rename = "Scte35Behavior")]
    pub scte35_behavior: Option<String>,
    #[serde(rename = "PcrPeriod")]
    pub pcr_period: Option<usize>,
    #[serde(rename = "AudioPids")]
    pub audio_pids: Option<String>,
    #[serde(rename = "PmtPid")]
    pub pmt_pid: Option<String>,
    #[serde(rename = "VideoPid")]
    pub video_pid: Option<String>,
    #[serde(rename = "ProgramNum")]
    pub program_num: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DvbSubSourceSettings {
    #[serde(rename = "OcrLanguage")]
    pub ocr_language: Option<String>,
    #[serde(rename = "Pid")]
    pub pid: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Mpeg2FilterSettings {
    #[serde(rename = "TemporalFilterSettings")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct VideoSelectorSettings {
    #[serde(rename = "VideoSelectorProgramId")]
    pub video_selector_program_id: Option<VideoSelectorProgramId>,
    #[serde(rename = "VideoSelectorPid")]
    pub video_selector_pid: Option<VideoSelectorPid>,

}

#[derive(serde::Serialize, Default)]
pub struct FailoverConditionSettings {
    #[serde(rename = "VideoBlackSettings")]
    pub video_black_settings: Option<VideoBlackFailoverSettings>,
    #[serde(rename = "AudioSilenceSettings")]
    pub audio_silence_settings: Option<AudioSilenceFailoverSettings>,
    #[serde(rename = "InputLossSettings")]
    pub input_loss_settings: Option<InputLossFailoverSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct Fmp4HlsSettings {
    #[serde(rename = "NielsenId3Behavior")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "TimedMetadataBehavior")]
    pub timed_metadata_behavior: Option<String>,
    #[serde(rename = "AudioRenditionSets")]
    pub audio_rendition_sets: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputChannelLevel {
    #[serde(rename = "InputChannel")]
    pub input_channel: Option<usize>,
    #[serde(rename = "Gain")]
    pub gain: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SmpteTtDestinationSettings {

}

#[derive(serde::Serialize, Default)]
pub struct Mpeg2Settings {
    #[serde(rename = "TimecodeBurninSettings")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
    #[serde(rename = "ScanType")]
    pub scan_type: Option<String>,
    #[serde(rename = "FilterSettings")]
    pub filter_settings: Option<Mpeg2FilterSettings>,
    #[serde(rename = "FixedAfd")]
    pub fixed_afd: Option<String>,
    #[serde(rename = "DisplayAspectRatio")]
    pub display_aspect_ratio: Option<String>,
    #[serde(rename = "FramerateDenominator")]
    pub framerate_denominator: Option<usize>,
    #[serde(rename = "GopClosedCadence")]
    pub gop_closed_cadence: Option<usize>,
    #[serde(rename = "FramerateNumerator")]
    pub framerate_numerator: Option<usize>,
    #[serde(rename = "ColorMetadata")]
    pub color_metadata: Option<String>,
    #[serde(rename = "SubgopLength")]
    pub subgop_length: Option<String>,
    #[serde(rename = "AdaptiveQuantization")]
    pub adaptive_quantization: Option<String>,
    #[serde(rename = "GopSize")]
    pub gop_size: Option<f64>,
    #[serde(rename = "GopSizeUnits")]
    pub gop_size_units: Option<String>,
    #[serde(rename = "AfdSignaling")]
    pub afd_signaling: Option<String>,
    #[serde(rename = "TimecodeInsertion")]
    pub timecode_insertion: Option<String>,
    #[serde(rename = "GopNumBFrames")]
    pub gop_num_bframes: Option<usize>,
    #[serde(rename = "ColorSpace")]
    pub color_space: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VideoSelectorColorSpaceSettings {
    #[serde(rename = "Hdr10Settings")]
    pub hdr10_settings: Option<Hdr10Settings>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsCdnSettings {
    #[serde(rename = "HlsWebdavSettings")]
    pub hls_webdav_settings: Option<HlsWebdavSettings>,
    #[serde(rename = "HlsBasicPutSettings")]
    pub hls_basic_put_settings: Option<HlsBasicPutSettings>,
    #[serde(rename = "HlsMediaStoreSettings")]
    pub hls_media_store_settings: Option<HlsMediaStoreSettings>,
    #[serde(rename = "HlsAkamaiSettings")]
    pub hls_akamai_settings: Option<HlsAkamaiSettings>,
    #[serde(rename = "HlsS3Settings")]
    pub hls_s3_settings: Option<HlsS3Settings>,

}

#[derive(serde::Serialize, Default)]
pub struct MsSmoothOutputSettings {
    #[serde(rename = "H265PackagingType")]
    pub h265_packaging_type: Option<String>,
    #[serde(rename = "NameModifier")]
    pub name_modifier: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PassThroughSettings {

}

#[derive(serde::Serialize, Default)]
pub struct MediaPackageOutputDestinationSettings {
    #[serde(rename = "ChannelId")]
    pub channel_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DvbNitSettings {
    #[serde(rename = "NetworkId")]
    pub network_id: Option<usize>,
    #[serde(rename = "NetworkName")]
    pub network_name: Option<String>,
    #[serde(rename = "RepInterval")]
    pub rep_interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RemixSettings {
    #[serde(rename = "ChannelsOut")]
    pub channels_out: Option<usize>,
    #[serde(rename = "ChannelsIn")]
    pub channels_in: Option<usize>,
    #[serde(rename = "ChannelMappings")]
    pub channel_mappings: Option<Vec<AudioChannelMapping>>,

}

#[derive(serde::Serialize, Default)]
pub struct MediaPackageGroupSettings {
    #[serde(rename = "Destination")]
    pub destination: Option<OutputLocationRef>,

}

#[derive(serde::Serialize, Default)]
pub struct H264FilterSettings {
    #[serde(rename = "TemporalFilterSettings")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct VideoSelectorPid {
    #[serde(rename = "Pid")]
    pub pid: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioTrack {
    #[serde(rename = "Track")]
    pub track: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HtmlMotionGraphicsSettings {

}

#[derive(serde::Serialize, Default)]
pub struct AribDestinationSettings {

}

#[derive(serde::Serialize, Default)]
pub struct AudioLanguageSelection {
    #[serde(rename = "LanguageCode")]
    pub language_code: Option<String>,
    #[serde(rename = "LanguageSelectionPolicy")]
    pub language_selection_policy: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EmbeddedSourceSettings {
    #[serde(rename = "Source608TrackNumber")]
    pub source608_track_number: Option<usize>,
    #[serde(rename = "Convert608To708")]
    pub convert608_to708: Option<String>,
    #[serde(rename = "Source608ChannelNumber")]
    pub source608_channel_number: Option<usize>,
    #[serde(rename = "Scte20Detection")]
    pub scte20_detection: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioDolbyEDecode {
    #[serde(rename = "ProgramSelection")]
    pub program_selection: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NielsenConfiguration {
    #[serde(rename = "NielsenPcmToId3Tagging")]
    pub nielsen_pcm_to_id3_tagging: Option<String>,
    #[serde(rename = "DistributorId")]
    pub distributor_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NielsenWatermarksSettings {
    #[serde(rename = "NielsenCbetSettings")]
    pub nielsen_cbet_settings: Option<NielsenCBET>,
    #[serde(rename = "NielsenNaesIiNwSettings")]
    pub nielsen_naes_ii_nw_settings: Option<NielsenNaesIiNw>,
    #[serde(rename = "NielsenDistributionType")]
    pub nielsen_distribution_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OutputLocationRef {
    #[serde(rename = "DestinationRefId")]
    pub destination_ref_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsWebdavSettings {
    #[serde(rename = "NumRetries")]
    pub num_retries: Option<usize>,
    #[serde(rename = "FilecacheDuration")]
    pub filecache_duration: Option<usize>,
    #[serde(rename = "ConnectionRetryInterval")]
    pub connection_retry_interval: Option<usize>,
    #[serde(rename = "HttpTransferMode")]
    pub http_transfer_mode: Option<String>,
    #[serde(rename = "RestartDelay")]
    pub restart_delay: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct GlobalConfiguration {
    #[serde(rename = "InputEndAction")]
    pub input_end_action: Option<String>,
    #[serde(rename = "OutputLockingMode")]
    pub output_locking_mode: Option<String>,
    #[serde(rename = "InputLossBehavior")]
    pub input_loss_behavior: Option<InputLossBehavior>,
    #[serde(rename = "OutputTimingSource")]
    pub output_timing_source: Option<String>,
    #[serde(rename = "InitialAudioGain")]
    pub initial_audio_gain: Option<usize>,
    #[serde(rename = "SupportLowFramerateInputs")]
    pub support_low_framerate_inputs: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Rec601Settings {

}

#[derive(serde::Serialize, Default)]
pub struct AudioDescription {
    #[serde(rename = "AudioNormalizationSettings")]
    pub audio_normalization_settings: Option<AudioNormalizationSettings>,
    #[serde(rename = "AudioSelectorName")]
    pub audio_selector_name: Option<String>,
    #[serde(rename = "RemixSettings")]
    pub remix_settings: Option<RemixSettings>,
    #[serde(rename = "LanguageCodeControl")]
    pub language_code_control: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "LanguageCode")]
    pub language_code: Option<String>,
    #[serde(rename = "AudioWatermarkingSettings")]
    pub audio_watermarking_settings: Option<AudioWatermarkSettings>,
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "AudioTypeControl")]
    pub audio_type_control: Option<String>,
    #[serde(rename = "AudioType")]
    pub audio_type: Option<String>,
    #[serde(rename = "CodecSettings")]
    pub codec_settings: Option<AudioCodecSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkInputSettings {
    #[serde(rename = "HlsInputSettings")]
    pub hls_input_settings: Option<HlsInputSettings>,
    #[serde(rename = "ServerValidation")]
    pub server_validation: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AncillarySourceSettings {
    #[serde(rename = "SourceAncillaryChannelNumber")]
    pub source_ancillary_channel_number: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioSelectorSettings {
    #[serde(rename = "AudioPidSelection")]
    pub audio_pid_selection: Option<AudioPidSelection>,
    #[serde(rename = "AudioTrackSelection")]
    pub audio_track_selection: Option<AudioTrackSelection>,
    #[serde(rename = "AudioLanguageSelection")]
    pub audio_language_selection: Option<AudioLanguageSelection>,
    #[serde(rename = "AudioHlsRenditionSelection")]
    pub audio_hls_rendition_selection: Option<AudioHlsRenditionSelection>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcOutputSettings {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "PublicAddressAllocationIds")]
    pub public_address_allocation_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct TeletextDestinationSettings {

}

#[derive(serde::Serialize, Default)]
pub struct FrameCaptureOutputSettings {
    #[serde(rename = "NameModifier")]
    pub name_modifier: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ArchiveGroupSettings {
    #[serde(rename = "Destination")]
    pub destination: Option<OutputLocationRef>,
    #[serde(rename = "RolloverInterval")]
    pub rollover_interval: Option<usize>,
    #[serde(rename = "ArchiveCdnSettings")]
    pub archive_cdn_settings: Option<ArchiveCdnSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct AvailBlanking {
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "AvailBlankingImage")]
    pub avail_blanking_image: Option<InputLocation>,

}

#[derive(serde::Serialize, Default)]
pub struct H265ColorSpaceSettings {
    #[serde(rename = "Rec601Settings")]
    pub rec601_settings: Option<Rec601Settings>,
    #[serde(rename = "DolbyVision81Settings")]
    pub dolby_vision81_settings: Option<DolbyVision81Settings>,
    #[serde(rename = "Hdr10Settings")]
    pub hdr10_settings: Option<Hdr10Settings>,
    #[serde(rename = "ColorSpacePassthroughSettings")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,
    #[serde(rename = "Rec709Settings")]
    pub rec709_settings: Option<Rec709Settings>,

}

#[derive(serde::Serialize, Default)]
pub struct Eac3AtmosSettings {
    #[serde(rename = "DrcLine")]
    pub drc_line: Option<String>,
    #[serde(rename = "CodingMode")]
    pub coding_mode: Option<String>,
    #[serde(rename = "DrcRf")]
    pub drc_rf: Option<String>,
    #[serde(rename = "Dialnorm")]
    pub dialnorm: Option<usize>,
    #[serde(rename = "HeightTrim")]
    pub height_trim: Option<f64>,
    #[serde(rename = "SurroundTrim")]
    pub surround_trim: Option<f64>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct StandardHlsSettings {
    #[serde(rename = "M3u8Settings")]
    pub m3u8_settings: Option<M3u8Settings>,
    #[serde(rename = "AudioRenditionSets")]
    pub audio_rendition_sets: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ColorSpacePassthroughSettings {

}

#[derive(serde::Serialize, Default)]
pub struct VideoBlackFailoverSettings {
    #[serde(rename = "VideoBlackThresholdMsec")]
    pub video_black_threshold_msec: Option<usize>,
    #[serde(rename = "BlackDetectThreshold")]
    pub black_detect_threshold: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct KeyProviderSettings {
    #[serde(rename = "StaticKeySettings")]
    pub static_key_settings: Option<StaticKeySettings>,

}

#[derive(serde::Serialize, Default)]
pub struct FrameCaptureHlsSettings {

}

#[derive(serde::Serialize, Default)]
pub struct Scte27SourceSettings {
    #[serde(rename = "OcrLanguage")]
    pub ocr_language: Option<String>,
    #[serde(rename = "Pid")]
    pub pid: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct UdpContainerSettings {
    #[serde(rename = "M2tsSettings")]
    pub m2ts_settings: Option<M2tsSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct MotionGraphicsSettings {
    #[serde(rename = "HtmlMotionGraphicsSettings")]
    pub html_motion_graphics_settings: Option<HtmlMotionGraphicsSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct EmbeddedPlusScte20DestinationSettings {

}

#[derive(serde::Serialize, Default)]
pub struct MotionGraphicsConfiguration {
    #[serde(rename = "MotionGraphicsSettings")]
    pub motion_graphics_settings: Option<MotionGraphicsSettings>,
    #[serde(rename = "MotionGraphicsInsertion")]
    pub motion_graphics_insertion: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CaptionDescription {
    #[serde(rename = "Accessibility")]
    pub accessibility: Option<String>,
    #[serde(rename = "DestinationSettings")]
    pub destination_settings: Option<CaptionDestinationSettings>,
    #[serde(rename = "CaptionSelectorName")]
    pub caption_selector_name: Option<String>,
    #[serde(rename = "LanguageDescription")]
    pub language_description: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "LanguageCode")]
    pub language_code: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DolbyVision81Settings {

}

#[derive(serde::Serialize, Default)]
pub struct RtmpCaptionInfoDestinationSettings {

}

#[derive(serde::Serialize, Default)]
pub struct Scte35SpliceInsert {
    #[serde(rename = "NoRegionalBlackoutFlag")]
    pub no_regional_blackout_flag: Option<String>,
    #[serde(rename = "AdAvailOffset")]
    pub ad_avail_offset: Option<usize>,
    #[serde(rename = "WebDeliveryAllowedFlag")]
    pub web_delivery_allowed_flag: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputAttachment {
    #[serde(rename = "AutomaticInputFailoverSettings")]
    pub automatic_input_failover_settings: Option<AutomaticInputFailoverSettings>,
    #[serde(rename = "InputAttachmentName")]
    pub input_attachment_name: Option<String>,
    #[serde(rename = "InputId")]
    pub input_id: Option<String>,
    #[serde(rename = "InputSettings")]
    pub input_settings: Option<InputSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct ArchiveCdnSettings {
    #[serde(rename = "ArchiveS3Settings")]
    pub archive_s3_settings: Option<ArchiveS3Settings>,

}

#[derive(serde::Serialize, Default)]
pub struct OutputDestination {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "MultiplexSettings")]
    pub multiplex_settings: Option<MultiplexProgramChannelDestinationSettings>,
    #[serde(rename = "MediaPackageSettings")]
    pub media_package_settings: Option<Vec<MediaPackageOutputDestinationSettings>>,
    #[serde(rename = "Settings")]
    pub settings: Option<Vec<OutputDestinationSettings>>,

}

#[derive(serde::Serialize, Default)]
pub struct FeatureActivations {
    #[serde(rename = "InputPrepareScheduleActions")]
    pub input_prepare_schedule_actions: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VideoCodecSettings {
    #[serde(rename = "H264Settings")]
    pub h264_settings: Option<H264Settings>,
    #[serde(rename = "FrameCaptureSettings")]
    pub frame_capture_settings: Option<FrameCaptureSettings>,
    #[serde(rename = "Mpeg2Settings")]
    pub mpeg2_settings: Option<Mpeg2Settings>,
    #[serde(rename = "H265Settings")]
    pub h265_settings: Option<H265Settings>,

}

#[derive(serde::Serialize, Default)]
pub struct TemporalFilterSettings {
    #[serde(rename = "Strength")]
    pub strength: Option<String>,
    #[serde(rename = "PostFilterSharpening")]
    pub post_filter_sharpening: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Mp2Settings {
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<f64>,
    #[serde(rename = "SampleRate")]
    pub sample_rate: Option<f64>,
    #[serde(rename = "CodingMode")]
    pub coding_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OutputSettings {
    #[serde(rename = "UdpOutputSettings")]
    pub udp_output_settings: Option<UdpOutputSettings>,
    #[serde(rename = "ArchiveOutputSettings")]
    pub archive_output_settings: Option<ArchiveOutputSettings>,
    #[serde(rename = "MediaPackageOutputSettings")]
    pub media_package_output_settings: Option<MediaPackageOutputSettings>,
    #[serde(rename = "RtmpOutputSettings")]
    pub rtmp_output_settings: Option<RtmpOutputSettings>,
    #[serde(rename = "HlsOutputSettings")]
    pub hls_output_settings: Option<HlsOutputSettings>,
    #[serde(rename = "MsSmoothOutputSettings")]
    pub ms_smooth_output_settings: Option<MsSmoothOutputSettings>,
    #[serde(rename = "FrameCaptureOutputSettings")]
    pub frame_capture_output_settings: Option<FrameCaptureOutputSettings>,
    #[serde(rename = "MultiplexOutputSettings")]
    pub multiplex_output_settings: Option<MultiplexOutputSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct Esam {
    #[serde(rename = "AdAvailOffset")]
    pub ad_avail_offset: Option<usize>,
    #[serde(rename = "AcquisitionPointId")]
    pub acquisition_point_id: Option<String>,
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "PoisEndpoint")]
    pub pois_endpoint: Option<String>,
    #[serde(rename = "ZoneIdentity")]
    pub zone_identity: Option<String>,
    #[serde(rename = "PasswordParam")]
    pub password_param: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioSelector {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "SelectorSettings")]
    pub selector_settings: Option<AudioSelectorSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioOnlyHlsSettings {
    #[serde(rename = "AudioTrackType")]
    pub audio_track_type: Option<String>,
    #[serde(rename = "AudioGroupId")]
    pub audio_group_id: Option<String>,
    #[serde(rename = "AudioOnlyImage")]
    pub audio_only_image: Option<InputLocation>,
    #[serde(rename = "SegmentType")]
    pub segment_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CdiInputSpecification {
    #[serde(rename = "Resolution")]
    pub resolution: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EbuTtDDestinationSettings {
    #[serde(rename = "StyleControl")]
    pub style_control: Option<String>,
    #[serde(rename = "FillLineGap")]
    pub fill_line_gap: Option<String>,
    #[serde(rename = "FontFamily")]
    pub font_family: Option<String>,
    #[serde(rename = "CopyrightHolder")]
    pub copyright_holder: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OutputGroup {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Outputs")]
    pub outputs: Option<Vec<Output>>,
    #[serde(rename = "OutputGroupSettings")]
    pub output_group_settings: Option<OutputGroupSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct CaptionDestinationSettings {
    #[serde(rename = "EbuTtDDestinationSettings")]
    pub ebu_tt_ddestination_settings: Option<EbuTtDDestinationSettings>,
    #[serde(rename = "SmpteTtDestinationSettings")]
    pub smpte_tt_destination_settings: Option<SmpteTtDestinationSettings>,
    #[serde(rename = "EmbeddedPlusScte20DestinationSettings")]
    pub embedded_plus_scte20_destination_settings: Option<EmbeddedPlusScte20DestinationSettings>,
    #[serde(rename = "TtmlDestinationSettings")]
    pub ttml_destination_settings: Option<TtmlDestinationSettings>,
    #[serde(rename = "TeletextDestinationSettings")]
    pub teletext_destination_settings: Option<TeletextDestinationSettings>,
    #[serde(rename = "BurnInDestinationSettings")]
    pub burn_in_destination_settings: Option<BurnInDestinationSettings>,
    #[serde(rename = "WebvttDestinationSettings")]
    pub webvtt_destination_settings: Option<WebvttDestinationSettings>,
    #[serde(rename = "AribDestinationSettings")]
    pub arib_destination_settings: Option<AribDestinationSettings>,
    #[serde(rename = "RtmpCaptionInfoDestinationSettings")]
    pub rtmp_caption_info_destination_settings: Option<RtmpCaptionInfoDestinationSettings>,
    #[serde(rename = "Scte27DestinationSettings")]
    pub scte27_destination_settings: Option<Scte27DestinationSettings>,
    #[serde(rename = "DvbSubDestinationSettings")]
    pub dvb_sub_destination_settings: Option<DvbSubDestinationSettings>,
    #[serde(rename = "EmbeddedDestinationSettings")]
    pub embedded_destination_settings: Option<EmbeddedDestinationSettings>,
    #[serde(rename = "Scte20PlusEmbeddedDestinationSettings")]
    pub scte20_plus_embedded_destination_settings: Option<Scte20PlusEmbeddedDestinationSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct Scte27DestinationSettings {

}

#[derive(serde::Serialize, Default)]
pub struct FrameCaptureCdnSettings {
    #[serde(rename = "FrameCaptureS3Settings")]
    pub frame_capture_s3_settings: Option<FrameCaptureS3Settings>,

}

#[derive(serde::Serialize, Default)]
pub struct FrameCaptureGroupSettings {
    #[serde(rename = "FrameCaptureCdnSettings")]
    pub frame_capture_cdn_settings: Option<FrameCaptureCdnSettings>,
    #[serde(rename = "Destination")]
    pub destination: Option<OutputLocationRef>,

}

#[derive(serde::Serialize, Default)]
pub struct WebvttDestinationSettings {
    #[serde(rename = "StyleControl")]
    pub style_control: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Scte35TimeSignalApos {
    #[serde(rename = "AdAvailOffset")]
    pub ad_avail_offset: Option<usize>,
    #[serde(rename = "WebDeliveryAllowedFlag")]
    pub web_delivery_allowed_flag: Option<String>,
    #[serde(rename = "NoRegionalBlackoutFlag")]
    pub no_regional_blackout_flag: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ArchiveContainerSettings {
    #[serde(rename = "M2tsSettings")]
    pub m2ts_settings: Option<M2tsSettings>,
    #[serde(rename = "RawSettings")]
    pub raw_settings: Option<RawSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsS3Settings {
    #[serde(rename = "CannedAcl")]
    pub canned_acl: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NielsenNaesIiNw {
    #[serde(rename = "CheckDigitString")]
    pub check_digit_string: Option<String>,
    #[serde(rename = "Sid")]
    pub sid: Option<f64>,
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FailoverCondition {
    #[serde(rename = "FailoverConditionSettings")]
    pub failover_condition_settings: Option<FailoverConditionSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsBasicPutSettings {
    #[serde(rename = "ConnectionRetryInterval")]
    pub connection_retry_interval: Option<usize>,
    #[serde(rename = "FilecacheDuration")]
    pub filecache_duration: Option<usize>,
    #[serde(rename = "RestartDelay")]
    pub restart_delay: Option<usize>,
    #[serde(rename = "NumRetries")]
    pub num_retries: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Hdr10Settings {
    #[serde(rename = "MaxCll")]
    pub max_cll: Option<usize>,
    #[serde(rename = "MaxFall")]
    pub max_fall: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct H264Settings {
    #[serde(rename = "Level")]
    pub level: Option<String>,
    #[serde(rename = "MaxBitrate")]
    pub max_bitrate: Option<usize>,
    #[serde(rename = "SubgopLength")]
    pub subgop_length: Option<String>,
    #[serde(rename = "TimecodeInsertion")]
    pub timecode_insertion: Option<String>,
    #[serde(rename = "ForceFieldPictures")]
    pub force_field_pictures: Option<String>,
    #[serde(rename = "FramerateNumerator")]
    pub framerate_numerator: Option<usize>,
    #[serde(rename = "FilterSettings")]
    pub filter_settings: Option<H264FilterSettings>,
    #[serde(rename = "TimecodeBurninSettings")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
    #[serde(rename = "GopClosedCadence")]
    pub gop_closed_cadence: Option<usize>,
    #[serde(rename = "FlickerAq")]
    pub flicker_aq: Option<String>,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
    #[serde(rename = "EntropyEncoding")]
    pub entropy_encoding: Option<String>,
    #[serde(rename = "AdaptiveQuantization")]
    pub adaptive_quantization: Option<String>,
    #[serde(rename = "RateControlMode")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "ColorMetadata")]
    pub color_metadata: Option<String>,
    #[serde(rename = "ColorSpaceSettings")]
    pub color_space_settings: Option<H264ColorSpaceSettings>,
    #[serde(rename = "LookAheadRateControl")]
    pub look_ahead_rate_control: Option<String>,
    #[serde(rename = "FixedAfd")]
    pub fixed_afd: Option<String>,
    #[serde(rename = "Slices")]
    pub slices: Option<usize>,
    #[serde(rename = "ParNumerator")]
    pub par_numerator: Option<usize>,
    #[serde(rename = "ScanType")]
    pub scan_type: Option<String>,
    #[serde(rename = "QvbrQualityLevel")]
    pub qvbr_quality_level: Option<usize>,
    #[serde(rename = "FramerateControl")]
    pub framerate_control: Option<String>,
    #[serde(rename = "TemporalAq")]
    pub temporal_aq: Option<String>,
    #[serde(rename = "Softness")]
    pub softness: Option<usize>,
    #[serde(rename = "Syntax")]
    pub syntax: Option<String>,
    #[serde(rename = "GopNumBFrames")]
    pub gop_num_bframes: Option<usize>,
    #[serde(rename = "ParDenominator")]
    pub par_denominator: Option<usize>,
    #[serde(rename = "GopBReference")]
    pub gop_breference: Option<String>,
    #[serde(rename = "ParControl")]
    pub par_control: Option<String>,
    #[serde(rename = "FramerateDenominator")]
    pub framerate_denominator: Option<usize>,
    #[serde(rename = "GopSize")]
    pub gop_size: Option<f64>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<usize>,
    #[serde(rename = "AfdSignaling")]
    pub afd_signaling: Option<String>,
    #[serde(rename = "SpatialAq")]
    pub spatial_aq: Option<String>,
    #[serde(rename = "NumRefFrames")]
    pub num_ref_frames: Option<usize>,
    #[serde(rename = "QualityLevel")]
    pub quality_level: Option<String>,
    #[serde(rename = "MinIInterval")]
    pub min_iinterval: Option<usize>,
    #[serde(rename = "BufFillPct")]
    pub buf_fill_pct: Option<usize>,
    #[serde(rename = "SceneChangeDetect")]
    pub scene_change_detect: Option<String>,
    #[serde(rename = "BufSize")]
    pub buf_size: Option<usize>,
    #[serde(rename = "GopSizeUnits")]
    pub gop_size_units: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CaptionSelectorSettings {
    #[serde(rename = "DvbSubSourceSettings")]
    pub dvb_sub_source_settings: Option<DvbSubSourceSettings>,
    #[serde(rename = "AribSourceSettings")]
    pub arib_source_settings: Option<AribSourceSettings>,
    #[serde(rename = "Scte20SourceSettings")]
    pub scte20_source_settings: Option<Scte20SourceSettings>,
    #[serde(rename = "Scte27SourceSettings")]
    pub scte27_source_settings: Option<Scte27SourceSettings>,
    #[serde(rename = "TeletextSourceSettings")]
    pub teletext_source_settings: Option<TeletextSourceSettings>,
    #[serde(rename = "EmbeddedSourceSettings")]
    pub embedded_source_settings: Option<EmbeddedSourceSettings>,
    #[serde(rename = "AncillarySourceSettings")]
    pub ancillary_source_settings: Option<AncillarySourceSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct TimecodeBurninSettings {
    #[serde(rename = "Position")]
    pub position: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "FontSize")]
    pub font_size: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MultiplexOutputSettings {
    #[serde(rename = "Destination")]
    pub destination: Option<OutputLocationRef>,

}

#[derive(serde::Serialize, Default)]
pub struct MaintenanceCreateSettings {
    #[serde(rename = "MaintenanceDay")]
    pub maintenance_day: Option<String>,
    #[serde(rename = "MaintenanceStartTime")]
    pub maintenance_start_time: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputLossFailoverSettings {
    #[serde(rename = "InputLossThresholdMsec")]
    pub input_loss_threshold_msec: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VideoSelector {
    #[serde(rename = "ColorSpaceSettings")]
    pub color_space_settings: Option<VideoSelectorColorSpaceSettings>,
    #[serde(rename = "SelectorSettings")]
    pub selector_settings: Option<VideoSelectorSettings>,
    #[serde(rename = "ColorSpaceUsage")]
    pub color_space_usage: Option<String>,
    #[serde(rename = "ColorSpace")]
    pub color_space: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AvailSettings {
    #[serde(rename = "Scte35SpliceInsert")]
    pub scte35_splice_insert: Option<Scte35SpliceInsert>,
    #[serde(rename = "Esam")]
    pub esam: Option<Esam>,
    #[serde(rename = "Scte35TimeSignalApos")]
    pub scte35_time_signal_apos: Option<Scte35TimeSignalApos>,

}

#[derive(serde::Serialize, Default)]
pub struct RawSettings {

}

#[derive(serde::Serialize, Default)]
pub struct AudioWatermarkSettings {
    #[serde(rename = "NielsenWatermarksSettings")]
    pub nielsen_watermarks_settings: Option<NielsenWatermarksSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct CaptionSelector {
    #[serde(rename = "LanguageCode")]
    pub language_code: Option<String>,
    #[serde(rename = "SelectorSettings")]
    pub selector_settings: Option<CaptionSelectorSettings>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioSilenceFailoverSettings {
    #[serde(rename = "AudioSilenceThresholdMsec")]
    pub audio_silence_threshold_msec: Option<usize>,
    #[serde(rename = "AudioSelectorName")]
    pub audio_selector_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DvbTdtSettings {
    #[serde(rename = "RepInterval")]
    pub rep_interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct TtmlDestinationSettings {
    #[serde(rename = "StyleControl")]
    pub style_control: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioHlsRenditionSelection {
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StaticKeySettings {
    #[serde(rename = "KeyProviderServer")]
    pub key_provider_server: Option<InputLocation>,
    #[serde(rename = "StaticKeyValue")]
    pub static_key_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioChannelMapping {
    #[serde(rename = "InputChannelLevels")]
    pub input_channel_levels: Option<Vec<InputChannelLevel>>,
    #[serde(rename = "OutputChannel")]
    pub output_channel: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Rec709Settings {

}

#[derive(serde::Serialize, Default)]
pub struct AacSettings {
    #[serde(rename = "SampleRate")]
    pub sample_rate: Option<f64>,
    #[serde(rename = "VbrQuality")]
    pub vbr_quality: Option<String>,
    #[serde(rename = "CodingMode")]
    pub coding_mode: Option<String>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<f64>,
    #[serde(rename = "RawFormat")]
    pub raw_format: Option<String>,
    #[serde(rename = "RateControlMode")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "Spec")]
    pub spec: Option<String>,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
    #[serde(rename = "InputType")]
    pub input_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MsSmoothGroupSettings {
    #[serde(rename = "SparseTrackType")]
    pub sparse_track_type: Option<String>,
    #[serde(rename = "FilecacheDuration")]
    pub filecache_duration: Option<usize>,
    #[serde(rename = "TimestampOffsetMode")]
    pub timestamp_offset_mode: Option<String>,
    #[serde(rename = "InputLossAction")]
    pub input_loss_action: Option<String>,
    #[serde(rename = "RestartDelay")]
    pub restart_delay: Option<usize>,
    #[serde(rename = "EventIdMode")]
    pub event_id_mode: Option<String>,
    #[serde(rename = "NumRetries")]
    pub num_retries: Option<usize>,
    #[serde(rename = "ConnectionRetryInterval")]
    pub connection_retry_interval: Option<usize>,
    #[serde(rename = "Destination")]
    pub destination: Option<OutputLocationRef>,
    #[serde(rename = "EventStopBehavior")]
    pub event_stop_behavior: Option<String>,
    #[serde(rename = "AudioOnlyTimecodeControl")]
    pub audio_only_timecode_control: Option<String>,
    #[serde(rename = "AcquisitionPointId")]
    pub acquisition_point_id: Option<String>,
    #[serde(rename = "CertificateMode")]
    pub certificate_mode: Option<String>,
    #[serde(rename = "FragmentLength")]
    pub fragment_length: Option<usize>,
    #[serde(rename = "TimestampOffset")]
    pub timestamp_offset: Option<String>,
    #[serde(rename = "EventId")]
    pub event_id: Option<String>,
    #[serde(rename = "SendDelayMs")]
    pub send_delay_ms: Option<usize>,
    #[serde(rename = "SegmentationMode")]
    pub segmentation_mode: Option<String>,
    #[serde(rename = "StreamManifestBehavior")]
    pub stream_manifest_behavior: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Eac3Settings {
    #[serde(rename = "DcFilter")]
    pub dc_filter: Option<String>,
    #[serde(rename = "StereoDownmix")]
    pub stereo_downmix: Option<String>,
    #[serde(rename = "DrcLine")]
    pub drc_line: Option<String>,
    #[serde(rename = "CodingMode")]
    pub coding_mode: Option<String>,
    #[serde(rename = "SurroundMode")]
    pub surround_mode: Option<String>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<f64>,
    #[serde(rename = "LoRoCenterMixLevel")]
    pub lo_ro_center_mix_level: Option<f64>,
    #[serde(rename = "DrcRf")]
    pub drc_rf: Option<String>,
    #[serde(rename = "BitstreamMode")]
    pub bitstream_mode: Option<String>,
    #[serde(rename = "SurroundExMode")]
    pub surround_ex_mode: Option<String>,
    #[serde(rename = "LfeControl")]
    pub lfe_control: Option<String>,
    #[serde(rename = "PhaseControl")]
    pub phase_control: Option<String>,
    #[serde(rename = "LtRtCenterMixLevel")]
    pub lt_rt_center_mix_level: Option<f64>,
    #[serde(rename = "LfeFilter")]
    pub lfe_filter: Option<String>,
    #[serde(rename = "Dialnorm")]
    pub dialnorm: Option<usize>,
    #[serde(rename = "LoRoSurroundMixLevel")]
    pub lo_ro_surround_mix_level: Option<f64>,
    #[serde(rename = "LtRtSurroundMixLevel")]
    pub lt_rt_surround_mix_level: Option<f64>,
    #[serde(rename = "AttenuationControl")]
    pub attenuation_control: Option<String>,
    #[serde(rename = "PassthroughControl")]
    pub passthrough_control: Option<String>,
    #[serde(rename = "MetadataControl")]
    pub metadata_control: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Ac3Settings {
    #[serde(rename = "MetadataControl")]
    pub metadata_control: Option<String>,
    #[serde(rename = "CodingMode")]
    pub coding_mode: Option<String>,
    #[serde(rename = "Dialnorm")]
    pub dialnorm: Option<usize>,
    #[serde(rename = "BitstreamMode")]
    pub bitstream_mode: Option<String>,
    #[serde(rename = "LfeFilter")]
    pub lfe_filter: Option<String>,
    #[serde(rename = "DrcProfile")]
    pub drc_profile: Option<String>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ArchiveS3Settings {
    #[serde(rename = "CannedAcl")]
    pub canned_acl: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioPidSelection {
    #[serde(rename = "Pid")]
    pub pid: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VideoSelectorProgramId {
    #[serde(rename = "ProgramId")]
    pub program_id: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct H265FilterSettings {
    #[serde(rename = "TemporalFilterSettings")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct AutomaticInputFailoverSettings {
    #[serde(rename = "SecondaryInputId")]
    pub secondary_input_id: Option<String>,
    #[serde(rename = "ErrorClearTimeMsec")]
    pub error_clear_time_msec: Option<usize>,
    #[serde(rename = "InputPreference")]
    pub input_preference: Option<String>,
    #[serde(rename = "FailoverConditions")]
    pub failover_conditions: Option<Vec<FailoverCondition>>,

}

#[derive(serde::Serialize, Default)]
pub struct FrameCaptureSettings {
    #[serde(rename = "CaptureIntervalUnits")]
    pub capture_interval_units: Option<String>,
    #[serde(rename = "CaptureInterval")]
    pub capture_interval: Option<usize>,
    #[serde(rename = "TimecodeBurninSettings")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsGroupSettings {
    #[serde(rename = "IvInManifest")]
    pub iv_in_manifest: Option<String>,
    #[serde(rename = "AdMarkers")]
    pub ad_markers: Option<Vec<String>>,
    #[serde(rename = "ProgramDateTimeClock")]
    pub program_date_time_clock: Option<String>,
    #[serde(rename = "MinSegmentLength")]
    pub min_segment_length: Option<usize>,
    #[serde(rename = "ClientCache")]
    pub client_cache: Option<String>,
    #[serde(rename = "TimedMetadataId3Period")]
    pub timed_metadata_id3_period: Option<usize>,
    #[serde(rename = "IncompleteSegmentBehavior")]
    pub incomplete_segment_behavior: Option<String>,
    #[serde(rename = "ProgramDateTime")]
    pub program_date_time: Option<String>,
    #[serde(rename = "SegmentsPerSubdirectory")]
    pub segments_per_subdirectory: Option<usize>,
    #[serde(rename = "Destination")]
    pub destination: Option<OutputLocationRef>,
    #[serde(rename = "EncryptionType")]
    pub encryption_type: Option<String>,
    #[serde(rename = "BaseUrlManifest")]
    pub base_url_manifest: Option<String>,
    #[serde(rename = "InputLossAction")]
    pub input_loss_action: Option<String>,
    #[serde(rename = "HlsId3SegmentTagging")]
    pub hls_id3_segment_tagging: Option<String>,
    #[serde(rename = "IndexNSegments")]
    pub index_nsegments: Option<usize>,
    #[serde(rename = "SegmentationMode")]
    pub segmentation_mode: Option<String>,
    #[serde(rename = "TimedMetadataId3Frame")]
    pub timed_metadata_id3_frame: Option<String>,
    #[serde(rename = "KeyFormat")]
    pub key_format: Option<String>,
    #[serde(rename = "BaseUrlContent")]
    pub base_url_content: Option<String>,
    #[serde(rename = "KeyFormatVersions")]
    pub key_format_versions: Option<String>,
    #[serde(rename = "OutputSelection")]
    pub output_selection: Option<String>,
    #[serde(rename = "CaptionLanguageSetting")]
    pub caption_language_setting: Option<String>,
    #[serde(rename = "KeepSegments")]
    pub keep_segments: Option<usize>,
    #[serde(rename = "DiscontinuityTags")]
    pub discontinuity_tags: Option<String>,
    #[serde(rename = "ManifestCompression")]
    pub manifest_compression: Option<String>,
    #[serde(rename = "ManifestDurationFormat")]
    pub manifest_duration_format: Option<String>,
    #[serde(rename = "CodecSpecification")]
    pub codec_specification: Option<String>,
    #[serde(rename = "KeyProviderSettings")]
    pub key_provider_settings: Option<KeyProviderSettings>,
    #[serde(rename = "BaseUrlContent1")]
    pub base_url_content1: Option<String>,
    #[serde(rename = "SegmentLength")]
    pub segment_length: Option<usize>,
    #[serde(rename = "TimestampDeltaMilliseconds")]
    pub timestamp_delta_milliseconds: Option<usize>,
    #[serde(rename = "ConstantIv")]
    pub constant_iv: Option<String>,
    #[serde(rename = "RedundantManifest")]
    pub redundant_manifest: Option<String>,
    #[serde(rename = "HlsCdnSettings")]
    pub hls_cdn_settings: Option<HlsCdnSettings>,
    #[serde(rename = "Mode")]
    pub mode: Option<String>,
    #[serde(rename = "ProgramDateTimePeriod")]
    pub program_date_time_period: Option<usize>,
    #[serde(rename = "IvSource")]
    pub iv_source: Option<String>,
    #[serde(rename = "IFrameOnlyPlaylists")]
    pub iframe_only_playlists: Option<String>,
    #[serde(rename = "StreamInfResolution")]
    pub stream_inf_resolution: Option<String>,
    #[serde(rename = "TsFileMode")]
    pub ts_file_mode: Option<String>,
    #[serde(rename = "CaptionLanguageMappings")]
    pub caption_language_mappings: Option<Vec<CaptionLanguageMapping>>,
    #[serde(rename = "BaseUrlManifest1")]
    pub base_url_manifest1: Option<String>,
    #[serde(rename = "DirectoryStructure")]
    pub directory_structure: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Output {
    #[serde(rename = "CaptionDescriptionNames")]
    pub caption_description_names: Option<Vec<String>>,
    #[serde(rename = "OutputName")]
    pub output_name: Option<String>,
    #[serde(rename = "OutputSettings")]
    pub output_settings: Option<OutputSettings>,
    #[serde(rename = "AudioDescriptionNames")]
    pub audio_description_names: Option<Vec<String>>,
    #[serde(rename = "VideoDescriptionName")]
    pub video_description_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OutputGroupSettings {
    #[serde(rename = "RtmpGroupSettings")]
    pub rtmp_group_settings: Option<RtmpGroupSettings>,
    #[serde(rename = "MultiplexGroupSettings")]
    pub multiplex_group_settings: Option<MultiplexGroupSettings>,
    #[serde(rename = "FrameCaptureGroupSettings")]
    pub frame_capture_group_settings: Option<FrameCaptureGroupSettings>,
    #[serde(rename = "UdpGroupSettings")]
    pub udp_group_settings: Option<UdpGroupSettings>,
    #[serde(rename = "ArchiveGroupSettings")]
    pub archive_group_settings: Option<ArchiveGroupSettings>,
    #[serde(rename = "HlsGroupSettings")]
    pub hls_group_settings: Option<HlsGroupSettings>,
    #[serde(rename = "MediaPackageGroupSettings")]
    pub media_package_group_settings: Option<MediaPackageGroupSettings>,
    #[serde(rename = "MsSmoothGroupSettings")]
    pub ms_smooth_group_settings: Option<MsSmoothGroupSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct OutputDestinationSettings {
    #[serde(rename = "PasswordParam")]
    pub password_param: Option<String>,
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "Url")]
    pub url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputSpecification {
    #[serde(rename = "Resolution")]
    pub resolution: Option<String>,
    #[serde(rename = "Codec")]
    pub codec: Option<String>,
    #[serde(rename = "MaximumBitrate")]
    pub maximum_bitrate: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsAkamaiSettings {
    #[serde(rename = "Salt")]
    pub salt: Option<String>,
    #[serde(rename = "ConnectionRetryInterval")]
    pub connection_retry_interval: Option<usize>,
    #[serde(rename = "Token")]
    pub token: Option<String>,
    #[serde(rename = "HttpTransferMode")]
    pub http_transfer_mode: Option<String>,
    #[serde(rename = "NumRetries")]
    pub num_retries: Option<usize>,
    #[serde(rename = "RestartDelay")]
    pub restart_delay: Option<usize>,
    #[serde(rename = "FilecacheDuration")]
    pub filecache_duration: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AvailConfiguration {
    #[serde(rename = "AvailSettings")]
    pub avail_settings: Option<AvailSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct H264ColorSpaceSettings {
    #[serde(rename = "Rec601Settings")]
    pub rec601_settings: Option<Rec601Settings>,
    #[serde(rename = "ColorSpacePassthroughSettings")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,
    #[serde(rename = "Rec709Settings")]
    pub rec709_settings: Option<Rec709Settings>,

}

#[derive(serde::Serialize, Default)]
pub struct FrameCaptureS3Settings {
    #[serde(rename = "CannedAcl")]
    pub canned_acl: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AribSourceSettings {

}

#[derive(serde::Serialize, Default)]
pub struct TeletextSourceSettings {
    #[serde(rename = "OutputRectangle")]
    pub output_rectangle: Option<CaptionRectangle>,
    #[serde(rename = "PageNumber")]
    pub page_number: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HlsOutputSettings {
    #[serde(rename = "NameModifier")]
    pub name_modifier: Option<String>,
    #[serde(rename = "H265PackagingType")]
    pub h265_packaging_type: Option<String>,
    #[serde(rename = "HlsSettings")]
    pub hls_settings: Option<HlsSettings>,
    #[serde(rename = "SegmentModifier")]
    pub segment_modifier: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioCodecSettings {
    #[serde(rename = "PassThroughSettings")]
    pub pass_through_settings: Option<PassThroughSettings>,
    #[serde(rename = "WavSettings")]
    pub wav_settings: Option<WavSettings>,
    #[serde(rename = "Mp2Settings")]
    pub mp2_settings: Option<Mp2Settings>,
    #[serde(rename = "Eac3AtmosSettings")]
    pub eac3_atmos_settings: Option<Eac3AtmosSettings>,
    #[serde(rename = "Eac3Settings")]
    pub eac3_settings: Option<Eac3Settings>,
    #[serde(rename = "Ac3Settings")]
    pub ac3_settings: Option<Ac3Settings>,
    #[serde(rename = "AacSettings")]
    pub aac_settings: Option<AacSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct CaptionLanguageMapping {
    #[serde(rename = "CaptionChannel")]
    pub caption_channel: Option<usize>,
    #[serde(rename = "LanguageCode")]
    pub language_code: Option<String>,
    #[serde(rename = "LanguageDescription")]
    pub language_description: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct M2tsSettings {
    #[serde(rename = "EtvPlatformPid")]
    pub etv_platform_pid: Option<String>,
    #[serde(rename = "EcmPid")]
    pub ecm_pid: Option<String>,
    #[serde(rename = "AudioBufferModel")]
    pub audio_buffer_model: Option<String>,
    #[serde(rename = "EbpPlacement")]
    pub ebp_placement: Option<String>,
    #[serde(rename = "Scte35Pid")]
    pub scte35_pid: Option<String>,
    #[serde(rename = "BufferModel")]
    pub buffer_model: Option<String>,
    #[serde(rename = "AudioFramesPerPes")]
    pub audio_frames_per_pes: Option<usize>,
    #[serde(rename = "Arib")]
    pub arib: Option<String>,
    #[serde(rename = "RateMode")]
    pub rate_mode: Option<String>,
    #[serde(rename = "PmtInterval")]
    pub pmt_interval: Option<usize>,
    #[serde(rename = "PmtPid")]
    pub pmt_pid: Option<String>,
    #[serde(rename = "DvbSubPids")]
    pub dvb_sub_pids: Option<String>,
    #[serde(rename = "FragmentTime")]
    pub fragment_time: Option<f64>,
    #[serde(rename = "PcrPid")]
    pub pcr_pid: Option<String>,
    #[serde(rename = "AbsentInputAudioBehavior")]
    pub absent_input_audio_behavior: Option<String>,
    #[serde(rename = "EtvSignalPid")]
    pub etv_signal_pid: Option<String>,
    #[serde(rename = "Scte35Control")]
    pub scte35_control: Option<String>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<usize>,
    #[serde(rename = "EsRateInPes")]
    pub es_rate_in_pes: Option<String>,
    #[serde(rename = "EbpLookaheadMs")]
    pub ebp_lookahead_ms: Option<usize>,
    #[serde(rename = "DvbSdtSettings")]
    pub dvb_sdt_settings: Option<DvbSdtSettings>,
    #[serde(rename = "DvbTdtSettings")]
    pub dvb_tdt_settings: Option<DvbTdtSettings>,
    #[serde(rename = "TimedMetadataPid")]
    pub timed_metadata_pid: Option<String>,
    #[serde(rename = "Scte27Pids")]
    pub scte27_pids: Option<String>,
    #[serde(rename = "Ebif")]
    pub ebif: Option<String>,
    #[serde(rename = "TimedMetadataBehavior")]
    pub timed_metadata_behavior: Option<String>,
    #[serde(rename = "ProgramNum")]
    pub program_num: Option<usize>,
    #[serde(rename = "NullPacketBitrate")]
    pub null_packet_bitrate: Option<f64>,
    #[serde(rename = "Scte35PrerollPullupMilliseconds")]
    pub scte35_preroll_pullup_milliseconds: Option<f64>,
    #[serde(rename = "AribCaptionsPidControl")]
    pub arib_captions_pid_control: Option<String>,
    #[serde(rename = "EbpAudioInterval")]
    pub ebp_audio_interval: Option<String>,
    #[serde(rename = "SegmentationTime")]
    pub segmentation_time: Option<f64>,
    #[serde(rename = "CcDescriptor")]
    pub cc_descriptor: Option<String>,
    #[serde(rename = "DvbNitSettings")]
    pub dvb_nit_settings: Option<DvbNitSettings>,
    #[serde(rename = "AribCaptionsPid")]
    pub arib_captions_pid: Option<String>,
    #[serde(rename = "AudioPids")]
    pub audio_pids: Option<String>,
    #[serde(rename = "DvbTeletextPid")]
    pub dvb_teletext_pid: Option<String>,
    #[serde(rename = "KlvDataPids")]
    pub klv_data_pids: Option<String>,
    #[serde(rename = "VideoPid")]
    pub video_pid: Option<String>,
    #[serde(rename = "NielsenId3Behavior")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "PcrPeriod")]
    pub pcr_period: Option<usize>,
    #[serde(rename = "AudioStreamType")]
    pub audio_stream_type: Option<String>,
    #[serde(rename = "Klv")]
    pub klv: Option<String>,
    #[serde(rename = "SegmentationStyle")]
    pub segmentation_style: Option<String>,
    #[serde(rename = "PcrControl")]
    pub pcr_control: Option<String>,
    #[serde(rename = "PatInterval")]
    pub pat_interval: Option<usize>,
    #[serde(rename = "TransportStreamId")]
    pub transport_stream_id: Option<usize>,
    #[serde(rename = "SegmentationMarkers")]
    pub segmentation_markers: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct UdpOutputSettings {
    #[serde(rename = "ContainerSettings")]
    pub container_settings: Option<UdpContainerSettings>,
    #[serde(rename = "Destination")]
    pub destination: Option<OutputLocationRef>,
    #[serde(rename = "FecOutputSettings")]
    pub fec_output_settings: Option<FecOutputSettings>,
    #[serde(rename = "BufferMsec")]
    pub buffer_msec: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct MultiplexGroupSettings {

}

#[derive(serde::Serialize, Default)]
pub struct HlsSettings {
    #[serde(rename = "AudioOnlyHlsSettings")]
    pub audio_only_hls_settings: Option<AudioOnlyHlsSettings>,
    #[serde(rename = "FrameCaptureHlsSettings")]
    pub frame_capture_hls_settings: Option<FrameCaptureHlsSettings>,
    #[serde(rename = "Fmp4HlsSettings")]
    pub fmp4_hls_settings: Option<Fmp4HlsSettings>,
    #[serde(rename = "StandardHlsSettings")]
    pub standard_hls_settings: Option<StandardHlsSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct Scte20SourceSettings {
    #[serde(rename = "Convert608To708")]
    pub convert608_to708: Option<String>,
    #[serde(rename = "Source608ChannelNumber")]
    pub source608_channel_number: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DvbSubDestinationSettings {
    #[serde(rename = "TeletextGridControl")]
    pub teletext_grid_control: Option<String>,
    #[serde(rename = "FontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "ShadowXOffset")]
    pub shadow_xoffset: Option<usize>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "BackgroundOpacity")]
    pub background_opacity: Option<usize>,
    #[serde(rename = "ShadowOpacity")]
    pub shadow_opacity: Option<usize>,
    #[serde(rename = "XPosition")]
    pub xposition: Option<usize>,
    #[serde(rename = "Alignment")]
    pub alignment: Option<String>,
    #[serde(rename = "FontResolution")]
    pub font_resolution: Option<usize>,
    #[serde(rename = "ShadowColor")]
    pub shadow_color: Option<String>,
    #[serde(rename = "OutlineSize")]
    pub outline_size: Option<usize>,
    #[serde(rename = "OutlineColor")]
    pub outline_color: Option<String>,
    #[serde(rename = "Font")]
    pub font: Option<InputLocation>,
    #[serde(rename = "ShadowYOffset")]
    pub shadow_yoffset: Option<usize>,
    #[serde(rename = "YPosition")]
    pub yposition: Option<usize>,
    #[serde(rename = "FontOpacity")]
    pub font_opacity: Option<usize>,
    #[serde(rename = "FontSize")]
    pub font_size: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MediaPackageOutputSettings {

}

#[derive(serde::Serialize, Default)]
pub struct HlsMediaStoreSettings {
    #[serde(rename = "FilecacheDuration")]
    pub filecache_duration: Option<usize>,
    #[serde(rename = "ConnectionRetryInterval")]
    pub connection_retry_interval: Option<usize>,
    #[serde(rename = "NumRetries")]
    pub num_retries: Option<usize>,
    #[serde(rename = "MediaStoreStorageClass")]
    pub media_store_storage_class: Option<String>,
    #[serde(rename = "RestartDelay")]
    pub restart_delay: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct FecOutputSettings {
    #[serde(rename = "ColumnDepth")]
    pub column_depth: Option<usize>,
    #[serde(rename = "RowLength")]
    pub row_length: Option<usize>,
    #[serde(rename = "IncludeFec")]
    pub include_fec: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RtmpOutputSettings {
    #[serde(rename = "CertificateMode")]
    pub certificate_mode: Option<String>,
    #[serde(rename = "NumRetries")]
    pub num_retries: Option<usize>,
    #[serde(rename = "ConnectionRetryInterval")]
    pub connection_retry_interval: Option<usize>,
    #[serde(rename = "Destination")]
    pub destination: Option<OutputLocationRef>,

}

#[derive(serde::Serialize, Default)]
pub struct NielsenCBET {
    #[serde(rename = "Csid")]
    pub csid: Option<String>,
    #[serde(rename = "CbetCheckDigitString")]
    pub cbet_check_digit_string: Option<String>,
    #[serde(rename = "CbetStepaside")]
    pub cbet_stepaside: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct WavSettings {
    #[serde(rename = "BitDepth")]
    pub bit_depth: Option<f64>,
    #[serde(rename = "CodingMode")]
    pub coding_mode: Option<String>,
    #[serde(rename = "SampleRate")]
    pub sample_rate: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct InputLocation {
    #[serde(rename = "PasswordParam")]
    pub password_param: Option<String>,
    #[serde(rename = "Uri")]
    pub uri: Option<String>,
    #[serde(rename = "Username")]
    pub username: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RtmpGroupSettings {
    #[serde(rename = "CacheLength")]
    pub cache_length: Option<usize>,
    #[serde(rename = "RestartDelay")]
    pub restart_delay: Option<usize>,
    #[serde(rename = "CacheFullBehavior")]
    pub cache_full_behavior: Option<String>,
    #[serde(rename = "AdMarkers")]
    pub ad_markers: Option<Vec<String>>,
    #[serde(rename = "InputLossAction")]
    pub input_loss_action: Option<String>,
    #[serde(rename = "AuthenticationScheme")]
    pub authentication_scheme: Option<String>,
    #[serde(rename = "CaptionData")]
    pub caption_data: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Scte20PlusEmbeddedDestinationSettings {

}

#[derive(serde::Serialize, Default)]
pub struct H265Settings {
    #[serde(rename = "ColorSpaceSettings")]
    pub color_space_settings: Option<H265ColorSpaceSettings>,
    #[serde(rename = "BufSize")]
    pub buf_size: Option<usize>,
    #[serde(rename = "GopClosedCadence")]
    pub gop_closed_cadence: Option<usize>,
    #[serde(rename = "QvbrQualityLevel")]
    pub qvbr_quality_level: Option<usize>,
    #[serde(rename = "ParDenominator")]
    pub par_denominator: Option<usize>,
    #[serde(rename = "SceneChangeDetect")]
    pub scene_change_detect: Option<String>,
    #[serde(rename = "LookAheadRateControl")]
    pub look_ahead_rate_control: Option<String>,
    #[serde(rename = "AdaptiveQuantization")]
    pub adaptive_quantization: Option<String>,
    #[serde(rename = "MinIInterval")]
    pub min_iinterval: Option<usize>,
    #[serde(rename = "Tier")]
    pub tier: Option<String>,
    #[serde(rename = "RateControlMode")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "AlternativeTransferFunction")]
    pub alternative_transfer_function: Option<String>,
    #[serde(rename = "FixedAfd")]
    pub fixed_afd: Option<String>,
    #[serde(rename = "FilterSettings")]
    pub filter_settings: Option<H265FilterSettings>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<usize>,
    #[serde(rename = "AfdSignaling")]
    pub afd_signaling: Option<String>,
    #[serde(rename = "FramerateDenominator")]
    pub framerate_denominator: Option<usize>,
    #[serde(rename = "MaxBitrate")]
    pub max_bitrate: Option<usize>,
    #[serde(rename = "GopSize")]
    pub gop_size: Option<f64>,
    #[serde(rename = "ColorMetadata")]
    pub color_metadata: Option<String>,
    #[serde(rename = "Level")]
    pub level: Option<String>,
    #[serde(rename = "ParNumerator")]
    pub par_numerator: Option<usize>,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
    #[serde(rename = "FramerateNumerator")]
    pub framerate_numerator: Option<usize>,
    #[serde(rename = "GopSizeUnits")]
    pub gop_size_units: Option<String>,
    #[serde(rename = "FlickerAq")]
    pub flicker_aq: Option<String>,
    #[serde(rename = "ScanType")]
    pub scan_type: Option<String>,
    #[serde(rename = "TimecodeBurninSettings")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
    #[serde(rename = "Slices")]
    pub slices: Option<usize>,
    #[serde(rename = "TimecodeInsertion")]
    pub timecode_insertion: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DvbSdtSettings {
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "OutputSdt")]
    pub output_sdt: Option<String>,
    #[serde(rename = "ServiceProviderName")]
    pub service_provider_name: Option<String>,
    #[serde(rename = "RepInterval")]
    pub rep_interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct EmbeddedDestinationSettings {

}

#[derive(serde::Serialize, Default)]
pub struct UdpGroupSettings {
    #[serde(rename = "TimedMetadataId3Frame")]
    pub timed_metadata_id3_frame: Option<String>,
    #[serde(rename = "TimedMetadataId3Period")]
    pub timed_metadata_id3_period: Option<usize>,
    #[serde(rename = "InputLossAction")]
    pub input_loss_action: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputSettings {
    #[serde(rename = "NetworkInputSettings")]
    pub network_input_settings: Option<NetworkInputSettings>,
    #[serde(rename = "DenoiseFilter")]
    pub denoise_filter: Option<String>,
    #[serde(rename = "Smpte2038DataPreference")]
    pub smpte2038_data_preference: Option<String>,
    #[serde(rename = "VideoSelector")]
    pub video_selector: Option<VideoSelector>,
    #[serde(rename = "DeblockFilter")]
    pub deblock_filter: Option<String>,
    #[serde(rename = "FilterStrength")]
    pub filter_strength: Option<usize>,
    #[serde(rename = "InputFilter")]
    pub input_filter: Option<String>,
    #[serde(rename = "Scte35Pid")]
    pub scte35_pid: Option<usize>,
    #[serde(rename = "AudioSelectors")]
    pub audio_selectors: Option<Vec<AudioSelector>>,
    #[serde(rename = "CaptionSelectors")]
    pub caption_selectors: Option<Vec<CaptionSelector>>,
    #[serde(rename = "SourceEndBehavior")]
    pub source_end_behavior: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputLossBehavior {
    #[serde(rename = "BlackFrameMsec")]
    pub black_frame_msec: Option<usize>,
    #[serde(rename = "InputLossImageSlate")]
    pub input_loss_image_slate: Option<InputLocation>,
    #[serde(rename = "InputLossImageColor")]
    pub input_loss_image_color: Option<String>,
    #[serde(rename = "InputLossImageType")]
    pub input_loss_image_type: Option<String>,
    #[serde(rename = "RepeatFrameMsec")]
    pub repeat_frame_msec: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioNormalizationSettings {
    #[serde(rename = "Algorithm")]
    pub algorithm: Option<String>,
    #[serde(rename = "TargetLkfs")]
    pub target_lkfs: Option<f64>,
    #[serde(rename = "AlgorithmControl")]
    pub algorithm_control: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ArchiveOutputSettings {
    #[serde(rename = "Extension")]
    pub extension: Option<String>,
    #[serde(rename = "ContainerSettings")]
    pub container_settings: Option<ArchiveContainerSettings>,
    #[serde(rename = "NameModifier")]
    pub name_modifier: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AudioTrackSelection {
    #[serde(rename = "DolbyEDecode")]
    pub dolby_edecode: Option<AudioDolbyEDecode>,
    #[serde(rename = "Tracks")]
    pub tracks: Option<Vec<AudioTrack>>,

}

#[derive(serde::Serialize, Default)]
pub struct MultiplexProgramChannelDestinationSettings {
    #[serde(rename = "MultiplexId")]
    pub multiplex_id: Option<String>,
    #[serde(rename = "ProgramName")]
    pub program_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CaptionRectangle {
    #[serde(rename = "Width")]
    pub width: Option<f64>,
    #[serde(rename = "TopOffset")]
    pub top_offset: Option<f64>,
    #[serde(rename = "LeftOffset")]
    pub left_offset: Option<f64>,
    #[serde(rename = "Height")]
    pub height: Option<f64>,

}


}

pub mod cfn_input_security_group {

#[derive(serde::Serialize, Default)]
pub struct CfnInputSecurityGroup {
    /// List of InputWhitelistRuleCidr
    #[serde(rename = "WhitelistRules")]
    pub whitelist_rules: Option<Vec<InputWhitelistRuleCidr>>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct InputWhitelistRuleCidr {
    #[serde(rename = "Cidr")]
    pub cidr: Option<String>,

}


}
