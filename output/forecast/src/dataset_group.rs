/// Creates a dataset group, which holds a collection of related datasets. You can add    datasets to the dataset group when you create the dataset group, or later by using the UpdateDatasetGroup operation.
///
/// After creating a dataset group and adding datasets, you use the dataset group when you    create a predictor. For more information, see Dataset groups.
///
/// To get a list of all your datasets groups, use the ListDatasetGroups    operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDatasetGroup {
    ///
    /// An array of Amazon Resource Names (ARNs) of the datasets that you want to include in the    dataset group.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetArns")]
    pub dataset_arns: Option<Vec<String>>,

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
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetGroupName")]
    pub dataset_group_name: String,

    ///
    /// The domain associated with the dataset group. When you add a dataset to a dataset group,    this value and the value specified for the Domain parameter of the CreateDataset    operation must match.
    ///
    /// The Domain and DatasetType that you choose determine the fields    that must be present in training data that you import to a dataset. For example, if you choose    the RETAIL domain and TARGET_TIME_SERIES as the     DatasetType, Amazon Forecast requires that item_id,     timestamp, and demand fields are present in your data. For more    information, see Dataset groups.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOM | EC2_CAPACITY | INVENTORY_PLANNING | METRICS | RETAIL | WEB_TRAFFIC | WORK_FORCE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domain")]
    pub domain: DatasetGroupDomainEnum,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DatasetGroupDomainEnum {
    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// EC2_CAPACITY
    #[serde(rename = "EC2_CAPACITY")]
    Ec2capacity,

    /// INVENTORY_PLANNING
    #[serde(rename = "INVENTORY_PLANNING")]
    Inventoryplanning,

    /// METRICS
    #[serde(rename = "METRICS")]
    Metrics,

    /// RETAIL
    #[serde(rename = "RETAIL")]
    Retail,

    /// WEB_TRAFFIC
    #[serde(rename = "WEB_TRAFFIC")]
    Webtraffic,

    /// WORK_FORCE
    #[serde(rename = "WORK_FORCE")]
    Workforce,
}

impl Default for DatasetGroupDomainEnum {
    fn default() -> Self {
        DatasetGroupDomainEnum::Custom
    }
}

impl cfn_resources::CfnResource for CfnDatasetGroup {
    fn type_string(&self) -> &'static str {
        "AWS::Forecast::DatasetGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.dataset_group_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'dataset_group_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.dataset_group_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'dataset_group_name'. {} is less than 1",
                the_val.len()
            ));
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
