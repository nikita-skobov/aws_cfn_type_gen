
pub mod cfn_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCluster {
    /// No documentation provided by AWS
    #[serde(rename = "OSReleaseLabel")]
    pub osrelease_label: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,
    /// List of BootstrapActionConfig
    #[serde(rename = "BootstrapActions")]
    pub bootstrap_actions: Option<Vec<BootstrapActionConfig>>,
    /// No documentation provided by AWS
    #[serde(rename = "ManagedScalingPolicy")]
    pub managed_scaling_policy: Option<ManagedScalingPolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "Instances")]
    pub instances: JobFlowInstancesConfig,
    /// No documentation provided by AWS
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ReleaseLabel")]
    pub release_label: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KerberosAttributes")]
    pub kerberos_attributes: Option<KerberosAttributes>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoScalingRole")]
    pub auto_scaling_role: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ScaleDownBehavior")]
    pub scale_down_behavior: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EbsRootVolumeSize")]
    pub ebs_root_volume_size: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoTerminationPolicy")]
    pub auto_termination_policy: Option<AutoTerminationPolicy>,
    /// List of StepConfig
    #[serde(rename = "Steps")]
    pub steps: Option<Vec<StepConfig>>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceRole")]
    pub service_role: String,
    /// No documentation provided by AWS
    #[serde(rename = "VisibleToAllUsers")]
    pub visible_to_all_users: Option<bool>,
    /// List of Application
    #[serde(rename = "Applications")]
    pub applications: Option<Vec<Application>>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LogEncryptionKmsKeyId")]
    pub log_encryption_kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StepConcurrencyLevel")]
    pub step_concurrency_level: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "LogUri")]
    pub log_uri: Option<String>,
    /// List of Configuration
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "JobFlowRole")]
    pub job_flow_role: String,

}


#[derive(serde::Serialize, Default)]
pub struct CloudWatchAlarmDefinition {
    #[serde(rename = "Period")]
    pub period: usize,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "Statistic")]
    pub statistic: Option<String>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: Option<usize>,
    #[serde(rename = "Threshold")]
    pub threshold: f64,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceFleetProvisioningSpecifications {
    #[serde(rename = "OnDemandSpecification")]
    pub on_demand_specification: Option<OnDemandProvisioningSpecification>,
    #[serde(rename = "SpotSpecification")]
    pub spot_specification: Option<SpotProvisioningSpecification>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceGroupConfig {
    #[serde(rename = "BidPrice")]
    pub bid_price: Option<String>,
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "Market")]
    pub market: Option<String>,
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,
    #[serde(rename = "AutoScalingPolicy")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,
    #[serde(rename = "EbsConfiguration")]
    pub ebs_configuration: Option<EbsConfiguration>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SimpleScalingPolicyConfiguration {
    #[serde(rename = "AdjustmentType")]
    pub adjustment_type: Option<String>,
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: usize,
    #[serde(rename = "CoolDown")]
    pub cool_down: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotProvisioningSpecification {
    #[serde(rename = "BlockDurationMinutes")]
    pub block_duration_minutes: Option<usize>,
    #[serde(rename = "TimeoutAction")]
    pub timeout_action: String,
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "TimeoutDurationMinutes")]
    pub timeout_duration_minutes: usize,

}

#[derive(serde::Serialize, Default)]
pub struct EbsBlockDeviceConfig {
    #[serde(rename = "VolumesPerInstance")]
    pub volumes_per_instance: Option<usize>,
    #[serde(rename = "VolumeSpecification")]
    pub volume_specification: VolumeSpecification,

}

#[derive(serde::Serialize, Default)]
pub struct OnDemandProvisioningSpecification {
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: String,

}

