

/// The AWS::Athena::DataCatalog resource specifies an Amazon Athena data catalog, which       contains a name, description, type, parameters, and tags. For more information, see         DataCatalog in the Amazon Athena API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataCatalog {


    /// 
    /// A description of the data catalog.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the data catalog. The catalog name must be unique for the AWS account and can use a maximum of 128 alphanumeric, underscore, at sign,       or hyphen characters.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Specifies the Lambda function or functions to use for the data catalog. The mapping       used depends on the catalog type.
    /// 
    /// The HIVE data catalog type uses the following syntax. The             metadata-function parameter is required. The             sdk-version parameter is optional and defaults to the currently           supported version.         metadata-function=lambda_arn,               sdk-version=version_number               The LAMBDA data catalog type uses one of the following sets of           required parameters, but not both.                                                    When one Lambda function processes metadata and another Lambda               function reads data, the following syntax is used. Both parameters are               required.             metadata-function=lambda_arn,                   record-function=lambda_arn                       A composite Lambda function that processes both metadata and data uses               the following syntax.             function=lambda_arn                          The GLUE type takes a catalog ID parameter and is required. The               catalog_id is the account ID of the             AWS account to which the Glue catalog belongs.         catalog-id=catalog_id                                                               The GLUE data catalog type also applies to the default                 AwsDataCatalog that already exists in your account, of               which you can have only one and cannot modify.                       Queries that specify a GLUE data catalog other than the default                 AwsDataCatalog must be run on Athena engine version               2.                       In Regions where Athena engine version 2 is not available, creating               new GLUE data catalogs results in an INVALID_INPUT               error.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The tags (key-value pairs) to associate with this resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The type of data catalog: LAMBDA for a federated catalog,         GLUE for AWS Glue Catalog, or HIVE for an external hive       metastore.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}



impl cfn_resources::CfnResource for CfnDataCatalog {
    fn type_string() -> &'static str {
        "AWS::Athena::DataCatalog"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}