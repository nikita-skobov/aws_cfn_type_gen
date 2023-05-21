

/// The AWS::CodeBuild::Project resource configures how AWS CodeBuild builds your source     code. For example, it tells CodeBuild where to get the source code and which build environment to     use.
#[derive(Default, serde::Serialize)]
pub struct CfnProject {


    /// 
    /// An array of ProjectSource objects.
    /// 
    /// Required: No
    ///
    /// Type: List of Source
    ///
    /// Maximum: 12
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondarySources")]
    pub secondary_sources: Option<Vec<Source>>,


    /// 
    /// The maximum number of concurrent builds that are allowed for this project.
    /// 
    /// New builds are only started if the current number of builds is less than or equal to this limit.  If the current build count meets this limit, new builds are throttled and are not run.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConcurrentBuildLimit")]
    pub concurrent_build_limit: Option<i64>,


    /// 
    /// A description that makes the build project easy to identify.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The ARN of the IAM role that enables CodeBuild to access the CloudWatch Logs and Amazon S3 artifacts for    the project's builds.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceAccessRole")]
    pub resource_access_role: Option<String>,


    /// 
    /// The source code settings for the project, such as the source code's repository type and location.
    /// 
    /// Required: Yes
    ///
    /// Type: Source
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: Source,


    /// 
    /// The build environment settings for the project, such as the environment type or the       environment variables to use for the build environment.
    /// 
    /// Required: Yes
    ///
    /// Type: Environment
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    pub environment: Environment,


    /// 
    /// Indicates whether AWS CodeBuild generates a publicly accessible URL for your project's build badge. For more information,       see Build Badges Sample       in the AWS CodeBuild User Guide.
    /// 
    /// Note        Including build badges with your project is currently not supported if the source type is CodePipeline.         If you specify CODEPIPELINE for the Source property, do not specify the         BadgeEnabled property.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BadgeEnabled")]
    pub badge_enabled: Option<bool>,


    /// 
    /// Information about logs for the build project. A project can create logs in CloudWatch Logs, an    S3 bucket, or both.
    /// 
    /// Required: No
    ///
    /// Type: LogsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogsConfig")]
    pub logs_config: Option<LogsConfig>,


    /// 
    /// For an existing AWS CodeBuild build project that has its source code stored      in a GitHub repository, enables AWS CodeBuild to begin automatically rebuilding      the source code every time a code change is pushed to the repository.
    /// 
    /// Required: No
    ///
    /// Type: ProjectTriggers
    ///
    /// Update requires: No interruption
    #[serde(rename = "Triggers")]
    pub triggers: Option<ProjectTriggers>,


    /// 
    /// Settings that AWS CodeBuild uses to store and reuse build dependencies.
    /// 
    /// Required: No
    ///
    /// Type: ProjectCache
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cache")]
    pub cache: Option<ProjectCache>,


    /// 
    /// VpcConfig specifies settings that enable AWS CodeBuild to access resources in an Amazon VPC. For more information, see       Use AWS CodeBuild with Amazon Virtual Private Cloud in       the AWS CodeBuild User Guide.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,


    /// 
    /// A version of the build input to be built for this project. If not specified, the    latest version is used. If specified, it must be one of:
    /// 
    /// For CodeCommit: the commit ID, branch, or Git tag to use.               For GitHub: the commit ID, pull request ID, branch name, or tag name that      corresponds to the version of the source code you want to build. If a pull      request ID is specified, it must use the format pr/pull-request-ID      (for example pr/25). If a branch name is specified, the branch's      HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is      used.               For Bitbucket: the commit ID, branch name, or tag name that corresponds to the      version of the source code you want to build. If a branch name is specified, the      branch's HEAD commit ID is used. If not specified, the default branch's HEAD      commit ID is used.               For Amazon S3: the version ID of the object that represents the build input ZIP      file to use.
    /// 
    /// If sourceVersion is specified at the build level, then that version    takes precedence over this sourceVersion (at the project level).
    /// 
    /// For more information, see Source Version Sample    with CodeBuild in the         AWS CodeBuild User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceVersion")]
    pub source_version: Option<String>,


    /// 
    /// Artifacts is a property of the       AWS::CodeBuild::Project resource that specifies output settings for     artifacts generated by an AWS CodeBuild build.
    /// 
    /// Required: Yes
    ///
    /// Type: Artifacts
    ///
    /// Update requires: No interruption
    #[serde(rename = "Artifacts")]
    pub artifacts: Artifacts,


    /// 
    /// An arbitrary set of tags (key-value pairs) for the AWS CodeBuild project.
    /// 
    /// These tags are available for use by AWS services that support AWS CodeBuild build project     tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The AWS Key Management Service customer master key (CMK) to be used for encrypting the build output    artifacts.
    /// 
    /// NoteYou can use a cross-account KMS key to encrypt the build output artifacts if your     service role has permission to that key.
    /// 
    /// You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using     the format alias/<alias-name>). If you don't specify a     value, CodeBuild uses the managed CMK for Amazon Simple Storage Service (Amazon S3).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionKey")]
    pub encryption_key: Option<String>,


    /// 
    /// A list of Artifacts objects. Each artifacts object specifies output settings that the project generates during a build.
    /// 
    /// Required: No
    ///
    /// Type: List of Artifacts
    ///
    /// Maximum: 12
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryArtifacts")]
    pub secondary_artifacts: Option<Vec<Artifacts>>,


