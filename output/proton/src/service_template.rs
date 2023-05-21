

/// Create a service template. The administrator creates a service template to define    standardized infrastructure and an optional CI/CD service pipeline. Developers, in turn,    select the service template from AWS Proton. If the selected service template includes a    service pipeline definition, they provide a link to their source code repository. AWS Proton    then deploys and manages the infrastructure defined by the selected service template. For more    information, see AWS Proton templates in the AWS Proton User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServiceTemplate {


    /// 
    /// A description of the service template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The service template name as displayed in the developer interface.
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
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,


    /// 
    /// The customer provided service template encryption key that's used to encrypt data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov):[a-zA-Z0-9-]+:[a-zA-Z0-9-]*:\d{12}:([\w+=,.@-]+[/:])*[\w+=,.@-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionKey")]
    pub encryption_key: Option<String>,


    /// 
    /// The name of the service template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[0-9A-Za-z]+[0-9A-Za-z_\-]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// If pipelineProvisioning is true, a service pipeline is included    in the service template. Otherwise, a service pipeline isn't included in    the service template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOMER_MANAGED
    ///
    /// Update requires: Replacement
    #[serde(rename = "PipelineProvisioning")]
    pub pipeline_provisioning: Option<ServiceTemplatePipelineProvisioningEnum>,


    /// 
    /// An object that includes the template bundle S3 bucket path and name for the new version of    a service template.
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
pub enum ServiceTemplatePipelineProvisioningEnum {

    /// CUSTOMER_MANAGED
    #[serde(rename = "CUSTOMER_MANAGED")]
    Customermanaged,

}

impl Default for ServiceTemplatePipelineProvisioningEnum {
    fn default() -> Self {
        ServiceTemplatePipelineProvisioningEnum::Customermanaged
    }
}


impl cfn_resources::CfnResource for CfnServiceTemplate {
    fn type_string() -> &'static str {
        "AWS::Proton::ServiceTemplate"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.description {

        if the_val.len() > 500 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 500", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 0", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.display_name {

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'display_name'. {} is greater than 100", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.display_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'display_name'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.encryption_key {

        if the_val.len() > 200 as _ {
            return Err(format!("Max validation failed on field 'encryption_key'. {} is greater than 200", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.encryption_key {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'encryption_key'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 100", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
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