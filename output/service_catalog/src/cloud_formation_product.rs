/// Specifies a product.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnCloudFormationProduct {
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub accept_language: Option<cfn_resources::StrVal>,

    ///
    /// The description of the product.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 8191
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The distributor of the product.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 8191
    ///
    /// Update requires: No interruption
    #[serde(rename = "Distributor")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub distributor: Option<cfn_resources::StrVal>,

    ///
    /// The name of the product.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 8191
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The owner of the product.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 8191
    ///
    /// Update requires: No interruption
    #[serde(rename = "Owner")]
    pub owner: cfn_resources::StrVal,

    ///
    /// The type of product.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CLOUD_FORMATION_TEMPLATE | MARKETPLACE | TERRAFORM_OPEN_SOURCE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub product_type: Option<CloudFormationProductProductTypeEnum>,

    ///
    /// The configuration of the provisioning artifact (also known as a version).
    ///
    /// Required: No
    ///
    /// Type: List of ProvisioningArtifactProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisioningArtifactParameters")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub provisioning_artifact_parameters: Option<Vec<ProvisioningArtifactProperties>>,

    ///
    /// This property is turned off by default. If turned off, you can update provisioning artifacts or product attributes (such as description, distributor, name, owner, and more)      and the associated provisioning artifacts will retain the same unique identifier. Provisioning artifacts are matched within the CloudFormationProduct resource, and only those that have been updated will be    changed. Provisioning artifacts are matched by a combinaton of provisioning artifact template URL and name.
    ///
    /// If turned on, provisioning artifacts will be given a new unique identifier when you update the product or provisioning artifacts.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplaceProvisioningArtifacts")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub replace_provisioning_artifacts: Option<bool>,

    ///
    /// A top level ProductViewDetail response containing details about the product’s connection.     AWS Service Catalog returns this field for the CreateProduct, UpdateProduct,      DescribeProductAsAdmin, and SearchProductAsAdmin APIs.      This response contains the same fields as the ConnectionParameters request, with the      addition of the LastSync response.
    ///
    /// Required: No
    ///
    /// Type: SourceConnection
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceConnection")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub source_connection: Option<SourceConnection>,

    ///
    /// The support information about the product.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 8191
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportDescription")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub support_description: Option<cfn_resources::StrVal>,

    ///
    /// The contact email for product support.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 254
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportEmail")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub support_email: Option<cfn_resources::StrVal>,

    ///
    /// The contact URL for product support.
    ///
    /// ^https?:\/\// / is the pattern used to validate SupportUrl.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2083
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportUrl")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub support_url: Option<cfn_resources::StrVal>,

    ///
    /// One or more tags.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_product_name: CfnCloudFormationProductproductname,

    #[serde(skip_serializing)]
    pub att_provisioning_artifact_ids: CfnCloudFormationProductprovisioningartifactids,

    #[serde(skip_serializing)]
    pub att_provisioning_artifact_names: CfnCloudFormationProductprovisioningartifactnames,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CloudFormationProductProductTypeEnum {
    /// CLOUD_FORMATION_TEMPLATE
    #[serde(rename = "CLOUD_FORMATION_TEMPLATE")]
    Cloudformationtemplate,

    /// MARKETPLACE
    #[serde(rename = "MARKETPLACE")]
    Marketplace,

    /// TERRAFORM_OPEN_SOURCE
    #[serde(rename = "TERRAFORM_OPEN_SOURCE")]
    Terraformopensource,
}

impl Default for CloudFormationProductProductTypeEnum {
    fn default() -> Self {
        CloudFormationProductProductTypeEnum::Cloudformationtemplate
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFormationProductproductname;
impl CfnCloudFormationProductproductname {
    pub fn att_name(&self) -> &'static str {
        r#"ProductName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFormationProductprovisioningartifactids;
impl CfnCloudFormationProductprovisioningartifactids {
    pub fn att_name(&self) -> &'static str {
        r#"ProvisioningArtifactIds"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCloudFormationProductprovisioningartifactnames;
impl CfnCloudFormationProductprovisioningartifactnames {
    pub fn att_name(&self) -> &'static str {
        r#"ProvisioningArtifactNames"#
    }
}

impl cfn_resources::CfnResource for CfnCloudFormationProduct {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::CloudFormationProduct"
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

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 8191 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 8191",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.distributor {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 8191 as _ {
                    return Err(format!(
                        "Max validation failed on field 'distributor'. {} is greater than 8191",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 8191 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 8191",
                    s.len()
                ));
            }
        }

        let the_val = &self.owner;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 8191 as _ {
                return Err(format!(
                    "Max validation failed on field 'owner'. {} is greater than 8191",
                    s.len()
                ));
            }
        }

