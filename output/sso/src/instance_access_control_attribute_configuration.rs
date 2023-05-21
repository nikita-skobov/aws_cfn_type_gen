

/// Enables the attribute-based access control (ABAC) feature for the specified IAM Identity Center     instance. You can also specify new attributes to add to your ABAC configuration during the     enabling process. For more information about ABAC, see Attribute-Based Access Control in     the IAM Identity Center User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnInstanceAccessControlAttributeConfiguration {


    /// 
    /// Lists the attributes that are configured for ABAC in the specified IAM Identity Center     instance.
    /// 
    /// Required: No
    ///
    /// Type: List of AccessControlAttribute
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessControlAttributes")]
    pub access_control_attributes: Option<Vec<AccessControlAttribute>>,


    /// 
    /// The ARN of the IAM Identity Center instance under which the operation will be executed.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 1224
    ///
    /// Pattern: arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b):sso:::instance/(sso)?ins-[a-zA-Z0-9-.]{16}
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceArn")]
    pub instance_arn: String,

}


/// The value used for mapping a specified attribute to an identity source.
#[derive(Default, serde::Serialize)]
pub struct AccessControlAttributeValue {


    /// 
    /// The identity source to use when mapping a specified attribute to IAM Identity Center.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: Vec<String>,

}


/// These are IAM Identity Center identity store attributes that you can configure for use in     attributes-based access control (ABAC). You can create permissions policies that determine     who can access your AWS resources based upon the configured attribute values. When you     enable ABAC and specify AccessControlAttributes, IAM Identity Center passes the attribute     values of the authenticated user into IAM for use in policy evaluation.
#[derive(Default, serde::Serialize)]
pub struct AccessControlAttribute {


    /// 
    /// The value used for mapping a specified attribute to an identity source.
    /// 
    /// Required: Yes
    ///
    /// Type: AccessControlAttributeValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: AccessControlAttributeValue,


    /// 
    /// The name of the attribute associated with your identities in your identity source. This     is used to map a specified attribute in your identity source with an attribute in IAM Identity Center.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\p{L}\p{Z}\p{N}_.:\/=+\-@]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}
