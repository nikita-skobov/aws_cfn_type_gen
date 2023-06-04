/// Provisions the specified product.
///
/// A provisioned product is a resourced instance of a product. For example, provisioning     a product based on a AWS CloudFormation template launches a AWS CloudFormation stack and its     underlying resources. You can check the status of this request using DescribeRecord.
///
/// If the request contains a tag key with an empty list of values, there is a tag     conflict for that key. Do not include conflicted keys as tags, or this causes the error     "Parameter validation failed: Missing required parameter in       Tags[N]:Value".
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnCloudFormationProvisionedProduct {
    ///
    /// The language code.
    ///
    /// jp - Japanese                        zh - Chinese
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<cfn_resources::StrVal>,

    ///
    /// Passed to AWS CloudFormation. The SNS topic ARNs to which to publish stack-related     events.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotificationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,

    ///
    /// The path identifier of the product. This value is optional if the product has a     default path, and required if the product has more than one path. To list the paths for a     product, use ListLaunchPaths.
    ///
    /// NoteYou must provide the name or ID, but not both.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the path. This value is optional if the product has a     default path, and required if the product has more than one path. To list the paths for a     product, use ListLaunchPaths.
    ///
    /// NoteYou must provide the name or ID, but not both.
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
    #[serde(rename = "PathName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_name: Option<cfn_resources::StrVal>,

    ///
    /// The product identifier.
    ///
    /// NoteYou must specify either the ID or the name of the product,        but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the Service Catalog product.
    ///
    /// Each time a stack is created or updated, if ProductName is provided it will     successfully resolve to ProductId as long as only one product exists in the     account or Region with that ProductName.
    ///
    /// NoteYou must specify either       the name or the ID of the product, but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9][a-zA-Z0-9._-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<cfn_resources::StrVal>,

    ///
    /// A user-friendly name for the provisioned product. This value must be     unique for the AWS account and cannot be updated after the product is provisioned.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9][a-zA-Z0-9._-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<cfn_resources::StrVal>,

    ///
    /// The identifier of the provisioning artifact (also known as a version).
    ///
    /// NoteYou must specify either the ID or the name of the provisioning artifact, but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the provisioning artifact (also known as a version) for the product. This     name must be unique for the product.
    ///
    /// Note You must specify either the name or the ID of the provisioning artifact, but not both. You must also specify either the name or the ID of the product, but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Maximum: 8192
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<cfn_resources::StrVal>,

    ///
    /// Parameters specified by the administrator that are required for provisioning the     product.
    ///
    /// Required: No
    ///
    /// Type: List of ProvisioningParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<ProvisioningParameter>>,

    ///
    /// StackSet preferences that are required for provisioning the product or updating a provisioned product.
    ///
    /// Required: No
    ///
    /// Type: ProvisioningPreferences
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisioningPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_preferences: Option<ProvisioningPreferences>,

    ///
    /// One or more tags.
    ///
    /// NoteRequires the provisioned product to have an ResourceUpdateConstraint resource with       TagUpdatesOnProvisionedProduct set to ALLOWED to allow tag       updates. If RESOURCE_UPDATE constraint is not present, tags updates are ignored.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_cloudformation_stack_arn: CfnCloudFormationProvisionedProductcloudformationstackarn,

    #[serde(skip_serializing)]
    pub att_provisioned_product_id: CfnCloudFormationProvisionedProductprovisionedproductid,

    #[serde(skip_serializing)]
    pub att_record_id: CfnCloudFormationProvisionedProductrecordid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFormationProvisionedProductcloudformationstackarn;
impl CfnCloudFormationProvisionedProductcloudformationstackarn {
    pub fn att_name(&self) -> &'static str {
        r#"CloudformationStackArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFormationProvisionedProductprovisionedproductid;
impl CfnCloudFormationProvisionedProductprovisionedproductid {
    pub fn att_name(&self) -> &'static str {
        r#"ProvisionedProductId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFormationProvisionedProductrecordid;
impl CfnCloudFormationProvisionedProductrecordid {
    pub fn att_name(&self) -> &'static str {
        r#"RecordId"#
    }
}

impl cfn_resources::CfnResource for CfnCloudFormationProvisionedProduct {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::CloudFormationProvisionedProduct"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.accept_language {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'accept_language'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.notification_arns {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'notification_arns'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.path_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'path_id'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'path_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'path_name'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'path_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.product_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'product_id'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.product_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'product_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.product_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'product_name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.product_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'product_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.provisioned_product_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'provisioned_product_name'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.provisioned_product_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'provisioned_product_name'. {} is less than 1", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.provisioning_artifact_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!("Max validation failed on field 'provisioning_artifact_id'. {} is greater than 100", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.provisioning_artifact_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'provisioning_artifact_id'. {} is less than 1", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.provisioning_artifact_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 8192 as _ {
                    return Err(format!("Max validation failed on field 'provisioning_artifact_name'. {} is greater than 8192", s.len()));
                }
            }
        }

        self.provisioning_preferences
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Information about a parameter used to provision a product.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvisioningParameter {
    ///
    /// The parameter key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The parameter value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ProvisioningParameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'key'. {} is greater than 1000",
                    s.len()
                ));
            }
        }

        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'key'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 4096",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The user-defined preferences that will be applied when updating a provisioned     product. Not all preferences are applicable to all provisioned product type
