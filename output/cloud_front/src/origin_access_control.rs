

/// Creates a new origin access control in CloudFront. After you create an origin access 			control, you can add it to an origin in a CloudFront distribution so that CloudFront sends 			authenticated (signed) requests to the origin.
///
/// This makes it possible to block public access to the origin, allowing viewers (users) to 			access the origin's content only through CloudFront.
///
/// For more information about using a CloudFront origin access control, see Restricting access to an AWS origin in the 				Amazon CloudFront Developer Guide.
#[derive(Default, serde::Serialize)]
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

}


/// Creates a new origin access control in CloudFront. After you create an origin access 			control, you can add it to an origin in a CloudFront distribution so that CloudFront sends 			authenticated (signed) requests to the origin.
///
/// This makes it possible to block public access to the origin, allowing viewers (users) to 			access the origin's content only through CloudFront.
///
/// For more information about using a CloudFront origin access control, see Restricting access to an AWS origin in the 				Amazon CloudFront Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct OriginAccessControlConfig {


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
    pub signing_protocol: String,


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
    pub signing_behavior: String,


    /// 
    /// A name to identify the origin access control.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


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
    pub origin_access_control_origin_type: String,


    /// 
    /// A description of the origin access control.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}
