

/// Creates a storage virtual machine (SVM) for an Amazon FSx for ONTAP file system.
#[derive(Default, serde::Serialize)]
pub struct CfnStorageVirtualMachine {


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
    pub name: String,


    /// 
    /// Describes the Microsoft Active Directory configuration to which the SVM is joined, if applicable.
    /// 
    /// Required: No
    ///
    /// Type: ActiveDirectoryConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActiveDirectoryConfiguration")]
    pub active_directory_configuration: Option<ActiveDirectoryConfiguration>,


    /// 
    /// Specifies the password to use when logging on to the SVM using a secure shell (SSH) connection to the SVM's management endpoint.       Doing so enables you to manage the SVM using the NetApp ONTAP CLI or REST API. If you do not specify a password, you can still       use the file system's fsxadmin user to manage the SVM. For more information, see                Managing SVMs using the NetApp ONTAP CLI in the FSx for ONTAP User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SvmAdminPassword")]
    pub svm_admin_password: Option<String>,


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
    pub root_volume_security_style: Option<String>,


    /// 
    /// Specifies the FSx for ONTAP file system on which to create the SVM.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,


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
    pub tags: Option<Vec<Tag>>,

}


/// The configuration that Amazon FSx uses to join a FSx for Windows File Server file system or an ONTAP storage virtual machine (SVM) to       a self-managed (including on-premises) Microsoft Active Directory (AD)       directory. For more information, see                Using Amazon FSx with your self-managed Microsoft Active Directory or       Managing SVMs.
#[derive(Default, serde::Serialize)]
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
    pub dns_ips: Option<Vec<String>>,


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
    pub password: Option<String>,


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
    pub domain_name: Option<String>,


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
    pub file_system_administrators_group: Option<String>,


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
    pub user_name: Option<String>,


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
    pub organizational_unit_distinguished_name: Option<String>,

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


/// Describes the self-managed Microsoft Active Directory to which you want to join the SVM.    Joining an Active Directory provides user authentication and access control for SMB clients,    including Microsoft Windows and macOS client accessing the file system.
#[derive(Default, serde::Serialize)]
pub struct ActiveDirectoryConfiguration {


    /// 
    /// The configuration that Amazon FSx uses to join the ONTAP storage virtual machine       (SVM) to your self-managed (including on-premises) Microsoft Active Directory (AD) directory.
    /// 
    /// Required: No
    ///
    /// Type: SelfManagedActiveDirectoryConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    pub self_managed_active_directory_configuration: Option<SelfManagedActiveDirectoryConfiguration>,


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
    pub net_bios_name: Option<String>,

}
