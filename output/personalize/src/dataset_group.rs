

/// A dataset group is a collection of related datasets (Interactions,    User, and Item). You create a dataset group by calling CreateDatasetGroup. You then create a dataset and add it to a    dataset group by calling CreateDataset. The dataset group is used to create and train a    solution by calling CreateSolution. A dataset group can contain only one of each    type of dataset.
///
/// You can specify an AWS Key Management Service (KMS) key to encrypt the datasets in    the group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDatasetGroup {


    /// 
    /// The domain of a Domain dataset group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ECOMMERCE | VIDEO_ON_DEMAND
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    pub domain: Option<DatasetGroupDomainEnum>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Key Management Service (KMS) key used to    encrypt the datasets.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws.*:kms:.*:[0-9]{12}:key/.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,


    /// 
    /// The name of the dataset group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The ARN of the IAM role that has permissions to create the dataset    group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DatasetGroupDomainEnum {

    /// ECOMMERCE
    #[serde(rename = "ECOMMERCE")]
    Ecommerce,

    /// VIDEO_ON_DEMAND
    #[serde(rename = "VIDEO_ON_DEMAND")]
    Videoondemand,

}

impl Default for DatasetGroupDomainEnum {
    fn default() -> Self {
        DatasetGroupDomainEnum::Ecommerce
    }
}


impl cfn_resources::CfnResource for CfnDatasetGroup {
    fn type_string() -> &'static str {
        "AWS::Personalize::DatasetGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.kms_key_arn {

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'kms_key_arn'. {} is greater than 2048", the_val.len()));
        }

        }
        
        let the_val = &self.name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 63", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.role_arn {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'role_arn'. {} is greater than 256", the_val.len()));
        }

        }
        
        Ok(())
    }
}