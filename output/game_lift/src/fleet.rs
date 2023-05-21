

/// The AWS::GameLift::Fleet resource creates an Amazon GameLift (GameLift) fleet to host      custom game server or Realtime Servers. A fleet is a set of EC2 instances, configured with instructions to      run game servers on each instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFleet {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AnywhereConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnywhereConfiguration")]
    pub anywhere_configuration: Option<AnywhereConfiguration>,


    /// 
    /// A unique identifier for a build to be deployed on the new fleet. If you are    deploying the fleet with a custom game build, you must specify this property. The build must    have been successfully uploaded to Amazon GameLift and be in a READY status. This    fleet setting cannot be changed once the fleet is created.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Pattern: ^build-\S+|^arn:.*:build\/build-\S+
    ///
    /// Update requires: Replacement
    #[serde(rename = "BuildId")]
    pub build_id: Option<String>,


    /// 
    /// Prompts Amazon GameLift to generate a TLS/SSL certificate for the fleet. Amazon GameLift uses the       certificates to encrypt traffic between game clients and the game servers running on       Amazon GameLift. By default, the CertificateConfiguration is DISABLED.       You can't change this property after you create the fleet.
    /// 
    /// AWS Certificate Manager (ACM) certificates expire after 13 months.       Certificate expiration can cause fleets to fail, preventing players from connecting to       instances in the fleet. We recommend you replace fleets before 13 months, consider using       fleet aliases for a smooth transition.
    /// 
    /// NoteACM isn't available in all AWS regions. A fleet creation request         with certificate generation enabled in an unsupported Region, fails with a 4xx         error. For more information about the supported Regions, see Supported           Regions in the          AWS Certificate Manager User         Guide.
    /// 
    /// Required: No
    ///
    /// Type: CertificateConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateConfiguration")]
    pub certificate_configuration: Option<CertificateConfiguration>,


    /// 
    /// The type of compute resource used to host your game servers. You can use your own       compute resources with Amazon GameLift Anywhere or use Amazon EC2 instances with managed       Amazon GameLift.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ANYWHERE | EC2
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComputeType")]
    pub compute_type: Option<FleetComputeTypeEnum>,


    /// 
    /// A description for the fleet.
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
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The number of EC2 instances that you want this fleet to host. When creating a new    fleet, GameLift automatically sets this value to "1" and initiates a single instance. Once the    fleet is active, update this value to trigger GameLift to add or remove instances from the    fleet.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredEC2Instances")]
    pub desired_ec2_instances: Option<i64>,


    /// 
    /// The allowed IP address ranges and port settings that allow inbound traffic to access       game sessions on this fleet. If the fleet is hosting a custom game build, this property       must be set before players can connect to game sessions. For Realtime Servers fleets, Amazon GameLift       automatically sets TCP and UDP ranges.
    /// 
    /// Required: No
    ///
    /// Type: List of IpPermission
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "EC2InboundPermissions")]
    pub ec2_inbound_permissions: Option<Vec<IpPermission>>,


    /// 
    /// The Amazon GameLift-supported Amazon EC2 instance type to use for all fleet instances. Instance       type determines the computing resources that will be used to host your game servers,       including CPU, memory, storage, and networking capacity. See Amazon Elastic Compute Cloud Instance Types for detailed descriptions       of Amazon EC2 instance types.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: c3.2xlarge | c3.4xlarge | c3.8xlarge | c3.large | c3.xlarge | c4.2xlarge | c4.4xlarge | c4.8xlarge | c4.large | c4.xlarge | c5.12xlarge | c5.18xlarge | c5.24xlarge | c5.2xlarge | c5.4xlarge | c5.9xlarge | c5.large | c5.xlarge | c5a.12xlarge | c5a.16xlarge | c5a.24xlarge | c5a.2xlarge | c5a.4xlarge | c5a.8xlarge | c5a.large | c5a.xlarge | c5d.12xlarge | c5d.18xlarge | c5d.24xlarge | c5d.2xlarge | c5d.4xlarge | c5d.9xlarge | c5d.large | c5d.xlarge | c6a.12xlarge | c6a.16xlarge | c6a.24xlarge | c6a.2xlarge | c6a.4xlarge | c6a.8xlarge | c6a.large | c6a.xlarge | c6i.12xlarge | c6i.16xlarge | c6i.24xlarge | c6i.2xlarge | c6i.4xlarge | c6i.8xlarge | c6i.large | c6i.xlarge | m3.2xlarge | m3.large | m3.medium | m3.xlarge | m4.10xlarge | m4.2xlarge | m4.4xlarge | m4.large | m4.xlarge | m5.12xlarge | m5.16xlarge | m5.24xlarge | m5.2xlarge | m5.4xlarge | m5.8xlarge | m5.large | m5.xlarge | m5a.12xlarge | m5a.16xlarge | m5a.24xlarge | m5a.2xlarge | m5a.4xlarge | m5a.8xlarge | m5a.large | m5a.xlarge | r3.2xlarge | r3.4xlarge | r3.8xlarge | r3.large | r3.xlarge | r4.16xlarge | r4.2xlarge | r4.4xlarge | r4.8xlarge | r4.large | r4.xlarge | r5.12xlarge | r5.16xlarge | r5.24xlarge | r5.2xlarge | r5.4xlarge | r5.8xlarge | r5.large | r5.xlarge | r5a.12xlarge | r5a.16xlarge | r5a.24xlarge | r5a.2xlarge | r5a.4xlarge | r5a.8xlarge | r5a.large | r5a.xlarge | r5d.12xlarge | r5d.16xlarge | r5d.24xlarge | r5d.2xlarge | r5d.4xlarge | r5d.8xlarge | r5d.large | r5d.xlarge | t2.large | t2.medium | t2.micro | t2.small
    ///
    /// Update requires: Replacement
    #[serde(rename = "EC2InstanceType")]
    pub ec2_instance_type: Option<FleetEC2InstanceTypeEnum>,


    /// 
    /// Indicates whether to use On-Demand or Spot instances for this fleet. By default, this       property is set to ON_DEMAND. Learn more about when to use On-Demand versus Spot Instances. This property cannot be changed after the       fleet is created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ON_DEMAND | SPOT
    ///
    /// Update requires: Replacement
    #[serde(rename = "FleetType")]
    pub fleet_type: Option<FleetFleetTypeEnum>,


    /// 
    /// A unique identifier for an IAM role that manages access to your AWS services.     With an instance role ARN set, any application that runs on an instance in this fleet can assume the role,     including install scripts, server processes, and daemons (background processes). Create a role or look up a role's     ARN by using the IAM dashboard in the AWS Management Console.     Learn more about using on-box credentials for your game servers at          Access external resources from a game server. This property cannot be changed after the fleet is created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceRoleARN")]
    pub instance_role_arn: Option<String>,


    /// 
    /// A set of remote locations to deploy additional instances to and manage as part of the       fleet. This parameter can only be used when creating fleets in AWS Regions that       support multiple locations. You can add any Amazon GameLift-supported AWS Region as a remote       location, in the form of an AWS Region code such as us-west-2. To create       a fleet with instances in the home Region only, don't use this parameter.
    /// 
    /// To use this parameter, Amazon GameLift requires you to use your home location in the       request.
    /// 
    /// Required: No
    ///
    /// Type: List of LocationConfiguration
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Locations")]
    pub locations: Option<Vec<LocationConfiguration>>,


    /// 
    /// The maximum number of instances that are allowed in the specified fleet location. If       this parameter is not set, the default is 1.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSize")]
    pub max_size: Option<i64>,


    /// 
    /// The name of an AWS CloudWatch metric group to add this fleet to. A metric group is       used to aggregate the metrics for multiple fleets. You can specify an existing metric       group name or set a new name to create a new metric group. A fleet can be included in       only one metric group at a time.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricGroups")]
    pub metric_groups: Option<Vec<String>>,


    /// 
    /// The minimum number of instances that are allowed in the specified fleet location. If       this parameter is not set, the default is 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinSize")]
    pub min_size: Option<i64>,


    /// 
    /// A descriptive label that is associated with a fleet. Fleet names do not need to be unique.
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
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The status of termination protection for active game sessions on the fleet. By    default, this property is set to NoProtection.
    /// 
    /// NoProtection - Game sessions can be terminated      during active gameplay as a result of a scale-down event.              FullProtection - Game sessions in      ACTIVE status cannot be terminated during a scale-down      event.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FullProtection | NoProtection
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewGameSessionProtectionPolicy")]
    pub new_game_session_protection_policy: Option<FleetNewGameSessionProtectionPolicyEnum>,


    /// 
    /// Used when peering your Amazon GameLift fleet with a VPC, the unique identifier for the AWS       account that owns the VPC. You can find your account ID in the AWS Management Console under account       settings.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "PeerVpcAwsAccountId")]
    pub peer_vpc_aws_account_id: Option<String>,


    /// 
    /// A unique identifier for a VPC with resources to be accessed by your Amazon GameLift fleet. The       VPC must be in the same Region as your fleet. To look up a VPC ID, use the       VPC Dashboard in the AWS Management Console.       Learn more about VPC peering in VPC Peering with Amazon GameLift Fleets.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "PeerVpcId")]
    pub peer_vpc_id: Option<String>,


    /// 
    /// A policy that limits the number of game sessions that an individual player can create       on instances in this fleet within a specified span of time.
    /// 
    /// Required: No
    ///
    /// Type: ResourceCreationLimitPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceCreationLimitPolicy")]
    pub resource_creation_limit_policy: Option<ResourceCreationLimitPolicy>,


    /// 
    /// Instructions for how to launch and maintain server processes on instances in the    fleet. The runtime configuration defines one or more server process configurations, each    identifying a build executable or Realtime script file and the number of processes of    that type to run concurrently.
    /// 
    /// NoteThe RuntimeConfiguration parameter is required unless the fleet is     being configured using the older parameters ServerLaunchPath and     ServerLaunchParameters, which are still supported for backward     compatibility.
    /// 
    /// Required: Conditional
    ///
    /// Type: RuntimeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeConfiguration")]
    pub runtime_configuration: Option<RuntimeConfiguration>,


    /// 
    /// The unique identifier for a Realtime configuration script to be deployed on fleet    instances. You can use either the script ID or ARN. Scripts must be uploaded to Amazon GameLift    prior to creating the fleet. This fleet property cannot be changed later.
    /// 
    /// NoteYou can't use the !Ref command to reference a     script created with a CloudFormation template for the fleet property ScriptId.     Instead, use Fn::GetAtt Script.Arn or Fn::GetAtt Script.Id to     retrieve either of these properties as input for ScriptId. Alternatively, enter a     ScriptId string manually.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Pattern: ^script-\S+|^arn:.*:script\/script-\S+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScriptId")]
    pub script_id: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FleetComputeTypeEnum {

    /// ANYWHERE
    #[serde(rename = "ANYWHERE")]
    Anywhere,

    /// EC2
    #[serde(rename = "EC2")]
    Ec2,

}

