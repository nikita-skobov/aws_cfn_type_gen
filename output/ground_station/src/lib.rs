
pub mod cfn_config {

#[derive(serde::Serialize, Default)]
pub struct CfnConfig {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigData")]
    pub config_data: ConfigData,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}

pub type BucketArn = String;pub type EirpUnits = String;
#[derive(serde::Serialize, Default)]
pub struct DataflowEndpointConfig {
    #[serde(rename = "DataflowEndpointRegion")]
    pub dataflow_endpoint_region: Option<String>,
    #[serde(rename = "DataflowEndpointName")]
    pub dataflow_endpoint_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SpectrumConfig {
    #[serde(rename = "CenterFrequency")]
    pub center_frequency: Option<Frequency>,
    #[serde(rename = "Polarization")]
    pub polarization: Option<Polarization>,
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Option<FrequencyBandwidth>,

}

#[derive(serde::Serialize, Default)]
pub struct UplinkEchoConfig {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "AntennaUplinkConfigArn")]
    pub antenna_uplink_config_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct S3RecordingConfig {
    #[serde(rename = "BucketArn")]
    pub bucket_arn: Option<BucketArn>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<RoleArn>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<S3KeyPrefix>,

}

#[derive(serde::Serialize, Default)]
pub struct DemodulationConfig {
    #[serde(rename = "UnvalidatedJSON")]
    pub unvalidated_json: Option<JsonString>,

}

#[derive(serde::Serialize, Default)]
pub struct Frequency {
    #[serde(rename = "Value")]
    pub value: Option<f64>,
    #[serde(rename = "Units")]
    pub units: Option<FrequencyUnits>,

}

#[derive(serde::Serialize, Default)]
pub struct UplinkSpectrumConfig {
    #[serde(rename = "Polarization")]
    pub polarization: Option<Polarization>,
    #[serde(rename = "CenterFrequency")]
    pub center_frequency: Option<Frequency>,

}
pub type BandwidthUnits = String;
#[derive(serde::Serialize, Default)]
pub struct DecodeConfig {
    #[serde(rename = "UnvalidatedJSON")]
    pub unvalidated_json: Option<JsonString>,

}

#[derive(serde::Serialize, Default)]
pub struct FrequencyBandwidth {
    #[serde(rename = "Value")]
    pub value: Option<f64>,
    #[serde(rename = "Units")]
    pub units: Option<BandwidthUnits>,

}

#[derive(serde::Serialize, Default)]
pub struct TrackingConfig {
    #[serde(rename = "Autotrack")]
    pub autotrack: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigData {
    #[serde(rename = "S3RecordingConfig")]
    pub s3_recording_config: Option<S3RecordingConfig>,
    #[serde(rename = "AntennaDownlinkConfig")]
    pub antenna_downlink_config: Option<AntennaDownlinkConfig>,
    #[serde(rename = "UplinkEchoConfig")]
    pub uplink_echo_config: Option<UplinkEchoConfig>,
    #[serde(rename = "TrackingConfig")]
    pub tracking_config: Option<TrackingConfig>,
    #[serde(rename = "DataflowEndpointConfig")]
    pub dataflow_endpoint_config: Option<DataflowEndpointConfig>,
    #[serde(rename = "AntennaDownlinkDemodDecodeConfig")]
    pub antenna_downlink_demod_decode_config: Option<AntennaDownlinkDemodDecodeConfig>,
    #[serde(rename = "AntennaUplinkConfig")]
    pub antenna_uplink_config: Option<AntennaUplinkConfig>,

}
pub type Polarization = String;
#[derive(serde::Serialize, Default)]
pub struct AntennaDownlinkDemodDecodeConfig {
    #[serde(rename = "DecodeConfig")]
    pub decode_config: Option<DecodeConfig>,
    #[serde(rename = "SpectrumConfig")]
    pub spectrum_config: Option<SpectrumConfig>,
    #[serde(rename = "DemodulationConfig")]
    pub demodulation_config: Option<DemodulationConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct Eirp {
    #[serde(rename = "Value")]
    pub value: Option<f64>,
    #[serde(rename = "Units")]
    pub units: Option<EirpUnits>,

}
pub type FrequencyUnits = String;pub type RoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct AntennaUplinkConfig {
    #[serde(rename = "TargetEirp")]
    pub target_eirp: Option<Eirp>,
    #[serde(rename = "TransmitDisabled")]
    pub transmit_disabled: Option<bool>,
    #[serde(rename = "SpectrumConfig")]
    pub spectrum_config: Option<UplinkSpectrumConfig>,

}
pub type JsonString = String;pub type S3KeyPrefix = String;
#[derive(serde::Serialize, Default)]
pub struct AntennaDownlinkConfig {
    #[serde(rename = "SpectrumConfig")]
    pub spectrum_config: Option<SpectrumConfig>,

}


}

pub mod cfn_dataflow_endpoint_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDataflowEndpointGroup {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Amount of time, in seconds, before a contact starts that the Ground Station Dataflow Endpoint Group will be in a PREPASS state. A Ground Station Dataflow Endpoint Group State Change event will be emitted when the Dataflow Endpoint Group enters and exits the PREPASS state.
    #[serde(rename = "ContactPrePassDurationSeconds")]
    pub contact_pre_pass_duration_seconds: Option<usize>,
    /// Amount of time, in seconds, after a contact ends that the Ground Station Dataflow Endpoint Group will be in a POSTPASS state. A Ground Station Dataflow Endpoint Group State Change event will be emitted when the Dataflow Endpoint Group enters and exits the POSTPASS state.
    #[serde(rename = "ContactPostPassDurationSeconds")]
    pub contact_post_pass_duration_seconds: Option<usize>,
    /// List of EndpointDetails
    #[serde(rename = "EndpointDetails")]
    pub endpoint_details: Vec<EndpointDetails>,

}


#[derive(serde::Serialize, Default)]
pub struct SocketAddress {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RangedSocketAddress {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "PortRange")]
    pub port_range: Option<IntegerRange>,

}
pub type AuditResults = String;
#[derive(serde::Serialize, Default)]
pub struct ConnectionDetails {
    #[serde(rename = "SocketAddress")]
    pub socket_address: Option<SocketAddress>,
    #[serde(rename = "Mtu")]
    pub mtu: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RangedConnectionDetails {
    #[serde(rename = "Mtu")]
    pub mtu: Option<usize>,
    #[serde(rename = "SocketAddress")]
    pub socket_address: Option<RangedSocketAddress>,

}

#[derive(serde::Serialize, Default)]
pub struct EndpointDetails {
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<DataflowEndpoint>,
    #[serde(rename = "AwsGroundStationAgentEndpoint")]
    pub aws_ground_station_agent_endpoint: Option<AwsGroundStationAgentEndpoint>,
    #[serde(rename = "SecurityDetails")]
    pub security_details: Option<SecurityDetails>,

}
pub type AgentStatus = String;
#[derive(serde::Serialize, Default)]
pub struct AwsGroundStationAgentEndpoint {
    #[serde(rename = "AuditResults")]
    pub audit_results: Option<AuditResults>,
    #[serde(rename = "IngressAddress")]
    pub ingress_address: Option<RangedConnectionDetails>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "AgentStatus")]
    pub agent_status: Option<AgentStatus>,
    #[serde(rename = "EgressAddress")]
    pub egress_address: Option<ConnectionDetails>,

}

