

/// A collection of AWS resources supported by DevOps Guru. The one type of AWS resource 			collection supported is AWS CloudFormation stacks. DevOps Guru can be configured to analyze 			only the AWS resources that are defined in the stacks.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourceCollection {


    /// 
    /// Information about a filter used to specify which AWS resources are analyzed for anomalous behavior by DevOps Guru.
    /// 
    /// Required: Yes
    ///
    /// Type: ResourceCollectionFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceCollectionFilter")]
    pub resource_collection_filter: ResourceCollectionFilter,

}



impl cfn_resources::CfnResource for CfnResourceCollection {
    fn type_string() -> &'static str {
        "AWS::DevOpsGuru::ResourceCollection"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Information about AWS CloudFormation stacks. You can use up to 500 			stacks to specify which AWS resources in your account to analyze. For more 			information, see Stacks in the 				        AWS CloudFormation User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudFormationCollectionFilter {


    /// 
    /// An array of CloudFormation stack names.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackNames")]
    pub stack_names: Option<Vec<String>>,

}




/// A collection of AWS tags.
///
/// Tags help you identify and organize your AWS resources. Many AWS services support  		tagging, so you can assign the same tag to resources from different services to indicate  		that the resources are related. For example, you can assign the same tag to an Amazon DynamoDB  		table resource that you assign to an AWS Lambda function. For more information about  		using tags, see the Tagging  			best practices whitepaper.
///
/// Each AWS tag has two parts.
///
/// Together these are known as key-value pairs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagCollection {


    /// 
    /// An AWS tag key that is used to identify the AWS resources that    	DevOps Guru analyzes. All AWS resources in your account and Region tagged with this key make    up your DevOps Guru application and analysis boundary.
    /// 
    /// ImportantThe string used for a key in a tag that you use to define your resource coverage must begin with the 			prefix Devops-guru-. The tag key might be 			DevOps-Guru-deployment-application or 			devops-guru-rds-application. When you create a key, the case of characters in the key can be whatever you choose. After you create a key, it is case-sensitive. 			 For example, DevOps Guru works with a 			key named devops-guru-rds and a key named 			DevOps-Guru-RDS, and these act as two different keys. Possible key/value pairs in your 			application might be Devops-Guru-production-application/RDS or 			Devops-Guru-production-application/containers.
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
    #[serde(rename = "AppBoundaryKey")]
    pub app_boundary_key: Option<String>,


    /// 
    /// The values in an AWS tag collection.
    /// 
    /// The tag's value is an optional field used to associate a string with 					the tag key (for example, 111122223333, Production, or a team  				name). The key and value are the tag's key pair.   				Omitting the tag value is the same as using an empty  				string. Like tag keys, tag values are  				case-sensitive. You can specify a maximum of 256 characters for a tag value.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagValues")]
    pub tag_values: Option<Vec<String>>,

}




/// Information about a filter used to specify which AWS resources are analyzed for 			anomalous behavior by DevOps Guru.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceCollectionFilter {


    /// 
    /// The AWS tags used to filter the resources in the resource collection.
    /// 
    /// Tags help you identify and organize your AWS resources. Many AWS services support  		tagging, so you can assign the same tag to resources from different services to indicate  		that the resources are related. For example, you can assign the same tag to an Amazon DynamoDB  		table resource that you assign to an AWS Lambda function. For more information about  		using tags, see the Tagging  			best practices whitepaper.
    /// 
    /// Each AWS tag has two parts.
    /// 
    /// A tag key (for example, CostCenter,  				Environment, Project, or Secret). Tag  				keys are case-sensitive.               An optional field known as a tag value (for example,  				111122223333, Production, or a team  				name). Omitting the tag value is the same as using an empty  				string. Like tag keys, tag values are  				case-sensitive.
    /// 
    /// Together these are known as key-value pairs.
    /// 
    /// ImportantThe string used for a key in a tag that you use to define your resource coverage must begin with the 			prefix Devops-guru-. The tag key might be 			DevOps-Guru-deployment-application or 			devops-guru-rds-application. When you create a key, the case of characters in the key can be whatever you choose. After you create a key, it is case-sensitive. 			 For example, DevOps Guru works with a 			key named devops-guru-rds and a key named 			DevOps-Guru-RDS, and these act as two different keys. Possible key/value pairs in your 			application might be Devops-Guru-production-application/RDS or 			Devops-Guru-production-application/containers.
    /// 
    /// Required: No
    ///
    /// Type: List of TagCollection
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagCollection>>,


    /// 
    /// Information about AWS CloudFormation stacks. You can use up to 500 			stacks to specify which AWS resources in your account to analyze. For more 			information, see Stacks in the 				        AWS CloudFormation User Guide.
    /// 
    /// Required: No
    ///
    /// Type: CloudFormationCollectionFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudFormation")]
    pub cloud_formation: Option<CloudFormationCollectionFilter>,

}


