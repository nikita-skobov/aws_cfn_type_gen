/// Creates a machine learning (ML) project that can contain one or more templates that set       up an ML pipeline from training to deploying an approved model.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnProject {
    ///
    /// The description of the project.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\p{L}\p{M}\p{Z}\p{S}\p{N}\p{P}]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the project.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,31}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectName")]
    pub project_name: cfn_resources::StrVal,

    ///
    /// The product ID and provisioning artifact ID to provision a service catalog. For       information, see What is AWS Service Catalog.
    ///
    /// Required: Yes
    ///
    /// Type: ServiceCatalogProvisioningDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceCatalogProvisioningDetails")]
    pub service_catalog_provisioning_details: ServiceCatalogProvisioningDetails,

    ///
    /// A list of key-value pairs to apply to this resource.
    ///
    /// For more information, see Resource         Tag and Using         Cost Allocation Tags in the         AWS Billing and Cost Management User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnProjectcreationtime,

    #[serde(skip_serializing)]
    pub att_project_arn: CfnProjectprojectarn,

    #[serde(skip_serializing)]
    pub att_project_id: CfnProjectprojectid,

    #[serde(skip_serializing)]
    pub att_project_status: CfnProjectprojectstatus,

    #[serde(skip_serializing)]
    pub att_service_catalog_provisioned_product_details_provisioned_product_id:
        CfnProjectservicecatalogprovisionedproductdetailsprovisionedproductid,

    #[serde(skip_serializing)]
    pub att_service_catalog_provisioned_product_details_provisioned_product_status_message:
        CfnProjectservicecatalogprovisionedproductdetailsprovisionedproductstatusmessage,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectcreationtime;
impl CfnProjectcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectprojectarn;
impl CfnProjectprojectarn {
    pub fn att_name(&self) -> &'static str {
        r#"ProjectArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectprojectid;
impl CfnProjectprojectid {
    pub fn att_name(&self) -> &'static str {
        r#"ProjectId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectprojectstatus;
impl CfnProjectprojectstatus {
    pub fn att_name(&self) -> &'static str {
        r#"ProjectStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectservicecatalogprovisionedproductdetailsprovisionedproductid;
impl CfnProjectservicecatalogprovisionedproductdetailsprovisionedproductid {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceCatalogProvisionedProductDetails.ProvisionedProductId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProjectservicecatalogprovisionedproductdetailsprovisionedproductstatusmessage;
impl CfnProjectservicecatalogprovisionedproductdetailsprovisionedproductstatusmessage {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceCatalogProvisionedProductDetails.ProvisionedProductStatusMessage"#
    }
}

impl cfn_resources::CfnResource for CfnProject {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::Project"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.project_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'project_description'. {} is greater than 1024", s.len()));
                }
            }
        }

        let the_val = &self.project_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'project_name'. {} is greater than 32",
                    s.len()
                ));
            }
        }

        let the_val = &self.project_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'project_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.service_catalog_provisioning_details.validate()?;

        Ok(())
    }
}

/// A key value pair used when you provision a project as a service catalog product. For       information, see What is AWS Service         Catalog.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvisioningParameter {
    ///
    /// The key that identifies a provisioning parameter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value of the provisioning parameter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
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

/// Details of a provisioned service catalog product. For information about service catalog,       see What is AWS Service         Catalog.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServiceCatalogProvisionedProductDetails {
    ///
    /// The ID of the provisioned product.
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
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<cfn_resources::StrVal>,

    ///
    /// The current status of the product.
    ///
    /// AVAILABLE - Stable state, ready to perform any operation. The most recent operation succeeded and completed.                        UNDER_CHANGE - Transitive state. Operations performed might not have valid results. Wait for an AVAILABLE status before performing operations.                        TAINTED - Stable state, ready to perform any operation. The stack has completed the requested operation but is not exactly what was requested. For example, a request to update to a new version failed and the stack rolled back to the current version.                        ERROR - An unexpected error occurred. The provisioned product exists but the stack is not running. For example, CloudFormation received a parameter value that was not valid and could not launch the stack.                        PLAN_IN_PROGRESS - Transitive state. The plan operations were performed to provision a new product, but resources have not yet been created. After reviewing the list of resources to be created, execute the plan. Wait for an AVAILABLE status before performing operations.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedProductStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_status_message: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ServiceCatalogProvisionedProductDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.provisioned_product_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!("Max validation failed on field 'provisioned_product_id'. {} is greater than 100", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.provisioned_product_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'provisioned_product_id'. {} is less than 1", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Details that you specify to provision a service catalog product. For information about       service catalog, see What is AWS Service         Catalog.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServiceCatalogProvisioningDetails {
    ///
    /// The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path.
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
    /// Update requires: Replacement
    #[serde(rename = "PathId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the product to provision.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[a-zA-Z0-9_\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProductId")]
    pub product_id: cfn_resources::StrVal,

    ///
    /// The ID of the provisioning artifact.
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
    /// Update requires: Replacement
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<cfn_resources::StrVal>,

    ///
    /// A list of key value pairs that you specify when you provision a product.
    ///
    /// Required: No
    ///
    /// Type: List of ProvisioningParameter
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProvisioningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<ProvisioningParameter>>,
}

impl cfn_resources::CfnResource for ServiceCatalogProvisioningDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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

        let the_val = &self.product_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'product_id'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.product_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'product_id'. {} is less than 1",
                    s.len()
                ));
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