impl Default for FleetComputeTypeEnum {
    fn default() -> Self {
        FleetComputeTypeEnum::Anywhere
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FleetEC2InstanceTypeEnum {

    /// c3.2xlarge
    #[serde(rename = "c3.2xlarge")]
    C32xlarge,

    /// c3.4xlarge
    #[serde(rename = "c3.4xlarge")]
    C34xlarge,

    /// c3.8xlarge
    #[serde(rename = "c3.8xlarge")]
    C38xlarge,

    /// c3.large
    #[serde(rename = "c3.large")]
    C3large,

    /// c3.xlarge
    #[serde(rename = "c3.xlarge")]
    C3xlarge,

    /// c4.2xlarge
    #[serde(rename = "c4.2xlarge")]
    C42xlarge,

    /// c4.4xlarge
    #[serde(rename = "c4.4xlarge")]
    C44xlarge,

    /// c4.8xlarge
    #[serde(rename = "c4.8xlarge")]
    C48xlarge,

    /// c4.large
    #[serde(rename = "c4.large")]
    C4large,

    /// c4.xlarge
    #[serde(rename = "c4.xlarge")]
    C4xlarge,

    /// c5.12xlarge
    #[serde(rename = "c5.12xlarge")]
    C512xlarge,

    /// c5.18xlarge
    #[serde(rename = "c5.18xlarge")]
    C518xlarge,

    /// c5.24xlarge
    #[serde(rename = "c5.24xlarge")]
    C524xlarge,

    /// c5.2xlarge
    #[serde(rename = "c5.2xlarge")]
    C52xlarge,

    /// c5.4xlarge
    #[serde(rename = "c5.4xlarge")]
    C54xlarge,

    /// c5.9xlarge
    #[serde(rename = "c5.9xlarge")]
    C59xlarge,

    /// c5.large
    #[serde(rename = "c5.large")]
    C5large,

    /// c5.xlarge
    #[serde(rename = "c5.xlarge")]
    C5xlarge,

    /// c5a.12xlarge
    #[serde(rename = "c5a.12xlarge")]
    C5a12xlarge,

    /// c5a.16xlarge
    #[serde(rename = "c5a.16xlarge")]
    C5a16xlarge,

    /// c5a.24xlarge
    #[serde(rename = "c5a.24xlarge")]
    C5a24xlarge,

    /// c5a.2xlarge
    #[serde(rename = "c5a.2xlarge")]
    C5a2xlarge,

    /// c5a.4xlarge
    #[serde(rename = "c5a.4xlarge")]
    C5a4xlarge,

    /// c5a.8xlarge
    #[serde(rename = "c5a.8xlarge")]
    C5a8xlarge,

    /// c5a.large
    #[serde(rename = "c5a.large")]
    C5alarge,

    /// c5a.xlarge
    #[serde(rename = "c5a.xlarge")]
    C5axlarge,

    /// c5d.12xlarge
    #[serde(rename = "c5d.12xlarge")]
    C5d12xlarge,

    /// c5d.18xlarge
    #[serde(rename = "c5d.18xlarge")]
    C5d18xlarge,

    /// c5d.24xlarge
    #[serde(rename = "c5d.24xlarge")]
    C5d24xlarge,

    /// c5d.2xlarge
    #[serde(rename = "c5d.2xlarge")]
    C5d2xlarge,

    /// c5d.4xlarge
    #[serde(rename = "c5d.4xlarge")]
    C5d4xlarge,

    /// c5d.9xlarge
    #[serde(rename = "c5d.9xlarge")]
    C5d9xlarge,

    /// c5d.large
    #[serde(rename = "c5d.large")]
    C5dlarge,

    /// c5d.xlarge
    #[serde(rename = "c5d.xlarge")]
    C5dxlarge,

    /// c6a.12xlarge
    #[serde(rename = "c6a.12xlarge")]
    C6a12xlarge,

    /// c6a.16xlarge
    #[serde(rename = "c6a.16xlarge")]
    C6a16xlarge,

    /// c6a.24xlarge
    #[serde(rename = "c6a.24xlarge")]
    C6a24xlarge,

    /// c6a.2xlarge
    #[serde(rename = "c6a.2xlarge")]
    C6a2xlarge,

    /// c6a.4xlarge
    #[serde(rename = "c6a.4xlarge")]
    C6a4xlarge,

    /// c6a.8xlarge
    #[serde(rename = "c6a.8xlarge")]
    C6a8xlarge,

    /// c6a.large
    #[serde(rename = "c6a.large")]
    C6alarge,

    /// c6a.xlarge
    #[serde(rename = "c6a.xlarge")]
    C6axlarge,

    /// c6i.12xlarge
    #[serde(rename = "c6i.12xlarge")]
    C6i12xlarge,

    /// c6i.16xlarge
    #[serde(rename = "c6i.16xlarge")]
    C6i16xlarge,

    /// c6i.24xlarge
    #[serde(rename = "c6i.24xlarge")]
    C6i24xlarge,

    /// c6i.2xlarge
    #[serde(rename = "c6i.2xlarge")]
    C6i2xlarge,

    /// c6i.4xlarge
    #[serde(rename = "c6i.4xlarge")]
    C6i4xlarge,

    /// c6i.8xlarge
    #[serde(rename = "c6i.8xlarge")]
    C6i8xlarge,

    /// c6i.large
    #[serde(rename = "c6i.large")]
    C6ilarge,

    /// c6i.xlarge
    #[serde(rename = "c6i.xlarge")]
    C6ixlarge,

    /// m3.2xlarge
    #[serde(rename = "m3.2xlarge")]
    M32xlarge,

    /// m3.large
    #[serde(rename = "m3.large")]
    M3large,

    /// m3.medium
    #[serde(rename = "m3.medium")]
    M3medium,

    /// m3.xlarge
    #[serde(rename = "m3.xlarge")]
    M3xlarge,

    /// m4.10xlarge
    #[serde(rename = "m4.10xlarge")]
    M410xlarge,

    /// m4.2xlarge
    #[serde(rename = "m4.2xlarge")]
    M42xlarge,

    /// m4.4xlarge
    #[serde(rename = "m4.4xlarge")]
    M44xlarge,

    /// m4.large
    #[serde(rename = "m4.large")]
    M4large,

    /// m4.xlarge
    #[serde(rename = "m4.xlarge")]
    M4xlarge,

    /// m5.12xlarge
    #[serde(rename = "m5.12xlarge")]
    M512xlarge,

    /// m5.16xlarge
    #[serde(rename = "m5.16xlarge")]
    M516xlarge,

    /// m5.24xlarge
    #[serde(rename = "m5.24xlarge")]
    M524xlarge,

    /// m5.2xlarge
    #[serde(rename = "m5.2xlarge")]
    M52xlarge,

    /// m5.4xlarge
    #[serde(rename = "m5.4xlarge")]
    M54xlarge,

    /// m5.8xlarge
    #[serde(rename = "m5.8xlarge")]
    M58xlarge,

    /// m5.large
    #[serde(rename = "m5.large")]
    M5large,

    /// m5.xlarge
    #[serde(rename = "m5.xlarge")]
    M5xlarge,

    /// m5a.12xlarge
    #[serde(rename = "m5a.12xlarge")]
    M5a12xlarge,

    /// m5a.16xlarge
    #[serde(rename = "m5a.16xlarge")]
    M5a16xlarge,

    /// m5a.24xlarge
    #[serde(rename = "m5a.24xlarge")]
    M5a24xlarge,

    /// m5a.2xlarge
    #[serde(rename = "m5a.2xlarge")]
    M5a2xlarge,

    /// m5a.4xlarge
    #[serde(rename = "m5a.4xlarge")]
    M5a4xlarge,

    /// m5a.8xlarge
    #[serde(rename = "m5a.8xlarge")]
    M5a8xlarge,

    /// m5a.large
    #[serde(rename = "m5a.large")]
    M5alarge,

    /// m5a.xlarge
    #[serde(rename = "m5a.xlarge")]
    M5axlarge,

    /// r3.2xlarge
    #[serde(rename = "r3.2xlarge")]
    R32xlarge,

    /// r3.4xlarge
    #[serde(rename = "r3.4xlarge")]
    R34xlarge,

    /// r3.8xlarge
    #[serde(rename = "r3.8xlarge")]
    R38xlarge,

    /// r3.large
    #[serde(rename = "r3.large")]
    R3large,

    /// r3.xlarge
    #[serde(rename = "r3.xlarge")]
    R3xlarge,

    /// r4.16xlarge
    #[serde(rename = "r4.16xlarge")]
    R416xlarge,

    /// r4.2xlarge
    #[serde(rename = "r4.2xlarge")]
    R42xlarge,

    /// r4.4xlarge
    #[serde(rename = "r4.4xlarge")]
    R44xlarge,

    /// r4.8xlarge
    #[serde(rename = "r4.8xlarge")]
    R48xlarge,

    /// r4.large
    #[serde(rename = "r4.large")]
    R4large,

    /// r4.xlarge
    #[serde(rename = "r4.xlarge")]
    R4xlarge,

    /// r5.12xlarge
    #[serde(rename = "r5.12xlarge")]
    R512xlarge,

    /// r5.16xlarge
    #[serde(rename = "r5.16xlarge")]
    R516xlarge,

    /// r5.24xlarge
    #[serde(rename = "r5.24xlarge")]
    R524xlarge,

    /// r5.2xlarge
    #[serde(rename = "r5.2xlarge")]
    R52xlarge,

    /// r5.4xlarge
    #[serde(rename = "r5.4xlarge")]
    R54xlarge,

    /// r5.8xlarge
    #[serde(rename = "r5.8xlarge")]
    R58xlarge,

    /// r5.large
    #[serde(rename = "r5.large")]
    R5large,

    /// r5.xlarge
    #[serde(rename = "r5.xlarge")]
    R5xlarge,

    /// r5a.12xlarge
    #[serde(rename = "r5a.12xlarge")]
    R5a12xlarge,

    /// r5a.16xlarge
    #[serde(rename = "r5a.16xlarge")]
    R5a16xlarge,

    /// r5a.24xlarge
    #[serde(rename = "r5a.24xlarge")]
    R5a24xlarge,

    /// r5a.2xlarge
    #[serde(rename = "r5a.2xlarge")]
    R5a2xlarge,

    /// r5a.4xlarge
    #[serde(rename = "r5a.4xlarge")]
    R5a4xlarge,

    /// r5a.8xlarge
    #[serde(rename = "r5a.8xlarge")]
    R5a8xlarge,

    /// r5a.large
    #[serde(rename = "r5a.large")]
    R5alarge,

    /// r5a.xlarge
    #[serde(rename = "r5a.xlarge")]
    R5axlarge,

    /// r5d.12xlarge
    #[serde(rename = "r5d.12xlarge")]
    R5d12xlarge,

    /// r5d.16xlarge
    #[serde(rename = "r5d.16xlarge")]
    R5d16xlarge,

    /// r5d.24xlarge
    #[serde(rename = "r5d.24xlarge")]
    R5d24xlarge,

    /// r5d.2xlarge
    #[serde(rename = "r5d.2xlarge")]
    R5d2xlarge,

    /// r5d.4xlarge
    #[serde(rename = "r5d.4xlarge")]
    R5d4xlarge,

    /// r5d.8xlarge
    #[serde(rename = "r5d.8xlarge")]
    R5d8xlarge,

    /// r5d.large
    #[serde(rename = "r5d.large")]
    R5dlarge,

    /// r5d.xlarge
    #[serde(rename = "r5d.xlarge")]
    R5dxlarge,

    /// t2.large
    #[serde(rename = "t2.large")]
    T2large,

    /// t2.medium
    #[serde(rename = "t2.medium")]
    T2medium,

    /// t2.micro
    #[serde(rename = "t2.micro")]
    T2micro,

    /// t2.small
    #[serde(rename = "t2.small")]
    T2small,

}

impl Default for FleetEC2InstanceTypeEnum {
    fn default() -> Self {
        FleetEC2InstanceTypeEnum::C32xlarge
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FleetFleetTypeEnum {

    /// ON_DEMAND
    #[serde(rename = "ON_DEMAND")]
    Ondemand,

    /// SPOT
    #[serde(rename = "SPOT")]
    Spot,

}

impl Default for FleetFleetTypeEnum {
    fn default() -> Self {
        FleetFleetTypeEnum::Ondemand
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FleetNewGameSessionProtectionPolicyEnum {

    /// FullProtection
    #[serde(rename = "FullProtection")]
    Fullprotection,

    /// NoProtection
    #[serde(rename = "NoProtection")]
    Noprotection,

}

impl Default for FleetNewGameSessionProtectionPolicyEnum {
    fn default() -> Self {
        FleetNewGameSessionProtectionPolicyEnum::Fullprotection
    }
}


impl cfn_resources::CfnResource for CfnFleet {
    fn type_string() -> &'static str {
        "AWS::GameLift::Fleet"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.anywhere_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.certificate_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.desired_ec2_instances {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'desired_ec2_instances'. {} is less than 0", the_val));
        }

        }
        
        if let Some(the_val) = &self.ec2_inbound_permissions {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'ec2_inbound_permissions'. {} is greater than 50", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.instance_role_arn {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'instance_role_arn'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.locations {

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'locations'. {} is greater than 100", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.max_size {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'max_size'. {} is less than 0", the_val));
        }

        }
        
        if let Some(the_val) = &self.metric_groups {

        if the_val.len() > 1 as _ {
            return Err(format!("Max validation failed on field 'metric_groups'. {} is greater than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.min_size {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'min_size'. {} is less than 0", the_val));
        }

        }
        
        let the_val = &self.name;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.peer_vpc_aws_account_id {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'peer_vpc_aws_account_id'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.peer_vpc_aws_account_id {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'peer_vpc_aws_account_id'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.peer_vpc_id {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'peer_vpc_id'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.peer_vpc_id {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'peer_vpc_id'. {} is less than 1", the_val.len()));
        }

        }
        
        self.resource_creation_limit_policy.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.runtime_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Amazon GameLift Anywhere configuration options for your Anywhere fleets.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnywhereConfiguration {


    /// 
    /// The cost to run your fleet per hour. Amazon GameLift uses the provided cost of your fleet to       balance usage in queues. For more information about queues, see Setting         up queues in the Amazon GameLift Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 11
    ///
    /// Pattern: ^\d{1,5}(?:\.\d{1,5})?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cost")]
    pub cost: String,

}



impl cfn_resources::CfnResource for AnywhereConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.cost;

        if the_val.len() > 11 as _ {
            return Err(format!("Max validation failed on field 'cost'. {} is greater than 11", the_val.len()));
        }

        
        let the_val = &self.cost;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'cost'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Determines whether a TLS/SSL certificate is generated for a fleet. This feature must be       enabled when creating the fleet. All instances in a fleet share the same       certificate. The certificate can be retrieved by calling the         GameLift Server         SDK operation GetInstanceCertificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CertificateConfiguration {


    /// 
    /// Indicates whether a TLS/SSL certificate is generated for a fleet.
    /// 
    /// Valid values include:
    /// 
    /// GENERATED - Generate a TLS/SSL certificate           for this fleet.                        DISABLED - (default) Do not generate a           TLS/SSL certificate for this fleet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | GENERATED
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateType")]
    pub certificate_type: CertificateConfigurationCertificateTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateConfigurationCertificateTypeEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// GENERATED
    #[serde(rename = "GENERATED")]
    Generated,

}

impl Default for CertificateConfigurationCertificateTypeEnum {
    fn default() -> Self {
        CertificateConfigurationCertificateTypeEnum::Disabled
    }
}


impl cfn_resources::CfnResource for CertificateConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A range of IP addresses and port settings that allow inbound traffic to connect to    server processes on an instance in a fleet. New game sessions are assigned an IP    address/port number combination, which must fall into the fleet's allowed ranges. Fleets    with custom game builds must have permissions explicitly set. For Realtime Servers fleets, GameLift    automatically opens two port ranges, one for TCP messaging and one for UDP.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IpPermission {


    /// 
    /// A starting value for a range of allowed port numbers.
    /// 
    /// For fleets using Linux builds, only ports 22 and 1026-60000 are valid.
    /// 
    /// For fleets using Windows builds, only ports 1026-60000 are valid.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 60000
    ///
    /// Update requires: No interruption
    #[serde(rename = "FromPort")]
    pub from_port: i64,


    /// 
    /// A range of allowed IP addresses. This value must be expressed in CIDR notation.       Example: "000.000.000.000/[subnet mask]" or optionally the shortened       version "0.0.0.0/[subnet mask]".
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [^\s]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpRange")]
    pub ip_range: String,


    /// 
    /// The network communication protocol used by the fleet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: TCP | UDP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: IpPermissionProtocolEnum,


    /// 
    /// An ending value for a range of allowed port numbers. Port numbers are end-inclusive.       This value must be equal to or greater than FromPort.
    /// 
    /// For fleets using Linux builds, only ports 22 and 1026-60000 are valid.
    /// 
    /// For fleets using Windows builds, only ports 1026-60000 are valid.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 60000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ToPort")]
    pub to_port: i64,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum IpPermissionProtocolEnum {

    /// TCP
    #[serde(rename = "TCP")]
    Tcp,

    /// UDP
    #[serde(rename = "UDP")]
    Udp,

}

impl Default for IpPermissionProtocolEnum {
    fn default() -> Self {
        IpPermissionProtocolEnum::Tcp
    }
}


impl cfn_resources::CfnResource for IpPermission {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.from_port;

        if *the_val > 60000 as _ {
            return Err(format!("Max validation failed on field 'from_port'. {} is greater than 60000", the_val));
        }

        
        let the_val = &self.from_port;

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'from_port'. {} is less than 1", the_val));
        }

        
        let the_val = &self.to_port;

        if *the_val > 60000 as _ {
            return Err(format!("Max validation failed on field 'to_port'. {} is greater than 60000", the_val));
        }

        
        let the_val = &self.to_port;

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'to_port'. {} is less than 1", the_val));
        }

        
        Ok(())
    }
}

/// Current resource capacity settings in a specified fleet or location. The location       value might refer to a fleet's remote location or its home Region.
///
/// Related actions
///
/// DescribeFleetCapacity | DescribeFleetLocationCapacity | UpdateFleetCapacity
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LocationCapacity {


    /// 
    /// The number of Amazon EC2 instances you want to maintain in the specified fleet location.       This value must fall between the minimum and maximum size limits.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredEC2Instances")]
    pub desired_ec2_instances: i64,


    /// 
    /// The maximum number of instances that are allowed in the specified fleet location. If       this parameter is not set, the default is 1.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSize")]
    pub max_size: i64,


    /// 
    /// The minimum number of instances that are allowed in the specified fleet location. If       this parameter is not set, the default is 0.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinSize")]
    pub min_size: i64,

}



impl cfn_resources::CfnResource for LocationCapacity {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.desired_ec2_instances;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'desired_ec2_instances'. {} is less than 0", the_val));
        }

        
        let the_val = &self.max_size;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'max_size'. {} is less than 0", the_val));
        }

        
        let the_val = &self.min_size;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'min_size'. {} is less than 0", the_val));
        }

        
        Ok(())
    }
}

