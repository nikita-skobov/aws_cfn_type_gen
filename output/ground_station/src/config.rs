

/// Creates a Config with the specified parameters.
///
/// Config objects provide Ground Station with the details necessary in order to schedule and execute satellite contacts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConfig {


    /// 
    /// Object containing the parameters of a config.       Only one subtype may be specified per config.       See the subtype definitions for a description of each config subtype.
    /// 
    /// Required: Yes
    ///
    /// Type: ConfigData
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigData")]
    pub config_data: ConfigData,


    /// 
    /// The name of the config object.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Tags assigned to a resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnConfig {
    fn type_string() -> &'static str {
        "AWS::GroundStation::Config"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Provides information about how AWS Ground Station should configure an antenna for downlink during a contact.       Use an antenna downlink config in a mission profile to receive the downlink data in raw DigIF format.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AntennaDownlinkConfig {


    /// 
    /// Defines the spectrum configuration.
    /// 
    /// Required: No
    ///
    /// Type: SpectrumConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpectrumConfig")]
    pub spectrum_config: Option<SpectrumConfig>,

}




/// Provides information about how AWS Ground Station should configure an antenna for downlink during a contact.       Use an antenna downlink demod decode config in a mission profile to receive the downlink data that has been demodulated and decoded.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AntennaDownlinkDemodDecodeConfig {


    /// 
    /// Defines how the RF signal will be decoded.
    /// 
    /// Required: No
    ///
    /// Type: DecodeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecodeConfig")]
    pub decode_config: Option<DecodeConfig>,


    /// 
    /// Defines how the RF signal will be demodulated.
    /// 
    /// Required: No
    ///
    /// Type: DemodulationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DemodulationConfig")]
    pub demodulation_config: Option<DemodulationConfig>,


    /// 
    /// Defines the spectrum configuration.
    /// 
    /// Required: No
    ///
    /// Type: SpectrumConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpectrumConfig")]
    pub spectrum_config: Option<SpectrumConfig>,

}




/// Provides information about how AWS Ground Station should configure an antenna for uplink during a contact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AntennaUplinkConfig {


    /// 
    /// Defines the spectrum configuration.
    /// 
    /// Required: No
    ///
    /// Type: UplinkSpectrumConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpectrumConfig")]
    pub spectrum_config: Option<UplinkSpectrumConfig>,


    /// 
    /// The equivalent isotropically radiated power (EIRP) to use for uplink transmissions. Valid values are between 20.0 to 50.0 dBW.
    /// 
    /// Required: No
    ///
    /// Type: Eirp
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetEirp")]
    pub target_eirp: Option<Eirp>,


    /// 
    /// Whether or not uplink transmit is disabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransmitDisabled")]
    pub transmit_disabled: Option<bool>,

}




/// Config objects provide information to Ground Station about how to configure the antenna and how data flows during a contact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfigData {


    /// 
    /// Provides information for an antenna downlink config object.       Antenna downlink config objects are used to provide parameters for downlinks where no demodulation or decoding is performed by Ground Station (RF over IP downlinks).
    /// 
    /// Required: No
    ///
    /// Type: AntennaDownlinkConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AntennaDownlinkConfig")]
    pub antenna_downlink_config: Option<AntennaDownlinkConfig>,


    /// 
    /// Provides information for a downlink demod decode config object.       Downlink demod decode config objects are used to provide parameters for downlinks where the Ground Station service will demodulate and decode the downlinked data.
    /// 
    /// Required: No
    ///
    /// Type: AntennaDownlinkDemodDecodeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AntennaDownlinkDemodDecodeConfig")]
    pub antenna_downlink_demod_decode_config: Option<AntennaDownlinkDemodDecodeConfig>,


    /// 
    /// Provides information for an uplink config object.       Uplink config objects are used to provide parameters for uplink contacts.
    /// 
    /// Required: No
    ///
    /// Type: AntennaUplinkConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AntennaUplinkConfig")]
    pub antenna_uplink_config: Option<AntennaUplinkConfig>,


    /// 
    /// Provides information for a dataflow endpoint config object.       Dataflow endpoint config objects are used to provide parameters about which IP endpoint(s) to use during a contact.       Dataflow endpoints are where Ground Station sends data during a downlink contact and where Ground Station receives data to send to the satellite during an uplink contact.
    /// 
    /// Required: No
    ///
    /// Type: DataflowEndpointConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataflowEndpointConfig")]
    pub dataflow_endpoint_config: Option<DataflowEndpointConfig>,


    /// 
    /// Provides information for an S3 recording config object.       S3 recording config objects are used to provide parameters for S3 recording during downlink contacts.
    /// 
    /// Required: No
    ///
    /// Type: S3RecordingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3RecordingConfig")]
    pub s3_recording_config: Option<S3RecordingConfig>,


    /// 
    /// Provides information for a tracking config object.       Tracking config objects are used to provide parameters about how to track the satellite through the sky during a contact.
    /// 
    /// Required: No
    ///
    /// Type: TrackingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrackingConfig")]
    pub tracking_config: Option<TrackingConfig>,


    /// 
    /// Provides information for an uplink echo config object.       Uplink echo config objects are used to provide parameters for uplink echo during uplink contacts.
    /// 
    /// Required: No
    ///
    /// Type: UplinkEchoConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "UplinkEchoConfig")]
    pub uplink_echo_config: Option<UplinkEchoConfig>,

}




