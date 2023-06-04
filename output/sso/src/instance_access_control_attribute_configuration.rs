/// Enables the attribute-based access control (ABAC) feature for the specified IAM Identity Center     instance. You can also specify new attributes to add to your ABAC configuration during the     enabling process. For more information about ABAC, see Attribute-Based Access Control in     the IAM Identity Center User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    pub instance_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnInstanceAccessControlAttributeConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::SSO::InstanceAccessControlAttributeConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_control_attributes {
            if the_val.len() > 50 as _ {
                return Err(format!("Max validation failed on field 'access_control_attributes'. {} is greater than 50", the_val.len()));
            }
        }

        let the_val = &self.instance_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1224 as _ {
                return Err(format!(
                    "Max validation failed on field 'instance_arn'. {} is greater than 1224",
                    s.len()
                ));
            }
        }

        let the_val = &self.instance_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 10 as _ {
                return Err(format!(
                    "Min validation failed on field 'instance_arn'. {} is less than 10",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// These are IAM Identity Center identity store attributes that you can configure for use in     attributes-based access control (ABAC). You can create permissions policies that determine     who can access your AWS resources based upon the configured attribute values. When you     enable ABAC and specify AccessControlAttributes, IAM Identity Center passes the attribute     values of the authenticated user into IAM for use in policy evaluation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessControlAttribute {
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
    pub key: cfn_resources::StrVal,

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
}

impl cfn_resources::CfnResource for AccessControlAttribute {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'key'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'key'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.value.validate()?;

        Ok(())
    }
}

/// The value used for mapping a specified attribute to an identity source.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for AccessControlAttributeValue {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.source;

        if the_val.len() > 1 as _ {
            return Err(format!(
                "Max validation failed on field 'source'. {} is greater than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
