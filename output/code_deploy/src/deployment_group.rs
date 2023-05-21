

/// The AWS::CodeDeploy::DeploymentGroup resource creates an AWS CodeDeploy deployment group that specifies which instances your application revisions    are deployed to, along with other deployment options. For more information, see CreateDeploymentGroup in the CodeDeploy API     Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeploymentGroup {


    /// 
    /// Information about the load balancer to use in a deployment. For more information, see     Integrating      CodeDeploy with Elastic Load Balancing in the AWS CodeDeploy User Guide.
    /// 
    /// Required: No
    ///
    /// Type: LoadBalancerInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerInfo")]
    pub load_balancer_info: Option<LoadBalancerInfo>,


    /// 
    /// A deployment configuration name or a predefined configuration name. With predefined    configurations, you can deploy application revisions to one instance at a time     (CodeDeployDefault.OneAtATime), half of the instances at a time     (CodeDeployDefault.HalfAtATime), or all the instances at once     (CodeDeployDefault.AllAtOnce). For more information and valid values, see     Working with Deployment Configurations in the AWS CodeDeploy     User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentConfigName")]
    pub deployment_config_name: Option<String>,


    /// 
    /// The target Amazon ECS services in the deployment group. This applies only to       deployment groups that use the Amazon ECS compute platform. A target Amazon ECS service is specified as an Amazon ECS cluster and service name       pair using the format <clustername>:<servicename>.
    /// 
    /// Required: No
    ///
    /// Type: List of ECSService
    ///
    /// Update requires: No interruption
    #[serde(rename = "ECSServices")]
    pub ecsservices: Option<Vec<ECSService>>,


    /// 
    /// A service role Amazon Resource Name (ARN) that grants CodeDeploy permission to    make calls to AWS services on your behalf. For more information, see Create a Service     Role for AWS CodeDeploy in the AWS CodeDeploy User     Guide.
    /// 
    /// Note In some cases, you might need to add a dependency on the service role's policy. For     more information, see IAM role policy in DependsOn      Attribute.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,


    /// 
    /// Information about groups of tags applied to on-premises instances. The deployment group    includes only on-premises instances identified by all the tag groups.
    /// 
    /// You can specify OnPremisesInstanceTagFilters or     OnPremisesInstanceTagSet, but not both.
    /// 
    /// Required: No
    ///
    /// Type: OnPremisesTagSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnPremisesTagSet")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Information about triggers associated with the deployment group. Duplicates are not    allowed
    /// 
    /// Required: No
    ///
    /// Type: List of TriggerConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerConfigurations")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,


    /// 
    /// Information about the Amazon CloudWatch alarms that are associated with the    deployment group.
    /// 
    /// Required: No
    ///
    /// Type: AlarmConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmConfiguration")]
    pub alarm_configuration: Option<AlarmConfiguration>,


    /// 
    /// The Amazon EC2 tags that are already applied to Amazon EC2 instances    that you want to include in the deployment group. CodeDeploy includes all Amazon EC2 instances identified by any of the tags you specify in this deployment group.    Duplicates are not allowed.
    /// 
    /// You can specify EC2TagFilters or Ec2TagSet, but not both.
    /// 
    /// Required: No
    ///
    /// Type: List of EC2TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ec2TagFilters")]
    pub ec2_tag_filters: Option<Vec<EC2TagFilter>>,


    /// 
    /// Information about groups of tags applied to Amazon EC2 instances. The deployment    group includes only Amazon EC2 instances identified by all the tag groups. Cannot be    used in the same call as ec2TagFilter.
    /// 
    /// Required: No
    ///
    /// Type: EC2TagSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ec2TagSet")]
    pub ec2_tag_set: Option<EC2TagSet>,


    /// 
    /// A name for the deployment group. If you don't specify a name, AWS CloudFormation    generates a unique physical ID and uses that ID for the deployment group name. For more    information, see Name Type.
    /// 
    /// Important If you specify a name, you cannot perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you must replace     the resource, specify a new name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentGroupName")]
    pub deployment_group_name: Option<String>,


    /// 
    /// A list of associated Auto Scaling groups that CodeDeploy automatically    deploys revisions to when new instances are created. Duplicates are not allowed.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingGroups")]
    pub auto_scaling_groups: Option<Vec<String>>,


    /// 
    /// Information about blue/green deployment options for a deployment group.
    /// 
    /// Required: No
    ///
    /// Type: BlueGreenDeploymentConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlueGreenDeploymentConfiguration")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,


    /// 
    /// Information about the automatic rollback configuration that is associated with the    deployment group. If you specify this property, don't specify the Deployment    property.
    /// 
    /// Required: No
    ///
    /// Type: AutoRollbackConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoRollbackConfiguration")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,


    /// 
    /// Attributes that determine the type of deployment to run and whether to route deployment    traffic behind a load balancer.
    /// 
    /// If you specify this property with a blue/green deployment type, don't specify the     AutoScalingGroups, LoadBalancerInfo, or Deployment    properties.
    /// 
    /// Note For blue/green deployments, AWS CloudFormation supports deployments on Lambda compute platforms only. You can perform Amazon ECS blue/green     deployments using AWS::CodeDeploy::BlueGreen hook. See Perform       Amazon ECS blue/green deployments through CodeDeploy using AWS CloudFormation for more information.
    /// 
    /// Required: No
    ///
    /// Type: DeploymentStyle
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentStyle")]
    pub deployment_style: Option<DeploymentStyle>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutdatedInstancesStrategy")]
    pub outdated_instances_strategy: Option<String>,


    /// 
    /// The name of an existing CodeDeploy application to associate this deployment    group with.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: String,


    /// 
    /// The on-premises instance tags already applied to on-premises instances that you want to    include in the deployment group. CodeDeploy includes all on-premises instances    identified by any of the tags you specify in this deployment group. To register on-premises    instances with CodeDeploy, see Working with On-Premises     Instances for CodeDeploy in the AWS CodeDeploy User     Guide. Duplicates are not allowed.
    /// 
    /// You can specify OnPremisesInstanceTagFilters or     OnPremisesInstanceTagSet, but not both.
    /// 
    /// Required: No
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnPremisesInstanceTagFilters")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,


    /// 
    /// The application revision to deploy to this deployment group. If you specify this    property, your target application revision is deployed as soon as the provisioning process is    complete. If you specify this property, don't specify the     AutoRollbackConfiguration property.
    /// 
    /// Required: No
    ///
    /// Type: Deployment
    ///
    /// Update requires: No interruption
    #[serde(rename = "Deployment")]
    pub deployment: Option<Deployment>,

}

