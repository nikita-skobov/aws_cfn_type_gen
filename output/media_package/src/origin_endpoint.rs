/// Create an endpoint on an AWS Elemental MediaPackage channel.
///
/// An endpoint represents a single delivery point of a channel, and defines content output handling through various components, such as packaging protocols, DRM and encryption integration, and more.
///
/// After it's created, an endpoint provides a fixed public URL. This URL remains the same     throughout the lifetime of the endpoint, regardless of any failures or upgrades that might occur. Integrate the URL with a downstream CDN (such as Amazon CloudFront) or playback     device.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnOriginEndpoint {
    ///
    /// Parameters for CDN authorization.
    ///
    /// Required: No
    ///
    /// Type: Authorization
    ///
    /// Update requires: No interruption
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,

    ///
    /// The ID of the channel associated with this endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelId")]
    pub channel_id: cfn_resources::StrVal,

    ///
    /// Parameters for Common Media Application Format (CMAF) packaging.
    ///
    /// Required: No
    ///
    /// Type: CmafPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,

    ///
    /// Parameters for DASH packaging.
    ///
    /// Required: No
    ///
    /// Type: DashPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,

    ///
    /// Any descriptive information that you want to add to the endpoint for future identification purposes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Parameters for Apple HLS packaging.
    ///
    /// Required: No
    ///
    /// Type: HlsPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,

    ///
    /// The manifest ID is required and must be unique within the OriginEndpoint. The ID can't be changed after the endpoint is created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// A short string that's appended to the end of the endpoint URL to create a unique path to this endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<cfn_resources::StrVal>,

    ///
    /// Parameters for Microsoft Smooth Streaming packaging.
    ///
    /// Required: No
    ///
    /// Type: MssPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,

    ///
    /// Controls video origination from this endpoint.
    ///
    /// Valid values:
    ///
    /// ALLOW - enables this endpoint to serve content to requesting devices.             DENY - prevents this endpoint from serving content. Denying origination is helpful for harvesting live-to-VOD assets. For more information about harvesting and origination, see                Live-to-VOD Requirements.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<cfn_resources::StrVal>,

    ///
    /// Maximum duration (seconds) of content to retain for startover playback. Omit this attribute or enter 0 to indicate that startover playback is disabled for this endpoint.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,

    ///
    /// The tags to assign to the endpoint.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Minimum duration (seconds) of delay to enforce on the playback of live content. Omit this attribute or enter 0 to indicate that there is no time delay in effect for this endpoint.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,

    ///
    /// The IP addresses that can access this endpoint.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnOriginEndpointarn,

    #[serde(skip_serializing)]
    pub att_url: CfnOriginEndpointurl,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnOriginEndpointarn;
impl CfnOriginEndpointarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnOriginEndpointurl;
impl CfnOriginEndpointurl {
    pub fn att_name(&self) -> &'static str {
        r#"Url"#
    }
}

impl cfn_resources::CfnResource for CfnOriginEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::MediaPackage::OriginEndpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.authorization
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cmaf_package
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dash_package
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_package
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.mss_package
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Parameters for enabling CDN authorization on the endpoint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Authorization {
    ///
    /// The Amazon Resource Name (ARN) for the secret in AWS Secrets Manager that your Content Delivery Network (CDN) uses for authorization to access your endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdnIdentifierSecret")]
    pub cdn_identifier_secret: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) for the IAM role that allows AWS Elemental MediaPackage to communicate with AWS Secrets Manager.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretsRoleArn")]
    pub secrets_role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Authorization {
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

/// Holds encryption information so that access to the content can be controlled by a DRM solution.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CmafEncryption {
    ///
    /// An optional 128-bit, 16-byte hex value represented by a 32-character string, used in conjunction with the key for encrypting blocks. If you don't specify a value, then AWS Elemental MediaPackage creates the constant initialization vector (IV).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<cfn_resources::StrVal>,

    ///
    /// The encryption method to use.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<cfn_resources::StrVal>,

    ///
    /// Number of seconds before AWS Elemental MediaPackage rotates to a new key. By default, rotation is set to 60 seconds. Set to 0 to disable key rotation.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,

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

impl cfn_resources::CfnResource for CmafEncryption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.speke_key_provider.validate()?;

        Ok(())
    }
}