    /// 
    /// How long, in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait before timing out any    related build that did not get marked as completed. The default is 60 minutes.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 5
    ///
    /// Maximum: 480
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutInMinutes")]
    pub timeout_in_minutes: Option<i64>,


    /// 
    /// The ARN of the IAM role that enables AWS CodeBuild to interact with dependent AWS services    on behalf of the AWS account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRole")]
    pub service_role: String,


    /// 
    /// The name of the build project. The name must be unique across all of the projects in your      AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 255
    ///
    /// Pattern: [A-Za-z0-9][A-Za-z0-9\-_]{1,254}
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// A    ProjectBuildBatchConfig    object that defines the batch build    options for the project.
    /// 
    /// Required: No
    ///
    /// Type: ProjectBuildBatchConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "BuildBatchConfig")]
    pub build_batch_config: Option<ProjectBuildBatchConfig>,


    /// 
    /// An array of ProjectFileSystemLocation objects for a CodeBuild build project. A ProjectFileSystemLocation object    specifies the identifier, location, mountOptions,    mountPoint, and type of a file system created using Amazon Elastic File System.
    /// 
    /// Required: No
    ///
    /// Type: List of ProjectFileSystemLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileSystemLocations")]
    pub file_system_locations: Option<Vec<ProjectFileSystemLocation>>,


    /// 
    /// An array of ProjectSourceVersion objects. If    secondarySourceVersions is specified at the build level, then they take    over these secondarySourceVersions (at the project level).
    /// 
    /// Required: No
    ///
    /// Type: List of ProjectSourceVersion
    ///
    /// Maximum: 12
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondarySourceVersions")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,


    /// 
    /// The number of minutes a build is allowed to be queued before it times out.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 5
    ///
    /// Maximum: 480
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueuedTimeoutInMinutes")]
    pub queued_timeout_in_minutes: Option<i64>,


    /// 
    /// Specifies the visibility of the project's builds. Possible values are:
    /// 
    /// PUBLIC_READ                  The project builds are visible to the public.                       PRIVATE                  The project builds are not visible to the public.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,

}


/// RegistryCredential is a property of the      AWS CodeBuild Project Environment      property type that specifies information about credentials that provide access to a private Docker registry. When this is set:
///
/// For more information, see Private Registry with      AWS Secrets Manager Sample for AWS CodeBuild.
#[derive(Default, serde::Serialize)]
pub struct RegistryCredential {


    /// 
    /// The service that created the credentials to access a private Docker registry. The       valid value, SECRETS_MANAGER, is for AWS Secrets Manager.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SECRETS_MANAGER
    ///
    /// Update requires: No interruption
    #[serde(rename = "CredentialProvider")]
    pub credential_provider: String,


    /// 
    /// The Amazon Resource Name (ARN) or name of credentials created using AWS Secrets Manager.
    /// 
    /// Note The credential can use the name of the credentials only if they         exist in your current AWS Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credential")]
    pub credential: String,

}


/// WebhookFilter is a structure of the FilterGroups property on the      AWS CodeBuild Project ProjectTriggers      property type      that specifies which webhooks trigger an AWS CodeBuild build.
#[derive(Default, serde::Serialize)]
pub struct WebhookFilter {


    /// 
    /// Used to indicate that the pattern determines which webhook events do not       trigger a build. If true, then a webhook event that does not match the         pattern triggers a build. If false, then a webhook event that matches       the pattern triggers a build.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeMatchedPattern")]
    pub exclude_matched_pattern: Option<bool>,


    /// 
    /// The type of webhook filter. There are six webhook filter types: EVENT,         ACTOR_ACCOUNT_ID, HEAD_REF, BASE_REF,         FILE_PATH, and COMMIT_MESSAGE.
    /// 
    /// EVENT                         A webhook event triggers a build when the provided pattern             matches one of five event types: PUSH,               PULL_REQUEST_CREATED, PULL_REQUEST_UPDATED,               PULL_REQUEST_REOPENED, and             PULL_REQUEST_MERGED. The EVENT patterns are             specified as a comma-separated string. For example, PUSH,               PULL_REQUEST_CREATED, PULL_REQUEST_UPDATED filters all push, pull             request created, and pull request updated events.          Note The PULL_REQUEST_REOPENED works with GitHub and GitHub               Enterprise only.                                ACTOR_ACCOUNT_ID                         A webhook event triggers a build when a GitHub, GitHub Enterprise, or             Bitbucket account ID matches the regular expression pattern.                                          HEAD_REF                         A webhook event triggers a build when the head reference matches the             regular expression pattern. For example,               refs/heads/branch-name and refs/tags/tag-name.           Works with GitHub and GitHub Enterprise push, GitHub and GitHub             Enterprise pull request, Bitbucket push, and Bitbucket pull request events.                                          BASE_REF                         A webhook event triggers a build when the base reference matches the             regular expression pattern. For example,               refs/heads/branch-name.          Note Works with pull request events only.                                FILE_PATH                         A webhook triggers a build when the path of a changed file matches the             regular expression pattern.          Note Works with GitHub and Bitbucket events push and pull requests events.               Also works with GitHub Enterprise push events, but does not work with               GitHub Enterprise pull request events.                        COMMIT_MESSAGE                  A webhook triggers a build when the head commit message matches the             regular expression pattern.          Note Works with GitHub and Bitbucket events push and pull requests events.               Also works with GitHub Enterprise push events, but does not work with               GitHub Enterprise pull request events.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ACTOR_ACCOUNT_ID | BASE_REF | COMMIT_MESSAGE | EVENT | FILE_PATH | HEAD_REF
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// For a WebHookFilter that uses EVENT type, a comma-separated       string that specifies one or more events. For example, the webhook filter PUSH,         PULL_REQUEST_CREATED, PULL_REQUEST_UPDATED allows all push, pull request       created, and pull request updated events to trigger a build.
    /// 
    /// For a WebHookFilter that uses any of the other filter types, a regular       expression pattern. For example, a WebHookFilter that uses         HEAD_REF for its type and the pattern         ^refs/heads/ triggers a build when the head reference is a branch with       a reference name refs/heads/branch-name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pattern")]
    pub pattern: String,

}