impl cfn_resources::CfnResource for CfnDeploymentGroup {
    fn type_string() -> &'static str {
        "AWS::CodeDeploy::DeploymentGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// RevisionLocation is a property that defines the location of the CodeDeploy application revision to deploy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RevisionLocation {


    /// 
    /// The type of application revision:
    /// 
    /// S3: An application revision stored in Amazon S3.               GitHub: An application revision stored in GitHub (EC2/On-premises deployments           only).               String: A YAML-formatted or JSON-formatted string (AWS Lambda           deployments only).               AppSpecContent: An AppSpecContent object that contains the           contents of an AppSpec file for an AWS Lambda or Amazon ECS           deployment. The content is formatted as JSON or YAML stored as a           RawString.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AppSpecContent | GitHub | S3 | String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RevisionType")]
    pub revision_type: Option<String>,


    /// 
    /// Information about the location of application artifacts stored in GitHub.
    /// 
    /// Required: No
    ///
    /// Type: GitHubLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "GitHubLocation")]
    pub git_hub_location: Option<GitHubLocation>,


    /// 
    /// Information about the location of a revision stored in Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Location")]
    pub s3_location: Option<S3Location>,

}


/// Information about how traffic is rerouted to instances in a replacement environment in       a blue/green deployment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentReadyOption {


    /// 
    /// The number of minutes to wait before the status of a blue/green deployment is changed       to Stopped if rerouting is not started manually. Applies only to the         STOP_DEPLOYMENT option for actionOnTimeout.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WaitTimeInMinutes")]
    pub wait_time_in_minutes: Option<i64>,


    /// 
    /// Information about when to reroute traffic from an original environment to a replacement    environment in a blue/green deployment.
    /// 
    /// CONTINUE_DEPLOYMENT: Register new instances with the load balancer immediately after      the new application revision is installed on the instances in the replacement      environment.        STOP_DEPLOYMENT: Do not register new instances with a load balancer unless traffic      rerouting is started using ContinueDeployment      . If traffic rerouting is not started before the end of the specified wait period,      the deployment status is changed to Stopped.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONTINUE_DEPLOYMENT | STOP_DEPLOYMENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionOnTimeout")]
    pub action_on_timeout: Option<String>,

}


