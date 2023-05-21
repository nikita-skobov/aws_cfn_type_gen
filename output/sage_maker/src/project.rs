

/// Creates a machine learning (ML) project that can contain one or more templates that set       up an ML pipeline from training to deploying an approved model.
#[derive(Default, serde::Serialize)]
pub struct CfnProject {


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
    pub tags: Option<Vec<Tag>>,


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
    pub project_name: String,


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
    pub project_description: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// Details of a provisioned service catalog product. For information about service catalog,       see What is AWS Service         Catalog.
#[derive(Default, serde::Serialize)]
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
    pub provisioned_product_id: Option<String>,


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
    pub provisioned_product_status_message: Option<String>,

}


/// Details that you specify to provision a service catalog product. For information about       service catalog, see What is AWS Service         Catalog.
#[derive(Default, serde::Serialize)]
pub struct ServiceCatalogProvisioningDetails {


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
    pub product_id: String,


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
    pub path_id: Option<String>,


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
    pub provisioning_artifact_id: Option<String>,


    /// 
    /// A list of key value pairs that you specify when you provision a product.
    /// 
    /// Required: No
    ///
    /// Type: List of ProvisioningParameter
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProvisioningParameters")]
    pub provisioning_parameters: Option<Vec<ProvisioningParameter>>,

}


/// A key value pair used when you provision a project as a service catalog product. For       information, see What is AWS Service         Catalog.
#[derive(Default, serde::Serialize)]
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
    pub key: String,


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
    pub value: String,

}
