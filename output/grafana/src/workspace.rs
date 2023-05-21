

/// Specifies a workspace. In a workspace, you can create Grafana       dashboards and visualizations to analyze your metrics, logs, and traces. You don't have to       build, package, or deploy any hardware to run the Grafana server.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWorkspace {


    /// 
    /// Specifies whether the workspace can access AWS resources in this AWS account only, or whether it can also access AWS resources in       other accounts in the same organization. If this is ORGANIZATION, the       OrganizationalUnits parameter specifies which organizational units       the workspace can access.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CURRENT_ACCOUNT | ORGANIZATION
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountAccessType")]
    pub account_access_type: WorkspaceAccountAccessTypeEnum,


    /// 
    /// Specifies whether this workspace uses SAML 2.0, AWS IAM Identity Center (successor to AWS Single Sign-On), or both to       authenticate users for using the Grafana console within a workspace. For more       information, see User authentication in           Amazon Managed Grafana.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationProviders")]
    pub authentication_providers: Vec<String>,


    /// 
    /// A unique, case-sensitive, user-provided identifier to ensure the idempotency of the       request.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[!-~]{1,64}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientToken")]
    pub client_token: Option<String>,


    /// 
    /// Specifies the AWS data sources that have been configured to have         IAM roles and permissions created to allow Amazon Managed Grafana to read       data from these sources.
    /// 
    /// This list is only used when the workspace was created through the AWS       console, and the permissionType is SERVICE_MANAGED.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSources")]
    pub data_sources: Option<Vec<String>>,


    /// 
    /// The user-defined description of the workspace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the workspace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9-._~]{1,255}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The configuration settings for network access to your workspace.
    /// 
    /// Required: No
    ///
    /// Type: NetworkAccessControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkAccessControl")]
    pub network_access_control: Option<NetworkAccessControl>,


    /// 
    /// The AWS notification channels that Amazon Managed Grafana can automatically       create IAM roles and permissions for, to allow Amazon Managed Grafana to use       these channels.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationDestinations")]
    pub notification_destinations: Option<Vec<String>>,


    /// 
    /// The name of the IAM role that is used to access resources through         Organizations.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationRoleName")]
    pub organization_role_name: Option<String>,


    /// 
    /// Specifies the organizational units that this workspace is allowed to use data sources       from, if this workspace is in an account that is part of an organization.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationalUnits")]
    pub organizational_units: Option<Vec<String>>,


    /// 
    /// If this is SERVICE_MANAGED, and the workplace was created through the       Amazon Managed Grafana console, then Amazon Managed Grafana automatically creates the       IAM roles and provisions the permissions that the workspace needs to       use AWS data sources and notification channels.
    /// 
    /// If this is CUSTOMER_MANAGED, you must manage those roles and permissions       yourself.
    /// 
    /// If you are working with a workspace in a member account of an organization and       that account is not a delegated administrator account, and you want the workspace to       access data sources in other AWS accounts in the organization, this       parameter must be set to CUSTOMER_MANAGED.
    /// 
    /// For more information about converting between customer and service managed, see       Managing permissions for data sources and notification channels. For more       information about the roles and permissions that must be managed for customer managed       workspaces, see Amazon Managed Grafana       permissions and policies for AWS data sources and notification       channels
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOMER_MANAGED | SERVICE_MANAGED
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionType")]
    pub permission_type: WorkspacePermissionTypeEnum,


    /// 
    /// The IAM role that grants permissions to the AWS       resources that the workspace will view data from. This role must already exist.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// If the workspace uses SAML, use this structure to map SAML assertion attributes to       workspace user information and define which groups in the assertion attribute are to       have the Admin and Editor roles in the workspace.
    /// 
    /// Required: No
    ///
    /// Type: SamlConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamlConfiguration")]
    pub saml_configuration: Option<SamlConfiguration>,


    /// 
    /// The name of the AWS CloudFormation stack set that is used to generate IAM roles to be used for this workspace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackSetName")]
    pub stack_set_name: Option<String>,


    /// 
    /// The configuration for connecting to data sources in a private VPC (Amazon Virtual Private Cloud).
    /// 
    /// Required: No
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum WorkspaceAccountAccessTypeEnum {

    /// CURRENT_ACCOUNT
    #[serde(rename = "CURRENT_ACCOUNT")]
    Currentaccount,

    /// ORGANIZATION
    #[serde(rename = "ORGANIZATION")]
    Organization,

}

