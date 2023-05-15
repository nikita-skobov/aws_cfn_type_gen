
pub mod cfn_app {

#[derive(serde::Serialize, Default)]
pub struct CfnApp {
    /// No documentation provided by AWS
    #[serde(rename = "AutoBranchCreationConfig")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Platform")]
    pub platform: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Repository")]
    pub repository: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BuildSpec")]
    pub build_spec: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of EnvironmentVariable
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    /// List of CustomRule
    #[serde(rename = "CustomRules")]
    pub custom_rules: Option<Vec<CustomRule>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomHeaders")]
    pub custom_headers: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BasicAuthConfig")]
    pub basic_auth_config: Option<BasicAuthConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableBranchAutoDeletion")]
    pub enable_branch_auto_deletion: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "IAMServiceRole")]
    pub iamservice_role: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OauthToken")]
    pub oauth_token: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct CustomRule {
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Condition")]
    pub condition: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AutoBranchCreationConfig {
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    #[serde(rename = "EnablePerformanceMode")]
    pub enable_performance_mode: Option<bool>,
    #[serde(rename = "Framework")]
    pub framework: Option<String>,
    #[serde(rename = "BuildSpec")]
    pub build_spec: Option<String>,
    #[serde(rename = "EnableAutoBuild")]
    pub enable_auto_build: Option<bool>,
    #[serde(rename = "BasicAuthConfig")]
    pub basic_auth_config: Option<BasicAuthConfig>,
    #[serde(rename = "PullRequestEnvironmentName")]
    pub pull_request_environment_name: Option<String>,
    #[serde(rename = "EnableAutoBranchCreation")]
    pub enable_auto_branch_creation: Option<bool>,
    #[serde(rename = "AutoBranchCreationPatterns")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    #[serde(rename = "Stage")]
    pub stage: Option<String>,
    #[serde(rename = "EnablePullRequestPreview")]
    pub enable_pull_request_preview: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct BasicAuthConfig {
    #[serde(rename = "Password")]
    pub password: Option<String>,
    #[serde(rename = "EnableBasicAuth")]
    pub enable_basic_auth: Option<bool>,
    #[serde(rename = "Username")]
    pub username: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct EnvironmentVariable {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Name")]
    pub name: String,

}


}

pub mod cfn_branch {

#[derive(serde::Serialize, Default)]
pub struct CfnBranch {
    /// No documentation provided by AWS
    #[serde(rename = "Framework")]
    pub framework: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Stage")]
    pub stage: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AppId")]
    pub app_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "BuildSpec")]
    pub build_spec: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnablePerformanceMode")]
    pub enable_performance_mode: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "PullRequestEnvironmentName")]
    pub pull_request_environment_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "BasicAuthConfig")]
    pub basic_auth_config: Option<BasicAuthConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableAutoBuild")]
    pub enable_auto_build: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "BranchName")]
    pub branch_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EnablePullRequestPreview")]
    pub enable_pull_request_preview: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of EnvironmentVariable
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct BasicAuthConfig {
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "EnableBasicAuth")]
    pub enable_basic_auth: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct EnvironmentVariable {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Name")]
    pub name: String,

}


}

pub mod cfn_domain {

#[derive(serde::Serialize, Default)]
pub struct CfnDomain {
    /// No documentation provided by AWS
    #[serde(rename = "AutoSubDomainIAMRole")]
    pub auto_sub_domain_iamrole: Option<String>,
    /// List of SubDomainSetting
    #[serde(rename = "SubDomainSettings")]
    pub sub_domain_settings: Vec<SubDomainSetting>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EnableAutoSubDomain")]
    pub enable_auto_sub_domain: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AppId")]
    pub app_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "AutoSubDomainCreationPatterns")]
    pub auto_sub_domain_creation_patterns: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct SubDomainSetting {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "BranchName")]
    pub branch_name: String,

}


}
