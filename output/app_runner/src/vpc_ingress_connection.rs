/// The AWS::AppRunner::VpcIngressConnection resource is an AWS App Runner resource type that specifies an App Runner VPC    Ingress Connection.
///
/// App Runner requires this resource when you want to associate your App Runner service to an Amazon VPC endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVpcIngressConnection {
    ///
    /// Specifications for the customer’s Amazon VPC and the related AWS PrivateLink VPC endpoint that are used to create the VPC Ingress Connection    resource.
    ///
    /// Required: Yes
    ///
    /// Type: IngressVpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngressVpcConfiguration")]
    pub ingress_vpc_configuration: IngressVpcConfiguration,

    ///
    /// The Amazon Resource Name (ARN) for this App Runner service that is used to create the VPC Ingress Connection resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Pattern: arn:aws(-[\w]+)*:[a-z0-9-\\.]{0,63}:[a-z0-9-\\.]{0,63}:[0-9]{12}:(\w|\/|-){1,1011}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceArn")]
    pub service_arn: cfn_resources::StrVal,

    ///
    /// An optional list of metadata items that you can associate with the VPC Ingress Connection resource. A tag is a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The customer-provided VPC Ingress Connection name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 40
    ///
    /// Pattern: [A-Za-z0-9][A-Za-z0-9\-_]{3,39}
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcIngressConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnVpcIngressConnection {
    fn type_string(&self) -> &'static str {
        "AWS::AppRunner::VpcIngressConnection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ingress_vpc_configuration.validate()?;

        let the_val = &self.service_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1011 as _ {
                return Err(format!(
                    "Max validation failed on field 'service_arn'. {} is greater than 1011",
                    s.len()
                ));
            }
        }

        let the_val = &self.service_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'service_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.vpc_ingress_connection_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 40 as _ {
                    return Err(format!("Max validation failed on field 'vpc_ingress_connection_name'. {} is greater than 40", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.vpc_ingress_connection_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 4 as _ {
                    return Err(format!("Min validation failed on field 'vpc_ingress_connection_name'. {} is less than 4", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Specifications for the customer’s VPC and related PrivateLink VPC endpoint that are used to associate with the VPC Ingress Connection resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IngressVpcConfiguration {
    ///
    /// The ID of the VPC endpoint that your App Runner service connects to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 51200
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: cfn_resources::StrVal,

    ///
    /// The ID of the VPC that is used for the VPC endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 51200
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for IngressVpcConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.vpc_endpoint_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 51200 as _ {
                return Err(format!(
                    "Max validation failed on field 'vpc_endpoint_id'. {} is greater than 51200",
                    s.len()
                ));
            }
        }

        let the_val = &self.vpc_endpoint_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'vpc_endpoint_id'. {} is less than 0",
                    s.len()
                ));
            }
        }

        let the_val = &self.vpc_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 51200 as _ {
                return Err(format!(
                    "Max validation failed on field 'vpc_id'. {} is greater than 51200",
                    s.len()
                ));
            }
        }

        let the_val = &self.vpc_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'vpc_id'. {} is less than 0",
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
