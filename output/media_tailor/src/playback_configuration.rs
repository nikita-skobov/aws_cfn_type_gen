

/// Adds a new playback configuration to AWS Elemental MediaTailor.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPlaybackConfiguration {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdDecisionServerUrl")]
    pub ad_decision_server_url: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AvailSuppression
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailSuppression")]
    pub avail_suppression: Option<AvailSuppression>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Bumper
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bumper")]
    pub bumper: Option<Bumper>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: CdnConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CdnConfiguration")]
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
    pub configuration_aliases: Option<std::collections::HashMap<String, serde_json::Value>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DashConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DashConfiguration")]
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
    pub hls_configuration: Option<HlsConfiguration>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: LivePreRollConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LivePreRollConfiguration")]
    pub live_pre_roll_configuration: Option<LivePreRollConfiguration>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ManifestProcessingRules
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManifestProcessingRules")]
    pub manifest_processing_rules: Option<ManifestProcessingRules>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PersonalizationThresholdSeconds")]
    pub personalization_threshold_seconds: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SlateAdUrl")]
    pub slate_ad_url: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TranscodeProfileName")]
    pub transcode_profile_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VideoContentSourceUrl")]
    pub video_content_source_url: String,

}



impl cfn_resources::CfnResource for CfnPlaybackConfiguration {
    fn type_string() -> &'static str {
        "AWS::MediaTailor::PlaybackConfiguration"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The AdMarkerPassthrough property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdMarkerPassthrough {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}




/// The AvailSuppression property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AvailSuppression {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    pub mode: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}




/// The Bumper property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Bumper {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndUrl")]
    pub end_url: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartUrl")]
    pub start_url: Option<String>,

}




/// The CdnConfiguration property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CdnConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdSegmentUrlPrefix")]
    pub ad_segment_url_prefix: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentSegmentUrlPrefix")]
    pub content_segment_url_prefix: Option<String>,

}




/// The configuration for DASH content.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub manifest_endpoint_prefix: Option<String>,


    /// 
    /// The setting that controls whether MediaTailor includes the Location tag in DASH manifests. MediaTailor populates the Location tag with the URL for manifest update requests, to be used by players that don't support sticky redirects. Disable this if you have CDN routing rules set up for accessing MediaTailor manifests, and you are either using client-side reporting or your players support sticky HTTP redirects. Valid values are DISABLED and EMT_DEFAULT. The EMT_DEFAULT setting enables the inclusion of the tag and is the default value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MpdLocation")]
    pub mpd_location: Option<String>,


    /// 
    /// The setting that controls whether MediaTailor handles manifests from the origin server as multi-period manifests or single-period manifests. If your origin server produces single-period manifests, set this to SINGLE_PERIOD. The default setting is MULTI_PERIOD. For multi-period manifests, omit this setting or set it to MULTI_PERIOD.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginManifestType")]
    pub origin_manifest_type: Option<String>,

}




/// The configuration for HLS content.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub manifest_endpoint_prefix: Option<String>,

}




/// The LivePreRollConfiguration property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LivePreRollConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdDecisionServerUrl")]
    pub ad_decision_server_url: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxDurationSeconds")]
    pub max_duration_seconds: Option<i64>,

}




/// The ManifestProcessingRules property type specifies Property description not available. for an AWS::MediaTailor::PlaybackConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ManifestProcessingRules {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AdMarkerPassthrough
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdMarkerPassthrough")]
    pub ad_marker_passthrough: Option<AdMarkerPassthrough>,

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
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


