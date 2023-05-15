
pub mod cfn_assignment {

#[derive(serde::Serialize, Default)]
pub struct CfnAssignment {
    /// The account id to be provisioned.
    #[serde(rename = "TargetId")]
    pub target_id: String,
    /// The sso instance that the permission set is owned.
    #[serde(rename = "InstanceArn")]
    pub instance_arn: String,
    /// The type of resource to be provsioned to, only aws account now
    #[serde(rename = "TargetType")]
    pub target_type: String,
    /// The permission set that the assignemt will be assigned
    #[serde(rename = "PermissionSetArn")]
    pub permission_set_arn: String,
    /// The assignee's type, user/group
    #[serde(rename = "PrincipalType")]
    pub principal_type: String,
    /// The assignee's identifier, user id/group id
    #[serde(rename = "PrincipalId")]
    pub principal_id: String,

}



}

pub mod cfn_instance_access_control_attribute_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnInstanceAccessControlAttributeConfiguration {
    /// The ARN of the AWS SSO instance under which the operation will be executed.
    #[serde(rename = "InstanceArn")]
    pub instance_arn: String,
    /// The InstanceAccessControlAttributeConfiguration property has been deprecated but is still supported for backwards compatibility purposes. We recomend that you use  AccessControlAttributes property instead.
    #[serde(rename = "InstanceAccessControlAttributeConfiguration")]
    pub instance_access_control_attribute_configuration: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessControlAttributes")]
    pub access_control_attributes: Option<AccessControlAttributeList>,

}


#[derive(serde::Serialize, Default)]
pub struct AccessControlAttributeList {

}

#[derive(serde::Serialize, Default)]
pub struct AccessControlAttributeValue {
    #[serde(rename = "Source")]
    pub source: AccessControlAttributeValueSourceList,

}

#[derive(serde::Serialize, Default)]
pub struct AccessControlAttribute {
    #[serde(rename = "Value")]
    pub value: AccessControlAttributeValue,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct AccessControlAttributeValueSourceList {

}
pub type AccessControlAttributeValueSource = String;

}

pub mod cfn_permission_set {

#[derive(serde::Serialize, Default)]
pub struct CfnPermissionSet {
    /// The sso instance arn that the permission set is owned.
    #[serde(rename = "InstanceArn")]
    pub instance_arn: String,
    /// List of ManagedPolicyArn
    #[serde(rename = "ManagedPolicies")]
    pub managed_policies: Option<Vec<ManagedPolicyArn>>,
    /// The inline policy to put in permission set.
    #[serde(rename = "InlinePolicy")]
    pub inline_policy: Option<()>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of CustomerManagedPolicyReference
    #[serde(rename = "CustomerManagedPolicyReferences")]
    pub customer_managed_policy_references: Option<Vec<CustomerManagedPolicyReference>>,
    /// No documentation provided by AWS
    #[serde(rename = "PermissionsBoundary")]
    pub permissions_boundary: Option<PermissionsBoundary>,
    /// The length of time that a user can be signed in to an AWS account.
    #[serde(rename = "SessionDuration")]
    pub session_duration: Option<String>,
    /// The name you want to assign to this permission set.
    #[serde(rename = "Name")]
    pub name: String,
    /// The permission set description.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The relay state URL that redirect links to any service in the AWS Management Console.
    #[serde(rename = "RelayStateType")]
    pub relay_state_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct PermissionsBoundary {
    #[serde(rename = "ManagedPolicyArn")]
    pub managed_policy_arn: Option<ManagedPolicyArn>,
    #[serde(rename = "CustomerManagedPolicyReference")]
    pub customer_managed_policy_reference: Option<CustomerManagedPolicyReference>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomerManagedPolicyReference {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}
pub type ManagedPolicyArn = String;

}
