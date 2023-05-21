

/// The AWS::Amplify::Branch resource specifies a new branch within an app.
#[derive(Default, serde::Serialize)]
pub struct CfnBranch {


    /// 
    /// The unique ID for an Amplify app.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of 20.
    /// 
    /// Pattern: d[a-z0-9]+
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AppId")]
    pub app_id: String,


    /// 
    /// The tag for the branch.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The description for the branch that is part of an Amplify app.
    /// 
    /// Length Constraints: Maximum length of 1000.
    /// 
    /// Pattern: (?s).*
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The build specification (build spec) for the branch.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of    25000.
    /// 
    /// Pattern: (?s).+
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BuildSpec")]
    pub build_spec: Option<String>,


    /// 
    /// Enables performance mode for the branch.
    /// 
    /// Performance mode optimizes for faster hosting performance by keeping content cached at       the edge for a longer interval. When performance mode is enabled, hosting configuration       or code changes can take up to 10 minutes to roll out.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnablePerformanceMode")]
    pub enable_performance_mode: Option<bool>,


    /// 
    /// If pull request previews are enabled for this branch, you can use this property to    specify a dedicated backend environment for your previews. For example, you could specify an    environment named prod, test, or dev that you    initialized with the Amplify CLI and mapped to this branch.
    /// 
    /// To enable pull request previews, set the EnablePullRequestPreview property    to true.
    /// 
    /// If you don't specify an environment, Amplify Hosting provides backend support for    each preview by automatically provisioning a temporary backend environment. Amplify Hosting    deletes this environment when the pull request is closed.
    /// 
    /// For more information about creating backend environments, see Feature Branch     Deployments and Team Workflows in the AWS Amplify Hosting     User Guide.
    /// 
    /// Length Constraints: Maximum length of 20.
    /// 
    /// Pattern: (?s).*
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PullRequestEnvironmentName")]
    pub pull_request_environment_name: Option<String>,


    /// 
    /// The environment variables for the branch.
    /// 
    /// Required: No
    ///
    /// Type: List of EnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,


    /// 
    /// Describes the current stage for the branch.
    /// 
    /// Valid Values: PRODUCTION | BETA | DEVELOPMENT | EXPERIMENTAL |    PULL_REQUEST
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stage")]
    pub stage: Option<String>,


    /// 
    /// Specifies whether Amplify Hosting creates a preview for each pull request that is made    for this branch. If this property is enabled, Amplify deploys your app to a unique    preview URL after each pull request is opened. Development and QA teams can use this preview    to test the pull request before it's merged into a production or integration branch.
    /// 
    /// To provide backend support for your preview, Amplify automatically    provisions a temporary backend environment that it deletes when the pull request is closed. If    you want to specify a dedicated backend environment for your previews, use the     PullRequestEnvironmentName property.
    /// 
    /// For more information, see Web Previews in the      AWS Amplify Hosting User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnablePullRequestPreview")]
    pub enable_pull_request_preview: Option<bool>,


    /// 
    /// The basic authorization credentials for a branch of an Amplify app. You must       base64-encode the authorization credentials and provide them in the format         user:password.
    /// 
    /// Required: No
    ///
    /// Type: BasicAuthConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuthConfig")]
    pub basic_auth_config: Option<BasicAuthConfig>,


    /// 
    /// Enables auto building for the branch.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableAutoBuild")]
    pub enable_auto_build: Option<bool>,


    /// 
    /// The framework for the branch.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Framework")]
    pub framework: Option<String>,


    /// 
    /// The name for the branch.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of    255.
    /// 
    /// Pattern: (?s).+
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BranchName")]
    pub branch_name: String,

}


/// Use the BasicAuthConfig property type to set password protection for a specific    branch.
#[derive(Default, serde::Serialize)]
pub struct BasicAuthConfig {


    /// 
    /// Enables basic authorization for the branch.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableBasicAuth")]
    pub enable_basic_auth: Option<bool>,


    /// 
    /// The password for basic authorization.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of    255.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,


    /// 
    /// The user name for basic authorization.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of    255.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: String,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The EnvironmentVariable property type sets environment variables for a specific branch.    Environment variables are key-value pairs that are available at build time.
#[derive(Default, serde::Serialize)]
pub struct EnvironmentVariable {


    /// 
    /// The environment variable value.
    /// 
    /// Length Constraints: Maximum length of 5500.
    /// 
    /// Pattern: (?s).*
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The environment variable name.
    /// 
    /// Length Constraints: Maximum length of 255.
    /// 
    /// Pattern: (?s).*
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}