/// A remote location where a multi-location fleet can deploy game servers for game       hosting.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LocationConfiguration {


    /// 
    /// An AWS Region code, such as us-west-2.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[A-Za-z0-9\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: String,


    /// 
    /// Current resource capacity settings in a specified fleet or location. The location       value might refer to a fleet's remote location or its home Region.
    /// 
    /// Related actions
    /// 
    /// DescribeFleetCapacity | DescribeFleetLocationCapacity | UpdateFleetCapacity
    /// 
    /// Required: No
    ///
    /// Type: LocationCapacity
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocationCapacity")]
    pub location_capacity: Option<LocationCapacity>,

}



impl cfn_resources::CfnResource for LocationConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.location;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'location'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.location;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'location'. {} is less than 1", the_val.len()));
        }

        
        self.location_capacity.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A policy that limits the number of game sessions a player can create on the same fleet.    This optional policy gives game owners control over how players can consume available game    server resources. A resource creation policy makes the following statement: "An individual    player can create a maximum number of new game sessions within a specified time    period".
///
/// The policy is evaluated when a player tries to create a new game session. For example,    assume you have a policy of 10 new game sessions and a time period of 60 minutes. On receiving    a CreateGameSession request, Amazon GameLift checks that the player (identified    by CreatorId) has created fewer than 10 game sessions in the past 60    minutes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceCreationLimitPolicy {


    /// 
    /// A policy that puts limits on the number of game sessions that a player can create       within a specified span of time. With this policy, you can control players' ability to       consume available resources.
    /// 
    /// The policy is evaluated when a player tries to create a new game session. On receiving       a CreateGameSession request, Amazon GameLift checks that the player (identified by         CreatorId) has created fewer than game session limit in the specified       time period.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewGameSessionsPerCreator")]
    pub new_game_sessions_per_creator: Option<i64>,


    /// 
    /// The time span used in evaluating the resource creation limit policy.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyPeriodInMinutes")]
    pub policy_period_in_minutes: Option<i64>,

}