/// Provides information to AWS Ground Station about which IP endpoints to use during a contact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataflowEndpointConfig {


    /// 
    /// The name of the dataflow endpoint to use during contacts.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataflowEndpointName")]
    pub dataflow_endpoint_name: Option<String>,


    /// 
    /// The region of the dataflow endpoint to use during contacts. When omitted, Ground Station will use the region of the contact.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataflowEndpointRegion")]
    pub dataflow_endpoint_region: Option<String>,

}




/// Defines decoding settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DecodeConfig {


    /// 
    /// The decoding settings are in JSON format and define a set of steps to perform to decode the data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnvalidatedJSON")]
    pub unvalidated_json: Option<String>,

}




/// Defines demodulation settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DemodulationConfig {


    /// 
    /// The demodulation settings are in JSON format and define parameters for demodulation, for example which modulation scheme (e.g. PSK, QPSK, etc.) and matched filter to use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnvalidatedJSON")]
    pub unvalidated_json: Option<String>,

}




/// Defines an equivalent isotropically radiated power (EIRP).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Eirp {


    /// 
    /// The units of the EIRP.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Units")]
    pub units: Option<String>,


    /// 
    /// The value of the EIRP. Valid values are between 20.0 to 50.0 dBW.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}




/// Defines a frequency.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Frequency {


    /// 
    /// The units of the frequency.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Units")]
    pub units: Option<String>,


    /// 
    /// The value of the frequency. Valid values are between 2200 to 2300 MHz and 7750 to 8400 MHz for downlink and 2025 to 2120 MHz for uplink.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}




/// Defines a bandwidth.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FrequencyBandwidth {


    /// 
    /// The units of the bandwidth.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Units")]
    pub units: Option<String>,


    /// 
    /// The value of the bandwidth. AWS Ground Station currently has the following bandwidth limitations:
    ///
    /// For AntennaDownlinkDemodDecodeconfig, valid values are between 125 kHz to 650 MHz. For AntennaDownlinkconfig, valid values are between 10 kHz to 54 MHz. For AntennaUplinkConfig, valid values are between 10 kHz to 54 MHz.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}




/// Provides information about how AWS Ground Station should save downlink data to S3.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3RecordingConfig {


    /// 
    /// S3 Bucket where the data is written. The name of the S3 Bucket provided must begin with aws-groundstation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketArn")]
    pub bucket_arn: Option<String>,


    /// 
    /// The prefix of the S3 data object.        If you choose to use any optional keys for substitution, these values will be replaced with the corresponding information from your contact details.        For example, a prefix of {satellite_id}/{year}/{month}/{day}/ will replaced with fake_satellite_id/2021/01/10/
    /// 
    /// Optional keys for substitution: {satellite_id} | {config-name} | {config-id} | {year} | {month} | {day}
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// Defines the ARN of the role assumed for putting archives to S3.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,

}




/// Defines a spectrum.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SpectrumConfig {


    /// 
    /// The bandwidth of the spectrum. AWS Ground Station currently has the following bandwidth limitations:
    ///
    /// For AntennaDownlinkDemodDecodeconfig, valid values are between 125 kHz to 650 MHz. For AntennaDownlinkconfig, valid values are between 10 kHz to 54 MHz. For AntennaUplinkConfig, valid values are between 10 kHz to 54 MHz.
    /// 
    /// Required: No
    ///
    /// Type: FrequencyBandwidth
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Option<FrequencyBandwidth>,


    /// 
    /// The center frequency of the spectrum. Valid values are between 2200 to 2300 MHz and 7750 to 8400 MHz for downlink and 2025 to 2120 MHz for uplink.
    /// 
    /// Required: No
    ///
    /// Type: Frequency
    ///
    /// Update requires: No interruption
    #[serde(rename = "CenterFrequency")]
    pub center_frequency: Option<Frequency>,


    /// 
    /// The polarization of the spectrum. Valid values are "RIGHT_HAND" and "LEFT_HAND". Capturing both "RIGHT_HAND" and "LEFT_HAND" polarization requires two separate configs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Polarization")]
    pub polarization: Option<String>,

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




/// Provides information about how AWS Ground Station should track the satellite through the sky during a contact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrackingConfig {


    /// 
    /// Specifies whether or not to use autotrack.       REMOVED specifies that program track should only be used during the contact.       PREFERRED specifies that autotracking is preferred during the contact but fallback to program track if the signal is lost.       REQUIRED specifies that autotracking is required during the contact and not to use program track if the signal is lost.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Autotrack")]
    pub autotrack: Option<String>,

}




/// Provides information about how AWS Ground Station should echo back uplink transmissions to a dataflow endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UplinkEchoConfig {


    /// 
    /// Defines the ARN of the uplink config to echo back to a dataflow endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AntennaUplinkConfigArn")]
    pub antenna_uplink_config_arn: Option<String>,


    /// 
    /// Whether or not uplink echo is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}




/// Defines a uplink spectrum.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UplinkSpectrumConfig {


    /// 
    /// The center frequency of the spectrum. Valid values are between 2200 to 2300 MHz and 7750 to 8400 MHz for downlink and 2025 to 2120 MHz for uplink.
    /// 
    /// Required: No
    ///
    /// Type: Frequency
    ///
    /// Update requires: No interruption
    #[serde(rename = "CenterFrequency")]
    pub center_frequency: Option<Frequency>,


    /// 
    /// The polarization of the spectrum. Valid values are "RIGHT_HAND" and "LEFT_HAND".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Polarization")]
    pub polarization: Option<String>,

}


