

/// The AWS::Batch::ComputeEnvironment resource defines your AWS Batch compute  environment. You can define MANAGED or UNMANAGED compute environments. MANAGED  compute environments can use Amazon EC2 or AWS Fargate resources. UNMANAGED compute  environments can only use EC2 resources. For more information, see Compute Environments in the AWS Batch User Guide.
///
/// In a managed compute environment, AWS Batch manages the capacity and instance types of the compute  resources within the environment. This is based on the compute resource specification that you define or the launch template that you  specify when you create the compute environment. You can choose either to use EC2 On-Demand Instances and EC2 Spot  Instances, or to use Fargate and Fargate Spot capacity in your managed compute environment. You can optionally set a  maximum price so that Spot Instances only launch when the Spot Instance price is below a specified percentage of the  On-Demand price.
///
/// In an unmanaged compute environment, you can manage your own EC2 compute resources and have a lot of flexibility  with how you configure your compute resources. For example, you can use custom AMI. However, you need to verify that  your AMI meets the Amazon ECS container instance AMI specification. For more information, see container instance   AMIs in the Amazon Elastic Container Service Developer Guide. After you have created  your unmanaged compute environment, you can use the DescribeComputeEnvironments operation  to find the Amazon ECS cluster that is associated with it. Then, manually launch your container instances into that  Amazon ECS cluster. For more information, see Launching an Amazon ECS container   instance in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnComputeEnvironment {


    /// 
    /// The name for your compute environment. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComputeEnvironmentName")]
    pub compute_environment_name: Option<String>,


    /// 
    /// The ComputeResources property type specifies details of the compute resources managed by the compute  environment. This parameter is required for managed compute environments. For more information, see Compute Environments in the   AWS Batch User Guide.
    /// 
    /// Required: No
    ///
    /// Type: ComputeResources
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeResources")]
    pub compute_resources: Option<ComputeResources>,


    /// 
    /// The details for the Amazon EKS cluster that supports the compute environment.
    /// 
    /// Required: No
    ///
    /// Type: EksConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EksConfiguration")]
    pub eks_configuration: Option<EksConfiguration>,


    /// 
    /// Specifies whether the compute environment is replaced if an update is made that requires replacing the instances  in the compute environment. The default value is true. To enable more properties to be updated, set this  property to false. When changing the value of this property to false, do not change any  other properties at the same time. If other properties are changed at the same time, and the change needs to be  rolled back but it can't, it's possible for the stack to go into the UPDATE_ROLLBACK_FAILED state. You  can't update a stack that is in the UPDATE_ROLLBACK_FAILED state. However, if you can continue to roll  it back, you can return the stack to its original settings and then try to update it again. For more information, see   Continue rolling back   an update in the AWS CloudFormation User Guide.
    /// 
    /// The properties that can't be changed without replacing the compute environment are in the ComputeResources property type: AllocationStrategy, BidPercentage, Ec2Configuration, Ec2KeyPair, Ec2KeyPair, ImageId, InstanceRole, InstanceTypes, LaunchTemplate, MaxvCpus, MinvCpus, PlacementGroup, SecurityGroupIds, Subnets, Tags, Type, and UpdateToLatestImageVersion.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplaceComputeEnvironment")]
    pub replace_compute_environment: Option<bool>,


    /// 
    /// The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf. For  more information, see AWS Batch service IAM   role in the         AWS Batch User Guide.
    /// 
    /// ImportantIf your account already created the AWS Batch service-linked role, that role is used by default for your compute   environment unless you specify a different role here. If the AWS Batch service-linked role doesn't exist in your   account, and no role is specified here, the service attempts to create the AWS Batch service-linked role in your   account.
    /// 
    /// If your specified role has a path other than /, then you must specify either the full role ARN  (recommended) or prefix the role name with the path. For example, if a role with the name bar has a path  of /foo/, specify /foo/bar as the role name. For more information, see Friendly names  and paths in the IAM User Guide.
    /// 
    /// NoteDepending on how you created your AWS Batch service role, its ARN might contain the service-role   path prefix. When you only specify the name of the service role, AWS Batch assumes that your ARN doesn't use the   service-role path prefix. Because of this, we recommend that you specify the full ARN of your service   role when you create compute environments.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRole")]
    pub service_role: Option<String>,


    /// 
    /// The state of the compute environment. If the state is ENABLED, then the    compute environment accepts jobs from a queue and can scale out automatically based on    queues.
    /// 
    /// If the state is ENABLED, then the AWS Batch scheduler can attempt to place jobs    from an associated job queue on the compute resources within the environment. If the compute    environment is managed, then it can scale its instances out or in automatically, based on the    job queue demand.
    /// 
    /// If the state is DISABLED, then the AWS Batch scheduler doesn't attempt to place    jobs within the environment. Jobs in a STARTING or RUNNING state    continue to progress normally. Managed compute environments in the DISABLED state    don't scale out.
    /// 
    /// NoteCompute environments in a DISABLED state may continue to incur billing     charges. To prevent additional charges, turn off and then delete the compute environment.     For more information, see State in the          AWS Batch User Guide.
    /// 
    /// When an instance is idle, the instance scales down to the minvCpus value.    However, the instance size doesn't change. For example, consider a c5.8xlarge    instance with a minvCpus value of 4 and a desiredvCpus    value of 36. This instance doesn't scale down to a c5.large    instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: Option<ComputeEnvironmentStateEnum>,


    /// 
    /// The tags applied to the compute environment.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The type of the compute environment: MANAGED or UNMANAGED. For more information, see   Compute Environments in the           AWS Batch User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MANAGED | UNMANAGED
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: ComputeEnvironmentTypeEnum,


    /// 
    /// The maximum number of vCPUs for an unmanaged compute environment. This parameter is only used for fair share  scheduling to reserve vCPU capacity for new share identifiers. If this parameter isn't provided for a fair share job  queue, no vCPU capacity is reserved.
    /// 
    /// NoteThis parameter is only supported when the type parameter is set to UNMANAGED.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnmanagedvCpus")]
    pub unmanagedv_cpus: Option<i64>,


    /// 
    /// Specifies the infrastructure update policy for the compute environment. For more information  about infrastructure updates, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// Required: No
    ///
    /// Type: UpdatePolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdatePolicy")]
    pub update_policy: Option<UpdatePolicy>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ComputeEnvironmentStateEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

}

