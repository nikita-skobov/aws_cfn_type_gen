
pub mod cfn_notification_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnNotificationChannel {
    /// No documentation provided by AWS
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: ResourceArn,
    /// No documentation provided by AWS
    #[serde(rename = "SnsRoleName")]
    pub sns_role_name: ResourceArn,

}

pub type ResourceArn = String;

}

pub mod cfn_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnPolicy {
    /// List of Base62Id
    #[serde(rename = "ResourceSetIds")]
    pub resource_set_ids: Option<Vec<Base62Id>>,
    /// No documentation provided by AWS
    #[serde(rename = "ExcludeResourceTags")]
    pub exclude_resource_tags: bool,
    /// No documentation provided by AWS
    #[serde(rename = "ExcludeMap")]
    pub exclude_map: Option<IEMap>,
    /// No documentation provided by AWS
    #[serde(rename = "IncludeMap")]
    pub include_map: Option<IEMap>,
    /// Firewall security service policy data.
    #[serde(rename = "SecurityServicePolicyData")]
    pub security_service_policy_data: SecurityServicePolicyData,
    /// No documentation provided by AWS
    #[serde(rename = "RemediationEnabled")]
    pub remediation_enabled: bool,
    /// List of ResourceTag
    #[serde(rename = "ResourceTags")]
    pub resource_tags: Option<Vec<ResourceTag>>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyDescription")]
    pub policy_description: Option<String>,
    /// List of ResourceType
    #[serde(rename = "ResourceTypeList")]
    pub resource_type_list: Option<Vec<ResourceType>>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DeleteAllPolicyResources")]
    pub delete_all_policy_resources: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourcesCleanUp")]
    pub resources_clean_up: Option<bool>,
    /// List of PolicyTag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<PolicyTag>>,
    /// An AWS resource type
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<ResourceType>,

}


#[derive(serde::Serialize, Default)]
pub struct PolicyTag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct SecurityServicePolicyData {
    #[serde(rename = "ManagedServiceData")]
    pub managed_service_data: Option<ManagedServiceData>,
    #[serde(rename = "Type")]
    pub ty: PolicyType,
    #[serde(rename = "PolicyOption")]
    pub policy_option: Option<PolicyOption>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkFirewallPolicy {
    #[serde(rename = "FirewallDeploymentModel")]
    pub firewall_deployment_model: FirewallDeploymentModel,

}
pub type ResourceType = String;
#[derive(serde::Serialize, Default)]
pub struct PolicyOption {
    #[serde(rename = "NetworkFirewallPolicy")]
    pub network_firewall_policy: Option<NetworkFirewallPolicy>,
    #[serde(rename = "ThirdPartyFirewallPolicy")]
    pub third_party_firewall_policy: Option<ThirdPartyFirewallPolicy>,

}
pub type ManagedServiceData = String;pub type AccountId = String;
#[derive(serde::Serialize, Default)]
pub struct ResourceTag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type Base62Id = String;pub type PolicyType = String;
#[derive(serde::Serialize, Default)]
pub struct ThirdPartyFirewallPolicy {
    #[serde(rename = "FirewallDeploymentModel")]
    pub firewall_deployment_model: FirewallDeploymentModel,

}
pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct IEMap {
    #[serde(rename = "ACCOUNT")]
    pub account: Option<Vec<AccountId>>,
    #[serde(rename = "ORGUNIT")]
    pub orgunit: Option<Vec<OrganizationalUnitId>>,

}
pub type OrganizationalUnitId = String;pub type FirewallDeploymentModel = String;

}

pub mod cfn_resource_set {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceSet {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of ResourceType
    #[serde(rename = "ResourceTypeList")]
    pub resource_type_list: Vec<ResourceType>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Resource
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<Resource>>,

}

pub type ResourceType = String;pub type Base62Id = String;pub type Resource = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
