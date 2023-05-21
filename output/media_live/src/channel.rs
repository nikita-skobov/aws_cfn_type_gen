/// The AWS::MediaLive::Channel resource is a MediaLive resource type       that creates a channel.
///
/// A MediaLive channel ingests and transcodes (decodes and encodes)       source content from the inputs that are attached to that channel,       and packages the new content into outputs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnChannel {
    ///
    /// Specification of CDI inputs for this channel.
    ///
    /// Required: No
    ///
    /// Type: CdiInputSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,

    ///
    /// The class for this channel. For a channel with two pipelines, the       class is STANDARD. For a channel with one pipeline, the class is       SINGLE_PIPELINE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<cfn_resources::StrVal>,

    ///
    /// The settings that identify the destination for the outputs in this       MediaLive output package.
    ///
    /// Required: No
    ///
    /// Type: List of OutputDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,

    ///
    /// The encoding configuration for the output content.
    ///
    /// Required: No
    ///
    /// Type: EncoderSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncoderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,

    ///
    /// The list of input attachments for the channel.
    ///
    /// Required: No
    ///
    /// Type: List of InputAttachment
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,

    ///
    /// The input specification for this channel. It specifies the key       characteristics of the inputs for this channel: the maximum bitrate,       the resolution, and the codec.
    ///
    /// Required: No
    ///
    /// Type: InputSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,

    ///
    /// The verbosity for logging activity for this channel. Charges for       logging (which are generated through Amazon CloudWatch Logging) are       higher for higher verbosities.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: MaintenanceCreateSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Maintenance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceCreateSettings>,

    ///
    /// A name for this audio selector. The AudioDescription (in an       output) references this name in order to identify a specific input       audio to include in that output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The IAM role for MediaLive to assume when running this channel.       The role is identified by its ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// A collection of tags for this channel. Each tag is a key-value       pair.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    ///
    /// Settings to enable VPC mode in the channel, so that the endpoints       for all outputs are in your VPC.
    ///
    /// Required: No
    ///
    /// Type: VpcOutputSettings
    ///
    /// Update requires: Replacement
    #[serde(rename = "Vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettings>,
}

impl cfn_resources::CfnResource for CfnChannel {
    fn type_string(&self) -> &'static str {
        "AWS::MediaLive::Channel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cdi_input_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.encoder_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.input_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.maintenance
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for an AAC audio encode in the output.
///
/// The parent of this entity is AudioCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AacSettings {
    ///
    /// The average bitrate in bits/second. Valid values depend on the       rate control mode and profile.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,

    ///
    /// Mono, stereo, or 5.1 channel layout. Valid values depend on the       rate control mode and profile. The adReceiverMix setting receives a       stereo description plus control track, and emits a mono AAC encode       of the description track, with control data emitted in the PES       header as per ETSI TS 101 154 Annex E.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<cfn_resources::StrVal>,

    ///
    /// Set to broadcasterMixedAd when the input contains pre-mixed main       audio + AD (narration) as a stereo pair. The Audio Type field       (audioType) will be set to 3, which signals to downstream systems       that this stream contains broadcaster mixed AD. Note that the input       received by the encoder must contain pre-mixed audio; MediaLive does       not perform the mixing. The values in audioTypeControl and audioType       (in AudioDescription) are ignored when set to broadcasterMixedAd.       Leave this set to normal when the input does not contain pre-mixed       audio + AD.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<cfn_resources::StrVal>,

    ///
    /// The AAC profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<cfn_resources::StrVal>,

    ///
    /// The rate control mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<cfn_resources::StrVal>,

    ///
    /// Sets the LATM/LOAS AAC output for raw containers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RawFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_format: Option<cfn_resources::StrVal>,

    ///
    /// The sample rate in Hz. Valid values depend on the rate control       mode and profile.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,

    ///
    /// Uses MPEG-2 AAC audio instead of MPEG-4 AAC audio for raw or       MPEG-2 Transport Stream containers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<cfn_resources::StrVal>,

    ///
    /// The VBR quality level. This is used only if rateControlMode is       VBR.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VbrQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AacSettings {
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

/// The settings for an AC3 audio encode in the output.
///
/// The parent of this entity is AudioCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ac3Settings {
    ///
    /// The average bitrate in bits/second. Valid bitrates depend on the       coding mode.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,

    ///
    /// Specifies the bitstream mode (bsmod) for the emitted AC-3 stream.       For more information about these values, see ATSC A/52-2012.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<cfn_resources::StrVal>,

    ///
    /// The Dolby Digital coding mode. This determines the number of       channels.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<cfn_resources::StrVal>,

    ///
    /// Sets the dialnorm for the output. If excluded and the input audio       is Dolby Digital, dialnorm is passed through.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,

    ///
    /// If set to filmStandard, adds dynamic range compression signaling       to the output bitstream as defined in the Dolby Digital       specification.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DrcProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_profile: Option<cfn_resources::StrVal>,

    ///
    /// When set to enabled, applies a 120Hz lowpass filter to the LFE       channel prior to encoding. This is valid only in codingMode32Lfe       mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<cfn_resources::StrVal>,

    ///
    /// When set to followInput, encoder metadata is sourced from the DD,       DD+, or DolbyE decoder that supplies this audio data. If the audio       is supplied from one of these streams, the static metadata settings       are used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Ac3Settings {
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

/// Information about the ancillary captions to extract from the       input.
///
/// The parent of this entity is CaptionSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AncillarySourceSettings {
    ///
    /// Specifies the number (1 to 4) of the captions channel you want to extract from the ancillary captions. If you plan to convert the ancillary captions to another format, complete this field. If you plan to choose Embedded as the captions destination in the output (to pass through all the channels in the ancillary captions), leave this field blank because MediaLive ignores the field.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceAncillaryChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ancillary_channel_number: Option<i64>,
}

impl cfn_resources::CfnResource for AncillarySourceSettings {
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

/// Settings to configure the destination of an Archive output.
///
/// The parent of this entity is ArchiveGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArchiveCdnSettings {
    ///
    /// Sets up Amazon S3 as the destination for this Archive       output.
    ///
    /// Required: No
    ///
    /// Type: ArchiveS3Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArchiveS3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_s3_settings: Option<ArchiveS3Settings>,
}

impl cfn_resources::CfnResource for ArchiveCdnSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.archive_s3_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The archive container settings.
///
/// The parent of this entity is ArchiveOutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArchiveContainerSettings {
    ///
    /// The settings for the M2TS in the archive output.
    ///
    /// Required: No
    ///
    /// Type: M2tsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "M2tsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m2ts_settings: Option<M2tsSettings>,

    ///
    /// The settings for Raw archive output type.
    ///
    /// Required: No
    ///
    /// Type: RawSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RawSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_settings: Option<RawSettings>,
}

impl cfn_resources::CfnResource for ArchiveContainerSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.m2ts_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.raw_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for an archive output group.
///
/// The parent of this entity is OutputGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArchiveGroupSettings {
    ///
    /// Settings to configure the destination of an Archive output.
    ///
    /// Required: No
    ///
    /// Type: ArchiveCdnSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArchiveCdnSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_cdn_settings: Option<ArchiveCdnSettings>,

    ///
    /// A directory and base file name where archive files should be       written.
    ///
    /// Required: No
    ///
    /// Type: OutputLocationRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OutputLocationRef>,

    ///
    /// The number of seconds to write to an archive file before closing       and starting a new one.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RolloverInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_interval: Option<i64>,
}

impl cfn_resources::CfnResource for ArchiveGroupSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.archive_cdn_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The archive output settings.
///
/// The parent of this entity is OutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArchiveOutputSettings {
    ///
    /// The settings that are specific to the container type of the       file.
    ///
    /// Required: No
    ///
    /// Type: ArchiveContainerSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<ArchiveContainerSettings>,

    ///
    /// The output file extension. If excluded, this is auto-selected from       the container type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<cfn_resources::StrVal>,

    ///
    /// A string that is concatenated to the end of the destination file       name. The string is required for multiple outputs of the same       type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ArchiveOutputSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.container_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Sets up Amazon S3 as the destination for this Archive       output.
///
/// The parent of this entity is ArchiveCdnSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArchiveS3Settings {
    ///
    /// Specify the canned ACL to apply to each S3 request. Defaults to none.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ArchiveS3Settings {
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

/// The configuration of ARIB captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AribDestinationSettings {}

impl cfn_resources::CfnResource for AribDestinationSettings {
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

/// Information about the ARIB captions to extract from the       input.
///
/// The parent of this entity is CaptionSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AribSourceSettings {}

impl cfn_resources::CfnResource for AribSourceSettings {
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

/// The settings for remixing audio.
///
/// The parent of this entity is RemixSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioChannelMapping {
    ///
    /// The indices and gain values for each input channel that should be       remixed into this output channel.
    ///
    /// Required: No
    ///
    /// Type: List of InputChannelLevel
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputChannelLevels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_channel_levels: Option<Vec<InputChannelLevel>>,

    ///
    /// The index of the output channel that is being produced.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_channel: Option<i64>,
}

impl cfn_resources::CfnResource for AudioChannelMapping {
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

/// The configuration of the audio codec in the audio output.
///
/// The parent of this entity is AudioDescription.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioCodecSettings {
    ///
    /// The setup of the AAC audio codec in the output.
    ///
    /// Required: No
    ///
    /// Type: AacSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AacSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aac_settings: Option<AacSettings>,

    ///
    /// The setup of an AC3 audio codec in the output.
    ///
    /// Required: No
    ///
    /// Type: Ac3Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac3_settings: Option<Ac3Settings>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Eac3AtmosSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Eac3AtmosSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac3_atmos_settings: Option<Eac3AtmosSettings>,

    ///
    /// The setup of an EAC3 audio codec in the output.
    ///
    /// Required: No
    ///
    /// Type: Eac3Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Eac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac3_settings: Option<Eac3Settings>,

    ///
    /// The setup of an MP2 audio codec in the output.
    ///
    /// Required: No
    ///
    /// Type: Mp2Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mp2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp2_settings: Option<Mp2Settings>,

    ///
    /// The setup to pass through the Dolby audio codec to the       output.
    ///
    /// Required: No
    ///
    /// Type: PassThroughSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "PassThroughSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_through_settings: Option<PassThroughSettings>,

    ///
    /// Settings for audio encoded with the WAV codec.
    ///
    /// Required: No
    ///
    /// Type: WavSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "WavSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wav_settings: Option<WavSettings>,
}

impl cfn_resources::CfnResource for AudioCodecSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.aac_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ac3_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.eac3_atmos_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.eac3_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.mp2_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.pass_through_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.wav_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The encoding information for one output audio.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioDescription {
    ///
    /// The advanced audio normalization settings.
    ///
    /// Required: No
    ///
    /// Type: AudioNormalizationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioNormalizationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_normalization_settings: Option<AudioNormalizationSettings>,

    ///
    /// The name of the AudioSelector that is used as the source for this       AudioDescription.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioSelectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selector_name: Option<cfn_resources::StrVal>,

    ///
    /// Applies only if audioTypeControl is useConfigured. The values for       audioType are defined in ISO-IEC 13818-1.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type: Option<cfn_resources::StrVal>,

    ///
    /// Determines how audio type is determined. followInput: If the input       contains an ISO 639 audioType, then that value is passed through to       the output. If the input contains no ISO 639 audioType, the value in       Audio Type is included in the output. useConfigured: The value in       Audio Type is included in the output. Note that this field and       audioType are both ignored if inputType is       broadcasterMixedAd.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioTypeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type_control: Option<cfn_resources::StrVal>,

    ///
    /// Settings to configure one or more solutions that insert audio watermarks in the audio encode
    ///
    /// Required: No
    ///
    /// Type: AudioWatermarkSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioWatermarkingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_watermarking_settings: Option<AudioWatermarkSettings>,

    ///
    /// The audio codec settings.
    ///
    /// Required: No
    ///
    /// Type: AudioCodecSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodecSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<AudioCodecSettings>,

    ///
    /// Indicates the language of the audio output track. Used only if       languageControlMode is useConfigured, or there is no ISO 639       language code specified in the input.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<cfn_resources::StrVal>,

    ///
    /// Choosing followInput causes the ISO 639 language code of the       output to follow the ISO 639 language code of the input. The       languageCode setting is used when useConfigured is set, or when       followInput is selected but there is no ISO 639 language code       specified by the input.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageCodeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_control: Option<cfn_resources::StrVal>,

    ///
    /// The name of this AudioDescription. Outputs use this name to       uniquely identify this AudioDescription. Description names should be       unique within this channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The settings that control how input audio channels are remixed       into the output audio channels.
    ///
    /// Required: No
    ///
    /// Type: RemixSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemixSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_settings: Option<RemixSettings>,

    ///
    /// Used for Microsoft Smooth and Apple HLS outputs. Indicates the       name displayed by the player (for example, English or Director       Commentary).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AudioDescription {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.audio_normalization_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.audio_watermarking_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.codec_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.remix_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AudioDolbyEDecode property type specifies Property description not available. for an AWS::MediaLive::Channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioDolbyEDecode {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_selection: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AudioDolbyEDecode {
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

/// Selector for HLS audio rendition.
///
/// The parent of this entity is AudioSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioHlsRenditionSelection {
    ///
    /// Specifies the GROUP-ID in the #EXT-X-MEDIA tag of the target HLS audio rendition.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the NAME in the #EXT-X-MEDIA tag of the target HLS audio rendition.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AudioHlsRenditionSelection {
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

/// Information about the audio language to extract.
///
/// The parent of this entity is AudioSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioLanguageSelection {
    ///
    /// Selects a specific three-letter language code from within an audio       source.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<cfn_resources::StrVal>,

    ///
    /// When set to "strict," the transport stream demux strictly       identifies audio streams by their language descriptor. If a PMT       update occurs such that an audio stream matching the initially       selected language is no longer present, then mute is encoded until       the language returns. If set to "loose," then on a PMT update the       demux chooses another audio stream in the program with the same       stream type if it can't find one with the same language.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageSelectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_selection_policy: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AudioLanguageSelection {
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

/// The settings for normalizing video.
///
/// The parent of this entity is AudioDescription.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioNormalizationSettings {
    ///
    /// The audio normalization algorithm to use. itu17701 conforms to the       CALM Act specification. itu17702 conforms to the EBU R-128       specification.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<cfn_resources::StrVal>,

    ///
    /// When set to correctAudio, the output audio is corrected using the       chosen algorithm. If set to measureOnly, the audio is measured but       not adjusted.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlgorithmControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_control: Option<cfn_resources::StrVal>,

    ///
    /// The Target LKFS(loudness) to adjust volume to. If no value is       entered, a default value is used according to the chosen algorithm.       The CALM Act (1770-1) recommends a target of -24 LKFS. The EBU R-128       specification (1770-2) recommends a target of -23 LKFS.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetLkfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lkfs: Option<f64>,
}

impl cfn_resources::CfnResource for AudioNormalizationSettings {
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

/// The configuration of an audio-only HLS output.
///
/// The parent of this entity is HlsSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioOnlyHlsSettings {
    ///
    /// Specifies the group that the audio rendition belongs to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_group_id: Option<cfn_resources::StrVal>,

    ///
    /// Used with an audio-only stream. It must be a .jpg or .png file. If       given, this image is used as the cover art for the audio-only       output. Ideally, it should be formatted for an iPhone screen for two       reasons. The iPhone does not resize the image; instead, it crops a       centered image on the top/bottom and left/right. Additionally, this       image file gets saved bit-for-bit into every 10-second segment file,       so it increases bandwidth by {image file size} * {segment count} *       {user count.}.
    ///
    /// Required: No
    ///
    /// Type: InputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioOnlyImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_image: Option<InputLocation>,

    ///
    /// Four types of audio-only tracks are supported: Audio-Only Variant       Stream The client can play back this audio-only stream instead of       video in low-bandwidth scenarios. Represented as an EXT-X-STREAM-INF       in the HLS manifest. Alternate Audio, Auto Select, Default Alternate       rendition that the client should try to play back by default.       Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=YES,       AUTOSELECT=YES Alternate Audio, Auto Select, Not Default Alternate       rendition that the client might try to play back by default.       Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=NO,       AUTOSELECT=YES Alternate Audio, not Auto Select Alternate rendition       that the client will not try to play back by default. Represented as       an EXT-X-MEDIA in the HLS manifest with DEFAULT=NO,       AUTOSELECT=NO.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioTrackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_type: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the segment type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AudioOnlyHlsSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.audio_only_image
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Used to extract audio by The PID.
///
/// The parent of this entity is AudioSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioPidSelection {
    ///
    /// Select the audio by this PID.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

impl cfn_resources::CfnResource for AudioPidSelection {
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

/// Information about one audio to extract from the input.
///
/// The parent of this entity is InputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioSelector {
    ///
    /// A name for this AudioSelector.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Information about the specific audio to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: AudioSelectorSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectorSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<AudioSelectorSettings>,
}

impl cfn_resources::CfnResource for AudioSelector {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.selector_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the audio to extract from the input.
///
/// The parent of this entity is AudioSelector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioSelectorSettings {
    /// Selector for HLS audio rendition.
    ///
    /// Required: No
    ///
    /// Type: AudioHlsRenditionSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioHlsRenditionSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_hls_rendition_selection: Option<AudioHlsRenditionSelection>,

    ///
    /// The language code of the audio to select.
    ///
    /// Required: No
    ///
    /// Type: AudioLanguageSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioLanguageSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_language_selection: Option<AudioLanguageSelection>,

    ///
    /// The PID of the audio to select.
    ///
    /// Required: No
    ///
    /// Type: AudioPidSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioPidSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pid_selection: Option<AudioPidSelection>,

    /// Information about the audio track to extract.
    ///
    /// Required: No
    ///
    /// Type: AudioTrackSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioTrackSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_selection: Option<AudioTrackSelection>,
}

impl cfn_resources::CfnResource for AudioSelectorSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.audio_hls_rendition_selection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.audio_language_selection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.audio_pid_selection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.audio_track_selection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// MediaLive will perform a failover if audio is not detected in this       input for the specified period.
///
/// The parent of this entity is FailoverConditionSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioSilenceFailoverSettings {
    ///
    /// The name of the audio selector in the input that MediaLive should monitor to detect silence. Select your most important rendition. If you didn't create an audio selector in this input, leave blank.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioSelectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selector_name: Option<cfn_resources::StrVal>,

    ///
    /// The amount of time (in milliseconds) that the active input must be silent before automatic input failover occurs. Silence is defined as audio loss or audio quieter than -50 dBFS.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioSilenceThresholdMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_silence_threshold_msec: Option<i64>,
}

impl cfn_resources::CfnResource for AudioSilenceFailoverSettings {
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

/// Information about one audio track to extract. You can select       multiple tracks.
///
/// The parent of this entity is AudioTrackSelection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioTrack {
    ///
    /// 1-based integer value that maps to a specific audio track
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Track")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<i64>,
}

impl cfn_resources::CfnResource for AudioTrack {
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

/// Information about the audio track to extract.
///
/// The parent of this entity is AudioSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioTrackSelection {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AudioDolbyEDecode
    ///
    /// Update requires: No interruption
    #[serde(rename = "DolbyEDecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dolby_edecode: Option<AudioDolbyEDecode>,

