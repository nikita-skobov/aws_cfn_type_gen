/// Registers your account as a publisher of public extensions in the CloudFormation registry. Public  extensions are available for use by all CloudFormation users.
///
/// For information on requirements for registering as a public extension publisher, see Registering your account   to publish CloudFormation extensions in the CloudFormation CLI User   Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub connection_arn: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_identity_provider: CfnPublisheridentityprovider,

    #[serde(skip_serializing)]
    pub att_publisher_id: CfnPublisherpublisherid,

    #[serde(skip_serializing)]
    pub att_publisher_profile: CfnPublisherpublisherprofile,

    #[serde(skip_serializing)]
    pub att_publisher_status: CfnPublisherpublisherstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublisheridentityprovider;
impl CfnPublisheridentityprovider {
    pub fn att_name(&self) -> &'static str {
        r#"IdentityProvider"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublisherpublisherid;
impl CfnPublisherpublisherid {
    pub fn att_name(&self) -> &'static str {
        r#"PublisherId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublisherpublisherprofile;
impl CfnPublisherpublisherprofile {
    pub fn att_name(&self) -> &'static str {
        r#"PublisherProfile"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPublisherpublisherstatus;
impl CfnPublisherpublisherstatus {
    pub fn att_name(&self) -> &'static str {
        r#"PublisherStatus"#
    }
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
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'connection_arn'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.connection_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'connection_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
