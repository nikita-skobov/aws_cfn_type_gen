

/// Specifies a new AWS Glue DataBrew project.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProject {


    /// 
    /// The dataset that the project is to act upon.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,


    /// 
    /// The unique name of a project.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The name of a recipe that will be developed during a project session.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecipeName")]
    pub recipe_name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the role that will be assumed for this       project.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The sample size and sampling type to apply to the data. If this parameter isn't       specified, then the sample consists of the first 500 rows from the dataset.
    /// 
    /// Required: No
    ///
    /// Type: Sample
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sample")]
    pub sample: Option<Sample>,


    /// 
    /// Metadata tags that have been applied to the project.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnProject {
    fn type_string(&self) -> &'static str {
        "AWS::DataBrew::Project"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.dataset_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'dataset_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.dataset_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'dataset_name'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.recipe_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'recipe_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.recipe_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'recipe_name'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'role_arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.role_arn;

        if the_val.len() < 20 as _ {
            return Err(format!("Min validation failed on field 'role_arn'. {} is less than 20", the_val.len()));
        }

        
        self.sample.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents the sample size and sampling type for DataBrew to use for interactive data       analysis.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Sample {


    /// 
    /// The number of rows in the sample.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 5000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: Option<i64>,


    /// 
    /// The way in which DataBrew obtains rows from a dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FIRST_N | LAST_N | RANDOM
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: SampleTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SampleTypeEnum {

    /// FIRST_N
    #[serde(rename = "FIRST_N")]
    Firstn,

    /// LAST_N
    #[serde(rename = "LAST_N")]
    Lastn,

    /// RANDOM
    #[serde(rename = "RANDOM")]
    Random,

}

impl Default for SampleTypeEnum {
    fn default() -> Self {
        SampleTypeEnum::Firstn
    }
}


impl cfn_resources::CfnResource for Sample {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.size {

        if *the_val > 5000 as _ {
            return Err(format!("Max validation failed on field 'size'. {} is greater than 5000", the_val));
        }

        }
        
        if let Some(the_val) = &self.size {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'size'. {} is less than 1", the_val));
        }

        }
        
        Ok(())
    }
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
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for Tag {
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