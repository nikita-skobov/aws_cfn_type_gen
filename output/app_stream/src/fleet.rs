

/// The AWS::AppStream::Fleet resource creates a fleet for Amazon AppStream 2.0. A fleet consists of streaming instances that run a specified image when using Always-On or On-Demand.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFleet {


    /// 
    /// The instance type to use when launching fleet instances. The following instance types are available for non-Elastic fleets:
    /// 
    /// stream.standard.small               stream.standard.medium               stream.standard.large               stream.compute.large               stream.compute.xlarge               stream.compute.2xlarge               stream.compute.4xlarge               stream.compute.8xlarge               stream.memory.large               stream.memory.xlarge               stream.memory.2xlarge               stream.memory.4xlarge               stream.memory.8xlarge               stream.memory.z1d.large               stream.memory.z1d.xlarge               stream.memory.z1d.2xlarge               stream.memory.z1d.3xlarge               stream.memory.z1d.6xlarge               stream.memory.z1d.12xlarge               stream.graphics-design.large               stream.graphics-design.xlarge               stream.graphics-design.2xlarge               stream.graphics-design.4xlarge               stream.graphics-desktop.2xlarge               stream.graphics.g4dn.xlarge               stream.graphics.g4dn.2xlarge               stream.graphics.g4dn.4xlarge               stream.graphics.g4dn.8xlarge               stream.graphics.g4dn.12xlarge               stream.graphics.g4dn.16xlarge               stream.graphics-pro.4xlarge               stream.graphics-pro.8xlarge               stream.graphics-pro.16xlarge
    /// 
    /// The following instance types are available for Elastic fleets:
    /// 
    /// stream.standard.small            stream.standard.medium
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: String,


    /// 
    /// The amount of time that users can be idle (inactive) before they are disconnected       from their streaming session and the DisconnectTimeoutInSeconds time       interval begins. Users are notified before they are disconnected due to inactivity. If       they try to reconnect to the streaming session before the time interval specified in       DisconnectTimeoutInSeconds elapses, they are connected to their       previous session. Users are considered idle when they stop providing keyboard or mouse       input during their streaming session. File uploads and downloads, audio in, audio out,       and pixels changing do not qualify as user activity. If users continue to be idle after       the time interval in IdleDisconnectTimeoutInSeconds elapses, they are       disconnected.
    /// 
    /// To prevent users from being disconnected due to inactivity, specify a value of 0. Otherwise, specify a value between 60 and 3600.
    /// 
    /// If you enable this feature, we recommend that you specify a value that corresponds exactly to a whole number of minutes (for example, 60, 120, and 180). If you don't do this, the value is rounded to the nearest minute. For example, if you specify a value of 70, users are disconnected after 1 minute of inactivity. If you specify a value that is at the midpoint between two different minutes, the value is rounded up. For example, if you specify a value of 90, users are disconnected after 2 minutes of inactivity.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdleDisconnectTimeoutInSeconds")]
    pub idle_disconnect_timeout_in_seconds: Option<i64>,


    /// 
    /// The fleet name to display.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,


    /// 
    /// The name of the image used to create the fleet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageName")]
    pub image_name: Option<String>,


    /// The maximum number of concurrent sessions that can be run on an Elastic fleet. This setting is     required for Elastic fleets, but is not used for other fleet types.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConcurrentSessions")]
    pub max_concurrent_sessions: Option<i64>,


    /// 
    /// A unique name for the fleet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,100}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The desired capacity for the fleet. This is not allowed for Elastic fleets.
    /// 
    /// Required: No
    ///
    /// Type: ComputeCapacity
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeCapacity")]
    pub compute_capacity: Option<ComputeCapacity>,


    /// 
    /// The VPC configuration for the fleet. This is required for Elastic fleets, but not required for other fleet types.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,


    /// 
    /// The fleet type.
    /// 
    /// ALWAYS_ON                   Provides users with instant-on access to their apps.             You are charged for all running instances in your fleet, even if no users are streaming apps.                        ON_DEMAND                   Provide users with access to applications after they connect, which takes one to two minutes.             You are charged for instance streaming when users are connected and a             small hourly fee for instances that are not streaming apps.                       ELASTIC                 The pool of streaming instances is managed by Amazon AppStream 2.0. When a          user selects their application or desktop to launch, they will start streaming          after the app block has been downloaded and mounted to a streaming          instance.
    /// 
    /// Allowed Values: ALWAYS_ON | ELASTIC | ON_DEMAND
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FleetType")]
    pub fleet_type: Option<String>,


    /// The USB device filter strings that specify which USB devices a user can redirect to the fleet streaming session, when using the Windows native client. This is allowed but not required for Elastic fleets.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UsbDeviceFilterStrings")]
    pub usb_device_filter_strings: Option<Vec<String>>,


    /// The platform of the fleet. Platform is a required setting for Elastic fleets, and is not used     for other fleet types.
    ///
    /// Allowed Values: WINDOWS_SERVER_2019 | AMAZON_LINUX2
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AMAZON_LINUX2 | WINDOWS | WINDOWS_SERVER_2016 | WINDOWS_SERVER_2019
    ///
    /// Update requires: No interruption
    #[serde(rename = "Platform")]
    pub platform: Option<String>,


    /// 
    /// The ARN of the public, private, or shared image to use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws(?:\-cn|\-iso\-b|\-iso|\-us\-gov)?:[A-Za-z0-9][A-Za-z0-9_/.-]{0,62}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9][A-Za-z0-9:_/+=,@.\\-]{0,1023}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageArn")]
    pub image_arn: Option<String>,


    /// The S3 location of the session scripts configuration zip file. This only applies to Elastic fleets.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionScriptS3Location")]
    pub session_script_s3_location: Option<S3Location>,


    /// 
    /// An array of key-value pairs.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The maximum amount of time that a streaming session can remain active, in seconds. If users are still connected to a streaming instance five minutes before this limit is reached, they are prompted to save any open documents before being disconnected. After this time elapses, the instance is terminated and replaced by a new instance.
    /// 
    /// Specify a value between 600 and 360000.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxUserDurationInSeconds")]
    pub max_user_duration_in_seconds: Option<i64>,


    /// 
    /// The name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. This is not allowed for Elastic fleets.
    /// 
    /// Required: No
    ///
    /// Type: DomainJoinInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainJoinInfo")]
    pub domain_join_info: Option<DomainJoinInfo>,


    /// 
    /// Enables or disables default internet access for the fleet.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDefaultInternetAccess")]
    pub enable_default_internet_access: Option<bool>,


    /// 
    /// The ARN of the IAM role that is applied to the fleet. To assume a role, the fleet instance calls the AWS Security Token Service AssumeRole API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the appstream_machine_role credential profile on the instance.
    /// 
    /// For more information, see Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances in the Amazon AppStream 2.0 Administration Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws(?:\-cn|\-iso\-b|\-iso|\-us\-gov)?:[A-Za-z0-9][A-Za-z0-9_/.-]{0,62}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9_/.-]{0,63}:[A-Za-z0-9][A-Za-z0-9:_/+=,@.\\-]{0,1023}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: Option<String>,


    /// 
    /// The AppStream 2.0 view that is displayed to your users when they stream from the fleet. When APP is specified, only the windows of applications opened by users display. When DESKTOP is specified, the standard desktop that is provided by the operating system displays.
    /// 
    /// The default value is APP.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: APP | DESKTOP
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamView")]
    pub stream_view: Option<String>,


    /// 
    /// The amount of time that a streaming session remains active after users disconnect. If users try to reconnect to the streaming session after a disconnection or network interruption within this time interval, they are connected to their previous session. Otherwise, they are connected to a new session with a new streaming instance.
    /// 
    /// Specify a value between 60 and 360000.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    pub disconnect_timeout_in_seconds: Option<i64>,


    /// 
    /// The description to display.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

impl cfn_resources::CfnResource for CfnFleet {
    fn type_string() -> &'static str {
        "AWS::AppStream::Fleet"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The name of the directory and organizational unit (OU) to use to join a fleet to a Microsoft Active Directory domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DomainJoinInfo {


    /// 
    /// The fully qualified name of the directory (for example, corp.example.com).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<String>,


    /// 
    /// The distinguished name of the organizational unit for computer accounts.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2000
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    pub organizational_unit_distinguished_name: Option<String>,

}


/// Describes the S3 location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {


    /// The S3 key of the S3 object.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Key")]
    pub s3_key: String,


    /// The S3 bucket of the S3 object.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[0-9a-z\.\-]*(?<!\.)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,

}


/// The VPC configuration information for the fleet.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfig {


    /// 
    /// The identifiers of the subnets to which a network interface is attached from the fleet instance. Fleet instances can use one or two subnets.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,


    /// 
    /// The identifiers of the security groups for the fleet.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The desired capacity for a fleet.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComputeCapacity {


    /// 
    /// The desired number of streaming instances.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredInstances")]
    pub desired_instances: i64,

}
