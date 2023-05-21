

/// Registers your account as a publisher of public extensions in the CloudFormation registry. Public  extensions are available for use by all CloudFormation users.
///
/// For information on requirements for registering as a public extension publisher, see Registering your account   to publish CloudFormation extensions in the CloudFormation CLI User   Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnPublisher {


    /// 
    /// If you are using a Bitbucket or GitHub account for identity verification, the Amazon Resource Name (ARN) for  your connection to that account.
    /// 
    /// For more information, see Registering your account   to publish CloudFormation extensions in the CloudFormation CLI User   Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:aws(-[\w]+)*:.+:.+:[0-9]{12}:.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: Option<String>,


    /// 
    /// Whether you accept the Terms and Conditions for publishing extensions in the CloudFormation registry. You must accept the  terms and conditions in order to register to publish public extensions to the CloudFormation  registry.
    /// 
    /// The default is false.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceptTermsAndConditions")]
    pub accept_terms_and_conditions: bool,

}
