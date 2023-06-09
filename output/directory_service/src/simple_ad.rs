/// The AWS::DirectoryService::SimpleAD resource specifies an AWS Directory Service Simple Active Directory (Simple AD) in AWS so that your directory users and groups can    access the AWS Management Console and AWS applications using their existing credentials.    Simple AD is a Microsoft Active Directory–compatible directory. For more information, see     Simple Active     Directory in the AWS Directory Service Admin Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSimpleAD {
    ///
    /// If set to true, specifies an alias for a directory and assigns the alias to    the directory. The alias is used to construct the access URL for the directory, such as     http://<alias>.awsapps.com. By default, this property is set to     false.
    ///
    /// ImportantAfter an alias has been created, it cannot be deleted or reused, so this operation     should only be used when absolutely necessary.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "CreateAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_alias: Option<bool>,

    ///
    /// A description for the directory.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^([a-zA-Z0-9_])[\\a-zA-Z0-9_@#%*+=:?./!\s-]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Whether to enable single sign-on for a directory. If you don't specify a value, AWS CloudFormation disables single sign-on by default.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableSso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sso: Option<bool>,

    ///
    /// The fully qualified name for the directory, such as corp.example.com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^([a-zA-Z0-9]+[\\.-])+([a-zA-Z0-9])+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The password for the directory administrator. The directory creation process creates a    directory administrator account with the user name Administrator and this    password.
    ///
    /// If you need to change the password for the administrator account, see the ResetUserPassword API call in the AWS Directory Service API     Reference.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: (?=^.{8,64}$)((?=.*\d)(?=.*[A-Z])(?=.*[a-z])|(?=.*\d)(?=.*[^A-Za-z0-9\s])(?=.*[a-z])|(?=.*[^A-Za-z0-9\s])(?=.*[A-Z])(?=.*[a-z])|(?=.*\d)(?=.*[A-Z])(?=.*[^A-Za-z0-9\s]))^.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Password")]
    pub password: cfn_resources::StrVal,

    ///
    /// The NetBIOS name of the directory, such as CORP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[^\\/:*?"<>|.]+[^\\/:*?"<>|]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<cfn_resources::StrVal>,

    ///
    /// The size of the directory. For valid values, see CreateDirectory in    the AWS Directory Service API Reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Large | Small
    ///
    /// Update requires: Replacement
    #[serde(rename = "Size")]
    pub size: SimpleADSizeEnum,

    ///
    /// A DirectoryVpcSettings object that contains additional information for the    operation.
    ///
    /// Required: Yes
    ///
    /// Type: VpcSettings
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcSettings")]
    pub vpc_settings: VpcSettings,

    #[serde(skip_serializing)]
    pub att_alias: CfnSimpleADalias,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SimpleADSizeEnum {
    /// Large
    #[serde(rename = "Large")]
    Large,

    /// Small
    #[serde(rename = "Small")]
    Small,
}

impl Default for SimpleADSizeEnum {
    fn default() -> Self {
        SimpleADSizeEnum::Large
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSimpleADalias;
impl CfnSimpleADalias {
    pub fn att_name(&self) -> &'static str {
        r#"Alias"#
    }
}

impl cfn_resources::CfnResource for CfnSimpleAD {
    fn type_string(&self) -> &'static str {
        "AWS::DirectoryService::SimpleAD"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.vpc_settings.validate()?;

        Ok(())
    }
}

/// Contains VPC information for the CreateDirectory or     CreateMicrosoftAD    operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VpcSettings {
    ///
    /// The identifiers of the subnets for the directory servers. The two subnets must be in    different Availability Zones. AWS Directory Service specifies a directory server and a DNS    server in each of these subnets.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
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
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for VpcSettings {
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
