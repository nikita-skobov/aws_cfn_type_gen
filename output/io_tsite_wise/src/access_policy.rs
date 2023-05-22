/// Creates an access policy that grants the specified identity (IAM Identity Center user, IAM Identity Center group, or    IAM user) access to the specified AWS IoT SiteWise Monitor portal or project resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPolicy {
    ///
    /// The identity for this access policy. Choose an IAM Identity Center user, an IAM Identity Center group, or an IAM user.
    ///
    /// Required: Yes
    ///
    /// Type: AccessPolicyIdentity
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicyIdentity")]
    pub access_policy_identity: AccessPolicyIdentity,

    ///
    /// The permission level for this access policy. Choose either a ADMINISTRATOR or VIEWER.       Note that a project ADMINISTRATOR is also known as a project owner.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicyPermission")]
    pub access_policy_permission: cfn_resources::StrVal,

    ///
    /// The AWS IoT SiteWise Monitor resource for this access policy. Choose either a portal or a project.
    ///
    /// Required: Yes
    ///
    /// Type: AccessPolicyResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicyResource")]
    pub access_policy_resource: AccessPolicyResource,

    #[serde(skip_serializing)]
    pub att_access_policy_arn: CfnAccessPolicyaccesspolicyarn,

    #[serde(skip_serializing)]
    pub att_access_policy_id: CfnAccessPolicyaccesspolicyid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPolicyaccesspolicyarn;
impl CfnAccessPolicyaccesspolicyarn {
    pub fn att_name(&self) -> &'static str {
        r#"AccessPolicyArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPolicyaccesspolicyid;
impl CfnAccessPolicyaccesspolicyid {
    pub fn att_name(&self) -> &'static str {
        r#"AccessPolicyId"#
    }
}

impl cfn_resources::CfnResource for CfnAccessPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::IoTSiteWise::AccessPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.access_policy_identity.validate()?;

        self.access_policy_resource.validate()?;

        Ok(())
    }
}

/// The identity (IAM Identity Center user, IAM Identity Center group, or IAM user) to which this access policy    applies.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessPolicyIdentity {
    ///
    /// An IAM role identity.
    ///
    /// Required: No
    ///
    /// Type: IamRole
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<IamRole>,

    ///
    /// An IAM user identity.
    ///
    /// Required: No
    ///
    /// Type: IamUser
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user: Option<IamUser>,

    ///
    /// The IAM Identity Center user to which this access policy maps.
    ///
    /// Required: No
    ///
    /// Type: User
    ///
    /// Update requires: No interruption
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl cfn_resources::CfnResource for AccessPolicyIdentity {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.iam_role
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iam_user
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.user.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AWS IoT SiteWise Monitor resource for this access policy. Choose either a portal or a project.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessPolicyResource {
    ///
    /// The AWS IoT SiteWise Monitor portal for this access policy.
    ///
    /// Required: No
    ///
    /// Type: Portal
    ///
    /// Update requires: No interruption
    #[serde(rename = "Portal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal: Option<Portal>,

    ///
    /// The AWS IoT SiteWise Monitor project for this access policy.
    ///
    /// Required: No
    ///
    /// Type: Project
    ///
    /// Update requires: No interruption
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

impl cfn_resources::CfnResource for AccessPolicyResource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.portal.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.project.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about an AWS Identity and Access Management role. For more information, see IAM roles in the     IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IamRole {
    ///
    /// The ARN of the IAM role. For more information, see IAM ARNs in the     IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for IamRole {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Contains information about an AWS Identity and Access Management user.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IamUser {
    ///
    /// The ARN of the IAM user. For more information, see IAM ARNs in the     IAM User Guide.
    ///
    /// NoteIf you delete the IAM user, access policies that contain this identity include an     empty arn. You can delete the access policy for the IAM user that no longer     exists.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for IamUser {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The Portal property type specifies the AWS IoT SiteWise Monitor portal for an       AWS::IoTSiteWise::AccessPolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Portal {
    ///
    /// The ID of the portal.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Portal {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The Project property type specifies the AWS IoT SiteWise Monitor project for an       AWS::IoTSiteWise::AccessPolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Project {
    ///
    /// The ID of the project.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Project {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The User property type specifies the AWS IoT SiteWise Monitor user for       an AWS::IoTSiteWise::AccessPolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct User {
    ///
    /// The ID of the user.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for User {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