    ///
    /// Selects one or more unique audio tracks from within a source.
    ///
    /// Required: No
    ///
    /// Type: List of AudioTrack
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tracks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<AudioTrack>>,
}

impl cfn_resources::CfnResource for AudioTrackSelection {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dolby_edecode
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Audio Watermark Settings
///
/// The parent of this entity is AudioDescription.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AudioWatermarkSettings {
    ///
    /// Settings to configure Nielsen Watermarks in the audio encode
    ///
    /// Required: No
    ///
    /// Type: NielsenWatermarksSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenWatermarksSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_watermarks_settings: Option<NielsenWatermarksSettings>,
}

impl cfn_resources::CfnResource for AudioWatermarkSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.nielsen_watermarks_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings to configure the conditions that will define the input as       unhealthy and that will make MediaLive fail over to the other input       in the input failover pair.
///
/// The parent of this entity is InputAttachment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutomaticInputFailoverSettings {
    ///
    /// This clear time defines the requirement a recovered input must meet to be considered healthy. The input must have no failover conditions for this length of time. Enter a time in milliseconds. This value is particularly important if the input_preference for the failover pair is set to PRIMARY_INPUT_PREFERRED, because after this time, MediaLive will switch back to the primary input.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorClearTimeMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_clear_time_msec: Option<i64>,

    ///
    /// A list of failover conditions. If any of these conditions occur, MediaLive will perform a failover to the other input.
    ///
    /// Required: No
    ///
    /// Type: List of FailoverCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailoverConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_conditions: Option<Vec<FailoverCondition>>,

    ///
    /// Input preference when deciding which input to make active when a previously failed input has recovered.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_preference: Option<cfn_resources::StrVal>,

    ///
    /// The input ID of the secondary input in the automatic input failover pair.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryInputId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_input_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AutomaticInputFailoverSettings {
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

/// The configuration of ad avail blanking in the output.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AvailBlanking {
    ///
    /// The blanking image to be used. Keep empty for solid black. Only       .bmp and .png images are supported.
    ///
    /// Required: No
    ///
    /// Type: InputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailBlankingImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking_image: Option<InputLocation>,

    ///
    /// When set to enabled, the video, audio, and captions are blanked       when insertion metadata is added.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AvailBlanking {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.avail_blanking_image
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The setup of ad avail handling in the output.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AvailConfiguration {
    ///
    /// The setup of ad avail handling in the output.
    ///
    /// Required: No
    ///
    /// Type: AvailSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_settings: Option<AvailSettings>,
}

impl cfn_resources::CfnResource for AvailConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.avail_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for the ad avail setup in the output.
///
/// The parent of this entity is AvailConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AvailSettings {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Esam
    ///
    /// Update requires: No interruption
    #[serde(rename = "Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esam: Option<Esam>,

    ///
    /// The setup for SCTE-35 splice insert handling.
    ///
    /// Required: No
    ///
    /// Type: Scte35SpliceInsert
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35SpliceInsert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_splice_insert: Option<Scte35SpliceInsert>,

    ///
    /// The setup for SCTE-35 time signal APOS handling.
    ///
    /// Required: No
    ///
    /// Type: Scte35TimeSignalApos
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35TimeSignalApos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_time_signal_apos: Option<Scte35TimeSignalApos>,
}

