
pub mod cfn_workspace {

#[derive(serde::Serialize, Default)]
pub struct CfnWorkspace {
    /// These enums represent valid permission types to use when creating or configuring a Grafana workspace. The SERVICE_MANAGED permission type means the Managed Grafana service will create a workspace IAM role on your behalf. The CUSTOMER_MANAGED permission type means that the customer is expected to provide an IAM role that the Grafana workspace can use to query data sources.
    #[serde(rename = "PermissionType")]
    pub permission_type: PermissionType,
    /// Description of a workspace.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of notification destinations on the customers service managed IAM role that the Grafana workspace can query.
    #[serde(rename = "NotificationDestinations")]
    pub notification_destinations: Option<Vec<NotificationDestinationType>>,
    /// List of Organizational Units containing AWS accounts the Grafana workspace can pull data from.
    #[serde(rename = "OrganizationalUnits")]
    pub organizational_units: Option<Vec<String>>,
    /// SAML configuration data associated with an AMG workspace.
    #[serde(rename = "SamlConfiguration")]
    pub saml_configuration: Option<SamlConfiguration>,
    /// A unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.
    #[serde(rename = "ClientToken")]
    pub client_token: Option<String>,
    /// IAM Role that will be used to grant the Grafana workspace access to a customers AWS resources.
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// List of authentication providers to enable.
    #[serde(rename = "AuthenticationProviders")]
    pub authentication_providers: Vec<AuthenticationProviderTypes>,
    /// The configuration settings for Network Access Control.
    #[serde(rename = "NetworkAccessControl")]
    pub network_access_control: Option<NetworkAccessControl>,
    /// The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to.
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,
    /// The name of the AWS CloudFormation stack set to use to generate IAM roles to be used for this workspace.
    #[serde(rename = "StackSetName")]
    pub stack_set_name: Option<String>,
    /// List of data sources on the service managed IAM role.
    #[serde(rename = "DataSources")]
    pub data_sources: Option<Vec<DataSourceType>>,
    /// The name of an IAM role that already exists to use with AWS Organizations to access AWS data sources and notification channels in other accounts in an organization.
    #[serde(rename = "OrganizationRoleName")]
    pub organization_role_name: Option<String>,
    /// The user friendly name of a workspace.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// These enums represent valid account access types. Specifically these enums determine whether the workspace can access AWS resources in the AWS account only, or whether it can also access resources in other accounts in the same organization. If the value CURRENT_ACCOUNT is used, a workspace role ARN must be provided. If the value is ORGANIZATION, a list of organizational units must be provided.
    #[serde(rename = "AccountAccessType")]
    pub account_access_type: AccountAccessType,

}

pub type DataSourceType = String;
#[derive(serde::Serialize, Default)]
pub struct RoleValues {
    #[serde(rename = "Editor")]
    pub editor: Option<Vec<String>>,
    #[serde(rename = "Admin")]
    pub admin: Option<Vec<String>>,

}
pub type PermissionType = String;
#[derive(serde::Serialize, Default)]
pub struct NetworkAccessControl {
    #[serde(rename = "VpceIds")]
    pub vpce_ids: Option<Vec<String>>,
    #[serde(rename = "PrefixListIds")]
    pub prefix_list_ids: Option<Vec<String>>,

}
pub type AuthenticationProviderTypes = String;pub type NotificationDestinationType = String;pub type WorkspaceStatus = String;
#[derive(serde::Serialize, Default)]
pub struct SamlConfiguration {
    #[serde(rename = "IdpMetadata")]
    pub idp_metadata: IdpMetadata,
    #[serde(rename = "RoleValues")]
    pub role_values: Option<RoleValues>,
    #[serde(rename = "AssertionAttributes")]
    pub assertion_attributes: Option<AssertionAttributes>,
    #[serde(rename = "LoginValidityDuration")]
    pub login_validity_duration: Option<f64>,
    #[serde(rename = "AllowedOrganizations")]
    pub allowed_organizations: Option<Vec<String>>,

}
pub type SamlConfigurationStatus = String;
#[derive(serde::Serialize, Default)]
pub struct VpcConfiguration {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct IdpMetadata {
    #[serde(rename = "Xml")]
    pub xml: Option<String>,
    #[serde(rename = "Url")]
    pub url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AssertionAttributes {
    #[serde(rename = "Login")]
    pub login: Option<String>,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "Role")]
    pub role: Option<String>,
    #[serde(rename = "Groups")]
    pub groups: Option<String>,
    #[serde(rename = "Org")]
    pub org: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}
pub type AccountAccessType = String;

}