/// A source identifier and its corresponding version.
#[derive(Default, serde::Serialize)]
pub struct ProjectSourceVersion {


    /// 
    /// The source version for the corresponding source identifier. If specified, must be one      of:
    /// 
    /// For CodeCommit: the commit ID, branch, or Git tag to use.               For GitHub: the commit ID, pull request ID, branch name, or tag name that          corresponds to the version of the source code you want to build. If a pull          request ID is specified, it must use the format pr/pull-request-ID          (for example, pr/25). If a branch name is specified, the branch's          HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is          used.               For Bitbucket: the commit ID, branch name, or tag name that corresponds to the          version of the source code you want to build. If a branch name is specified, the          branch's HEAD commit ID is used. If not specified, the default branch's HEAD          commit ID is used.               For Amazon S3: the version ID of the object that represents the build input ZIP          file to use.
    /// 
    /// For more information, see Source Version Sample        with CodeBuild in the         AWS CodeBuild User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceVersion")]
    pub source_version: Option<String>,


    /// 
    /// An identifier for a source in the build project. The identifier can only contain       alphanumeric characters and underscores, and must be less than 128 characters in length.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: String,

}


/// LogsConfig is a property of the AWS CodeBuild Project resource      that specifies information about logs for a build project. These can be logs in Amazon CloudWatch Logs, built in a     specified S3 bucket, or both.
#[derive(Default, serde::Serialize)]
pub struct LogsConfig {


    /// 
    /// Information about CloudWatch Logs for a build project. CloudWatch Logs are enabled by default.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLogsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogs")]
    pub cloud_watch_logs: Option<CloudWatchLogsConfig>,


    /// 
    /// Information about logs built to an S3 bucket for a build project. S3 logs are not       enabled by default.
    /// 
    /// Required: No
    ///
    /// Type: S3LogsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Logs")]
    pub s3_logs: Option<S3LogsConfig>,

}


/// Contains configuration information about a batch build project.
#[derive(Default, serde::Serialize)]
pub struct ProjectBuildBatchConfig {


    /// 
    /// Specifies if the build artifacts for the batch build should be combined into a single       artifact location.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CombineArtifacts")]
    pub combine_artifacts: Option<bool>,


    /// 
    /// Specifies the service role ARN for the batch build project.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRole")]
    pub service_role: Option<String>,


    /// 
    /// A BatchRestrictions object that specifies the restrictions for the batch       build.
    /// 
    /// Required: No
    ///
    /// Type: BatchRestrictions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Restrictions")]
    pub restrictions: Option<BatchRestrictions>,


    /// 
    /// Specifies the maximum amount of time, in minutes, that the batch build must be completed in.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutInMins")]
    pub timeout_in_mins: Option<i64>,


    /// 
    /// Specifies how build status reports are sent to the source provider for the batch build. This property is only used        when the source provider for your project is Bitbucket, GitHub, or GitHub Enterprise,        and your project is configured to report build statuses to the source provider.
    /// 
    /// REPORT_AGGREGATED_BATCH                  (Default) Aggregate all of the build statuses into a single status report.                       REPORT_INDIVIDUAL_BUILDS                  Send a separate status report for each individual build.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: REPORT_AGGREGATED_BATCH | REPORT_INDIVIDUAL_BUILDS
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchReportMode")]
    pub batch_report_mode: Option<String>,

}


/// Source is a property of the AWS::CodeBuild::Project resource that specifies       the source code settings for the project, such as the source code's repository type and location.
#[derive(Default, serde::Serialize)]
pub struct Source {


    /// 
    /// The type of repository that contains the source code to be built. Valid values       include:
    /// 
    /// BITBUCKET: The source code is in a Bitbucket repository.                        CODECOMMIT: The source code is in an CodeCommit repository.                        CODEPIPELINE: The source code settings are specified in the           source action of a pipeline in CodePipeline.                        GITHUB: The source code is in a GitHub or GitHub Enterprise Cloud           repository.                        GITHUB_ENTERPRISE: The source code is in a GitHub Enterprise           Server repository.                        NO_SOURCE: The project does not have input source code.                        S3: The source code is in an Amazon S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BITBUCKET | CODECOMMIT | CODEPIPELINE | GITHUB | GITHUB_ENTERPRISE | NO_SOURCE | S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// An identifier for this project source. The identifier can only contain      alphanumeric characters and underscores, and must be less than 128 characters in length.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: Option<String>,


    /// 
    /// Information about the authorization settings for AWS CodeBuild to access the source code to be     built.
    /// 
    /// This information is for the AWS CodeBuild console's use only. Your code should not get or set     Auth directly.
    /// 
    /// Required: No
    ///
    /// Type: SourceAuth
    ///
    /// Update requires: No interruption
    #[serde(rename = "Auth")]
    pub auth: Option<SourceAuth>,


