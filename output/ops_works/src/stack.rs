/// Creates a new stack. For more information, see Create a New     Stack.
///
/// Required Permissions: To use this action, an IAM user must have an attached policy    that explicitly grants permissions. For more information about user permissions, see Managing User     Permissions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStack {
    ///
    /// The default AWS OpsWorks Stacks agent version. You have the following options:
    ///
    /// Auto-update - Set this parameter to LATEST. AWS OpsWorks Stacks     automatically installs new agent versions on the stack's instances as soon as     they are available.               Fixed version - Set this parameter to your preferred agent version. To update the agent version,        you must edit the stack configuration and specify a new version. AWS OpsWorks Stacks installs        that version on the stack's instances.
    ///
    /// The default setting is the most recent release of the agent. To specify an agent version,    you must use the complete version number, not the abbreviated number shown on the console.    For a list of available agent version numbers, call DescribeAgentVersions. AgentVersion cannot be set to Chef 12.2.
    ///
    /// NoteYou can also specify an agent version when you create or update an instance,      which overrides the stack's default setting.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentVersion")]
    pub agent_version: Option<String>,

    ///
    /// One or more user-defined key-value pairs to be added to the stack attributes.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Option<std::collections::HashMap<String, String>>,

    ///
    /// A ChefConfiguration object that specifies whether to enable Berkshelf and the    Berkshelf version on Chef 11.10 stacks. For more information, see Create a New Stack.
    ///
    /// Required: No
    ///
    /// Type: ChefConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChefConfiguration")]
    pub chef_configuration: Option<ChefConfiguration>,

    ///
    /// If you're cloning an AWS OpsWorks stack, a list of AWS OpsWorks application stack IDs     from the source stack to include in the cloned stack.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CloneAppIds")]
    pub clone_app_ids: Option<Vec<String>>,

    ///
    /// If you're cloning an AWS OpsWorks stack, indicates whether to clone the source     stack's permissions.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClonePermissions")]
    pub clone_permissions: Option<bool>,

    ///
    /// The configuration manager. When you create a stack we recommend that you use the configuration manager to specify the      Chef version: 12, 11.10, or 11.4 for Linux stacks, or 12.2 for Windows stacks. The default value for Linux stacks is      currently 12.
    ///
    /// Required: No
    ///
    /// Type: StackConfigurationManager
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigurationManager")]
    pub configuration_manager: Option<StackConfigurationManager>,

    ///
    /// Contains the information required to retrieve an app or cookbook from a repository. For more information,       see Adding Apps or       Cookbooks and Recipes.
    ///
    /// Required: No
    ///
    /// Type: Source
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomCookbooksSource")]
    pub custom_cookbooks_source: Option<Source>,

    ///
    /// A string that contains user-defined, custom JSON. It can be used to override the corresponding default stack configuration      attribute values or to pass data to recipes. The string should be in the following format:
    ///
    /// "{\"key1\": \"value1\", \"key2\": \"value2\",...}"
    ///
    /// For more information about custom JSON, see Use Custom JSON to     Modify the Stack Configuration Attributes.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomJson")]
    pub custom_json: Option<serde_json::Value>,

    ///
    /// The stack's default Availability Zone, which must be in the specified region. For more    information, see Regions and     Endpoints. If you also specify a value for DefaultSubnetId, the subnet must    be in the same zone. For more information, see the VpcId parameter description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultAvailabilityZone")]
    pub default_availability_zone: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of an IAM profile that is the default profile for all of the stack's EC2 instances.      For more information about IAM ARNs, see Using    Identifiers.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: String,

    ///
    /// The stack's default operating system, which is installed on every instance unless you specify a different operating      system when you create the instance. You can specify one of the following.
    ///
    /// A supported Linux operating system: An Amazon Linux version, such as Amazon Linux 2, Amazon Linux 2018.03, Amazon Linux 2017.09, Amazon Linux 2017.03, Amazon Linux 2016.09,        Amazon Linux 2016.03, Amazon Linux 2015.09, or Amazon Linux 2015.03.               A supported Ubuntu operating system, such as Ubuntu 18.04 LTS, Ubuntu 16.04 LTS, Ubuntu 14.04 LTS, or Ubuntu 12.04 LTS.                        CentOS Linux 7                                Red Hat Enterprise Linux 7                       A supported Windows operating system, such as Microsoft Windows Server 2012 R2 Base,        Microsoft Windows Server 2012 R2 with SQL Server Express,        Microsoft Windows Server 2012 R2 with SQL Server Standard, or        Microsoft Windows Server 2012 R2 with SQL Server Web.               A custom AMI: Custom. You specify the custom AMI you want to use when     you create instances. For more     information, see     Using Custom AMIs.
    ///
    /// The default option is the current Amazon Linux version.      Not all operating systems are supported with all versions of Chef. For more information about supported operating systems,      see AWS OpsWorks Stacks Operating Systems.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultOs")]
    pub default_os: Option<String>,

    ///
    /// The default root device type. This value is the default for all instances in the stack,    but you can override it when you create an instance. The default option is     instance-store. For more information, see Storage for the Root Device.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ebs | instance-store
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRootDeviceType")]
    pub default_root_device_type: Option<StackDefaultRootDeviceTypeEnum>,

    ///
    /// A default Amazon EC2 key pair name. The default value is none. If you specify a key pair name,      AWS OpsWorks installs the public key on the instance and you can use the private key with an SSH    client to log in to the instance. For more information, see Using SSH to     Communicate with an Instance and Managing SSH     Access. You can override this setting by specifying a different key pair, or no key    pair, when you     create an instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultSshKeyName")]
    pub default_ssh_key_name: Option<String>,

    ///
    /// The stack's default subnet ID. All instances are launched into this subnet unless you specify another subnet ID when you create the instance.     This parameter is required if you specify a value for the     VpcId parameter. If you also specify a value for     DefaultAvailabilityZone, the subnet must be in that zone.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultSubnetId")]
    pub default_subnet_id: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon Elastic Container Service (Amazon ECS)     cluster to register with the AWS OpsWorks stack.
    ///
    /// NoteIf you specify a cluster that's registered with another AWS OpsWorks stack, AWS CloudFormation       deregisters the existing association before registering the       cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcsClusterArn")]
    pub ecs_cluster_arn: Option<String>,

    ///
    /// A list of Elastic IP addresses to register with the AWS OpsWorks stack.
    ///
    /// NoteIf you specify an IP address that's registered with another AWS OpsWorks stack, AWS CloudFormation       deregisters the existing association before registering the IP       address.
    ///
    /// Required: No
    ///
    /// Type: List of ElasticIp
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticIps")]
    pub elastic_ips: Option<Vec<ElasticIp>>,

    ///
    /// The stack's host name theme, with spaces replaced by underscores. The theme is used to    generate host names for the stack's instances. By default, HostnameTheme is set    to Layer_Dependent, which creates host names by appending integers to the layer's    short name. The other themes are:
    ///
    /// Baked_Goods                                Clouds                                Europe_Cities                                Fruits                                Greek_Deities_and_Titans                                Legendary_creatures_from_Japan                                Planets_and_Moons                                Roman_Deities                                Scottish_Islands                                US_Cities                                Wild_Cats
    ///
    /// To obtain a generated host name, call GetHostNameSuggestion, which returns a    host name based on the current theme.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostnameTheme")]
    pub hostname_theme: Option<String>,

    ///
    /// The stack name. Stack names can be a maximum of 64 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The Amazon Relational Database Service (Amazon RDS) database instance to register with the     AWS OpsWorks stack.
    ///
    /// NoteIf you specify a database instance that's registered with another AWS OpsWorks stack, AWS CloudFormation       deregisters the existing association before registering the database       instance.
    ///
    /// Required: No
    ///
    /// Type: List of RdsDbInstance
    ///
    /// Update requires: No interruption
    #[serde(rename = "RdsDbInstances")]
    pub rds_db_instances: Option<Vec<RdsDbInstance>>,

    ///
    /// The stack's IAM role, which allows AWS OpsWorks Stacks to work with AWS    resources on your behalf. You must set this parameter to the Amazon Resource Name (ARN) for an    existing IAM role. For more information about IAM ARNs, see      Using    Identifiers.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,

    ///
    /// If you're cloning an AWS OpsWorks stack, the stack ID of the source AWS OpsWorks     stack to clone.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceStackId")]
    pub source_stack_id: Option<String>,

    ///
    /// A map that contains tag keys and tag values that are attached to a stack or layer.
    ///
    /// The key cannot be empty.               The key can be a maximum of 127 characters, and can contain only Unicode letters, numbers, or separators,        or the following special characters: + - = . _ : /                       The value can be a maximum 255 characters, and contain only Unicode letters, numbers, or separators,        or the following special characters: + - = . _ : /                       Leading and trailing white spaces are trimmed from both the key and value.               A maximum of 40 tags is allowed for any resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Whether the stack uses custom cookbooks.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseCustomCookbooks")]
    pub use_custom_cookbooks: Option<bool>,

    ///
    /// Whether to associate the AWS OpsWorks Stacks built-in security groups with the stack's layers.
    ///
    /// AWS OpsWorks Stacks provides a standard set of built-in security groups, one for each layer, which are    associated with layers by default. With UseOpsworksSecurityGroups you can instead    provide your own custom security groups. UseOpsworksSecurityGroups has the    following settings:
    ///
    /// True - AWS OpsWorks Stacks automatically associates the appropriate built-in security group with each        layer (default setting). You can associate additional security groups with a layer after you create it, but you cannot        delete the built-in security group.               False - AWS OpsWorks Stacks does not associate built-in security groups with layers. You must create        appropriate EC2 security groups and associate a security group with each layer that you create. However, you can still        manually associate a built-in security group with a layer on creation; custom security groups are required only for those        layers that need custom settings.
    ///
    /// For more information, see Create a New     Stack.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseOpsworksSecurityGroups")]
    pub use_opsworks_security_groups: Option<bool>,

    ///
    /// The ID of the VPC that the stack is to be launched into. The VPC must be in the stack's region. All instances are launched      into this VPC. You cannot change the ID later.
    ///
    /// If your account supports EC2-Classic, the default value is no VPC.               If your account does not support EC2-Classic, the default value is the default VPC for the specified region.
    ///
    /// If the VPC ID corresponds to a default VPC and you have specified either the     DefaultAvailabilityZone or the DefaultSubnetId parameter only,      AWS OpsWorks Stacks infers the value of the      other parameter. If you specify neither parameter, AWS OpsWorks Stacks sets    these parameters to the first valid Availability Zone for the specified region and the    corresponding default VPC subnet ID, respectively.
    ///
    /// If you specify a nondefault VPC ID, note the following:
    ///
    /// It must belong to a VPC in your account that is in the specified region.               You must specify a value for DefaultSubnetId.
    ///
    /// For more information about how to use AWS OpsWorks Stacks with a VPC, see Running a Stack in a     VPC. For more information about default VPC and EC2-Classic, see Supported     Platforms.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum StackDefaultRootDeviceTypeEnum {
    /// ebs
    #[serde(rename = "ebs")]
    Ebs,

    /// instance-store
    #[serde(rename = "instance-store")]
    Instancestore,
}