/// Parameters for Common Media Application Format (CMAF) packaging.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryption>,

    ///
    /// A list of HLS manifest configurations that are available from this endpoint.
    ///
    /// Required: No
    ///
    /// Type: List of HlsManifest
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<HlsManifest>>,

    ///
    /// Duration (in seconds) of each segment. Actual segments are rounded to the nearest multiple of the source segment duration.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,

    ///
    /// An optional custom string that is prepended to the name of each segment. If not specified, the segment prefix defaults to the ChannelId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_prefix: Option<cfn_resources::StrVal>,

    ///
    /// Limitations for outputs from the endpoint, based on the video bitrate.
    ///
    /// Required: No
    ///
    /// Type: StreamSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

impl cfn_resources::CfnResource for CmafPackage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.stream_selection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Holds encryption information so that access to the content can be controlled by a DRM solution.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DashEncryption {
    ///
    /// Number of seconds before AWS Elemental MediaPackage rotates to a new key. By default, rotation is set to 60 seconds. Set to 0 to disable key rotation.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,

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

impl cfn_resources::CfnResource for DashEncryption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.speke_key_provider.validate()?;

        Ok(())
    }
}

/// Parameters for DASH packaging.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DashPackage {
    ///
    /// Specifies the SCTE-35 message types that AWS Elemental MediaPackage treats as ad markers in the output manifest.
    ///
    /// Valid values:
    ///
    /// BREAK                           DISTRIBUTOR_ADVERTISEMENT                           DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY.                           DISTRIBUTOR_PLACEMENT_OPPORTUNITY.                           PROVIDER_ADVERTISEMENT.                           PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY.                           PROVIDER_PLACEMENT_OPPORTUNITY.                           SPLICE_INSERT.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,

    ///
    /// The flags on SCTE-35 segmentation descriptors that have to be present for AWS Elemental MediaPackage to insert ad markers in the output manifest. For information about SCTE-35 in AWS Elemental MediaPackage, see SCTE-35 Message Options in AWS Elemental MediaPackage.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<cfn_resources::StrVal>,

    ///
    /// Parameters for encrypting content.
    ///
    /// Required: No
    ///
    /// Type: DashEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashEncryption>,

    ///
    /// This applies only to stream sets with a single video track. When true, the stream set includes an         additional I-frame trick-play only stream, along with the other tracks. If false, this extra stream is not included.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,

    ///
    /// Determines the position of some tags in the manifest.
    ///
    /// Valid values:
    ///
    /// FULL - Elements like SegmentTemplate and ContentProtection are included in each Representation.                     COMPACT - Duplicate elements are combined and presented at the AdaptationSet level.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestLayout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_layout: Option<cfn_resources::StrVal>,

    ///
    /// Time window (in seconds) contained in each manifest.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i64>,

    ///
    /// Minimum amount of content (measured in seconds) that a player must keep available in the buffer.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinBufferTimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time_seconds: Option<i64>,

    ///
    /// Minimum amount of time (in seconds) that the player should wait before requesting updates to the manifest.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinUpdatePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_update_period_seconds: Option<i64>,

    ///
    /// Controls whether AWS Elemental MediaPackage produces single-period or multi-period DASH manifests. For more information about periods, see Multi-period DASH in AWS Elemental MediaPackage.
    ///
    /// Valid values:
    ///
    /// ADS - AWS Elemental MediaPackage will produce multi-period DASH manifests. Periods are created based on the SCTE-35 ad markers present in the input manifest.                     No value - AWS Elemental MediaPackage will produce single-period DASH manifests. This is the default setting.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeriodTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_triggers: Option<Vec<String>>,

    ///
    /// The DASH profile for the output.
    ///
    /// Valid values:
    ///
    /// NONE - The output doesn't use a DASH profile.                           HBBTV_1_5 - The output is compliant with HbbTV v1.5.                           DVB_DASH_2014 - The output is compliant with DVB-DASH 2014.
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
    /// Duration (in seconds) of each fragment. Actual fragments are rounded to the nearest multiple of the source fragment duration.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,

    ///
    /// Determines the type of variable used in the media URL of the SegmentTemplate tag in the manifest. Also specifies if segment timeline information is included in SegmentTimeline or SegmentTemplate.
    ///
    /// Valid values:
    ///
    /// NUMBER_WITH_TIMELINE - The $Number$ variable is used in the media URL. The value of this variable is the sequential number of the segment. A full SegmentTimeline object is presented in each SegmentTemplate.                     NUMBER_WITH_DURATION - The $Number$ variable is used in the media URL and a duration attribute is added to        the segment template. The SegmentTimeline object is removed from the representation.                      TIME_WITH_TIMELINE - The $Time$ variable is used in the media URL. The value of this variable is the timestamp of when the segment starts. A full SegmentTimeline object is presented in each SegmentTemplate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentTemplateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_template_format: Option<cfn_resources::StrVal>,

    ///
    /// Limitations for outputs from the endpoint, based on the video bitrate.
    ///
    /// Required: No
    ///
    /// Type: StreamSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,

    ///
    /// Amount of time (in seconds) that the player should be from the live point at the end of the manifest.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuggestedPresentationDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_presentation_delay_seconds: Option<i64>,

    ///
    /// Determines the type of UTC timing included in the DASH Media Presentation Description (MPD).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UtcTiming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_timing: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the value attribute of the UTC timing field when utcTiming is set to HTTP-ISO or HTTP-HEAD.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UtcTimingUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_timing_uri: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DashPackage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.stream_selection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Use encryptionContractConfiguration to configure one or more content        encryption keys for your endpoints that use SPEKE Version 2.0. The encryption contract        defines the content keys used to encrypt the audio and video tracks in your stream.        To configure the encryption contract, specify which audio and video encryption        presets to use. For more information about these presets, see SPEKE Version 2.0 Presets.
