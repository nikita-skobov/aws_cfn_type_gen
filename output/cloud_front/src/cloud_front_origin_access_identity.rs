/// The request to create a new origin access identity (OAI). An origin access identity is 			a special CloudFront user that you can associate with Amazon S3 origins, so that you can secure all 			or just some of your Amazon S3 content. For more information, see Restricting Access to Amazon S3 Content by Using an Origin Access Identity in 			the Amazon CloudFront Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFrontOriginAccessIdentity {
    ///
    /// The current configuration information for the identity.
    ///
    /// Required: Yes
    ///
    /// Type: CloudFrontOriginAccessIdentityConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudFrontOriginAccessIdentityConfig")]
    pub cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig,

    #[serde(skip_serializing)]
    pub att_id: CfnCloudFrontOriginAccessIdentityid,

    #[serde(skip_serializing)]
    pub att_s3_canonical_user_id: CfnCloudFrontOriginAccessIdentitys3canonicaluserid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFrontOriginAccessIdentityid;
impl CfnCloudFrontOriginAccessIdentityid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFrontOriginAccessIdentitys3canonicaluserid;
impl CfnCloudFrontOriginAccessIdentitys3canonicaluserid {
    pub fn att_name(&self) -> &'static str {
        r#"S3CanonicalUserId"#
    }
}

impl cfn_resources::CfnResource for CfnCloudFrontOriginAccessIdentity {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::CloudFrontOriginAccessIdentity"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_front_origin_access_identity_config.validate()?;

        Ok(())
    }
}

/// Origin access identity configuration. Send a GET request to the 					/CloudFront API version/CloudFront/identity ID/config 			resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CloudFrontOriginAccessIdentityConfig {
    ///
    /// A comment to describe the origin access identity. The comment cannot be longer than 			128 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CloudFrontOriginAccessIdentityConfig {
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
