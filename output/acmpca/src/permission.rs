

/// Grants permissions to the AWS Certificate Manager (ACM) service       principal (acm.amazonaws.com) to perform IssueCertificate, GetCertificate, and ListPermissions       actions on a CA. These actions are needed for the ACM principal to renew private PKI       certificates requested through ACM and residing in the same AWS account       as the CA.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPermission {


    /// 
    /// The Amazon Resource Number (ARN) of the private CA from which the permission was 			issued.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Maximum: 200
    ///
    /// Pattern: arn:[\w+=/,.@-]+:[\w+=/,.@-]+:[\w+=/,.@-]*:[0-9]*:[\w+=,.@-]+(/[\w+=,.@-]+)*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,


    /// 
    /// The ID of the account that assigned the permission.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: [0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceAccount")]
    pub source_account: Option<String>,


    /// 
    /// The private CA actions that can be performed by the designated AWS       service. Supported actions are IssueCertificate,         GetCertificate, and ListPermissions.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: Replacement
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,


    /// 
    /// The AWS service or entity that holds the permission. At this time, the only valid 			principal is acm.amazonaws.com.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[^*]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: String,

}

impl cfn_resources::CfnResource for CfnPermission {
    fn type_string() -> &'static str {
        "AWS::ACMPCA::Permission"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
