/// The AWS::MediaConnect::FlowOutput resource defines the destination address, protocol,       and port that AWS Elemental MediaConnect sends the ingested video to.       Each flow can have up to 50 outputs. An output can have the same protocol or a different       protocol from the source. The following protocols are supported: RIST, RTP, RTP-FEC, SRT-listener, SRT-caller, Zixi pull, Zixi push, and Fujitsu-QoS. CDI and ST 2110 JPEG XS protocols are not currently supported by AWS CloudFormation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFlowOutput {
    ///
    /// The range of IP addresses that are allowed to initiate output requests to this        flow. Format the IP addresses as a Classless Inter-Domain Routing (CIDR) block; for        example, 10.0.0.0/16.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrAllowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,

    ///
    /// A description of the output. This description is not visible outside of the        current AWS account even if the account grants entitlements to other accounts.
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
    /// The IP address where you want to send the output.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<cfn_resources::StrVal>,

    ///
    /// The encryption credentials that you want to use for the output.
    ///
    /// Required: No
    ///
    /// Type: Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,

    ///
    /// The Amazon Resource Name (ARN) of the flow this output is attached to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowArn")]
    pub flow_arn: cfn_resources::StrVal,

    ///
    /// The maximum latency in milliseconds. This parameter applies only to RIST-based, Zixi-based, and Fujitsu-based streams.
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
    /// The name of the VPC interface.
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
    /// The port to use when MediaConnect distributes content to the output.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// The protocol to use for the output.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: cfn_resources::StrVal,

    ///
    /// The identifier that is assigned to the Zixi receiver. This parameter applies only        to outputs that use Zixi pull.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<cfn_resources::StrVal>,

    ///
    /// The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmoothingLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i64>,

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
    /// The VPC interface that you want to send your output to.
    ///
    /// Required: No
    ///
    /// Type: VpcInterfaceAttachment
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcInterfaceAttachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,

    #[serde(skip_serializing)]
    pub att_output_arn: CfnFlowOutputoutputarn,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFlowOutputoutputarn;
impl CfnFlowOutputoutputarn {
    pub fn att_name(&self) -> &'static str {
        r#"OutputArn"#
    }
}

impl cfn_resources::CfnResource for CfnFlowOutput {
    fn type_string(&self) -> &'static str {
        "AWS::MediaConnect::FlowOutput"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc_interface_attachment
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the encryption of the flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: cfn_resources::StrVal,
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

/// The VPC interface that you want to send your output to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcInterfaceAttachment {
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
}

impl cfn_resources::CfnResource for VpcInterfaceAttachment {
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