    /// 
    /// This is used with GitHub Enterprise only. Set to true to ignore SSL warnings while connecting to your GitHub Enterprise project repository.       The default value is false. InsecureSsl should be used for testing purposes only. It should not be used in a production environment.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsecureSsl")]
    pub insecure_ssl: Option<bool>,


    /// 
    /// The build specification for the project. If this value is not provided, then the source code must contain a      buildspec file named buildspec.yml at the root level. If this value is provided, it can be either      a single string containing the entire build specification, or the path to an alternate buildspec file relative      to the value of the built-in environment variable CODEBUILD_SRC_DIR. The alternate buildspec file      can have a name other than buildspec.yml, for example myspec.yml or      build_spec_qa.yml or similar. For more information, see the Build Spec Reference in the      AWS CodeBuild User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BuildSpec")]
    pub build_spec: Option<String>,


    /// 
    /// The depth of history to download. Minimum value is 0. If this value is 0,      greater than 25, or not provided, then the full history is downloaded with each build project.      If your source type is Amazon S3, this value is not supported.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "GitCloneDepth")]
    pub git_clone_depth: Option<i64>,


    /// 
    /// Contains information that defines how the build project reports the build status to    the source provider. This option is only used when the source provider is    GITHUB, GITHUB_ENTERPRISE, or    BITBUCKET.
    /// 
    /// Required: No
    ///
    /// Type: BuildStatusConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "BuildStatusConfig")]
    pub build_status_config: Option<BuildStatusConfig>,


    /// 
    /// Set to true to report the status of a build's start and finish to your source provider.     This option is valid only when your source provider is GitHub, GitHub Enterprise, or     Bitbucket. If this is set and you use a different source provider, an invalidInputException     is thrown.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReportBuildStatus")]
    pub report_build_status: Option<bool>,


    /// 
    /// Information about the location of the source code to be built. Valid values       include:
    /// 
    /// For source code settings that are specified in the source action of a pipeline           in CodePipeline, location should not be specified. If it is specified,           CodePipeline ignores it. This is because CodePipeline uses the settings in a pipeline's source           action instead of this value.               For source code in an CodeCommit repository, the HTTPS clone URL to the repository           that contains the source code and the buildspec file (for example,             https://git-codecommit.<region-ID>.amazonaws.com/v1/repos/<repo-name>).               For source code in an Amazon S3 input bucket, one of the following.                                                The path to the ZIP file that contains the source code (for example,                 <bucket-name>/<path>/<object-name>.zip).                     The path to the folder that contains the source code (for example,                 <bucket-name>/<path-to-source-code>/<folder>/).                         For source code in a GitHub repository, the HTTPS clone URL to the repository           that contains the source and the buildspec file. You must connect your AWS account           to your GitHub account. Use the AWS CodeBuild console to start creating a build           project. When you use the console to connect (or reconnect) with GitHub, on the           GitHub Authorize application page, for             Organization access, choose Request access next to each repository you want to           allow AWS CodeBuild to have access to, and then choose Authorize             application. (After you have connected to your GitHub account,           you do not need to finish creating the build project. You can leave the AWS CodeBuild           console.) To instruct AWS CodeBuild to use this connection, in the source           object, set the auth object's type value to             OAUTH.               For source code in a Bitbucket repository, the HTTPS clone URL to the           repository that contains the source and the buildspec file. You must connect           your AWS account to your Bitbucket account. Use the AWS CodeBuild console to start           creating a build project. When you use the console to connect (or reconnect)           with Bitbucket, on the Bitbucket Confirm access to your             account page, choose Grant             access. (After you have connected to your Bitbucket account, you           do not need to finish creating the build project. You can leave the AWS CodeBuild           console.) To instruct AWS CodeBuild to use this connection, in the source           object, set the auth object's type value to             OAUTH.
    /// 
    /// If you specify CODEPIPELINE for the Type property, don't specify this      property. For all of the other types, you must specify Location.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<String>,


    /// 
    /// Information about the Git submodules configuration for the build project.
    /// 
    /// Required: No
    ///
    /// Type: GitSubmodulesConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "GitSubmodulesConfig")]
    pub git_submodules_config: Option<GitSubmodulesConfig>,

}


/// VpcConfig is a property of the AWS::CodeBuild::Project resource       that enable AWS CodeBuild to access resources in an Amazon VPC. For more information, see       Use AWS CodeBuild with Amazon Virtual Private Cloud in       the AWS CodeBuild User Guide.
#[derive(Default, serde::Serialize)]
pub struct VpcConfig {


    /// 
    /// A list of one or more security groups IDs in your Amazon VPC. The maximum count is 5.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// A list of one or more subnet IDs in your Amazon VPC. The maximum count is 16.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,


    /// 
    /// The ID of the Amazon VPC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

}


/// EnvironmentVariable is a property of the AWS CodeBuild Project Environment     property type that specifies the name and value of an environment variable for an AWS CodeBuild      project environment. When you use the environment to run a build, these variables are available for your builds to use. EnvironmentVariable      contains a list of EnvironmentVariable property types.
#[derive(Default, serde::Serialize)]
pub struct EnvironmentVariable {


