

/// Creates an AWS Resilience Hub application. An AWS Resilience Hub application is a    collection of AWS resources structured to prevent and recover AWS application disruptions. To describe a AWS Resilience Hub application,    you provide an    application name, resources from one or more AWS CloudFormation stacks, AWS Resource Groups, Terraform state files, AppRegistry applications, and an appropriate    resiliency policy. In addition, you can also add resources that are located on Amazon Elastic Kubernetes Service (Amazon EKS) clusters as optional resources. For more information    about the number of resources supported per application, see Service    quotas.
///
/// After you create an AWS Resilience Hub application, you publish it so that you can run a resiliency    assessment on it. You can then use recommendations from the assessment to improve resiliency    by running another assessment, comparing results, and then iterating the process until you    achieve your goals for recovery time objective (RTO) and recovery point objective    (RPO).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApp {


    /// 
    /// Assessment execution schedule with 'Daily' or 'Disabled' values.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppAssessmentSchedule")]
    pub app_assessment_schedule: Option<String>,


    /// 
    /// A JSON string that provides information about your application structure. To learn more    about the appTemplateBody template, see the sample template provided in the    Examples section.
    /// 
    /// The appTemplateBody JSON string has the following structure:
    /// 
    /// resources     The list of logical resources that needs to be included in the AWS Resilience Hub application.     Type: Array     NoteDon't add the resources that you want to exclude.     Each resources array item includes the following fields:                                        logicalResourceId             The logical identifier of the resource.       Type: Object       Each logicalResourceId object includes the following fields:                                                              identifier         The identifier of the resource.         Type: String                logicalStackName         The name of the AWS CloudFormation stack this resource belongs to.         Type: String                resourceGroupName         The name of the resource group this resource belongs to.         Type: String                terraformSourceName                 The name of the Terraform S3 state file this resource belongs to.         Type: String                eksSourceName                 The name of the Amazon Elastic Kubernetes Service cluster and namespace this resource belongs to.NoteThis parameter accepts values in "eks-cluster/namespace" format.         Type: String                    type       The type of resource.       Type: string            name       The name of the resource.       Type: String            additionalInfo       Additional configuration parameters for an AWS Resilience Hub application.        If you want to implement additionalInfo through the AWS Resilience Hub console rather than using an API call, see        Configure the application configuration parameters.       NoteCurrently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.Key: "failover-regions"Value: "[{"region":"<REGION>", "accounts":[{"id":"<ACCOUNT_ID>"}]}]"              appComponents     The list of Application Components (AppComponent) that this resource belongs to. If an AppComponent is not part of the AWS Resilience Hub application, it will be added.     Type: Array     Each appComponents array item includes the following fields:                                             name       The name of the AppComponent.       Type: String            type       The type of AppComponent. For more information about the types of AppComponent, see Grouping resources in an AppComponent.       Type: String            resourceNames       The list of included resources that are assigned to the AppComponent.       Type: Array of strings            additionalInfo       Additional configuration parameters for an AWS Resilience Hub application.        If you want to implement additionalInfo through the AWS Resilience Hub console rather than using an API call, see        Configure the application configuration parameters.       NoteCurrently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.Key: "failover-regions"Value: "[{"region":"<REGION>", "accounts":[{"id":"<ACCOUNT_ID>"}]}]"              excludedResources     The list of logical resource identifiers to be excluded from the application.     Type: Array     NoteDon't add the resources that you want to include.     Each excludedResources array item includes the following fields:                      logicalResourceIds             The logical identifier of the resource.       Type: Object       NoteYou can configure only one of the following fields:                                                     logicalStackName                  resourceGroupName                  terraformSourceName                  eksSourceName               Each logicalResourceIds object includes the following fields:                                                              identifier         The identifier of the resource.         Type: String                logicalStackName         The name of the AWS CloudFormation stack this resource belongs to.         Type: String                resourceGroupName         The name of the resource group this resource belongs to.         Type: String                terraformSourceName                 The name of the Terraform S3 state file this resource belongs to.         Type: String                eksSourceName                 The name of the Amazon Elastic Kubernetes Service cluster and namespace this resource belongs to.NoteThis parameter accepts values in "eks-cluster/namespace" format.         Type: String                     version     The AWS Resilience Hub application version.        additionalInfo     Additional configuration parameters for an AWS Resilience Hub application.      If you want to implement additionalInfo through the AWS Resilience Hub console rather than using an API call, see      Configure the application configuration parameters.     NoteCurrently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.Key: "failover-regions"Value: "[{"region":"<REGION>", "accounts":[{"id":"<ACCOUNT_ID>"}]}]"
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppTemplateBody")]
    pub app_template_body: String,


    /// 
    /// The optional description for an app.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name for the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the resiliency policy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResiliencyPolicyArn")]
    pub resiliency_policy_arn: Option<String>,


    /// 
    /// An array of ResourceMapping objects.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ResourceMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceMappings")]
    pub resource_mappings: Vec<ResourceMapping>,


    /// 
    /// The tags assigned to the resource. A tag is a label that you assign to an AWS resource. Each tag consists of a key/value pair.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,

}



