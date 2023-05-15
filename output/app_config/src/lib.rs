
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// List of Tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tags>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}

pub mod cfn_configuration_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnConfigurationProfile {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RetrievalRoleArn")]
    pub retrieval_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tags>>,
    /// List of Validators
    #[serde(rename = "Validators")]
    pub validators: Option<Vec<Validators>>,
    /// No documentation provided by AWS
    #[serde(rename = "LocationUri")]
    pub location_uri: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Validators {
    #[serde(rename = "Content")]
    pub content: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}


}

pub mod cfn_deployment {

#[derive(serde::Serialize, Default)]
pub struct CfnDeployment {
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyIdentifier")]
    pub kms_key_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentStrategyId")]
    pub deployment_strategy_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
    /// List of Tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tags>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationVersion")]
    pub configuration_version: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}

pub mod cfn_deployment_strategy {

#[derive(serde::Serialize, Default)]
pub struct CfnDeploymentStrategy {
    /// No documentation provided by AWS
    #[serde(rename = "GrowthFactor")]
    pub growth_factor: f64,
    /// No documentation provided by AWS
    #[serde(rename = "FinalBakeTimeInMinutes")]
    pub final_bake_time_in_minutes: Option<f64>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentDurationInMinutes")]
    pub deployment_duration_in_minutes: f64,
    /// No documentation provided by AWS
    #[serde(rename = "GrowthType")]
    pub growth_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicateTo")]
    pub replicate_to: String,
    /// List of Tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tags>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}

pub mod cfn_environment {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironment {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// List of Tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tags>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Monitors
    #[serde(rename = "Monitors")]
    pub monitors: Option<Vec<Monitors>>,

}


#[derive(serde::Serialize, Default)]
pub struct Monitors {
    #[serde(rename = "AlarmArn")]
    pub alarm_arn: Option<String>,
    #[serde(rename = "AlarmRoleArn")]
    pub alarm_role_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}

pub mod cfn_extension {

#[derive(serde::Serialize, Default)]
pub struct CfnExtension {
    /// A list of actions for an extension to take at a specific action point.
    #[serde(rename = "Actions")]
    pub actions: (),
    /// An array of key-value tags to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// Name of the extension.
    #[serde(rename = "Name")]
    pub name: String,
    /// Description of the extension.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LatestVersionNumber")]
    pub latest_version_number: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Uri")]
    pub uri: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Parameter {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Required")]
    pub required: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Actions {

}


}

pub mod cfn_extension_association {

#[derive(serde::Serialize, Default)]
pub struct CfnExtensionAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "ExtensionIdentifier")]
    pub extension_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ExtensionVersionNumber")]
    pub extension_version_number: Option<usize>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_hosted_configuration_version {

#[derive(serde::Serialize, Default)]
pub struct CfnHostedConfigurationVersion {
    /// No documentation provided by AWS
    #[serde(rename = "ContentType")]
    pub content_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "VersionLabel")]
    pub version_label: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LatestVersionNumber")]
    pub latest_version_number: Option<f64>,
    /// No documentation provided by AWS
    #[serde(rename = "Content")]
    pub content: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationId")]
    pub application_id: String,

}



}