        self.source_connection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.support_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 8191 as _ {
                    return Err(format!("Max validation failed on field 'support_description'. {} is greater than 8191", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.support_email {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 254 as _ {
                    return Err(format!(
                        "Max validation failed on field 'support_email'. {} is greater than 254",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.support_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2083 as _ {
                    return Err(format!(
                        "Max validation failed on field 'support_url'. {} is greater than 2083",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The subtype containing details about the Codestar connection Type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CodeStarParameters {
    ///
    /// The absolute path wehre the artifact resides within the repo and branch, formatted as      "folder/file.json."
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactPath")]
    pub artifact_path: cfn_resources::StrVal,

    ///
    /// The specific branch where the artifact resides.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 250
    ///
    /// Update requires: No interruption
    #[serde(rename = "Branch")]
    pub branch: cfn_resources::StrVal,

    ///
    /// The CodeStar ARN, which is the connection between AWS Service Catalog and the external repository.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1224
    ///
    /// Pattern: arn:[a-z0-9][-.a-z0-9]{0,62}:codestar-connections:([a-z0-9][-.a-z0-9]{0,62})?:([a-z0-9][-.a-z0-9]{0,62})?:[^/].{0,1023}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: cfn_resources::StrVal,

    ///
    /// The specific repository where the product’s artifact-to-be-synced resides, formatted as      "Account/Repo."
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Repository")]
    pub repository: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CodeStarParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.artifact_path;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'artifact_path'. {} is greater than 4096",
                    s.len()
                ));
            }
        }

        let the_val = &self.artifact_path;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'artifact_path'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.branch;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 250 as _ {
                return Err(format!(
                    "Max validation failed on field 'branch'. {} is greater than 250",
                    s.len()
                ));
            }
        }

        let the_val = &self.branch;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'branch'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.connection_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1224 as _ {
                return Err(format!(
                    "Max validation failed on field 'connection_arn'. {} is greater than 1224",
                    s.len()
                ));
            }
        }

        let the_val = &self.connection_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'connection_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.repository;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'repository'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.repository;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'repository'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Provides connection details.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConnectionParameters {
    ///
    /// Provides ConnectionType details.
    ///
    /// Required: No
    ///
    /// Type: CodeStarParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeStar")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub code_star: Option<CodeStarParameters>,
}

impl cfn_resources::CfnResource for ConnectionParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.code_star
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about a provisioning artifact (also known as a version) for a product.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvisioningArtifactProperties {
    ///
    /// The description of the provisioning artifact, including how it differs from the previous provisioning artifact.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 8192
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// If set to true, AWS Service Catalog stops validating the specified provisioning artifact even if it is invalid.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableTemplateValidation")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub disable_template_validation: Option<bool>,

    ///
    /// Specify the template source with one of the following options, but not both.     Keys accepted: [ LoadTemplateFromURL, ImportFromPhysicalId ]
    ///
    /// The URL of the AWS CloudFormation template in Amazon S3 in JSON format.    Specify the URL in JSON format as follows:
    ///
    /// "LoadTemplateFromURL": "https://s3.amazonaws.com/cf-templates-ozkq9d3hgiq2-us-east-1/..."
    ///
    /// ImportFromPhysicalId: The physical id of the resource that contains the     template. Currently only supports AWS CloudFormation stack arn. Specify the physical id in JSON     format as follows: ImportFromPhysicalId: “arn:aws:cloudformation:[us-east-1]:[accountId]:stack/[StackName]/[resourceId]
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Info")]
    pub info: serde_json::Value,

    ///
    /// The name of the provisioning artifact (for example, v1 v2beta). No spaces are allowed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 8192
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The type of provisioning artifact.
    ///
    /// CLOUD_FORMATION_TEMPLATE - AWS CloudFormation template                        MARKETPLACE_AMI - AWS Marketplace AMI                        MARKETPLACE_CAR - AWS Marketplace Clusters and AWS Resources                        TERRAFORM_OPEN_SOURCE - Terraform open source configuration file
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CLOUD_FORMATION_TEMPLATE | MARKETPLACE_AMI | MARKETPLACE_CAR | TERRAFORM_OPEN_SOURCE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cfn_type: Option<ProvisioningArtifactPropertiesTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ProvisioningArtifactPropertiesTypeEnum {
    /// CLOUD_FORMATION_TEMPLATE
    #[serde(rename = "CLOUD_FORMATION_TEMPLATE")]
    Cloudformationtemplate,

    /// MARKETPLACE_AMI
    #[serde(rename = "MARKETPLACE_AMI")]
    Marketplaceami,

    /// MARKETPLACE_CAR
    #[serde(rename = "MARKETPLACE_CAR")]
    Marketplacecar,

    /// TERRAFORM_OPEN_SOURCE
    #[serde(rename = "TERRAFORM_OPEN_SOURCE")]
    Terraformopensource,
}

impl Default for ProvisioningArtifactPropertiesTypeEnum {
    fn default() -> Self {
        ProvisioningArtifactPropertiesTypeEnum::Cloudformationtemplate
    }
}

impl cfn_resources::CfnResource for ProvisioningArtifactProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 8192 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 8192",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 8192 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 8192",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A top level ProductViewDetail response containing details about the product’s connection.     AWS Service Catalog returns this field for the CreateProduct, UpdateProduct,      DescribeProductAsAdmin, and SearchProductAsAdmin APIs.      This response contains the same fields as the ConnectionParameters request, with the      addition of the LastSync response.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceConnection {
    ///
    /// The connection details based on the connection Type.
    ///
    /// Required: Yes
    ///
    /// Type: ConnectionParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionParameters")]
    pub connection_parameters: ConnectionParameters,

    ///
    /// The only supported SourceConnection type is Codestar.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SourceConnection {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.connection_parameters.validate()?;

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
