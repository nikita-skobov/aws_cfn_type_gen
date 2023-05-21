/// The AWS::EMRContainers::VirtualCluster resource specifies a virtual cluster. A virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list, and delete virtual clusters. They do not consume any additional resources in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVirtualCluster {
    ///
    /// The container provider of the virtual cluster.
    ///
    /// Required: Yes
    ///
    /// Type: ContainerProvider
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerProvider")]
    pub container_provider: ContainerProvider,

    ///
    /// The name of the virtual cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [\.\-_/#A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnVirtualCluster {
    fn type_string(&self) -> &'static str {
        "AWS::EMRContainers::VirtualCluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.container_provider.validate()?;

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The information about the container used for a job run or a managed endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContainerInfo {
    ///
    /// The information about the Amazon EKS cluster.
    ///
    /// Required: Yes
    ///
    /// Type: EksInfo
    ///
    /// Update requires: Replacement
    #[serde(rename = "EksInfo")]
    pub eks_info: EksInfo,
}

impl cfn_resources::CfnResource for ContainerInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.eks_info.validate()?;

        Ok(())
    }
}

/// The information about the container provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContainerProvider {
    ///
    /// The ID of the container cluster.
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[0-9A-Za-z][A-Za-z0-9\-_]*
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The information about the container cluster.
    ///
    /// Required: Yes
    ///
    /// Type: ContainerInfo
    ///
    /// Update requires: Replacement
    #[serde(rename = "Info")]
    pub info: ContainerInfo,

    ///
    /// The type of the container provider. Amazon EKS is the only supported type as of now.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EKS
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: ContainerProviderTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ContainerProviderTypeEnum {
    /// EKS
    #[serde(rename = "EKS")]
    Eks,
}

impl Default for ContainerProviderTypeEnum {
    fn default() -> Self {
        ContainerProviderTypeEnum::Eks
    }
}

impl cfn_resources::CfnResource for ContainerProvider {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'id'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.info.validate()?;

        Ok(())
    }
}

/// The information about the Amazon EKS cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksInfo {
    ///
    /// The namespaces of the EKS cluster.
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: [a-z0-9]([-a-z0-9]*[a-z0-9])?
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Namespace")]
    pub namespace: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EksInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.namespace;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'namespace'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.namespace;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'namespace'. {} is less than 1",
                    s.len()
                ));
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
