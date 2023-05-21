

/// An AWS Firewall Manager policy.
///
/// Firewall Manager provides the following types of policies:
///
/// Each policy is specific to one of the types. If you want to enforce more than one    policy type across accounts, create multiple policies. You can create multiple    policies for each type.
///
/// These policies require some setup to use. For more information, see the sections on prerequisites and getting started       under AWS Firewall Manager.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPolicy {


    /// 
    /// Specifies the AWS account IDs and AWS Organizations organizational units (OUs) to include in the policy.      Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.
    /// 
    /// You can specify inclusions or exclusions, but not both. If you specify an IncludeMap, AWS Firewall Manager      applies the policy to all accounts specified by the IncludeMap, and      does not evaluate any ExcludeMap specifications. If you do not specify an IncludeMap, then Firewall Manager       applies the policy to all accounts except for those specified by the ExcludeMap.
    /// 
    /// You can specify account IDs, OUs, or a combination:
    /// 
    /// Specify account IDs by setting the key to ACCOUNT. For example, the following is a valid map:    {“ACCOUNT” : [“accountID1”, “accountID2”]}.               Specify OUs by setting the key to ORGUNIT. For example, the following is a valid map:  {“ORGUNIT” : [“ouid111”, “ouid112”]}.               Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map:    {“ACCOUNT” : [“accountID1”, “accountID2”], “ORGUNIT” : [“ouid111”, “ouid112”]}.
    /// 
    /// Required: No
    ///
    /// Type: IEMap
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeMap")]
    pub include_map: Option<IEMap>,


    /// 
    /// The type of resource protected by or in scope of the policy. This is in the format shown     in the AWS Resource Types Reference.           To apply this policy to multiple resource types, specify a resource type of ResourceTypeList and then specify the resource types in a ResourceTypeList.
    /// 
    /// For AWS WAF and Shield Advanced, example resource types include     AWS::ElasticLoadBalancingV2::LoadBalancer and     AWS::CloudFront::Distribution. For a security group common policy, valid values    are AWS::EC2::NetworkInterface and AWS::EC2::Instance. For a    security group content audit policy, valid values are AWS::EC2::SecurityGroup,     AWS::EC2::NetworkInterface, and AWS::EC2::Instance. For a security       group usage audit policy, the value is AWS::EC2::SecurityGroup. For an AWS Network Firewall policy or DNS Firewall policy,         the value is AWS::EC2::VPC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,


    /// 
    /// The unique identifiers of the resource sets used by the policy.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceSetIds")]
    pub resource_set_ids: Option<Vec<String>>,


    /// 
    /// Specifies the AWS account IDs and AWS Organizations organizational units (OUs) to exclude from the policy.      Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.
    /// 
    /// You can specify inclusions or exclusions, but not both. If you specify an IncludeMap, AWS Firewall Manager      applies the policy to all accounts specified by the IncludeMap, and      does not evaluate any ExcludeMap specifications. If you do not specify an IncludeMap, then Firewall Manager       applies the policy to all accounts except for those specified by the ExcludeMap.
    /// 
    /// You can specify account IDs, OUs, or a combination:
    /// 
    /// Specify account IDs by setting the key to ACCOUNT. For example, the following is a valid map:    {“ACCOUNT” : [“accountID1”, “accountID2”]}.               Specify OUs by setting the key to ORGUNIT. For example, the following is a valid map:  {“ORGUNIT” : [“ouid111”, “ouid112”]}.               Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map:    {“ACCOUNT” : [“accountID1”, “accountID2”], “ORGUNIT” : [“ouid111”, “ouid112”]}.
    /// 
    /// Required: No
    ///
    /// Type: IEMap
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeMap")]
    pub exclude_map: Option<IEMap>,


    /// 
    /// Indicates whether AWS Firewall Manager should automatically remove protections from resources that leave the policy scope and clean up resources    that Firewall Manager is managing for accounts when those accounts leave policy scope. For example, Firewall Manager will disassociate a Firewall Manager managed web ACL    from a protected customer resource when the customer resource leaves policy scope.
    /// 
    /// By default, Firewall Manager doesn't remove protections or delete Firewall Manager managed resources.
    /// 
    /// This option is not available for Shield Advanced or AWS WAF Classic policies.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcesCleanUp")]
    pub resources_clean_up: Option<bool>,


    /// 
    /// An array of ResourceType objects. Use this only to specify multiple resource types. To specify a single resource type, use ResourceType.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTypeList")]
    pub resource_type_list: Option<Vec<String>>,


    /// 
    /// A collection of key:value pairs associated with an AWS resource. The key:value pair can be anything you define. Typically, the tag key represents a category (such as "environment") and the tag value represents a specific value within that category (such as "test," "development," or "production"). You can add up to 50 tags to each AWS resource.
    /// 
    /// Required: No
    ///
    /// Type: List of PolicyTag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<PolicyTag>>,


    /// 
    /// Used only when tags are specified in the ResourceTags property. If this property        is True, resources with the specified tags are not in scope of the policy. If it's False,          only resources with the specified tags are in scope of the policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeResourceTags")]
    pub exclude_resource_tags: bool,


    /// 
    /// The name of the AWS Firewall Manager policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyName")]
    pub policy_name: String,


    /// 
    /// Used when deleting a policy. If true, Firewall Manager performs cleanup according to the policy type.
    /// 
    /// For AWS WAF and Shield Advanced policies, Firewall Manager does the following:
    /// 
    /// Deletes rule groups created by Firewall ManagerRemoves web ACLs from in-scope resources Deletes web ACLs that contain no rules or rule groups
    /// 
    /// For security group policies, Firewall Manager does the following for each security group in the policy:
    /// 
    /// Disassociates the security group from in-scope resources Deletes the security group if it was created through Firewall Manager and if it's no longer associated with any resources through another policy
    /// 
    /// After the cleanup, in-scope resources are no longer protected by web ACLs in this policy. Protection of out-of-scope resources remains unchanged. Scope is determined by tags that you create and accounts that you associate with the policy. When creating the policy, if you specify that only resources in specific accounts or with specific tags are in scope of the policy, those accounts and resources are handled by the policy. All others are out of scope. If you don't specify tags or accounts, all resources are in scope.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteAllPolicyResources")]
    pub delete_all_policy_resources: Option<bool>,


    /// 
    /// Indicates if the policy should be automatically applied to new resources.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemediationEnabled")]
    pub remediation_enabled: bool,


    /// 
    /// An array of ResourceTag objects, used to explicitly include resources in the      policy scope or explicitly exclude them. If this isn't set, then tags aren't used to modify policy scope. See      also ExcludeResourceTags.
    /// 
    /// Required: No
    ///
    /// Type: List of ResourceTag
    ///
    /// Maximum: 8
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTags")]
    pub resource_tags: Option<Vec<ResourceTag>>,


    /// 
    /// Details about the security service that is being used to protect the resources.
    /// 
    /// This contains the following settings:
    /// 
    /// Type - Indicates the service type that the policy uses to protect the resource.       For security group policies, Firewall Manager supports one security group for       each common policy and for each content audit policy. This is an adjustable limit that you       can increase by contacting AWS Support.        Valid values: DNS_FIREWALL | NETWORK_FIREWALL | SECURITY_GROUPS_COMMON | SECURITY_GROUPS_CONTENT_AUDIT | SECURITY_GROUPS_USAGE_AUDIT | SHIELD_ADVANCED | THIRD_PARTY_FIREWALL | WAFV2 | WAF    ManagedServiceData - Details about the service that are specific to the service type, in JSON format.                                                                                             Example: DNS_FIREWALL                    "{\"type\":\"DNS_FIREWALL\",\"preProcessRuleGroups\":[{\"ruleGroupId\":\"rslvr-frg-1\",\"priority\":10}],\"postProcessRuleGroups\":[{\"ruleGroupId\":\"rslvr-frg-2\",\"priority\":9911}]}"            NoteValid values for preProcessRuleGroups are between 1 and 99. Valid       values for postProcessRuleGroups are between 9901 and 10000.           Example: NETWORK_FIREWALL - Centralized deployment     model              "{\"type\":\"NETWORK_FIREWALL\",\"awsNetworkFirewallConfig\":{\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\":\"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":true}},\"firewallDeploymentModel\":{\"centralizedFirewallDeploymentModel\":{\"centralizedFirewallOrchestrationConfig\":{\"inspectionVpcIds\":[{\"resourceId\":\"vpc-1234\",\"accountId\":\"123456789011\"}],\"firewallCreationConfig\":{\"endpointLocation\":{\"availabilityZoneConfigList\":[{\"availabilityZoneId\":null,\"availabilityZoneName\":\"us-east-1a\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]}]}},\"allowedIPV4CidrList\":[]}}}}"            To use the distributed deployment model, you must set FirewallDeploymentModel to       DISTRIBUTED.           Example: NETWORK_FIREWALL - Distributed deployment model with      automatic Availability Zone configuration                    "{\"type\":\"NETWORK_FIREWALL\",\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallOrchestrationConfig\":{\"singleFirewallEndpointPerVPC\":false,\"allowedIPV4CidrList\":[\"10.0.0.0/28\",\"192.168.0.0/28\"],\"routeManagementAction\":\"OFF\"},\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\":\"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":true}}"                  With automatic Availbility Zone configuration, Firewall Manager chooses which Availability Zones to create the endpoints in. To use the distributed deployment model, you must set FirewallDeploymentModel to       DISTRIBUTED.           Example: NETWORK_FIREWALL - Distributed deployment model with      automatic Availability Zone configuration and route management                    "{\"type\":\"NETWORK_FIREWALL\",\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallOrchestrationConfig\":{\"singleFirewallEndpointPerVPC\":false,\"allowedIPV4CidrList\":[\"10.0.0.0/28\",\"192.168.0.0/28\"],\"routeManagementAction\":\"MONITOR\",\"routeManagementTargetTypes\":[\"InternetGateway\"]},\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\": \"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":true}}"                  To use the distributed deployment model, you must set FirewallDeploymentModel to       DISTRIBUTED.           Example: NETWORK_FIREWALL - Distributed deployment model with      custom Availability Zone configuration              "{\"type\":\"NETWORK_FIREWALL\",\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"fragmentcustomactionname\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\", \"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}},{\"actionName\":\"fragmentcustomactionname\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"fragmentmetricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallOrchestrationConfig\":{\"firewallCreationConfig\":{ \"endpointLocation\":{\"availabilityZoneConfigList\":[{\"availabilityZoneName\":\"us-east-1a\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]},{\"availabilityZoneName\":\"us-east-1b\",\"allowedIPV4CidrList\":[ \"10.0.0.0/28\"]}]} },\"singleFirewallEndpointPerVPC\":false,\"allowedIPV4CidrList\":null,\"routeManagementAction\":\"OFF\",\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\":\"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":boolean}}"                       With custom Availability Zone configuration,     you define which specific Availability Zones to create endpoints in by configuring       firewallCreationConfig. To configure the Availability Zones in firewallCreationConfig, specify either the availabilityZoneName or availabilityZoneId parameter, not both parameters.        To use the distributed deployment model, you must set FirewallDeploymentModel to     DISTRIBUTED.           Example: NETWORK_FIREWALL - Distributed deployment model with      custom Availability Zone configuration and route management              "{\"type\":\"NETWORK_FIREWALL\",\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"fragmentcustomactionname\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}},{\"actionName\":\"fragmentcustomactionname\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"fragmentmetricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallOrchestrationConfig\":{\"firewallCreationConfig\":{\"endpointLocation\":{\"availabilityZoneConfigList\":[{\"availabilityZoneName\":\"us-east-1a\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]},{\"availabilityZoneName\":\"us-east-1b\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]}]}},\"singleFirewallEndpointPerVPC\":false,\"allowedIPV4CidrList\":null,\"routeManagementAction\":\"MONITOR\",\"routeManagementTargetTypes\":[\"InternetGateway\"],\"routeManagementConfig\":{\"allowCrossAZTrafficIfNoEndpoint\":true}},\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\":\"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":boolean}}"                  To use the distributed deployment model, you must set FirewallDeploymentModel to       DISTRIBUTED.           Example: THIRD_PARTY_FIREWALL - Palo Alto Networks Cloud Next-Generation Firewall centralized deployment model                    "{ \"type\":\"THIRD_PARTY_FIREWALL\", \"thirdPartyFirewall\":\"PALO_ALTO_NETWORKS_CLOUD_NGFW\", \"thirdPartyFirewallConfig\":{ \"thirdPartyFirewallPolicyList\":[\"global-1\"] },\"firewallDeploymentModel\":{\"centralizedFirewallDeploymentModel\":{\"centralizedFirewallOrchestrationConfig\":{\"inspectionVpcIds\":[{\"resourceId\":\"vpc-1234\",\"accountId\":\"123456789011\"}],\"firewallCreationConfig\":{\"endpointLocation\":{\"availabilityZoneConfigList\":[{\"availabilityZoneId\":null,\"availabilityZoneName\":\"us-east-1a\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]}]}},\"allowedIPV4CidrList\":[]}}}}"            To use the distributed deployment model, you must set FirewallDeploymentModel to       CENTRALIZED.           Example: THIRD_PARTY_FIREWALL - Palo Alto Networks Cloud Next-Generation Firewall distributed deployment model                    "{\"type\":\"THIRD_PARTY_FIREWALL\",\"thirdPartyFirewall\":\"PALO_ALTO_NETWORKS_CLOUD_NGFW\",\"thirdPartyFirewallConfig\":{\"thirdPartyFirewallPolicyList\":[\"global-1\"] },\"firewallDeploymentModel\":{ \"distributedFirewallDeploymentModel\":{ \"distributedFirewallOrchestrationConfig\":{\"firewallCreationConfig\":{\"endpointLocation\":{ \"availabilityZoneConfigList\":[ {\"availabilityZoneName\":\"${AvailabilityZone}\" } ] } }, \"allowedIPV4CidrList\":[ ] } } } }"            To use the distributed deployment model, you must set FirewallDeploymentModel to       DISTRIBUTED.           Specification for SHIELD_ADVANCED for Amazon CloudFront distributions              "{\"type\":\"SHIELD_ADVANCED\",\"automaticResponseConfiguration\":       {\"automaticResponseStatus\":\"ENABLED|IGNORED|DISABLED\",       \"automaticResponseAction\":\"BLOCK|COUNT\"},       \"overrideCustomerWebaclClassic\":true|false}"            For example:       "{\"type\":\"SHIELD_ADVANCED\",\"automaticResponseConfiguration\":       {\"automaticResponseStatus\":\"ENABLED\",       \"automaticResponseAction\":\"COUNT\"}}"            The default value for automaticResponseStatus is       IGNORED. The value for automaticResponseAction is only      required when automaticResponseStatus is set to ENABLED.      The default value for overrideCustomerWebaclClassic is       false.      For other resource types that you can protect with a Shield Advanced policy, this       ManagedServiceData configuration is an empty string.           Example: WAFV2                    "{\"type\":\"WAFV2\",\"preProcessRuleGroups\":[{\"ruleGroupArn\":null,\"overrideAction\":{\"type\":\"NONE\"},\"managedRuleGroupIdentifier\":{\"version\":null,\"vendorName\":\"AWS\",\"managedRuleGroupName\":\"AWSManagedRulesAmazonIpReputationList\"},\"ruleGroupType\":\"ManagedRuleGroup\",\"excludeRules\":[{\"name\":\"NoUserAgent_HEADER\"}]}],\"postProcessRuleGroups\":[],\"defaultAction\":{\"type\":\"ALLOW\"},\"overrideCustomerWebACLAssociation\":false,\"loggingConfiguration\":{\"logDestinationConfigs\":[\"arn:aws:firehose:us-west-2:12345678912:deliverystream/aws-waf-logs-fms-admin-destination\"],\"redactedFields\":[{\"redactedFieldType\":\"SingleHeader\",\"redactedFieldValue\":\"Cookies\"},{\"redactedFieldType\":\"Method\"}]}}"            In the loggingConfiguration, you can specify one       logDestinationConfigs, you can optionally provide up to 20       redactedFields, and the RedactedFieldType must be one of       URI, QUERY_STRING, HEADER, or       METHOD.           Example:          AWS WAF Classic                    "{\"type\": \"WAF\", \"ruleGroups\":       [{\"id\":\"12345678-1bcd-9012-efga-0987654321ab\", \"overrideAction\" : {\"type\":       \"COUNT\"}}], \"defaultAction\": {\"type\": \"BLOCK\"}}"                 Example: WAFV2 - AWS Firewall Manager support for AWS WAF managed rule group versioning                 "{\"type\":\"WAFV2\",\"preProcessRuleGroups\":[{\"ruleGroupArn\":null,\"overrideAction\":{\"type\":\"NONE\"},\"managedRuleGroupIdentifier\":{\"versionEnabled\":true,\"version\":\"Version_2.0\",\"vendorName\":\"AWS\",\"managedRuleGroupName\":\"AWSManagedRulesCommonRuleSet\"},\"ruleGroupType\":\"ManagedRuleGroup\",\"excludeRules\":[{\"name\":\"NoUserAgent_HEADER\"}]}],\"postProcessRuleGroups\":[],\"defaultAction\":{\"type\":\"ALLOW\"},\"overrideCustomerWebACLAssociation\":false,\"loggingConfiguration\":{\"logDestinationConfigs\":[\"arn:aws:firehose:us-west-2:12345678912:deliverystream/aws-waf-logs-fms-admin-destination\"],\"redactedFields\":[{\"redactedFieldType\":\"SingleHeader\",\"redactedFieldValue\":\"Cookies\"},{\"redactedFieldType\":\"Method\"}]}}"                 To use a specific version of a AWS WAF managed rule group in your Firewall Manager policy, you must set versionEnabled to true, and set version to the version you'd like to use. If you don't set versionEnabled to true, or if you omit versionEnabled, then Firewall Manager uses the default version of the AWS WAF managed rule group.              Example: SECURITY_GROUPS_COMMON                    "{\"type\":\"SECURITY_GROUPS_COMMON\",\"revertManualSecurityGroupChanges\":false,\"exclusiveResourceSecurityGroupManagement\":false,       \"applyToAllEC2InstanceENIs\":false,\"securityGroups\":[{\"id\":\"       sg-000e55995d61a06bd\"}]}"                 Example: Shared VPCs. Apply the preceding policy to resources in shared VPCs as      well as to those in VPCs that the account owns              "{\"type\":\"SECURITY_GROUPS_COMMON\",\"revertManualSecurityGroupChanges\":false,\"exclusiveResourceSecurityGroupManagement\":false,       \"applyToAllEC2InstanceENIs\":false,\"includeSharedVPC\":true,\"securityGroups\":[{\"id\":\"       sg-000e55995d61a06bd\"}]}"                 Example: SECURITY_GROUPS_CONTENT_AUDIT                    "{\"type\":\"SECURITY_GROUPS_CONTENT_AUDIT\",\"securityGroups\":[{\"id\":\"sg-000e55995d61a06bd\"}],\"securityGroupAction\":{\"type\":\"ALLOW\"}}"            The security group action for content audit can be ALLOW or       DENY. For ALLOW, all in-scope security group rules must      be within the allowed range of the policy's security group rules. For       DENY, all in-scope security group rules must not contain a value or a      range that matches a rule value or range in the policy security group.           Example: SECURITY_GROUPS_USAGE_AUDIT                    "{\"type\":\"SECURITY_GROUPS_USAGE_AUDIT\",\"deleteUnusedSecurityGroups\":true,\"coalesceRedundantSecurityGroups\":true}"
    /// 
    /// Required: Yes
    ///
    /// Type: SecurityServicePolicyData
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityServicePolicyData")]
    pub security_service_policy_data: SecurityServicePolicyData,


    /// 
    /// The definition of the AWS Network Firewall firewall policy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDescription")]
    pub policy_description: Option<String>,

}

