/// Registers your account as a publisher of public extensions in the CloudFormation registry. Public  extensions are available for use by all CloudFormation users.
///
/// For information on requirements for registering as a public extension publisher, see Registering your account   to publish CloudFormation extensions in the CloudFormation CLI User   Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPublisher {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
}

impl cfn_resources::CfnResource for CfnPublisher {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::Publisher"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.connection_arn {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'connection_arn'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.connection_arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'connection_arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
