/// The AWS::EMR::StudioSessionMapping resource is an Amazon EMR resource type that maps a user or group to the Amazon EMR Studio specified by StudioId, and     applies a session policy that defines Studio permissions for that user or group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnStudioSessionMapping {
    ///
    /// The name of the user or group. For more information, see UserName and DisplayName in the IAM Identity Center Identity Store API Reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityName")]
    pub identity_name: cfn_resources::StrVal,

    ///
    /// Specifies whether the identity to map to the Amazon EMR Studio is a user or a     group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GROUP | USER
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityType")]
    pub identity_type: StudioSessionMappingIdentityTypeEnum,

    ///
    /// The Amazon Resource Name (ARN) for the session policy that will be applied to the user     or group. Session policies refine Studio user permissions without the need to use multiple     IAM user roles. For more information, see Create an EMR Studio user role with session policies in the Amazon EMR Management Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionPolicyArn")]
    pub session_policy_arn: cfn_resources::StrVal,

    ///
    /// The ID of the Amazon EMR Studio to which the user or group will be     mapped.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "StudioId")]
    pub studio_id: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StudioSessionMappingIdentityTypeEnum {
    /// GROUP
    #[serde(rename = "GROUP")]
    Group,

    /// USER
    #[serde(rename = "USER")]
    User,
}

impl Default for StudioSessionMappingIdentityTypeEnum {
    fn default() -> Self {
        StudioSessionMappingIdentityTypeEnum::Group
    }
}

impl cfn_resources::CfnResource for CfnStudioSessionMapping {
    fn type_string(&self) -> &'static str {
        "AWS::EMR::StudioSessionMapping"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.identity_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'identity_name'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.identity_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'identity_name'. {} is less than 0",
                    s.len()
                ));
            }
        }

        let the_val = &self.session_policy_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'session_policy_arn'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.session_policy_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'session_policy_arn'. {} is less than 0",
                    s.len()
                ));
            }
        }

        let the_val = &self.studio_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'studio_id'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.studio_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'studio_id'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