impl cfn_resources::CfnResource for CfnPolicy {
    fn type_string() -> &'static str {
        "AWS::FMS::Policy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Configures the firewall policy deployment model of AWS Network Firewall. For information about     Network Firewall deployment models, see AWS Network Firewall example       architectures with routing in the Network Firewall Developer     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkFirewallPolicy {


    /// 
    /// Defines the deployment model to use for the firewall policy. To use a distributed model,      set FirewallDeploymentModel to       DISTRIBUTED.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CENTRALIZED | DISTRIBUTED
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirewallDeploymentModel")]
    pub firewall_deployment_model: String,

}


/// Contains the AWS Network Firewall firewall policy options to configure the policy's deployment model and third-party firewall policy settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PolicyOption {


    /// 
    /// Defines the deployment model to use for the firewall policy.
    /// 
    /// Required: No
    ///
    /// Type: NetworkFirewallPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkFirewallPolicy")]
    pub network_firewall_policy: Option<NetworkFirewallPolicy>,


    /// 
    /// Defines the policy options for a third-party firewall policy.
    /// 
    /// Required: No
    ///
    /// Type: ThirdPartyFirewallPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThirdPartyFirewallPolicy")]
    pub third_party_firewall_policy: Option<ThirdPartyFirewallPolicy>,

}