///
/// Note the following considerations when using        encryptionContractConfiguration:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncryptionContractConfiguration {}

impl cfn_resources::CfnResource for EncryptionContractConfiguration {
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

/// Holds encryption information so that access to the content can be controlled by a DRM solution.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HlsEncryption {
    ///
    /// A 128-bit, 16-byte hex value represented by a 32-character string, used with the key for encrypting blocks.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<cfn_resources::StrVal>,

    ///
    /// HLS encryption type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<cfn_resources::StrVal>,

    ///
    /// Number of seconds before AWS Elemental MediaPackage rotates to a new key. By default, rotation is set to 60 seconds. Set to 0 to disable key rotation.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,

    ///
    /// Repeat the EXT-X-KEY directive for every media segment. This     might result in an increase in client requests to the DRM server.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepeatExtXKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_ext_xkey: Option<bool>,

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

impl cfn_resources::CfnResource for HlsEncryption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.speke_key_provider.validate()?;

        Ok(())
    }
}

/// An HTTP Live Streaming (HLS) manifest configuration on a CMAF endpoint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HlsManifest {
    ///
    /// Controls how ad markers are included in the packaged endpoint.
    ///
    /// Valid values:
    ///
    /// NONE - Omits all SCTE-35 ad markers from the output.                           PASSTHROUGH - Creates a copy in the output of the SCTE-35 ad markers (comments) taken directly from the input manifest.                           SCTE35_ENHANCED - Generates ad markers and blackout tags in the output based on the SCTE-35 messages from the input manifest.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the SCTE-35 message types that AWS Elemental MediaPackage treats as ad markers in the output manifest.
    ///
    /// Valid values:
    ///
    /// BREAK                   DISTRIBUTOR_ADVERTISEMENT                   DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY                   DISTRIBUTOR_PLACEMENT_OPPORTUNITY                   PROVIDER_ADVERTISEMENT                   PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY                   PROVIDER_PLACEMENT_OPPORTUNITY                   SPLICE_INSERT
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,

    ///
    /// The flags on SCTE-35 segmentation descriptors that have to be present for AWS Elemental MediaPackage to insert ad markers in the output manifest. For information about SCTE-35 in AWS Elemental MediaPackage, see SCTE-35 Message Options in AWS Elemental MediaPackage.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<cfn_resources::StrVal>,

    ///
    /// The manifest ID is required and must be unique within the OriginEndpoint. The ID can't be changed after the endpoint is created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// Applies to stream sets with a single video track only. When true, the stream set includes an additional I-frame only stream, along with the other tracks. If false, this extra stream is not included.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,

    ///
    /// A short string that's appended to the end of the endpoint URL to create a unique path to this endpoint. The manifestName on the HLSManifest object overrides the     manifestName that you provided on the originEndpoint object.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<cfn_resources::StrVal>,

    ///
    /// When specified as either event or vod, a     corresponding EXT-X-PLAYLIST-TYPE entry is included in the media playlist.     Indicates if the playlist is live-to-VOD content.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlaylistType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<cfn_resources::StrVal>,

    ///
    /// Time window (in seconds) contained in each parent manifest.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlaylistWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i64>,

    ///
    /// Inserts EXT-X-PROGRAM-DATE-TIME tags in the output manifest at the interval that you specify. Additionally, ID3Timed metadata messages are generated every     5 seconds starting when the content was ingested.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i64>,

    ///
    /// The URL that's used to request this manifest from this endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HlsManifest {
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

