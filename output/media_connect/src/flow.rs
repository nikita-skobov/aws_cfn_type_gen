/// The AWS::MediaConnect::Flow resource defines a connection between one or more video       sources and one or more outputs. For each flow, you specify the transport protocol to       use, encryption information, and details for any outputs or entitlements that you want.         AWS Elemental MediaConnect returns an ingest endpoint where you can       send your live video as a single unicast stream. The service replicates and distributes       the video to every output that you specify, whether inside or outside the AWS Cloud. You can also set up entitlements on a flow to allow other         AWS accounts to access your content.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// The name of the flow.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

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

    ///
    /// The settings for source failover.
    ///
    /// Required: No
    ///
    /// Type: FailoverConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFailoverConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<FailoverConfig>,
}

impl cfn_resources::CfnResource for CfnFlow {
    fn type_string(&self) -> &'static str {
        "AWS::MediaConnect::Flow"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.source.validate()?;

        self.source_failover_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the encryption of the flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Encryption {
    ///
    /// The type of algorithm that is used for static key encryption (such as aes128, aes192, or        aes256). If you are using SPEKE or SRT-password encryption, this property must be left blank.
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
    /// The type of key that is used for the encryption. If you don't specify a          keyType value, the service uses the default setting            (static-key). Valid key types are: static-key, speke, and srt-password.
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

/// The settings for source failover.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FailoverConfig {
    ///
    /// The type of failover you choose for this flow. MERGE combines the source streams        into a single stream, allowing graceful recovery from any single-source loss.        FAILOVER allows switching between different streams. The string for this property must be entered as MERGE or FAILOVER. No other string entry is valid.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailoverMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<cfn_resources::StrVal>,

    ///
    /// The size of the buffer (delay) that the service maintains. A larger buffer means a        longer delay in transmitting the stream, but more room for error correction. A        smaller buffer means a shorter delay, but less room for error correction. You can choose a value from 100-500 ms. If you keep this field blank, the service uses the default value of 200 ms. This setting only applies when Failover Mode is set to MERGE.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecoveryWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_priority: Option<SourcePriority>,

    ///
    /// The state of source failover on the flow. If the state is inactive, the flow can        have only one source. If the state is active, the flow can have one or two        sources.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FailoverConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.source_priority
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The details of the sources of the flow.
///
/// If you are creating a flow with a VPC source, you must first create the flow with a       temporary standard source by doing the following:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Source {
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
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the entitlement that allows you to subscribe to content that comes from        another AWS account. The entitlement is set by the content originator and the ARN is        generated as part of the originator’s flow.
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
    /// The IP address that the flow listens on for incoming content.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngestIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_ip: Option<cfn_resources::StrVal>,

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
    /// The maximum latency in milliseconds for a RIST or Zixi-based source.
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
    /// The minimum latency in milliseconds for SRT-based streams. In streams that use the SRT       protocol, this value that you set on your MediaConnect source or output represents the       minimal potential latency of that connection. The latency of the stream is set to the       highest number between the sender’s minimum latency and the receiver’s minimum       latency.
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
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The protocol that is used by the source. AWS CloudFormation does not currently support CDI or ST 2110 JPEG XS source protocols.
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
    /// The ARN of the source.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<cfn_resources::StrVal>,

    ///
    /// The port that the flow will be listening on for incoming content.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceIngestPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ingest_port: Option<cfn_resources::StrVal>,

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
    /// The stream ID that you want to use for the transport. This parameter applies only to       Zixi-based streams.
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
    /// The name of the VPC interface that the source content comes from.
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
}

impl cfn_resources::CfnResource for Source {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
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

/// The priority you want to assign to a source. You can have a primary stream and a backup stream or two equally prioritized streams. This setting only applies when Failover Mode is set to FAILOVER.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub primary_source: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SourcePriority {
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
