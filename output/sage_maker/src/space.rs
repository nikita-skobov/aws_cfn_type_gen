

/// Creates a space used for real time collaboration in a Domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSpace {


    /// 
    /// The ID of the associated Domain.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainId")]
    pub domain_id: String,


    /// 
    /// The name of the space.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Replacement
    #[serde(rename = "SpaceName")]
    pub space_name: String,


    /// 
    /// A collection of space settings.
    /// 
    /// Required: No
    ///
    /// Type: SpaceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpaceSettings")]
    pub space_settings: Option<SpaceSettings>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnSpace {
    fn type_string() -> &'static str {
        "AWS::SageMaker::Space"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.domain_id;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'domain_id'. {} is greater than 63", the_val.len()));
        }

        
        let the_val = &self.space_name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'space_name'. {} is greater than 63", the_val.len()));
        }

        
        self.space_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.tags {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 50", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// A custom SageMaker image. For more information, see    Bring your own SageMaker image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomImage {


    /// 
    /// The name of the AppImageConfig.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppImageConfigName")]
    pub app_image_config_name: String,


    /// 
    /// The name of the CustomImage. Must be unique to your account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9]([-.]?[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageName")]
    pub image_name: String,


    /// 
    /// The version number of the CustomImage.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageVersionNumber")]
    pub image_version_number: Option<i64>,

}



impl cfn_resources::CfnResource for CustomImage {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.app_image_config_name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'app_image_config_name'. {} is greater than 63", the_val.len()));
        }

        
        let the_val = &self.image_name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'image_name'. {} is greater than 63", the_val.len()));
        }

        
        let the_val = &self.image_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'image_name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.image_version_number {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'image_version_number'. {} is less than 0", the_val));
        }

        }
        
        Ok(())
    }
}

/// The JupyterServer app settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JupyterServerAppSettings {


    /// 
    /// The default instance type and the Amazon Resource Name (ARN) of the default SageMaker image used by the JupyterServer app. If you use the LifecycleConfigArns parameter, then this parameter is also required.
    /// 
    /// Required: No
    ///
    /// Type: ResourceSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}



impl cfn_resources::CfnResource for JupyterServerAppSettings {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.default_resource_spec.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The KernelGateway app settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KernelGatewayAppSettings {


    /// 
    /// A list of custom SageMaker images that are configured to run as a KernelGateway app.
    /// 
    /// Required: No
    ///
    /// Type: List of CustomImage
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomImages")]
    pub custom_images: Option<Vec<CustomImage>>,


    /// 
    /// The default instance type and the Amazon Resource Name (ARN) of the default SageMaker image used by the KernelGateway app.
    /// 
    /// NoteThe Amazon SageMaker Studio UI does not use the default instance type value set here. The default      instance type set here is used when Apps are created using the AWS Command Line Interface or AWS CloudFormation       and the instance type parameter value is not passed.
    /// 
    /// Required: No
    ///
    /// Type: ResourceSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}



impl cfn_resources::CfnResource for KernelGatewayAppSettings {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.custom_images {

        if the_val.len() > 200 as _ {
            return Err(format!("Max validation failed on field 'custom_images'. {} is greater than 200", the_val.len()));
        }

        }
        
