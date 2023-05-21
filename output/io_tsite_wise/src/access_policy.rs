

/// Creates an access policy that grants the specified identity (IAM Identity Center user, IAM Identity Center group, or    IAM user) access to the specified AWS IoT SiteWise Monitor portal or project resource.
#[derive(Default, serde::Serialize)]
pub struct CfnAccessPolicy {


    /// 
    /// The permission level for this access policy. Choose either a ADMINISTRATOR or VIEWER.       Note that a project ADMINISTRATOR is also known as a project owner.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicyPermission")]
    pub access_policy_permission: String,


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
    /// The AWS IoT SiteWise Monitor resource for this access policy. Choose either a portal or a project.
    /// 
    /// Required: Yes
    ///
    /// Type: AccessPolicyResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicyResource")]
    pub access_policy_resource: AccessPolicyResource,

}


/// The Project property type specifies the AWS IoT SiteWise Monitor project for an       AWS::IoTSiteWise::AccessPolicy.
#[derive(Default, serde::Serialize)]
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
    pub id: Option<String>,

}


/// Contains information about an AWS Identity and Access Management user.
#[derive(Default, serde::Serialize)]
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
    pub arn: Option<String>,

}


/// The AWS IoT SiteWise Monitor resource for this access policy. Choose either a portal or a project.
#[derive(Default, serde::Serialize)]
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
    pub project: Option<Project>,

}


/// The identity (IAM Identity Center user, IAM Identity Center group, or IAM user) to which this access policy    applies.
#[derive(Default, serde::Serialize)]
pub struct AccessPolicyIdentity {


    /// 
    /// An IAM user identity.
    /// 
    /// Required: No
    ///
    /// Type: IamUser
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamUser")]
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
    pub user: Option<User>,


    /// 
    /// An IAM role identity.
    /// 
    /// Required: No
    ///
    /// Type: IamRole
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRole")]
    pub iam_role: Option<IamRole>,

}


/// The Portal property type specifies the AWS IoT SiteWise Monitor portal for an       AWS::IoTSiteWise::AccessPolicy.
#[derive(Default, serde::Serialize)]
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
    pub id: Option<String>,

}


/// The User property type specifies the AWS IoT SiteWise Monitor user for       an AWS::IoTSiteWise::AccessPolicy.
#[derive(Default, serde::Serialize)]
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
    pub id: Option<String>,

}


/// Contains information about an AWS Identity and Access Management role. For more information, see IAM roles in the     IAM User Guide.
#[derive(Default, serde::Serialize)]
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
    pub arn: Option<String>,

}