impl Default for ComputeEnvironmentStateEnum {
    fn default() -> Self {
        ComputeEnvironmentStateEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ComputeEnvironmentTypeEnum {

    /// MANAGED
    #[serde(rename = "MANAGED")]
    Managed,

    /// UNMANAGED
    #[serde(rename = "UNMANAGED")]
    Unmanaged,

}

impl Default for ComputeEnvironmentTypeEnum {
    fn default() -> Self {
        ComputeEnvironmentTypeEnum::Managed
    }
}


impl cfn_resources::CfnResource for CfnComputeEnvironment {
    fn type_string() -> &'static str {
        "AWS::Batch::ComputeEnvironment"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.compute_resources.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.eks_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.update_policy.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Details about the compute resources managed by the compute environment. This parameter is required for managed  compute environments. For more information, see Compute Environments in the         AWS Batch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComputeResources {


    /// 
    /// The allocation strategy to use for the compute resource if not enough instances of the best fitting instance  type can be allocated. This might be because of availability of the instance type in the Region or Amazon EC2 service limits. For  more information, see Allocation   strategies in the   AWS Batch User Guide.
    /// 
    /// When updating a compute environment, changing the allocation strategy requires an infrastructure update of the  compute environment. For more information, see Updating compute environments in the   AWS Batch User Guide. BEST_FIT is not supported when updating a compute  environment.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources, and shouldn't be   specified.
    /// 
    /// BEST_FIT (default)      AWS Batch selects an instance type that best fits the needs of the jobs with a preference for    the lowest-cost instance type. If additional instances of the selected instance type aren't available, AWS Batch waits for the additional instances to be available. If there aren't enough instances available,    or if the user is reaching Amazon EC2 service limits then additional jobs aren't run until the currently running jobs have    completed. This allocation strategy keeps costs lower but can limit scaling. If you are using Spot Fleets with    BEST_FIT then the Spot Fleet IAM role must be specified.        BEST_FIT_PROGRESSIVE         AWS Batch will select additional instance types that are large enough to meet the requirements of    the jobs in the queue, with a preference for instance types with a lower cost per unit vCPU. If additional    instances of the previously selected instance types aren't available, AWS Batch will select new    instance types.        SPOT_CAPACITY_OPTIMIZED         AWS Batch will select one or more instance types that are large enough to meet the requirements of    the jobs in the queue, with a preference for instance types that are less likely to be interrupted. This    allocation strategy is only available for Spot Instance compute resources.
    /// 
    /// With both BEST_FIT_PROGRESSIVE and SPOT_CAPACITY_OPTIMIZED allocation strategies using  On-Demand or Spot Instances, and the BEST_FIT strategy using Spot Instances, AWS Batch  might need to go above maxvCpus to meet your capacity requirements. In this event, AWS Batch never exceeds maxvCpus by more than a single instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BEST_FIT_PROGRESSIVE | SPOT_CAPACITY_OPTIMIZED
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<ComputeResourcesAllocationStrategyEnum>,


    /// 
    /// The maximum percentage that a Spot Instance price can be when compared with the On-Demand  price for that instance type before instances are launched. For example, if your maximum  percentage is 20%, the Spot price must be less than 20% of the current On-Demand price for that  Amazon EC2 instance. You always pay the lowest (market) price and never more than your maximum  percentage. For most use  cases, we recommend leaving this field empty.
    /// 
    /// When updating a compute environment, changing the bid percentage requires an infrastructure  update of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "BidPercentage")]
    pub bid_percentage: Option<i64>,


    /// 
    /// The desired number of Amazon EC2 vCPUS in the compute environment. AWS Batch modifies this value  between the minimum and maximum values based on job queue demand.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// Note        AWS Batch doesn't support changing the desired number of vCPUs of an existing compute   environment. Don't specify this parameter for compute environments using Amazon EKS clusters.
    /// 
    /// NoteWhen you update the desiredvCpus setting, the value must be between the   minvCpus and maxvCpus values. Additionally, the updated desiredvCpus value must be greater than or equal to   the current desiredvCpus value. For more information, see Troubleshooting   AWS Batch in the          AWS Batch User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredvCpus")]
    pub desiredv_cpus: Option<i64>,


    /// 
    /// Provides information used to select Amazon Machine Images (AMIs) for EC2 instances in the  compute environment. If Ec2Configuration isn't specified, the default is   ECS_AL2.
    /// 
    /// When updating a compute environment, changing this setting requires an infrastructure update  of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide. To remove the EC2 configuration and any custom AMI ID  specified in imageIdOverride, set this value to an empty string.
    /// 
    /// One or two values can be provided.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// Required: No
    ///
    /// Type: List of Ec2ConfigurationObject
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Ec2Configuration")]
    pub ec2_configuration: Option<Vec<Ec2ConfigurationObject>>,


    /// 
    /// The Amazon EC2 key pair that's used for instances launched in the compute environment. You can  use this key pair to log in to your instances with SSH. To remove the Amazon EC2 key pair, set this  value to an empty string.
    /// 
    /// When updating a compute environment, changing the EC2 key pair requires an infrastructure  update of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Ec2KeyPair")]
    pub ec2_key_pair: Option<String>,


    /// 
    /// The Amazon Machine Image (AMI) ID used for instances launched in the compute environment.  This parameter is overridden by the imageIdOverride member of the   Ec2Configuration structure. To remove the custom AMI ID and use the default AMI ID,  set this value to an empty string.
    /// 
    /// When updating a compute environment, changing the AMI ID requires an infrastructure update  of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// NoteThe AMI that you choose for a compute environment must match the architecture of the instance types that   you intend to use for that compute environment. For example, if your compute environment uses A1 instance types,   the compute resource AMI that you choose must support ARM instances. Amazon ECS vends both x86 and ARM versions of the   Amazon ECS-optimized Amazon Linux 2 AMI. For more information, see Amazon ECS-optimized   Amazon Linux 2 AMI   in the Amazon Elastic Container Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ImageId")]
    pub image_id: Option<String>,


    /// 
    /// The Amazon ECS instance profile applied to Amazon EC2 instances in a compute environment. You can  specify the short name or full Amazon Resource Name (ARN) of an instance profile. For example,           ecsInstanceRole       or   arn:aws:iam::<aws_account_id>:instance-profile/ecsInstanceRole       .  For more information, see Amazon ECS instance role in the         AWS Batch User Guide.
    /// 
    /// When updating a compute environment, changing this setting requires an infrastructure update  of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "InstanceRole")]
    pub instance_role: Option<String>,


    /// 
    /// The instances types that can be launched. You can specify instance families to launch any  instance type within those families (for example, c5 or p3), or you can  specify specific sizes within a family (such as c5.8xlarge). You can also choose   optimal to select instance types (from the C4, M4, and R4 instance families) that  match the demand of your job queues.
    /// 
    /// When updating a compute environment, changing this setting requires an infrastructure update  of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// NoteWhen you create a compute environment, the instance types that you select for the compute environment must   share the same architecture. For example, you can't mix x86 and ARM instances in the same compute   environment.
    /// 
    /// NoteCurrently, optimal uses instance types from the C4, M4, and R4 instance   families. In Regions that don't have instance types from those instance families, instance types   from the C5, M5, and R5 instance families are used.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "InstanceTypes")]
    pub instance_types: Option<Vec<String>>,


    /// 
    /// The launch template to use for your compute resources. Any other compute resource parameters that you specify in  a CreateComputeEnvironment API operation override the same parameters in the launch template. You must  specify either the launch template ID or launch template name in the request, but not both. For more information, see   Launch Template Support in  the AWS Batch User Guide. Removing the launch template from a compute environment  will not remove the AMI specified in the launch template. In order to update the AMI specified in a launch template,  the updateToLatestImageVersion parameter must be set to true.
    /// 
    /// When updating a compute environment, changing the launch template requires an infrastructure update of the  compute environment. For more information, see Updating compute environments in the   AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    /// 
    /// Required: No
    ///
    /// Type: LaunchTemplateSpecification
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplateSpecification>,


    /// 
    /// The maximum number of Amazon EC2 vCPUs that an environment can reach.
    /// 
    /// NoteWith both BEST_FIT_PROGRESSIVE and SPOT_CAPACITY_OPTIMIZED   allocation strategies using On-Demand or Spot Instances, and the BEST_FIT strategy   using Spot Instances, AWS Batch might need to exceed maxvCpus to meet your capacity   requirements. In this event, AWS Batch never exceeds maxvCpus by more than a single   instance. That is, no more than a single instance from among those specified in your compute   environment.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxvCpus")]
    pub maxv_cpus: i64,


    /// 
    /// The minimum number of Amazon EC2 vCPUs that an environment should maintain (even if the compute  environment is DISABLED).
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinvCpus")]
    pub minv_cpus: Option<i64>,


    /// 
    /// The Amazon EC2 placement group to associate with your compute resources. If you intend to submit  multi-node parallel jobs to your compute environment, you should consider creating a cluster  placement group and associate it with your compute resources. This keeps your multi-node parallel  job on a logical grouping of instances within a single Availability Zone with high network flow  potential. For more information, see Placement groups in the   Amazon EC2 User Guide for Linux Instances.
    /// 
    /// When updating a compute environment, changing the placement group requires an infrastructure  update of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PlacementGroup")]
    pub placement_group: Option<String>,


    /// 
    /// The Amazon EC2 security groups that are associated with instances launched in the compute  environment. This parameter is required for Fargate compute resources, where it can contain up  to 5 security groups. For Fargate compute resources, providing an empty list is handled as if  this parameter wasn't specified and no change is made. For EC2 compute resources, providing an  empty list removes the security groups from the compute resource.
    /// 
    /// When updating a compute environment, changing the EC2 security groups requires an  infrastructure update of the compute environment. For more information, see Updating compute   environments in the         AWS Batch User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon EC2 Spot Fleet IAM role applied to a SPOT compute  environment. This role is required if the allocation strategy set to BEST_FIT or if  the allocation strategy isn't specified. For more information, see Amazon EC2 spot fleet role in the           AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// ImportantTo tag your Spot Instances on creation, the Spot Fleet IAM role specified here must use   the newer AmazonEC2SpotFleetTaggingRole managed policy. The   previously recommended AmazonEC2SpotFleetRole managed policy   doesn't have the required permissions to tag Spot Instances. For more information, see Spot instances   not tagged on creation in the          AWS Batch User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SpotIamFleetRole")]
    pub spot_iam_fleet_role: Option<String>,


    /// 
    /// The VPC subnets where the compute resources are launched. Fargate compute resources can  contain up to 16 subnets. For Fargate compute resources, providing an empty list will be  handled as if this parameter wasn't specified and no change is made. For EC2 compute resources,  providing an empty list removes the VPC subnets from the compute resource. For more information,  see VPCs and   subnets in the Amazon VPC User Guide.
    /// 
    /// When updating a compute environment, changing the VPC subnets requires an infrastructure  update of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// Note        AWS Batch on Amazon EC2 and AWS Batch on Amazon EKS support Local Zones. For more information, see Local   Zones in the Amazon EC2 User Guide for Linux Instances, Amazon EKS and AWS Local   Zones in the Amazon EKS User Guide and Amazon ECS   clusters in Local Zones, Wavelength Zones, and AWS Outposts in the Amazon ECS   Developer Guide.        AWS Batch on Fargate doesn't currently support Local Zones.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,


    /// 
    /// Key-value pair tags to be applied to EC2 resources that are launched in the compute  environment. For AWS Batch, these take the form of "String1": "String2", where   String1 is the tag key and String2 is the tag value-for example,   { "Name": "Batch Instance - C4OnDemand" }. This is helpful for recognizing your  AWS Batch instances in the Amazon EC2 console. These tags aren't seen when using the AWS Batch       ListTagsForResource API operation.
    /// 
    /// When updating a compute environment, changing this setting requires an infrastructure update  of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't specify it.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The type of compute environment: EC2, SPOT, FARGATE, or   FARGATE_SPOT. For more information, see Compute environments in the   AWS Batch User Guide.
    /// 
    /// If you choose SPOT, you must also specify an Amazon EC2 Spot Fleet role with the   spotIamFleetRole parameter. For more information, see Amazon EC2 spot fleet role in the   AWS Batch User Guide.
    /// 
    /// When updating compute environment, changing the type of a compute environment requires an infrastructure update  of the compute environment. For more information, see Updating compute environments in the   AWS Batch User Guide.
    /// 
    /// When updating the type of a compute environment, changing between EC2 and SPOT or  between FARGATE and FARGATE_SPOT will initiate an infrastructure update, but if you switch  between EC2 and FARGATE, AWS CloudFormation will create a new compute environment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EC2 | FARGATE | FARGATE_SPOT | SPOT
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Type")]
    pub cfn_type: ComputeResourcesTypeEnum,


    /// 
    /// Specifies whether the AMI ID is updated to the latest one that's supported by AWS Batch when  the compute environment has an infrastructure update. The default value is  false.
    /// 
    /// NoteAn AMI ID can either be specified in the imageId or   imageIdOverride parameters or be determined by the launch template that's   specified in the launchTemplate parameter. If an AMI ID is specified any of these   ways, this parameter is ignored. For more information about to update AMI IDs during an   infrastructure update, see Updating   the AMI ID in the          AWS Batch User Guide.
    /// 
    /// When updating a compute environment, changing this setting requires an infrastructure update  of the compute environment. For more information, see Updating compute environments in the           AWS Batch User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateToLatestImageVersion")]
    pub update_to_latest_image_version: Option<bool>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ComputeResourcesAllocationStrategyEnum {

    /// BEST_FIT_PROGRESSIVE
    #[serde(rename = "BEST_FIT_PROGRESSIVE")]
    Bestfitprogressive,

    /// SPOT_CAPACITY_OPTIMIZED
    #[serde(rename = "SPOT_CAPACITY_OPTIMIZED")]
    Spotcapacityoptimized,

}

impl Default for ComputeResourcesAllocationStrategyEnum {
    fn default() -> Self {
        ComputeResourcesAllocationStrategyEnum::Bestfitprogressive
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ComputeResourcesTypeEnum {

    /// EC2
    #[serde(rename = "EC2")]
    Ec2,

    /// FARGATE
    #[serde(rename = "FARGATE")]
    Fargate,

    /// FARGATE_SPOT
    #[serde(rename = "FARGATE_SPOT")]
    Fargatespot,

    /// SPOT
    #[serde(rename = "SPOT")]
    Spot,

}

impl Default for ComputeResourcesTypeEnum {
    fn default() -> Self {
        ComputeResourcesTypeEnum::Ec2
    }
}


impl cfn_resources::CfnResource for ComputeResources {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.launch_template.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Provides information used to select Amazon Machine Images (AMIs) for instances in the  compute environment. If Ec2Configuration isn't specified, the default is   ECS_AL2 (Amazon Linux 2).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ec2ConfigurationObject {


    /// 
    /// The AMI ID used for instances launched in the compute environment that match the image type.  This setting overrides the imageId set in the computeResource  object.
    /// 
    /// NoteThe AMI that you choose for a compute environment must match the architecture of the instance types that   you intend to use for that compute environment. For example, if your compute environment uses A1 instance types,   the compute resource AMI that you choose must support ARM instances. Amazon ECS vends both x86 and ARM versions of the   Amazon ECS-optimized Amazon Linux 2 AMI. For more information, see Amazon ECS-optimized   Amazon Linux 2 AMI   in the Amazon Elastic Container Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ImageIdOverride")]
    pub image_id_override: Option<String>,


    /// 
    /// The Kubernetes version for the compute environment. If you don't specify a value, the latest  version that AWS Batch supports is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ImageKubernetesVersion")]
    pub image_kubernetes_version: Option<String>,


    /// 
    /// The image type to match with the instance type to select an AMI. The supported values are  different for ECS and EKS resources.
    /// 
    /// ECS                  If the imageIdOverride parameter isn't specified, then a recent Amazon ECS-optimized Amazon Linux 2 AMI (ECS_AL2) is used. If a new image type is    specified in an update, but neither an imageId nor a imageIdOverride    parameter is specified, then the latest Amazon ECS optimized AMI for that image type that's    supported by AWS Batch is used.                                                                  ECS_AL2                                          Amazon Linux     2: Default for all non-GPU instance families.                                     ECS_AL2_NVIDIA                                          Amazon Linux 2      (GPU): Default for all GPU instance families (for example P4 and      G4) and can be used for all non AWS Graviton-based instance types.                                     ECS_AL1                                          Amazon Linux. Amazon Linux has     reached the end-of-life of standard support. For more information, see Amazon Linux AMI.                                               EKS                  If the imageIdOverride parameter isn't specified, then a recent Amazon EKS-optimized Amazon Linux    AMI (EKS_AL2) is used. If a new image type is specified in an update,    but neither an imageId nor a imageIdOverride parameter is specified,    then the latest Amazon EKS optimized AMI for that image type that AWS Batch supports is used.                                                       EKS_AL2                                          Amazon      Linux 2: Default for all non-GPU instance families.                                     EKS_AL2_NVIDIA                                          Amazon      Linux 2 (accelerated): Default for all GPU instance families (for example,      P4 and G4) and can be used for all non AWS Graviton-based     instance types.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ImageType")]
    pub image_type: String,

}



impl cfn_resources::CfnResource for Ec2ConfigurationObject {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.image_id_override {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'image_id_override'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.image_id_override {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'image_id_override'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.image_kubernetes_version {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'image_kubernetes_version'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.image_kubernetes_version {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'image_kubernetes_version'. {} is less than 1", the_val.len()));
        }

        }
        
        let the_val = &self.image_type;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'image_type'. {} is greater than 256", the_val.len()));
        }

        
        let the_val = &self.image_type;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'image_type'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Configuration for the Amazon EKS cluster that supports the AWS Batch compute environment. The  cluster must exist before the compute environment can be created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon EKS cluster. An example is   arn:aws:eks:us-east-1:123456789012:cluster/ClusterForBatch       .
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EksClusterArn")]
    pub eks_cluster_arn: String,


    /// 
    /// The namespace of the Amazon EKS cluster. AWS Batch manages pods in this namespace. The value  can't left empty or null. It must be fewer than 64 characters long, can't be set to   default, can't start with "kube-," and must match this regular  expression: ^[a-z0-9]([-a-z0-9]*[a-z0-9])?$. For more information, see Namespaces in the Kubernetes documentation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KubernetesNamespace")]
    pub kubernetes_namespace: String,

}



impl cfn_resources::CfnResource for EksConfiguration {
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

/// An object that represents a launch template that's associated with a compute resource. You  must specify either the launch template ID or launch template name in the request, but not  both.
///
/// If security groups are specified using both the securityGroupIds parameter of   CreateComputeEnvironment and the launch template, the values in the   securityGroupIds parameter of CreateComputeEnvironment will be  used.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchTemplateSpecification {


    /// 
    /// The ID of the launch template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,


    /// 
    /// The name of the launch template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,


    /// 
    /// The version number of the launch template, $Latest, or  $Default.
    /// 
    /// If the value is $Latest, the latest version of the launch template is used. If  the value is $Default, the default version of the launch template is used.
    /// 
    /// ImportantIf the AMI ID that's used in a compute environment is from the launch template, the AMI   isn't changed when the compute environment is updated. It's only changed if the   updateToLatestImageVersion parameter for the compute environment is set to   true. During an infrastructure update, if either $Latest or   $Default is specified, AWS Batch re-evaluates the launch template version, and it   might use a different version of the launch template. This is the case even if the launch   template isn't specified in the update. When updating a compute environment, changing the launch   template requires an infrastructure update of the compute environment. For more information, see   Updating compute   environments in the          AWS Batch User Guide.
    /// 
    /// Default: $Default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Version")]
    pub version: Option<String>,

}



impl cfn_resources::CfnResource for LaunchTemplateSpecification {
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

/// Specifies the infrastructure update policy for the compute environment. For more information  about infrastructure updates, see Updating compute environments in the           AWS Batch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePolicy {


    /// 
    /// Specifies the job timeout (in minutes) when the compute environment infrastructure is  updated. The default value is 30.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobExecutionTimeoutMinutes")]
    pub job_execution_timeout_minutes: Option<i64>,


    /// 
    /// Specifies whether jobs are automatically terminated when the computer environment  infrastructure is updated. The default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminateJobsOnUpdate")]
    pub terminate_jobs_on_update: Option<bool>,

}



impl cfn_resources::CfnResource for UpdatePolicy {
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