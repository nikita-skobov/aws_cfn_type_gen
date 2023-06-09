/// Adds a new playback configuration to AWS Elemental MediaTailor.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPlaybackConfiguration {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdDecisionServerUrl")]
    pub ad_decision_server_url: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AvailSuppression
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailSuppression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_suppression: Option<AvailSuppression>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Bumper
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bumper")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bumper: Option<Bumper>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: CdnConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,

    ///
    /// The player parameters and aliases used as dynamic variables during session initialization. For more information, see Domain Variables.
    ///
    /// Required: No
    ///
    /// Type: Map of Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aliases: Option<std::collections::HashMap<String, serde_json::Value>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DashConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_configuration: Option<DashConfiguration>,

    ///
    /// The configuration for HLS content.
    ///
    /// Required: No
    ///
    /// Type: HlsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "HlsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_configuration: Option<HlsConfiguration>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: LivePreRollConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LivePreRollConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_pre_roll_configuration: Option<LivePreRollConfiguration>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ManifestProcessingRules
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestProcessingRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_processing_rules: Option<ManifestProcessingRules>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PersonalizationThresholdSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_threshold_seconds: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TranscodeProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode_profile_name: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoContentSourceUrl")]
    pub video_content_source_url: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_dash_configuration_manifest_endpoint_prefix:
        CfnPlaybackConfigurationdashconfigurationmanifestendpointprefix,

    #[serde(skip_serializing)]
    pub att_hls_configuration_manifest_endpoint_prefix:
        CfnPlaybackConfigurationhlsconfigurationmanifestendpointprefix,

    #[serde(skip_serializing)]
    pub att_playback_configuration_arn: CfnPlaybackConfigurationplaybackconfigurationarn,

    #[serde(skip_serializing)]
    pub att_playback_endpoint_prefix: CfnPlaybackConfigurationplaybackendpointprefix,

    #[serde(skip_serializing)]
    pub att_session_initialization_endpoint_prefix:
        CfnPlaybackConfigurationsessioninitializationendpointprefix,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaybackConfigurationdashconfigurationmanifestendpointprefix;
impl CfnPlaybackConfigurationdashconfigurationmanifestendpointprefix {
    pub fn att_name(&self) -> &'static str {
        r#"DashConfiguration.ManifestEndpointPrefix"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaybackConfigurationhlsconfigurationmanifestendpointprefix;
impl CfnPlaybackConfigurationhlsconfigurationmanifestendpointprefix {
    pub fn att_name(&self) -> &'static str {
        r#"HlsConfiguration.ManifestEndpointPrefix"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaybackConfigurationplaybackconfigurationarn;
impl CfnPlaybackConfigurationplaybackconfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"PlaybackConfigurationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaybackConfigurationplaybackendpointprefix;
impl CfnPlaybackConfigurationplaybackendpointprefix {
    pub fn att_name(&self) -> &'static str {
        r#"PlaybackEndpointPrefix"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaybackConfigurationsessioninitializationendpointprefix;
impl CfnPlaybackConfigurationsessioninitializationendpointprefix {
    pub fn att_name(&self) -> &'static str {
        r#"SessionInitializationEndpointPrefix"#
    }
}

impl cfn_resources::CfnResource for CfnPlaybackConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::MediaTailor::PlaybackConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.avail_suppression
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.bumper.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.cdn_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dash_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.hls_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.live_pre_roll_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.manifest_processing_rules
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AdMarkerPassthrough property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdMarkerPassthrough {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl cfn_resources::CfnResource for AdMarkerPassthrough {
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

/// The AvailSuppression property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AvailSuppression {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AvailSuppression {
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

/// The Bumper property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Bumper {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_url: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_url: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Bumper {
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

/// The CdnConfiguration property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CdnConfiguration {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdSegmentUrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_segment_url_prefix: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentSegmentUrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_segment_url_prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CdnConfiguration {
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

/// The configuration for DASH content.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DashConfiguration {
    ///
    /// The URL generated by MediaTailor to initiate a playback session. The session uses server-side reporting. This setting is ignored in PUT operations.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_endpoint_prefix: Option<cfn_resources::StrVal>,

    ///
    /// The setting that controls whether MediaTailor includes the Location tag in DASH manifests. MediaTailor populates the Location tag with the URL for manifest update requests, to be used by players that don't support sticky redirects. Disable this if you have CDN routing rules set up for accessing MediaTailor manifests, and you are either using client-side reporting or your players support sticky HTTP redirects. Valid values are DISABLED and EMT_DEFAULT. The EMT_DEFAULT setting enables the inclusion of the tag and is the default value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MpdLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_location: Option<cfn_resources::StrVal>,

    ///
    /// The setting that controls whether MediaTailor handles manifests from the origin server as multi-period manifests or single-period manifests. If your origin server produces single-period manifests, set this to SINGLE_PERIOD. The default setting is MULTI_PERIOD. For multi-period manifests, omit this setting or set it to MULTI_PERIOD.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginManifestType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_manifest_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DashConfiguration {
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

/// The configuration for HLS content.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HlsConfiguration {
    ///
    /// The URL that is used to initiate a playback session for devices that support Apple HLS. The session uses server-side reporting.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_endpoint_prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HlsConfiguration {
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

/// The LivePreRollConfiguration property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LivePreRollConfiguration {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<i64>,
}

impl cfn_resources::CfnResource for LivePreRollConfiguration {
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

/// The ManifestProcessingRules property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ManifestProcessingRules {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AdMarkerPassthrough
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdMarkerPassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_marker_passthrough: Option<AdMarkerPassthrough>,
}

impl cfn_resources::CfnResource for ManifestProcessingRules {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ad_marker_passthrough
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
