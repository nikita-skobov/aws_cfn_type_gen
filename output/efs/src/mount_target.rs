

/// The AWS::EFS::MountTarget resource is an Amazon EFS resource that creates a mount target for an EFS     file system. You can then mount the file system on Amazon EC2 instances or other resources by using the mount target.
#[derive(Default, serde::Serialize)]
pub struct CfnMountTarget {


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
    pub ip_address: Option<String>,


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
    pub file_system_id: String,


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
    pub subnet_id: String,

}
