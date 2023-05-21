

/// The AWS::MediaConnect::FlowEntitlement resource defines the permission that an AWS account grants to another AWS account to allow access       to the content in a specific AWS Elemental MediaConnect flow. The       content originator grants an entitlement to a specific AWS account (the       subscriber). When an entitlement is granted, the subscriber can create a flow using the       originator's flow as the source. Each flow can have up to 50 entitlements.
#[derive(Default, serde::Serialize)]
pub struct CfnFlowEntitlement {


    /// 
    /// A description of the entitlement. This description appears only on the        MediaConnect console and is not visible outside of the current AWS account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: String,


    /// 
    /// The type of encryption that MediaConnect will use on the output that is associated        with the entitlement.
    /// 
    /// Required: No
    ///
    /// Type: Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    pub encryption: Option<Encryption>,


    /// 
    /// The AWS account IDs that you want to share your content with. The receiving        accounts (subscribers) will be allowed to create their own flows using your content        as the source.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<String>,


    /// 
    /// An indication of whether the new entitlement should be enabled or disabled as soon        as it is created. If you don’t specify the entitlementStatus field in your request,        MediaConnect sets it to ENABLED.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntitlementStatus")]
    pub entitlement_status: Option<String>,


    /// 
    /// The name of the entitlement. This value must be unique within the current        flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,


    /// 
    /// The percentage of the entitlement data transfer fee that you want the subscriber        to be responsible for.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,

}


/// Information about the encryption of the flow.
#[derive(Default, serde::Serialize)]
pub struct Encryption {


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
    /// The type of algorithm that is used for static key encryption (such as aes128, aes192, or       aes256). If you are using SPEKE or SRT-password encryption, this property must be left blank.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Algorithm")]
    pub algorithm: String,


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
    /// The AWS Region that the API Gateway proxy endpoint was created in. This parameter        is required for SPEKE encryption and is not valid for static key encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: Option<String>,

}