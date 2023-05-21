

/// The request to create a new origin access identity (OAI). An origin access identity is 			a special CloudFront user that you can associate with Amazon S3 origins, so that you can secure all 			or just some of your Amazon S3 content. For more information, see Restricting Access to Amazon S3 Content by Using an Origin Access Identity in 			the Amazon CloudFront Developer Guide.
#[derive(Default, serde::Serialize)]
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

}


/// Origin access identity configuration. Send a GET request to the 					/CloudFront API version/CloudFront/identity ID/config 			resource.
#[derive(Default, serde::Serialize)]
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
    pub comment: String,

}
