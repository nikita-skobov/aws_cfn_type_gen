
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// The metadata that you apply to CodeDeploy applications to help you organize and categorize them. Each tag consists of a key and an optional value, both of which you define.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The compute platform that CodeDeploy deploys the application to.
    #[serde(rename = "ComputePlatform")]
    pub compute_platform: Option<String>,
    /// A name for the application. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the application name.
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_deployment_config {

#[derive(serde::Serialize, Default)]
pub struct CfnDeploymentConfig {
    /// The minimum number of healthy instances that should be available at any time during the deployment. There are two parameters expected in the input: type and value.
    #[serde(rename = "MinimumHealthyHosts")]
    pub minimum_healthy_hosts: Option<MinimumHealthyHosts>,
    /// The configuration that specifies how the deployment traffic is routed.
    #[serde(rename = "TrafficRoutingConfig")]
    pub traffic_routing_config: Option<TrafficRoutingConfig>,
    /// A name for the deployment configuration. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the deployment configuration name. For more information, see Name Type.
    #[serde(rename = "DeploymentConfigName")]
    pub deployment_config_name: Option<String>,
    /// The destination platform type for the deployment (Lambda, Server, or ECS).
    #[serde(rename = "ComputePlatform")]
    pub compute_platform: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct TimeBasedLinear {
    #[serde(rename = "LinearPercentage")]
    pub linear_percentage: usize,
    #[serde(rename = "LinearInterval")]
    pub linear_interval: usize,

}

#[derive(serde::Serialize, Default)]
pub struct MinimumHealthyHosts {
    #[serde(rename = "Value")]
    pub value: usize,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct TrafficRoutingConfig {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "TimeBasedCanary")]
    pub time_based_canary: Option<TimeBasedCanary>,
    #[serde(rename = "TimeBasedLinear")]
    pub time_based_linear: Option<TimeBasedLinear>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeBasedCanary {
    #[serde(rename = "CanaryPercentage")]
    pub canary_percentage: usize,
    #[serde(rename = "CanaryInterval")]
    pub canary_interval: usize,

}


}

pub mod cfn_deployment_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDeploymentGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Deployment")]
    pub deployment: Option<Deployment>,
    /// List of EC2TagFilter
    #[serde(rename = "Ec2TagFilters")]
    pub ec2_tag_filters: Option<Vec<EC2TagFilter>>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentStyle")]
    pub deployment_style: Option<DeploymentStyle>,
    /// List of TriggerConfig
    #[serde(rename = "TriggerConfigurations")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
    /// No documentation provided by AWS
    #[serde(rename = "Ec2TagSet")]
    pub ec2_tag_set: Option<EC2TagSet>,
    /// No documentation provided by AWS
    #[serde(rename = "OnPremisesTagSet")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    /// List of ECSService
    #[serde(rename = "ECSServices")]
    pub ecsservices: Option<Vec<ECSService>>,
    /// List of TagFilter
    #[serde(rename = "OnPremisesInstanceTagFilters")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentGroupName")]
    pub deployment_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "LoadBalancerInfo")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoScalingGroups")]
    pub auto_scaling_groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "BlueGreenDeploymentConfiguration")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "AlarmConfiguration")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoRollbackConfiguration")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "OutdatedInstancesStrategy")]
    pub outdated_instances_strategy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentConfigName")]
    pub deployment_config_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationName")]
    pub application_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct ELBInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GreenFleetProvisioningOption {
    #[serde(rename = "Action")]
    pub action: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EC2TagSetListObject {
    #[serde(rename = "Ec2TagGroup")]
    pub ec2_tag_group: Option<Vec<EC2TagFilter>>,

}

#[derive(serde::Serialize, Default)]
pub struct AlarmConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "Alarms")]
    pub alarms: Option<Vec<Alarm>>,
    #[serde(rename = "IgnorePollAlarmFailure")]
    pub ignore_poll_alarm_failure: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct GitHubLocation {
    #[serde(rename = "Repository")]
    pub repository: String,
    #[serde(rename = "CommitId")]
    pub commit_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct BlueGreenDeploymentConfiguration {
    #[serde(rename = "DeploymentReadyOption")]
    pub deployment_ready_option: Option<DeploymentReadyOption>,
    #[serde(rename = "TerminateBlueInstancesOnDeploymentSuccess")]
    pub terminate_blue_instances_on_deployment_success: Option<BlueInstanceTerminationOption>,
    #[serde(rename = "GreenFleetProvisioningOption")]
    pub green_fleet_provisioning_option: Option<GreenFleetProvisioningOption>,

}

#[derive(serde::Serialize, Default)]
pub struct AutoRollbackConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "Events")]
    pub events: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentReadyOption {
    #[serde(rename = "WaitTimeInMinutes")]
    pub wait_time_in_minutes: Option<usize>,
    #[serde(rename = "ActionOnTimeout")]
    pub action_on_timeout: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EC2TagSet {
    #[serde(rename = "Ec2TagSetList")]
    pub ec2_tag_set_list: Option<Vec<EC2TagSetListObject>>,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentStyle {
    #[serde(rename = "DeploymentType")]
    pub deployment_type: Option<String>,
    #[serde(rename = "DeploymentOption")]
    pub deployment_option: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ECSService {
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "ETag")]
    pub etag: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "BundleType")]
    pub bundle_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Deployment {
    #[serde(rename = "Revision")]
    pub revision: RevisionLocation,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "IgnoreApplicationStopFailures")]
    pub ignore_application_stop_failures: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct TagFilter {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LoadBalancerInfo {
    #[serde(rename = "TargetGroupPairInfoList")]
    pub target_group_pair_info_list: Option<Vec<TargetGroupPairInfo>>,
    #[serde(rename = "ElbInfoList")]
    pub elb_info_list: Option<Vec<ELBInfo>>,
    #[serde(rename = "TargetGroupInfoList")]
    pub target_group_info_list: Option<Vec<TargetGroupInfo>>,

}

#[derive(serde::Serialize, Default)]
pub struct OnPremisesTagSet {
    #[serde(rename = "OnPremisesTagSetList")]
    pub on_premises_tag_set_list: Option<Vec<OnPremisesTagSetListObject>>,

}

#[derive(serde::Serialize, Default)]
pub struct RevisionLocation {
    #[serde(rename = "GitHubLocation")]
    pub git_hub_location: Option<GitHubLocation>,
    #[serde(rename = "RevisionType")]
    pub revision_type: Option<String>,
    #[serde(rename = "S3Location")]
    pub s3_location: Option<S3Location>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupPairInfo {
    #[serde(rename = "TargetGroups")]
    pub target_groups: Option<Vec<TargetGroupInfo>>,
    #[serde(rename = "ProdTrafficRoute")]
    pub prod_traffic_route: Option<TrafficRoute>,
    #[serde(rename = "TestTrafficRoute")]
    pub test_traffic_route: Option<TrafficRoute>,

}

#[derive(serde::Serialize, Default)]
pub struct Alarm {
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct TriggerConfig {
    #[serde(rename = "TriggerEvents")]
    pub trigger_events: Option<Vec<String>>,
    #[serde(rename = "TriggerTargetArn")]
    pub trigger_target_arn: Option<String>,
    #[serde(rename = "TriggerName")]
    pub trigger_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BlueInstanceTerminationOption {
    #[serde(rename = "TerminationWaitTimeInMinutes")]
    pub termination_wait_time_in_minutes: Option<usize>,
    #[serde(rename = "Action")]
    pub action: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EC2TagFilter {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OnPremisesTagSetListObject {
    #[serde(rename = "OnPremisesTagGroup")]
    pub on_premises_tag_group: Option<Vec<TagFilter>>,

}

#[derive(serde::Serialize, Default)]
pub struct TrafficRoute {
    #[serde(rename = "ListenerArns")]
    pub listener_arns: Option<Vec<String>>,

}


}