#[derive(serde::Serialize, Default)]
pub struct ManagedScalingPolicy {
    #[serde(rename = "ComputeLimits")]
    pub compute_limits: Option<ComputeLimits>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricDimension {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct PlacementType {
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,

}

#[derive(serde::Serialize, Default)]
pub struct Configuration {
    #[serde(rename = "Classification")]
    pub classification: Option<String>,
    #[serde(rename = "ConfigurationProperties")]
    pub configuration_properties: Option<()>,
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,

}

#[derive(serde::Serialize, Default)]
pub struct ScriptBootstrapActionConfig {
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Path")]
    pub path: String,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceFleetConfig {
    #[serde(rename = "InstanceTypeConfigs")]
    pub instance_type_configs: Option<Vec<InstanceTypeConfig>>,
    #[serde(rename = "TargetSpotCapacity")]
    pub target_spot_capacity: Option<usize>,
    #[serde(rename = "TargetOnDemandCapacity")]
    pub target_on_demand_capacity: Option<usize>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "LaunchSpecifications")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,

}

#[derive(serde::Serialize, Default)]
pub struct AutoScalingPolicy {
    #[serde(rename = "Rules")]
    pub rules: Vec<ScalingRule>,
    #[serde(rename = "Constraints")]
    pub constraints: ScalingConstraints,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingTrigger {
    #[serde(rename = "CloudWatchAlarmDefinition")]
    pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,

}

#[derive(serde::Serialize, Default)]
pub struct ComputeLimits {
    #[serde(rename = "MaximumOnDemandCapacityUnits")]
    pub maximum_on_demand_capacity_units: Option<usize>,
    #[serde(rename = "MaximumCapacityUnits")]
    pub maximum_capacity_units: usize,
    #[serde(rename = "MinimumCapacityUnits")]
    pub minimum_capacity_units: usize,
    #[serde(rename = "UnitType")]
    pub unit_type: String,
    #[serde(rename = "MaximumCoreCapacityUnits")]
    pub maximum_core_capacity_units: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingConstraints {
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: usize,
    #[serde(rename = "MinCapacity")]
    pub min_capacity: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingAction {
    #[serde(rename = "Market")]
    pub market: Option<String>,
    #[serde(rename = "SimpleScalingPolicyConfiguration")]
    pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct VolumeSpecification {
    #[serde(rename = "VolumeType")]
    pub volume_type: String,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "SizeInGB")]
    pub size_in_gb: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingRule {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Trigger")]
    pub trigger: ScalingTrigger,
    #[serde(rename = "Action")]
    pub action: ScalingAction,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct KerberosAttributes {
    #[serde(rename = "KdcAdminPassword")]
    pub kdc_admin_password: String,
    #[serde(rename = "CrossRealmTrustPrincipalPassword")]
    pub cross_realm_trust_principal_password: Option<String>,
    #[serde(rename = "Realm")]
    pub realm: String,
    #[serde(rename = "ADDomainJoinUser")]
    pub addomain_join_user: Option<String>,
    #[serde(rename = "ADDomainJoinPassword")]
    pub addomain_join_password: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KeyValue {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EbsConfiguration {
    #[serde(rename = "EbsBlockDeviceConfigs")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct AutoTerminationPolicy {
    #[serde(rename = "IdleTimeout")]
    pub idle_timeout: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Application {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: Option<()>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct StepConfig {
    #[serde(rename = "HadoopJarStep")]
    pub hadoop_jar_step: HadoopJarStepConfig,
    #[serde(rename = "ActionOnFailure")]
    pub action_on_failure: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct HadoopJarStepConfig {
    #[serde(rename = "StepProperties")]
    pub step_properties: Option<Vec<KeyValue>>,
    #[serde(rename = "MainClass")]
    pub main_class: Option<String>,
    #[serde(rename = "Jar")]
    pub jar: String,
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct JobFlowInstancesConfig {
    #[serde(rename = "HadoopVersion")]
    pub hadoop_version: Option<String>,
    #[serde(rename = "KeepJobFlowAliveWhenNoSteps")]
    pub keep_job_flow_alive_when_no_steps: Option<bool>,
    #[serde(rename = "CoreInstanceGroup")]
    pub core_instance_group: Option<InstanceGroupConfig>,
    #[serde(rename = "Placement")]
    pub placement: Option<PlacementType>,
    #[serde(rename = "EmrManagedMasterSecurityGroup")]
    pub emr_managed_master_security_group: Option<String>,
    #[serde(rename = "ServiceAccessSecurityGroup")]
    pub service_access_security_group: Option<String>,
    #[serde(rename = "MasterInstanceFleet")]
    pub master_instance_fleet: Option<InstanceFleetConfig>,
    #[serde(rename = "TaskInstanceGroups")]
    pub task_instance_groups: Option<Vec<InstanceGroupConfig>>,
    #[serde(rename = "Ec2KeyName")]
    pub ec2_key_name: Option<String>,
    #[serde(rename = "MasterInstanceGroup")]
    pub master_instance_group: Option<InstanceGroupConfig>,
    #[serde(rename = "AdditionalSlaveSecurityGroups")]
    pub additional_slave_security_groups: Option<Vec<String>>,
    #[serde(rename = "CoreInstanceFleet")]
    pub core_instance_fleet: Option<InstanceFleetConfig>,
    #[serde(rename = "TerminationProtected")]
    pub termination_protected: Option<bool>,
    #[serde(rename = "Ec2SubnetIds")]
    pub ec2_subnet_ids: Option<Vec<String>>,
    #[serde(rename = "EmrManagedSlaveSecurityGroup")]
    pub emr_managed_slave_security_group: Option<String>,
    #[serde(rename = "AdditionalMasterSecurityGroups")]
    pub additional_master_security_groups: Option<Vec<String>>,
    #[serde(rename = "TaskInstanceFleets")]
    pub task_instance_fleets: Option<Vec<InstanceFleetConfig>>,
    #[serde(rename = "Ec2SubnetId")]
    pub ec2_subnet_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceTypeConfig {
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<usize>,
    #[serde(rename = "BidPrice")]
    pub bid_price: Option<String>,
    #[serde(rename = "EbsConfiguration")]
    pub ebs_configuration: Option<EbsConfiguration>,
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct BootstrapActionConfig {
    #[serde(rename = "ScriptBootstrapAction")]
    pub script_bootstrap_action: ScriptBootstrapActionConfig,
    #[serde(rename = "Name")]
    pub name: String,

}


}

pub mod cfn_instance_fleet_config {

#[derive(serde::Serialize, Default)]
pub struct CfnInstanceFleetConfig {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// List of InstanceTypeConfig
    #[serde(rename = "InstanceTypeConfigs")]
    pub instance_type_configs: Option<Vec<InstanceTypeConfig>>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceFleetType")]
    pub instance_fleet_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "TargetSpotCapacity")]
    pub target_spot_capacity: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "LaunchSpecifications")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetOnDemandCapacity")]
    pub target_on_demand_capacity: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct EbsConfiguration {
    #[serde(rename = "EbsBlockDeviceConfigs")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct OnDemandProvisioningSpecification {
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: String,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceFleetProvisioningSpecifications {
    #[serde(rename = "SpotSpecification")]
    pub spot_specification: Option<SpotProvisioningSpecification>,
    #[serde(rename = "OnDemandSpecification")]
    pub on_demand_specification: Option<OnDemandProvisioningSpecification>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceTypeConfig {
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<usize>,
    #[serde(rename = "EbsConfiguration")]
    pub ebs_configuration: Option<EbsConfiguration>,
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,
    #[serde(rename = "BidPrice")]
    pub bid_price: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VolumeSpecification {
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "SizeInGB")]
    pub size_in_gb: usize,
    #[serde(rename = "VolumeType")]
    pub volume_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct EbsBlockDeviceConfig {
    #[serde(rename = "VolumesPerInstance")]
    pub volumes_per_instance: Option<usize>,
    #[serde(rename = "VolumeSpecification")]
    pub volume_specification: VolumeSpecification,

}

#[derive(serde::Serialize, Default)]
pub struct SpotProvisioningSpecification {
    #[serde(rename = "BlockDurationMinutes")]
    pub block_duration_minutes: Option<usize>,
    #[serde(rename = "TimeoutAction")]
    pub timeout_action: String,
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "TimeoutDurationMinutes")]
    pub timeout_duration_minutes: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Configuration {
    #[serde(rename = "ConfigurationProperties")]
    pub configuration_properties: Option<()>,
    #[serde(rename = "Classification")]
    pub classification: Option<String>,
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,

}


}

pub mod cfn_instance_group_config {

#[derive(serde::Serialize, Default)]
pub struct CfnInstanceGroupConfig {
    /// No documentation provided by AWS
    #[serde(rename = "InstanceRole")]
    pub instance_role: String,
    /// No documentation provided by AWS
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "Market")]
    pub market: Option<String>,
    /// List of Configuration
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,
    /// No documentation provided by AWS
    #[serde(rename = "AutoScalingPolicy")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "EbsConfiguration")]
    pub ebs_configuration: Option<EbsConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "BidPrice")]
    pub bid_price: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct CloudWatchAlarmDefinition {
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "Period")]
    pub period: usize,
    #[serde(rename = "Statistic")]
    pub statistic: Option<String>,
    #[serde(rename = "Threshold")]
    pub threshold: f64,
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VolumeSpecification {
    #[serde(rename = "SizeInGB")]
    pub size_in_gb: usize,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "VolumeType")]
    pub volume_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingAction {
    #[serde(rename = "Market")]
    pub market: Option<String>,
    #[serde(rename = "SimpleScalingPolicyConfiguration")]
    pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingTrigger {
    #[serde(rename = "CloudWatchAlarmDefinition")]
    pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,

}

#[derive(serde::Serialize, Default)]
pub struct AutoScalingPolicy {
    #[serde(rename = "Rules")]
    pub rules: Vec<ScalingRule>,
    #[serde(rename = "Constraints")]
    pub constraints: ScalingConstraints,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingConstraints {
    #[serde(rename = "MinCapacity")]
    pub min_capacity: usize,
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingRule {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Trigger")]
    pub trigger: ScalingTrigger,
    #[serde(rename = "Action")]
    pub action: ScalingAction,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Configuration {
    #[serde(rename = "Classification")]
    pub classification: Option<String>,
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(rename = "ConfigurationProperties")]
    pub configuration_properties: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct EbsBlockDeviceConfig {
    #[serde(rename = "VolumesPerInstance")]
    pub volumes_per_instance: Option<usize>,
    #[serde(rename = "VolumeSpecification")]
    pub volume_specification: VolumeSpecification,

}

#[derive(serde::Serialize, Default)]
pub struct SimpleScalingPolicyConfiguration {
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: usize,
    #[serde(rename = "AdjustmentType")]
    pub adjustment_type: Option<String>,
    #[serde(rename = "CoolDown")]
    pub cool_down: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct EbsConfiguration {
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "EbsBlockDeviceConfigs")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricDimension {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_security_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityConfiguration {
    /// The security configuration details in JSON format.
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: (),
    /// The name of the security configuration.
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



}

pub mod cfn_step {

#[derive(serde::Serialize, Default)]
pub struct CfnStep {
    /// No documentation provided by AWS
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ActionOnFailure")]
    pub action_on_failure: String,
    /// No documentation provided by AWS
    #[serde(rename = "HadoopJarStep")]
    pub hadoop_jar_step: HadoopJarStepConfig,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct HadoopJarStepConfig {
    #[serde(rename = "MainClass")]
    pub main_class: Option<String>,
    #[serde(rename = "StepProperties")]
    pub step_properties: Option<Vec<KeyValue>>,
    #[serde(rename = "Jar")]
    pub jar: String,
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct KeyValue {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_studio {

#[derive(serde::Serialize, Default)]
pub struct CfnStudio {
    /// A descriptive name for the Amazon EMR Studio.
    #[serde(rename = "Name")]
    pub name: String,
    /// A list of tags to associate with the Studio. Tags are user-defined key-value pairs that consist of a required key string with a maximum of 128 characters, and an optional value string with a maximum of 256 characters.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The default Amazon S3 location to back up EMR Studio Workspaces and notebook files. A Studio user can select an alternative Amazon S3 location when creating a Workspace.
    #[serde(rename = "DefaultS3Location")]
    pub default_s3_location: String,
    /// The IAM role that will be assumed by the Amazon EMR Studio. The service role provides a way for Amazon EMR Studio to interoperate with other AWS services.
    #[serde(rename = "ServiceRole")]
    pub service_role: Arn,
    /// Your identity provider's authentication endpoint. Amazon EMR Studio redirects federated users to this endpoint for authentication when logging in to a Studio with the Studio URL.
    #[serde(rename = "IdpAuthUrl")]
    pub idp_auth_url: Option<String>,
    /// The name of relay state parameter for external Identity Provider.
    #[serde(rename = "IdpRelayStateParameterName")]
    pub idp_relay_state_parameter_name: Option<String>,
    /// The ID of the Amazon EMR Studio Workspace security group. The Workspace security group allows outbound network traffic to resources in the Engine security group, and it must be in the same VPC specified by VpcId.
    #[serde(rename = "WorkspaceSecurityGroupId")]
    pub workspace_security_group_id: String,
    /// The ID of the Amazon EMR Studio Engine security group. The Engine security group allows inbound network traffic from the Workspace security group, and it must be in the same VPC specified by VpcId.
    #[serde(rename = "EngineSecurityGroupId")]
    pub engine_security_group_id: String,
    /// Specifies whether the Studio authenticates users using single sign-on (SSO) or IAM. Amazon EMR Studio currently only supports SSO authentication.
    #[serde(rename = "AuthMode")]
    pub auth_mode: String,
    /// A list of up to 5 subnet IDs to associate with the Studio. The subnets must belong to the VPC specified by VpcId. Studio users can create a Workspace in any of the specified subnets.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<SubnetId>,
    /// The IAM user role that will be assumed by users and groups logged in to a Studio. The permissions attached to this IAM role can be scoped down for each user or group using session policies.
    #[serde(rename = "UserRole")]
    pub user_role: Option<Arn>,
    /// The ID of the Amazon Virtual Private Cloud (Amazon VPC) to associate with the Studio.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// A detailed description of the Studio.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type SubnetId = String;
#[derive(serde::Serialize, Default)]
pub struct Tags {

}
pub type Arn = String;

}

pub mod cfn_studio_session_mapping {

#[derive(serde::Serialize, Default)]
pub struct CfnStudioSessionMapping {
    /// The Amazon Resource Name (ARN) for the session policy that will be applied to the user or group. Session policies refine Studio user permissions without the need to use multiple IAM user roles.
    #[serde(rename = "SessionPolicyArn")]
    pub session_policy_arn: IamPolicyArn,
    /// Specifies whether the identity to map to the Studio is a user or a group.
    #[serde(rename = "IdentityType")]
    pub identity_type: String,
    /// The ID of the Amazon EMR Studio to which the user or group will be mapped.
    #[serde(rename = "StudioId")]
    pub studio_id: String,
    /// The name of the user or group. For more information, see UserName and DisplayName in the AWS SSO Identity Store API Reference. Either IdentityName or IdentityId must be specified.
    #[serde(rename = "IdentityName")]
    pub identity_name: String,

}

pub type IamPolicyArn = String;

}
