/// The AWS::CodeStar::GitHubRepository resource creates a GitHub       repository where users can store source code for use with AWS workflows. You must provide a location for       the source code ZIP file in the AWS CloudFormation template, so the       code can be uploaded to the created repository. You must have       created a personal access token in GitHub to provide in the AWS CloudFormation template. AWS uses this token to connect to GitHub on your behalf. For more information       about using a GitHub source repository with AWS CodeStar projects, see AWS CodeStar Project Files and Resources.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGitHubRepository {
    ///
    /// Information about code to be committed to a repository after it is created in an       AWS CloudFormation stack.
    ///
    /// Required: No
    ///
    /// Type: Code
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,

    ///
    /// Indicates whether to enable issues for the GitHub repository. You can use GitHub       issues to track information and bugs for your repository.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "EnableIssues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_issues: Option<bool>,

    ///
    /// Indicates whether the GitHub repository is a private repository. If so, you choose       who can see and commit to this repository.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "IsPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,

    ///
    /// The GitHub user's personal access token for the GitHub repository.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RepositoryAccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_access_token: Option<String>,

    ///
    /// A comment or description about the new repository. This description is displayed in       GitHub after the repository is created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RepositoryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,

    ///
    /// The name of the repository you want to create in GitHub with AWS CloudFormation       stack creation.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RepositoryName")]
    pub repository_name: String,

    ///
    /// The GitHub user name for the owner of the GitHub repository to be created. If this       repository should be owned by a GitHub organization, provide its name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RepositoryOwner")]
    pub repository_owner: String,
}

impl cfn_resources::CfnResource for CfnGitHubRepository {
    fn type_string(&self) -> &'static str {
        "AWS::CodeStar::GitHubRepository"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.code.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The Code property type specifies information about code to be       committed.
///
/// Code is a property of the AWS::CodeStar::GitHubRepository       resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Code {
    ///
    /// Information about the Amazon S3 bucket that contains a ZIP file of code to be       committed to the repository.
    ///
    /// Required: Yes
    ///
    /// Type: S3
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "S3")]
    pub s3: S3,
}

impl cfn_resources::CfnResource for Code {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3.validate()?;

        Ok(())
    }
}

/// The S3 property type specifies information about the Amazon S3 bucket       that contains the code to be committed to the new repository.
///
/// S3 is a property of the AWS::CodeStar::GitHubRepository       resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3 {
    ///
    /// The name of the Amazon S3 bucket that contains the ZIP file with the content to be       committed to the new repository.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Bucket")]
    pub bucket: String,

    ///
    /// The S3 object key or file name for the ZIP file.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The object version of the ZIP file, if versioning is enabled for the Amazon S3       bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version: Option<String>,
}

impl cfn_resources::CfnResource for S3 {
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
