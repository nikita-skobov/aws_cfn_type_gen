

/// The AWS::DirectoryService::MicrosoftAD resource specifies a Microsoft Active    Directory in AWS so that your directory users and groups can access the AWS Management Console    and AWS applications using their existing credentials. For more information, see AWS Managed Microsoft AD in the AWS Directory Service Admin     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMicrosoftAD {


    /// 
    /// The password for the default administrative user named Admin.
    /// 
    /// If you need to change the password for the administrator account, see the ResetUserPassword API call in the AWS Directory Service API     Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: (?=^.{8,64}$)((?=.*\d)(?=.*[A-Z])(?=.*[a-z])|(?=.*\d)(?=.*[^A-Za-z0-9\s])(?=.*[a-z])|(?=.*[^A-Za-z0-9\s])(?=.*[A-Z])(?=.*[a-z])|(?=.*\d)(?=.*[A-Z])(?=.*[^A-Za-z0-9\s]))^.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Password")]
    pub password: String,


    /// 
    /// Whether to enable single sign-on for a Microsoft Active Directory in AWS. Single sign-on    allows users in your directory to access certain AWS services from a computer joined to the    directory without having to enter their credentials separately. If you don't specify a value,    AWS CloudFormation disables single sign-on by default.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableSso")]
    pub enable_sso: Option<bool>,


    /// 
    /// AWS Managed Microsoft AD is available in two editions: Standard and     Enterprise. Enterprise is the default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Enterprise | Standard
    ///
    /// Update requires: Replacement
    #[serde(rename = "Edition")]
    pub edition: Option<MicrosoftADEditionEnum>,


    /// 
    /// The NetBIOS name for your domain, such as CORP. If you don't specify a    NetBIOS name, it will default to the first part of your directory DNS. For example,     CORP for the directory DNS corp.example.com.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[^\\/:*?"<>|.]+[^\\/:*?"<>|]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShortName")]
    pub short_name: Option<String>,


    /// 
    /// The fully qualified domain name for the AWS Managed Microsoft AD directory, such as     corp.example.com. This name will resolve inside your VPC only. It does not need    to be publicly resolvable.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^([a-zA-Z0-9]+[\\.-])+([a-zA-Z0-9])+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Specifies the VPC settings of the Microsoft AD directory server in AWS.
    /// 
    /// Required: Yes
    ///
    /// Type: VpcSettings
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcSettings")]
    pub vpc_settings: VpcSettings,


    /// 
    /// Specifies an alias for a directory and assigns the alias to the directory. The alias is    used to construct the access URL for the directory, such as    http://<alias>.awsapps.com. By default, AWS CloudFormation does not    create an alias.
    /// 
    /// ImportantAfter an alias has been created, it cannot be deleted or reused, so this operation     should only be used when absolutely necessary.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "CreateAlias")]
    pub create_alias: Option<bool>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum MicrosoftADEditionEnum {

    /// Enterprise
    #[serde(rename = "Enterprise")]
    Enterprise,

    /// Standard
    #[serde(rename = "Standard")]
    Standard,

}

impl Default for MicrosoftADEditionEnum {
    fn default() -> Self {
        MicrosoftADEditionEnum::Enterprise
    }
}


impl cfn_resources::CfnResource for CfnMicrosoftAD {
    fn type_string() -> &'static str {
        "AWS::DirectoryService::MicrosoftAD"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains VPC information for the CreateDirectory or     CreateMicrosoftAD    operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcSettings {


    /// 
    /// The identifiers of the subnets for the directory servers. The two subnets must be in    different Availability Zones. AWS Directory Service specifies a directory server and a DNS    server in each of these subnets.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


    /// 
    /// The identifier of the VPC in which to create the directory.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^(vpc-[0-9a-f]{8}|vpc-[0-9a-f]{17})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}