impl cfn_resources::CfnResource for AvailSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.esam.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.scte35_splice_insert
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.scte35_time_signal_apos
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for a blackout slate.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlackoutSlate {
    ///
    /// The blackout slate image to be used. Keep empty for solid black.       Only .bmp and .png images are supported.
    ///
    /// Required: No
    ///
    /// Type: InputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlackoutSlateImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blackout_slate_image: Option<InputLocation>,

    ///
    /// Setting to enabled causes MediaLive to blackout the video, audio,       and captions, and raise the "Network Blackout Image" slate when an       SCTE104/35 Network End Segmentation Descriptor is encountered. The       blackout is lifted when the Network Start Segmentation Descriptor is       encountered. The Network End and Network Start descriptors must       contain a network ID that matches the value entered in Network       ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkEndBlackout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_end_blackout: Option<cfn_resources::StrVal>,

    ///
    /// The path to the local file to use as the Network End Blackout       image. The image is scaled to fill the entire output raster.
    ///
    /// Required: No
    ///
    /// Type: InputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkEndBlackoutImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_end_blackout_image: Option<InputLocation>,

    ///
    /// Provides a Network ID that matches EIDR ID format (for example,       "10.XXXX/XXXX-XXXX-XXXX-XXXX-XXXX-C").
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<cfn_resources::StrVal>,

    ///
    /// When set to enabled, this causes video, audio, and captions to be       blanked when indicated by program metadata.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for BlackoutSlate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.blackout_slate_image
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.network_end_blackout_image
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for burn-in captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BurnInDestinationSettings {
    ///
    /// If no explicit xPosition or yPosition is provided, setting       alignment to centered places the captions at the bottom center of       the output. Similarly, setting a left alignment aligns captions to       the bottom left of the output. If x and y positions are specified in       conjunction with the alignment parameter, the font is justified       (either left or centered) relative to those coordinates. Selecting       "smart" justification left-justifies live subtitles and       center-justifies pre-recorded subtitles. All burn-in and DVB-Sub       font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the color of the rectangle behind the captions. All       burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the opacity of the background rectangle. 255 is opaque;       0 is transparent. Keeping this parameter blank is equivalent to       setting it to 0 (transparent). All burn-in and DVB-Sub font settings       must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,

    ///
    /// The external font file that is used for captions burn-in. The file       extension must be .ttf or .tte. Although you can select output fonts       for many different types of input captions, embedded, STL, and       Teletext sources use a strict grid system. Using external fonts with       these captions sources could cause an unexpected display of       proportional fonts. All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: InputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<InputLocation>,

    ///
    /// Specifies the color of the burned-in captions. This option is not       valid for source captions that are STL, 608/embedded, or Teletext.       These source settings are already pre-defined by the captions       stream. All burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the opacity of the burned-in captions. 255 is opaque; 0       is transparent. All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i64>,

    ///
    /// The font resolution in DPI (dots per inch). The default is 96 dpi.       All burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,

    ///
    /// When set to auto, fontSize scales depending on the size of the       output. Providing a positive integer specifies the exact font size       in points. All burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the font outline color. This option is not valid for       source captions that are either 608/embedded or Teletext. These       source settings are already pre-defined by the captions stream. All       burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<cfn_resources::StrVal>,

    ///
    /// Specifies font outline size in pixels. This option is not valid       for source captions that are either 608/embedded or Teletext. These       source settings are already pre-defined by the captions stream. All       burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutlineSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i64>,

    ///
    /// Specifies the color of the shadow cast by the captions. All       burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the opacity of the shadow. 255 is opaque; 0 is       transparent. Keeping this parameter blank is equivalent to setting       it to 0 (transparent). All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i64>,

    ///
    /// Specifies the horizontal offset of the shadow that is relative to       the captions in pixels. A value of -2 would result in a shadow       offset 2 pixels to the left. All burn-in and DVB-Sub font settings       must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowXOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_xoffset: Option<i64>,

    ///
    /// Specifies the vertical offset of the shadow that is relative to       the captions in pixels. A value of -2 would result in a shadow       offset 2 pixels above the text. All burn-in and DVB-Sub font       settings must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowYOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_yoffset: Option<i64>,

    ///
    /// Controls whether a fixed grid size is used to generate the output       subtitles bitmap. This applies only to Teletext inputs and       DVB-Sub/Burn-in outputs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TeletextGridControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_grid_control: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the horizontal position of the captions relative to the       left side of the output in pixels. A value of 10 would result in the       captions starting 10 pixels from the left of the output. If no       explicit xPosition is provided, the horizontal captions position is       determined by the alignment parameter. All burn-in and DVB-Sub font       settings must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "XPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xposition: Option<i64>,

    ///
    /// Specifies the vertical position of the captions relative to the       top of the output in pixels. A value of 10 would result in the       captions starting 10 pixels from the top of the output. If no       explicit yPosition is provided, the captions are positioned towards       the bottom of the output. All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "YPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yposition: Option<i64>,
}

impl cfn_resources::CfnResource for BurnInDestinationSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.font.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The encoding information for output captions.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptionDescription {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Accessibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<cfn_resources::StrVal>,

    ///
    /// Specifies which input captions selector to use as a captions       source when generating output captions. This field should match a       captionSelector name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptionSelectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selector_name: Option<cfn_resources::StrVal>,

    ///
    /// Additional settings for a captions destination that depend on the       destination type.
    ///
    /// Required: No
    ///
    /// Type: CaptionDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<CaptionDestinationSettings>,

    ///
    /// An ISO 639-2 three-digit code. For more information, see       http://www.loc.gov/standards/iso639-2/.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<cfn_resources::StrVal>,

    ///
    /// Human-readable information to indicate the captions that are       available for players (for example, English or Spanish).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the captions description. The name is used to       associate a captions description with an output. Names must be       unique within a channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CaptionDescription {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration of one captions encode in the output.
///
/// The parent of this entity is CaptionDescription.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptionDestinationSettings {
    ///
    /// The configuration of one ARIB captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: AribDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AribDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_destination_settings: Option<AribDestinationSettings>,

    ///
    /// The configuration of one burn-in captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: BurnInDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "BurnInDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_in_destination_settings: Option<BurnInDestinationSettings>,

    ///
    /// The configuration of one DVB Sub captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: DvbSubDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DvbSubDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_destination_settings: Option<DvbSubDestinationSettings>,

    /// Settings for EBU-TT captions in the output.
    ///
    /// Required: No
    ///
    /// Type: EbuTtDDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbuTtDDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebu_tt_ddestination_settings: Option<EbuTtDDestinationSettings>,

    ///
    /// The configuration of one embedded captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: EmbeddedDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmbeddedDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_destination_settings: Option<EmbeddedDestinationSettings>,

    ///
    /// The configuration of one embedded plus SCTE-20 captions encode in       the output.
    ///
    /// Required: No
    ///
    /// Type: EmbeddedPlusScte20DestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmbeddedPlusScte20DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_plus_scte20_destination_settings: Option<EmbeddedPlusScte20DestinationSettings>,

    ///
    /// The configuration of one RTMPCaptionInfo captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: RtmpCaptionInfoDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RtmpCaptionInfoDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_caption_info_destination_settings: Option<RtmpCaptionInfoDestinationSettings>,

    ///
    /// The configuration of one SCTE20 plus embedded captions encode in       the output.
    ///
    /// Required: No
    ///
    /// Type: Scte20PlusEmbeddedDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte20PlusEmbeddedDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte20_plus_embedded_destination_settings: Option<Scte20PlusEmbeddedDestinationSettings>,

    ///
    /// The configuration of one SCTE-27 captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: Scte27DestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte27DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte27_destination_settings: Option<Scte27DestinationSettings>,

    ///
    /// The configuration of one SMPTE-TT captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: SmpteTtDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmpteTtDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte_tt_destination_settings: Option<SmpteTtDestinationSettings>,

    ///
    /// The configuration of one Teletext captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: TeletextDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TeletextDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_destination_settings: Option<TeletextDestinationSettings>,

    ///
    /// The configuration of one TTML captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: TtmlDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TtmlDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttml_destination_settings: Option<TtmlDestinationSettings>,

    ///
    /// The configuration of one WebVTT captions encode in the       output.
    ///
    /// Required: No
    ///
    /// Type: WebvttDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebvttDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webvtt_destination_settings: Option<WebvttDestinationSettings>,
}

impl cfn_resources::CfnResource for CaptionDestinationSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.arib_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.burn_in_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dvb_sub_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ebu_tt_ddestination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.embedded_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.embedded_plus_scte20_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rtmp_caption_info_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.scte20_plus_embedded_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.scte27_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.smpte_tt_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.teletext_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ttml_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.webvtt_destination_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Maps a captions channel to an ISO 693-2 language code       (http://www.loc.gov/standards/iso639-2), with an optional       description.
///
/// The parent of this entity is HlsGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptionLanguageMapping {
    ///
    /// The closed caption channel being described by this       CaptionLanguageMapping. Each channel mapping must have a unique       channel number (maximum of 4).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptionChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_channel: Option<i64>,

    ///
    /// A three-character ISO 639-2 language code (see       http://www.loc.gov/standards/iso639-2).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<cfn_resources::StrVal>,

    ///
    /// The textual description of language.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CaptionLanguageMapping {
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

/// Settings to configure the caption rectangle for an output captions       that will be created using this Teletext source captions.
///
/// The parent of this entity is TeletextSourceSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptionRectangle {
    ///
    /// See the description in leftOffset.
    ///
    /// For height, specify the entire height of the rectangle as a       percentage of the underlying frame height. For example, \"80\" means       the rectangle height is 80% of the underlying frame height. The       topOffset and rectangleHeight must add up to 100% or less. This       field corresponds to tts:extent - Y in the TTML standard.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    ///
    /// Applies only if you plan to convert these source captions to       EBU-TT-D or TTML in an output. (Make sure to leave the default if       you don't have either of these formats in the output.) You can       define a display rectangle for the captions that is smaller than the       underlying video frame. You define the rectangle by specifying the       position of the left edge, top edge, bottom edge, and right edge of       the rectangle, all within the underlying video frame. The units for       the measurements are percentages. If you specify a value for one of       these fields, you must specify a value for all of them.
    ///
    /// For leftOffset, specify the position of the left edge of the       rectangle, as a percentage of the underlying frame width, and       relative to the left edge of the frame. For example, \"10\" means       the measurement is 10% of the underlying frame width. The rectangle       left edge starts at that position from the left edge of the frame.       This field corresponds to tts:origin - X in the TTML       standard.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LeftOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_offset: Option<f64>,

    ///
    /// See the description in leftOffset.
    ///
    /// For topOffset, specify the position of the top edge of the       rectangle, as a percentage of the underlying frame height, and       relative to the top edge of the frame. For example, \"10\" means the       measurement is 10% of the underlying frame height. The rectangle top       edge starts at that position from the top edge of the frame. This       field corresponds to tts:origin - Y in the TTML standard.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_offset: Option<f64>,

    ///
    /// See the description in leftOffset.
    ///
    /// For width, specify the entire width of the rectangle as a       percentage of the underlying frame width. For example, \"80\" means       the rectangle width is 80% of the underlying frame width. The       leftOffset and rectangleWidth must add up to 100% or less. This       field corresponds to tts:extent - X in the TTML standard.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

impl cfn_resources::CfnResource for CaptionRectangle {
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

/// Information about one caption to extract from the input.
///
/// The parent of this entity is InputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptionSelector {
    ///
    /// When specified, this field indicates the three-letter language       code of the captions track to extract from the source.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<cfn_resources::StrVal>,

    ///
    /// The name identifier for a captions selector. This name is used to       associate this captions selector with one or more captions       descriptions. Names must be unique within a channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Information about the specific audio to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: CaptionSelectorSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectorSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<CaptionSelectorSettings>,
}

impl cfn_resources::CfnResource for CaptionSelector {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.selector_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Captions Selector Settings
///
/// The parent of this entity is CaptionSelector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptionSelectorSettings {
    ///
    /// Information about the ancillary captions to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: AncillarySourceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AncillarySourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_source_settings: Option<AncillarySourceSettings>,

    ///
    /// Information about the ARIB captions to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: AribSourceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AribSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_source_settings: Option<AribSourceSettings>,

    ///
    /// Information about the DVB Sub captions to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: DvbSubSourceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DvbSubSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_source_settings: Option<DvbSubSourceSettings>,

    ///
    /// Information about the embedded captions to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: EmbeddedSourceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmbeddedSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_source_settings: Option<EmbeddedSourceSettings>,

    ///
    /// Information about the SCTE-20 captions to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: Scte20SourceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte20SourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte20_source_settings: Option<Scte20SourceSettings>,

    ///
    /// Information about the SCTE-27 captions to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: Scte27SourceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte27SourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte27_source_settings: Option<Scte27SourceSettings>,

    ///
    /// Information about the Teletext captions to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: TeletextSourceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TeletextSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_source_settings: Option<TeletextSourceSettings>,
}

impl cfn_resources::CfnResource for CaptionSelectorSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ancillary_source_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.arib_source_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dvb_sub_source_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.embedded_source_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.scte20_source_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.scte27_source_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.teletext_source_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The input specification for this channel. It specifies the key       characteristics of CDI inputs for this channel, when those       characteristics are different from other inputs.
///
/// This entity is at the top level in the channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CdiInputSpecification {
    ///
    /// Maximum CDI input resolution
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CdiInputSpecification {
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

/// Passthrough applies no color space conversion to the output.
///
/// The parents of this entity are H264ColorSpaceSettings and       H265ColorSpaceSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColorSpacePassthroughSettings {}

impl cfn_resources::CfnResource for ColorSpacePassthroughSettings {
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

/// The DolbyVision81Settings property type specifies Property description not available. for an AWS::MediaLive::Channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DolbyVision81Settings {}

impl cfn_resources::CfnResource for DolbyVision81Settings {
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

/// The configuration of DVB NIT.
///
/// The parent of this entity is M2tsSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DvbNitSettings {
    ///
    /// The numeric value placed in the Network Information Table       (NIT).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<i64>,

    ///
    /// The network name text placed in the networkNameDescriptor inside       the Network Information Table (NIT). The maximum length is 256       characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<cfn_resources::StrVal>,

    ///
    /// The number of milliseconds between instances of this table in the       output transport stream.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i64>,
}

impl cfn_resources::CfnResource for DvbNitSettings {
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

/// A DVB Service Description Table (SDT).
///
/// The parent of this entity is M2tsSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DvbSdtSettings {
    ///
    /// Selects a method of inserting SDT information into an output       stream. The sdtFollow setting copies SDT information from input       stream to output stream. The sdtFollowIfPresent setting copies SDT       information from input stream to output stream if SDT information is       present in the input. Otherwise, it falls back on the user-defined       values. The sdtManual setting means that the user will enter the SDT       information. The sdtNone setting means that the output stream will       not contain SDT information.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputSdt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_sdt: Option<cfn_resources::StrVal>,

    ///
    /// The number of milliseconds between instances of this table in the       output transport stream.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i64>,

    ///
    /// The service name placed in the serviceDescriptor in the Service       Description Table (SDT). The maximum length is 256       characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<cfn_resources::StrVal>,

    ///
    /// The service provider name placed in the serviceDescriptor in the       Service Description Table (SDT). The maximum length is 256       characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DvbSdtSettings {
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

/// The settings for DVB Sub captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DvbSubDestinationSettings {
    ///
    /// If no explicit xPosition or yPosition is provided, setting the       alignment to centered places the captions at the bottom center of       the output. Similarly, setting a left alignment aligns captions to       the bottom left of the output. If x and y positions are specified in       conjunction with the alignment parameter, the font is justified       (either left or centered) relative to those coordinates. Selecting       "smart" justification left-justifies live subtitles and       center-justifies pre-recorded subtitles. This option is not valid       for source captions that are STL or 608/embedded. These source       settings are already pre-defined by the captions stream. All burn-in       and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the color of the rectangle behind the captions. All       burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the opacity of the background rectangle. 255 is opaque;       0 is transparent. Keeping this parameter blank is equivalent to       setting it to 0 (transparent). All burn-in and DVB-Sub font settings       must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,

    ///
    /// The external font file that is used for captions burn-in. The file       extension must be .ttf or .tte. Although you can select output fonts       for many different types of input captions, embedded, STL, and       Teletext sources use a strict grid system. Using external fonts with       these captions sources could cause an unexpected display of       proportional fonts. All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: InputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<InputLocation>,

    ///
    /// Specifies the color of the burned-in captions. This option is not       valid for source captions that are STL, 608/embedded, or Teletext.       These source settings are already pre-defined by the captions       stream. All burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the opacity of the burned-in captions. 255 is opaque; 0       is transparent. All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i64>,

    ///
    /// The font resolution in DPI (dots per inch). The default is 96 dpi.       All burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,

    ///
    /// When set to auto, fontSize scales depending on the size of the       output. Providing a positive integer specifies the exact font size       in points. All burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the font outline color. This option is not valid for       source captions that are either 608/embedded or Teletext. These       source settings are already pre-defined by the captions stream. All       burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the font outline size in pixels. This option is not       valid for source captions that are either 608/embedded or Teletext.       These source settings are already pre-defined by the captions       stream. All burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutlineSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i64>,

    ///
    /// Specifies the color of the shadow that is cast by the captions.       All burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the opacity of the shadow. 255 is opaque; 0 is       transparent. Keeping this parameter blank is equivalent to setting       it to 0 (transparent). All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i64>,

    ///
    /// Specifies the horizontal offset of the shadow relative to the       captions in pixels. A value of -2 would result in a shadow offset 2       pixels to the left. All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowXOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_xoffset: Option<i64>,

    ///
    /// Specifies the vertical offset of the shadow relative to the       captions in pixels. A value of -2 would result in a shadow offset 2       pixels above the text. All burn-in and DVB-Sub font settings must       match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowYOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_yoffset: Option<i64>,

    ///
    /// Controls whether a fixed grid size is used to generate the output       subtitles bitmap. This applies to only Teletext inputs and       DVB-Sub/Burn-in outputs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TeletextGridControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_grid_control: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the horizontal position of the captions relative to the       left side of the output in pixels. A value of 10 would result in the       captions starting 10 pixels from the left of the output. If no       explicit xPosition is provided, the horizontal captions position is       determined by the alignment parameter. This option is not valid for       source captions that are STL, 608/embedded, or Teletext. These       source settings are already pre-defined by the captions stream. All       burn-in and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "XPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xposition: Option<i64>,

    ///
    /// Specifies the vertical position of the captions relative to the       top of the output in pixels. A value of 10 would result in the       captions starting 10 pixels from the top of the output. If no       explicit yPosition is provided, the captions are positioned towards       the bottom of the output. This option is not valid for source       captions that are STL, 608/embedded, or Teletext. These source       settings are already pre-defined by the captions stream. All burn-in       and DVB-Sub font settings must match.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "YPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yposition: Option<i64>,
}

impl cfn_resources::CfnResource for DvbSubDestinationSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.font.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the DVB Sub captions to extract from the       input.
///
/// The parent of this entity is CaptionSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DvbSubSourceSettings {
    ///
    /// If you will configure a WebVTT caption description that references this caption selector, use this field to provide the language to consider when translating the image-based source to text.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OcrLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_language: Option<cfn_resources::StrVal>,

    ///
    /// When using DVB-Sub with burn-in or SMPTE-TT, use this PID for the       source content. It is unused for DVB-Sub passthrough. All DVB-Sub       content is passed through, regardless of selectors.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

impl cfn_resources::CfnResource for DvbSubSourceSettings {
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

/// The DVB Time and Date Table (TDT).
///
/// The parent of this entity is M2tsSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DvbTdtSettings {
    ///
    /// The number of milliseconds between instances of this table in the       output transport stream.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i64>,
}

impl cfn_resources::CfnResource for DvbTdtSettings {
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

/// The Eac3AtmosSettings property type specifies Property description not available. for an AWS::MediaLive::Channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Eac3AtmosSettings {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DrcLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_line: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DrcRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_rf: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeightTrim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_trim: Option<f64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "SurroundTrim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_trim: Option<f64>,
}

impl cfn_resources::CfnResource for Eac3AtmosSettings {
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

/// The settings for an EAC3 audio encode in the output.
///
/// The parent of this entity is AudioCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Eac3Settings {
    ///
    /// When set to attenuate3Db, applies a 3 dB attenuation to the       surround channels. Used only for the 3/2 coding mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttenuationControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation_control: Option<cfn_resources::StrVal>,

    ///
    /// The average bitrate in bits/second. Valid bitrates depend on the       coding mode.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,

    ///
    /// Specifies the bitstream mode (bsmod) for the emitted E-AC-3       stream. For more information, see ATSC A/52-2012 (Annex E).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<cfn_resources::StrVal>,

    ///
    /// The Dolby Digital Plus coding mode. This mode determines the       number of channels.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<cfn_resources::StrVal>,

    ///
    /// When set to enabled, activates a DC highpass filter for all input       channels.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DcFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_filter: Option<cfn_resources::StrVal>,

    ///
    /// Sets the dialnorm for the output. If blank and the input audio is       Dolby Digital Plus, dialnorm will be passed through.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,

    ///
    /// Sets the Dolby dynamic range compression profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DrcLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_line: Option<cfn_resources::StrVal>,

    ///
    /// Sets the profile for heavy Dolby dynamic range compression,       ensuring that the instantaneous signal peaks do not exceed specified       levels.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DrcRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_rf: Option<cfn_resources::StrVal>,

    ///
    /// When encoding 3/2 audio, setting to lfe enables the LFE       channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LfeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_control: Option<cfn_resources::StrVal>,

    ///
    /// When set to enabled, applies a 120Hz lowpass filter to the LFE       channel prior to encoding. Valid only with a codingMode32 coding       mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<cfn_resources::StrVal>,

    ///
    /// The Left only/Right only center mix level. Used only for the 3/2       coding mode.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoRoCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_center_mix_level: Option<f64>,

    ///
    /// The Left only/Right only surround mix level. Used only for a 3/2       coding mode.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoRoSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_surround_mix_level: Option<f64>,

    ///
    /// The Left total/Right total center mix level. Used only for a 3/2       coding mode.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LtRtCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_center_mix_level: Option<f64>,

    ///
    /// The Left total/Right total surround mix level. Used only for the       3/2 coding mode.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LtRtSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_surround_mix_level: Option<f64>,

    ///
    /// When set to followInput, encoder metadata is sourced from the DD,       DD+, or DolbyE decoder that supplies this audio data. If the audio       is not supplied from one of these streams, then the static metadata       settings are used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<cfn_resources::StrVal>,

    ///
    /// When set to whenPossible, input DD+ audio will be passed through       if it is present on the input. This detection is dynamic over the       life of the transcode. Inputs that alternate between DD+ and non-DD+       content will have a consistent DD+ output as the system alternates       between passthrough and encoding.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PassthroughControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_control: Option<cfn_resources::StrVal>,

    ///
    /// When set to shift90Degrees, applies a 90-degree phase shift to the       surround channels. Used only for a 3/2 coding mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PhaseControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_control: Option<cfn_resources::StrVal>,

    ///
    /// A stereo downmix preference. Used only for the 3/2 coding       mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StereoDownmix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stereo_downmix: Option<cfn_resources::StrVal>,

    ///
    /// When encoding 3/2 audio, sets whether an extra center back       surround channel is matrix encoded into the left and right surround       channels.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SurroundExMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_ex_mode: Option<cfn_resources::StrVal>,

    ///
    /// When encoding 2/0 audio, sets whether Dolby Surround is       matrix-encoded into the two channels.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SurroundMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_mode: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Eac3Settings {
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

/// Settings for EBU-TT captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EbuTtDDestinationSettings {
    ///
    /// Applies only if you plan to convert these source captions to EBU-TT-D or TTML in an output. Complete this field if you want to include the name of the copyright holder in the copyright metadata tag in the TTML
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyrightHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_holder: Option<cfn_resources::StrVal>,

    ///
    /// Specifies how to handle the gap between the lines (in multi-line captions). - enabled: Fill with the captions background color (as specified in the input captions). - disabled: Leave the gap unfilled.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FillLineGap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_line_gap: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the font family to include in the font data attached to the EBU-TT captions. Valid only if styleControl is set to include. If you leave this field empty, the font family is set to "monospaced". (If styleControl is set to exclude, the font family is always set to "monospaced".) You specify only the font family. All other style information (color, bold, position and so on) is copied from the input captions. The size is always set to 100% to allow the downstream player to choose the size. - Enter a list of font families, as a comma-separated list of font names, in order of preference. The name can be a font family (such as “Arial”), or a generic font family (such as “serif”), or “default” (to let the downstream player choose the font). - Leave blank to set the family to “monospace”.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the style information (font color, font position, and so on) to include in the font data that is attached to the EBU-TT captions. - include: Take the style information (font color, font position, and so on) from the source captions and include that information in the font data attached to the EBU-TT captions. This option is valid only if the source captions are Embedded or Teletext. - exclude: In the font data attached to the EBU-TT captions, set the font family to "monospaced". Do not include any other style information.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StyleControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_control: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for EbuTtDDestinationSettings {
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

/// The configuration of embedded captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EmbeddedDestinationSettings {}

impl cfn_resources::CfnResource for EmbeddedDestinationSettings {
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

/// The settings for embedded plus SCTE-20 captions in the       output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EmbeddedPlusScte20DestinationSettings {}

impl cfn_resources::CfnResource for EmbeddedPlusScte20DestinationSettings {
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

/// Information about the embedded captions to extract from the       input.
///
/// The parent of this entity is CaptionSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EmbeddedSourceSettings {
    ///
    /// If this is upconvert, 608 data is both passed through the "608       compatibility bytes" fields of the 708 wrapper as well as translated       into 708. If 708 data is present in the source content, it is       discarded.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert608_to708: Option<cfn_resources::StrVal>,

    ///
    /// Set to "auto" to handle streams with intermittent or non-aligned       SCTE-20 and embedded captions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte20Detection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte20_detection: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the 608/708 channel number within the video track from       which to extract captions. This is unused for passthrough.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source608ChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source608_channel_number: Option<i64>,

    ///
    /// This field is unused and deprecated.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source608TrackNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source608_track_number: Option<i64>,
}

impl cfn_resources::CfnResource for EmbeddedSourceSettings {
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

/// The settings for the encoding of outputs.
///
/// This entity is at the top level in the channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncoderSettings {
    ///
    /// The encoding information for output audio.
    ///
    /// Required: No
    ///
    /// Type: List of AudioDescription
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_descriptions: Option<Vec<AudioDescription>>,

    ///
    /// The settings for ad avail blanking.
    ///
    /// Required: No
    ///
    /// Type: AvailBlanking
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailBlanking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking: Option<AvailBlanking>,

    ///
    /// The configuration settings for the ad avail handling.
    ///
    /// Required: No
    ///
    /// Type: AvailConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_configuration: Option<AvailConfiguration>,

    ///
    /// The settings for the blackout slate.
    ///
    /// Required: No
    ///
    /// Type: BlackoutSlate
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlackoutSlate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blackout_slate: Option<BlackoutSlate>,

    ///
    /// The encoding information for output captions.
    ///
    /// Required: No
    ///
    /// Type: List of CaptionDescription
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_descriptions: Option<Vec<CaptionDescription>>,

    ///
    /// Settings to enable specific features.
    ///
    /// Required: No
    ///
    /// Type: FeatureActivations
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeatureActivations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_activations: Option<FeatureActivations>,

    ///
    /// The configuration settings that apply to the entire       channel.
    ///
    /// Required: No
    ///
    /// Type: GlobalConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_configuration: Option<GlobalConfiguration>,

    ///
    /// Settings to enable and configure the motion graphics overlay       feature in the channel.
    ///
    /// Required: No
    ///
    /// Type: MotionGraphicsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "MotionGraphicsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_configuration: Option<MotionGraphicsConfiguration>,

    ///
    /// The settings to configure Nielsen watermarks.
    ///
    /// Required: No
    ///
    /// Type: NielsenConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,

    ///
    /// The settings for the output groups in the channel.
    ///
    /// Required: No
    ///
    /// Type: List of OutputGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_groups: Option<Vec<OutputGroup>>,

    ///
    /// Contains settings used to acquire and adjust timecode information       from the inputs.
    ///
    /// Required: No
    ///
    /// Type: TimecodeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimecodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_config: Option<TimecodeConfig>,

    ///
    /// The encoding information for output videos.
    ///
    /// Required: No
    ///
    /// Type: List of VideoDescription
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_descriptions: Option<Vec<VideoDescription>>,
}

impl cfn_resources::CfnResource for EncoderSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.avail_blanking
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.avail_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.blackout_slate
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.feature_activations
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.global_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.motion_graphics_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.nielsen_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.timecode_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The Esam property type specifies Property description not available. for an AWS::MediaLive::Channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Esam {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcquisitionPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_point_id: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PasswordParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PoisEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pois_endpoint: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZoneIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_identity: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Esam {
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

/// Failover Condition settings. There can be multiple failover       conditions inside AutomaticInputFailoverSettings.
///
/// The parent of this entity is       AutomaticInputFailoverSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FailoverCondition {
    ///
    /// Settings for a specific failover condition.
    ///
    /// Required: No
    ///
    /// Type: FailoverConditionSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailoverConditionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_condition_settings: Option<FailoverConditionSettings>,
}

impl cfn_resources::CfnResource for FailoverCondition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.failover_condition_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for one failover condition.
///
/// The parent of this entity is FailoverCondition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FailoverConditionSettings {
    ///
    /// MediaLive will perform a failover if the specified audio selector is silent for the specified period.
    ///
    /// Required: No
    ///
    /// Type: AudioSilenceFailoverSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioSilenceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_silence_settings: Option<AudioSilenceFailoverSettings>,

    ///
    /// MediaLive will perform a failover if content is not detected in       this input for the specified period.
    ///
    /// Required: No
    ///
    /// Type: InputLossFailoverSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_settings: Option<InputLossFailoverSettings>,

    ///
    /// MediaLive will perform a failover if content is considered black for the specified period.
    ///
    /// Required: No
    ///
    /// Type: VideoBlackFailoverSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoBlackSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_black_settings: Option<VideoBlackFailoverSettings>,
}

impl cfn_resources::CfnResource for FailoverConditionSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.audio_silence_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.input_loss_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.video_black_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings to enable specific features. You can't configure these       features until you have enabled them in the channel.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FeatureActivations {
    ///
    /// Enables the Input Prepare feature. You can create Input Prepare actions in the schedule only if this feature is enabled. If you disable the feature on an existing schedule, make sure that you first delete all input prepare actions from the schedule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputPrepareScheduleActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_prepare_schedule_actions: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FeatureActivations {
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

/// The settings for FEC.
///
/// The parent of this entity is UdpOutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FecOutputSettings {
    ///
    /// The parameter D from SMPTE 2022-1. The height of the FEC       protection matrix. The number of transport stream packets per column       error correction packet. The number must be between 4 and 20,       inclusive.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_depth: Option<i64>,

    ///
    /// Enables column only or column and row-based FEC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeFec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fec: Option<cfn_resources::StrVal>,

    ///
    /// The parameter L from SMPTE 2022-1. The width of the FEC protection       matrix. Must be between 1 and 20, inclusive. If only Column FEC is       used, then larger values increase robustness. If Row FEC is used,       then this is the number of transport stream packets per row error       correction packet, and the value must be between 4 and 20,       inclusive, if includeFec is columnAndRow. If includeFec is column,       this value must be 1 to 20, inclusive.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_length: Option<i64>,
}

impl cfn_resources::CfnResource for FecOutputSettings {
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

/// Settings for the fMP4 containers.
///
/// The parent of this entity is HlsSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Fmp4HlsSettings {
    ///
    /// List all the audio groups that are used with the video output stream. Input all the audio GROUP-IDs that are associated to the video, separate by ','.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioRenditionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<cfn_resources::StrVal>,

    ///
    /// If set to passthrough, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenId3Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<cfn_resources::StrVal>,

    ///
    /// When set to passthrough, timed metadata is passed through from input to output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Fmp4HlsSettings {
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

/// Settings to configure the destination of a Frame Capture       output.
///
/// The parent of this entity is FrameCaptureGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrameCaptureCdnSettings {
    ///
    /// Sets up Amazon S3 as the destination for this Frame Capture       output.
    ///
    /// Required: No
    ///
    /// Type: FrameCaptureS3Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameCaptureS3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_s3_settings: Option<FrameCaptureS3Settings>,
}

impl cfn_resources::CfnResource for FrameCaptureCdnSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.frame_capture_s3_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for a frame capture output group.
///
/// The parent of this entity is OutputGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrameCaptureGroupSettings {
    ///
    /// The destination for the frame capture files. The destination is       either the URI for an Amazon S3 bucket and object, plus a file name       prefix (for example,       s3ssl://sportsDelivery/highlights/20180820/curling_) or the URI for       a MediaStore container, plus a file name prefix (for example,       mediastoressl://sportsDelivery/20180820/curling_). The final file       names consist of the prefix from the destination field (for example,       "curling_") + name modifier + the counter (5 digits, starting from       00001) + extension (which is always .jpg). For example,       curlingLow.00001.jpg.
    ///
    /// Required: No
    ///
    /// Type: OutputLocationRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OutputLocationRef>,

    ///
    /// Settings to configure the destination of a Frame Capture       output.
    ///
    /// Required: No
    ///
    /// Type: FrameCaptureCdnSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameCaptureCdnSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_cdn_settings: Option<FrameCaptureCdnSettings>,
}

impl cfn_resources::CfnResource for FrameCaptureGroupSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.frame_capture_cdn_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for a frame capture output in an HLS output group.
///
/// The parent of this entity is HlsSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrameCaptureHlsSettings {}

impl cfn_resources::CfnResource for FrameCaptureHlsSettings {
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

/// The frame capture output settings.
///
/// The parent of this entity is OutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrameCaptureOutputSettings {
    ///
    /// Required if the output group contains more than one output. This       modifier forms part of the output file name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FrameCaptureOutputSettings {
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

/// Sets up Amazon S3 as the destination for this Frame Capture       output.
///
/// The parent of this entity is FrameCaptureCdnSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrameCaptureS3Settings {
    ///
    /// Specify the canned ACL to apply to each S3 request. Defaults to       none.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FrameCaptureS3Settings {
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

/// The frame capture settings.
///
/// The parent of this entity is VideoCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrameCaptureSettings {
    ///
    /// The frequency, in seconds, for capturing frames for inclusion in       the output. For example, "10" means capture a frame every 10       seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptureInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_interval: Option<i64>,

    ///
    /// Unit for the frame capture interval.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptureIntervalUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_interval_units: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TimecodeBurninSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimecodeBurninSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
}

impl cfn_resources::CfnResource for FrameCaptureSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.timecode_burnin_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration settings that apply to the entire       channel.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GlobalConfiguration {
    ///
    /// The value to set the initial audio gain for the channel.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InitialAudioGain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_audio_gain: Option<i64>,

    ///
    /// Indicates the action to take when the current input completes (for       example, end-of-file). When switchAndLoopInputs is configured,       MediaLive restarts at the beginning of the first input. When "none"       is configured, MediaLive transcodes either black, a solid color, or       a user-specified slate images per the "Input Loss Behavior"       configuration until the next input switch occurs (which is       controlled through the Channel Schedule API).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputEndAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_end_action: Option<cfn_resources::StrVal>,

    ///
    /// The settings for system actions when the input is lost.
    ///
    /// Required: No
    ///
    /// Type: InputLossBehavior
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_behavior: Option<InputLossBehavior>,

    ///
    /// Indicates how MediaLive pipelines are synchronized.       PIPELINELOCKING - MediaLive attempts to synchronize the output of       each pipeline to the other. EPOCHLOCKING - MediaLive attempts to       synchronize the output of each pipeline to the Unix epoch.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputLockingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_locking_mode: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether the rate of frames emitted by the Live encoder       should be paced by its system clock (which optionally might be       locked to another source through NTP) or should be locked to the       clock of the source that is providing the input stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputTimingSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_timing_source: Option<cfn_resources::StrVal>,

    ///
    /// Adjusts the video input buffer for streams with very low video       frame rates. This is commonly set to enabled for music channels with       less than one video frame per second.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportLowFramerateInputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_low_framerate_inputs: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for GlobalConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.input_loss_behavior
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for configuring color space in an H264 video       encode.
///
/// The parent of this entity is H264Settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct H264ColorSpaceSettings {
    /// Passthrough applies no color space conversion to the output.
    ///
    /// Required: No
    ///
    /// Type: ColorSpacePassthroughSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSpacePassthroughSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,

    ///
    /// Settings to configure the handling of Rec601 color space.
    ///
    /// Required: No
    ///
    /// Type: Rec601Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rec601Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec601_settings: Option<Rec601Settings>,

    ///
    /// Settings to configure the handling of Rec709 color space.
    ///
    /// Required: No
    ///
    /// Type: Rec709Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rec709Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec709_settings: Option<Rec709Settings>,
}

impl cfn_resources::CfnResource for H264ColorSpaceSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.color_space_passthrough_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rec601_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rec709_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings to configure video filters that apply to the H264       codec.
///
/// The parent of this entity is H264Settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct H264FilterSettings {
    ///
    /// Settings for applying the temporal filter to the video.
    ///
    /// Required: No
    ///
    /// Type: TemporalFilterSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemporalFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

impl cfn_resources::CfnResource for H264FilterSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.temporal_filter_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for the H.264 codec in the output.
///
/// The parent of this entity is VideoCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct H264Settings {
    ///
    /// The adaptive quantization. This allows intra-frame quantizers to       vary to improve visual quality.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<cfn_resources::StrVal>,

    ///
    /// Indicates that AFD values will be written into the output stream.       If afdSignaling is auto, the system tries to preserve the input AFD       value (in cases where multiple AFD values are valid). If set to       fixed, the AFD value is the value configured in the fixedAfd       parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AfdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<cfn_resources::StrVal>,

    ///
    /// The average bitrate in bits/second. This is required when the rate       control mode is VBR or CBR. It isn't used for QVBR. In a Microsoft       Smooth output group, each output must have a unique value when its       bitrate is rounded down to the nearest multiple of 1000.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,

    ///
    /// The percentage of the buffer that should initially be filled (HRD       buffer model).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufFillPct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_fill_pct: Option<i64>,

    ///
    /// The size of the buffer (HRD buffer model) in bits/second.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_size: Option<i64>,

    ///
    /// Includes color space metadata in the output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<cfn_resources::StrVal>,

    ///
    /// Settings to configure the color space handling for the       video.
    ///
    /// Required: No
    ///
    /// Type: H264ColorSpaceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSpaceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<H264ColorSpaceSettings>,

    ///
    /// The entropy encoding mode. Use cabac (must be in Main or High       profile) or cavlc.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntropyEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy_encoding: Option<cfn_resources::StrVal>,

    ///
    /// Optional filters that you can apply to an encode.
    ///
    /// Required: No
    ///
    /// Type: H264FilterSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<H264FilterSettings>,

    ///
    /// A four-bit AFD value to write on all frames of video in the output       stream. Valid only when afdSignaling is set to Fixed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<cfn_resources::StrVal>,

    ///
    /// If set to enabled, adjusts the quantization within each frame to       reduce flicker or pop on I-frames.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlickerAq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_aq: Option<cfn_resources::StrVal>,

    ///
    /// This setting applies only when scan type is "interlaced." It controls whether coding is performed on a field basis or on a frame basis. (When the video is progressive, the coding is always performed on a frame basis.) enabled: Force MediaLive to code on a field basis, so that odd and even sets of fields are coded separately. disabled: Code the two sets of fields separately (on a field basis) or together (on a frame basis using PAFF), depending on what is most appropriate for the content.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForceFieldPictures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_field_pictures: Option<cfn_resources::StrVal>,

    ///
    /// Indicates how the output video frame rate is specified. If you       select "specified," the output video frame rate is determined by       framerateNumerator and framerateDenominator. If you select       "initializeFromSource," the output video frame rate is set equal to       the input video frame rate of the first input.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<cfn_resources::StrVal>,

    ///
    /// The frame rate denominator.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,

    ///
    /// The frame rate numerator. The frame rate is a fraction, for       example, 24000/1001 = 23.976 fps.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,

    ///
    /// If enabled, uses reference B frames for GOP structures that have B       frames > 1.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopBReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_breference: Option<cfn_resources::StrVal>,

    ///
    /// The frequency of closed GOPs. In streaming applications, we       recommend that you set this to 1 so that a decoder joining       mid-stream will receive an IDR frame as quickly as possible. Setting       this value to 0 will break output segmenting.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,

    ///
    /// The number of B-frames between reference frames.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopNumBFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_num_bframes: Option<i64>,

    ///
    /// The GOP size (keyframe interval) in units of either frames or       seconds per gopSizeUnits. The value must be greater than       zero.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,

    ///
    /// Indicates if the gopSize is specified in frames or seconds. If       seconds, the system converts the gopSize into a frame count at       runtime.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<cfn_resources::StrVal>,

    ///
    /// The H.264 level.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<cfn_resources::StrVal>,

    ///
    /// The amount of lookahead. A value of low can decrease latency and       memory usage, while high can produce better quality for certain       content.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LookAheadRateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_ahead_rate_control: Option<cfn_resources::StrVal>,

    ///
    /// For QVBR: See the tooltip for Quality level. For VBR: Set the       maximum bitrate in order to accommodate expected spikes in the       complexity of the video.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,

    ///
    /// Meaningful only if sceneChangeDetect is set to enabled. This       setting enforces separation between repeated (cadence) I-frames and       I-frames inserted by Scene Change Detection. If a scene change       I-frame is within I-interval frames of a cadence I-frame, the GOP is       shrunk or stretched to the scene change I-frame. GOP stretch       requires enabling lookahead as well as setting the I-interval. The       normal cadence resumes for the next GOP. Note that the maximum GOP       stretch = GOP size + Min-I-interval - 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_iinterval: Option<i64>,

    ///
    /// The number of reference frames to use. The encoder might use more       than requested if you use B-frames or interlaced encoding.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRefFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ref_frames: Option<i64>,

    ///
    /// Indicates how the output pixel aspect ratio is specified. If       "specified" is selected, the output video pixel aspect ratio is       determined by parNumerator and parDenominator. If       "initializeFromSource" is selected, the output pixels aspect ratio       will be set equal to the input video pixel aspect ratio of the first       input.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<cfn_resources::StrVal>,

    ///
    /// The Pixel Aspect Ratio denominator.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,

    ///
    /// The Pixel Aspect Ratio numerator.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,

    ///
    /// An H.264 profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<cfn_resources::StrVal>,

    ///
    /// Leave as STANDARD_QUALITY or choose a different value (which might result in additional costs to run the channel). - ENHANCED_QUALITY: Produces a slightly better video quality without an increase in the bitrate. Has an effect only when the Rate control mode is QVBR or CBR. If this channel is in a MediaLive multiplex, the value must be ENHANCED_QUALITY. - STANDARD_QUALITY: Valid for any Rate control mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_level: Option<cfn_resources::StrVal>,

    ///
    /// Controls the target quality for the video encode. This applies       only when the rate control mode is QVBR. Set values for the QVBR       quality level field and Max bitrate field that suit your most       important viewing devices. Recommended values are: - Primary screen:       Quality level: 8 to 10. Max bitrate: 4M - PC or tablet: Quality       level: 7. Max bitrate: 1.5M to 3M - Smartphone: Quality level: 6.       Max bitrate: 1M to 1.5M.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "QvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,

    ///
    /// The rate control mode. QVBR: The quality will match the specified       quality level except when it is constrained by the maximum bitrate.       We recommend this if you or your viewers pay for bandwidth. VBR: The       quality and bitrate vary, depending on the video complexity. We       recommend this instead of QVBR if you want to maintain a specific       average bitrate over the duration of the channel. CBR: The quality       varies, depending on the video complexity. We recommend this only if       you distribute your assets to devices that can't handle variable       bitrates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<cfn_resources::StrVal>,

    ///
    /// Sets the scan type of the output to progressive or top-field-first       interlaced.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<cfn_resources::StrVal>,

    ///
    /// The scene change detection. On: inserts I-frames when the scene       change is detected. Off: does not force an I-frame when the scene       change is detected.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<cfn_resources::StrVal>,

    ///
    /// The number of slices per picture. The number must be less than or       equal to the number of macroblock rows for progressive pictures, and       less than or equal to half the number of macroblock rows for       interlaced pictures. This field is optional. If you don't specify a       value, MediaLive chooses the number of slices based on the encode       resolution.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,

    ///
    /// Softness. Selects a quantizer matrix. Larger values reduce       high-frequency content in the encoded image.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,

    ///
    /// If set to enabled, adjusts quantization within each frame based on       the spatial variation of content complexity.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpatialAq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_aq: Option<cfn_resources::StrVal>,

    ///
    /// If set to fixed, uses gopNumBFrames B-frames per sub-GOP. If set       to dynamic, optimizes the number of B-frames used for each sub-GOP       to improve visual quality.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubgopLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgop_length: Option<cfn_resources::StrVal>,

    ///
    /// Produces a bitstream that is compliant with SMPTE RP-2027.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<cfn_resources::StrVal>,

    ///
    /// If set to enabled, adjusts quantization within each frame based on       the temporal variation of content complexity.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemporalAq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_aq: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TimecodeBurninSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimecodeBurninSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,

    ///
    /// Determines how timecodes should be inserted into the video       elementary stream. disabled: don't include timecodes. picTimingSei:       pass through picture timing SEI messages from the source specified       in Timecode Config.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimecodeInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for H264Settings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.color_space_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.filter_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.timecode_burnin_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// H265 Color Space Settings
///
/// The parent of this entity is H265Settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct H265ColorSpaceSettings {
    ///
    /// Passthrough applies no color space conversion to the output.
    ///
    /// Required: No
    ///
    /// Type: ColorSpacePassthroughSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSpacePassthroughSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DolbyVision81Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DolbyVision81Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dolby_vision81_settings: Option<DolbyVision81Settings>,

    ///
    /// Settings to configure the handling of HDR10 color space.
    ///
    /// Required: No
    ///
    /// Type: Hdr10Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hdr10Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr10_settings: Option<Hdr10Settings>,

    ///
    /// Settings to configure the handling of Rec601 color space.
    ///
    /// Required: No
    ///
    /// Type: Rec601Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rec601Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec601_settings: Option<Rec601Settings>,

    ///
    /// Settings to configure the handling of Rec709 color space.
    ///
    /// Required: No
    ///
    /// Type: Rec709Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rec709Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec709_settings: Option<Rec709Settings>,
}

impl cfn_resources::CfnResource for H265ColorSpaceSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.color_space_passthrough_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dolby_vision81_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hdr10_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rec601_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rec709_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings to configure video filters that apply to the H265       codec.
///
/// The parent of this entity is H265Settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct H265FilterSettings {
    /// Settings for applying the temporal filter to the video.
    ///
    /// Required: No
    ///
    /// Type: TemporalFilterSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemporalFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

impl cfn_resources::CfnResource for H265FilterSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.temporal_filter_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// H265 Settings
///
/// The parent of this entity is VideoCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct H265Settings {
    ///
    /// Adaptive quantization. Allows intra-frame quantizers to vary to improve visual quality.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<cfn_resources::StrVal>,

    ///
    /// Indicates that AFD values will be written into the output stream. If afdSignaling is "auto", the system will try to preserve the input AFD value (in cases where multiple AFD values are valid). If set to "fixed", the AFD value will be the value configured in the fixedAfd parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AfdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<cfn_resources::StrVal>,

    ///
    /// Whether or not EML should insert an Alternative Transfer Function SEI message to support backwards compatibility with non-HDR decoders and displays.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlternativeTransferFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_transfer_function: Option<cfn_resources::StrVal>,

    ///
    /// Average bitrate in bits/second. Required when the rate control mode is VBR or CBR. Not used for QVBR. In an MS Smooth output group, each output must have a unique value when its bitrate is rounded down to the nearest multiple of 1000.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,

    ///
    /// Size of buffer (HRD buffer model) in bits.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_size: Option<i64>,

    ///
    /// Includes colorspace metadata in the output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<cfn_resources::StrVal>,

    ///
    /// Color Space settings
    ///
    /// Required: No
    ///
    /// Type: H265ColorSpaceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSpaceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<H265ColorSpaceSettings>,

    ///
    /// Optional filters that you can apply to an encode.
    ///
    /// Required: No
    ///
    /// Type: H265FilterSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<H265FilterSettings>,

    ///
    /// Four bit AFD value to write on all frames of video in the output stream. Only valid when afdSignaling is set to 'Fixed'.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<cfn_resources::StrVal>,

    ///
    /// If set to enabled, adjust quantization within each frame to reduce flicker or 'pop' on I-frames.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlickerAq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_aq: Option<cfn_resources::StrVal>,

    ///
    /// Framerate denominator.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,

    ///
    /// Framerate numerator - framerate is a fraction, e.g. 24000 / 1001 = 23.976 fps.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,

    ///
    /// Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,

    ///
    /// GOP size (keyframe interval) in units of either frames or seconds per gopSizeUnits. If gopSizeUnits is frames, gopSize must be an integer and must be greater than or equal to 1. If gopSizeUnits is seconds, gopSize must be greater than 0, but need not be an integer.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,

    ///
    /// Indicates if the gopSize is specified in frames or seconds. If seconds the system will convert the gopSize into a frame count at run time.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<cfn_resources::StrVal>,

    ///
    /// H.265 Level.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<cfn_resources::StrVal>,

    ///
    /// Amount of lookahead. A value of low can decrease latency and memory usage, while high can produce better quality for certain content.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LookAheadRateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_ahead_rate_control: Option<cfn_resources::StrVal>,

    ///
    /// For QVBR: See the tooltip for Quality level
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,

    ///
    /// Only meaningful if sceneChangeDetect is set to enabled. Defaults to 5 if multiplex rate control is used. Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_iinterval: Option<i64>,

    ///
    /// Pixel Aspect Ratio denominator.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,

    ///
    /// Pixel Aspect Ratio numerator.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,

    ///
    /// H.265 Profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<cfn_resources::StrVal>,

    ///
    /// Controls the target quality for the video encode. Applies only when the rate control mode is QVBR. Set values for the QVBR quality level field and Max bitrate field that suit your most important viewing devices. Recommended values are: - Primary screen: Quality level: 8 to 10. Max bitrate: 4M - PC or tablet: Quality level: 7. Max bitrate: 1.5M to 3M - Smartphone: Quality level: 6. Max bitrate: 1M to 1.5M
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "QvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,

    ///
    /// Rate control mode. QVBR: Quality will match the specified quality level except when it is constrained by the maximum bitrate. Recommended if you or your viewers pay for bandwidth. CBR: Quality varies, depending on the video complexity. Recommended only if you distribute your assets to devices that cannot handle variable bitrates. Multiplex: This rate control mode is only supported (and is required) when the video is being delivered to a MediaLive Multiplex in which case the rate control configuration is controlled by the properties within the Multiplex Program.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<cfn_resources::StrVal>,

    ///
    /// Sets the scan type of the output to progressive or top-field-first interlaced.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<cfn_resources::StrVal>,

    ///
    /// Scene change detection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<cfn_resources::StrVal>,

    ///
    /// Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures. This field is optional; when no value is specified the encoder will choose the number of slices based on encode resolution.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,

    ///
    /// H.265 Tier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TimecodeBurninSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimecodeBurninSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,

    ///
    /// Determines how timecodes should be inserted into the video elementary stream. - 'disabled': Do not include timecodes - 'picTimingSei': Pass through picture timing SEI messages from the source specified in Timecode Config
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimecodeInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for H265Settings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.color_space_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.filter_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.timecode_burnin_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Hdr10 Settings
///
/// The parents of this entity are H265ColorSpaceSettings (for color       space settings in the output) and VideoSelectorColorSpaceSettings       (for color space settings in the input).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Hdr10Settings {
    ///
    /// Maximum Content Light Level An integer metadata value defining the maximum light level, in nits, of any single pixel within an encoded HDR video stream or file.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cll: Option<i64>,

    ///
    /// Maximum Frame Average Light Level An integer metadata value defining the maximum average light level, in nits, for any single frame within an encoded HDR video stream or file.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fall: Option<i64>,
}

impl cfn_resources::CfnResource for Hdr10Settings {
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

/// The Akamai settings in an HLS output.
///
/// The parent of this entity is HlsCdnSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsAkamaiSettings {
    ///
    /// The number of seconds to wait before retrying a connection to the       CDN if the connection is lost.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,

    ///
    /// The size, in seconds, of the file cache for streaming       outputs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,

    ///
    /// Specifies whether to use chunked transfer encoding to Akamai. To       enable this feature, contact Akamai.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpTransferMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_transfer_mode: Option<cfn_resources::StrVal>,

    ///
    /// The number of retry attempts that will be made before the channel       is put into an error state.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,

    ///
    /// If a streaming output fails, the number of seconds to wait until a       restart is initiated. A value of 0 means never restart.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,

    ///
    /// The salt for authenticated Akamai.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<cfn_resources::StrVal>,

    ///
    /// The token parameter for authenticated Akamai. If this is not       specified, _gda_ is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HlsAkamaiSettings {
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

/// The configuration of HLS Basic Put Settings.
///
/// The parent of this entity is HlsCdnSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsBasicPutSettings {
    ///
    /// The number of seconds to wait before retrying a connection to the       CDN if the connection is lost.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,

    ///
    /// The size, in seconds, of the file cache for streaming       outputs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,

    ///
    /// The number of retry attempts that MediaLive makes before the       channel is put into an error state.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,

    ///
    /// If a streaming output fails, the number of seconds to wait until a       restart is initiated. A value of 0 means never restart.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
}

impl cfn_resources::CfnResource for HlsBasicPutSettings {
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

/// The settings for the CDN of an HLS output.
///
/// The parent of this entity is HlsGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsCdnSettings {
    ///
    /// Sets up Akamai as the downstream system for the HLS output       group.
    ///
    /// Required: No
    ///
    /// Type: HlsAkamaiSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsAkamaiSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_akamai_settings: Option<HlsAkamaiSettings>,

    ///
    /// The settings for Basic Put for the HLS output.
    ///
    /// Required: No
    ///
    /// Type: HlsBasicPutSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsBasicPutSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_basic_put_settings: Option<HlsBasicPutSettings>,

    ///
    /// Sets up MediaStore as the destination for the HLS output.
    ///
    /// Required: No
    ///
    /// Type: HlsMediaStoreSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsMediaStoreSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_media_store_settings: Option<HlsMediaStoreSettings>,

    ///
    /// Sets up Amazon S3 as the destination for this HLS output.
    ///
    /// Required: No
    ///
    /// Type: HlsS3Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsS3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_s3_settings: Option<HlsS3Settings>,

    ///
    /// The settings for Web VTT captions in the HLS output group.
    ///
    /// The parent of this entity is HlsGroupSettings.
    ///
    /// Required: No
    ///
    /// Type: HlsWebdavSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsWebdavSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_webdav_settings: Option<HlsWebdavSettings>,
}

impl cfn_resources::CfnResource for HlsCdnSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.hls_akamai_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_basic_put_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_media_store_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_s3_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_webdav_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for an HLS output group.
///
/// The parent of this entity is OutputGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsGroupSettings {
    ///
    /// Chooses one or more ad marker types to pass SCTE35 signals through       to this group of Apple HLS outputs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,

    ///
    /// A partial URI prefix that will be prepended to each output in the       media .m3u8 file. The partial URI prefix can be used if the base       manifest is delivered from a different URL than the main .m3u8       file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseUrlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_content: Option<cfn_resources::StrVal>,

    ///
    /// Optional. One value per output group. This field is required only if you are completing Base URL content A, and the downstream system has notified you that the media files for pipeline 1 of all outputs are in a location different from the media files for pipeline 0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseUrlContent1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_content1: Option<cfn_resources::StrVal>,

    ///
    /// A partial URI prefix that will be prepended to each output in the       media .m3u8 file. The partial URI prefix can be used if the base       manifest is delivered from a different URL than the main .m3u8       file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseUrlManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_manifest: Option<cfn_resources::StrVal>,

    ///
    /// Optional. One value per output group. Complete this field only if you are completing Base URL manifest A, and the downstream system has notified you that the child manifest files for pipeline 1 of all outputs are in a location different from the child manifest files for pipeline 0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseUrlManifest1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_manifest1: Option<cfn_resources::StrVal>,

    ///
    /// A mapping of up to 4 captions channels to captions languages. This       is meaningful only if captionLanguageSetting is set to       "insert."
    ///
    /// Required: No
    ///
    /// Type: List of CaptionLanguageMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptionLanguageMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_mappings: Option<Vec<CaptionLanguageMapping>>,

    ///
    /// Applies only to 608 embedded output captions. Insert: Include       CLOSED-CAPTIONS lines in the manifest. Specify at least one language       in the CC1 Language Code field. One CLOSED-CAPTION line is added for       each Language Code that you specify. Make sure to specify the       languages in the order in which they appear in the original source       (if the source is embedded format) or the order of the captions       selectors (if the source is other than embedded). Otherwise,       languages in the manifest will not match properly with the output       captions. None: Include the CLOSED-CAPTIONS=NONE line in the       manifest. Omit: Omit any CLOSED-CAPTIONS line from the       manifest.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptionLanguageSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_setting: Option<cfn_resources::StrVal>,

    ///
    /// When set to "disabled," sets the #EXT-X-ALLOW-CACHE:no tag in the       manifest, which prevents clients from saving media segments for       later replay.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<cfn_resources::StrVal>,

    ///
    /// The specification to use (RFC-6381 or the default RFC-4281) during       m3u8 playlist generation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodecSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<cfn_resources::StrVal>,

    ///
    /// Used with encryptionType. This is a 128-bit, 16-byte hex value       that is represented by a 32-character text string. If ivSource is       set to "explicit," this parameter is required and is used as the IV       for encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantIv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_iv: Option<cfn_resources::StrVal>,

    ///
    /// A directory or HTTP destination for the HLS segments, manifest       files, and encryption keys (if enabled).
    ///
    /// Required: No
    ///
    /// Type: OutputLocationRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OutputLocationRef>,

    ///
    /// Places segments in subdirectories.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DirectoryStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_structure: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether to insert EXT-X-DISCONTINUITY tags in the HLS child manifests for this output group. Typically, choose Insert because these tags are required in the manifest (according to the HLS specification) and serve an important purpose. Choose Never Insert only if the downstream system is doing real-time failover (without using the MediaLive automatic failover feature) and only if that downstream system has advised you to exclude the tags.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DiscontinuityTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discontinuity_tags: Option<cfn_resources::StrVal>,

    ///
    /// Encrypts the segments with the specified encryption scheme.       Exclude this parameter if you don't want encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<cfn_resources::StrVal>,

    ///
    /// The parameters that control interactions with the CDN.
    ///
    /// Required: No
    ///
    /// Type: HlsCdnSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsCdnSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_cdn_settings: Option<HlsCdnSettings>,

    ///
    /// State of HLS ID3 Segment Tagging
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsId3SegmentTagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_id3_segment_tagging: Option<cfn_resources::StrVal>,

    ///
    /// DISABLED: Don't create an I-frame-only manifest, but do create the       master and media manifests (according to the Output Selection       field). STANDARD: Create an I-frame-only manifest for each output       that contains video, as well as the other manifests (according to       the Output Selection field). The I-frame manifest contains a       #EXT-X-I-FRAMES-ONLY tag to indicate it is I-frame only, and one or       more #EXT-X-BYTERANGE entries identifying the I-frame position. For       example, #EXT-X-BYTERANGE:160364@1461888".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IFrameOnlyPlaylists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe_only_playlists: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether to include the final (incomplete) segment in the media output when the pipeline stops producing output because of a channel stop, a channel pause or a loss of input to the pipeline. Auto means that MediaLive decides whether to include the final segment, depending on the channel class and the types of output groups. Suppress means to never include the incomplete segment. We recommend you choose Auto and let MediaLive control the behavior.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncompleteSegmentBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_segment_behavior: Option<cfn_resources::StrVal>,

    ///
    /// Applies only if the Mode field is LIVE. Specifies the maximum       number of segments in the media manifest file. After this maximum,       older segments are removed from the media manifest. This number must       be less than or equal to the Keep Segments field.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexNSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_nsegments: Option<i64>,

    ///
    /// A parameter that controls output group behavior on an input       loss.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<cfn_resources::StrVal>,

    ///
    /// Used with encryptionType. The IV (initialization vector) is a       128-bit number used in conjunction with the key for encrypting       blocks. If set to "include," the IV is listed in the manifest.       Otherwise, the IV is not in the manifest.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IvInManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_in_manifest: Option<cfn_resources::StrVal>,

    ///
    /// Used with encryptionType. The IV (initialization vector) is a       128-bit number used in conjunction with the key for encrypting       blocks. If this setting is "followsSegmentNumber," it causes the IV       to change every segment (to match the segment number). If this is       set to "explicit," you must enter a constantIv value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IvSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_source: Option<cfn_resources::StrVal>,

    ///
    /// Applies only if the Mode field is LIVE. Specifies the number of       media segments (.ts files) to retain in the destination       directory.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeepSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_segments: Option<i64>,

    ///
    /// Specifies how the key is represented in the resource identified by       the URI. If the parameter is absent, an implicit value of "identity"       is used. A reverse DNS string can also be specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format: Option<cfn_resources::StrVal>,

    ///
    /// Either a single positive integer version value or a       slash-delimited list of version values (1/2/3).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyFormatVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format_versions: Option<cfn_resources::StrVal>,

    ///
    /// The key provider settings.
    ///
    /// Required: No
    ///
    /// Type: KeyProviderSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyProviderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_provider_settings: Option<KeyProviderSettings>,

    ///
    /// When set to gzip, compresses HLS playlist.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether the output manifest should use a floating point       or integer values for segment duration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestDurationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<cfn_resources::StrVal>,

    ///
    /// When set, minimumSegmentLength is enforced by looking ahead and       back within the specified range for a nearby avail and extending the       segment size if needed.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_segment_length: Option<i64>,

    ///
    /// If "vod," all segments are indexed and kept permanently in the       destination and manifest. If "live," only the number segments       specified in keepSegments and indexNSegments are kept. Newer       segments replace older segments, which might prevent players from       rewinding all the way to the beginning of the channel. VOD mode uses       HLS EXT-X-PLAYLIST-TYPE of EVENT while the channel is running,       converting it to a "VOD" type manifest on completion of the       stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<cfn_resources::StrVal>,

    ///
    /// MANIFESTSANDSEGMENTS: Generates manifests (the master manifest, if       applicable, and media manifests) for this output group.       SEGMENTSONLY: Doesn't generate any manifests for this output       group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<cfn_resources::StrVal>,

    ///
    /// Includes or excludes the EXT-X-PROGRAM-DATE-TIME tag in .m3u8       manifest files. The value is calculated as follows: Either the       program date and time are initialized using the input timecode       source, or the time is initialized using the input timecode source       and the date is initialized using the timestampOffset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramDateTimeClock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_clock: Option<cfn_resources::StrVal>,

    ///
    /// The period of insertion of the EXT-X-PROGRAM-DATE-TIME entry, in       seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramDateTimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_period: Option<i64>,

    ///
    /// ENABLED: The master manifest (.m3u8 file) for each pipeline       includes information about both pipelines: first its own media       files, then the media files of the other pipeline. This feature       allows a playout device that supports stale manifest detection to       switch from one manifest to the other, when the current manifest       seems to be stale. There are still two destinations and two master       manifests, but both master manifests reference the media files from       both pipelines. DISABLED: The master manifest (.m3u8 file) for each       pipeline includes information about its own pipeline only. For an       HLS output group with MediaPackage as the destination, the DISABLED       behavior is always followed. MediaPackage regenerates the manifests       it serves to players, so a redundant manifest from MediaLive is       irrelevant.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedundantManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_manifest: Option<cfn_resources::StrVal>,

    ///
    /// The length of the MPEG-2 Transport Stream segments to create, in       seconds. Note that segments will end on the next keyframe after this       number of seconds, so the actual segment length might be       longer.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,

    ///
    /// useInputSegmentation has been deprecated. The configured segment       size is always used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_mode: Option<cfn_resources::StrVal>,

    ///
    /// The number of segments to write to a subdirectory before starting       a new one. For this setting to have an effect, directoryStructure       must be subdirectoryPerStream.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentsPerSubdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_per_subdirectory: Option<i64>,

    ///
    /// The include or exclude RESOLUTION attribute for a video in the       EXT-X-STREAM-INF tag of a variant manifest.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamInfResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_inf_resolution: Option<cfn_resources::StrVal>,

    ///
    /// Indicates the ID3 frame that has the timecode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataId3Frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_frame: Option<cfn_resources::StrVal>,

    ///
    /// The timed metadata interval, in seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataId3Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_period: Option<i64>,

    ///
    /// Provides an extra millisecond delta offset to fine tune the       timestamps.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampDeltaMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_delta_milliseconds: Option<i64>,

    ///
    /// SEGMENTEDFILES: Emits the program as segments -multiple .ts media       files. SINGLEFILE: Applies only if the Mode field is VOD. Emits the       program as a single .ts media file. The media manifest includes       #EXT-X-BYTERANGE tags to index segments for playback. A typical use       for this value is when sending the output to AWS Elemental       MediaConvert, which can accept only a single media file. Playback       while the channel is running is not guaranteed due to HTTP server       caching.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TsFileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_file_mode: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HlsGroupSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_cdn_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.key_provider_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about how to connect to the upstream system.
///
/// The parent of this entity is NetworkInputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsInputSettings {
    ///
    /// When specified, the HLS stream with the m3u8 bandwidth that most       closely matches this value is chosen. Otherwise, the highest       bandwidth stream in the m3u8 is chosen. The bitrate is specified in       bits per second, as in an HLS manifest.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,

    ///
    /// When specified, reading of the HLS input begins this many buffer       segments from the end (most recently written segment). When not       specified, the HLS input begins with the first segment specified in       the m3u8.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_segments: Option<i64>,

    ///
    /// The number of consecutive times that attempts to read a manifest       or segment must fail before the input is considered       unavailable.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,

    ///
    /// The number of seconds between retries when an attempt to read a       manifest or segment fails.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,

    ///
    /// Identifies the source for the SCTE-35 messages that MediaLive will ingest. Messages can be ingested from the content segments (in the stream) or from tags in the playlist (the HLS manifest). MediaLive ignores SCTE-35 information in the source that is not selected.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_source: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HlsInputSettings {
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

/// The configuration of a MediaStore container as the destination for       an HLS output.
///
/// The parent of this entity is HlsCdnSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsMediaStoreSettings {
    ///
    /// The number of seconds to wait before retrying a connection to the       CDN if the connection is lost.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,

    ///
    /// The size, in seconds, of the file cache for streaming       outputs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,

    ///
    /// When set to temporal, output files are stored in non-persistent       memory for faster reading and writing.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MediaStoreStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_store_storage_class: Option<cfn_resources::StrVal>,

    ///
    /// The number of retry attempts that are made before the channel is       put into an error state.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,

    ///
    /// If a streaming output fails, the number of seconds to wait until a       restart is initiated. A value of 0 means never restart.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
}

impl cfn_resources::CfnResource for HlsMediaStoreSettings {
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

/// The settings for an HLS output.
///
/// The parent of this entity is OutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsOutputSettings {
    ///
    /// Only applicable when this output is referencing an H.265 video description. Specifies whether MP4 segments should be packaged as HEV1 or HVC1.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "H265PackagingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_packaging_type: Option<cfn_resources::StrVal>,

    ///
    /// The settings regarding the underlying stream. These settings are       different for audio-only outputs.
    ///
    /// Required: No
    ///
    /// Type: HlsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_settings: Option<HlsSettings>,

    ///
    /// A string that is concatenated to the end of the destination file       name. Accepts \"Format       Identifiers\":#formatIdentifierParameters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<cfn_resources::StrVal>,

    ///
    /// A string that is concatenated to the end of segment file       names.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_modifier: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HlsOutputSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.hls_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Sets up Amazon S3 as the destination for this HLS output.
///
/// The parent of this entity is HlsCdnSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsS3Settings {
    ///
    /// Specify the canned ACL to apply to each S3 request. Defaults to none.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HlsS3Settings {
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

/// The settings for an HLS output.
///
/// The parent of this entity is HlsOutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsSettings {
    ///
    /// The settings for an audio-only output.
    ///
    /// Required: No
    ///
    /// Type: AudioOnlyHlsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioOnlyHlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_hls_settings: Option<AudioOnlyHlsSettings>,

    ///
    /// The settings for an fMP4 container.
    ///
    /// Required: No
    ///
    /// Type: Fmp4HlsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Fmp4HlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmp4_hls_settings: Option<Fmp4HlsSettings>,

    ///
    /// Settings for a frame capture output in an HLS output group.
    ///
    /// Required: No
    ///
    /// Type: FrameCaptureHlsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameCaptureHlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_hls_settings: Option<FrameCaptureHlsSettings>,

    ///
    /// The settings for a standard output (an output that is not       audio-only).
    ///
    /// Required: No
    ///
    /// Type: StandardHlsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "StandardHlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_hls_settings: Option<StandardHlsSettings>,
}

impl cfn_resources::CfnResource for HlsSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.audio_only_hls_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.fmp4_hls_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.frame_capture_hls_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.standard_hls_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration of a WebDav server as the downstream system for       an HLS output.
///
/// The parent of this entity is HlsCdnSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HlsWebdavSettings {
    ///
    /// The number of seconds to wait before retrying a connection to the       CDN if the connection is lost.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,

    ///
    /// The size, in seconds, of the file cache for streaming       outputs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,

    ///
    /// Specifies whether to use chunked transfer encoding to       WebDAV.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpTransferMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_transfer_mode: Option<cfn_resources::StrVal>,

    ///
    /// The number of retry attempts that are made before the channel is       put into an error state.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,

    ///
    /// If a streaming output fails, the number of seconds to wait until a       restart is initiated. A value of 0 means never restart.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
}

