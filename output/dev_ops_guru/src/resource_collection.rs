/// A collection of AWS resources supported by DevOps Guru. The one type of AWS resource 			collection supported is AWS CloudFormation stacks. DevOps Guru can be configured to analyze 			only the AWS resources that are defined in the stacks.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

    #[serde(skip_serializing)]
    pub att_resource_collection_type: CfnResourceCollectionresourcecollectiontype,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceCollectionresourcecollectiontype;
impl CfnResourceCollectionresourcecollectiontype {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceCollectionType"#
    }
}

impl cfn_resources::CfnResource for CfnResourceCollection {
    fn type_string(&self) -> &'static str {
        "AWS::DevOpsGuru::ResourceCollection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.resource_collection_filter.validate()?;

        Ok(())
    }
}

/// Information about AWS CloudFormation stacks. You can use up to 500 			stacks to specify which AWS resources in your account to analyze. For more 			information, see Stacks in the 				        AWS CloudFormation User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_names: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CloudFormationCollectionFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Information about a filter used to specify which AWS resources are analyzed for 			anomalous behavior by DevOps Guru.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResourceCollectionFilter {
    ///
    /// Information about AWS CloudFormation stacks. You can use up to 500 			stacks to specify which AWS resources in your account to analyze. For more 			information, see Stacks in the 				        AWS CloudFormation User Guide.
    ///
    /// Required: No
    ///
    /// Type: CloudFormationCollectionFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudFormation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation: Option<CloudFormationCollectionFilter>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagCollection>>,
}

impl cfn_resources::CfnResource for ResourceCollectionFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_formation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A collection of AWS tags.
///
/// Tags help you identify and organize your AWS resources. Many AWS services support  		tagging, so you can assign the same tag to resources from different services to indicate  		that the resources are related. For example, you can assign the same tag to an Amazon DynamoDB  		table resource that you assign to an AWS Lambda function. For more information about  		using tags, see the Tagging  			best practices whitepaper.
///
/// Each AWS tag has two parts.
///
/// Together these are known as key-value pairs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_boundary_key: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for TagCollection {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.app_boundary_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'app_boundary_key'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.app_boundary_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'app_boundary_key'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
