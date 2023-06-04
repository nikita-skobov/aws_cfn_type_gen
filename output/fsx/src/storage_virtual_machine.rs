/// Creates a storage virtual machine (SVM) for an Amazon FSx for ONTAP file system.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageVirtualMachine {
    ///
    /// Describes the Microsoft Active Directory configuration to which the SVM is joined, if applicable.
    ///
    /// Required: No
    ///
    /// Type: ActiveDirectoryConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActiveDirectoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_configuration: Option<ActiveDirectoryConfiguration>,

    ///
    /// Specifies the FSx for ONTAP file system on which to create the SVM.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemId")]
    pub file_system_id: cfn_resources::StrVal,

    ///
    /// The name of the SVM.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 47
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,47}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The security style of the root volume of the SVM. Specify one of the following values:
    ///
    /// UNIX if the file system is managed by a UNIX         administrator, the majority of users are NFS clients, and an application         accessing the data uses a UNIX user as the service account.                        NTFS if the file system is managed by a Windows         administrator, the majority of users are SMB clients, and an application         accessing the data uses a Windows user as the service account.                        MIXED if the file system is managed by both UNIX         and Windows administrators and users consist of both NFS and SMB clients.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MIXED | NTFS | UNIX
    ///
    /// Update requires: Replacement
    #[serde(rename = "RootVolumeSecurityStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_security_style: Option<StorageVirtualMachineRootVolumeSecurityStyleEnum>,

    ///
    /// Specifies the password to use when logging on to the SVM using a secure shell (SSH) connection to the SVM's management endpoint.       Doing so enables you to manage the SVM using the NetApp ONTAP CLI or REST API. If you do not specify a password, you can still       use the file system's fsxadmin user to manage the SVM. For more information, see                Managing SVMs using the NetApp ONTAP CLI in the FSx for ONTAP User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SvmAdminPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svm_admin_password: Option<cfn_resources::StrVal>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
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
    pub att_resource_arn: CfnStorageVirtualMachineresourcearn,

    #[serde(skip_serializing)]
    pub att_storage_virtual_machine_id: CfnStorageVirtualMachinestoragevirtualmachineid,

    #[serde(skip_serializing)]
    pub att_uuid: CfnStorageVirtualMachineuuid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StorageVirtualMachineRootVolumeSecurityStyleEnum {
    /// MIXED
    #[serde(rename = "MIXED")]
    Mixed,

    /// NTFS
    #[serde(rename = "NTFS")]
    Ntfs,

    /// UNIX
    #[serde(rename = "UNIX")]
    Unix,
}

impl Default for StorageVirtualMachineRootVolumeSecurityStyleEnum {
    fn default() -> Self {
        StorageVirtualMachineRootVolumeSecurityStyleEnum::Mixed
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageVirtualMachineresourcearn;
impl CfnStorageVirtualMachineresourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceARN"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageVirtualMachinestoragevirtualmachineid;
impl CfnStorageVirtualMachinestoragevirtualmachineid {
    pub fn att_name(&self) -> &'static str {
        r#"StorageVirtualMachineId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnStorageVirtualMachineuuid;
impl CfnStorageVirtualMachineuuid {
    pub fn att_name(&self) -> &'static str {
        r#"UUID"#
    }
}

impl cfn_resources::CfnResource for CfnStorageVirtualMachine {
    fn type_string(&self) -> &'static str {
        "AWS::FSx::StorageVirtualMachine"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.active_directory_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 47 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 47",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Describes the self-managed Microsoft Active Directory to which you want to join the SVM.    Joining an Active Directory provides user authentication and access control for SMB clients,    including Microsoft Windows and macOS client accessing the file system.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ActiveDirectoryConfiguration {
    ///
    /// The NetBIOS name of the Active Directory computer object that will be created for your SVM.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,255}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetBiosName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_bios_name: Option<cfn_resources::StrVal>,

    ///
    /// The configuration that Amazon FSx uses to join the ONTAP storage virtual machine       (SVM) to your self-managed (including on-premises) Microsoft Active Directory (AD) directory.
    ///
    /// Required: No
    ///
    /// Type: SelfManagedActiveDirectoryConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfiguration>,
}

impl cfn_resources::CfnResource for ActiveDirectoryConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.net_bios_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 15 as _ {
                    return Err(format!(
                        "Max validation failed on field 'net_bios_name'. {} is greater than 15",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.net_bios_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'net_bios_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.self_managed_active_directory_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration that Amazon FSx uses to join a FSx for Windows File Server file system or an ONTAP storage virtual machine (SVM) to       a self-managed (including on-premises) Microsoft Active Directory (AD)       directory. For more information, see                Using Amazon FSx with your self-managed Microsoft Active Directory or       Managing SVMs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SelfManagedActiveDirectoryConfiguration {
    ///
    /// A list of up to three IP addresses of DNS servers or domain controllers in the       self-managed AD directory.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,

    ///
    /// The fully qualified domain name of the self-managed AD directory, such as         corp.example.com.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,255}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<cfn_resources::StrVal>,

    ///
    /// (Optional) The name of the domain group whose members are granted administrative       privileges for the file system. Administrative privileges include taking ownership of       files and folders, setting audit controls (audit ACLs) on files and folders, and               administering the file system remotely by using the FSx Remote PowerShell.       The group that you specify must already exist in your domain. If you don't provide one,       your AD domain's Domain Admins group is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,256}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemAdministratorsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_administrators_group: Option<cfn_resources::StrVal>,

    ///
    /// (Optional) The fully qualified distinguished name of the organizational unit within       your self-managed AD directory. Amazon       FSx only accepts OU as the direct parent of the file system. An example is         OU=FSx,DC=yourdomain,DC=corp,DC=com. To learn more, see RFC 2253. If none is provided, the       FSx file system is created in the default location of your self-managed AD directory.
    ///
    /// ImportantOnly Organizational Unit (OU) objects can be the direct parent of the file system         that you're creating.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2000
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,2000}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<cfn_resources::StrVal>,

    ///
    /// The password for the service account on your self-managed AD domain that Amazon FSx       will use to join to your AD domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^.{1,256}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<cfn_resources::StrVal>,

    ///
    /// The user name for the service account on your self-managed AD domain that Amazon FSx       will use to join to your AD domain. This account must have the permission to join       computers to the domain in the organizational unit provided in         OrganizationalUnitDistinguishedName, or in the default location of your       AD domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,256}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SelfManagedActiveDirectoryConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.dns_ips {
            if the_val.len() > 3 as _ {
                return Err(format!(
                    "Max validation failed on field 'dns_ips'. {} is greater than 3",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.domain_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'domain_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.domain_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'domain_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.file_system_administrators_group {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'file_system_administrators_group'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.file_system_administrators_group {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'file_system_administrators_group'. {} is less than 1", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.organizational_unit_distinguished_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2000 as _ {
                    return Err(format!("Max validation failed on field 'organizational_unit_distinguished_name'. {} is greater than 2000", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.organizational_unit_distinguished_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'organizational_unit_distinguished_name'. {} is less than 1", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.password {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'password'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.password {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'password'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.user_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'user_name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.user_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'user_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

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