impl cfn_resources::CfnResource for HlsWebdavSettings {
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

/// Settings to configure the motion graphics overlay to use an HTML       asset.
///
/// The parent of this entity is MotionGraphicsSetting.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HtmlMotionGraphicsSettings {}

impl cfn_resources::CfnResource for HtmlMotionGraphicsSettings {
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

/// An input to attach to this channel.
///
/// This entity is at the top level in the channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputAttachment {
    ///
    /// Settings to implement automatic input failover in this input.
    ///
    /// Required: No
    ///
    /// Type: AutomaticInputFailoverSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomaticInputFailoverSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_input_failover_settings: Option<AutomaticInputFailoverSettings>,

    ///
    /// A name for the attachment. This is required if you want to use       this input in an input switch action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputAttachmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachment_name: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the input to attach.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InputId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<cfn_resources::StrVal>,

    ///
    /// Information about the content to extract from the input and about       the general handling of the content.
    ///
    /// Required: No
    ///
    /// Type: InputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_settings: Option<InputSettings>,
}

impl cfn_resources::CfnResource for InputAttachment {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.automatic_input_failover_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.input_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The setting to remix the audio.
///
/// The parent of this entity is AudioChannelMappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputChannelLevel {
    ///
    /// The remixing value. Units are in dB, and acceptable values are       within the range from -60 (mute) to 6 dB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Gain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gain: Option<i64>,

