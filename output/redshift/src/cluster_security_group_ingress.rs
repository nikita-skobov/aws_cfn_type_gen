/// Adds an inbound (ingress) rule to an Amazon Redshift security group. Depending on whether       the application accessing your cluster is running on the Internet or an Amazon EC2       instance, you can authorize inbound access to either a Classless Interdomain Routing       (CIDR)/Internet Protocol (IP) range or to an Amazon EC2 security group. You can add as       many as 20 ingress rules to an Amazon Redshift security group.
///
/// If you authorize access to an Amazon EC2 security group, specify         EC2SecurityGroupName and         EC2SecurityGroupOwnerId. The Amazon EC2 security group and       Amazon Redshift cluster must be in the same AWS Region.
///
/// If you authorize access to a CIDR/IP address range, specify         CIDRIP. For an overview of CIDR blocks, see the Wikipedia       article on Classless Inter-Domain Routing.
///
/// You must also associate the security group with a cluster so that clients running       on these IP addresses or the EC2 instance are authorized to connect to the cluster. For       information about managing security groups, go to Working with Security         Groups in the Amazon Redshift Cluster Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnClusterSecurityGroupIngress {
    ///
    /// The IP range to be added the Amazon Redshift security group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "CIDRIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrip: Option<cfn_resources::StrVal>,

    ///
    /// The name of the security group to which the ingress rule is added.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterSecurityGroupName")]
    pub cluster_security_group_name: cfn_resources::StrVal,

    ///
    /// The EC2 security group to be added the Amazon Redshift security group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The AWS account number of the owner of the security group specified by       the EC2SecurityGroupName parameter. The AWS Access       Key ID is not an acceptable value.
    ///
    /// Example: 111122223333
    ///
    /// Conditional. If you specify the EC2SecurityGroupName property, you must       specify this property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_owner_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnClusterSecurityGroupIngress {
    fn type_string(&self) -> &'static str {
        "AWS::Redshift::ClusterSecurityGroupIngress"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.cidrip {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!(
                        "Max validation failed on field 'cidrip'. {} is greater than 2147483647",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.cluster_security_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'cluster_security_group_name'. {} is greater than 2147483647", s.len()));
            }
        }

        if let Some(the_val) = &self.ec2_security_group_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!("Max validation failed on field 'ec2_security_group_name'. {} is greater than 2147483647", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.ec2_security_group_owner_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!("Max validation failed on field 'ec2_security_group_owner_id'. {} is greater than 2147483647", s.len()));
                }
            }
        }

        Ok(())
    }
}
