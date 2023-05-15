
pub mod cfn_accelerator {

#[derive(serde::Serialize, Default)]
pub struct CfnAccelerator {
    /// IP Address type.
    #[serde(rename = "IpAddressType")]
    pub ip_address_type: Option<String>,
    /// Indicates whether an accelerator is enabled. The value is true or false.
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The IP addresses from BYOIP Prefix pool.
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Option<Vec<IpAddress>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type IpAddress = String;

}

pub mod cfn_endpoint_group {

#[derive(serde::Serialize, Default)]
pub struct CfnEndpointGroup {
    /// The Amazon Resource Name (ARN) of the listener
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,
    /// The list of endpoint objects.
    #[serde(rename = "EndpointConfigurations")]
    pub endpoint_configurations: Option<Vec<EndpointConfiguration>>,
    /// List of PortOverride
    #[serde(rename = "PortOverrides")]
    pub port_overrides: Option<Vec<PortOverride>>,
    /// The name of the AWS Region where the endpoint group is located
    #[serde(rename = "EndpointGroupRegion")]
    pub endpoint_group_region: String,
    /// The number of consecutive health checks required to set the state of the endpoint to unhealthy.
    #[serde(rename = "ThresholdCount")]
    pub threshold_count: Option<usize>,
    /// The time in seconds between each health check for an endpoint. Must be a value of 10 or 30
    #[serde(rename = "HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: Option<usize>,
    /// The port that AWS Global Accelerator uses to check the health of endpoints in this endpoint group.
    #[serde(rename = "HealthCheckPort")]
    pub health_check_port: Option<usize>,
    /// The protocol that AWS Global Accelerator uses to check the health of endpoints in this endpoint group.
    #[serde(rename = "HealthCheckProtocol")]
    pub health_check_protocol: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HealthCheckPath")]
    pub health_check_path: Option<String>,
    /// The percentage of traffic to sent to an AWS Region
    #[serde(rename = "TrafficDialPercentage")]
    pub traffic_dial_percentage: Option<f64>,

}

pub type Port = usize;
#[derive(serde::Serialize, Default)]
pub struct EndpointConfiguration {
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,
    #[serde(rename = "ClientIPPreservationEnabled")]
    pub client_ippreservation_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PortOverride {
    #[serde(rename = "ListenerPort")]
    pub listener_port: Port,
    #[serde(rename = "EndpointPort")]
    pub endpoint_port: Port,

}


}

pub mod cfn_listener {

#[derive(serde::Serialize, Default)]
pub struct CfnListener {
    /// The Amazon Resource Name (ARN) of the accelerator.
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
    /// The protocol for the listener.
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// Client affinity lets you direct all requests from a user to the same endpoint.
    #[serde(rename = "ClientAffinity")]
    pub client_affinity: Option<String>,
    /// List of PortRange
    #[serde(rename = "PortRanges")]
    pub port_ranges: Vec<PortRange>,

}

pub type Port = usize;
#[derive(serde::Serialize, Default)]
pub struct PortRange {
    #[serde(rename = "ToPort")]
    pub to_port: Port,
    #[serde(rename = "FromPort")]
    pub from_port: Port,

}


}