/// The TargetGroupPairInfo property type specifies Property description not available. for an AWS::CodeDeploy::DeploymentGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetGroupPairInfo {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TrafficRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProdTrafficRoute")]
    pub prod_traffic_route: Option<TrafficRoute>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TrafficRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "TestTrafficRoute")]
    pub test_traffic_route: Option<TrafficRoute>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of TargetGroupInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroups")]
    pub target_groups: Option<Vec<TargetGroupInfo>>,

}


/// Information about an Amazon EC2 tag filter.
///
/// For more information about using tags and tag groups to help manage your Amazon EC2 instances and on-premises instances, see Tagging Instances for Deployment     Groups in AWS CodeDeploy in the AWS CodeDeploy User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EC2TagFilter {


    /// 
    /// The tag filter value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The tag filter key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The tag filter type:
    /// 
    /// KEY_ONLY: Key only.                        VALUE_ONLY: Value only.                        KEY_AND_VALUE: Key and value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: KEY_AND_VALUE | KEY_ONLY | VALUE_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}


/// Information about notification triggers for the deployment group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TriggerConfig {


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon Simple Notification Service topic through       which notifications about deployment or instance events are sent.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerTargetArn")]
    pub trigger_target_arn: Option<String>,


    /// 
    /// The name of the notification trigger.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerName")]
    pub trigger_name: Option<String>,


    /// 
    /// The event type or types that trigger notifications.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerEvents")]
    pub trigger_events: Option<Vec<String>>,

}


/// TagFilter is a property type of the AWS::CodeDeploy::DeploymentGroup resource that specifies which on-premises    instances to associate with the deployment group. To register on-premise instances with     AWS CodeDeploy, see Configure Existing On-Premises     Instances by Using AWS CodeDeploy in the AWS CodeDeploy User Guide.
///
/// For more information about using tags and tag groups to help manage your Amazon EC2 instances and on-premises instances, see Tagging Instances for Deployment     Groups in AWS CodeDeploy in the AWS CodeDeploy User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagFilter {


    /// 
    /// The on-premises instance tag filter value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The on-premises instance tag filter type:
    /// 
    /// KEY_ONLY: Key only.               VALUE_ONLY: Value only.               KEY_AND_VALUE: Key and value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: KEY_AND_VALUE | KEY_ONLY | VALUE_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The on-premises instance tag filter key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


/// Information about blue/green deployment options for a deployment group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlueGreenDeploymentConfiguration {


    /// 
    /// Information about whether to terminate instances in the original fleet during a       blue/green deployment.
    /// 
    /// Required: No
    ///
    /// Type: BlueInstanceTerminationOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminateBlueInstancesOnDeploymentSuccess")]
    pub terminate_blue_instances_on_deployment_success: Option<BlueInstanceTerminationOption>,


    /// 
    /// Information about how instances are provisioned for a replacement environment in a       blue/green deployment.
    /// 
    /// Required: No
    ///
    /// Type: GreenFleetProvisioningOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "GreenFleetProvisioningOption")]
    pub green_fleet_provisioning_option: Option<GreenFleetProvisioningOption>,


    /// 
    /// Information about the action to take when newly provisioned instances are ready to       receive traffic in a blue/green deployment.
    /// 
    /// Required: No
    ///
    /// Type: DeploymentReadyOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentReadyOption")]
    pub deployment_ready_option: Option<DeploymentReadyOption>,

}


