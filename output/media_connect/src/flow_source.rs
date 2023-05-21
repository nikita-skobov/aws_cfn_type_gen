

/// The AWS::MediaConnect::FlowSource resource is used to add additional sources to an       existing flow. Adding an additional source requires Failover to be enabled. When you       enable Failover, the additional source must use the same protocol as the existing       source. A source is the external video content that includes configuration information       (encryption and source type) and a network address. Each flow has at least one source. A       standard source comes from a source other than another AWS Elemental       MediaConnect flow, such as an on-premises encoder.
#[derive(Default, serde::Serialize)]
pub struct CfnFlowSource {


    /// 
    /// The minimum latency in milliseconds for SRT-based streams. In streams that use the        SRT protocol, this value that you set on your MediaConnect source or output        represents the minimal potential latency of that connection. The latency of the        stream is set to the highest number between the sender’s minimum latency and the        receiver’s minimum latency.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinLatency")]
    pub min_latency: Option<i64>,


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
    /// A description of the source. This description is not visible outside of the        current AWS account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: String,


    /// 
    /// The maximum latency in milliseconds. This parameter applies only to RIST-based,        Zixi-based, and Fujitsu-based streams.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxLatency")]
    pub max_latency: Option<i64>,


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
    /// The range of IP addresses that are allowed to contribute content to your source.        Format the IP addresses as a Classless Inter-Domain Routing (CIDR) block; for        example, 10.0.0.0/16.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WhitelistCidr")]
    pub whitelist_cidr: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the flow this source is connected to. The flow must have Failover enabled to add an additional source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowArn")]
    pub flow_arn: Option<String>,


    /// 
    /// The protocol that the source uses to deliver the content to       MediaConnect. Adding additional sources to an existing flow requires Failover to be       enabled. When you enable Failover, the additional source must use the same protocol as       the existing source. Only the following protocols support failover: Zixi-push, RTP-FEC,       RTP, RIST and SRT protocols.
    /// 
    /// If you use failover with SRT caller or listener,       the FailoverMode property must be set to FAILOVER. The         FailoverMode property is found in the FailoverConfig       resource of the same flow ARN you used for the source's FlowArn property.       SRT caller/listener does not support merge mode failover.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,


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
    /// The ARN of the entitlement that allows you to subscribe to the flow. The        entitlement is set by the content originator, and the ARN is generated as part of the        originator's flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: Option<String>,


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
    /// The stream ID that you want to use for this transport. This parameter applies only to Zixi and SRT caller-based streams.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamId")]
    pub stream_id: Option<String>,


    /// 
    /// The name of the source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The name of the VPC interface that you want to send your output to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcInterfaceName")]
    pub vpc_interface_name: Option<String>,


    /// 
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
    /// Source IP or domain name for SRT-caller protocol.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceListenerAddress")]
    pub source_listener_address: Option<String>,

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
    /// The type of algorithm that is used for static key encryption (such as aes128, aes192, or       aes256). If you are using SPEKE or SRT-password encryption, this property must be left blank.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Algorithm")]
    pub algorithm: Option<String>,


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
    /// The type of key that is used for the encryption. If you don't specify a       keyType value, the service uses the default setting       (static-key). Valid key types are: static-key, speke, and srt-password.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,


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

}