#[derive(serde::Serialize, Default)]
pub struct DataflowEndpoint {
    #[serde(rename = "Mtu")]
    pub mtu: Option<usize>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Address")]
    pub address: Option<SocketAddress>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerRange {
    #[serde(rename = "Minimum")]
    pub minimum: Option<usize>,
    #[serde(rename = "Maximum")]
    pub maximum: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SecurityDetails {
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_mission_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnMissionProfile {
    /// Post-pass time needed after the contact.
    #[serde(rename = "ContactPostPassDurationSeconds")]
    pub contact_post_pass_duration_seconds: Option<usize>,
    /// The ARN of a KMS Key used for encrypting data during transmission from the source to destination locations.
    #[serde(rename = "StreamsKmsKey")]
    pub streams_kms_key: Option<StreamsKmsKey>,
    /// Pre-pass time needed before the contact.
    #[serde(rename = "ContactPrePassDurationSeconds")]
    pub contact_pre_pass_duration_seconds: Option<usize>,
    /// Visibilities with shorter duration than the specified minimum viable contact duration will be ignored when searching for available contacts.
    #[serde(rename = "MinimumViableContactDurationSeconds")]
    pub minimum_viable_contact_duration_seconds: usize,
    /// The ARN of the KMS Key or Alias Key role used to define permissions on KMS Key usage.
    #[serde(rename = "StreamsKmsRole")]
    pub streams_kms_role: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of DataflowEdge
    #[serde(rename = "DataflowEdges")]
    pub dataflow_edges: Vec<DataflowEdge>,
    /// A name used to identify a mission profile.
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "TrackingConfigArn")]
    pub tracking_config_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct StreamsKmsKey {
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "KmsAliasArn")]
    pub kms_alias_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataflowEdge {
    #[serde(rename = "Destination")]
    pub destination: Option<String>,
    #[serde(rename = "Source")]
    pub source: Option<String>,

}


}
