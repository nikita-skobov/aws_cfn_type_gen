
pub mod cfn_flow {

#[derive(serde::Serialize, Default)]
pub struct CfnFlow {
    /// The source failover config of the flow.
    #[serde(rename = "SourceFailoverConfig")]
    pub source_failover_config: Option<FailoverConfig>,
    /// The name of the flow.
    #[serde(rename = "Name")]
    pub name: String,
    /// The source of the flow.
    #[serde(rename = "Source")]
    pub source: Source,

}


#[derive(serde::Serialize, Default)]
pub struct Encryption {
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,
    #[serde(rename = "Algorithm")]
    pub algorithm: Option<String>,
    #[serde(rename = "ConstantInitializationVector")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,
    #[serde(rename = "Url")]
    pub url: Option<String>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,
    #[serde(rename = "Region")]
    pub region: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Source {
    #[serde(rename = "SenderControlPort")]
    pub sender_control_port: Option<usize>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "SenderIpAddress")]
    pub sender_ip_address: Option<String>,
    #[serde(rename = "MinLatency")]
    pub min_latency: Option<usize>,
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: Option<String>,
    #[serde(rename = "SourceListenerAddress")]
    pub source_listener_address: Option<String>,
    #[serde(rename = "SourceArn")]
    pub source_arn: Option<String>,
    #[serde(rename = "SourceListenerPort")]
    pub source_listener_port: Option<usize>,
    #[serde(rename = "IngestPort")]
    pub ingest_port: Option<usize>,
    #[serde(rename = "WhitelistCidr")]
    pub whitelist_cidr: Option<String>,
    #[serde(rename = "Decryption")]
    pub decryption: Option<Encryption>,
    #[serde(rename = "VpcInterfaceName")]
    pub vpc_interface_name: Option<String>,
    #[serde(rename = "MaxLatency")]
    pub max_latency: Option<usize>,
    #[serde(rename = "StreamId")]
    pub stream_id: Option<String>,
    #[serde(rename = "MaxBitrate")]
    pub max_bitrate: Option<usize>,
    #[serde(rename = "SourceIngestPort")]
    pub source_ingest_port: Option<String>,
    #[serde(rename = "IngestIp")]
    pub ingest_ip: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FailoverConfig {
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "FailoverMode")]
    pub failover_mode: Option<String>,
    #[serde(rename = "RecoveryWindow")]
    pub recovery_window: Option<usize>,
    #[serde(rename = "SourcePriority")]
    pub source_priority: Option<()>,

}


}

pub mod cfn_flow_entitlement {

#[derive(serde::Serialize, Default)]
pub struct CfnFlowEntitlement {
    /// Percentage from 0-100 of the data transfer cost to be billed to the subscriber.
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    pub data_transfer_subscriber_fee_percent: Option<usize>,
    /// The name of the entitlement.
    #[serde(rename = "Name")]
    pub name: String,
    /// An indication of whether the entitlement is enabled.
    #[serde(rename = "EntitlementStatus")]
    pub entitlement_status: Option<String>,
    /// The AWS account IDs that you want to share your content with. The receiving accounts (subscribers) will be allowed to create their own flow using your content as the source.
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<String>,
    /// The type of encryption that will be used on the output that is associated with this entitlement.
    #[serde(rename = "Encryption")]
    pub encryption: Option<Encryption>,
    /// The ARN of the flow.
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// A description of the entitlement.
    #[serde(rename = "Description")]
    pub description: String,

}


#[derive(serde::Serialize, Default)]
pub struct Encryption {
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,
    #[serde(rename = "ConstantInitializationVector")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Url")]
    pub url: Option<String>,

}


}