/// Details about the security service that is being used to protect the resources.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SecurityServicePolicyData {


    /// 
    /// The service that the policy is using to protect the resources. This specifies the type of    policy that is created, either an AWS WAF policy, a Shield Advanced policy, or a security    group policy. For security group policies, Firewall Manager supports one security group for    each common policy and for each content audit policy. This is an adjustable limit that you can    increase by contacting AWS Support.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DNS_FIREWALL | IMPORT_NETWORK_FIREWALL | NETWORK_FIREWALL | SECURITY_GROUPS_COMMON | SECURITY_GROUPS_CONTENT_AUDIT | SECURITY_GROUPS_USAGE_AUDIT | SHIELD_ADVANCED | THIRD_PARTY_FIREWALL | WAF | WAFV2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// Details about the service that are specific to the service type, in JSON format.
    /// 
    /// Example: DNS_FIREWALL                          "{\"type\":\"DNS_FIREWALL\",\"preProcessRuleGroups\":[{\"ruleGroupId\":\"rslvr-frg-1\",\"priority\":10}],\"postProcessRuleGroups\":[{\"ruleGroupId\":\"rslvr-frg-2\",\"priority\":9911}]}"                NoteValid values for preProcessRuleGroups are between 1 and 99. Valid         values for postProcessRuleGroups are between 9901 and 10000.               Example: NETWORK_FIREWALL - Centralized deployment       model                  "{\"type\":\"NETWORK_FIREWALL\",\"awsNetworkFirewallConfig\":{\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\":\"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":true}},\"firewallDeploymentModel\":{\"centralizedFirewallDeploymentModel\":{\"centralizedFirewallOrchestrationConfig\":{\"inspectionVpcIds\":[{\"resourceId\":\"vpc-1234\",\"accountId\":\"123456789011\"}],\"firewallCreationConfig\":{\"endpointLocation\":{\"availabilityZoneConfigList\":[{\"availabilityZoneId\":null,\"availabilityZoneName\":\"us-east-1a\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]}]}},\"allowedIPV4CidrList\":[]}}}}"                To use the distributed deployment model, you must set FirewallDeploymentModel to         DISTRIBUTED.               Example: NETWORK_FIREWALL - Distributed deployment model with        automatic Availability Zone configuration                          "{\"type\":\"NETWORK_FIREWALL\",\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallOrchestrationConfig\":{\"singleFirewallEndpointPerVPC\":false,\"allowedIPV4CidrList\":[\"10.0.0.0/28\",\"192.168.0.0/28\"],\"routeManagementAction\":\"OFF\"},\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\":\"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":true}}"                        With automatic Availbility Zone configuration, Firewall Manager chooses which Availability Zones to create the endpoints in. To use the distributed deployment model, you must set FirewallDeploymentModel to         DISTRIBUTED.               Example: NETWORK_FIREWALL - Distributed deployment model with        automatic Availability Zone configuration and route management                          "{\"type\":\"NETWORK_FIREWALL\",\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallOrchestrationConfig\":{\"singleFirewallEndpointPerVPC\":false,\"allowedIPV4CidrList\":[\"10.0.0.0/28\",\"192.168.0.0/28\"],\"routeManagementAction\":\"MONITOR\",\"routeManagementTargetTypes\":[\"InternetGateway\"]},\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\": \"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":true}}"                        To use the distributed deployment model, you must set FirewallDeploymentModel to         DISTRIBUTED.               Example: NETWORK_FIREWALL - Distributed deployment model with        custom Availability Zone configuration                  "{\"type\":\"NETWORK_FIREWALL\",\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"fragmentcustomactionname\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\", \"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}},{\"actionName\":\"fragmentcustomactionname\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"fragmentmetricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallOrchestrationConfig\":{\"firewallCreationConfig\":{ \"endpointLocation\":{\"availabilityZoneConfigList\":[{\"availabilityZoneName\":\"us-east-1a\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]},{\"availabilityZoneName\":\"us-east-1b\",\"allowedIPV4CidrList\":[ \"10.0.0.0/28\"]}]} },\"singleFirewallEndpointPerVPC\":false,\"allowedIPV4CidrList\":null,\"routeManagementAction\":\"OFF\",\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\":\"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":boolean}}"                               With custom Availability Zone configuration,       you define which specific Availability Zones to create endpoints in by configuring         firewallCreationConfig. To configure the Availability Zones in firewallCreationConfig, specify either the availabilityZoneName or availabilityZoneId parameter, not both parameters.            To use the distributed deployment model, you must set FirewallDeploymentModel to       DISTRIBUTED.               Example: NETWORK_FIREWALL - Distributed deployment model with        custom Availability Zone configuration and route management                  "{\"type\":\"NETWORK_FIREWALL\",\"networkFirewallStatelessRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateless-rulegroup/test\",\"priority\":1}],\"networkFirewallStatelessDefaultActions\":[\"aws:forward_to_sfe\",\"customActionName\"],\"networkFirewallStatelessFragmentDefaultActions\":[\"aws:forward_to_sfe\",\"fragmentcustomactionname\"],\"networkFirewallStatelessCustomActions\":[{\"actionName\":\"customActionName\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"metricdimensionvalue\"}]}}},{\"actionName\":\"fragmentcustomactionname\",\"actionDefinition\":{\"publishMetricAction\":{\"dimensions\":[{\"value\":\"fragmentmetricdimensionvalue\"}]}}}],\"networkFirewallStatefulRuleGroupReferences\":[{\"resourceARN\":\"arn:aws:network-firewall:us-east-1:123456789011:stateful-rulegroup/test\"}],\"networkFirewallOrchestrationConfig\":{\"firewallCreationConfig\":{\"endpointLocation\":{\"availabilityZoneConfigList\":[{\"availabilityZoneName\":\"us-east-1a\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]},{\"availabilityZoneName\":\"us-east-1b\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]}]}},\"singleFirewallEndpointPerVPC\":false,\"allowedIPV4CidrList\":null,\"routeManagementAction\":\"MONITOR\",\"routeManagementTargetTypes\":[\"InternetGateway\"],\"routeManagementConfig\":{\"allowCrossAZTrafficIfNoEndpoint\":true}},\"networkFirewallLoggingConfiguration\":{\"logDestinationConfigs\":[{\"logDestinationType\":\"S3\",\"logType\":\"ALERT\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}},{\"logDestinationType\":\"S3\",\"logType\":\"FLOW\",\"logDestination\":{\"bucketName\":\"s3-bucket-name\"}}],\"overrideExistingConfig\":boolean}}"                        To use the distributed deployment model, you must set FirewallDeploymentModel to         DISTRIBUTED.               Example: THIRD_PARTY_FIREWALL - Palo Alto Networks Cloud Next-Generation Firewall centralized deployment model                          "{ \"type\":\"THIRD_PARTY_FIREWALL\", \"thirdPartyFirewall\":\"PALO_ALTO_NETWORKS_CLOUD_NGFW\", \"thirdPartyFirewallConfig\":{ \"thirdPartyFirewallPolicyList\":[\"global-1\"] },\"firewallDeploymentModel\":{\"centralizedFirewallDeploymentModel\":{\"centralizedFirewallOrchestrationConfig\":{\"inspectionVpcIds\":[{\"resourceId\":\"vpc-1234\",\"accountId\":\"123456789011\"}],\"firewallCreationConfig\":{\"endpointLocation\":{\"availabilityZoneConfigList\":[{\"availabilityZoneId\":null,\"availabilityZoneName\":\"us-east-1a\",\"allowedIPV4CidrList\":[\"10.0.0.0/28\"]}]}},\"allowedIPV4CidrList\":[]}}}}"                To use the distributed deployment model, you must set FirewallDeploymentModel to         CENTRALIZED.               Example: THIRD_PARTY_FIREWALL - Palo Alto Networks Cloud Next-Generation Firewall distributed deployment model                          "{\"type\":\"THIRD_PARTY_FIREWALL\",\"thirdPartyFirewall\":\"PALO_ALTO_NETWORKS_CLOUD_NGFW\",\"thirdPartyFirewallConfig\":{\"thirdPartyFirewallPolicyList\":[\"global-1\"] },\"firewallDeploymentModel\":{ \"distributedFirewallDeploymentModel\":{ \"distributedFirewallOrchestrationConfig\":{\"firewallCreationConfig\":{\"endpointLocation\":{ \"availabilityZoneConfigList\":[ {\"availabilityZoneName\":\"${AvailabilityZone}\" } ] } }, \"allowedIPV4CidrList\":[ ] } } } }"                To use the distributed deployment model, you must set FirewallDeploymentModel to         DISTRIBUTED.               Specification for SHIELD_ADVANCED for Amazon CloudFront distributions                  "{\"type\":\"SHIELD_ADVANCED\",\"automaticResponseConfiguration\":         {\"automaticResponseStatus\":\"ENABLED|IGNORED|DISABLED\",         \"automaticResponseAction\":\"BLOCK|COUNT\"},         \"overrideCustomerWebaclClassic\":true|false}"                For example:         "{\"type\":\"SHIELD_ADVANCED\",\"automaticResponseConfiguration\":         {\"automaticResponseStatus\":\"ENABLED\",         \"automaticResponseAction\":\"COUNT\"}}"                The default value for automaticResponseStatus is         IGNORED. The value for automaticResponseAction is only        required when automaticResponseStatus is set to ENABLED.        The default value for overrideCustomerWebaclClassic is         false.        For other resource types that you can protect with a Shield Advanced policy, this         ManagedServiceData configuration is an empty string.               Example: WAFV2                          "{\"type\":\"WAFV2\",\"preProcessRuleGroups\":[{\"ruleGroupArn\":null,\"overrideAction\":{\"type\":\"NONE\"},\"managedRuleGroupIdentifier\":{\"version\":null,\"vendorName\":\"AWS\",\"managedRuleGroupName\":\"AWSManagedRulesAmazonIpReputationList\"},\"ruleGroupType\":\"ManagedRuleGroup\",\"excludeRules\":[{\"name\":\"NoUserAgent_HEADER\"}]}],\"postProcessRuleGroups\":[],\"defaultAction\":{\"type\":\"ALLOW\"},\"overrideCustomerWebACLAssociation\":false,\"loggingConfiguration\":{\"logDestinationConfigs\":[\"arn:aws:firehose:us-west-2:12345678912:deliverystream/aws-waf-logs-fms-admin-destination\"],\"redactedFields\":[{\"redactedFieldType\":\"SingleHeader\",\"redactedFieldValue\":\"Cookies\"},{\"redactedFieldType\":\"Method\"}]}}"                In the loggingConfiguration, you can specify one         logDestinationConfigs, you can optionally provide up to 20         redactedFields, and the RedactedFieldType must be one of         URI, QUERY_STRING, HEADER, or         METHOD.               Example:            AWS WAF Classic                          "{\"type\": \"WAF\", \"ruleGroups\":         [{\"id\":\"12345678-1bcd-9012-efga-0987654321ab\", \"overrideAction\" : {\"type\":         \"COUNT\"}}], \"defaultAction\": {\"type\": \"BLOCK\"}}"                       Example: WAFV2 - AWS Firewall Manager support for AWS WAF managed rule group versioning                       "{\"type\":\"WAFV2\",\"preProcessRuleGroups\":[{\"ruleGroupArn\":null,\"overrideAction\":{\"type\":\"NONE\"},\"managedRuleGroupIdentifier\":{\"versionEnabled\":true,\"version\":\"Version_2.0\",\"vendorName\":\"AWS\",\"managedRuleGroupName\":\"AWSManagedRulesCommonRuleSet\"},\"ruleGroupType\":\"ManagedRuleGroup\",\"excludeRules\":[{\"name\":\"NoUserAgent_HEADER\"}]}],\"postProcessRuleGroups\":[],\"defaultAction\":{\"type\":\"ALLOW\"},\"overrideCustomerWebACLAssociation\":false,\"loggingConfiguration\":{\"logDestinationConfigs\":[\"arn:aws:firehose:us-west-2:12345678912:deliverystream/aws-waf-logs-fms-admin-destination\"],\"redactedFields\":[{\"redactedFieldType\":\"SingleHeader\",\"redactedFieldValue\":\"Cookies\"},{\"redactedFieldType\":\"Method\"}]}}"                       To use a specific version of a AWS WAF managed rule group in your Firewall Manager policy, you must set versionEnabled to true, and set version to the version you'd like to use. If you don't set versionEnabled to true, or if you omit versionEnabled, then Firewall Manager uses the default version of the AWS WAF managed rule group.                    Example: SECURITY_GROUPS_COMMON                          "{\"type\":\"SECURITY_GROUPS_COMMON\",\"revertManualSecurityGroupChanges\":false,\"exclusiveResourceSecurityGroupManagement\":false,         \"applyToAllEC2InstanceENIs\":false,\"securityGroups\":[{\"id\":\"         sg-000e55995d61a06bd\"}]}"                       Example: Shared VPCs. Apply the preceding policy to resources in shared VPCs as        well as to those in VPCs that the account owns                  "{\"type\":\"SECURITY_GROUPS_COMMON\",\"revertManualSecurityGroupChanges\":false,\"exclusiveResourceSecurityGroupManagement\":false,         \"applyToAllEC2InstanceENIs\":false,\"includeSharedVPC\":true,\"securityGroups\":[{\"id\":\"         sg-000e55995d61a06bd\"}]}"                       Example: SECURITY_GROUPS_CONTENT_AUDIT                          "{\"type\":\"SECURITY_GROUPS_CONTENT_AUDIT\",\"securityGroups\":[{\"id\":\"sg-000e55995d61a06bd\"}],\"securityGroupAction\":{\"type\":\"ALLOW\"}}"                The security group action for content audit can be ALLOW or         DENY. For ALLOW, all in-scope security group rules must        be within the allowed range of the policy's security group rules. For         DENY, all in-scope security group rules must not contain a value or a        range that matches a rule value or range in the policy security group.               Example: SECURITY_GROUPS_USAGE_AUDIT                          "{\"type\":\"SECURITY_GROUPS_USAGE_AUDIT\",\"deleteUnusedSecurityGroups\":true,\"coalesceRedundantSecurityGroups\":true}"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10000
    ///
    /// Pattern: ^((?!\\[nr]).)+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedServiceData")]
    pub managed_service_data: Option<String>,


    /// 
    /// Contains the Network Firewall firewall policy options to configure a centralized deployment     model.
    /// 
    /// Required: No
    ///
    /// Type: PolicyOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyOption")]
    pub policy_option: Option<PolicyOption>,

}