    /// 
    /// The type of environment variable. Valid values include:
    /// 
    /// PARAMETER_STORE: An environment variable stored in Systems Manager           Parameter Store. For environment variables of this type, specify the name of the parameter as the value of the           EnvironmentVariable. The parameter value will be substituted for the name at runtime. You can also define Parameter           Store environment variables in the buildspec. To learn how to do so,           see env/parameter-store in the                      AWS CodeBuild User Guide.                        PLAINTEXT: An environment variable in plain text format. This is           the default value.                        SECRETS_MANAGER: An environment variable stored in AWS Secrets Manager. For environment variables of this type,           specify the name of the secret as the value of the EnvironmentVariable. The secret value will be substituted for the           name at runtime. You can also define AWS Secrets Manager environment variables in the buildspec. To learn how to do so, see             env/secrets-manager in the                      AWS CodeBuild User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PARAMETER_STORE | PLAINTEXT | SECRETS_MANAGER
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The name or key of the environment variable.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The value of the environment variable.
    /// 
    /// ImportantWe strongly discourage the use of PLAINTEXT environment variables to         store sensitive values, especially AWS secret key IDs and secret access keys.           PLAINTEXT environment variables can be displayed in plain text         using the AWS CodeBuild console and the AWS CLI. For sensitive values, we recommend you use an         environment variable of type PARAMETER_STORE or           SECRETS_MANAGER.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


/// A list of WebhookFilter objects used to determine which webhook events are triggered.      At least one WebhookFilter in the list must specify EVENT as its type. The FilterGroups property      of the AWS CodeBuild Project ProjectTriggers property type is a list of FilterGroup objects.
///
/// Required: No
///
/// Type: A list of WebhookFilter objects
///
/// Update requires: No interruption
#[derive(Default, serde::Serialize)]
pub struct FilterGroup {

}


/// S3Logs is a property of the      AWS CodeBuild Project LogsConfig      property type that specifies settings for logs generated by an AWS CodeBuild build in an S3 bucket.
#[derive(Default, serde::Serialize)]
pub struct S3LogsConfig {


    /// 
    /// The ARN of an S3 bucket and the path prefix for S3 logs. If your Amazon S3 bucket       name is my-bucket, and your path prefix is build-log, then       acceptable formats are my-bucket/build-log or         arn:aws:s3:::my-bucket/build-log.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<String>,


    /// 
    /// Set to true if you do not want your S3 build log output encrypted. By default S3       build logs are encrypted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionDisabled")]
    pub encryption_disabled: Option<bool>,


    /// 
    /// The current status of the S3 build logs. Valid values are:
    /// 
    /// ENABLED: S3 build logs are enabled for this build project.                        DISABLED: S3 build logs are not enabled for this build           project.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: String,

}


/// SourceAuth is a property of the AWS CodeBuild Project Source      property type that specifies authorization settings for AWS CodeBuild to access the source code to be built.
///
/// SourceAuth is for use by the CodeBuild console only. Do not get or set it directly.
#[derive(Default, serde::Serialize)]
pub struct SourceAuth {


    /// 
    /// The authorization type to use. The only valid value is OAUTH, which     represents the OAuth authorization type.
    /// 
    /// Note        This data type is used by the AWS CodeBuild console only.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: OAUTH
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The resource value that applies to the specified authorization type.
    /// 
    /// Note       This data type is used by the AWS CodeBuild console only.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resource")]
    pub resource: Option<String>,

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
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


/// ProjectTriggers is a property of the AWS CodeBuild Project      resource that specifies webhooks that trigger an AWS CodeBuild build.
#[derive(Default, serde::Serialize)]
pub struct ProjectTriggers {


    /// 
    /// Specifies whether or not to begin automatically rebuilding the source code every time a code change is pushed to the repository.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Webhook")]
    pub webhook: Option<bool>,


    /// 
    /// Specifies the type of build this webhook will trigger. Allowed values are:
    /// 
    /// BUILD             A single build               BUILD_BATCH             A batch build
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BuildType")]
    pub build_type: Option<String>,


    /// 
    /// A list of lists of WebhookFilter objects used to determine which webhook      events are triggered. At least one WebhookFilter in the array must specify EVENT as its type.
    /// 
    /// Required: No
    ///
    /// Type: List of FilterGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterGroups")]
    pub filter_groups: Option<Vec<FilterGroup>>,

}


/// Environment is a property of the AWS::CodeBuild::Project resource that specifies       the environment for an AWS CodeBuild project.
#[derive(Default, serde::Serialize)]
pub struct Environment {


    /// 
    /// The type of compute environment. This determines the number of CPU cores and memory the build environment uses. Available values     include:
    /// 
    /// BUILD_GENERAL1_SMALL: Use up to 3 GB memory and 2 vCPUs for        builds.                        BUILD_GENERAL1_MEDIUM: Use up to 7 GB memory and 4 vCPUs for        builds.                        BUILD_GENERAL1_LARGE: Use up to 15 GB memory and 8 vCPUs for        builds.
    /// 
    /// For more information, see Build Environment Compute Types in the       AWS CodeBuild User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeType")]
    pub compute_type: String,


    /// 
    /// Enables running the Docker daemon inside a Docker container. Set to true only if the     build project is used to build Docker images. Otherwise, a build that attempts      to interact with the Docker daemon fails. The default setting is false.
    /// 
    /// You can initialize the Docker daemon during the install phase of your build by adding one of the following sets of commands      to the install phase of your buildspec file:
    /// 
    /// If the operating system's base image is Ubuntu Linux:
    /// 
    /// - nohup /usr/local/bin/dockerd --host=unix:///var/run/docker.sock --host=tcp://0.0.0.0:2375 --storage-driver=overlay&
    /// 
    /// - timeout 15 sh -c "until docker info; do echo .; sleep 1; done"
    /// 
    /// If the operating system's base image is Alpine Linux and the previous command does not work, add the -t argument     to timeout:
    /// 
    /// - nohup /usr/local/bin/dockerd --host=unix:///var/run/docker.sock --host=tcp://0.0.0.0:2375 --storage-driver=overlay&
    /// 
    /// - timeout -t 15 sh -c "until docker info; do echo .; sleep 1; done"
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivilegedMode")]
    pub privileged_mode: Option<bool>,


