

/// Represents the hypervisor's permissions to which the gateway will connect.
///
/// A hypervisor is hardware, software, or firmware that creates and manages virtual machines,    and allocates resources to them.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnHypervisor {


    /// 
    /// The server host of the hypervisor. This can be either an IP address or a fully-qualified    domain name (FQDN).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: .+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Key Management Service used to encrypt the    hypervisor.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 50
    ///
    /// Maximum: 500
    ///
    /// Pattern: (^arn:(aws|aws-cn|aws-us-gov):kms:([a-zA-Z0-9-]+):([0-9]+):(key|alias)/(\S+)$)|(^alias/(\S+)$)
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the group of gateways within    the requested log.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: $|^arn:(aws|aws-cn|aws-us-gov):logs:([a-zA-Z0-9-]+):([0-9]+):log-group:[a-zA-Z0-9_\-\/\.]+:\*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupArn")]
    pub log_group_arn: Option<String>,


    /// 
    /// The name of the hypervisor.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [a-zA-Z0-9-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The password for the hypervisor.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [ -~]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: Option<String>,


    /// 
    /// The tags of the hypervisor configuration to import.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The username for the hypervisor.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [ -\.0-\[\]-~]*[!-\.0-\[\]-~][ -\.0-\[\]-~]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: Option<String>,

}



impl cfn_resources::CfnResource for CfnHypervisor {
    fn type_string(&self) -> &'static str {
        "AWS::BackupGateway::Hypervisor"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.host {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'host'. {} is greater than 128", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.host {

        if the_val.len() < 3 as _ {
            return Err(format!("Min validation failed on field 'host'. {} is less than 3", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.kms_key_arn {

        if the_val.len() > 500 as _ {
            return Err(format!("Max validation failed on field 'kms_key_arn'. {} is greater than 500", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.kms_key_arn {

        if the_val.len() < 50 as _ {
            return Err(format!("Min validation failed on field 'kms_key_arn'. {} is less than 50", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.log_group_arn {

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'log_group_arn'. {} is greater than 2048", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.log_group_arn {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'log_group_arn'. {} is less than 0", the_val.len()));
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
        
        if let Some(the_val) = &self.password {

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'password'. {} is greater than 100", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.password {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'password'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.username {

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'username'. {} is greater than 100", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.username {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'username'. {} is less than 1", the_val.len()));
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