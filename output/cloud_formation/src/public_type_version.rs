

/// Tests and publishes a registered extension as a public, third-party extension.
///
/// CloudFormation first tests the extension to make sure it meets all necessary requirements for being  published in the CloudFormation registry. If it does, CloudFormation then publishes it to the  registry as a public third-party extension in this Region. Public extensions are available for use by all CloudFormation users.
///
/// For more information, see Testing your public   extension prior to publishing in the CloudFormation CLI User Guide.
///
/// If you don't specify a version, CloudFormation uses the default version of the extension in your  account and Region for testing.
///
/// To perform testing, CloudFormation assumes the execution role specified when the type was  registered.
///
/// An extension must have a test status of PASSED before it can be published. For more information,  see Publishing   extensions to make them available for public use in the CloudFormation CLI User   Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnPublicTypeVersion {


    /// 
    /// The name of the extension to test.
    /// 
    /// Conditional: You must specify Arn, or TypeName and Type.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 204
    ///
    /// Pattern: [A-Za-z0-9]{2,64}::[A-Za-z0-9]{2,64}::[A-Za-z0-9]{2,64}(::MODULE){0,1}
    ///
    /// Update requires: Replacement
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,


    /// 
    /// The type of the extension to test.
    /// 
    /// Conditional: You must specify Arn, or TypeName and Type.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: HOOK | MODULE | RESOURCE
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The Amazon Resource Number (ARN) of the extension.
    /// 
    /// Conditional: You must specify Arn, or TypeName and Type.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:aws[A-Za-z0-9-]{0,64}:cloudformation:[A-Za-z0-9-]{1,64}:([0-9]{12})?:type/.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    pub arn: Option<String>,


    /// 
    /// The S3 bucket to which CloudFormation delivers the contract test execution logs.
    /// 
    /// CloudFormation delivers the logs by the time contract testing has completed and the extension has been  assigned a test type status of PASSED or FAILED.
    /// 
    /// The user initiating the stack operation must be able to access items in the specified S3 bucket. Specifically,  the user needs the following permissions:
    /// 
    /// GetObject     PutObject
    /// 
    /// For more information, see Actions, Resources, and Condition Keys for Amazon S3 in the AWS Identity and Access Management User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [\s\S]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogDeliveryBucket")]
    pub log_delivery_bucket: Option<String>,


    /// 
    /// The version number to assign to this version of the extension.
    /// 
    /// Use the following format, and adhere to semantic versioning when assigning a version number to your  extension:
    /// 
    /// MAJOR.MINOR.PATCH
    /// 
    /// For more information, see Semantic Versioning 2.0.0.
    /// 
    /// If you don't specify a version number, CloudFormation increments the version number by one minor  version release.
    /// 
    /// You cannot specify a version number the first time you publish a type. AWS CloudFormation automatically sets the first  version number to be 1.0.0.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Pattern: ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "PublicVersionNumber")]
    pub public_version_number: Option<String>,

}