    /// 
    /// The type of build environment to use for related builds.
    /// 
    /// The environment type ARM_CONTAINER is available only in regions           US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland),           Asia Pacific (Mumbai), Asia Pacific (Tokyo), Asia Pacific (Sydney), and           EU (Frankfurt).               The environment type LINUX_CONTAINER with compute type             build.general1.2xlarge is available only in regions           US East (N. Virginia), US East (Ohio), US West (Oregon),           Canada (Central), EU (Ireland), EU (London),           EU (Frankfurt), Asia Pacific (Tokyo), Asia Pacific (Seoul),           Asia Pacific (Singapore), Asia Pacific (Sydney), China (Beijing), and           China (Ningxia).               The environment type LINUX_GPU_CONTAINER is available only in           regions US East (N. Virginia), US East (Ohio), US West (Oregon),           Canada (Central), EU (Ireland), EU (London),           EU (Frankfurt), Asia Pacific (Tokyo), Asia Pacific (Seoul),           Asia Pacific (Singapore), Asia Pacific (Sydney) , China (Beijing), and           China (Ningxia).
    /// 
    /// The environment types WINDOWS_CONTAINER and             WINDOWS_SERVER_2019_CONTAINER are available only in regions           US East (N. Virginia), US East (Ohio), US West (Oregon), and           EU (Ireland).
    /// 
    /// For more information, see Build environment compute types in the         AWS CodeBuild         user guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ARM_CONTAINER | LINUX_CONTAINER | LINUX_GPU_CONTAINER | WINDOWS_CONTAINER | WINDOWS_SERVER_2019_CONTAINER
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// A set of environment variables to make available to builds for this build       project.
    /// 
    /// Required: No
    ///
    /// Type: List of EnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,


    /// 
    /// The type of credentials AWS CodeBuild uses to pull images in your build. There are two valid       values:
    /// 
    /// CODEBUILD specifies that AWS CodeBuild uses its own credentials.           This requires that you modify your ECR repository policy to trust AWS CodeBuild service principal.                         SERVICE_ROLE specifies that AWS CodeBuild uses your build project's service           role.
    /// 
    /// When you use a cross-account or private registry image, you must use SERVICE_ROLE       credentials. When you use an AWS CodeBuild curated image, you must use CODEBUILD credentials.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CODEBUILD | SERVICE_ROLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImagePullCredentialsType")]
    pub image_pull_credentials_type: Option<String>,


    /// 
    /// RegistryCredential is a property of the AWS::CodeBuild::Project Environment      property that specifies information about credentials that provide access to a private Docker registry. When this is set:
    /// 
    /// imagePullCredentialsType must be set to SERVICE_ROLE.                          images cannot be curated or an Amazon ECR image.
    /// 
    /// Required: No
    ///
    /// Type: RegistryCredential
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistryCredential")]
    pub registry_credential: Option<RegistryCredential>,


    /// 
    /// The image tag or image digest that identifies the Docker image to use for this build       project. Use the following formats:
    /// 
    /// For an image tag: <registry>/<repository>:<tag>. For           example, in the Docker repository that CodeBuild uses to manage its Docker           images, this would be aws/codebuild/standard:4.0.               For an image digest: <registry>/<repository>@<digest>.           For example, to specify an image with the digest           "sha256:cbbf2f9a99b47fc460d422812b6a5adff7dfee951d8fa2e4a98caa0382cfbdbf," use             <registry>/<repository>@sha256:cbbf2f9a99b47fc460d422812b6a5adff7dfee951d8fa2e4a98caa0382cfbdbf.
    /// 
    /// For more information, see Docker images provided by CodeBuild in the         AWS CodeBuild user         guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Image")]
    pub image: String,


    /// 
    /// The ARN of the Amazon S3 bucket, path prefix, and object key that contains the PEM-encoded       certificate for the build project. For more information, see certificate in the                 AWS CodeBuild User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,

}


/// Contains information that defines how the AWS CodeBuild build project reports the build status       to the source provider.
#[derive(Default, serde::Serialize)]
pub struct BuildStatusConfig {


    /// 
    /// Specifies the target url of the build status CodeBuild sends to the source provider. The       usage of this parameter depends on the source provider.
    /// 
    /// Bitbucket                  This parameter is used for the url parameter in the Bitbucket             commit status. For more information, see build in the Bitbucket API documentation.                       GitHub/GitHub Enterprise Server                  This parameter is used for the target_url parameter in the             GitHub commit status. For more information, see Create a commit status in the GitHub developer guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetUrl")]
    pub target_url: Option<String>,


    /// 
    /// Specifies the context of the build status CodeBuild sends to the source provider. The       usage of this parameter depends on the source provider.
    /// 
    /// Bitbucket                  This parameter is used for the name parameter in the             Bitbucket commit status. For more information, see build in the Bitbucket API documentation.                       GitHub/GitHub Enterprise Server                  This parameter is used for the context parameter in the             GitHub commit status. For more information, see Create a commit status in the GitHub developer guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Context")]
    pub context: Option<String>,

}


/// CloudWatchLogs is a property of the      AWS CodeBuild Project LogsConfig      property type that specifies settings for CloudWatch logs generated by an AWS CodeBuild build.
#[derive(Default, serde::Serialize)]
pub struct CloudWatchLogsConfig {