impl cfn_resources::CfnResource for CfnApp {
    fn type_string(&self) -> &'static str {
        "AWS::ResilienceHub::App"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Defines a physical resource identifier.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PhysicalResourceId {


    /// 
    /// The AWS account that owns the physical resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,


    /// 
    /// The AWS Region that the physical resource is located in.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsRegion")]
    pub aws_region: Option<String>,


    /// 
    /// The identifier of the physical resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Identifier")]
    pub identifier: String,


    /// 
    /// Specifies the type of physical resource identifier.
    /// 
    /// Arn          The resource identifier is an Amazon Resource Name (ARN) .             Native          The resource identifier is an AWS Resilience Hub-native identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}



impl cfn_resources::CfnResource for PhysicalResourceId {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Defines a resource mapping.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceMapping {


    /// 
    /// The name of the CloudFormation stack this resource is mapped to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogicalStackName")]
    pub logical_stack_name: Option<String>,


    /// 
    /// Specifies the type of resource mapping.
    /// 
    /// Valid Values: CfnStack | Resource | AppRegistryApp | ResourceGroup | Terraform
    /// 
    /// AppRegistryApp          The resource is mapped to another application. The name of the application is       contained in the appRegistryAppName property.             CfnStack          The resource is mapped to a CloudFormation stack. The name of the CloudFormation stack is contained in       the logicalStackName property.             Resource          The resource is mapped to another resource. The name of the resource is contained in       the resourceName property.             ResourceGroup          The resource is mapped to a resource group. The name of the resource group is       contained in the resourceGroupName property.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappingType")]
    pub mapping_type: ResourceMappingMappingTypeEnum,


    /// 
    /// The identifier of this resource.
    /// 
    /// Required: Yes
    ///
    /// Type: PhysicalResourceId
    ///
    /// Update requires: No interruption
    #[serde(rename = "PhysicalResourceId")]
    pub physical_resource_id: PhysicalResourceId,


    /// 
    /// The name of the resource this resource is mapped to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,


    /// 
    /// The short name of the Terraform source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerraformSourceName")]
    pub terraform_source_name: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ResourceMappingMappingTypeEnum {

    /// CfnStack
    #[serde(rename = "CfnStack")]
    Cfnstack,

    /// Resource
    #[serde(rename = "Resource")]
    Resource,

    /// AppRegistryApp
    #[serde(rename = "AppRegistryApp")]
    Appregistryapp,

    /// ResourceGroup
    #[serde(rename = "ResourceGroup")]
    Resourcegroup,

    /// Terraform
    #[serde(rename = "Terraform")]
    Terraform,

}

impl Default for ResourceMappingMappingTypeEnum {
    fn default() -> Self {
        ResourceMappingMappingTypeEnum::Cfnstack
    }
}


impl cfn_resources::CfnResource for ResourceMapping {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.physical_resource_id.validate()?;

        Ok(())
    }
}