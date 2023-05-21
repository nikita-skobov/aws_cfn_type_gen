

/// The AWS::EMR::Studio resource specifies an Amazon EMR Studio. An EMR Studio is a web-based, integrated development environment for fully managed Jupyter notebooks that run on Amazon EMR clusters. For more information, see the Amazon EMR Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStudio {


    /// 
    /// Specifies whether the Studio authenticates users using IAM Identity Center or IAM.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: IAM | SSO
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthMode")]
    pub auth_mode: StudioAuthModeEnum,


    /// 
    /// The Amazon S3 location to back up EMR Studio Workspaces and notebook files.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultS3Location")]
    pub default_s3_location: String,


    /// 
    /// A detailed description of the Amazon EMR Studio.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The ID of the Amazon EMR Studio Engine security group. The Engine security group     allows inbound network traffic from the Workspace security group, and it must be in the     same VPC specified by VpcId.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineSecurityGroupId")]
    pub engine_security_group_id: String,


    /// 
    /// Your identity provider's authentication endpoint. Amazon EMR Studio redirects     federated users to this endpoint for authentication when logging in to a Studio with the     Studio URL.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdpAuthUrl")]
    pub idp_auth_url: Option<String>,


    /// 
    /// The name of your identity provider's RelayState parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdpRelayStateParameterName")]
    pub idp_relay_state_parameter_name: Option<String>,


    /// 
    /// A descriptive name for the Amazon EMR Studio.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that will be assumed by the Amazon EMR Studio. The service role provides a     way for Amazon EMR Studio to interoperate with other AWS services.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceRole")]
    pub service_role: String,


    /// 
    /// A list of subnet IDs to associate with the Amazon EMR Studio. A Studio can have     a maximum of 5 subnets. The subnets must belong to the VPC specified by VpcId.     Studio users can create a Workspace in any of the specified subnets.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


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


    /// 
    /// The Amazon Resource Name (ARN) of the IAM user role that will be assumed by users and groups logged in to a Studio. The     permissions attached to this IAM role can be scoped down for each user or group using     session policies. You only need to specify UserRole when you set AuthMode to SSO.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserRole")]
    pub user_role: Option<String>,


    /// 
    /// The ID of the Amazon Virtual Private Cloud (Amazon VPC) to associate with the     Studio.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,


    /// 
    /// The ID of the Workspace security group associated with the Amazon EMR Studio.     The Workspace security group allows outbound network traffic to resources in the Engine     security group and to the internet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkspaceSecurityGroupId")]
    pub workspace_security_group_id: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum StudioAuthModeEnum {

    /// IAM
    #[serde(rename = "IAM")]
    Iam,

    /// SSO
    #[serde(rename = "SSO")]
    Sso,

}

impl Default for StudioAuthModeEnum {
    fn default() -> Self {
        StudioAuthModeEnum::Iam
    }
}


impl cfn_resources::CfnResource for CfnStudio {
    fn type_string(&self) -> &'static str {
        "AWS::EMR::Studio"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.default_s3_location;

        if the_val.len() > 10280 as _ {
            return Err(format!("Max validation failed on field 'default_s3_location'. {} is greater than 10280", the_val.len()));
        }

        
        let the_val = &self.default_s3_location;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'default_s3_location'. {} is less than 0", the_val.len()));
        }

        
        if let Some(the_val) = &self.description {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 0", the_val.len()));
        }

        }
        
        let the_val = &self.engine_security_group_id;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'engine_security_group_id'. {} is greater than 256", the_val.len()));
        }

        
        let the_val = &self.engine_security_group_id;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'engine_security_group_id'. {} is less than 0", the_val.len()));
        }

        
        if let Some(the_val) = &self.idp_auth_url {

        if the_val.len() > 10280 as _ {
            return Err(format!("Max validation failed on field 'idp_auth_url'. {} is greater than 10280", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.idp_auth_url {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'idp_auth_url'. {} is less than 0", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.idp_relay_state_parameter_name {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'idp_relay_state_parameter_name'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.idp_relay_state_parameter_name {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'idp_relay_state_parameter_name'. {} is less than 0", the_val.len()));
        }

        }
        
        let the_val = &self.name;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 256", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 0", the_val.len()));
        }

        
        let the_val = &self.service_role;

        if the_val.len() > 10280 as _ {
            return Err(format!("Max validation failed on field 'service_role'. {} is greater than 10280", the_val.len()));
        }

        
        let the_val = &self.service_role;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'service_role'. {} is less than 0", the_val.len()));
        }

        
        if let Some(the_val) = &self.user_role {

        if the_val.len() > 10280 as _ {
            return Err(format!("Max validation failed on field 'user_role'. {} is greater than 10280", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.user_role {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'user_role'. {} is less than 0", the_val.len()));
        }

        }
        
        let the_val = &self.vpc_id;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'vpc_id'. {} is greater than 256", the_val.len()));
        }

        
        let the_val = &self.vpc_id;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'vpc_id'. {} is less than 0", the_val.len()));
        }

        
        let the_val = &self.workspace_security_group_id;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'workspace_security_group_id'. {} is greater than 256", the_val.len()));
        }

        
        let the_val = &self.workspace_security_group_id;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'workspace_security_group_id'. {} is less than 0", the_val.len()));
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}