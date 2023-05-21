/// Creates an app for a specified stack. For more information, see Creating    Apps.
///
/// Required Permissions: To use this action, an IAM user must have a Manage permissions    level for the stack, or an attached policy that explicitly grants permissions. For more    information on user permissions, see Managing User     Permissions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApp {
    ///
    /// A Source object that specifies the app repository.
    ///
    /// Required: No
    ///
    /// Type: Source
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppSource")]
    pub app_source: Option<Source>,

    ///
    /// One or more user-defined key/value pairs to be added to the stack attributes.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Option<std::collections::HashMap<String, String>>,

    ///
    /// The app's data source.
    ///
    /// Required: No
    ///
    /// Type: List of DataSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSources")]
    pub data_sources: Option<Vec<DataSource>>,

    ///
    /// A description of the app.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The app virtual host settings, with multiple domains separated by commas. For example:     'www.example.com, example.com'
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domains")]
    pub domains: Option<Vec<String>>,

    ///
    /// Whether to enable SSL for the app.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableSsl")]
    pub enable_ssl: Option<bool>,

    ///
    /// An array of EnvironmentVariable objects that specify environment variables to be    associated with the app. After you deploy the app, these variables are defined on the    associated app server instance. For more information, see Environment Variables.
    ///
    /// There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variables' names, values, and protected flag values - cannot exceed 20 KB. This limit should accommodate most if not all use cases. Exceeding it will cause an exception with the message, "Environment: is too large (maximum is 20KB)."
    ///
    /// NoteIf you have specified one or more environment variables, you cannot modify the stack's Chef version.
    ///
    /// Required: No
    ///
    /// Type: List of EnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<EnvironmentVariable>>,

    ///
    /// The app name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The app's short name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Shortname")]
    pub shortname: Option<String>,

    ///
    /// An SslConfiguration object with the SSL configuration.
    ///
    /// Required: No
    ///
    /// Type: SslConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslConfiguration")]
    pub ssl_configuration: Option<SslConfiguration>,

    ///
    /// The stack ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackId")]
    pub stack_id: String,

    ///
    /// The app type. Each supported type is associated with a particular layer. For example, PHP      applications are associated with a PHP layer. AWS OpsWorks Stacks deploys an application to those instances    that are members of the corresponding layer. If your app isn't one of the standard types, or    you prefer to implement your own Deploy recipes, specify other.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: aws-flow-ruby | java | nodejs | other | php | rails | static
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: AppTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AppTypeEnum {
    /// aws-flow-ruby
    #[serde(rename = "aws-flow-ruby")]
    Awsflowruby,

    /// java
    #[serde(rename = "java")]
    Java,

    /// nodejs
    #[serde(rename = "nodejs")]
    Nodejs,

    /// other
    #[serde(rename = "other")]
    Other,

    /// php
    #[serde(rename = "php")]
    Php,

    /// rails
    #[serde(rename = "rails")]
    Rails,

    /// static
    #[serde(rename = "static")]
    Static,
}

impl Default for AppTypeEnum {
    fn default() -> Self {
        AppTypeEnum::Awsflowruby
    }
}

impl cfn_resources::CfnResource for CfnApp {
    fn type_string(&self) -> &'static str {
        "AWS::OpsWorks::App"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.app_source
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ssl_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an app's data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSource {
    ///
    /// The data source's ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

    ///
    /// The database name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,

    ///
    /// The data source's type, AutoSelectOpsworksMysqlInstance,     OpsworksMysqlInstance, RdsDbInstance, or None.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,
}

impl cfn_resources::CfnResource for DataSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Represents an app's environment variable.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EnvironmentVariable {
    ///
    /// (Required) The environment variable's name, which can consist of up to 64 characters and must be specified.      The name can contain upper- and lowercase letters, numbers, and underscores (_), but it must start with a letter or underscore.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// (Optional) Whether the variable's value is returned by the DescribeApps action.    To hide an environment variable's value, set Secure to true.     DescribeApps returns *****FILTERED***** instead of the actual    value. The default value for Secure is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Secure")]
    pub secure: Option<bool>,

    ///
    /// (Optional) The environment variable's value, which can be left empty. If you specify a value,      it can contain up to 256 characters, which must all be printable.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for EnvironmentVariable {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Contains the information required to retrieve an app or cookbook from a repository. For more    information, see Creating Apps or Custom Recipes and     Cookbooks.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Source {
    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-pw
    #[serde(rename = "Password")]
    pub password: Option<String>,

    ///
    /// The application's version. AWS OpsWorks Stacks enables you to easily deploy new versions of an application.      One of the simplest approaches is to have branches or revisions in your repository that represent different      versions that can potentially be deployed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Revision")]
    pub revision: Option<String>,

    ///
    /// The repository's SSH key. For more information, see      Using Git Repository SSH Keys      in the AWS OpsWorks User Guide.     To pass in an SSH key as a parameter, see the following example:
    ///
    /// "Parameters" : { "GitSSHKey" : { "Description" : "Change SSH key newlines to       commas.", "Type" : "CommaDelimitedList", "NoEcho" : "true" }, ...       "CustomCookbooksSource": { "Revision" : { "Ref": "GitRevision"}, "SshKey" : { "Fn::Join"       : [ "\n", { "Ref": "GitSSHKey"} ] }, "Type": "git", "Url": { "Ref": "GitURL"} }       ...
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SshKey")]
    pub ssh_key: Option<String>,

    ///
    /// The repository type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: archive | git | s3 | svn
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<SourceTypeEnum>,

    ///
    /// The source URL. The following is an example of an Amazon S3 source      URL: https://s3.amazonaws.com/opsworks-demo-bucket/opsworks_cookbook_demo.tar.gz.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: Option<String>,

    ///
    /// This parameter depends on the repository type.
    ///
    /// For Amazon S3 bundles, set Username to the appropriate IAM access key     ID.               For HTTP bundles, Git repositories, and Subversion repositories, set Username     to the user name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SourceTypeEnum {
    /// archive
    #[serde(rename = "archive")]
    Archive,

    /// git
    #[serde(rename = "git")]
    Git,

    /// s3
    #[serde(rename = "s3")]
    S3,

    /// svn
    #[serde(rename = "svn")]
    Svn,
}

impl Default for SourceTypeEnum {
    fn default() -> Self {
        SourceTypeEnum::Archive
    }
}

impl cfn_resources::CfnResource for Source {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Describes an app's SSL configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SslConfiguration {
    ///
    /// The contents of the certificate's domain.crt file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,

    ///
    /// Optional. Can be used to specify an intermediate certificate authority key or client authentication.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Chain")]
    pub chain: Option<String>,

    ///
    /// The private key; the contents of the certificate's domain.kex file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateKey")]
    pub private_key: Option<String>,
}

impl cfn_resources::CfnResource for SslConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
