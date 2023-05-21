

/// The AWS::MediaConnect::FlowOutput resource defines the destination address, protocol,       and port that AWS Elemental MediaConnect sends the ingested video to.       Each flow can have up to 50 outputs. An output can have the same protocol or a different       protocol from the source. The following protocols are supported: RIST, RTP, RTP-FEC, SRT-listener, SRT-caller, Zixi pull, Zixi push, and Fujitsu-QoS. CDI and ST 2110 JPEG XS protocols are not currently supported by AWS CloudFormation.
#[derive(Default, serde::Serialize)]
pub struct CfnFlowOutput {


    /// 
    /// The VPC interface that you want to send your output to.
    /// 
    /// Required: No
    ///
    /// Type: VpcInterfaceAttachment
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcInterfaceAttachment")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,


    /// 
    /// The protocol to use for the output.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: String,


    /// 
    /// The range of IP addresses that are allowed to initiate output requests to this        flow. Format the IP addresses as a Classless Inter-Domain Routing (CIDR) block; for        example, 10.0.0.0/16.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrAllowList")]
    pub cidr_allow_list: Option<Vec<String>>,


    /// 
    /// The identifier that is assigned to the Zixi receiver. This parameter applies only        to outputs that use Zixi pull.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoteId")]
    pub remote_id: Option<String>,


    /// 
    /// The maximum latency in milliseconds. This parameter applies only to RIST-based, Zixi-based, and Fujitsu-based streams.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxLatency")]
    pub max_latency: Option<i64>,


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
    /// The Amazon Resource Name (ARN) of the flow this output is attached to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,


    /// 
    /// The name of the VPC interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The IP address where you want to send the output.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: Option<String>,


    /// 
    /// The encryption credentials that you want to use for the output.
    /// 
    /// Required: No
    ///
    /// Type: Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    pub encryption: Option<Encryption>,


    /// 
    /// The port to use when MediaConnect distributes content to the output.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SmoothingLatency")]
    pub smoothing_latency: Option<i64>,


    /// 
    /// A description of the output. This description is not visible outside of the        current AWS account even if the account grants entitlements to other accounts.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


/// The VPC interface that you want to send your output to.
#[derive(Default, serde::Serialize)]
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
    pub vpc_interface_name: Option<String>,

}


/// Information about the encryption of the flow.
#[derive(Default, serde::Serialize)]
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
    pub algorithm: Option<String>,


    /// 
    /// The ARN of the secret that you created in AWS Secrets Manager to store the        encryption key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,


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
    /// The type of key that is used for the encryption. If you don't specify a       keyType value, the service uses the default setting       (static-key). Valid key types are: static-key, speke, and srt-password.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,

}
