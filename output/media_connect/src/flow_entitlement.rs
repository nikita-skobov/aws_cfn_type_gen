/// The AWS::MediaConnect::FlowEntitlement resource defines the permission that an AWS account grants to another AWS account to allow access       to the content in a specific AWS Elemental MediaConnect flow. The       content originator grants an entitlement to a specific AWS account (the       subscriber). When an entitlement is granted, the subscriber can create a flow using the       originator's flow as the source. Each flow can have up to 50 entitlements.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnFlowEntitlement {
    ///
    /// The percentage of the entitlement data transfer fee that you want the subscriber        to be responsible for.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,

    ///
    /// A description of the entitlement. This description appears only on the        MediaConnect console and is not visible outside of the current AWS account.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: cfn_resources::StrVal,

    ///
    /// The type of encryption that MediaConnect will use on the output that is associated        with the entitlement.
    ///
    /// Required: No
    ///
    /// Type: Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub encryption: Option<Encryption>,

    ///
    /// An indication of whether the new entitlement should be enabled or disabled as soon        as it is created. If you don’t specify the entitlementStatus field in your request,        MediaConnect sets it to ENABLED.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntitlementStatus")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub entitlement_status: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the flow.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowArn")]
    pub flow_arn: cfn_resources::StrVal,

    ///
    /// The name of the entitlement. This value must be unique within the current        flow.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

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

    #[serde(skip_serializing)]
    pub att_entitlement_arn: CfnFlowEntitlemententitlementarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFlowEntitlemententitlementarn;
impl CfnFlowEntitlemententitlementarn {
    pub fn att_name(&self) -> &'static str {
        r#"EntitlementArn"#
    }
}

impl cfn_resources::CfnResource for CfnFlowEntitlement {
    fn type_string(&self) -> &'static str {
        "AWS::MediaConnect::FlowEntitlement"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption
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
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Algorithm")]
    pub algorithm: cfn_resources::StrVal,

    ///
    /// A 128-bit, 16-byte hex value represented by a 32-character string, to be used with        the key for encrypting content. This parameter is not valid for static key        encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
