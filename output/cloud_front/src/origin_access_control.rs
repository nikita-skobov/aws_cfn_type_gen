/// Creates a new origin access control in CloudFront. After you create an origin access 			control, you can add it to an origin in a CloudFront distribution so that CloudFront sends 			authenticated (signed) requests to the origin.
///
/// This makes it possible to block public access to the origin, allowing viewers (users) to 			access the origin's content only through CloudFront.
///
/// For more information about using a CloudFront origin access control, see Restricting access to an AWS origin in the 				Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnOriginAccessControl {
    ///
    /// The origin access control.
    ///
    /// Required: Yes
    ///
    /// Type: OriginAccessControlConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginAccessControlConfig")]
    pub origin_access_control_config: OriginAccessControlConfig,

    #[serde(skip_serializing)]
    pub att_id: CfnOriginAccessControlid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnOriginAccessControlid;
impl CfnOriginAccessControlid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnOriginAccessControl {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::OriginAccessControl"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.origin_access_control_config.validate()?;

        Ok(())
    }
}

/// Creates a new origin access control in CloudFront. After you create an origin access 			control, you can add it to an origin in a CloudFront distribution so that CloudFront sends 			authenticated (signed) requests to the origin.
///
/// This makes it possible to block public access to the origin, allowing viewers (users) to 			access the origin's content only through CloudFront.
///
/// For more information about using a CloudFront origin access control, see Restricting access to an AWS origin in the 				Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OriginAccessControlConfig {
    ///
    /// A description of the origin access control.
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
    /// A name to identify the origin access control.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The type of origin that this origin access control is for.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: mediastore | s3
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginAccessControlOriginType")]
    pub origin_access_control_origin_type:
        OriginAccessControlConfigOriginAccessControlOriginTypeEnum,

    ///
    /// Specifies which requests CloudFront signs (adds authentication information to). Specify 				always for the most common use case. For more information, see origin access control advanced settings in the 				Amazon CloudFront Developer Guide.
    ///
    /// This field can have one of the following values:
    ///
    /// always – CloudFront signs all origin requests, overwriting the 						Authorization header from the viewer request if one 					exists.                        never – CloudFront doesn't sign any origin requests. This value turns 					off origin access control for all origins in all distributions that use this 					origin access control.                        no-override – If the viewer request doesn't contain the 						Authorization header, then CloudFront signs the origin request. If 					the viewer request contains the Authorization header, then CloudFront 					doesn't sign the origin request and instead passes along the 						Authorization header from the viewer request. WARNING: To pass along the Authorization header 						from the viewer request, you must add the 							Authorization header to a cache policy for all cache behaviors that 						use origins associated with this origin access control.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: always | never | no-override
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningBehavior")]
    pub signing_behavior: OriginAccessControlConfigSigningBehaviorEnum,

    ///
    /// The signing protocol of the origin access control, which determines how CloudFront signs 			(authenticates) requests. The only valid value is sigv4.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: sigv4
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningProtocol")]
    pub signing_protocol: OriginAccessControlConfigSigningProtocolEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OriginAccessControlConfigOriginAccessControlOriginTypeEnum {
    /// mediastore
    #[serde(rename = "mediastore")]
    Mediastore,

    /// s3
    #[serde(rename = "s3")]
    S3,
}

impl Default for OriginAccessControlConfigOriginAccessControlOriginTypeEnum {
    fn default() -> Self {
        OriginAccessControlConfigOriginAccessControlOriginTypeEnum::Mediastore
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OriginAccessControlConfigSigningBehaviorEnum {
    /// always
    #[serde(rename = "always")]
    Always,

    /// never
    #[serde(rename = "never")]
    Never,

    /// no-override
    #[serde(rename = "no-override")]
    Nooverride,
}

impl Default for OriginAccessControlConfigSigningBehaviorEnum {
    fn default() -> Self {
        OriginAccessControlConfigSigningBehaviorEnum::Always
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OriginAccessControlConfigSigningProtocolEnum {
    /// sigv4
    #[serde(rename = "sigv4")]
    Sigv4,
}

impl Default for OriginAccessControlConfigSigningProtocolEnum {
    fn default() -> Self {
        OriginAccessControlConfigSigningProtocolEnum::Sigv4
    }
}

impl cfn_resources::CfnResource for OriginAccessControlConfig {
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