        self.default_resource_spec.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that   the version runs on.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceSpec {


    /// 
    /// The instance type that the image version runs on.
    /// 
    /// Note        JupyterServer apps only support the system value.For KernelGateway apps, the system       value is translated to ml.t3.medium. KernelGateway apps also support all other values for available       instance types.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ml.c5.12xlarge | ml.c5.18xlarge | ml.c5.24xlarge | ml.c5.2xlarge | ml.c5.4xlarge | ml.c5.9xlarge | ml.c5.large | ml.c5.xlarge | ml.g4dn.12xlarge | ml.g4dn.16xlarge | ml.g4dn.2xlarge | ml.g4dn.4xlarge | ml.g4dn.8xlarge | ml.g4dn.xlarge | ml.g5.12xlarge | ml.g5.16xlarge | ml.g5.24xlarge | ml.g5.2xlarge | ml.g5.48xlarge | ml.g5.4xlarge | ml.g5.8xlarge | ml.g5.xlarge | ml.geospatial.interactive | ml.m5.12xlarge | ml.m5.16xlarge | ml.m5.24xlarge | ml.m5.2xlarge | ml.m5.4xlarge | ml.m5.8xlarge | ml.m5.large | ml.m5.xlarge | ml.m5d.12xlarge | ml.m5d.16xlarge | ml.m5d.24xlarge | ml.m5d.2xlarge | ml.m5d.4xlarge | ml.m5d.8xlarge | ml.m5d.large | ml.m5d.xlarge | ml.p3.16xlarge | ml.p3.2xlarge | ml.p3.8xlarge | ml.p3dn.24xlarge | ml.p4d.24xlarge | ml.p4de.24xlarge | ml.r5.12xlarge | ml.r5.16xlarge | ml.r5.24xlarge | ml.r5.2xlarge | ml.r5.4xlarge | ml.r5.8xlarge | ml.r5.large | ml.r5.xlarge | ml.t3.2xlarge | ml.t3.large | ml.t3.medium | ml.t3.micro | ml.t3.small | ml.t3.xlarge | system
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<ResourceSpecInstanceTypeEnum>,


    /// 
    /// The ARN of the SageMaker image that the image version belongs to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws(-[\w]+)*:sagemaker:.+:[0-9]{12}:image/[a-z0-9]([-.]?[a-z0-9])*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SageMakerImageArn")]
    pub sage_maker_image_arn: Option<String>,


    /// 
    /// The ARN of the image version created on the instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws(-[\w]+)*:sagemaker:.+:[0-9]{12}:image-version/[a-z0-9]([-.]?[a-z0-9])*/[0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SageMakerImageVersionArn")]
    pub sage_maker_image_version_arn: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ResourceSpecInstanceTypeEnum {

    /// ml.c5.12xlarge
    #[serde(rename = "ml.c5.12xlarge")]
    Mlc512xlarge,

    /// ml.c5.18xlarge
    #[serde(rename = "ml.c5.18xlarge")]
    Mlc518xlarge,

    /// ml.c5.24xlarge
    #[serde(rename = "ml.c5.24xlarge")]
    Mlc524xlarge,

    /// ml.c5.2xlarge
    #[serde(rename = "ml.c5.2xlarge")]
    Mlc52xlarge,

    /// ml.c5.4xlarge
    #[serde(rename = "ml.c5.4xlarge")]
    Mlc54xlarge,

    /// ml.c5.9xlarge
    #[serde(rename = "ml.c5.9xlarge")]
    Mlc59xlarge,

    /// ml.c5.large
    #[serde(rename = "ml.c5.large")]
    Mlc5large,

    /// ml.c5.xlarge
    #[serde(rename = "ml.c5.xlarge")]
    Mlc5xlarge,

    /// ml.g4dn.12xlarge
    #[serde(rename = "ml.g4dn.12xlarge")]
    Mlg4dn12xlarge,

    /// ml.g4dn.16xlarge
    #[serde(rename = "ml.g4dn.16xlarge")]
    Mlg4dn16xlarge,

    /// ml.g4dn.2xlarge
    #[serde(rename = "ml.g4dn.2xlarge")]
    Mlg4dn2xlarge,

    /// ml.g4dn.4xlarge
    #[serde(rename = "ml.g4dn.4xlarge")]
    Mlg4dn4xlarge,

    /// ml.g4dn.8xlarge
    #[serde(rename = "ml.g4dn.8xlarge")]
    Mlg4dn8xlarge,

    /// ml.g4dn.xlarge
    #[serde(rename = "ml.g4dn.xlarge")]
    Mlg4dnxlarge,

    /// ml.g5.12xlarge
    #[serde(rename = "ml.g5.12xlarge")]
    Mlg512xlarge,

    /// ml.g5.16xlarge
    #[serde(rename = "ml.g5.16xlarge")]
    Mlg516xlarge,

    /// ml.g5.24xlarge
    #[serde(rename = "ml.g5.24xlarge")]
    Mlg524xlarge,

    /// ml.g5.2xlarge
    #[serde(rename = "ml.g5.2xlarge")]
    Mlg52xlarge,

    /// ml.g5.48xlarge
    #[serde(rename = "ml.g5.48xlarge")]
    Mlg548xlarge,

    /// ml.g5.4xlarge
    #[serde(rename = "ml.g5.4xlarge")]
    Mlg54xlarge,

    /// ml.g5.8xlarge
    #[serde(rename = "ml.g5.8xlarge")]
    Mlg58xlarge,

    /// ml.g5.xlarge
    #[serde(rename = "ml.g5.xlarge")]
    Mlg5xlarge,

    /// ml.geospatial.interactive
    #[serde(rename = "ml.geospatial.interactive")]
    Mlgeospatialinteractive,

    /// ml.m5.12xlarge
    #[serde(rename = "ml.m5.12xlarge")]
    Mlm512xlarge,

    /// ml.m5.16xlarge
    #[serde(rename = "ml.m5.16xlarge")]
    Mlm516xlarge,

    /// ml.m5.24xlarge
    #[serde(rename = "ml.m5.24xlarge")]
    Mlm524xlarge,

    /// ml.m5.2xlarge
    #[serde(rename = "ml.m5.2xlarge")]
    Mlm52xlarge,

    /// ml.m5.4xlarge
    #[serde(rename = "ml.m5.4xlarge")]
    Mlm54xlarge,

    /// ml.m5.8xlarge
    #[serde(rename = "ml.m5.8xlarge")]
    Mlm58xlarge,

    /// ml.m5.large
    #[serde(rename = "ml.m5.large")]
    Mlm5large,

    /// ml.m5.xlarge
    #[serde(rename = "ml.m5.xlarge")]
    Mlm5xlarge,

    /// ml.m5d.12xlarge
    #[serde(rename = "ml.m5d.12xlarge")]
    Mlm5d12xlarge,

    /// ml.m5d.16xlarge
    #[serde(rename = "ml.m5d.16xlarge")]
    Mlm5d16xlarge,

    /// ml.m5d.24xlarge
    #[serde(rename = "ml.m5d.24xlarge")]
    Mlm5d24xlarge,

    /// ml.m5d.2xlarge
    #[serde(rename = "ml.m5d.2xlarge")]
    Mlm5d2xlarge,

    /// ml.m5d.4xlarge
    #[serde(rename = "ml.m5d.4xlarge")]
    Mlm5d4xlarge,

    /// ml.m5d.8xlarge
    #[serde(rename = "ml.m5d.8xlarge")]
    Mlm5d8xlarge,

    /// ml.m5d.large
    #[serde(rename = "ml.m5d.large")]
    Mlm5dlarge,

    /// ml.m5d.xlarge
    #[serde(rename = "ml.m5d.xlarge")]
    Mlm5dxlarge,

    /// ml.p3.16xlarge
    #[serde(rename = "ml.p3.16xlarge")]
    Mlp316xlarge,

    /// ml.p3.2xlarge
    #[serde(rename = "ml.p3.2xlarge")]
    Mlp32xlarge,

    /// ml.p3.8xlarge
    #[serde(rename = "ml.p3.8xlarge")]
    Mlp38xlarge,

    /// ml.p3dn.24xlarge
    #[serde(rename = "ml.p3dn.24xlarge")]
    Mlp3dn24xlarge,

    /// ml.p4d.24xlarge
    #[serde(rename = "ml.p4d.24xlarge")]
    Mlp4d24xlarge,

    /// ml.p4de.24xlarge
    #[serde(rename = "ml.p4de.24xlarge")]
    Mlp4de24xlarge,

    /// ml.r5.12xlarge
    #[serde(rename = "ml.r5.12xlarge")]
    Mlr512xlarge,

    /// ml.r5.16xlarge
    #[serde(rename = "ml.r5.16xlarge")]
    Mlr516xlarge,

    /// ml.r5.24xlarge
    #[serde(rename = "ml.r5.24xlarge")]
    Mlr524xlarge,

    /// ml.r5.2xlarge
    #[serde(rename = "ml.r5.2xlarge")]
    Mlr52xlarge,

    /// ml.r5.4xlarge
    #[serde(rename = "ml.r5.4xlarge")]
    Mlr54xlarge,

    /// ml.r5.8xlarge
    #[serde(rename = "ml.r5.8xlarge")]
    Mlr58xlarge,

    /// ml.r5.large
    #[serde(rename = "ml.r5.large")]
    Mlr5large,

    /// ml.r5.xlarge
    #[serde(rename = "ml.r5.xlarge")]
    Mlr5xlarge,

    /// ml.t3.2xlarge
    #[serde(rename = "ml.t3.2xlarge")]
    Mlt32xlarge,

    /// ml.t3.large
    #[serde(rename = "ml.t3.large")]
    Mlt3large,

    /// ml.t3.medium
    #[serde(rename = "ml.t3.medium")]
    Mlt3medium,

    /// ml.t3.micro
    #[serde(rename = "ml.t3.micro")]
    Mlt3micro,

    /// ml.t3.small
    #[serde(rename = "ml.t3.small")]
    Mlt3small,

    /// ml.t3.xlarge
    #[serde(rename = "ml.t3.xlarge")]
    Mlt3xlarge,

    /// system
    #[serde(rename = "system")]
    System,

}

impl Default for ResourceSpecInstanceTypeEnum {
    fn default() -> Self {
        ResourceSpecInstanceTypeEnum::Mlc512xlarge
    }
}


impl cfn_resources::CfnResource for ResourceSpec {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.sage_maker_image_arn {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'sage_maker_image_arn'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.sage_maker_image_version_arn {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'sage_maker_image_version_arn'. {} is greater than 256", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// A collection of space settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SpaceSettings {


    /// 
    /// The JupyterServer app settings.
    /// 
    /// Required: No
    ///
    /// Type: JupyterServerAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "JupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,


    /// 
    /// The KernelGateway app settings.
    /// 
    /// Required: No
    ///
    /// Type: KernelGatewayAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,

}



impl cfn_resources::CfnResource for SpaceSettings {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.jupyter_server_app_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.kernel_gateway_app_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

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