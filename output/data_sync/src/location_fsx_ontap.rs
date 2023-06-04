/// The AWS::DataSync::LocationFSxONTAP resource creates an endpoint for an Amazon FSx for NetApp ONTAP file system. AWS DataSync can access this endpoint as a source or destination location.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnLocationFSxONTAP {
    ///
    /// Specifies the data transfer protocol that DataSync uses to access your Amazon FSx file system.
    ///
    /// Required: No
    ///
    /// Type: Protocol
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub protocol: Option<Protocol>,

    ///
    /// Specifies the Amazon Resource Names (ARNs) of the security groups that DataSync can use to access your FSx for ONTAP file system. You must configure the security groups to allow outbound    traffic on the following ports (depending on the protocol that you're using):
    ///
    /// Network File System (NFS): TCP ports 111, 635, and 2049                        Server Message Block (SMB): TCP port 445
    ///
    /// Your file system's security groups must also allow inbound traffic on the same port.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,

    ///
    /// Specifies the ARN of the storage virtual machine (SVM) in your file system where you want    to copy data to or from.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 162
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):fsx:[a-z\-0-9]+:[0-9]{12}:storage-virtual-machine/fs-[0-9a-f]+/svm-[0-9a-f]{17,}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageVirtualMachineArn")]
    pub storage_virtual_machine_arn: cfn_resources::StrVal,

    ///
    /// Specifies a path to the file share in the SVM where you'll copy your data.
    ///
    /// You can specify a junction path (also known as a mount point), qtree path (for NFS file    shares), or share name (for SMB file shares). For example, your mount path might be     /vol1, /vol1/tree1, or /share1.
    ///
    /// NoteDon't specify a junction path in the SVM's root volume. For more information, see Managing FSx for ONTAP storage virtual machines in the Amazon FSx for NetApp ONTAP User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,255}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub subdirectory: Option<cfn_resources::StrVal>,

    ///
    /// Specifies labels that help you categorize, filter, and search for your AWS    resources. We recommend creating at least a name tag for your location.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_fsx_filesystem_arn: CfnLocationFSxONTAPfsxfilesystemarn,

    #[serde(skip_serializing)]
    pub att_location_arn: CfnLocationFSxONTAPlocationarn,

    #[serde(skip_serializing)]
    pub att_location_uri: CfnLocationFSxONTAPlocationuri,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocationFSxONTAPfsxfilesystemarn;
impl CfnLocationFSxONTAPfsxfilesystemarn {
    pub fn att_name(&self) -> &'static str {
        r#"FsxFilesystemArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocationFSxONTAPlocationarn;
impl CfnLocationFSxONTAPlocationarn {
    pub fn att_name(&self) -> &'static str {
        r#"LocationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocationFSxONTAPlocationuri;
impl CfnLocationFSxONTAPlocationuri {
    pub fn att_name(&self) -> &'static str {
        r#"LocationUri"#
    }
}

impl cfn_resources::CfnResource for CfnLocationFSxONTAP {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationFSxONTAP"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.protocol
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.security_group_arns;

        if the_val.len() > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'security_group_arns'. {} is greater than 5",
                the_val.len()
            ));
        }

        let the_val = &self.storage_virtual_machine_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 162 as _ {
                return Err(format!("Max validation failed on field 'storage_virtual_machine_arn'. {} is greater than 162", s.len()));
            }
        }

        if let Some(the_val) = &self.subdirectory {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'subdirectory'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies the Network File System (NFS) protocol configuration that AWS DataSync uses to access a storage virtual machine (SVM) on your Amazon FSx for NetApp ONTAP file system. For more information, see Accessing FSx for ONTAP file systems.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NFS {
    ///
    /// Specifies how DataSync can access a location using the NFS protocol.
    ///
    /// Required: Yes
    ///
    /// Type: NfsMountOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "MountOptions")]
    pub mount_options: NfsMountOptions,
}

impl cfn_resources::CfnResource for NFS {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.mount_options.validate()?;

        Ok(())
    }
}

/// Specifies how DataSync can access a location using the NFS protocol.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NfsMountOptions {
    ///
    /// Specifies the NFS version that you want DataSync to use when mounting your NFS    share. If the server refuses to use the version specified, the task fails.
    ///
    /// You can specify the following options:
    ///
    /// AUTOMATIC (default): DataSync chooses NFS version 4.1.                        NFS3: Stateless protocol version that allows for asynchronous writes on      the server.                        NFSv4_0: Stateful, firewall-friendly protocol version that supports      delegations and pseudo file systems.                        NFSv4_1: Stateful protocol version that supports sessions, directory      delegations, and parallel data processing. NFS version 4.1 also includes all features      available in version 4.0.
    ///
    /// NoteDataSync currently only supports NFS version 3 with Amazon FSx for NetApp ONTAP locations.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | NFS3 | NFS4_0 | NFS4_1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub version: Option<NfsMountOptionsVersionEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NfsMountOptionsVersionEnum {
    /// AUTOMATIC
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    /// NFS3
    #[serde(rename = "NFS3")]
    Nfs3,

    /// NFS4_0
    #[serde(rename = "NFS4_0")]
    Nfs40,

    /// NFS4_1
    #[serde(rename = "NFS4_1")]
    Nfs41,
}

impl Default for NfsMountOptionsVersionEnum {
    fn default() -> Self {
        NfsMountOptionsVersionEnum::Automatic
    }
}

