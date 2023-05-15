
pub mod cfn_accepted_portfolio_share {

#[derive(serde::Serialize, Default)]
pub struct CfnAcceptedPortfolioShare {
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,

}



}

pub mod cfn_cloud_formation_product {

#[derive(serde::Serialize, Default)]
pub struct CfnCloudFormationProduct {
    /// List of ProvisioningArtifactProperties
    #[serde(rename = "ProvisioningArtifactParameters")]
    pub provisioning_artifact_parameters: Option<Vec<ProvisioningArtifactProperties>>,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "SupportUrl")]
    pub support_url: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceConnection")]
    pub source_connection: Option<SourceConnection>,
    /// No documentation provided by AWS
    #[serde(rename = "Distributor")]
    pub distributor: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProductType")]
    pub product_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SupportEmail")]
    pub support_email: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Owner")]
    pub owner: String,
    /// No documentation provided by AWS
    #[serde(rename = "ReplaceProvisioningArtifacts")]
    pub replace_provisioning_artifacts: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SupportDescription")]
    pub support_description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ConnectionParameters {
    #[serde(rename = "CodeStar")]
    pub code_star: Option<CodeStarParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct SourceConnection {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "ConnectionParameters")]
    pub connection_parameters: ConnectionParameters,

}

#[derive(serde::Serialize, Default)]
pub struct CodeStarParameters {
    #[serde(rename = "Repository")]
    pub repository: String,
    #[serde(rename = "Branch")]
    pub branch: String,
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: String,
    #[serde(rename = "ArtifactPath")]
    pub artifact_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct ProvisioningArtifactProperties {
    #[serde(rename = "Info")]
    pub info: (),
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "DisableTemplateValidation")]
    pub disable_template_validation: Option<bool>,

}


}

pub mod cfn_cloud_formation_provisioned_product {

#[derive(serde::Serialize, Default)]
pub struct CfnCloudFormationProvisionedProduct {
    /// No documentation provided by AWS
    #[serde(rename = "NotificationArns")]
    pub notification_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ProvisionedProductName")]
    pub provisioned_product_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProvisioningArtifactName")]
    pub provisioning_artifact_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "PathName")]
    pub path_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: Option<String>,
    /// List of ProvisioningParameter
    #[serde(rename = "ProvisioningParameters")]
    pub provisioning_parameters: Option<Vec<ProvisioningParameter>>,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProvisioningPreferences")]
    pub provisioning_preferences: Option<ProvisioningPreferences>,
    /// No documentation provided by AWS
    #[serde(rename = "PathId")]
    pub path_id: Option<String>,

}

pub type OutputType = String;
#[derive(serde::Serialize, Default)]
pub struct ProvisioningParameter {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ProvisioningPreferences {
    #[serde(rename = "StackSetOperationType")]
    pub stack_set_operation_type: Option<String>,
    #[serde(rename = "StackSetRegions")]
    pub stack_set_regions: Option<Vec<String>>,
    #[serde(rename = "StackSetMaxConcurrencyCount")]
    pub stack_set_max_concurrency_count: Option<usize>,
    #[serde(rename = "StackSetFailureToleranceCount")]
    pub stack_set_failure_tolerance_count: Option<usize>,
    #[serde(rename = "StackSetAccounts")]
    pub stack_set_accounts: Option<Vec<String>>,
    #[serde(rename = "StackSetFailureTolerancePercentage")]
    pub stack_set_failure_tolerance_percentage: Option<usize>,
    #[serde(rename = "StackSetMaxConcurrencyPercentage")]
    pub stack_set_max_concurrency_percentage: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_launch_notification_constraint {

#[derive(serde::Serialize, Default)]
pub struct CfnLaunchNotificationConstraint {
    /// No documentation provided by AWS
    #[serde(rename = "NotificationArns")]
    pub notification_arns: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



}

pub mod cfn_launch_role_constraint {

#[derive(serde::Serialize, Default)]
pub struct CfnLaunchRoleConstraint {
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LocalRoleName")]
    pub local_role_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,

}



}

pub mod cfn_launch_template_constraint {

#[derive(serde::Serialize, Default)]
pub struct CfnLaunchTemplateConstraint {
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Rules")]
    pub rules: String,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



}

pub mod cfn_portfolio {

#[derive(serde::Serialize, Default)]
pub struct CfnPortfolio {
    /// No documentation provided by AWS
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_portfolio_principal_association {

#[derive(serde::Serialize, Default)]
pub struct CfnPortfolioPrincipalAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "PrincipalType")]
    pub principal_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "PrincipalARN")]
    pub principal_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,

}



}

pub mod cfn_portfolio_product_association {

#[derive(serde::Serialize, Default)]
pub struct CfnPortfolioProductAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "SourcePortfolioId")]
    pub source_portfolio_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,

}



}

pub mod cfn_portfolio_share {

#[derive(serde::Serialize, Default)]
pub struct CfnPortfolioShare {
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ShareTagOptions")]
    pub share_tag_options: Option<bool>,

}



}

pub mod cfn_resource_update_constraint {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceUpdateConstraint {
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TagUpdateOnProvisionedProduct")]
    pub tag_update_on_provisioned_product: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ProductId")]
    pub product_id: String,

}



}

pub mod cfn_service_action {

#[derive(serde::Serialize, Default)]
pub struct CfnServiceAction {
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DefinitionType")]
    pub definition_type: String,
    /// List of DefinitionParameter
    #[serde(rename = "Definition")]
    pub definition: Vec<DefinitionParameter>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct DefinitionParameter {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_service_action_association {

#[derive(serde::Serialize, Default)]
pub struct CfnServiceActionAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceActionId")]
    pub service_action_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: String,

}



}

pub mod cfn_stack_set_constraint {

#[derive(serde::Serialize, Default)]
pub struct CfnStackSetConstraint {
    /// No documentation provided by AWS
    #[serde(rename = "StackInstanceControl")]
    pub stack_instance_control: String,
    /// No documentation provided by AWS
    #[serde(rename = "RegionList")]
    pub region_list: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProductId")]
    pub product_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ExecutionRole")]
    pub execution_role: String,
    /// No documentation provided by AWS
    #[serde(rename = "AdminRole")]
    pub admin_role: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: String,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AccountList")]
    pub account_list: Vec<String>,

}



}

pub mod cfn_tag_option {

#[derive(serde::Serialize, Default)]
pub struct CfnTagOption {
    /// No documentation provided by AWS
    #[serde(rename = "Value")]
    pub value: String,
    /// No documentation provided by AWS
    #[serde(rename = "Active")]
    pub active: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Key")]
    pub key: String,

}



}

pub mod cfn_tag_option_association {

#[derive(serde::Serialize, Default)]
pub struct CfnTagOptionAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "TagOptionId")]
    pub tag_option_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceId")]
    pub resource_id: String,

}



}