impl cfn_resources::CfnResource for ResourceCreationLimitPolicy {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.new_game_sessions_per_creator {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'new_game_sessions_per_creator'. {} is less than 0", the_val));
        }

        }
        
        if let Some(the_val) = &self.policy_period_in_minutes {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'policy_period_in_minutes'. {} is less than 0", the_val));
        }

        }
        
        Ok(())
    }
}

/// A collection of server process configurations that describe the set of processes to    run on each instance in a fleet. Server processes run either an executable in a custom    game build or a Realtime Servers script. GameLift launches the configured processes, manages their    life cycle, and replaces them as needed. Each instance checks regularly for an updated    runtime configuration.
///
/// A GameLift instance is limited to 50 processes running concurrently. To calculate the    total number of processes in a runtime configuration, add the values of the    ConcurrentExecutions parameter for each ServerProcess. Learn more about     Running Multiple     Processes on a Fleet.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RuntimeConfiguration {


    /// 
    /// The maximum amount of time (in seconds) allowed to launch a new game session and have       it report ready to host players. During this time, the game session is in status         ACTIVATING. If the game session does not become active before the       timeout, it is ended and the game session status is changed to       TERMINATED.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 600
    ///
    /// Update requires: No interruption
    #[serde(rename = "GameSessionActivationTimeoutSeconds")]
    pub game_session_activation_timeout_seconds: Option<i64>,


    /// 
    /// The number of game sessions in status ACTIVATING to allow on an instance.       This setting limits the instance resources that can be used for new game activations at       any one time.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConcurrentGameSessionActivations")]
    pub max_concurrent_game_session_activations: Option<i64>,


    /// 
    /// A collection of server process configurations that identify what server processes to       run on each instance in a fleet.
    /// 
    /// Required: No
    ///
    /// Type: List of ServerProcess
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerProcesses")]
    pub server_processes: Option<Vec<ServerProcess>>,

}



