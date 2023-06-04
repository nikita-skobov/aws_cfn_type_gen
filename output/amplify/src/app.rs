/// The AWS::Amplify::App resource specifies Apps in Amplify Hosting. An App is a    collection of branches.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnApp {
    ///
    /// The personal access token for a GitHub repository for an Amplify app. The personal access    token is used to authorize access to a GitHub repository using the Amplify GitHub App. The    token is not stored.
    ///
    /// Use AccessToken for GitHub repositories only. To authorize access to a    repository provider such as Bitbucket or CodeCommit, use OauthToken.
    ///
    /// You must specify either AccessToken or OauthToken when you    create a new app.
    ///
    /// Existing Amplify apps deployed from a GitHub repository using OAuth continue to work with    CI/CD. However, we strongly recommend that you migrate these apps to use the GitHub App. For    more information, see Migrating an existing OAuth app to the Amplify GitHub App in the Amplify     User Guide .
    ///
    /// Length Constraints: Minimum length of 1. Maximum length of    255.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<cfn_resources::StrVal>,

    ///
    /// Sets the configuration for your automatic branch creation.
    ///
    /// Required: No
    ///
    /// Type: AutoBranchCreationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoBranchCreationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,

    ///
    /// The credentials for basic authorization for an Amplify app. You must base64-encode       the authorization credentials and provide them in the format       user:password.
    ///
    /// Required: No
    ///
    /// Type: BasicAuthConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuthConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_config: Option<BasicAuthConfig>,

    ///
    /// The build specification (build spec) for an Amplify app.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<cfn_resources::StrVal>,

    ///
    /// The custom HTTP headers for an Amplify app.
    ///
    /// Length Constraints: Minimum length of 0. Maximum length of    25000.
    ///
    /// Pattern: (?s).*
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<cfn_resources::StrVal>,

    ///
    /// The custom rewrite and redirect rules for an Amplify app.
    ///
    /// Required: No
    ///
    /// Type: List of CustomRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,

    ///
    /// The description for an Amplify app.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Automatically disconnect a branch in Amplify Hosting when you delete a branch from    your Git repository.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableBranchAutoDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_deletion: Option<bool>,

    ///
    /// The environment variables map for an Amplify app.
    ///
    /// Required: No
    ///
    /// Type: List of EnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,

    ///
    /// The AWS Identity and Access Management (IAM) service role for the Amazon Resource Name (ARN) of the    Amplify app.
    ///
    /// Length Constraints: Minimum length of 0. Maximum length of    1000.
    ///
    /// Pattern: (?s).*
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IAMServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iamservice_role: Option<cfn_resources::StrVal>,

    ///
    /// The name for an Amplify app.
    ///
    /// Length Constraints: Minimum length of 1. Maximum length of    255.
    ///
    /// Pattern: (?s).+
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The OAuth token for a third-party source control system for an Amplify app. The OAuth    token is used to create a webhook and a read-only deploy key using SSH cloning. The OAuth    token is not stored.
    ///
    /// Use OauthToken for repository providers other than GitHub, such as Bitbucket    or CodeCommit. To authorize access to GitHub as your repository provider, use     AccessToken.
    ///
    /// You must specify either OauthToken or AccessToken when you    create a new app.
    ///
    /// Existing Amplify apps deployed from a GitHub repository using OAuth continue to work with    CI/CD. However, we strongly recommend that you migrate these apps to use the GitHub App. For    more information, see Migrating an existing OAuth app to the Amplify GitHub App in the Amplify     User Guide .
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
    #[serde(rename = "OauthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<cfn_resources::StrVal>,

    ///
    /// The platform for the Amplify app. For a static app, set the platform type to WEB.       For a dynamic server-side rendered (SSR) app, set the platform type to       WEB_COMPUTE. For an app requiring Amplify Hosting's original SSR support only, set the platform type to       WEB_DYNAMIC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<cfn_resources::StrVal>,

    ///
    /// The repository for an Amplify app.
    ///
    /// Pattern: (?s).*
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<cfn_resources::StrVal>,

    ///
    /// The tag for an Amplify app.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_app_id: CfnAppappid,

    #[serde(skip_serializing)]
    pub att_app_name: CfnAppappname,

    #[serde(skip_serializing)]
    pub att_arn: CfnApparn,

    #[serde(skip_serializing)]
    pub att_default_domain: CfnAppdefaultdomain,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAppappid;