impl cfn_resources::CfnResource for NfsMountOptions {
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

/// Specifies the data transfer protocol that AWS DataSync uses to access your     Amazon FSx file system.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Protocol {
    ///
    /// Specifies the Network File System (NFS) protocol configuration that DataSync uses to access your FSx for ONTAP file system's storage virtual machine (SVM).
    ///
    /// Required: No
    ///
    /// Type: NFS
    ///
    /// Update requires: Replacement
    #[serde(rename = "NFS")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub nfs: Option<NFS>,

    ///
    /// Specifies the Server Message Block (SMB) protocol configuration that DataSync uses to access your FSx for ONTAP file system's SVM.
    ///
    /// Required: No
    ///
    /// Type: SMB
    ///
    /// Update requires: Replacement
    #[serde(rename = "SMB")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub smb: Option<SMB>,
}

impl cfn_resources::CfnResource for Protocol {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.nfs.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.smb.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the Server Message Block (SMB) protocol configuration that AWS DataSync uses to access a storage virtual machine (SVM) on your Amazon FSx for NetApp ONTAP file system. For more information, see Accessing FSx for ONTAP file systems.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SMB {
    ///
    /// Specifies the fully qualified domain name (FQDN) of the Microsoft Active Directory that    your storage virtual machine (SVM) belongs to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^[A-Za-z0-9]((\.|-+)?[A-Za-z0-9]){0,252}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub domain: Option<cfn_resources::StrVal>,

    ///
    /// Specifies how DataSync can access a location using the SMB protocol.
    ///
    /// Required: Yes
    ///
    /// Type: SmbMountOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "MountOptions")]
    pub mount_options: SmbMountOptions,

    ///
    /// Specifies the password of a user who has permission to access your SVM.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 104
    ///
    /// Pattern: ^.{0,104}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Password")]
    pub password: cfn_resources::StrVal,

    ///
    /// Specifies a user name that can mount the location and access the files, folders, and metadata that you need in the SVM.
    ///
    /// If you provide a user in your Active Directory, note the following:
    ///
    /// If you're using AWS Directory Service for Microsoft Active Directory, the user        must be a member of the AWS Delegated          FSx Administrators group.            If you're using a self-managed Active        Directory, the user must be a member of either the        Domain Admins group or a        custom group that you specified for file system        administration when you created your file        system.
    ///
    /// Make sure that the user has the permissions it needs     to copy the data you want:
    ///
    /// SE_TCB_NAME: Required to        set object ownership and file metadata. With this        privilege, you also can copy NTFS discretionary        access lists (DACLs).          SE_SECURITY_NAME: May be       needed to copy NTFS system access control lists       (SACLs). This operation specifically requires the       Windows privilege, which is granted to members of       the Domain Admins group. If       you configure your task to copy SACLs, make sure       that the user has the required privileges. For       information about copying SACLs, see Ownership and permissions-related options.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 104
    ///
    /// Pattern: ^[^\x5B\x5D\\/:;|=,+*?]{1,104}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "User")]
    pub user: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SMB {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.domain {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 253 as _ {
                    return Err(format!(
                        "Max validation failed on field 'domain'. {} is greater than 253",
                        s.len()
                    ));
                }
            }
        }

        self.mount_options.validate()?;

        let the_val = &self.password;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 104 as _ {
                return Err(format!(
                    "Max validation failed on field 'password'. {} is greater than 104",
                    s.len()
                ));
            }
        }

        let the_val = &self.user;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 104 as _ {
                return Err(format!(
                    "Max validation failed on field 'user'. {} is greater than 104",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies the version of the Server Message Block (SMB) protocol that AWS DataSync uses to access an SMB file server.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SmbMountOptions {
    ///
    /// By default, DataSync automatically chooses an SMB protocol version based on    negotiation with your SMB file server. You also can configure DataSync to use a    specific SMB version, but we recommend doing this only if DataSync has trouble    negotiating with the SMB file server automatically.
    ///
    /// These are the following options for configuring the SMB version:
    ///
    /// AUTOMATIC (default): DataSync and the SMB file server negotiate      the highest version of SMB that they mutually support between 2.1 and 3.1.1.        This is the recommended option. If you instead choose a specific version that your      file server doesn't support, you may get an Operation Not Supported      error.                        SMB3: Restricts the protocol negotiation to only SMB version      3.0.2.                        SMB2: Restricts the protocol negotiation to only SMB version 2.1.                        SMB2_0: Restricts the protocol negotiation to only SMB version      2.0.                        SMB1: Restricts the protocol negotiation to only SMB version 1.0.        NoteThe SMB1 option isn't available when creating an Amazon FSx for NetApp ONTAP location.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | SMB1 | SMB2 | SMB2_0 | SMB3
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub version: Option<SmbMountOptionsVersionEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SmbMountOptionsVersionEnum {
    /// AUTOMATIC
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    /// SMB1
    #[serde(rename = "SMB1")]
    Smb1,

    /// SMB2
    #[serde(rename = "SMB2")]
    Smb2,

    /// SMB2_0
    #[serde(rename = "SMB2_0")]
    Smb20,

    /// SMB3
    #[serde(rename = "SMB3")]
    Smb3,
}

impl Default for SmbMountOptionsVersionEnum {
    fn default() -> Self {
        SmbMountOptionsVersionEnum::Automatic
    }
}

impl cfn_resources::CfnResource for SmbMountOptions {
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
