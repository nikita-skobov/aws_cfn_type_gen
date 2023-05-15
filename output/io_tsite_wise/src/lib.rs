
pub mod cfn_access_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessPolicy {
    /// The AWS IoT SiteWise Monitor resource for this access policy. Choose either portal or project but not both.
    #[serde(rename = "AccessPolicyResource")]
    pub access_policy_resource: AccessPolicyResource,
    /// The permission level for this access policy. Valid values are ADMINISTRATOR or VIEWER.
    #[serde(rename = "AccessPolicyPermission")]
    pub access_policy_permission: String,
    /// The identity for this access policy. Choose either a user or a group but not both.
    #[serde(rename = "AccessPolicyIdentity")]
    pub access_policy_identity: AccessPolicyIdentity,

}


#[derive(serde::Serialize, Default)]
pub struct User {
    #[serde(rename = "id")]
    pub id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct IamUser {
    #[serde(rename = "arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct IamRole {
    #[serde(rename = "arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Portal {
    #[serde(rename = "id")]
    pub id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Project {
    #[serde(rename = "id")]
    pub id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessPolicyIdentity {
    #[serde(rename = "IamUser")]
    pub iam_user: Option<IamUser>,
    #[serde(rename = "User")]
    pub user: Option<User>,
    #[serde(rename = "IamRole")]
    pub iam_role: Option<IamRole>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessPolicyResource {
    #[serde(rename = "Portal")]
    pub portal: Option<Portal>,
    #[serde(rename = "Project")]
    pub project: Option<Project>,

}


}

pub mod cfn_asset {

#[derive(serde::Serialize, Default)]
pub struct CfnAsset {
    /// A description for the asset
    #[serde(rename = "AssetDescription")]
    pub asset_description: Option<String>,
    /// The ID of the asset model from which to create the asset.
    #[serde(rename = "AssetModelId")]
    pub asset_model_id: String,
    /// List of AssetHierarchy
    #[serde(rename = "AssetHierarchies")]
    pub asset_hierarchies: Option<Vec<AssetHierarchy>>,
    /// A list of key-value pairs that contain metadata for the asset.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A unique, friendly name for the asset.
    #[serde(rename = "AssetName")]
    pub asset_name: String,
    /// List of AssetProperty
    #[serde(rename = "AssetProperties")]
    pub asset_properties: Option<Vec<AssetProperty>>,

}


#[derive(serde::Serialize, Default)]
pub struct AssetHierarchy {
    #[serde(rename = "LogicalId")]
    pub logical_id: String,
    #[serde(rename = "ChildAssetId")]
    pub child_asset_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct AssetProperty {
    #[serde(rename = "LogicalId")]
    pub logical_id: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "NotificationState")]
    pub notification_state: Option<String>,
    #[serde(rename = "Alias")]
    pub alias: Option<String>,

}


}

pub mod cfn_asset_model {

#[derive(serde::Serialize, Default)]
pub struct CfnAssetModel {
    /// The property definitions of the asset model. You can specify up to 200 properties per asset model.
    #[serde(rename = "AssetModelProperties")]
    pub asset_model_properties: Option<Vec<AssetModelProperty>>,
    /// The hierarchy definitions of the asset model. Each hierarchy specifies an asset model whose assets can be children of any other assets created from this asset model. You can specify up to 10 hierarchies per asset model.
    #[serde(rename = "AssetModelHierarchies")]
    pub asset_model_hierarchies: Option<Vec<AssetModelHierarchy>>,
    /// A description for the asset model.
    #[serde(rename = "AssetModelDescription")]
    pub asset_model_description: Option<String>,
    /// A list of key-value pairs that contain metadata for the asset model.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The composite asset models that are part of this asset model. Composite asset models are asset models that contain specific properties.
    #[serde(rename = "AssetModelCompositeModels")]
    pub asset_model_composite_models: Option<Vec<AssetModelCompositeModel>>,
    /// A unique, friendly name for the asset model.
    #[serde(rename = "AssetModelName")]
    pub asset_model_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct AssetModelCompositeModel {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CompositeModelProperties")]
    pub composite_model_properties: Option<Vec<AssetModelProperty>>,
    #[serde(rename = "Type")]
    pub ty: String,

}
pub type DataType = String;
#[derive(serde::Serialize, Default)]
pub struct Attribute {
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VariableValue {
    #[serde(rename = "PropertyLogicalId")]
    pub property_logical_id: String,
    #[serde(rename = "HierarchyLogicalId")]
    pub hierarchy_logical_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PropertyType {
    #[serde(rename = "Attribute")]
    pub attribute: Option<Attribute>,
    #[serde(rename = "TypeName")]
    pub type_name: TypeName,
    #[serde(rename = "Transform")]
    pub transform: Option<Transform>,
    #[serde(rename = "Metric")]
    pub metric: Option<Metric>,

}

#[derive(serde::Serialize, Default)]
pub struct Metric {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "Variables")]
    pub variables: Vec<ExpressionVariable>,
    #[serde(rename = "Window")]
    pub window: MetricWindow,

}

#[derive(serde::Serialize, Default)]
pub struct AssetModelProperty {
    #[serde(rename = "LogicalId")]
    pub logical_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DataTypeSpec")]
    pub data_type_spec: Option<DataTypeSpec>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "DataType")]
    pub data_type: DataType,
    #[serde(rename = "Type")]
    pub ty: PropertyType,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct TumblingWindow {
    #[serde(rename = "Offset")]
    pub offset: Option<Offset>,
    #[serde(rename = "Interval")]
    pub interval: Interval,

}
pub type Offset = String;
#[derive(serde::Serialize, Default)]
pub struct Transform {
    #[serde(rename = "Variables")]
    pub variables: Vec<ExpressionVariable>,
    #[serde(rename = "Expression")]
    pub expression: String,

}
pub type Interval = String;
#[derive(serde::Serialize, Default)]
pub struct ExpressionVariable {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: VariableValue,

}

#[derive(serde::Serialize, Default)]
pub struct MetricWindow {
    #[serde(rename = "Tumbling")]
    pub tumbling: Option<TumblingWindow>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetModelHierarchy {
    #[serde(rename = "LogicalId")]
    pub logical_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ChildAssetModelId")]
    pub child_asset_model_id: String,

}
pub type TypeName = String;pub type DataTypeSpec = String;

}

pub mod cfn_dashboard {

#[derive(serde::Serialize, Default)]
pub struct CfnDashboard {
    /// A description for the dashboard.
    #[serde(rename = "DashboardDescription")]
    pub dashboard_description: String,
    /// The ID of the project in which to create the dashboard.
    #[serde(rename = "ProjectId")]
    pub project_id: Option<String>,
    /// A friendly name for the dashboard.
    #[serde(rename = "DashboardName")]
    pub dashboard_name: String,
    /// A list of key-value pairs that contain metadata for the dashboard.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The dashboard definition specified in a JSON literal.
    #[serde(rename = "DashboardDefinition")]
    pub dashboard_definition: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_gateway {

#[derive(serde::Serialize, Default)]
pub struct CfnGateway {
    /// A list of key-value pairs that contain metadata for the gateway.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A unique, friendly name for the gateway.
    #[serde(rename = "GatewayName")]
    pub gateway_name: String,
    /// The gateway's platform. You can only specify one platform in a gateway.
    #[serde(rename = "GatewayPlatform")]
    pub gateway_platform: GatewayPlatform,
    /// A list of gateway capability summaries that each contain a namespace and status.
    #[serde(rename = "GatewayCapabilitySummaries")]
    pub gateway_capability_summaries: Option<Vec<GatewayCapabilitySummary>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct GatewayCapabilitySummary {
    #[serde(rename = "CapabilityNamespace")]
    pub capability_namespace: CapabilityNamespace,
    #[serde(rename = "CapabilityConfiguration")]
    pub capability_configuration: Option<CapabilityConfiguration>,

}
pub type CapabilityNamespace = String;
#[derive(serde::Serialize, Default)]
pub struct GatewayPlatform {
    #[serde(rename = "Greengrass")]
    pub greengrass: Option<Greengrass>,
    #[serde(rename = "GreengrassV2")]
    pub greengrass_v2: Option<GreengrassV2>,

}
pub type CapabilityConfiguration = String;
#[derive(serde::Serialize, Default)]
pub struct GreengrassV2 {
    #[serde(rename = "CoreDeviceThingName")]
    pub core_device_thing_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Greengrass {
    #[serde(rename = "GroupArn")]
    pub group_arn: String,

}


}

pub mod cfn_portal {

#[derive(serde::Serialize, Default)]
pub struct CfnPortal {
    /// The email address that sends alarm notifications.
    #[serde(rename = "NotificationSenderEmail")]
    pub notification_sender_email: Option<String>,
    /// A list of key-value pairs that contain metadata for the portal.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The AWS administrator's contact email address.
    #[serde(rename = "PortalContactEmail")]
    pub portal_contact_email: String,
    /// A description for the portal.
    #[serde(rename = "PortalDescription")]
    pub portal_description: Option<String>,
    /// Contains the configuration information of an alarm created in an AWS IoT SiteWise Monitor portal. You can use the alarm to monitor an asset property and get notified when the asset property value is outside a specified range.
    #[serde(rename = "Alarms")]
    pub alarms: Option<()>,
    /// The service to use to authenticate users to the portal. Choose from SSO or IAM. You can't change this value after you create a portal.
    #[serde(rename = "PortalAuthMode")]
    pub portal_auth_mode: Option<String>,
    /// A friendly name for the portal.
    #[serde(rename = "PortalName")]
    pub portal_name: String,
    /// The ARN of a service role that allows the portal's users to access your AWS IoT SiteWise resources on your behalf.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_project {

#[derive(serde::Serialize, Default)]
pub struct CfnProject {
    /// The ID of the portal in which to create the project.
    #[serde(rename = "PortalId")]
    pub portal_id: String,
    /// A friendly name for the project.
    #[serde(rename = "ProjectName")]
    pub project_name: String,
    /// A list of key-value pairs that contain metadata for the project.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A description for the project.
    #[serde(rename = "ProjectDescription")]
    pub project_description: Option<String>,
    /// The IDs of the assets to be associated to the project.
    #[serde(rename = "AssetIds")]
    pub asset_ids: Option<Vec<AssetId>>,

}

pub type AssetId = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
