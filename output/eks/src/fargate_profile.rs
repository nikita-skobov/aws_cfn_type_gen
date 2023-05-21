/// Creates an AWS Fargate profile for your Amazon EKS cluster. You       must have at least one Fargate profile in a cluster to be able to run       pods on Fargate.
///
/// The Fargate profile allows an administrator to declare which pods run       on Fargate and specify which pods run on which Fargate       profile. This declaration is done through the profile’s selectors. Each profile can have       up to five selectors that contain a namespace and labels. A namespace is required for       every selector. The label field consists of multiple optional key-value pairs. Pods that       match the selectors are scheduled on Fargate. If a to-be-scheduled pod       matches any of the selectors in the Fargate profile, then that pod is run       on Fargate.
///
/// When you create a Fargate profile, you must specify a pod execution       role to use with the pods that are scheduled with the profile. This role is added to the       cluster's Kubernetes Role Based Access Control (RBAC) for authorization so that the         kubelet that is running on the Fargate infrastructure       can register with your Amazon EKS cluster so that it can appear in your cluster       as a node. The pod execution role also provides IAM permissions to the         Fargate infrastructure to allow read access to Amazon ECR       image repositories. For more information, see Pod         Execution Role in the Amazon EKS User Guide.
///
/// Fargate profiles are immutable. However, you can create a new updated       profile to replace an existing profile and then delete the original after the updated       profile has finished creating.
///
/// If any Fargate profiles in a cluster are in the DELETING       status, you must wait for that Fargate profile to finish deleting before       you can create any other profiles in that cluster.
///
/// For more information, see AWS Fargate Profile in the       Amazon EKS User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFargateProfile {
    ///
    /// The name of the Amazon EKS cluster to apply the Fargate profile       to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,

    ///
    /// The name of the Fargate profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FargateProfileName")]
    pub fargate_profile_name: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the pod execution role to use for pods that match the selectors in       the Fargate profile. The pod execution role allows Fargate       infrastructure to register with your cluster as a node, and it provides read access to         Amazon ECR image repositories. For more information, see Pod         Execution Role in the Amazon EKS User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PodExecutionRoleArn")]
    pub pod_execution_role_arn: String,

    ///
    /// The selectors to match for pods to use this Fargate profile. Each       selector must have an associated namespace. Optionally, you can also specify labels for       a namespace. You may specify up to five selectors in a Fargate       profile.
    ///
    /// Required: Yes
    ///
    /// Type: List of Selector
    ///
    /// Update requires: Replacement
    #[serde(rename = "Selectors")]
    pub selectors: Vec<Selector>,

    ///
    /// The IDs of subnets to launch your pods into. At this time, pods running on Fargate are not assigned public IP addresses, so only private subnets (with       no direct route to an Internet Gateway) are accepted for this parameter.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,

    ///
    /// The metadata to apply to the Fargate profile to assist with       categorization and organization. Each tag consists of a key and an optional value. You       define both. Fargate profile tags do not propagate to any other resources       associated with the Fargate profile, such as the pods that are scheduled       with it.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnFargateProfile {
    fn type_string(&self) -> &'static str {
        "AWS::EKS::FargateProfile"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A key-value pair.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Label {
    ///
    /// Enter a key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// Enter a value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Label {
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

/// An object representing an AWS Fargate profile selector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Selector {
    ///
    /// The Kubernetes labels that the selector should match. A pod must contain all of the       labels that are specified in the selector for it to be considered a match.
    ///
    /// Required: No
    ///
    /// Type: List of Label
    ///
    /// Update requires: Replacement
    #[serde(rename = "Labels")]
    pub labels: Option<Vec<Label>>,

    ///
    /// The Kubernetes namespace that the selector should match.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

impl cfn_resources::CfnResource for Selector {
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
