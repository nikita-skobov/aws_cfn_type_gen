/// The AWS::Transfer::User resource creates a user and associates them with an     existing server. You can only create and associate users with servers that have the       IdentityProviderType set to SERVICE_MANAGED. Using parameters     for CreateUser, you can specify the user name, set the home directory, store     the user's public key, and assign the user's AWS Identity and Access Management (IAM) role.     You can also optionally add a session policy, and assign metadata with tags that can be     used to group and search for users.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUser {
    ///
    /// The landing directory (folder) for a user when they log in to the server using the client.
    ///
    /// A HomeDirectory example is /bucket_name/home/mydirectory.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^$|/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "HomeDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory: Option<cfn_resources::StrVal>,

    ///
    /// Logical directory mappings that specify what Amazon S3 paths and keys should be visible     to your user and how you want to make them visible. You will need to specify the       "Entry" and "Target" pair, where Entry shows how     the path is made visible and Target is the actual Amazon S3 path. If you only     specify a target, it will be displayed as is. You will need to also make sure that your IAM     role provides access to paths in Target. The following is an example.
    ///
    /// '[ { "Entry": "/", "Target":       "/bucket3/customized-reports/" } ]'
    ///
    /// In most cases, you can use this value instead of the session policy to lock your user     down to the designated home directory ("chroot"). To do this, you can set       Entry to '/' and set Target to the HomeDirectory parameter     value.
    ///
    /// NoteIf the target of a logical directory entry does not exist in Amazon S3, the entry       will be ignored. As a workaround, you can use the Amazon S3 API to create 0 byte objects       as place holders for your directory. If using the CLI, use the s3api call       instead of s3 so you can use the put-object operation. For example, you use       the following: AWS s3api put-object --bucket bucketname --key        path/to/folder/. Make sure that the end of the key name ends in a '/' for it       to be considered a folder.
    ///
    /// Required: No
    ///
    /// Type: List of HomeDirectoryMapEntry
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "HomeDirectoryMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,

    ///
    /// The type of landing directory (folder) that you want your users' home directory to be when they log in to the server.   If you set it to PATH, the user will see the absolute Amazon S3 bucket or EFS paths as is in their file transfer   protocol clients. If you set it LOGICAL, you need to provide mappings in the HomeDirectoryMappings for   how you want to make Amazon S3 or Amazon EFS paths visible to your users.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LOGICAL | PATH
    ///
    /// Update requires: No interruption
    #[serde(rename = "HomeDirectoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_directory_type: Option<UserHomeDirectoryTypeEnum>,

    ///
    /// A session policy for your user so you can use the same IAM role across multiple     users. This policy restricts user access to portions of their Amazon S3 bucket. Variables     that you can use inside this policy include ${Transfer:UserName},       ${Transfer:HomeDirectory}, and ${Transfer:HomeBucket}.
    ///
    /// NoteFor session policies, AWS Transfer Family stores the policy as a JSON blob,       instead of the Amazon Resource Name (ARN) of the policy. You save the policy as a JSON       blob and pass it in the Policy argument.For an example of a session policy, see Example session        policy.For more information, see AssumeRole in the        AWS Security Token Service API Reference.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the full POSIX identity, including user ID (Uid), group ID     (Gid), and any secondary groups IDs (SecondaryGids), that controls    your users' access to your Amazon Elastic File System (Amazon EFS) file systems. The POSIX    permissions that are set on files and directories in your file system determine the level of    access your users get when transferring files into and out of your Amazon EFS file    systems.
    ///
    /// Required: No
    ///
    /// Type: PosixProfile
    ///
    /// Update requires: No interruption
    #[serde(rename = "PosixProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_profile: Option<PosixProfile>,

    ///
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that controls your users' access to your Amazon S3    bucket or Amazon EFS file system. The policies attached to this role determine the level of access that you want to provide your users    when transferring files into and out of your Amazon S3 bucket or Amazon EFS file system. The IAM role should also contain a trust    relationship that allows the server to access your resources when servicing your users' transfer requests.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: cfn_resources::StrVal,

    ///
    /// A system-assigned unique identifier for a server instance. This is the specific server    that you added your user to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^s-([0-9a-f]{17})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerId")]
    pub server_id: cfn_resources::StrVal,

    ///
    /// Specifies the public key portion of the Secure Shell (SSH) keys stored for the described    user.
    ///
    /// Required: No
    ///
    /// Type: List of SshPublicKey
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "SshPublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_keys: Option<Vec<SshPublicKey>>,

    ///
    /// Key-value pairs that can be used to group and search for users. Tags are metadata attached    to users for any purpose.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// A unique string that identifies a user and is associated with a ServerId. This user name must be a minimum of 3 and a maximum of 100 characters    long. The following are valid characters: a-z, A-Z, 0-9, underscore '_', hyphen    '-', period '.', and at sign '@'. The user name can't start    with a hyphen, period, or at sign.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[\w][\w@.-]{2,99}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserName")]
    pub user_name: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum UserHomeDirectoryTypeEnum {
    /// LOGICAL
    #[serde(rename = "LOGICAL")]
    Logical,

    /// PATH
    #[serde(rename = "PATH")]
    Path,
}

