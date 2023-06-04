/// Assigns access to a Principal for a specified AWS account using a specified permission     set.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAssignment {
    ///
    /// The ARN of the IAM Identity Center instance under which the operation will be executed. For more     information about ARNs, see Amazon Resource Names (ARNs) and       AWS Service Namespaces in the AWS General Reference.
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

    ///
    /// The ARN of the permission set.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 1224
    ///
    /// Pattern: arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b):sso:::permissionSet/(sso)?ins-[a-zA-Z0-9-.]{16}/ps-[a-zA-Z0-9-./]{16}
    ///
    /// Update requires: Replacement
    #[serde(rename = "PermissionSetArn")]
    pub permission_set_arn: cfn_resources::StrVal,

    ///
    /// An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the IAM Identity Center Identity Store API Reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 47
    ///
    /// Pattern: ^([0-9a-f]{10}-|)[A-Fa-f0-9]{8}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{12}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrincipalId")]
    pub principal_id: cfn_resources::StrVal,

    ///
    /// The entity type for which the assignment will be created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GROUP | USER
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrincipalType")]
    pub principal_type: AssignmentPrincipalTypeEnum,

    ///
    /// TargetID is an AWS account identifier, (For example, 123456789012).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: \d{12}
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetId")]
    pub target_id: cfn_resources::StrVal,

    ///
    /// The entity type for which the assignment will be created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AWS_ACCOUNT
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetType")]
    pub target_type: AssignmentTargetTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AssignmentPrincipalTypeEnum {
    /// GROUP
    #[serde(rename = "GROUP")]
    Group,

    /// USER
    #[serde(rename = "USER")]
    User,
}

impl Default for AssignmentPrincipalTypeEnum {
    fn default() -> Self {
        AssignmentPrincipalTypeEnum::Group
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AssignmentTargetTypeEnum {
    /// AWS_ACCOUNT
    #[serde(rename = "AWS_ACCOUNT")]
    Awsaccount,
}

impl Default for AssignmentTargetTypeEnum {
    fn default() -> Self {
        AssignmentTargetTypeEnum::Awsaccount
    }
}

impl cfn_resources::CfnResource for CfnAssignment {
    fn type_string(&self) -> &'static str {
        "AWS::SSO::Assignment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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

        let the_val = &self.permission_set_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1224 as _ {
                return Err(format!(
                    "Max validation failed on field 'permission_set_arn'. {} is greater than 1224",
                    s.len()
                ));
            }
        }

        let the_val = &self.permission_set_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 10 as _ {
                return Err(format!(
                    "Min validation failed on field 'permission_set_arn'. {} is less than 10",
                    s.len()
                ));
            }
        }

        let the_val = &self.principal_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 47 as _ {
                return Err(format!(
                    "Max validation failed on field 'principal_id'. {} is greater than 47",
                    s.len()
                ));
            }
        }

        let the_val = &self.principal_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'principal_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.target_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 12 as _ {
                return Err(format!(
                    "Max validation failed on field 'target_id'. {} is greater than 12",
                    s.len()
                ));
            }
        }

        let the_val = &self.target_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 12 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_id'. {} is less than 12",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