/// Parameters for Apple HLS packaging.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HlsPackage {
    ///
    /// Controls how ad markers are included in the packaged endpoint.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the SCTE-35 message types that AWS Elemental MediaPackage treats as ad markers in the output manifest.
    ///
    /// Valid values:
    ///
    /// BREAK                      DISTRIBUTOR_ADVERTISEMENT                      DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY                       DISTRIBUTOR_PLACEMENT_OPPORTUNITY                       PROVIDER_ADVERTISEMENT                       PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY                        PROVIDER_PLACEMENT_OPPORTUNITY                        SPLICE_INSERT
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,

    ///
    /// The flags on SCTE-35 segmentation descriptors that have to be present for AWS Elemental MediaPackage to insert ad markers in the output manifest. For information about SCTE-35 in AWS Elemental MediaPackage, see SCTE-35 Message Options in AWS Elemental MediaPackage.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<cfn_resources::StrVal>,

    ///
    /// Parameters for encrypting content.
    ///
    /// Required: No
    ///
    /// Type: HlsEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<HlsEncryption>,

    ///
    /// When enabled, MediaPackage passes through digital video broadcasting (DVB) subtitles into the output.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeDvbSubtitles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_dvb_subtitles: Option<bool>,

    ///
    /// Only applies to stream sets with a single video track. When true, the stream set includes an additional I-frame only stream, along with the other tracks. If false, this extra stream is not included.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,

    ///
    /// When specified as either event or vod, a corresponding EXT-X-PLAYLIST-TYPE entry is included in the media playlist.     Indicates if the playlist is live-to-VOD content.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlaylistType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<cfn_resources::StrVal>,

    ///
    /// Time window (in seconds) contained in each parent manifest.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlaylistWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i64>,

    ///
    /// Inserts EXT-X-PROGRAM-DATE-TIME tags in the output manifest at the interval that you specify. Additionally, ID3Timed metadata messages are generated every     5 seconds starting when the content was ingested.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i64>,

    ///
    /// Duration (in seconds) of each fragment. Actual fragments are rounded to the nearest multiple of the source fragment duration.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,

    ///
    /// Limitations for outputs from the endpoint, based on the video bitrate.
    ///
    /// Required: No
    ///
    /// Type: StreamSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,

    ///
    /// When true, AWS Elemental MediaPackage bundles all audio tracks in a rendition group. All other tracks in the stream can be used with any audio rendition from the group.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseAudioRenditionGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_audio_rendition_group: Option<bool>,
}

impl cfn_resources::CfnResource for HlsPackage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.stream_selection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Holds encryption information so that access to the content can be controlled by a DRM solution.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for MssEncryption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.speke_key_provider.validate()?;

        Ok(())
    }
}

/// Parameters for Microsoft Smooth Streaming packaging.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MssPackage {
    ///
    /// Parameters for encrypting content.
    ///
    /// Required: No
    ///
    /// Type: MssEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<MssEncryption>,

    ///
    /// Time window (in seconds) contained in each manifest.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i64>,

    ///
    /// Duration (in seconds) of each fragment. Actual fragments are rounded to the nearest multiple of the source fragment duration.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,

    ///
    /// Limitations for outputs from the endpoint, based on the video bitrate.
    ///
    /// Required: No
    ///
    /// Type: StreamSelection
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

impl cfn_resources::CfnResource for MssPackage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.stream_selection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Key provider settings for DRM.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SpekeKeyProvider {
    ///
    /// The Amazon Resource Name (ARN) for the certificate that you imported to AWS Certificate Manager to add content key encryption to this endpoint. For this feature to work, your DRM key provider must support content key encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// Use encryptionContractConfiguration to configure one or more content encryption keys for your         endpoints that use SPEKE Version 2.0. The encryption contract defines which content keys are used to encrypt the         audio and video tracks in your stream. To configure the encryption contract, specify which audio and video encryption presets to use.
    ///
    /// Required: No
    ///
    /// Type: EncryptionContractConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionContractConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_contract_configuration: Option<EncryptionContractConfiguration>,

    ///
    /// Unique identifier for this endpoint, as it is configured in the key provider service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    pub resource_id: cfn_resources::StrVal,

    ///
    /// The ARN for the IAM role that's granted by the key provider to provide      access to the key provider API. This role must have a trust policy that allows AWS Elemental MediaPackage to assume the role, and it must have a sufficient permissions policy     to allow access to the specific key retrieval URL. Valid format: arn:aws:iam::{accountID}:role/{name}
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

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

    ///
    /// URL for the key provider’s key retrieval API endpoint. Must start with https://.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SpekeKeyProvider {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption_contract_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Limitations for outputs from the endpoint, based on the video bitrate.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_order: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for StreamSelection {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
