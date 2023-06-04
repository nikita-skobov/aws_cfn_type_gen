/// The AWS::MediaConnect::FlowSource resource is used to add additional sources to an       existing flow. Adding an additional source requires Failover to be enabled. When you       enable Failover, the additional source must use the same protocol as the existing       source. A source is the external video content that includes configuration information       (encryption and source type) and a network address. Each flow has at least one source. A       standard source comes from a source other than another AWS Elemental       MediaConnect flow, such as an on-premises encoder.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnFlowSource {
    ///
    /// The type of encryption that is used on the content ingested from the        source.
    ///
    /// Required: No
    ///
    /// Type: Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Decryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<Encryption>,

    ///
    /// A description of the source. This description is not visible outside of the        current AWS account.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: cfn_resources::StrVal,

    ///
    /// The ARN of the entitlement that allows you to subscribe to the flow. The        entitlement is set by the content originator, and the ARN is generated as part of the        originator's flow.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the flow this source is connected to. The flow must have Failover enabled to add an additional source.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<cfn_resources::StrVal>,

    ///
    /// The port that the flow listens on for incoming content. If the protocol of the        source is Zixi, the port must be set to 2088.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngestPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i64>,

    ///
    /// The maximum bitrate for RIST, RTP, and RTP-FEC streams.
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
    /// The maximum latency in milliseconds. This parameter applies only to RIST-based,        Zixi-based, and Fujitsu-based streams.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,

    ///
    /// The minimum latency in milliseconds for SRT-based streams. In streams that use the        SRT protocol, this value that you set on your MediaConnect source or output        represents the minimal potential latency of that connection. The latency of the        stream is set to the highest number between the sender’s minimum latency and the        receiver’s minimum latency.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i64>,

    ///
    /// The name of the source.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<cfn_resources::StrVal>,

    ///
    /// The port that the flow uses to send outbound requests to initiate connection with        the sender.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SenderControlPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_control_port: Option<i64>,

    ///
    /// The IP address that the flow communicates with to initiate connection with the        sender.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SenderIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_ip_address: Option<cfn_resources::StrVal>,

    ///
    /// Source IP or domain name for SRT-caller protocol.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceListenerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_listener_address: Option<cfn_resources::StrVal>,

    ///
    /// Source port for SRT-caller protocol.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_listener_port: Option<i64>,

    ///
    /// The stream ID that you want to use for this transport. This parameter applies only to Zixi and SRT caller-based streams.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the VPC interface that you want to send your output to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<cfn_resources::StrVal>,

    ///
    /// The range of IP addresses that are allowed to contribute content to your source.        Format the IP addresses as a Classless Inter-Domain Routing (CIDR) block; for        example, 10.0.0.0/16.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WhitelistCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_ingest_ip: CfnFlowSourceingestip,

    #[serde(skip_serializing)]
    pub att_source_arn: CfnFlowSourcesourcearn,

    #[serde(skip_serializing)]
    pub att_source_ingest_port: CfnFlowSourcesourceingestport,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFlowSourceingestip;
impl CfnFlowSourceingestip {
    pub fn att_name(&self) -> &'static str {
        r#"IngestIp"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFlowSourcesourcearn;
impl CfnFlowSourcesourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"SourceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFlowSourcesourceingestport;
impl CfnFlowSourcesourceingestport {
    pub fn att_name(&self) -> &'static str {
        r#"SourceIngestPort"#
    }
}

impl cfn_resources::CfnResource for CfnFlowSource {
    fn type_string(&self) -> &'static str {
        "AWS::MediaConnect::FlowSource"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.decryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the encryption of the flow.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Encryption {
    ///
    /// The type of algorithm that is used for static key encryption (such as aes128, aes192, or       aes256). If you are using SPEKE or SRT-password encryption, this property must be left blank.
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
    /// A 128-bit, 16-byte hex value represented by a 32-character string, to be used with        the key for encrypting content. This parameter is not valid for static key        encryption.
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
    /// The value of one of the devices that you configured with your digital rights        management (DRM) platform key provider. This parameter is required for SPEKE        encryption and is not valid for static key encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<cfn_resources::StrVal>,

    ///
    /// The type of key that is used for the encryption. If you don't specify a       keyType value, the service uses the default setting       (static-key). Valid key types are: static-key, speke, and srt-password.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<cfn_resources::StrVal>,

    ///
    /// The AWS Region that the API Gateway proxy endpoint was created in. This parameter        is required for SPEKE encryption and is not valid for static key encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<cfn_resources::StrVal>,

    ///
    /// An identifier for the content. The service sends this value to the key server to        identify the current endpoint. The resource ID is also known as the content ID. This        parameter is required for SPEKE encryption and is not valid for static key        encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the role that you created during setup (when you        set up MediaConnect as a trusted entity).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The ARN of the secret that you created in AWS Secrets Manager to store the        encryption key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<cfn_resources::StrVal>,

    ///
    /// The URL from the API Gateway proxy that you set up to talk to your key server.        This parameter is required for SPEKE encryption and is not valid for static key        encryption.
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

impl cfn_resources::CfnResource for Encryption {
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