    ///
    /// The index of the input channel that is used as a source.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_channel: Option<i64>,
}

impl cfn_resources::CfnResource for InputChannelLevel {
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

/// The input location.
///
/// The parent of this entity is InputLossBehavior.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputLocation {
    ///
    /// The password parameter that holds the password for accessing the       downstream system. This applies only if the downstream system       requires credentials.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PasswordParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<cfn_resources::StrVal>,

    ///
    /// The URI should be a path to a file that is accessible to the Live       system (for example, an http:// URI) depending on the output type.       For example, an RTMP destination should have a URI similar to       rtmp://fmsserver/live.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<cfn_resources::StrVal>,

    ///
    /// The user name to connect to the downstream system. This applies       only if the downstream system requires credentials.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for InputLocation {
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

/// The configuration of channel behavior when the input is       lost.
///
/// The parent of this entity is GlobalConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputLossBehavior {
    ///
    /// On input loss, the number of milliseconds to substitute black into       the output before switching to the frame specified by       inputLossImageType. A value x, where 0 <= x <= 1,000,000 and a       value of 1,000,000, is interpreted as infinite.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlackFrameMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_frame_msec: Option<i64>,

    ///
    /// When the input loss image type is "color," this field specifies       the color to use. Value: 6 hex characters that represent the values       of RGB.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossImageColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_color: Option<cfn_resources::StrVal>,

    ///
    /// When the input loss image type is "slate," these fields specify       the parameters for accessing the slate.
    ///
    /// Required: No
    ///
    /// Type: InputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossImageSlate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_slate: Option<InputLocation>,

    ///
    /// Indicates whether to substitute a solid color or a slate into the       output after the input loss exceeds blackFrameMsec.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossImageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_type: Option<cfn_resources::StrVal>,

    ///
    /// On input loss, the number of milliseconds to repeat the previous       picture before substituting black into the output. A value x, where       0 <= x <= 1,000,000 and a value of 1,000,000, is interpreted       as infinite.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepeatFrameMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_frame_msec: Option<i64>,
}

impl cfn_resources::CfnResource for InputLossBehavior {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.input_loss_image_slate
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// MediaLive will perform a failover if content is not detected in       this input for the specified period.
///
/// The parent of this entity is FailoverConditionSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputLossFailoverSettings {
    ///
    /// The amount of time (in milliseconds) that no input is detected. After that time, an input failover will occur.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossThresholdMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_threshold_msec: Option<i64>,
}

impl cfn_resources::CfnResource for InputLossFailoverSettings {
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

/// Information about extracting content from the input and about       handling the content.
///
/// The parent of this entity is InputAttachment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputSettings {
    ///
    /// Information about the specific audio to extract from the       input.
    ///
    /// The parent of this entity is InputSettings.
    ///
    /// Required: No
    ///
    /// Type: List of AudioSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selectors: Option<Vec<AudioSelector>>,

    ///
    /// Information about the specific captions to extract from the       input.
    ///
    /// Required: No
    ///
    /// Type: List of CaptionSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptionSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selectors: Option<Vec<CaptionSelector>>,

    ///
    /// Enables or disables the deblock filter when filtering.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeblockFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<cfn_resources::StrVal>,

    ///
    /// Enables or disables the denoise filter when filtering.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DenoiseFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<cfn_resources::StrVal>,

    ///
    /// Adjusts the magnitude of filtering from 1 (minimal) to 5       (strongest).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i64>,

    ///
    /// Turns on the filter for this input. MPEG-2 inputs have the       deblocking filter enabled by default. 1) auto - filtering is applied       depending on input type/quality 2) disabled - no filtering is       applied to the input 3) forced - filtering is applied regardless of       the input type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_filter: Option<cfn_resources::StrVal>,

    ///
    /// Information about how to connect to the upstream system.
    ///
    /// Required: No
    ///
    /// Type: NetworkInputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_input_settings: Option<NetworkInputSettings>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_pid: Option<i64>,

    ///
    /// Specifies whether to extract applicable ancillary data from a SMPTE-2038 source in this input. Applicable data types are captions, timecode, AFD, and SCTE-104 messages. - PREFER: Extract from SMPTE-2038 if present in this input, otherwise extract from another source (if any). - IGNORE: Never extract any ancillary data from SMPTE-2038.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Smpte2038DataPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte2038_data_preference: Option<cfn_resources::StrVal>,