impl CfnAppappid {
    pub fn att_name(&self) -> &'static str {
        r#"AppId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAppappname;
impl CfnAppappname {
    pub fn att_name(&self) -> &'static str {
        r#"AppName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnApparn;
impl CfnApparn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAppdefaultdomain;
impl CfnAppdefaultdomain {
    pub fn att_name(&self) -> &'static str {
        r#"DefaultDomain"#
    }
}

impl cfn_resources::CfnResource for CfnApp {
    fn type_string(&self) -> &'static str {
        "AWS::Amplify::App"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.auto_branch_creation_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.basic_auth_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Use the AutoBranchCreationConfig property type to automatically create branches that    match a certain pattern.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AutoBranchCreationConfig {
    ///
    /// Automated branch creation glob patterns for the Amplify app.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoBranchCreationPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,

    ///
    /// Sets password protection for your auto created branch.
    ///
    /// Required: No
    ///
    /// Type: BasicAuthConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuthConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_config: Option<BasicAuthConfig>,

    ///
    /// The build specification (build spec) for the autocreated branch.
    ///
    /// Length Constraints: Minimum length of 1. Maximum length of    25000.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BuildSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<cfn_resources::StrVal>,

    ///
    /// Enables automated branch creation for the Amplify app.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableAutoBranchCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,

    ///
    /// Enables auto building for the auto created branch.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableAutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,

    ///
    /// Sets whether pull request previews are enabled for each branch that Amplify Hosting    automatically creates for your app. Amplify creates previews by deploying your app to    a unique URL whenever a pull request is opened for the branch. Development and QA teams can    use this preview to test the pull request before it's merged into a production or integration    branch.
    ///
    /// To provide backend support for your preview, Amplify Hosting automatically    provisions a temporary backend environment that it deletes when the pull request is closed. If    you want to specify a dedicated backend environment for your previews, use the     PullRequestEnvironmentName property.
    ///
    /// For more information, see Web Previews in the      AWS Amplify Hosting User Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnablePullRequestPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pull_request_preview: Option<bool>,

    ///
    /// Environment variables for the auto created branch.
    ///
    /// Required: No
    ///
    /// Type: List of EnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,

    ///
    /// The framework for the autocreated branch.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Framework")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<cfn_resources::StrVal>,

    ///
    /// If pull request previews are enabled, you can use this property to specify a dedicated    backend environment for your previews. For example, you could specify an environment named     prod, test, or dev that you initialized with the    Amplify CLI.
    ///
    /// To enable pull request previews, set the EnablePullRequestPreview property    to true.
    ///
    /// If you don't specify an environment, Amplify Hosting provides backend support for    each preview by automatically provisioning a temporary backend environment. Amplify    deletes this environment when the pull request is closed.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<cfn_resources::StrVal>,

    ///
    /// Stage for the auto created branch.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AutoBranchCreationConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.basic_auth_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Use the BasicAuthConfig property type to set password protection at an app level to all    your branches.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BasicAuthConfig {
    ///
    /// Enables basic authorization for the Amplify app's branches.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableBasicAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,

    ///
    /// The password for basic authorization.
    ///
    /// Length Constraints: Minimum length of 1. Maximum length of    255.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<cfn_resources::StrVal>,

    ///
    /// The user name for basic authorization.
    ///
    /// Length Constraints: Minimum length of 1. Maximum length of    255.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for BasicAuthConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The CustomRule property type allows you to specify redirects, rewrites, and reverse    proxies. Redirects enable a web app to reroute navigation from one URL to another.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomRule {
    ///
    /// The condition for a URL rewrite or redirect rule, such as a country code.
    ///
    /// Length Constraints: Minimum length of 0. Maximum length of 2048.
    ///
    /// Pattern: (?s).*
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<cfn_resources::StrVal>,

    ///
    /// The source pattern for a URL rewrite or redirect rule.
    ///
    /// Length Constraints: Minimum length of 1. Maximum length of    2048.
    ///
    /// Pattern: (?s).+
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: cfn_resources::StrVal,

    ///
    /// The status code for a URL rewrite or redirect rule.
    ///
    /// 200          Represents a 200 rewrite rule.             301          Represents a 301 (moved pemanently) redirect rule. This and all future requests       should be directed to the target URL.              302          Represents a 302 temporary redirect rule.             404          Represents a 404 redirect rule.             404-200          Represents a 404 rewrite rule.
    ///
    /// Length Constraints: Minimum length of 3. Maximum length of 7.
    ///
    /// Pattern: .{3,7}
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,

    ///
    /// The target pattern for a URL rewrite or redirect rule.
    ///
    /// Length Constraints: Minimum length of 1. Maximum length of    2048.
    ///
    /// Pattern: (?s).+
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CustomRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Environment variables are key-value pairs that are available at build time. Set    environment variables for all branches in your app.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EnvironmentVariable {
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
    pub name: cfn_resources::StrVal,

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
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EnvironmentVariable {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
