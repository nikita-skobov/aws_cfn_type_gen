

/// The AWS::DataSync::LocationSMB resource specifies a Server Message Block     (SMB) location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationSMB {


    /// 
    /// The Amazon Resource Names (ARNs) of agents to use for a Server Message Block (SMB)    location.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,


    /// 
    /// Specifies the Windows domain name that your SMB file server belongs to.
    /// 
    /// For more information, see required permissions for SMB locations.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^[A-Za-z0-9]((\.|-+)?[A-Za-z0-9]){0,252}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domain")]
    pub domain: Option<String>,


    /// 
    /// Specifies the version of the SMB protocol that DataSync uses to access your SMB    file server.
    /// 
    /// Required: No
    ///
    /// Type: MountOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountOptions")]
    pub mount_options: Option<MountOptions>,


    /// 
    /// The password of the user who can mount the share and has the permissions to access files and    folders in the SMB share.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 104
    ///
    /// Pattern: ^.{0,104}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: Option<String>,


    /// 
    /// Specifies the Domain Name Service (DNS) name or IP address of the SMB file server that    your DataSync agent will mount.
    /// 
    /// NoteYou can't specify an IP version 6 (IPv6) address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerHostname")]
    pub server_hostname: Option<String>,


    /// 
    /// The subdirectory in the SMB file system that is used to read data from the SMB source    location or write data to the SMB destination. The SMB path should be a path that's    exported by the SMB server, or a subdirectory of that path. The path should be such that it    can be mounted by other SMB clients in your network.
    /// 
    /// Note        Subdirectory must be specified with forward slashes. For example,      /path/to/folder.
    /// 
    /// To transfer all the data in the folder you specified, DataSync must have permissions    to mount the SMB share, as well as to access all the data in that share. To ensure this,    either make sure that the user name and password specified belongs to the user who can mount the share,    and who has the appropriate permissions for all of the files and directories that you want    DataSync to access, or use credentials of a member of the Backup Operators group to mount    the share. Doing either one enables the agent to access the data. For the agent to access    directories, you must additionally enable all execute access.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\$\p{Zs}]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,


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
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The user who can mount the share and has the permissions to access files and folders in the    SMB share.
    /// 
    /// For information about choosing a user name that ensures sufficient permissions to files,       folders, and metadata, see user.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 104
    ///
    /// Pattern: ^[^\x5B\x5D\\/:;|=,+*?]{1,104}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "User")]
    pub user: String,

}



impl cfn_resources::CfnResource for CfnLocationSMB {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationSMB"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.agent_arns;

        if the_val.len() > 4 as _ {
            return Err(format!("Max validation failed on field 'agent_arns'. {} is greater than 4", the_val.len()));
        }

        
        if let Some(the_val) = &self.domain {

        if the_val.len() > 253 as _ {
            return Err(format!("Max validation failed on field 'domain'. {} is greater than 253", the_val.len()));
        }

        }
        
        self.mount_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.password {

        if the_val.len() > 104 as _ {
            return Err(format!("Max validation failed on field 'password'. {} is greater than 104", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.server_hostname {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'server_hostname'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.subdirectory {

        if the_val.len() > 4096 as _ {
            return Err(format!("Max validation failed on field 'subdirectory'. {} is greater than 4096", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.tags {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 50", the_val.len()));
        }

        }
        
        let the_val = &self.user;

        if the_val.len() > 104 as _ {
            return Err(format!("Max validation failed on field 'user'. {} is greater than 104", the_val.len()));
        }

        
        Ok(())
    }
}

/// Specifies the version of the SMB protocol that DataSync uses to access your SMB    file server.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MountOptions {


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
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<MountOptionsVersionEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum MountOptionsVersionEnum {

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

impl Default for MountOptionsVersionEnum {
    fn default() -> Self {
        MountOptionsVersionEnum::Automatic
    }
}


impl cfn_resources::CfnResource for MountOptions {
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