/// The EC2TagSet property type specifies information about groups of tags    applied to Amazon EC2 instances. The deployment group includes only Amazon EC2 instances identified by all the tag groups. Cannot be used in the same template    as EC2TagFilters.
///
/// For more information about using tags and tag groups to help manage your Amazon EC2 instances and on-premises instances, see Tagging Instances for Deployment     Groups in AWS CodeDeploy in the AWS CodeDeploy User     Guide.
///
/// EC2TagSet is a property of the DeploymentGroup resource type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EC2TagSetListObject {


    /// 
    /// A list that contains other lists of Amazon EC2 instance tag groups. For an       instance to be included in the deployment group, it must be identified by all of the tag       groups in the list.
    /// 
    /// Required: No
    ///
    /// Type: List of EC2TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ec2TagGroup")]
    pub ec2_tag_group: Option<Vec<EC2TagFilter>>,

}


/// The ELBInfo property type specifies information about the Elastic Load Balancing load balancer used for an CodeDeploy deployment group.
///
/// If you specify the ELBInfo property, the     DeploymentStyle.DeploymentOption property must be set to     WITH_TRAFFIC_CONTROL for AWS CodeDeploy to route your traffic using    the specified load balancers.
///
/// ELBInfo is a property of the AWS CodeDeploy DeploymentGroup LoadBalancerInfo property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ELBInfo {


    /// 
    /// For blue/green deployments, the name of the load balancer that is used to route traffic    from original instances to replacement instances in a blue/green deployment. For in-place    deployments, the name of the load balancer that instances are deregistered from so they are    not serving traffic during a deployment, and then re-registered with after the deployment is    complete.
    /// 
    /// NoteAWS CloudFormation supports blue/green deployments on AWS Lambda compute     platforms only.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The OnPremisesTagSetListObject property type specifies lists of on-premises    instance tag groups. In order for an instance to be included in the deployment group, it must    be identified by all the tag groups in the list.
///
/// OnPremisesTagSetListObject is a property of the CodeDeploy DeploymentGroup OnPremisesTagSet property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnPremisesTagSetListObject {


    /// 
    /// Information about groups of on-premises instance tags.
    /// 
    /// Required: No
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnPremisesTagGroup")]
    pub on_premises_tag_group: Option<Vec<TagFilter>>,

}


/// Information about the type of deployment, either in-place or blue/green, you want to       run and whether to route deployment traffic behind a load balancer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentStyle {


    /// 
    /// Indicates whether to run an in-place or blue/green deployment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BLUE_GREEN | IN_PLACE
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentType")]
    pub deployment_type: Option<String>,


    /// 
    /// Indicates whether to route deployment traffic behind a load balancer.
    /// 
    /// Note An Amazon EC2     Application Load Balancer or Network Load Balancer is required for an Amazon ECS     deployment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: WITH_TRAFFIC_CONTROL | WITHOUT_TRAFFIC_CONTROL
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentOption")]
    pub deployment_option: Option<String>,

}


/// The AlarmConfiguration property type configures CloudWatch alarms    for an AWS CodeDeploy deployment group. AlarmConfiguration is a    property of the DeploymentGroup resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AlarmConfiguration {


    /// 
    /// A list of alarms configured for the deployment or deployment group. A maximum of 10       alarms can be added.
    /// 
    /// Required: No
    ///
    /// Type: List of Alarm
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alarms")]
    pub alarms: Option<Vec<Alarm>>,


    /// 
    /// Indicates whether the alarm configuration is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// Indicates whether a deployment should continue if information about the current state of    alarms cannot be retrieved from Amazon CloudWatch. The default value is     false.
    /// 
    /// true: The deployment proceeds even if alarm status information can't be      retrieved from CloudWatch.              false: The deployment stops if alarm status information can't be retrieved      from CloudWatch.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnorePollAlarmFailure")]
    pub ignore_poll_alarm_failure: Option<bool>,

}