impl Default for UserHomeDirectoryTypeEnum {
    fn default() -> Self {
        UserHomeDirectoryTypeEnum::Logical
    }
}

impl cfn_resources::CfnResource for CfnUser {
    fn type_string(&self) -> &'static str {
        "AWS::Transfer::User"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.home_directory {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'home_directory'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.home_directory_mappings {
            if the_val.len() > 50 as _ {
                return Err(format!("Max validation failed on field 'home_directory_mappings'. {} is greater than 50", the_val.len()));
            }
        }

        if let Some(the_val) = &self.policy {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'policy'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        self.posix_profile
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.role;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'role'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.role;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'role'. {} is less than 20",
                    s.len()
                ));
            }
        }

        let the_val = &self.server_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 19 as _ {
                return Err(format!(
                    "Max validation failed on field 'server_id'. {} is greater than 19",
                    s.len()
                ));
            }
        }

        let the_val = &self.server_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 19 as _ {
                return Err(format!(
                    "Min validation failed on field 'server_id'. {} is less than 19",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.ssh_public_keys {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'ssh_public_keys'. {} is greater than 5",
                    the_val.len()
                ));
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

        let the_val = &self.user_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_name'. {} is less than 3",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents an object that contains entries and targets for       HomeDirectoryMappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HomeDirectoryMapEntry {
    ///
    /// Represents an entry for HomeDirectoryMappings.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Entry")]
    pub entry: cfn_resources::StrVal,

    ///
    /// Represents the map target that is used in a HomeDirectorymapEntry.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for HomeDirectoryMapEntry {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.entry;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'entry'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        let the_val = &self.target;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'target'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The full POSIX identity, including user ID (Uid), group ID    (Gid), and any secondary groups IDs (SecondaryGids), that controls    your users' access to your Amazon EFS file systems. The POSIX permissions that are set on    files and directories in your file system determine the level of access your users get when    transferring files into and out of your Amazon EFS file systems.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PosixProfile {
    ///
    /// The POSIX group ID used for all EFS operations by this user.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Gid")]
    pub gid: f64,

    ///
    /// The secondary POSIX group IDs used for all EFS operations by this user.
    ///
    /// Required: No
    ///
    /// Type: List of Double
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryGids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<f64>>,

    ///
    /// The POSIX user ID used for all EFS operations by this user.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Uid")]
    pub uid: f64,
}

impl cfn_resources::CfnResource for PosixProfile {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.secondary_gids {
            if the_val.len() > 16 as _ {
                return Err(format!(
                    "Max validation failed on field 'secondary_gids'. {} is greater than 16",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Provides information about the public Secure Shell (SSH) key that is associated with a    Transfer Family user account for the specific file transfer protocol-enabled server (as identified by     ServerId). The information returned includes the date the key was imported, the    public key contents, and the public key ID. A user can store more than one SSH public key    associated with their user name on a specific server.
///
/// SshPublicKeyBody
///
/// Specifies the content of the SSH public key as specified by the PublicKeyId.
///
/// AWS Transfer Family accepts RSA, ECDSA, and ED25519 keys.
///
/// Type: String
///
/// Length Constraints: Maximum length of 2048.
///
/// Required: Yes
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SshPublicKey {}

impl cfn_resources::CfnResource for SshPublicKey {
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