impl cfn_resources::CfnResource for RuntimeConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.game_session_activation_timeout_seconds {

        if *the_val > 600 as _ {
            return Err(format!("Max validation failed on field 'game_session_activation_timeout_seconds'. {} is greater than 600", the_val));
        }

        }
        
        if let Some(the_val) = &self.game_session_activation_timeout_seconds {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'game_session_activation_timeout_seconds'. {} is less than 1", the_val));
        }

        }
        
        if let Some(the_val) = &self.max_concurrent_game_session_activations {

        if *the_val > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'max_concurrent_game_session_activations'. {} is greater than 2147483647", the_val));
        }

        }
        
        if let Some(the_val) = &self.max_concurrent_game_session_activations {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'max_concurrent_game_session_activations'. {} is less than 1", the_val));
        }

        }
        
        if let Some(the_val) = &self.server_processes {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'server_processes'. {} is greater than 50", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// A set of instructions for launching server processes on each instance in a fleet.       Server processes run either an executable in a custom game build or a Realtime Servers script.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerProcess {


    /// 
    /// The number of server processes using this configuration that run concurrently on each       instance.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConcurrentExecutions")]
    pub concurrent_executions: i64,


    /// 
    /// The location of a game build executable or the Realtime script file that contains the         Init() function. Game builds and Realtime scripts are installed on       instances at the root:
    /// 
    /// Windows (custom game builds only): C:\game. Example:             "C:\game\MyGame\server.exe"               Linux: /local/game. Examples:             "/local/game/MyGame/server.exe" or             "/local/game/MyRealtimeScript.js"
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [A-Za-z0-9_:.+\/\\\- ]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchPath")]
    pub launch_path: String,


    /// 
    /// An optional list of parameters to pass to the server executable or Realtime script on       launch.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [A-Za-z0-9_:.+\/\\\- =@;{},?'\[\]"]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<String>,

}



impl cfn_resources::CfnResource for ServerProcess {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.concurrent_executions;

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'concurrent_executions'. {} is less than 1", the_val));
        }

        
        let the_val = &self.launch_path;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'launch_path'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.launch_path;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'launch_path'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.parameters {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'parameters'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.parameters {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'parameters'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}