/// S3Location is a property of the     CodeDeploy DeploymentGroup Revision property that specifies the location    of an application revision that is stored in Amazon Simple Storage Service (Amazon S3).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {


    /// 
    /// The ETag of the Amazon S3 object that represents the bundled artifacts for the       application revision.
    /// 
    /// If the ETag is not specified as an input parameter, ETag validation of the object is       skipped.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ETag")]
    pub etag: Option<String>,


    /// 
    /// A specific version of the Amazon S3 object that represents the bundled       artifacts for the application revision.
    /// 
    /// If the version is not specified, the system uses the most recent version by       default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// The file type of the application revision. Must be one of the following:
    /// 
    /// JSON        tar: A tar archive file.        tgz: A compressed tar archive file.        YAML        zip: A zip archive file.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: JSON | tar | tgz | YAML | zip
    ///
    /// Update requires: No interruption
    #[serde(rename = "BundleType")]
    pub bundle_type: Option<String>,


    /// 
    /// The name of the Amazon S3 bucket where the application revision is       stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,


    /// 
    /// The name of the Amazon S3 object that represents the bundled artifacts for the       application revision.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}


/// The LoadBalancerInfo property type specifies information about the load    balancer or target group used for an AWS CodeDeploy deployment group. For more    information, see Integrating      CodeDeploy with Elastic Load Balancing in the AWS CodeDeploy User Guide.
///
/// For AWS CloudFormation to use the properties specified in LoadBalancerInfo,    the DeploymentStyle.DeploymentOption property must be set to     WITH_TRAFFIC_CONTROL. If DeploymentStyle.DeploymentOption is not    set to WITH_TRAFFIC_CONTROL, AWS CloudFormation ignores any settings specified    in LoadBalancerInfo.
///
/// LoadBalancerInfo is a property of the DeploymentGroup resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoadBalancerInfo {


    /// 
    /// An array that contains information about the target group to use for load balancing in a    deployment. In Elastic Load Balancing, target groups are used with Application Load Balancers.
    /// 
    /// Note Adding more than one target group to the array is not supported.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of TargetGroupInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupInfoList")]
    pub target_group_info_list: Option<Vec<TargetGroupInfo>>,


    /// 
    /// An array that contains information about the load balancer to use for load balancing       in a deployment. In Elastic Load Balancing, load balancers are used with Classic Load       Balancers.
    /// 
    /// Note Adding more than one load balancer to the array is not supported.
    /// 
    /// Required: No
    ///
    /// Type: List of ELBInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElbInfoList")]
    pub elb_info_list: Option<Vec<ELBInfo>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of TargetGroupPairInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupPairInfoList")]
    pub target_group_pair_info_list: Option<Vec<TargetGroupPairInfo>>,

}