impl Default for WorkspaceAccountAccessTypeEnum {
    fn default() -> Self {
        WorkspaceAccountAccessTypeEnum::Currentaccount
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum WorkspacePermissionTypeEnum {

    /// CUSTOMER_MANAGED
    #[serde(rename = "CUSTOMER_MANAGED")]
    Customermanaged,

    /// SERVICE_MANAGED
    #[serde(rename = "SERVICE_MANAGED")]
    Servicemanaged,

}

impl Default for WorkspacePermissionTypeEnum {
    fn default() -> Self {
        WorkspacePermissionTypeEnum::Customermanaged
    }
}


impl cfn_resources::CfnResource for CfnWorkspace {
    fn type_string() -> &'static str {
        "AWS::Grafana::Workspace"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A structure that defines which attributes in the IdP assertion are to be used to       define information about the users authenticated by the IdP to use the workspace.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssertionAttributes {


    /// 
    /// The name of the attribute within the SAML assertion to use as the email names for SAML       users.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Email")]
    pub email: Option<String>,


    /// 
    /// The name of the attribute within the SAML assertion to use as the user full "friendly"       names for user groups.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Groups")]
    pub groups: Option<String>,


    /// 
    /// The name of the attribute within the SAML assertion to use as the login names for SAML       users.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Login")]
    pub login: Option<String>,


    /// 
    /// The name of the attribute within the SAML assertion to use as the user full "friendly"       names for SAML users.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The name of the attribute within the SAML assertion to use as the user full "friendly"       names for the users' organizations.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Org")]
    pub org: Option<String>,


    /// 
    /// The name of the attribute within the SAML assertion to use as the user roles.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: Option<String>,

}




/// A structure containing the identity provider (IdP) metadata used to integrate the       identity provider with this workspace. You can specify the metadata either by providing       a URL to its location in the url parameter, or by specifying the full       metadata in XML format in the xml parameter. Specifying both will cause an       error.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IdpMetadata {


    /// 
    /// The URL of the location containing the IdP metadata.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: Option<String>,


    /// 
    /// The full IdP metadata, in XML format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Xml")]
    pub xml: Option<String>,

}




/// The configuration settings for in-bound network access to your workspace.
///
/// When this is configured, only listed IP addresses and VPC endpoints will be able to       access your workspace. Standard Grafana authentication and authorization are still       required.
///
/// Access is granted to a caller that is in either the IP address list or the VPC       endpoint list - they do not need to be in both.
///
/// If this is not configured, or is removed, then all IP addresses and VPC endpoints are       allowed. Standard Grafana authentication and authorization are still       required.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkAccessControl {


    /// 
    /// An array of prefix list IDs. A prefix list is a list of CIDR ranges of IP addresses.       The IP addresses specified are allowed to access your workspace. If the list is not       included in the configuration (passed an empty array) then no IP addresses are       allowed to access the workspace. You create a prefix list using the Amazon VPC       console.
    /// 
    /// Prefix list IDs have the format pl-1a2b3c4d.
    /// 
    /// For more information about prefix lists, see Group CIDR blocks using managed         prefix listsin the Amazon Virtual Private Cloud User       Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixListIds")]
    pub prefix_list_ids: Option<Vec<String>>,


    /// 
    /// An array of Amazon VPC endpoint IDs for the workspace. You can create VPC       endpoints to your Amazon Managed Grafana workspace for access from within a VPC. If a         NetworkAccessConfiguration is specified then only VPC endpoints       specified here are allowed to access the workspace. If you pass in an empty array       of strings, then no VPCs are allowed to access the workspace.
    /// 
    /// VPC endpoint IDs have the format       vpce-1a2b3c4d.
    /// 
    /// For more information about creating an interface VPC endpoint, see Interface VPC         endpoints in the Amazon Managed Grafana User       Guide.
    /// 
    /// NoteThe only VPC endpoints that can be specified here are interface VPC endpoints for         Grafana workspaces (using the com.amazonaws.[region].grafana-workspace         service endpoint). Other VPC endpoints are ignored.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpceIds")]
    pub vpce_ids: Option<Vec<String>>,

}




/// This structure defines which groups defined in the SAML assertion attribute are to be       mapped to the Grafana Admin and Editor roles in the workspace.       SAML authenticated users not part of Admin or Editor role       groups have Viewer permission over the workspace.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RoleValues {


    /// 
    /// A list of groups from the SAML assertion attribute to grant the Grafana         Admin role to.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Admin")]
    pub admin: Option<Vec<String>>,


    /// 
    /// A list of groups from the SAML assertion attribute to grant the Grafana         Editor role to.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Editor")]
    pub editor: Option<Vec<String>>,

}




/// A structure containing information about how this workspace works with SAML.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SamlConfiguration {


    /// 
    /// Lists which organizations defined in the SAML assertion are allowed to use the Amazon Managed Grafana workspace. If this is empty, all organizations in the assertion attribute       have access.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedOrganizations")]
    pub allowed_organizations: Option<Vec<String>>,


    /// 
    /// A structure that defines which attributes in the SAML assertion are to be used to       define information about the users authenticated by that IdP to use the       workspace.
    /// 
    /// Required: No
    ///
    /// Type: AssertionAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssertionAttributes")]
    pub assertion_attributes: Option<AssertionAttributes>,


    /// 
    /// A structure containing the identity provider (IdP) metadata used to integrate the       identity provider with this workspace.
    /// 
    /// Required: Yes
    ///
    /// Type: IdpMetadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdpMetadata")]
    pub idp_metadata: IdpMetadata,


    /// 
    /// How long a sign-on session by a SAML user is valid, before the user has to sign on       again.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoginValidityDuration")]
    pub login_validity_duration: Option<f64>,


    /// 
    /// A structure containing arrays that map group names in the SAML assertion to the       Grafana Admin and Editor roles in the workspace.
    /// 
    /// Required: No
    ///
    /// Type: RoleValues
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleValues")]
    pub role_values: Option<RoleValues>,

}




/// The configuration settings for an Amazon VPC that contains data sources for       your Grafana workspace to connect to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfiguration {


    /// 
    /// The list of Amazon EC2 security group IDs attached to the Amazon VPC       for your Grafana workspace to connect. Duplicates not allowed.
    /// 
    /// Array Members: Minimum number of 1 items. Maximum number of 5 items.
    /// 
    /// Length: Minimum length of 0. Maximum length of 255.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,


    /// 
    /// The list of Amazon EC2 subnet IDs created in the Amazon VPC for       your Grafana workspace to connect. Duplicates not allowed.
    /// 
    /// Array Members: Minimum number of 2 items. Maximum number of 6 items.
    /// 
    /// Length: Minimum length of 0. Maximum length of 255.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}


