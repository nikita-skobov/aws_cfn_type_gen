/// The AWS::GameLift::Build resource creates a game server build that is    installed and run on instances in an Amazon GameLift fleet. This resource points to an Amazon    S3 location that contains a zip file with all of the components of the game server    build.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBuild {
    ///
    /// A descriptive label that is associated with a build. Build names do not need to be    unique.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The operating system that your game server binaries run on. This value determines the       type of fleet resources that you use for this build. If your game build contains       multiple executables, they all must run on the same operating system. You must specify a       valid operating system in this request. There is no default value. You can't change a       build's operating system later.
    ///
    /// NoteIf you have active fleets using the Windows Server 2012 operating system, you can continue to         create new builds using this OS until October 10, 2023, when Microsoft ends its         support. All others must use Windows Server 2016 when creating new Windows-based         builds.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AMAZON_LINUX | AMAZON_LINUX_2 | WINDOWS_2012 | WINDOWS_2016
    ///
    /// Update requires: Replacement
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<BuildOperatingSystemEnum>,

    ///
    /// A server SDK version you used when integrating your game server build with Amazon GameLift. For more information see Integrate games         with custom game servers. By default Amazon GameLift sets this value to         4.0.2.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^\d+\.\d+\.\d+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerSdkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_sdk_version: Option<cfn_resources::StrVal>,

    ///
    /// Information indicating where your game build files are stored. Use this parameter only       when creating a build with files stored in an Amazon S3 bucket that you own. The storage       location must specify an Amazon S3 bucket name and key. The location must also specify a role       ARN that you set up to allow Amazon GameLift to access your Amazon S3 bucket. The S3 bucket and your       new build must be in the same Region.
    ///
    /// If a StorageLocation is specified, the size of your file can be found in       your Amazon S3 bucket. Amazon GameLift will report a SizeOnDisk of 0.
    ///
    /// Required: No
    ///
    /// Type: StorageLocation
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<StorageLocation>,

    ///
    /// Version information that is associated with this build. Version strings do not need to be unique.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_build_id: CfnBuildbuildid,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum BuildOperatingSystemEnum {
    /// AMAZON_LINUX
    #[serde(rename = "AMAZON_LINUX")]
    Amazonlinux,

    /// AMAZON_LINUX_2
    #[serde(rename = "AMAZON_LINUX_2")]
    Amazonlinux2,

    /// WINDOWS_2012
    #[serde(rename = "WINDOWS_2012")]
    Windows2012,

    /// WINDOWS_2016
    #[serde(rename = "WINDOWS_2016")]
    Windows2016,
}

impl Default for BuildOperatingSystemEnum {
    fn default() -> Self {
        BuildOperatingSystemEnum::Amazonlinux
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBuildbuildid;
impl CfnBuildbuildid {
    pub fn att_name(&self) -> &'static str {
        r#"BuildId"#
    }
}

impl cfn_resources::CfnResource for CfnBuild {
    fn type_string(&self) -> &'static str {
        "AWS::GameLift::Build"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.server_sdk_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'server_sdk_version'. {} is greater than 128", s.len()));
                }
            }
        }

        self.storage_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'version'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'version'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The location in Amazon S3 where build or script files are stored for access by Amazon    GameLift.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StorageLocation {
    ///
    /// An Amazon S3 bucket identifier. Thename of the S3 bucket.
    ///
    /// NoteAmazon GameLift doesn't support uploading from Amazon S3 buckets with names that contain a dot         (.).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: cfn_resources::StrVal,

    ///
    /// The name of the zip file that contains the build files or script files.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The version of the file, if object versioning is turned on for the bucket. Amazon GameLift    uses this information when retrieving files from your S3 bucket. To retrieve a specific    version of the file, provide an object version. To retrieve the latest version of the    file, do not set this parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) for an IAM role that       allows Amazon GameLift to access the S3 bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StorageLocation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'bucket'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'key'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.object_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'object_version'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'role_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