    ///
    /// The loop input if it is a file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceEndBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_end_behavior: Option<cfn_resources::StrVal>,

    ///
    /// Information about one video to extract from the input.
    ///
    /// Required: No
    ///
    /// Type: VideoSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector: Option<VideoSelector>,
}

impl cfn_resources::CfnResource for InputSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.network_input_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.video_selector
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The input specification for this channel. It specifies the key       characteristics of the inputs for this channel: the maximum bitrate,       the resolution, and the codec.
///
/// This entity is at the top level in the channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputSpecification {
    ///
    /// The codec to include in the input specification for this       channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<cfn_resources::StrVal>,

    ///
    /// The maximum input bitrate for any input attached to this       channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<cfn_resources::StrVal>,

    ///
    /// The resolution for any input attached to this channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for InputSpecification {
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

/// The configuration of key provider settings.
///
/// The parent of this entity is HlsGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KeyProviderSettings {
    ///
    /// The configuration of static key settings.
    ///
    /// Required: No
    ///
    /// Type: StaticKeySettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticKeySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_settings: Option<StaticKeySettings>,
}

impl cfn_resources::CfnResource for KeyProviderSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.static_key_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration of the M2TS in the output.
///
/// The parents of this entity are ArchiveContainerSettings and       UdpContainerSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct M2tsSettings {
    ///
    /// When set to drop, the output audio streams are removed from the       program if the selected input audio stream is removed from the       input. This allows the output audio configuration to dynamically       change based on the input configuration. If this is set to       encodeSilence, all output audio streams will output encoded silence       when not connected to an active input stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AbsentInputAudioBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent_input_audio_behavior: Option<cfn_resources::StrVal>,

    ///
    /// When set to enabled, uses ARIB-compliant field muxing and removes       video descriptor.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib: Option<cfn_resources::StrVal>,

    ///
    /// The PID for ARIB Captions in the transport stream. You can enter       the value as a decimal or hexadecimal value. Valid values are 32 (or       0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AribCaptionsPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_captions_pid: Option<cfn_resources::StrVal>,

    ///
    /// If set to auto, The PID number used for ARIB Captions will be       auto-selected from unused PIDs. If set to useConfigured, ARIB       captions will be on the configured PID number.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AribCaptionsPidControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_captions_pid_control: Option<cfn_resources::StrVal>,

    ///
    /// When set to dvb, uses the DVB buffer model for Dolby Digital       audio. When set to atsc, the ATSC model is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioBufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_buffer_model: Option<cfn_resources::StrVal>,

    ///
    /// The number of audio frames to insert for each PES packet.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,

    ///
    /// The PID of the elementary audio streams in the transport stream.       Multiple values are accepted, and can be entered in ranges or by       comma separation. You can enter the value as a decimal or       hexadecimal value. Each PID specified must be in the range of 32 (or       0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<cfn_resources::StrVal>,

    ///
    /// When set to atsc, uses stream type = 0x81 for AC3 and stream type       = 0x87 for EAC3. When set to dvb, uses stream type = 0x06.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioStreamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_stream_type: Option<cfn_resources::StrVal>,

    ///
    /// The output bitrate of the transport stream in bits per second.       Setting to 0 lets the muxer automatically determine the appropriate       bitrate.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,

    ///
    /// If set to multiplex, uses the multiplex buffer model for accurate       interleaving. Setting to bufferModel to none can lead to lower       latency, but low-memory devices might not be able to play back the       stream without interruptions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_model: Option<cfn_resources::StrVal>,

    ///
    /// When set to enabled, generates captionServiceDescriptor in       PMT.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CcDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_descriptor: Option<cfn_resources::StrVal>,

    ///
    /// Inserts a DVB Network Information Table (NIT) at the specified       table repetition interval.
    ///
    /// Required: No
    ///
    /// Type: DvbNitSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DvbNitSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_nit_settings: Option<DvbNitSettings>,

    ///
    /// Inserts a DVB Service Description Table (SDT) at the specified       table repetition interval.
    ///
    /// Required: No
    ///
    /// Type: DvbSdtSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DvbSdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sdt_settings: Option<DvbSdtSettings>,

    ///
    /// The PID for the input source DVB Subtitle data to this output.       Multiple values are accepted, and can be entered in ranges and/or by       comma separation. You can enter the value as a decimal or       hexadecimal value. Each PID specified must be in the range of 32 (or       0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DvbSubPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_pids: Option<cfn_resources::StrVal>,

    ///
    /// Inserts DVB Time and Date Table (TDT) at the specified table       repetition interval.
    ///
    /// Required: No
    ///
    /// Type: DvbTdtSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DvbTdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_tdt_settings: Option<DvbTdtSettings>,

    ///
    /// The PID for the input source DVB Teletext data to this output. You       can enter the value as a decimal or hexadecimal value. Valid values       are 32 (or 0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DvbTeletextPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pid: Option<cfn_resources::StrVal>,

    ///
    /// If set to passthrough, passes any EBIF data from the input source       to this output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ebif")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebif: Option<cfn_resources::StrVal>,

    ///
    /// When videoAndFixedIntervals is selected, audio EBP markers are       added to partitions 3 and 4. The interval between these additional       markers is fixed, and is slightly shorter than the video EBP marker       interval. This is only available when EBP Cablelabs segmentation       markers are selected. Partitions 1 and 2 always follow the video       interval.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbpAudioInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_audio_interval: Option<cfn_resources::StrVal>,

    ///
    /// When set, enforces that Encoder Boundary Points do not come within       the specified time interval of each other by looking ahead at input       video. If another EBP is going to come in within the specified time       interval, the current EBP is not emitted, and the segment is       "stretched" to the next marker. The lookahead value does not add       latency to the system. The channel must be configured elsewhere to       create sufficient latency to make the lookahead accurate.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbpLookaheadMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_lookahead_ms: Option<i64>,

    ///
    /// Controls placement of EBP on audio PIDs. If set to       videoAndAudioPids, EBP markers are placed on the video PID and all       audio PIDs. If set to videoPid, EBP markers are placed on only the       video PID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbpPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_placement: Option<cfn_resources::StrVal>,

    ///
    /// This field is unused and deprecated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcmPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecm_pid: Option<cfn_resources::StrVal>,

    ///
    /// Includes or excludes the ES Rate field in the PES header.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EsRateInPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_rate_in_pes: Option<cfn_resources::StrVal>,

    ///
    /// The PID for the input source ETV Platform data to this output. You       can enter it as a decimal or hexadecimal value. Valid values are 32       (or 0x20) to 8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EtvPlatformPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_platform_pid: Option<cfn_resources::StrVal>,

    ///
    /// The PID for input source ETV Signal data to this output. You can       enter the value as a decimal or hexadecimal value. Valid values are       32 (or 0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EtvSignalPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_signal_pid: Option<cfn_resources::StrVal>,

    ///
    /// The length in seconds of each fragment. This is used only with EBP       markers.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "FragmentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_time: Option<f64>,

    ///
    /// If set to passthrough, passes any KLV data from the input source       to this output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Klv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv: Option<cfn_resources::StrVal>,

    ///
    /// The PID for the input source KLV data to this output. Multiple       values are accepted, and can be entered in ranges or by comma       separation. You can enter the value as a decimal or hexadecimal       value. Each PID specified must be in the range of 32 (or 0x20)..8182       (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KlvDataPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_data_pids: Option<cfn_resources::StrVal>,

    ///
    /// If set to passthrough, Nielsen inaudible tones for media tracking       will be detected in the input audio and an equivalent ID3 tag will       be inserted in the output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenId3Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<cfn_resources::StrVal>,

    ///
    /// The value, in bits per second, of extra null packets to insert       into the transport stream. This can be used if a downstream       encryption system requires periodic null packets.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "NullPacketBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_packet_bitrate: Option<f64>,

    ///
    /// The number of milliseconds between instances of this table in the       output transport stream. Valid values are 0, 10..1000.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,

    ///
    /// When set to pcrEveryPesPacket, a Program Clock Reference value is       inserted for every Packetized Elementary Stream (PES) header. This       parameter is effective only when the PCR PID is the same as the       video or audio elementary stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<cfn_resources::StrVal>,

    ///
    /// The maximum time, in milliseconds, between Program Clock       References (PCRs) inserted into the transport stream.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PcrPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_period: Option<i64>,

    ///
    /// The PID of the Program Clock Reference (PCR) in the transport       stream. When no value is given, MediaLive assigns the same value as       the video PID. You can enter the value as a decimal or hexadecimal       value. Valid values are 32 (or 0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<cfn_resources::StrVal>,

    ///
    /// The number of milliseconds between instances of this table in the       output transport stream. Valid values are 0, 10..1000.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,

    ///
    /// The PID for the Program Map Table (PMT) in the transport stream.       You can enter the value as a decimal or hexadecimal value. Valid       values are 32 (or 0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<cfn_resources::StrVal>,

    ///
    /// The value of the program number field in the Program Map Table       (PMT).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_num: Option<i64>,

    ///
    /// When VBR, does not insert null packets into the transport stream       to fill the specified bitrate. The bitrate setting acts as the       maximum bitrate when VBR is set.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_mode: Option<cfn_resources::StrVal>,

    ///
    /// The PID for the input source SCTE-27 data to this output. Multiple       values are accepted, and can be entered in ranges or by comma       separation. You can enter the value as a decimal or hexadecimal       value. Each PID specified must be in the range of 32 (or 0x20)..8182       (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte27Pids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte27_pids: Option<cfn_resources::StrVal>,

    ///
    /// Optionally passes SCTE-35 signals from the input source to this       output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35Control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_control: Option<cfn_resources::StrVal>,

    ///
    /// The PID of the SCTE-35 stream in the transport stream. You can       enter the value as a decimal or hexadecimal value. Valid values are       32 (or 0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_pid: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35PrerollPullupMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_preroll_pullup_milliseconds: Option<f64>,

    ///
    /// Inserts segmentation markers at each segmentationTime period.       raiSegstart sets the Random Access Indicator bit in the adaptation       field. raiAdapt sets the RAI bit and adds the current timecode in       the private data bytes. psiSegstart inserts PAT and PMT tables at       the start of segments. ebp adds Encoder Boundary Point information       to the adaptation field as per OpenCable specification       OC-SP-EBP-I01-130118. ebpLegacy adds Encoder Boundary Point       information to the adaptation field using a legacy proprietary       format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentationMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_markers: Option<cfn_resources::StrVal>,

    ///
    /// The segmentation style parameter controls how segmentation markers       are inserted into the transport stream. With avails, it is possible       that segments might be truncated, which can influence where future       segmentation markers are inserted. When a segmentation style of       resetCadence is selected and a segment is truncated due to an avail,       we will reset the segmentation cadence. This means the subsequent       segment will have a duration of $segmentationTime seconds. When a       segmentation style of maintainCadence is selected and a segment is       truncated due to an avail, we will not reset the segmentation       cadence. This means the subsequent segment will likely be truncated       as well. However, all segments after that will have a duration of       $segmentationTime seconds. Note that EBP lookahead is a slight       exception to this rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentationStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_style: Option<cfn_resources::StrVal>,

    ///
    /// The length, in seconds, of each segment. This is required unless       markers is set to None_.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_time: Option<f64>,

    ///
    /// When set to passthrough, timed metadata is passed through from       input to output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<cfn_resources::StrVal>,

    ///
    /// The PID of the timed metadata stream in the transport stream. You       can enter the value as a decimal or hexadecimal value. Valid values       are 32 (or 0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<cfn_resources::StrVal>,

    ///
    /// The value of the transport stream ID field in the Program Map       Table (PMT).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,

    ///
    /// The PID of the elementary video stream in the transport stream.       You can enter the value as a decimal or hexadecimal value. Valid       values are 32 (or 0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for M2tsSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dvb_nit_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dvb_sdt_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dvb_tdt_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for the M3U8 container.
///
/// The parent of this entity is StandardHlsSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct M3u8Settings {
    ///
    /// The number of audio frames to insert for each PES packet.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,

    ///
    /// The PID of the elementary audio streams in the transport stream.       Multiple values are accepted, and can be entered in ranges or by       comma separation. You can enter the value as a decimal or       hexadecimal value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<cfn_resources::StrVal>,

    ///
    /// This parameter is unused and deprecated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcmPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecm_pid: Option<cfn_resources::StrVal>,

    ///
    /// If set to passthrough, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenId3Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<cfn_resources::StrVal>,

    ///
    /// The number of milliseconds between instances of this table in the       output transport stream. A value of \"0\" writes out the PMT once       per segment file.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,

    ///
    /// When set to pcrEveryPesPacket, a Program Clock Reference value is       inserted for every Packetized Elementary Stream (PES) header. This       parameter is effective only when the PCR PID is the same as the       video or audio elementary stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<cfn_resources::StrVal>,

    ///
    /// The maximum time, in milliseconds, between Program Clock       References (PCRs) inserted into the transport stream.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PcrPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_period: Option<i64>,

    ///
    /// The PID of the Program Clock Reference (PCR) in the transport       stream. When no value is given, MediaLive assigns the same value as       the video PID. You can enter the value as a decimal or hexadecimal       value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<cfn_resources::StrVal>,

    ///
    /// The number of milliseconds between instances of this table in the       output transport stream. A value of \"0\" writes out the PMT once       per segment file.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,

    ///
    /// The PID for the Program Map Table (PMT) in the transport stream.       You can enter the value as a decimal or hexadecimal value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<cfn_resources::StrVal>,

    ///
    /// The value of the program number field in the Program Map Table       (PMT).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_num: Option<i64>,

    ///
    /// If set to passthrough, passes any SCTE-35 signals from the input       source to this output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_behavior: Option<cfn_resources::StrVal>,

    ///
    /// The PID of the SCTE-35 stream in the transport stream. You can       enter the value as a decimal or hexadecimal value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_pid: Option<cfn_resources::StrVal>,

    ///
    /// When set to passthrough, timed metadata is passed through from       input to output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<cfn_resources::StrVal>,

    ///
    /// The PID of the timed metadata stream in the transport stream. You       can enter the value as a decimal or hexadecimal value. Valid values       are 32 (or 0x20)..8182 (or 0x1ff6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<cfn_resources::StrVal>,

    ///
    /// The value of the transport stream ID field in the Program Map       Table (PMT).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,

    ///
    /// The PID of the elementary video stream in the transport stream.       You can enter the value as a decimal or hexadecimal value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for M3u8Settings {
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

/// The MaintenanceCreateSettings property type specifies Property description not available. for an AWS::MediaLive::Channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaintenanceCreateSettings {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_day: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_time: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MaintenanceCreateSettings {
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

/// The MaintenanceUpdateSettings property type specifies Property description not available. for an AWS::MediaLive::Channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaintenanceUpdateSettings {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_day: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceScheduledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_scheduled_date: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_time: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MaintenanceUpdateSettings {
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

/// The settings for the MediaPackage group.
///
/// The parent of this entity is OutputGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MediaPackageGroupSettings {
    ///
    /// The MediaPackage channel destination.
    ///
    /// Required: No
    ///
    /// Type: OutputLocationRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OutputLocationRef>,
}

impl cfn_resources::CfnResource for MediaPackageGroupSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Destination settings for a MediaPackage output.
///
/// The parent of this entity is OutputDestination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MediaPackageOutputDestinationSettings {
    ///
    /// The ID of the channel in MediaPackage that is the destination for       this output group. You don't need to specify the individual inputs       in MediaPackage; MediaLive handles the connection of the two       MediaLive pipelines to the two MediaPackage inputs. The MediaPackage       channel and MediaLive channel must be in the same Region.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MediaPackageOutputDestinationSettings {
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

/// The settings for a MediaPackage output.
///
/// The parent of this entity is OutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MediaPackageOutputSettings {}

impl cfn_resources::CfnResource for MediaPackageOutputSettings {
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

/// Settings to enable and configure the motion graphics overlay       feature in the channel.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MotionGraphicsConfiguration {
    ///
    /// Enables or disables the motion graphics overlay feature in the       channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MotionGraphicsInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_insertion: Option<cfn_resources::StrVal>,

    ///
    /// Settings to enable and configure the motion graphics overlay       feature in the channel.
    ///
    /// Required: No
    ///
    /// Type: MotionGraphicsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MotionGraphicsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_settings: Option<MotionGraphicsSettings>,
}

impl cfn_resources::CfnResource for MotionGraphicsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.motion_graphics_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings to enable and configure the motion graphics overlay       feature in the channel.
///
/// The parent of this entity is MotionGraphicsConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MotionGraphicsSettings {
    ///
    /// Settings to configure the motion graphics overlay to use an HTML       asset.
    ///
    /// Required: No
    ///
    /// Type: HtmlMotionGraphicsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HtmlMotionGraphicsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_motion_graphics_settings: Option<HtmlMotionGraphicsSettings>,
}

impl cfn_resources::CfnResource for MotionGraphicsSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.html_motion_graphics_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration for this MP2 audio.
///
/// The parent of this entity is AudioCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Mp2Settings {
    ///
    /// The average bitrate in bits/second.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,

    ///
    /// The MPEG2 Audio coding mode. Valid values are codingMode10 (for       mono) or codingMode20 (for stereo).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<cfn_resources::StrVal>,

    ///
    /// The sample rate in Hz.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
}

impl cfn_resources::CfnResource for Mp2Settings {
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

/// Settings to configure video filters that apply to the MPEG-2       codec.
///
/// The parent of this entity is Mpeg2FilterSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Mpeg2FilterSettings {
    /// Settings for applying the temporal filter to the video.
    ///
    /// Required: No
    ///
    /// Type: TemporalFilterSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemporalFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

impl cfn_resources::CfnResource for Mpeg2FilterSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.temporal_filter_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for the MPEG-2 codec in the output.
///
/// The parent of this entity is VideoCodecSetting.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Mpeg2Settings {
    ///
    /// Choose Off to disable adaptive quantization. Or choose another value to enable the quantizer and set its strength. The strengths are: Auto, Off, Low, Medium, High. When you enable this field, MediaLive allows intra-frame quantizers to vary, which might improve visual quality.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<cfn_resources::StrVal>,

    ///
    /// Indicates the AFD values that MediaLive will write into the video encode. If you do not know what AFD signaling is, or if your downstream system has not given you guidance, choose AUTO. AUTO: MediaLive will try to preserve the input AFD value (in cases where multiple AFD values are valid). FIXED: MediaLive will use the value you specify in fixedAFD.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AfdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether to include the color space metadata. The metadata describes the color space that applies to the video (the colorSpace field). We recommend that you insert the metadata.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<cfn_resources::StrVal>,

    ///
    /// Choose the type of color space conversion to apply to the output. For detailed information on setting up both the input and the output to obtain the desired color space in the output, see the section on \"MediaLive Features - Video - color space\" in the MediaLive User Guide. PASSTHROUGH: Keep the color space of the input content - do not convert it. AUTO:Convert all content that is SD to rec 601, and convert all content that is HD to rec 709.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<cfn_resources::StrVal>,

    ///
    /// Sets the pixel aspect ratio for the encode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayAspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_aspect_ratio: Option<cfn_resources::StrVal>,

    ///
    /// Optionally specify a noise reduction filter, which can improve quality of compressed content. If you do not choose a filter, no filter will be applied. TEMPORAL: This filter is useful for both source content that is noisy (when it has excessive digital artifacts) and source content that is clean. When the content is noisy, the filter cleans up the source content before the encoding phase, with these two effects: First, it improves the output video quality because the content has been cleaned up. Secondly, it decreases the bandwidth because MediaLive does not waste bits on encoding noise. When the content is reasonably clean, the filter tends to decrease the bitrate.
    ///
    /// Required: No
    ///
    /// Type: Mpeg2FilterSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<Mpeg2FilterSettings>,

