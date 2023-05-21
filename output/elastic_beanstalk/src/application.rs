

/// The AWS::ElasticBeanstalk::Application resource is an AWS Elastic Beanstalk Beanstalk resource    type that specifies an Elastic Beanstalk application.
#[derive(Default, serde::Serialize)]
pub struct CfnApplication {


    /// 
    /// Your description of the application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A name for the Elastic Beanstalk application. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the application name. For    more information, see Name Type.
    /// 
    /// ImportantIf you specify a name, you cannot perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you must replace     the resource, specify a new name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,


    /// 
    /// Specifies an application resource lifecycle configuration to prevent your application    from accumulating too many versions.
    /// 
    /// Required: No
    ///
    /// Type: ApplicationResourceLifecycleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceLifecycleConfig")]
    pub resource_lifecycle_config: Option<ApplicationResourceLifecycleConfig>,

}


/// The resource lifecycle configuration for an application. Defines lifecycle settings for    resources that belong to the application, and the service role that Elastic Beanstalk assumes    in order to apply lifecycle settings. The version lifecycle configuration defines lifecycle    settings for application versions.
///
/// ApplicationResourceLifecycleConfig is a property of the AWS::ElasticBeanstalk::Application resource.
#[derive(Default, serde::Serialize)]
pub struct ApplicationResourceLifecycleConfig {


    /// 
    /// The ARN of an IAM service role that Elastic Beanstalk has permission to    assume.
    /// 
    /// The ServiceRole property is required the first time that you provide a ResourceLifecycleConfig for the application.    After you provide it once, Elastic Beanstalk persists the Service Role with the application, and you don't need to specify it again.    You can, however, specify it in subsequent updates to change the Service Role to another value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRole")]
    pub service_role: Option<String>,


    /// 
    /// Defines lifecycle settings for application versions.
    /// 
    /// Required: No
    ///
    /// Type: ApplicationVersionLifecycleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionLifecycleConfig")]
    pub version_lifecycle_config: Option<ApplicationVersionLifecycleConfig>,

}


/// The application version lifecycle settings for an application. Defines the rules that    Elastic Beanstalk applies to an application's versions in order to avoid hitting the    per-region limit for application versions.
///
/// When Elastic Beanstalk deletes an application version from its database, you can no    longer deploy that version to an environment. The source bundle remains in S3 unless you    configure the rule to delete it.
///
/// ApplicationVersionLifecycleConfig is a property of the ApplicationResourceLifecycleConfig    property type.
#[derive(Default, serde::Serialize)]
pub struct ApplicationVersionLifecycleConfig {


    /// 
    /// Specify a max count rule to restrict the number of application versions that are    retained for an application.
    /// 
    /// Required: No
    ///
    /// Type: MaxCountRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCountRule")]
    pub max_count_rule: Option<MaxCountRule>,


    /// 
    /// Specify a max age rule to restrict the length of time that application versions are    retained for an application.
    /// 
    /// Required: No
    ///
    /// Type: MaxAgeRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxAgeRule")]
    pub max_age_rule: Option<MaxAgeRule>,

}


/// A lifecycle rule that deletes application versions after the specified number of    days.
///
/// MaxAgeRule is a property of the ApplicationVersionLifecycleConfig    property type.
#[derive(Default, serde::Serialize)]
pub struct MaxAgeRule {


    /// 
    /// Specify true to apply the rule, or false to disable    it.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// Set to true to delete a version's source bundle from Amazon S3 when    Elastic Beanstalk deletes the application version.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteSourceFromS3")]
    pub delete_source_from_s3: Option<bool>,


    /// 
    /// Specify the number of days to retain an application versions.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxAgeInDays")]
    pub max_age_in_days: Option<i64>,

}


/// A lifecycle rule that deletes the oldest application version when the maximum count is    exceeded.
///
/// MaxCountRule is a property of the ApplicationVersionLifecycleConfig    property type.
#[derive(Default, serde::Serialize)]
pub struct MaxCountRule {


    /// 
    /// Set to true to delete a version's source bundle from Amazon S3 when    Elastic Beanstalk deletes the application version.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteSourceFromS3")]
    pub delete_source_from_s3: Option<bool>,


    /// 
    /// Specify true to apply the rule, or false to disable    it.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// Specify the maximum number of application versions to retain.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCount")]
    pub max_count: Option<i64>,

}
