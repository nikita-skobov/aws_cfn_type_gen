

/// The AWS::MediaConnect::Flow resource defines a connection between one or more video       sources and one or more outputs. For each flow, you specify the transport protocol to       use, encryption information, and details for any outputs or entitlements that you want.         AWS Elemental MediaConnect returns an ingest endpoint where you can       send your live video as a single unicast stream. The service replicates and distributes       the video to every output that you specify, whether inside or outside the AWS Cloud. You can also set up entitlements on a flow to allow other         AWS accounts to access your content.
#[derive(Default, serde::Serialize)]
pub struct CfnFlow {


    /// 
    /// The Availability Zone that you want to create the flow in. These options are        limited to the Availability Zones within the current AWS Region.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The name of the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The settings for source failover.
    /// 
    /// Required: No
    ///
    /// Type: FailoverConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFailoverConfig")]
    pub source_failover_config: Option<FailoverConfig>,


    /// 
    /// The settings for the source that you want to use for the new flow.
    /// 
    /// Required: Yes
    ///
    /// Type: Source
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: Source,

}


/// Information about the encryption of the flow.
#[derive(Default, serde::Serialize)]
pub struct Encryption {


    /// 
    /// The ARN of the secret that you created in AWS Secrets Manager to store the        encryption key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,


    /// 
    /// The value of one of the devices that you configured with your digital rights        management (DRM) platform key provider. This parameter is required for SPEKE        encryption and is not valid for static key encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,


    /// 
    /// The AWS Region that the API Gateway proxy endpoint was created in. This parameter        is required for SPEKE encryption and is not valid for static key encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: Option<String>,


    /// 
    /// The URL from the API Gateway proxy that you set up to talk to your key server.        This parameter is required for SPEKE encryption and is not valid for static key        encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: Option<String>,


    /// 
    /// The type of key that is used for the encryption. If you don't specify a          keyType value, the service uses the default setting            (static-key). Valid key types are: static-key, speke, and srt-password.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,


    /// 
    /// A 128-bit, 16-byte hex value represented by a 32-character string, to be used with        the key for encrypting content. This parameter is not valid for static key        encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantInitializationVector")]
    pub constant_initialization_vector: Option<String>,


    /// 
    /// The type of algorithm that is used for static key encryption (such as aes128, aes192, or        aes256). If you are using SPEKE or SRT-password encryption, this property must be left blank.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Algorithm")]
    pub algorithm: Option<String>,


    /// 
    /// An identifier for the content. The service sends this value to the key server to        identify the current endpoint. The resource ID is also known as the content ID. This        parameter is required for SPEKE encryption and is not valid for static key        encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the role that you created during setup (when you        set up MediaConnect as a trusted entity).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


/// The settings for source failover.
#[derive(Default, serde::Serialize)]
pub struct FailoverConfig {


    /// 
    /// The state of source failover on the flow. If the state is inactive, the flow can        have only one source. If the state is active, the flow can have one or two        sources.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: Option<String>,


    /// 
    /// The size of the buffer (delay) that the service maintains. A larger buffer means a        longer delay in transmitting the stream, but more room for error correction. A        smaller buffer means a shorter delay, but less room for error correction. You can choose a value from 100-500 ms. If you keep this field blank, the service uses the default value of 200 ms. This setting only applies when Failover Mode is set to MERGE.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecoveryWindow")]
    pub recovery_window: Option<i64>,


    /// 
    /// The priority you want to assign to a source. You can have a primary stream and a backup stream or two equally prioritized streams. This setting only applies when Failover Mode is set to FAILOVER.
    /// 
    /// Required: No
    ///
    /// Type: SourcePriority
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePriority")]
    pub source_priority: Option<SourcePriority>,


    /// 
    /// The type of failover you choose for this flow. MERGE combines the source streams        into a single stream, allowing graceful recovery from any single-source loss.        FAILOVER allows switching between different streams. The string for this property must be entered as MERGE or FAILOVER. No other string entry is valid.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailoverMode")]
    pub failover_mode: Option<String>,

}


/// The details of the sources of the flow.
///
/// If you are creating a flow with a VPC source, you must first create the flow with a       temporary standard source by doing the following:
#[derive(Default, serde::Serialize)]
pub struct Source {


    /// 
    /// A description of the source. This description is not visible outside of the        current AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The maximum latency in milliseconds for a RIST or Zixi-based source.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxLatency")]
    pub max_latency: Option<i64>,


    /// 
    /// The port that the flow uses to send outbound requests to initiate connection with        the sender.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SenderControlPort")]
    pub sender_control_port: Option<i64>,


    /// 
    /// The stream ID that you want to use for the transport. This parameter applies only to       Zixi-based streams.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamId")]
    pub stream_id: Option<String>,


    /// 
    /// The range of IP addresses that are allowed to contribute content to your source.        Format the IP addresses as a Classless Inter-Domain Routing (CIDR) block; for        example, 10.0.0.0/16.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WhitelistCidr")]
    pub whitelist_cidr: Option<String>,


    /// Source port for SRT-caller protocol.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceListenerPort")]
    pub source_listener_port: Option<i64>,


    /// 
    /// Source IP or domain name for SRT-caller protocol.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceListenerAddress")]
    pub source_listener_address: Option<String>,


    /// 
    /// The ARN of the entitlement that allows you to subscribe to content that comes from        another AWS account. The entitlement is set by the content originator and the ARN is        generated as part of the originator’s flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: Option<String>,


    /// 
    /// The minimum latency in milliseconds for SRT-based streams. In streams that use the SRT       protocol, this value that you set on your MediaConnect source or output represents the       minimal potential latency of that connection. The latency of the stream is set to the       highest number between the sender’s minimum latency and the receiver’s minimum       latency.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinLatency")]
    pub min_latency: Option<i64>,


    /// 
    /// The name of the VPC interface that the source content comes from.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcInterfaceName")]
    pub vpc_interface_name: Option<String>,


    /// 
    /// The IP address that the flow listens on for incoming content.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngestIp")]
    pub ingest_ip: Option<String>,


    /// 
    /// The name of the source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The type of encryption that is used on the content ingested from the        source.
    /// 
    /// Required: No
    ///
    /// Type: Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Decryption")]
    pub decryption: Option<Encryption>,


    /// 
    /// The IP address that the flow communicates with to initiate connection with the        sender.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SenderIpAddress")]
    pub sender_ip_address: Option<String>,


    /// 
    /// The maximum bitrate for RIST, RTP, and RTP-FEC streams.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxBitrate")]
    pub max_bitrate: Option<i64>,


    /// 
    /// The ARN of the source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceArn")]
    pub source_arn: Option<String>,


    /// 
    /// The port that the flow will be listening on for incoming content.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceIngestPort")]
    pub source_ingest_port: Option<String>,


    /// 
    /// The port that the flow listens on for incoming content. If the protocol of the        source is Zixi, the port must be set to 2088.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngestPort")]
    pub ingest_port: Option<i64>,


    /// 
    /// The protocol that is used by the source. AWS CloudFormation does not currently support CDI or ST 2110 JPEG XS source protocols.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

}


/// The priority you want to assign to a source. You can have a primary stream and a backup stream or two equally prioritized streams. This setting only applies when Failover Mode is set to FAILOVER.
#[derive(Default, serde::Serialize)]
pub struct SourcePriority {


    /// 
    /// The name of the source you choose as the primary source for this flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimarySource")]
    pub primary_source: String,

}