/// Configures the deployment model for the third-party firewall.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ThirdPartyFirewallPolicy {


    /// 
    /// Defines the deployment model to use for the third-party firewall policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CENTRALIZED | DISTRIBUTED
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirewallDeploymentModel")]
    pub firewall_deployment_model: String,

}


/// Specifies the AWS account IDs and AWS Organizations organizational units (OUs) to include in or exclude from the policy.      Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.
///
/// This is used for the policy's IncludeMap and ExcludeMap.
///
/// You can specify account IDs, OUs, or a combination:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IEMap {


    /// 
    /// The account list for the map.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ACCOUNT")]
    pub account: Option<Vec<String>>,


    /// 
    /// The organizational unit list for the map.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ORGUNIT")]
    pub orgunit: Option<Vec<String>>,

}


/// A collection of key:value pairs associated with an AWS resource. The key:value pair can be anything you define. Typically, the tag key represents a category (such as "environment") and the tag value represents a specific value within that category (such as "test," "development," or "production"). You can add up to 50 tags to each AWS resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PolicyTag {


    /// 
    /// Part of the key:value pair that defines a tag. You can use a tag value to describe a specific value within a category, such as "companyA" or "companyB." Tag values are case-sensitive.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// Part of the key:value pair that defines a tag. You can use a tag key to describe a category of information, such as "customer." Tag keys are case-sensitive.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}


/// The resource tags that AWS Firewall Manager uses to determine if a particular resource    should be included or excluded from the AWS Firewall Manager policy. Tags enable you to    categorize your AWS resources in different ways, for example, by purpose, owner, or    environment. Each tag consists of a key and an optional value. Firewall Manager combines the    tags with "AND" so that, if you add more than one tag to a policy scope, a resource must have     all the specified tags to be included or excluded. For more information, see   Working with Tag Editor.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceTag {


    /// 
    /// The resource tag key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The resource tag value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}