    /// 
    /// The prefix of the stream name of the CloudWatch Logs. For more information, see Working         with Log Groups and Log Streams.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,


    /// 
    /// The current status of the logs in CloudWatch Logs for a build project. Valid values are:
    /// 
    /// ENABLED: CloudWatch Logs are enabled for this build project.                        DISABLED: CloudWatch Logs are not enabled for this build project.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: String,


    /// 
    /// The group name of the logs in CloudWatch Logs. For more information, see Working         with Log Groups and Log Streams.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

}


/// GitSubmodulesConfig is a property of the AWS CodeBuild Project Source       property type that specifies information about the Git submodules configuration for the build project.
#[derive(Default, serde::Serialize)]
pub struct GitSubmodulesConfig {


    /// 
    /// Set to true to fetch Git submodules for your AWS CodeBuild build project.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FetchSubmodules")]
    pub fetch_submodules: bool,

}


/// Specifies restrictions for the batch build.
#[derive(Default, serde::Serialize)]
pub struct BatchRestrictions {


    /// 
    /// Specifies the maximum number of builds allowed.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBuildsAllowed")]
    pub maximum_builds_allowed: Option<i64>,


    /// 
    /// An array of strings that specify the compute types that are allowed for the batch       build. See Build environment         compute types in the         AWS CodeBuild User Guide for these values.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeTypesAllowed")]
    pub compute_types_allowed: Option<Vec<String>>,

}


/// ProjectCache is a property of the       AWS CodeBuild Project      resource that specifies information about the cache for the build project. If ProjectCache is not specified, then both of its properties      default to NO_CACHE.
#[derive(Default, serde::Serialize)]
pub struct ProjectCache {


    /// 
    /// An array of strings that specify the local cache modes. You can use one or more local       cache modes at the same time. This is only used for LOCAL cache       types.
    /// 
    /// Possible values are:
    /// 
    /// LOCAL_SOURCE_CACHE                  Caches Git metadata for primary and secondary sources. After the cache is             created, subsequent builds pull only the change between commits. This mode             is a good choice for projects with a clean working directory and a source             that is a large Git repository. If you choose this option and your project             does not use a Git repository (GitHub, GitHub Enterprise, or Bitbucket), the             option is ignored.                        LOCAL_DOCKER_LAYER_CACHE                  Caches existing Docker layers. This mode is a good choice for projects             that build or pull large Docker images. It can prevent the performance             issues caused by pulling large Docker images down from the network.          Note                                                                You can use a Docker layer cache in the Linux environment                   only.                           The privileged flag must be set so that your                   project has the required Docker permissions.                           You should consider the security implications before you use a                   Docker layer cache.                                    LOCAL_CUSTOM_CACHE                  Caches directories you specify in the buildspec file. This mode is a good             choice if your build scenario is not suited to one of the other three local             cache modes. If you use a custom cache:                                                                   Only directories can be specified for caching. You cannot specify                 individual files.                        Symlinks are used to reference cached directories.                        Cached directories are linked to your build before it downloads                 its project sources. Cached items are overridden if a source item                 has the same name. Directories are specified using cache paths in                 the buildspec file.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Modes")]
    pub modes: Option<Vec<String>>,


    /// 
    /// Information about the cache location:
    /// 
    /// NO_CACHE or LOCAL: This value is ignored.                        S3: This is the S3 bucket name/prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<String>,


    /// 
    /// The type of cache used by the build project. Valid values include:
    /// 
    /// NO_CACHE: The build project does not use any cache.                        S3: The build project reads and writes from and to S3.                        LOCAL: The build project stores a cache locally on a build host           that is only available to that build host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: LOCAL | NO_CACHE | S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// Artifacts is a property of the       AWS::CodeBuild::Project resource that specifies output settings for     artifacts generated by an AWS CodeBuild build.
#[derive(Default, serde::Serialize)]
pub struct Artifacts {


    /// 
    /// Information about the build output artifact location:
    /// 
    /// If type is set to CODEPIPELINE, AWS CodePipeline ignores this value        if specified. This is because CodePipeline manages its build output locations instead of        CodeBuild.               If type is set to NO_ARTIFACTS, this value is ignored if        specified, because no build output is produced.               If type is set to S3, this is the name of the output        bucket.
    /// 
    /// If you specify CODEPIPELINE or NO_ARTIFACTS for the Type      property, don't specify this property. For all of the other types, you must specify this property.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<String>,


    /// 
    /// The type of build output artifact. Valid values include:
    /// 
    /// CODEPIPELINE: The build project has build output generated           through CodePipeline.         NoteThe CODEPIPELINE type is not supported for               secondaryArtifacts.                        NO_ARTIFACTS: The build project does not produce any build           output.                        S3: The build project stores build output in Amazon S3.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CODEPIPELINE | NO_ARTIFACTS | S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// An identifier for this artifact definition.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactIdentifier")]
    pub artifact_identifier: Option<String>,