/// Deployment is a property of the DeploymentGroup resource that specifies an AWS CodeDeploy application    revision to be deployed to instances in the deployment group. If you specify an application    revision, your target revision is deployed as soon as the provisioning process is complete.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Deployment {


    /// 
    /// If true, then if an ApplicationStop, BeforeBlockTraffic, or     AfterBlockTraffic deployment lifecycle event to an instance fails, then the    deployment continues to the next deployment lifecycle event. For example, if     ApplicationStop fails, the deployment continues with DownloadBundle. If     BeforeBlockTraffic fails, the deployment continues with     BlockTraffic. If AfterBlockTraffic fails, the deployment continues    with ApplicationStop.
    /// 
    /// If false or not specified, then if a lifecycle event fails during a deployment to an    instance, that deployment fails. If deployment to that instance is part of an overall    deployment and the number of healthy hosts is not less than the minimum number of healthy    hosts, then a deployment to the next instance is attempted.
    /// 
    /// During a deployment, the AWS CodeDeploy agent runs the scripts specified for     ApplicationStop, BeforeBlockTraffic, and     AfterBlockTraffic in the AppSpec file from the previous successful deployment.    (All other scripts are run from the AppSpec file in the current deployment.) If one of these    scripts contains an error and does not run successfully, the deployment can fail.
    /// 
    /// If the cause of the failure is a script from the last successful deployment that will    never run successfully, create a new deployment and use     ignoreApplicationStopFailures to specify that the ApplicationStop,     BeforeBlockTraffic, and AfterBlockTraffic failures should be    ignored.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnoreApplicationStopFailures")]
    pub ignore_application_stop_failures: Option<bool>,


    /// 
    /// Information about the location of stored application artifacts and the service from       which to retrieve them.
    /// 
    /// Required: Yes
    ///
    /// Type: RevisionLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Revision")]
    pub revision: RevisionLocation,


    /// 
    /// A comment about the deployment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


/// The TrafficRoute property type specifies Property description not available. for an AWS::CodeDeploy::DeploymentGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrafficRoute {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ListenerArns")]
    pub listener_arns: Option<Vec<String>>,

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


/// The OnPremisesTagSet property type specifies a list containing other lists of    on-premises instance tag groups. In order for an instance to be included in the deployment    group, it must be identified by all the tag groups in the list.
///
/// For more information about using tags and tag groups to help manage your Amazon EC2 instances and on-premises instances, see Tagging Instances for Deployment     Groups in AWS CodeDeploy in the AWS CodeDeploy User     Guide.
///
/// OnPremisesTagSet is a property of the DeploymentGroup resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnPremisesTagSet {


    /// 
    /// A list that contains other lists of on-premises instance tag groups. For an instance to be    included in the deployment group, it must be identified by all of the tag groups in the    list.
    /// 
    /// Duplicates are not allowed.
    /// 
    /// Required: No
    ///
    /// Type: List of OnPremisesTagSetListObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnPremisesTagSetList")]
    pub on_premises_tag_set_list: Option<Vec<OnPremisesTagSetListObject>>,

}


/// Information about whether instances in the original environment are terminated when a       blue/green deployment is successful. BlueInstanceTerminationOption does not       apply to Lambda deployments.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlueInstanceTerminationOption {


    /// 
    /// For an Amazon EC2 deployment, the number of minutes to wait after a successful       blue/green deployment before terminating instances from the original environment.
    /// 
    /// For an Amazon ECS deployment, the number of minutes before deleting the       original (blue) task set. During an Amazon ECS deployment, CodeDeploy shifts       traffic from the original (blue) task set to a replacement (green) task set.
    /// 
    /// The maximum setting is 2880 minutes (2 days).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminationWaitTimeInMinutes")]
    pub termination_wait_time_in_minutes: Option<i64>,


    /// 
    /// The action to take on instances in the original environment after a successful       blue/green deployment.
    /// 
    /// TERMINATE: Instances are terminated after a specified wait           time.                        KEEP_ALIVE: Instances are left running after they are           deregistered from the load balancer and removed from the deployment           group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: KEEP_ALIVE | TERMINATE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Option<String>,

}


/// Information about the instances that belong to the replacement environment in a       blue/green deployment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GreenFleetProvisioningOption {


    /// 
    /// The method used to add instances to a replacement environment.
    /// 
    /// DISCOVER_EXISTING: Use instances that already exist or will be           created manually.                        COPY_AUTO_SCALING_GROUP: Use settings from a specified Auto Scaling group to define and create instances in a new Auto Scaling           group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COPY_AUTO_SCALING_GROUP | DISCOVER_EXISTING
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Option<String>,

}


