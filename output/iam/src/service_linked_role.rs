

/// Creates an IAM role that is linked to a specific AWS service. The service controls       the attached policies and when the role can be deleted. This helps ensure that the       service is not broken by an unexpectedly changed or deleted role, which could put your       AWS resources into an unknown state. Allowing the service to control the role helps       improve service stability and proper cleanup when a service and its role are no longer       needed. For more information, see Using service-linked         roles in the IAM User Guide.
///
/// To attach a policy to this service-linked role, you must make the request using the       AWS service that depends on this role.
#[derive(Default, serde::Serialize)]
pub struct CfnServiceLinkedRole {


    /// 
    /// The service principal for the AWS service to which this role is attached. You use a       string similar to a URL but without the http:// in front. For example:         elasticbeanstalk.amazonaws.com.
    /// 
    /// Service principals are unique and case-sensitive. To find the exact service principal       for your service-linked role, see AWS services         that work with IAM in the IAM User Guide. Look for       the services that have Yes in the Service-Linked Role column. Choose the Yes link to view the service-linked role documentation for that       service.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "AWSServiceName")]
    pub awsservice_name: String,


    /// 
    /// The description of the role.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Pattern: [\u0009\u000A\u000D\u0020-\u007E\u00A1-\u00FF]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// 
    /// 
    /// A string that you provide, which is combined with the service-provided prefix to form       the complete role name. If you make multiple requests for the same service, then you       must supply a different CustomSuffix for each request. Otherwise the       request fails with a duplicate role name error. For example, you could add         -1 or -debug to the suffix.
    /// 
    /// Some services do not support the CustomSuffix parameter. If you provide       an optional suffix and the operation fails, try the operation again without the       suffix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomSuffix")]
    pub custom_suffix: Option<String>,

}
