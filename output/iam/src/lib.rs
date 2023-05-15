
pub mod cfn_access_key {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessKey {
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Serial")]
    pub serial: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "UserName")]
    pub user_name: String,

}



}

pub mod cfn_group {

#[derive(serde::Serialize, Default)]
pub struct CfnGroup {
    /// No documentation provided by AWS
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ManagedPolicyArns")]
    pub managed_policy_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Path")]
    pub path: Option<String>,
    /// List of Policy
    #[serde(rename = "Policies")]
    pub policies: Option<Vec<Policy>>,

}


#[derive(serde::Serialize, Default)]
pub struct Policy {
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),

}


}

pub mod cfn_instance_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnInstanceProfile {
    /// The name of the role to associate with the instance profile. Only one role can be assigned to an EC2 instance at a time, and all applications on the instance share the same role and permissions.
    #[serde(rename = "Roles")]
    pub roles: Vec<String>,
    /// The path to the instance profile.
    #[serde(rename = "Path")]
    pub path: Option<String>,
    /// The name of the instance profile to create.
    #[serde(rename = "InstanceProfileName")]
    pub instance_profile_name: Option<String>,

}



}

pub mod cfn_managed_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnManagedPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),
    /// No documentation provided by AWS
    #[serde(rename = "Roles")]
    pub roles: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ManagedPolicyName")]
    pub managed_policy_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Path")]
    pub path: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Users")]
    pub users: Option<Vec<String>>,

}



}

pub mod cfn_oidcprovider {

#[derive(serde::Serialize, Default)]
pub struct CfnOIDCProvider {
    /// No documentation provided by AWS
    #[serde(rename = "Url")]
    pub url: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClientIdList")]
    pub client_id_list: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ThumbprintList")]
    pub thumbprint_list: Vec<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "Roles")]
    pub roles: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Users")]
    pub users: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),
    /// No documentation provided by AWS
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyName")]
    pub policy_name: String,

}



}

pub mod cfn_role {

#[derive(serde::Serialize, Default)]
pub struct CfnRole {
    /// The trust policy that is associated with this role.
    #[serde(rename = "AssumeRolePolicyDocument")]
    pub assume_role_policy_document: (),
    /// A list of tags that are attached to the role.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ARN of the policy used to set the permissions boundary for the role.
    #[serde(rename = "PermissionsBoundary")]
    pub permissions_boundary: Option<String>,
    /// The maximum session duration (in seconds) that you want to set for the specified role. If you do not specify a value for this setting, the default maximum of one hour is applied. This setting can have a value from 1 hour to 12 hours.
    #[serde(rename = "MaxSessionDuration")]
    pub max_session_duration: Option<usize>,
    /// A description of the role that you provide.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// A list of Amazon Resource Names (ARNs) of the IAM managed policies that you want to attach to the role.
    #[serde(rename = "ManagedPolicyArns")]
    pub managed_policy_arns: Option<Vec<String>>,
    /// The path to the role.
    #[serde(rename = "Path")]
    pub path: Option<String>,
    /// Adds or updates an inline policy document that is embedded in the specified IAM role.
    #[serde(rename = "Policies")]
    pub policies: Option<Vec<Policy>>,
    /// A name for the IAM role, up to 64 characters in length.
    #[serde(rename = "RoleName")]
    pub role_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Policy {
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),
    #[serde(rename = "PolicyName")]
    pub policy_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_samlprovider {

#[derive(serde::Serialize, Default)]
pub struct CfnSAMLProvider {
    /// No documentation provided by AWS
    #[serde(rename = "SamlMetadataDocument")]
    pub saml_metadata_document: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_server_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnServerCertificate {
    /// No documentation provided by AWS
    #[serde(rename = "PrivateKey")]
    pub private_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateBody")]
    pub certificate_body: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServerCertificateName")]
    pub server_certificate_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Path")]
    pub path: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_service_linked_role {

#[derive(serde::Serialize, Default)]
pub struct CfnServiceLinkedRole {
    /// No documentation provided by AWS
    #[serde(rename = "CustomSuffix")]
    pub custom_suffix: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AWSServiceName")]
    pub awsservice_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



}

pub mod cfn_user {

#[derive(serde::Serialize, Default)]
pub struct CfnUser {
    /// List of Policy
    #[serde(rename = "Policies")]
    pub policies: Option<Vec<Policy>>,
    /// No documentation provided by AWS
    #[serde(rename = "LoginProfile")]
    pub login_profile: Option<LoginProfile>,
    /// No documentation provided by AWS
    #[serde(rename = "PermissionsBoundary")]
    pub permissions_boundary: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ManagedPolicyArns")]
    pub managed_policy_arns: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Path")]
    pub path: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct LoginProfile {
    #[serde(rename = "PasswordResetRequired")]
    pub password_reset_required: Option<bool>,
    #[serde(rename = "Password")]
    pub password: String,

}

#[derive(serde::Serialize, Default)]
pub struct Policy {
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_user_to_group_addition {

#[derive(serde::Serialize, Default)]
pub struct CfnUserToGroupAddition {
    /// No documentation provided by AWS
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Users")]
    pub users: Vec<String>,

}



}

pub mod cfn_virtual_mfadevice {

#[derive(serde::Serialize, Default)]
pub struct CfnVirtualMFADevice {
    /// No documentation provided by AWS
    #[serde(rename = "VirtualMfaDeviceName")]
    pub virtual_mfa_device_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Path")]
    pub path: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Users")]
    pub users: Vec<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