/// Contains the service and cluster names used to identify an Amazon ECS       deployment's target.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ECSService {


    /// 
    /// The name of the target Amazon ECS service.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: String,


    /// 
    /// The name of the cluster that the Amazon ECS service is associated with.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,

}


/// The TargetGroupInfo property type specifies information about a target group    in Elastic Load Balancing to use in a deployment. Instances are registered as targets in a    target group, and traffic is routed to the target group. For more information, see     TargetGroupInfo in the AWS CodeDeploy API Reference
///
/// If you specify the TargetGroupInfo property, the     DeploymentStyle.DeploymentOption property must be set to     WITH_TRAFFIC_CONTROL for CodeDeploy to route your traffic using the    specified target groups.
///
/// TargetGroupInfo is a property of the LoadBalancerInfo property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetGroupInfo {


    /// 
    /// For blue/green deployments, the name of the target group that instances in the original    environment are deregistered from, and instances in the replacement environment registered    with. For in-place deployments, the name of the target group that instances are deregistered    from, so they are not serving traffic during a deployment, and then re-registered with after    the deployment completes. No duplicates allowed.
    /// 
    /// NoteAWS CloudFormation supports blue/green deployments on AWS Lambda compute     platforms only.
    /// 
    /// This value cannot exceed 32 characters, so you should use the Name property    of the target group, or the TargetGroupName attribute with the     Fn::GetAtt intrinsic function, as shown in the following example. Don't use the    group's Amazon Resource Name (ARN) or TargetGroupFullName attribute.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The AutoRollbackConfiguration property type configures automatic rollback for    an AWS CodeDeploy deployment group when a deployment is not completed successfully.    For more information, see Automatic Rollbacks in the AWS CodeDeploy User    Guide.
///
/// AutoRollbackConfiguration is a property of the DeploymentGroup resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoRollbackConfiguration {


    /// 
    /// Indicates whether a defined automatic rollback configuration is currently       enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The event type or types that trigger a rollback. Valid values are     DEPLOYMENT_FAILURE, DEPLOYMENT_STOP_ON_ALARM, or     DEPLOYMENT_STOP_ON_REQUEST.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Option<Vec<String>>,

}


/// The Alarm property type specifies a CloudWatch alarm to use for an     AWS CodeDeploy deployment group. The Alarm property of the CodeDeploy DeploymentGroup AlarmConfiguration property contains a list of     Alarm property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Alarm {


    /// 
    /// The name of the alarm. Maximum length is 255 characters. Each alarm name can be used       only once in a list of alarms.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The EC2TagSet property type specifies information about groups of tags    applied to Amazon EC2 instances. The deployment group includes only Amazon EC2 instances identified by all the tag groups. EC2TagSet cannot be    used in the same template as EC2TagFilter.
///
/// For information about using tags and tag groups to help manage your Amazon EC2    instances and on-premises instances, see Tagging Instances for Deployment     Groups in AWS CodeDeploy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EC2TagSet {


    /// 
    /// The Amazon EC2 tags that are already applied to Amazon EC2 instances    that you want to include in the deployment group. CodeDeploy includes all Amazon EC2 instances identified by any of the tags you specify in this deployment group.
    /// 
    /// Duplicates are not allowed.
    /// 
    /// Required: No
    ///
    /// Type: List of EC2TagSetListObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ec2TagSetList")]
    pub ec2_tag_set_list: Option<Vec<EC2TagSetListObject>>,

}


/// GitHubLocation is a property of the CodeDeploy DeploymentGroup Revision property that specifies the    location of an application revision that is stored in GitHub.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GitHubLocation {


    /// 
    /// The SHA1 commit ID of the GitHub commit that represents the bundled artifacts for the       application revision.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CommitId")]
    pub commit_id: String,


    /// 
    /// The GitHub account and repository pair that stores a reference to the commit that    represents the bundled artifacts for the application revision.
    /// 
    /// Specify the value as account/repository.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Repository")]
    pub repository: String,

}