    ///
    /// Complete this field only when afdSignaling is set to FIXED. Enter the AFD value (4 bits) to write on all frames of the video encode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<cfn_resources::StrVal>,

    ///
    /// description": "The framerate denominator. For example, 1001. The framerate is the numerator divided by the denominator. For example, 24000 / 1001 = 23.976 FPS.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,

    ///
    /// The framerate numerator. For example, 24000. The framerate is the numerator divided by the denominator. For example, 24000 / 1001 = 23.976 FPS.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,

    ///
    /// MPEG2: default is open GOP.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,

    ///
    /// Relates to the GOP structure. The number of B-frames between reference frames. If you do not know what a B-frame is, use the default.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopNumBFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_num_bframes: Option<i64>,

    ///
    /// Relates to the GOP structure. The GOP size (keyframe interval) in the units specified in gopSizeUnits. If you do not know what GOP is, use the default. If gopSizeUnits is frames, then the gopSize must be an integer and must be greater than or equal to 1. If gopSizeUnits is seconds, the gopSize must be greater than 0, but does not need to be an integer.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,

    ///
    /// Relates to the GOP structure. Specifies whether the gopSize is specified in frames or seconds. If you do not plan to change the default gopSize, leave the default. If you specify SECONDS, MediaLive will internally convert the gop size to a frame count.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<cfn_resources::StrVal>,

    ///
    /// Set the scan type of the output to PROGRESSIVE or INTERLACED (top field first).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<cfn_resources::StrVal>,

    ///
    /// Relates to the GOP structure. If you do not know what GOP is, use the default. FIXED: Set the number of B-frames in each sub-GOP to the value in gopNumBFrames. DYNAMIC: Let MediaLive optimize the number of B-frames in each sub-GOP, to improve visual quality.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubgopLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgop_length: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TimecodeBurninSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimecodeBurninSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,

    ///
    /// Determines how MediaLive inserts timecodes in the output video. For detailed information about setting up the input and the output for a timecode, see the section on \"MediaLive Features - Timecode configuration\" in the MediaLive User Guide. DISABLED: do not include timecodes. GOP_TIMECODE: Include timecode metadata in the GOP header.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimecodeInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Mpeg2Settings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.filter_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.timecode_burnin_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for a Microsoft Smooth output group.
///
/// The parent of this entity is OutputGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MsSmoothGroupSettings {
    ///
    /// The value of the Acquisition Point Identity element that is used       in each message placed in the sparse track. Enabled only if       sparseTrackType is not "none."
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcquisitionPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_point_id: Option<cfn_resources::StrVal>,

    ///
    /// If set to passthrough for an audio-only Microsoft Smooth output,       the fragment absolute time is set to the current timecode. This       option does not write timecodes to the audio elementary       stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioOnlyTimecodeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_timecode_control: Option<cfn_resources::StrVal>,

    ///
    /// If set to verifyAuthenticity, verifies the HTTPS certificate chain       to a trusted certificate authority (CA). This causes HTTPS outputs       to self-signed certificates to fail.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<cfn_resources::StrVal>,

    ///
    /// The number of seconds to wait before retrying the connection to       the IIS server if the connection is lost. Content is cached during       this time, and the cache is delivered to the IIS server after the       connection is re-established.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,

    ///
    /// The Smooth Streaming publish point on an IIS server. MediaLive       acts as a "Push" encoder to IIS.
    ///
    /// Required: No
    ///
    /// Type: OutputLocationRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OutputLocationRef>,

    ///
    /// The Microsoft Smooth channel ID that is sent to the IIS server.       Specify the ID only if eventIdMode is set to useConfigured.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether to send a channel ID to the IIS server. If no       channel ID is sent and the same channel is used without changing the       publishing point, clients might see cached video from the previous       run. Options: - "useConfigured" - use the value provided in eventId       - "useTimestamp" - generate and send a channel ID based on the       current timestamp - "noEventId" - do not send a channel ID to the       IIS server.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventIdMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id_mode: Option<cfn_resources::StrVal>,

    ///
    /// When set to sendEos, sends an EOS signal to an IIS server when       stopping the channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventStopBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stop_behavior: Option<cfn_resources::StrVal>,

    ///
    /// The size, in seconds, of the file cache for streaming       outputs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,

    ///
    /// The length, in seconds, of mp4 fragments to generate. The fragment       length must be compatible with GOP size and frame rate.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FragmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i64>,

    ///
    /// A parameter that controls output group behavior on an input       loss.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<cfn_resources::StrVal>,

    ///
    /// The number of retry attempts.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,

    ///
    /// The number of seconds before initiating a restart due to output       failure, due to exhausting the numRetries on one segment, or       exceeding filecacheDuration.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,

    ///
    /// useInputSegmentation has been deprecated. The configured segment       size is always used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_mode: Option<cfn_resources::StrVal>,

    ///
    /// The number of milliseconds to delay the output from the second       pipeline.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SendDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_delay_ms: Option<i64>,

    ///
    /// If set to scte35, uses incoming SCTE-35 messages to generate a       sparse track in this group of Microsoft Smooth outputs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SparseTrackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse_track_type: Option<cfn_resources::StrVal>,

    ///
    /// When set to send, sends a stream manifest so that the publishing       point doesn't start until all streams start.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamManifestBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_manifest_behavior: Option<cfn_resources::StrVal>,

    ///
    /// The timestamp offset for the channel. Used only if       timestampOffsetMode is set to useConfiguredOffset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_offset: Option<cfn_resources::StrVal>,

    ///
    /// The type of timestamp date offset to use. - useEventStartDate: Use       the date the channel was started as the offset -       useConfiguredOffset: Use an explicitly configured date as the       offset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampOffsetMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_offset_mode: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MsSmoothGroupSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Configuration of a Microsoft Smooth output.
///
/// The parent of this entity is OutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MsSmoothOutputSettings {
    ///
    /// Only applicable when this output is referencing an H.265 video description. Specifies whether MP4 segments should be packaged as HEV1 or HVC1.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "H265PackagingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_packaging_type: Option<cfn_resources::StrVal>,

    ///
    /// A string that is concatenated to the end of the destination file       name. This is required for multiple outputs of the same type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MsSmoothOutputSettings {
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

/// The settings for a Multiplex output group.
///
/// The parent of this entity is OutputGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MultiplexGroupSettings {}

impl cfn_resources::CfnResource for MultiplexGroupSettings {
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

/// Configuration of a Multiplex output.
///
/// The parent of this entity is OutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MultiplexOutputSettings {
    ///
    /// Destination is a Multiplex.
    ///
    /// Required: No
    ///
    /// Type: OutputLocationRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OutputLocationRef>,
}

impl cfn_resources::CfnResource for MultiplexOutputSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Destination settings for a Multiplex output.
///
/// The parent of this entity is OutputDestination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MultiplexProgramChannelDestinationSettings {
    ///
    /// The ID of the Multiplex that the encoder is providing output to. You do not need to specify the individual inputs to the Multiplex; MediaLive will handle the connection of the two MediaLive pipelines to the two Multiplex instances. The Multiplex must be in the same region as the Channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiplexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_id: Option<cfn_resources::StrVal>,

    ///
    /// The program name of the Multiplex program that the encoder is providing output to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MultiplexProgramChannelDestinationSettings {
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

/// Information about how to connect to the upstream system.
///
/// The parent of this entity is InputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkInputSettings {
    ///
    /// Information about how to connect to the upstream system.
    ///
    /// Required: No
    ///
    /// Type: HlsInputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsInputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_input_settings: Option<HlsInputSettings>,