///
/// One or more AWS accounts that will have access to the provisioned product.
///
/// Applicable only to a CFN_STACKSET provisioned product type.
///
/// The AWS accounts specified should be within the list of accounts in the       STACKSET constraint. To get the list of accounts in the       STACKSET constraint, use the DescribeProvisioningParameters     operation.
///
/// If no values are specified, the default value is all accounts from the       STACKSET constraint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvisioningPreferences {
    ///
    /// One or more AWS accounts where the provisioned product will be available.
    ///
    /// Applicable only to a CFN_STACKSET provisioned product type.
    ///
    /// The specified accounts should be within the list of accounts from the STACKSET constraint. To get the list of accounts in the STACKSET constraint, use the DescribeProvisioningParameters operation.
    ///
    /// If no values are specified, the default value is all acounts from the STACKSET constraint.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackSetAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,

    ///
    /// The number of accounts, per Region, for which this operation can fail before AWS Service Catalog stops the operation in that Region. If the operation is stopped in a Region, AWS Service Catalog doesn't attempt the operation in any subsequent Regions.
    ///
    /// Applicable only to a CFN_STACKSET provisioned product type.
    ///
    /// Conditional: You must specify either StackSetFailureToleranceCount or StackSetFailureTolerancePercentage, but not both.
    ///
    /// The default value is 0 if no value is specified.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackSetFailureToleranceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_count: Option<i64>,

    ///
    /// The percentage of accounts, per Region, for which this stack operation can fail before AWS Service Catalog stops the operation in that Region. If the operation is stopped in a Region, AWS Service Catalog doesn't attempt the operation in any subsequent Regions.
    ///
    /// When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number.
    ///
    /// Applicable only to a CFN_STACKSET provisioned product type.
    ///
    /// Conditional: You must specify either StackSetFailureToleranceCount or StackSetFailureTolerancePercentage, but not both.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackSetFailureTolerancePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_percentage: Option<i64>,

    ///
    /// The maximum number of accounts in which to perform this operation at one time. This is dependent on the value of StackSetFailureToleranceCount. StackSetMaxConcurrentCount is at most one more than the StackSetFailureToleranceCount.
    ///
    /// Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.
    ///
    /// Applicable only to a CFN_STACKSET provisioned product type.
    ///
    /// Conditional: You must specify either StackSetMaxConcurrentCount or StackSetMaxConcurrentPercentage, but not both.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackSetMaxConcurrencyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_count: Option<i64>,

    ///
    /// The maximum percentage of accounts in which to perform this operation at one time.
    ///
    /// When calculating the number of accounts based on the specified percentage, AWS Service Catalog rounds down to the next whole number. This is true except in cases where rounding down would result is zero. In this case, AWS Service Catalog sets the number as 1 instead.
    ///
    /// Note that this setting lets you specify the maximum for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.
    ///
    /// Applicable only to a CFN_STACKSET provisioned product type.
    ///
    /// Conditional: You must specify either StackSetMaxConcurrentCount or StackSetMaxConcurrentPercentage, but not both.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackSetMaxConcurrencyPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_percentage: Option<i64>,

    ///
    /// Determines what action AWS Service Catalog performs to a stack set or a stack instance represented by the provisioned product. The default value is UPDATE if nothing is specified.
    ///
    /// Applicable only to a CFN_STACKSET provisioned product type.
    ///
    /// CREATE                  Creates a new stack instance in the stack set represented by the provisioned product. In this case, only new stack instances are created based on accounts and Regions; if new ProductId or ProvisioningArtifactID are passed, they will be ignored.                       UPDATE                  Updates the stack set represented by the provisioned product and also its stack instances.                       DELETE                  Deletes a stack instance in the stack set represented by the provisioned product.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CREATE | DELETE | UPDATE
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackSetOperationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_operation_type: Option<ProvisioningPreferencesStackSetOperationTypeEnum>,

    ///
    /// One or more AWS Regions where the provisioned product will be available.
    ///
    /// Applicable only to a CFN_STACKSET provisioned product type.
    ///
    /// The specified Regions should be within the list of Regions from the STACKSET constraint. To get the list of Regions in the STACKSET constraint, use the DescribeProvisioningParameters operation.
    ///
    /// If no values are specified, the default value is all Regions from the STACKSET constraint.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackSetRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ProvisioningPreferencesStackSetOperationTypeEnum {
    /// CREATE
    #[serde(rename = "CREATE")]
    Create,

    /// DELETE
    #[serde(rename = "DELETE")]
    Delete,

    /// UPDATE
    #[serde(rename = "UPDATE")]
    Update,
}

impl Default for ProvisioningPreferencesStackSetOperationTypeEnum {
    fn default() -> Self {
        ProvisioningPreferencesStackSetOperationTypeEnum::Create
    }
}

impl cfn_resources::CfnResource for ProvisioningPreferences {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.stack_set_failure_tolerance_count {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'stack_set_failure_tolerance_count'. {} is less than 0", the_val));
            }
        }

        if let Some(the_val) = &self.stack_set_failure_tolerance_percentage {
            if *the_val > 100 as _ {
                return Err(format!("Max validation failed on field 'stack_set_failure_tolerance_percentage'. {} is greater than 100", the_val));
            }
        }

        if let Some(the_val) = &self.stack_set_failure_tolerance_percentage {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'stack_set_failure_tolerance_percentage'. {} is less than 0", the_val));
            }
        }

        if let Some(the_val) = &self.stack_set_max_concurrency_count {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'stack_set_max_concurrency_count'. {} is less than 1", the_val));
            }
        }

        if let Some(the_val) = &self.stack_set_max_concurrency_percentage {
            if *the_val > 100 as _ {
                return Err(format!("Max validation failed on field 'stack_set_max_concurrency_percentage'. {} is greater than 100", the_val));
            }
        }

        if let Some(the_val) = &self.stack_set_max_concurrency_percentage {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'stack_set_max_concurrency_percentage'. {} is less than 1", the_val));
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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
