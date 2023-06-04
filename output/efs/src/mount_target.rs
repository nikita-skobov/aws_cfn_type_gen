/// The AWS::EFS::MountTarget resource is an Amazon EFS resource that creates a mount target for an EFS     file system. You can then mount the file system on Amazon EC2 instances or other resources by using the mount target.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMountTarget {
    ///
    /// The ID of the file system for which to create the mount target.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^(arn:aws[-a-z]*:elasticfilesystem:[0-9a-z-:]+:file-system/fs-[0-9a-f]{8,40}|fs-[0-9a-f]{8,40})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemId")]
    pub file_system_id: cfn_resources::StrVal,

    ///
    /// Valid IPv4 address within the address range of the specified subnet.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 7
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<cfn_resources::StrVal>,

    ///
    /// Up to five VPC security group IDs, of the form sg-xxxxxxxx. These must be    for the same VPC as subnet specified.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Vec<String>,

    ///
    /// The ID of the subnet to add the mount target in. For file systems that use One Zone storage classes, use the subnet   that is associated with the file system's Availability Zone.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 15
    ///
    /// Maximum: 47
    ///
    /// Pattern: ^subnet-[0-9a-f]{8,40}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_id: CfnMountTargetid,

    #[serde(skip_serializing)]
    pub att_ip_address: CfnMountTargetipaddress,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMountTargetid;
impl CfnMountTargetid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMountTargetipaddress;
impl CfnMountTargetipaddress {
    pub fn att_name(&self) -> &'static str {
        r#"IpAddress"#
    }
}

impl cfn_resources::CfnResource for CfnMountTarget {
    fn type_string(&self) -> &'static str {
        "AWS::EFS::MountTarget"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.file_system_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'file_system_id'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.ip_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 15 as _ {
                    return Err(format!(
                        "Max validation failed on field 'ip_address'. {} is greater than 15",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.ip_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 7 as _ {
                    return Err(format!(
                        "Min validation failed on field 'ip_address'. {} is less than 7",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.security_groups;

        if the_val.len() > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'security_groups'. {} is greater than 100",
                the_val.len()
            ));
        }

        let the_val = &self.subnet_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 47 as _ {
                return Err(format!(
                    "Max validation failed on field 'subnet_id'. {} is greater than 47",
                    s.len()
                ));
            }
        }

        let the_val = &self.subnet_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 15 as _ {
                return Err(format!(
                    "Min validation failed on field 'subnet_id'. {} is less than 15",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