pub mod cfn_flow_output {

#[derive(serde::Serialize, Default)]
pub struct CfnFlowOutput {
    /// The type of key used for the encryption. If no keyType is provided, the service will use the default setting (static-key).
    #[serde(rename = "Encryption")]
    pub encryption: Option<Encryption>,
    /// The name of the output. This value must be unique within the current flow.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The protocol that is used by the source or output.
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.
    #[serde(rename = "StreamId")]
    pub stream_id: Option<String>,
    /// The maximum latency in milliseconds. This parameter applies only to RIST-based and Zixi-based streams.
    #[serde(rename = "MaxLatency")]
    pub max_latency: Option<usize>,
    /// The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.
    #[serde(rename = "SmoothingLatency")]
    pub smoothing_latency: Option<usize>,
    /// The remote ID for the Zixi-pull stream.
    #[serde(rename = "RemoteId")]
    pub remote_id: Option<String>,
    /// A description of the output.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The Amazon Resource Name (ARN), a unique identifier for any AWS resource, of the flow.
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// The port to use when content is distributed to this output.
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    /// The name of the VPC interface attachment to use for this output.
    #[serde(rename = "VpcInterfaceAttachment")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
    /// The address where you want to send the output.
    #[serde(rename = "Destination")]
    pub destination: Option<String>,
    /// The range of IP addresses that should be allowed to initiate output requests to this flow. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.
    #[serde(rename = "CidrAllowList")]
    pub cidr_allow_list: Option<Vec<String>>,
    /// The minimum latency in milliseconds.
    #[serde(rename = "MinLatency")]
    pub min_latency: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct VpcInterfaceAttachment {
    #[serde(rename = "VpcInterfaceName")]
    pub vpc_interface_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Encryption {
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Algorithm")]
    pub algorithm: Option<String>,

}


}

pub mod cfn_flow_source {

#[derive(serde::Serialize, Default)]
pub struct CfnFlowSource {
    /// The port that the flow uses to send outbound requests to initiate connection with the sender for fujitsu-qos protocol.
    #[serde(rename = "SenderControlPort")]
    pub sender_control_port: Option<usize>,
    /// The ARN of the flow.
    #[serde(rename = "FlowArn")]
    pub flow_arn: Option<String>,
    /// The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.
    #[serde(rename = "StreamId")]
    pub stream_id: Option<String>,
    /// The minimum latency in milliseconds.
    #[serde(rename = "MinLatency")]
    pub min_latency: Option<usize>,
    /// The type of encryption that is used on the content ingested from this source.
    #[serde(rename = "Decryption")]
    pub decryption: Option<Encryption>,
    /// The range of IP addresses that should be allowed to contribute content to your source. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.
    #[serde(rename = "WhitelistCidr")]
    pub whitelist_cidr: Option<String>,
    /// The smoothing max bitrate for RIST, RTP, and RTP-FEC streams.
    #[serde(rename = "MaxBitrate")]
    pub max_bitrate: Option<usize>,
    /// The name of the source.
    #[serde(rename = "Name")]
    pub name: String,
    /// Source IP or domain name for SRT-caller protocol.
    #[serde(rename = "SourceListenerAddress")]
    pub source_listener_address: Option<String>,
    /// The protocol that is used by the source.
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    /// The IP address that the flow communicates with to initiate connection with the sender for fujitsu-qos protocol.
    #[serde(rename = "SenderIpAddress")]
    pub sender_ip_address: Option<String>,
    /// Source port for SRT-caller protocol.
    #[serde(rename = "SourceListenerPort")]
    pub source_listener_port: Option<usize>,
    /// The maximum latency in milliseconds. This parameter applies only to RIST-based and Zixi-based streams.
    #[serde(rename = "MaxLatency")]
    pub max_latency: Option<usize>,
    /// A description for the source. This value is not used or seen outside of the current AWS Elemental MediaConnect account.
    #[serde(rename = "Description")]
    pub description: String,
    /// The ARN of the entitlement that allows you to subscribe to content that comes from another AWS account. The entitlement is set by the content originator and the ARN is generated as part of the originator's flow.
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: Option<String>,
    /// The name of the VPC Interface this Source is configured with.
    #[serde(rename = "VpcInterfaceName")]
    pub vpc_interface_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Encryption {
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,
    #[serde(rename = "Url")]
    pub url: Option<String>,
    #[serde(rename = "Algorithm")]
    pub algorithm: Option<String>,
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,
    #[serde(rename = "ConstantInitializationVector")]
    pub constant_initialization_vector: Option<String>,

}


}

pub mod cfn_flow_vpc_interface {

#[derive(serde::Serialize, Default)]
pub struct CfnFlowVpcInterface {
    /// The Amazon Resource Name (ARN), a unique identifier for any AWS resource, of the flow.
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// Subnet must be in the AZ of the Flow
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// Role Arn MediaConnect can assumes to create ENIs in customer's account.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// Security Group IDs to be used on ENI.
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// Immutable and has to be a unique against other VpcInterfaces in this Flow.
    #[serde(rename = "Name")]
    pub name: String,

}



}