    /// 
    /// The type of build output artifact to create:
    /// 
    /// If type is set to CODEPIPELINE, CodePipeline ignores this           value if specified. This is because CodePipeline manages its build output artifacts           instead of AWS CodeBuild.               If type is set to NO_ARTIFACTS, this value is           ignored if specified, because no build output is produced.               If type is set to S3, valid values include:                                                            NONE: AWS CodeBuild creates in the output bucket a folder that               contains the build output. This is the default if packaging               is not specified.                                 ZIP: AWS CodeBuild creates in the output bucket a ZIP file that               contains the build output.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | ZIP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Packaging")]
    pub packaging: Option<String>,


    /// 
    /// Set to true if you do not want your output artifacts encrypted. This option is valid     only if your artifacts type is Amazon Simple Storage Service (Amazon S3). If this is set with another artifacts type, an       invalidInputException is thrown.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionDisabled")]
    pub encryption_disabled: Option<bool>,


    /// 
    /// Along with path and name, the pattern that AWS CodeBuild uses to       determine the name and location to store the output artifact:
    /// 
    /// If type is set to CODEPIPELINE, CodePipeline ignores this           value if specified. This is because CodePipeline manages its build output names instead           of AWS CodeBuild.               If type is set to NO_ARTIFACTS, this value is           ignored if specified, because no build output is produced.               If type is set to S3, valid values include:                                                            BUILD_ID: Include the build ID in the location of the               build output artifact.                                 NONE: Do not include the build ID. This is the default if                 namespaceType is not specified.
    /// 
    /// For example, if path is set to MyArtifacts,         namespaceType is set to BUILD_ID, and name is       set to MyArtifact.zip, the output artifact is stored in         MyArtifacts/<build-ID>/MyArtifact.zip.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BUILD_ID | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceType")]
    pub namespace_type: Option<String>,


    /// 
    /// Along with path and namespaceType, the pattern that AWS CodeBuild uses     to name and store the output artifact:
    /// 
    /// If type is set to CODEPIPELINE, AWS CodePipeline ignores this value        if specified. This is because CodePipeline manages its build output names instead of        AWS CodeBuild.               If type is set to NO_ARTIFACTS, this value is ignored if        specified, because no build output is produced.               If type is set to S3, this is the name of the output        artifact object. If you set the name to be a forward slash ("/"), the artifact is        stored in the root of the output bucket.
    /// 
    /// For example:
    /// 
    /// If path is set to MyArtifacts,          namespaceType is set to BUILD_ID, and name        is set to MyArtifact.zip, then the output artifact is stored in           MyArtifacts/build-ID/MyArtifact.zip.                If path is empty, namespaceType is set to          NONE, and name is set to "/", the output        artifact is stored in the root of the output bucket.                If path is set to MyArtifacts,          namespaceType is set to BUILD_ID, and name        is set to "/", the output artifact is stored in           MyArtifacts/build-ID          .
    /// 
    /// If you specify CODEPIPELINE or NO_ARTIFACTS for the Type      property, don't specify this property. For all of the other types, you must specify this property.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// If set to true a name specified in the buildspec file overrides the artifact name.      The name specified in a buildspec file is calculated at build time and uses the Shell command language.      For example, you can append a date and time to your artifact name so that it is always unique.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "OverrideArtifactName")]
    pub override_artifact_name: Option<bool>,


    /// 
    /// Along with namespaceType and name, the pattern that AWS CodeBuild       uses to name and store the output artifact:
    /// 
    /// If type is set to CODEPIPELINE, CodePipeline ignores this           value if specified. This is because CodePipeline manages its build output names instead           of AWS CodeBuild.               If type is set to NO_ARTIFACTS, this value is           ignored if specified, because no build output is produced.               If type is set to S3, this is the path to the output           artifact. If path is not specified, path is not           used.
    /// 
    /// For example, if path is set to MyArtifacts,         namespaceType is set to NONE, and name is set       to MyArtifact.zip, the output artifact is stored in the output bucket at         MyArtifacts/MyArtifact.zip.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,

}


/// Information about a file system created by Amazon Elastic File System (EFS). For more       information, see What Is         Amazon Elastic File System?
#[derive(Default, serde::Serialize)]
pub struct ProjectFileSystemLocation {


    /// 
    /// A string that specifies the location of the file system created by Amazon EFS. Its       format is efs-dns-name:/directory-path. You can find the DNS name of file       system when you view it in the Amazon EFS console. The directory path is a path to a       directory in the file system that CodeBuild mounts. For example, if the DNS name of a       file system is fs-abcd1234.efs.us-west-2.amazonaws.com, and its mount       directory is my-efs-mount-directory, then the location is         fs-abcd1234.efs.us-west-2.amazonaws.com:/my-efs-mount-directory.
    /// 
    /// The directory path in the format efs-dns-name:/directory-path is       optional. If you do not specify a directory path, the location is only the DNS name and       CodeBuild mounts the entire file system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: String,


    /// 
    /// The location in the container where you mount the file system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountPoint")]
    pub mount_point: String,


    /// 
    /// The type of the file system. The one supported type is EFS.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EFS
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The mount options for a file system created by Amazon EFS. The default mount options       used by CodeBuild are         nfsvers=4.1,rsize=1048576,wsize=1048576,hard,timeo=600,retrans=2. For       more information, see Recommended NFS Mount         Options.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountOptions")]
    pub mount_options: Option<String>,


    /// 
    /// The name used to access a file system created by Amazon EFS. CodeBuild creates an       environment variable by appending the identifier in all capital letters to         CODEBUILD_. For example, if you specify my_efs for         identifier, a new environment variable is create named         CODEBUILD_MY_EFS.
    /// 
    /// The identifier is used to mount your file system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Identifier")]
    pub identifier: String,

}
