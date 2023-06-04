/// A dataset group is a collection of related datasets (Interactions,    User, and Item). You create a dataset group by calling CreateDatasetGroup. You then create a dataset and add it to a    dataset group by calling CreateDataset. The dataset group is used to create and train a    solution by calling CreateSolution. A dataset group can contain only one of each    type of dataset.
///
/// You can specify an AWS Key Management Service (KMS) key to encrypt the datasets in    the group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<cfn_resources::StrVal>,

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
    pub name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_dataset_group_arn: CfnDatasetGroupdatasetgrouparn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDatasetGroupdatasetgrouparn;
impl CfnDatasetGroupdatasetgrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"DatasetGroupArn"#
    }
}

impl cfn_resources::CfnResource for CfnDatasetGroup {
    fn type_string(&self) -> &'static str {
        "AWS::Personalize::DatasetGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.kms_key_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