    ///
    /// Checks HTTPS server certificates. When set to       checkCryptographyOnly, cryptography in the certificate is checked,       but not the server's name. Certain subdomains (notably S3 buckets       that use dots in the bucket name) don't strictly match the       corresponding certificate's wildcard pattern and would otherwise       cause the channel to error. This setting is ignored for protocols       that do not use HTTPS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_validation: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NetworkInputSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.hls_input_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Complete these fields only if you want to insert watermarks of type Nielsen CBET
///
/// The parent of this entity is NielsenWatermarksSettings
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NielsenCBET {
    ///
    /// Enter the CBET check digits to use in the watermark.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CbetCheckDigitString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cbet_check_digit_string: Option<cfn_resources::StrVal>,

    ///
    /// Determines the method of CBET insertion mode when prior encoding is detected on the same layer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CbetStepaside")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cbet_stepaside: Option<cfn_resources::StrVal>,

    ///
    /// Enter the CBET Source ID (CSID) to use in the watermark
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Csid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csid: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NielsenCBET {
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

/// The settings to configure Nielsen watermarks.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NielsenConfiguration {
    ///
    /// Enter the Distributor ID assigned to your organization by Nielsen.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DistributorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor_id: Option<cfn_resources::StrVal>,

    ///
    /// Enables Nielsen PCM to ID3 tagging
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenPcmToId3Tagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_pcm_to_id3_tagging: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NielsenConfiguration {
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

/// Complete these fields only if you want to insert watermarks of type Nielsen NAES II (N2) and Nielsen NAES VI (NW).
///
/// The parent of this entity is NielsenWatermarksSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NielsenNaesIiNw {
    ///
    /// Enter the check digit string for the watermark
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CheckDigitString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_digit_string: Option<cfn_resources::StrVal>,

    ///
    /// Enter the Nielsen Source ID (SID) to include in the watermark
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<f64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NielsenNaesIiNw {
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

/// Settings to configure Nielsen Watermarks in the audio encode.
///
/// The parent of this entity is AudioWatermarkSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NielsenWatermarksSettings {
    ///
    /// Complete these fields only if you want to insert watermarks of type Nielsen CBET
    ///
    /// Required: No
    ///
    /// Type: NielsenCBET
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenCbetSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_cbet_settings: Option<NielsenCBET>,

    ///
    /// Choose the distribution types that you want to assign to the watermarks: - PROGRAM_CONTENT - FINAL_DISTRIBUTOR
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenDistributionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_distribution_type: Option<cfn_resources::StrVal>,

    ///
    /// Complete these fields only if you want to insert watermarks of type Nielsen NAES II (N2) and Nielsen NAES VI (NW).
    ///
    /// Required: No
    ///
    /// Type: NielsenNaesIiNw
    ///
    /// Update requires: No interruption
    #[serde(rename = "NielsenNaesIiNwSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_naes_ii_nw_settings: Option<NielsenNaesIiNw>,
}

impl cfn_resources::CfnResource for NielsenWatermarksSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.nielsen_cbet_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.nielsen_naes_ii_nw_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The output settings.
///
/// The parent of this entity is OutputGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Output {
    ///
    /// The names of the audio descriptions that are used as audio sources       for this output.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioDescriptionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_description_names: Option<Vec<String>>,

    ///
    /// The names of the caption descriptions that are used as captions       sources for this output.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptionDescriptionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_description_names: Option<Vec<String>>,

    ///
    /// The name that is used to identify an output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_name: Option<cfn_resources::StrVal>,

    ///
    /// The output type-specific settings.
    ///
    /// Required: No
    ///
    /// Type: OutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_settings: Option<OutputSettings>,

    ///
    /// The name of the VideoDescription that is used as the source for       this output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoDescriptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Output {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Configuration information for an output.
///
/// This entity is at the top level in the channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputDestination {
    ///
    /// The ID for this destination.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,

    ///
    /// The destination settings for a MediaPackage output.
    ///
    /// Required: No
    ///
    /// Type: List of MediaPackageOutputDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MediaPackageSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_settings: Option<Vec<MediaPackageOutputDestinationSettings>>,

    ///
    /// Destination settings for a Multiplex output; one destination for both encoders.
    ///
    /// Required: No
    ///
    /// Type: MultiplexProgramChannelDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexProgramChannelDestinationSettings>,

    ///
    /// The destination settings for an output.
    ///
    /// Required: No
    ///
    /// Type: List of OutputDestinationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<OutputDestinationSettings>>,
}

impl cfn_resources::CfnResource for OutputDestination {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.multiplex_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration information for this output.
///
/// The parent of this entity is OutputDestination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputDestinationSettings {
    ///
    /// The password parameter that holds the password for accessing the       downstream system. This password parameter applies only if the       downstream system requires credentials.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PasswordParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<cfn_resources::StrVal>,

    ///
    /// The stream name for the content. This applies only to RTMP       outputs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<cfn_resources::StrVal>,

    ///
    /// The URL for the destination.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<cfn_resources::StrVal>,

    ///
    /// The user name to connect to the downstream system. This applies       only if the downstream system requires credentials.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for OutputDestinationSettings {
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

/// The settings for one output group.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputGroup {
    ///
    /// A custom output group name that you can optionally define. Only       letters, numbers, and the underscore character are allowed. The       maximum length is 32 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The settings associated with the output group.
    ///
    /// Required: No
    ///
    /// Type: OutputGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_group_settings: Option<OutputGroupSettings>,

    ///
    /// The settings for the outputs in the output group.
    ///
    /// Required: No
    ///
    /// Type: List of Output
    ///
    /// Update requires: No interruption
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

impl cfn_resources::CfnResource for OutputGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.output_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration of the output group.
///
/// The parent of this entity is OutputGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputGroupSettings {
    ///
    /// The configuration of an archive output group.
    ///
    /// The parent of this entity is OutputGroupSettings.
    ///
    /// Required: No
    ///
    /// Type: ArchiveGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArchiveGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_group_settings: Option<ArchiveGroupSettings>,

    ///
    /// The configuration of a frame capture output group.
    ///
    /// Required: No
    ///
    /// Type: FrameCaptureGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameCaptureGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_group_settings: Option<FrameCaptureGroupSettings>,

    ///
    /// The configuration of an HLS output group.
    ///
    /// Required: No
    ///
    /// Type: HlsGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_group_settings: Option<HlsGroupSettings>,

    ///
    /// The configuration of a MediaPackage output group.
    ///
    /// Required: No
    ///
    /// Type: MediaPackageGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MediaPackageGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_group_settings: Option<MediaPackageGroupSettings>,

    ///
    /// The configuration of a Microsoft Smooth output group.
    ///
    /// Required: No
    ///
    /// Type: MsSmoothGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MsSmoothGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_group_settings: Option<MsSmoothGroupSettings>,

    /// The settings for a Multiplex output group.
    ///
    /// Required: No
    ///
    /// Type: MultiplexGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiplexGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_group_settings: Option<MultiplexGroupSettings>,

    ///
    /// The configuration of an RTMP output group.
    ///
    /// Required: No
    ///
    /// Type: RtmpGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RtmpGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_group_settings: Option<RtmpGroupSettings>,

    ///
    /// The configuration of a UDP output group.
    ///
    /// Required: No
    ///
    /// Type: UdpGroupSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "UdpGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_group_settings: Option<UdpGroupSettings>,
}

impl cfn_resources::CfnResource for OutputGroupSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.archive_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.frame_capture_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.media_package_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ms_smooth_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.multiplex_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rtmp_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.udp_group_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A reference to an OutputDestination ID that is defined in the       channel.
///
/// This entity is used by ArchiveGroupSettings,       FrameCaptureGroupSettings, HlsGroupSettings,       MediaPackageGroupSettings, MSSmoothGroupSettings,       RtmpOutputSettings, and UdpOutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputLocationRef {
    ///
    /// A reference ID for this destination.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationRefId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ref_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for OutputLocationRef {
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

/// The output settings.
///
/// The parent of this entity is Output.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputSettings {
    ///
    /// The settings for an archive output.
    ///
    /// Required: No
    ///
    /// Type: ArchiveOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArchiveOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_output_settings: Option<ArchiveOutputSettings>,

    ///
    /// The settings for a frame capture output.
    ///
    /// The parent of this entity is OutputGroupSettings.
    ///
    /// Required: No
    ///
    /// Type: FrameCaptureOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameCaptureOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_output_settings: Option<FrameCaptureOutputSettings>,

    ///
    /// The settings for an HLS output.
    ///
    /// The parent of this entity is OutputGroupSettings.
    ///
    /// Required: No
    ///
    /// Type: HlsOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_output_settings: Option<HlsOutputSettings>,

    ///
    /// The settings for a MediaPackage output.
    ///
    /// The parent of this entity is OutputGroupSettings.
    ///
    /// Required: No
    ///
    /// Type: MediaPackageOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MediaPackageOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_output_settings: Option<MediaPackageOutputSettings>,

    ///
    /// The settings for a Microsoft Smooth output.
    ///
    /// Required: No
    ///
    /// Type: MsSmoothOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MsSmoothOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_output_settings: Option<MsSmoothOutputSettings>,

    /// Configuration of a Multiplex output.
    ///
    /// Required: No
    ///
    /// Type: MultiplexOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiplexOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_output_settings: Option<MultiplexOutputSettings>,

    ///
    /// The settings for an RTMP output.
    ///
    /// The parent of this entity is OutputGroupSettings.
    ///
    /// Required: No
    ///
    /// Type: RtmpOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RtmpOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_output_settings: Option<RtmpOutputSettings>,

    ///
    /// The settings for a UDP output.
    ///
    /// The parent of this entity is OutputGroupSettings.
    ///
    /// Required: No
    ///
    /// Type: UdpOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "UdpOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_output_settings: Option<UdpOutputSettings>,
}

impl cfn_resources::CfnResource for OutputSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.archive_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.frame_capture_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.media_package_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ms_smooth_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.multiplex_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rtmp_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.udp_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for passing through audio to the output.
///
/// The parent of this entity is AudioCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PassThroughSettings {}

impl cfn_resources::CfnResource for PassThroughSettings {
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

/// The container for WAV audio in the output group.
///
/// The parent of this entity is ArchiveContainerSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RawSettings {}

impl cfn_resources::CfnResource for RawSettings {
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

/// Rec601 Settings
///
/// The parents of this entity are H264ColorSpaceSettings and       H265ColorSpaceSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Rec601Settings {}

impl cfn_resources::CfnResource for Rec601Settings {
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

/// Rec709 Settings
///
/// The parents of this entity are H264ColorSpaceSettings and       H265ColorSpaceSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Rec709Settings {}

impl cfn_resources::CfnResource for Rec709Settings {
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

/// The settings for remixing audio in the output.
///
/// The parent of this entity is AudioDescription.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RemixSettings {
    ///
    /// A mapping of input channels to output channels, with appropriate       gain adjustments.
    ///
    /// Required: No
    ///
    /// Type: List of AudioChannelMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_mappings: Option<Vec<AudioChannelMapping>>,

    ///
    /// The number of input channels to be used.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelsIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_in: Option<i64>,

    ///
    /// The number of output channels to be produced. Valid values: 1, 2,       4, 6, 8.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelsOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_out: Option<i64>,
}

impl cfn_resources::CfnResource for RemixSettings {
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

/// The settings for RTMPCaptionInfo captions encode in the       output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RtmpCaptionInfoDestinationSettings {}

impl cfn_resources::CfnResource for RtmpCaptionInfoDestinationSettings {
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

/// The configuration of an RTMP output group.
///
/// The parent of this entity is OutputGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RtmpGroupSettings {
    ///
    /// Choose the ad marker type for this output group. MediaLive will create a message based on the content of each SCTE-35 message, format it for that marker type, and insert it in the datastream.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,

    ///
    /// An authentication scheme to use when connecting with a CDN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_scheme: Option<cfn_resources::StrVal>,

    ///
    /// Controls behavior when the content cache fills up. If a remote       origin server stalls the RTMP connection and doesn't accept content       fast enough, the media cache fills up. When the cache reaches the       duration specified by cacheLength, the cache stops accepting new       content. If set to disconnectImmediately, the RTMP output forces a       disconnect. Clear the media cache, and reconnect after restartDelay       seconds. If set to waitForServer, the RTMP output waits up to 5       minutes to allow the origin server to begin accepting data       again.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheFullBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_full_behavior: Option<cfn_resources::StrVal>,

    ///
    /// The cache length, in seconds, that is used to calculate buffer       size.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_length: Option<i64>,

    ///
    /// Controls the types of data that pass to onCaptionInfo outputs. If       set to all, 608 and 708 carried DTVCC data is passed. If set to       field1AndField2608, DTVCC data is stripped out, but 608 data from       both fields is passed. If set to field1608, only the data carried in       608 from field 1 video is passed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_data: Option<cfn_resources::StrVal>,

    ///
    /// Controls the behavior of this RTMP group if the input becomes       unavailable. emitOutput: Emit a slate until the input returns.       pauseOutput: Stop transmitting data until the input returns. This       does not close the underlying RTMP connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<cfn_resources::StrVal>,

    ///
    /// If a streaming output fails, the number of seconds to wait until a       restart is initiated. A value of 0 means never restart.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
}

impl cfn_resources::CfnResource for RtmpGroupSettings {
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

/// The settings for one RTMP output.
///
/// The parent of this entity is OutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RtmpOutputSettings {
    ///
    /// If set to verifyAuthenticity, verifies the TLS certificate chain       to a trusted certificate authority (CA). This causes RTMPS outputs       with self-signed certificates to fail.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<cfn_resources::StrVal>,

    ///
    /// The number of seconds to wait before retrying a connection to the       Flash Media server if the connection is lost.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,

    ///
    /// The RTMP endpoint excluding the stream name (for example,       rtmp://host/appname).
    ///
    /// Required: No
    ///
    /// Type: OutputLocationRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OutputLocationRef>,

    ///
    /// The number of retry attempts.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,
}

impl cfn_resources::CfnResource for RtmpOutputSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration of SCTE-20 plus embedded captions in the       output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scte20PlusEmbeddedDestinationSettings {}

impl cfn_resources::CfnResource for Scte20PlusEmbeddedDestinationSettings {
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

/// Information about the SCTE-20 captions to extract from the       input.
///
/// The parent of this entity is CaptionSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scte20SourceSettings {
    ///
    /// If upconvert, 608 data is both passed through the "608       compatibility bytes" fields of the 708 wrapper as well as translated       into 708. Any 708 data present in the source content is       discarded.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert608_to708: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the 608/708 channel number within the video track from       which to extract captions.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source608ChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source608_channel_number: Option<i64>,
}

impl cfn_resources::CfnResource for Scte20SourceSettings {
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

/// The configuration of SCTE-27 captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scte27DestinationSettings {}

impl cfn_resources::CfnResource for Scte27DestinationSettings {
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

/// Information about the SCTE-27 captions to extract from the       input.
///
/// The parent of this entity is CaptionSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scte27SourceSettings {
    ///
    /// If you will configure a WebVTT caption description that references this caption selector, use this field to provide the language to consider when translating the image-based source to text.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OcrLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_language: Option<cfn_resources::StrVal>,

    ///
    /// The PID field is used in conjunction with the captions selector       languageCode field as follows: Specify PID and Language: Extracts       captions from that PID; the language is "informational." Specify PID       and omit Language: Extracts the specified PID. Omit PID and specify       Language: Extracts the specified language, whichever PID that       happens to be. Omit PID and omit Language: Valid only if source is       DVB-Sub that is being passed through; all languages are passed       through.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

impl cfn_resources::CfnResource for Scte27SourceSettings {
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

/// The setup of SCTE-35 splice insert handling.
///
/// The parent of this entity is AvailSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scte35SpliceInsert {
    ///
    /// When specified, this offset (in milliseconds) is added to the       input ad avail PTS time. This applies only to embedded SCTE 104/35       messages. It doesn't apply to OOB messages.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,

    ///
    /// When set to ignore, segment descriptors with       noRegionalBlackoutFlag set to 0 no longer trigger blackouts or ad       avail slates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoRegionalBlackoutFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_regional_blackout_flag: Option<cfn_resources::StrVal>,

    ///
    /// When set to ignore, segment descriptors with       webDeliveryAllowedFlag set to 0 no longer trigger blackouts or ad       avail slates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebDeliveryAllowedFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_delivery_allowed_flag: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Scte35SpliceInsert {
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

/// The settings for the SCTE-35 time signal APOS mode.
///
/// The parent of this entity is AvailSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scte35TimeSignalApos {
    ///
    /// When specified, this offset (in milliseconds) is added to the       input ad avail PTS time. This applies only to embedded SCTE 104/35       messages. It doesn't apply to OOB messages.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,

    ///
    /// When set to ignore, segment descriptors with       noRegionalBlackoutFlag set to 0 no longer trigger blackouts or ad       avail slates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoRegionalBlackoutFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_regional_blackout_flag: Option<cfn_resources::StrVal>,

    ///
    /// When set to ignore, segment descriptors with       webDeliveryAllowedFlag set to 0 no longer trigger blackouts or ad       avail slates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebDeliveryAllowedFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_delivery_allowed_flag: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Scte35TimeSignalApos {
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

/// The setup of SMPTE-TT captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SmpteTtDestinationSettings {}

impl cfn_resources::CfnResource for SmpteTtDestinationSettings {
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

/// The configuration of an HLS output that is a standard output (not       an audio-only output).
///
/// The parent of this entity is HlsSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StandardHlsSettings {
    ///
    /// Lists all the audio groups that are used with the video output       stream. This inputs all the audio GROUP-IDs that are associated with       the video, separated by a comma (,).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AudioRenditionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<cfn_resources::StrVal>,

    ///
    /// Settings for the M3U8 container.
    ///
    /// Required: No
    ///
    /// Type: M3u8Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "M3u8Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m3u8_settings: Option<M3u8Settings>,
}

impl cfn_resources::CfnResource for StandardHlsSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.m3u8_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The static key settings.
///
/// The parent of this entity is KeyProviderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StaticKeySettings {
    ///
    /// The URL of the license server that is used for protecting       content.
    ///
    /// Required: No
    ///
    /// Type: InputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyProviderServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_provider_server: Option<InputLocation>,

    ///
    /// The static key value as a 32 character hexadecimal string.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for StaticKeySettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.key_provider_server
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The settings for a Teletext captions output encode.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TeletextDestinationSettings {}

impl cfn_resources::CfnResource for TeletextDestinationSettings {
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

/// Information about the Teletext captions to extract from the       input.
///
/// The parent of this entity is CaptionSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TeletextSourceSettings {
    ///
    /// Settings to configure the caption rectangle for an output captions       that will be created using this Teletext source captions.
    ///
    /// Required: No
    ///
    /// Type: CaptionRectangle
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputRectangle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_rectangle: Option<CaptionRectangle>,

    ///
    /// Specifies the Teletext page number within the data stream from       which to extract captions. The range is 0x100 (256) to 0x8FF (2303).       This is unused for passthrough. It should be specified as a       hexadecimal string with no "0x" prefix.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TeletextSourceSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.output_rectangle
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for the temporal filter to apply to the video.
///
/// The parents of this entity are H264FilterSettings,       H265FilterSettings, and Mpeg2FilterSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TemporalFilterSettings {
    ///
    /// If you enable this filter, the results are the following: - If the source content is noisy (it contains excessive digital artifacts), the filter cleans up the source. - If the source content is already clean, the filter tends to decrease the bitrate, especially when the rate control mode is QVBR.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PostFilterSharpening")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_filter_sharpening: Option<cfn_resources::StrVal>,

    ///
    /// Choose a filter strength. We recommend a strength of 1 or 2. A higher strength might take out good information, resulting in an image that is overly soft.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Strength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TemporalFilterSettings {
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

/// The TimecodeBurninSettings property type specifies Property description not available. for an AWS::MediaLive::Channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TimecodeBurninSettings {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TimecodeBurninSettings {
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

/// The configuration of the timecode in the output.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TimecodeConfig {
    ///
    /// Identifies the source for the timecode that will be associated       with the channel outputs. Embedded (embedded): Initialize the output       timecode with timecode from the source. If no embedded timecode is       detected in the source, the system falls back to using "Start at 0"       (zerobased). System Clock (systemclock): Use the UTC time. Start at       0 (zerobased): The time of the first frame of the channel will be       00:00:00:00.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<cfn_resources::StrVal>,

    ///
    /// The threshold in frames beyond which output timecode is       resynchronized to the input timecode. Discrepancies below this       threshold are permitted to avoid unnecessary discontinuities in the       output timecode. There is no timecode sync when this is not       specified.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SyncThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_threshold: Option<i64>,
}

impl cfn_resources::CfnResource for TimecodeConfig {
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

/// The setup of TTML captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TtmlDestinationSettings {
    ///
    /// When set to passthrough, passes through style and position       information from a TTML-like input source (TTML, SMPTE-TT, CFF-TT)       to the CFF-TT output or TTML output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StyleControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_control: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TtmlDestinationSettings {
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

/// The configuration of a UDP output.
///
/// The parent of this entity is UdpOutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UdpContainerSettings {
    ///
    /// The M2TS configuration for this UDP output.
    ///
    /// Required: No
    ///
    /// Type: M2tsSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "M2tsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m2ts_settings: Option<M2tsSettings>,
}

impl cfn_resources::CfnResource for UdpContainerSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.m2ts_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration of a UDP output group.
///
/// The parent of this entity is OutputGroupSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UdpGroupSettings {
    ///
    /// Specifies the behavior of the last resort when the input video is       lost, and no more backup inputs are available. When dropTs is       selected, the entire transport stream stops emitting. When       dropProgram is selected, the program can be dropped from the       transport stream (and replaced with null packets to meet the TS       bitrate requirement). Or when emitProgram is selected, the transport       stream continues to be produced normally with repeat frames, black       frames, or slate frames substituted for the absent input       video.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLossAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<cfn_resources::StrVal>,

    ///
    /// Indicates the ID3 frame that has the timecode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataId3Frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_frame: Option<cfn_resources::StrVal>,

    ///
    /// The timed metadata interval in seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimedMetadataId3Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_period: Option<i64>,
}

impl cfn_resources::CfnResource for UdpGroupSettings {
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

/// The settings for one UDP output.
///
/// The parent of this entity is OutputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UdpOutputSettings {
    ///
    /// The UDP output buffering in milliseconds. Larger values increase       latency through the transcoder but simultaneously assist the       transcoder in maintaining a constant, low-jitter UDP/RTP output       while accommodating clock recovery, input switching, input       disruptions, picture reordering, and so on.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_msec: Option<i64>,

    ///
    /// The settings for the UDP output.
    ///
    /// Required: No
    ///
    /// Type: UdpContainerSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<UdpContainerSettings>,

    ///
    /// The destination address and port number for RTP or UDP packets.       These can be unicast or multicast RTP or UDP (for example,       rtp://239.10.10.10:5001 or udp://10.100.100.100:5002).
    ///
    /// Required: No
    ///
    /// Type: OutputLocationRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OutputLocationRef>,

    ///
    /// The settings for enabling and adjusting Forward Error Correction       on UDP outputs.
    ///
    /// Required: No
    ///
    /// Type: FecOutputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FecOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fec_output_settings: Option<FecOutputSettings>,
}

impl cfn_resources::CfnResource for UdpOutputSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.container_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.fec_output_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// MediaLive will perform a failover if content is considered black       for the specified period.
///
/// The parent of this entity is FailoverConditionSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VideoBlackFailoverSettings {
    ///
    /// A value used in calculating the threshold below which MediaLive considers a pixel to be 'black'. For the input to be considered black, every pixel in a frame must be below this threshold. The threshold is calculated as a percentage (expressed as a decimal) of white. Therefore .1 means 10% white (or 90% black). Note how the formula works for any color depth. For example, if you set this field to 0.1 in 10-bit color depth: (1023*0.1=102.3), which means a pixel value of 102 or less is 'black'. If you set this field to .1 in an 8-bit color depth: (255*0.1=25.5), which means a pixel value of 25 or less is 'black'. The range is 0.0 to 1.0, with any number of decimal places.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlackDetectThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_detect_threshold: Option<f64>,

    ///
    /// The amount of time (in milliseconds) that the active input must be black before automatic input failover occurs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoBlackThresholdMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_black_threshold_msec: Option<i64>,
}

impl cfn_resources::CfnResource for VideoBlackFailoverSettings {
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

/// The settings for the video codec in the output.
///
/// The parent of this entity is VideoDescription.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VideoCodecSettings {
    ///
    /// The settings for the video codec in a frame capture output.
    ///
    /// Required: No
    ///
    /// Type: FrameCaptureSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameCaptureSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_settings: Option<FrameCaptureSettings>,

    ///
    /// The settings for the H.264 codec in the output.
    ///
    /// Required: No
    ///
    /// Type: H264Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "H264Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h264_settings: Option<H264Settings>,

    ///
    /// Settings for video encoded with the H265 codec.
    ///
    /// Required: No
    ///
    /// Type: H265Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "H265Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_settings: Option<H265Settings>,

    ///
    /// Settings for video encoded with the MPEG-2 codec.
    ///
    /// Required: No
    ///
    /// Type: Mpeg2Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mpeg2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg2_settings: Option<Mpeg2Settings>,
}

impl cfn_resources::CfnResource for VideoCodecSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.frame_capture_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.h264_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.h265_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.mpeg2_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Encoding information for one output video.
///
/// The parent of this entity is EncoderSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VideoDescription {
    ///
    /// The video codec settings.
    ///
    /// Required: No
    ///
    /// Type: VideoCodecSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodecSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<VideoCodecSettings>,

    ///
    /// The output video height, in pixels. This must be an even number.       For most codecs, you can keep this field and width blank in order to       use the height and width (resolution) from the source. Note that we       don't recommend keeping the field blank. For the Frame Capture       codec, height and width are required.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    ///
    /// The name of this VideoDescription. Outputs use this name to       uniquely identify this description. Description names should be       unique within this channel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Indicates how to respond to the AFD values in the input stream.       RESPOND causes input video to be clipped, depending on the AFD       value, input display aspect ratio, and output display aspect ratio,       and (except for the FRAMECAPTURE codec) includes the values in the       output. PASSTHROUGH (does not apply to FRAMECAPTURE codec) ignores       the AFD values and includes the values in the output, so input video       is not clipped. NONE ignores the AFD values and does not include the       values through to the output, so input video is not clipped.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RespondToAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respond_to_afd: Option<cfn_resources::StrVal>,

    ///
    /// STRETCHTOOUTPUT configures the output position to stretch the       video to the specified output resolution (height and width). This       option overrides any position value. DEFAULT might insert black       boxes (pillar boxes or letter boxes) around the video to provide the       specified output resolution.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_behavior: Option<cfn_resources::StrVal>,

    ///
    /// Changes the strength of the anti-alias filter used for scaling. 0       is the softest setting, and 100 is the sharpest. We recommend a       setting of 50 for most content.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sharpness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<i64>,

    ///
    /// The output video width, in pixels. It must be an even number. For       most codecs, you can keep this field and height blank in order to       use the height and width (resolution) from the source. Note that we       don't recommend keeping the field blank. For the Frame Capture       codec, height and width are required.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl cfn_resources::CfnResource for VideoDescription {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.codec_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the video to extract from the input. An input       can contain only one video selector.
///
/// The parent of this entity is InputSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VideoSelector {
    ///
    /// Specifies the color space of an input. This setting works in       tandem with colorSpaceConversion to determine if MediaLive will       perform any conversion.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<cfn_resources::StrVal>,

    ///
    /// Settings to configure color space settings in the incoming       video.
    ///
    /// Required: No
    ///
    /// Type: VideoSelectorColorSpaceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSpaceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<VideoSelectorColorSpaceSettings>,

    ///
    /// Applies only if colorSpace is a value other than Follow. This       field controls how the value in the colorSpace field is used.       Fallback means that when the input does include color space data,       that data is used, but when the input has no color space data, the       value in colorSpace is used. Choose fallback if your input is       sometimes missing color space data, but when it does have color       space data, that data is correct. Force means to always use the       value in colorSpace. Choose force if your input usually has no color       space data or might have unreliable color space data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColorSpaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_usage: Option<cfn_resources::StrVal>,

    ///
    /// Information about the video to select from the content.
    ///
    /// Required: No
    ///
    /// Type: VideoSelectorSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectorSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<VideoSelectorSettings>,
}

impl cfn_resources::CfnResource for VideoSelector {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.color_space_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.selector_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings to configure color space settings in the incoming       video.
///
/// The parent of this entity is VideoSelector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VideoSelectorColorSpaceSettings {
    ///
    /// Settings to configure color space settings in the incoming       video.
    ///
    /// Required: No
    ///
    /// Type: Hdr10Settings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hdr10Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr10_settings: Option<Hdr10Settings>,
}

impl cfn_resources::CfnResource for VideoSelectorColorSpaceSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.hdr10_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Selects a specific PID from within a video source.
///
/// The parent of this entity is VideoSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VideoSelectorPid {
    ///
    /// Selects a specific PID from within a video source.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

impl cfn_resources::CfnResource for VideoSelectorPid {
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

/// Used to extract video by the program ID.
///
/// The parent of this entity is VideoSelectorSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VideoSelectorProgramId {
    ///
    /// Selects a specific program from within a multi-program transport       stream. If the program doesn't exist, MediaLive selects the first       program within the transport stream by default.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<i64>,
}

impl cfn_resources::CfnResource for VideoSelectorProgramId {
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

/// Information about the video to extract from the input.
///
/// The parent of this entity is VideoSelector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VideoSelectorSettings {
    ///
    /// Used to extract video by PID.
    ///
    /// Required: No
    ///
    /// Type: VideoSelectorPid
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoSelectorPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector_pid: Option<VideoSelectorPid>,

    ///
    /// Used to extract video by program ID.
    ///
    /// Required: No
    ///
    /// Type: VideoSelectorProgramId
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoSelectorProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector_program_id: Option<VideoSelectorProgramId>,
}

impl cfn_resources::CfnResource for VideoSelectorSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.video_selector_pid
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.video_selector_program_id
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings to enable VPC mode in the channel, so that the endpoints       for all outputs are in your VPC.
///
/// This entity is at the top level in the channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcOutputSettings {
    ///
    /// List of public address allocation IDs to associate with ENIs that       will be created in Output VPC. Must specify one for SINGLE_PIPELINE,       two for STANDARD channels
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicAddressAllocationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_address_allocation_ids: Option<Vec<String>>,

    ///
    /// A list of up to 5 EC2 VPC security group IDs to attach to the Output VPC network interfaces. If none are specified then the VPC default security group will be used
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of VPC subnet IDs from the same VPC. If STANDARD channel, subnet IDs must be mapped to two unique availability zones (AZ).
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for VpcOutputSettings {
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

/// The setup of WAV audio in the output.
///
/// The parent of this entity is AudioCodecSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WavSettings {
    ///
    /// Bits per sample.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "BitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<f64>,

    ///
    /// The audio coding mode for the WAV audio. The mode determines the number of channels in the audio.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<cfn_resources::StrVal>,

    ///
    /// Sample rate in Hz.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
}

impl cfn_resources::CfnResource for WavSettings {
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

/// The configuration of Web VTT captions in the output.
///
/// The parent of this entity is CaptionDestinationSettings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebvttDestinationSettings {
    ///
    /// Controls whether the color and position of the source captions is passed through to the WebVTT output captions. PASSTHROUGH - Valid only if the source captions are EMBEDDED or TELETEXT. NO_STYLE_DATA - Don't pass through the style. The output captions will not contain any font styling information.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StyleControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_control: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for WebvttDestinationSettings {
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