impl Default for StackDefaultRootDeviceTypeEnum {
    fn default() -> Self {
        StackDefaultRootDeviceTypeEnum::Ebs
    }
}

impl cfn_resources::CfnResource for CfnStack {
    fn type_string(&self) -> &'static str {
        "AWS::OpsWorks::Stack"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.chef_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.configuration_manager
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.custom_cookbooks_source
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the Chef configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ChefConfiguration {
    ///
    /// The Berkshelf version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BerkshelfVersion")]
    pub berkshelf_version: Option<String>,

    ///
    /// The Berkshelf version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManageBerkshelf")]
    pub manage_berkshelf: Option<bool>,
}

impl cfn_resources::CfnResource for ChefConfiguration {
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

/// Describes an Elastic IP address.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticIp {
    ///
    /// The IP address.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ip")]
    pub ip: String,

    ///
    /// The name, which can be a maximum of 32 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl cfn_resources::CfnResource for ElasticIp {
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

/// Describes an Amazon RDS instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RdsDbInstance {
    ///
    /// AWS OpsWorks Stacks returns *****FILTERED***** instead of the actual value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DbPassword")]
    pub db_password: String,

    ///
    /// The master user name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DbUser")]
    pub db_user: String,

    ///
    /// The instance's ARN.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RdsDbInstanceArn")]
    pub rds_db_instance_arn: String,
}

impl cfn_resources::CfnResource for RdsDbInstance {
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
    ///
    /// When included in a request, the parameter depends on the repository type.
    ///
    /// For Amazon S3 bundles, set Password to the appropriate IAM secret access     key.               For HTTP bundles and Subversion repositories, set Password to the     password.
    ///
    /// For more information on how to safely handle IAM credentials, see https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html.
    ///
    /// In responses, AWS OpsWorks Stacks returns *****FILTERED***** instead of the actual value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
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

/// Describes the configuration manager.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StackConfigurationManager {
    ///
    /// The name. This parameter must be set to Chef.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// The Chef version. This parameter must be set to 12, 11.10, or 11.4 for Linux stacks, and to 12.2 for Windows stacks.      The default value for Linux stacks is 12.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl cfn_resources::CfnResource for StackConfigurationManager {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for Tag {
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
