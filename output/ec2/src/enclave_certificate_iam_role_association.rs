

/// Associates an AWS Identity and Access Management (IAM) role with an AWS Certificate Manager (ACM) certificate. 			This enables the certificate to be used by the ACM for Nitro Enclaves application inside an enclave. For more 			information, see AWS Certificate Manager for Nitro Enclaves in the         AWS Nitro Enclaves 					User Guide.
///
/// When the IAM role is associated with the ACM certificate, the certificate, certificate chain, and encrypted 			private key are placed in an Amazon S3 location that only the associated IAM role can access. The private key of the certificate 			is encrypted with an AWS managed key that has an attached attestation-based key policy.
///
/// To enable the IAM role to access the Amazon S3 object, you must grant it permission to call s3:GetObject 			on the Amazon S3 bucket returned by the command. To enable the IAM role to access the KMS key, 			you must grant it permission to call kms:Decrypt on the KMS key returned by the command. 			For more information, see 				Grant the role permission to access the certificate and encryption key in the 			        AWS Nitro Enclaves User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnEnclaveCertificateIamRoleAssociation {


    /// 
    /// The ARN of the IAM role to associate with the ACM certificate. You can associate up to 16 IAM roles with an ACM 			certificate.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The ARN of the ACM certificate with which to associate the IAM role.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,

}
