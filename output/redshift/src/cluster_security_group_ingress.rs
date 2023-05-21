

/// Adds an inbound (ingress) rule to an Amazon Redshift security group. Depending on whether       the application accessing your cluster is running on the Internet or an Amazon EC2       instance, you can authorize inbound access to either a Classless Interdomain Routing       (CIDR)/Internet Protocol (IP) range or to an Amazon EC2 security group. You can add as       many as 20 ingress rules to an Amazon Redshift security group.
///
/// If you authorize access to an Amazon EC2 security group, specify         EC2SecurityGroupName and         EC2SecurityGroupOwnerId. The Amazon EC2 security group and       Amazon Redshift cluster must be in the same AWS Region.
///
/// If you authorize access to a CIDR/IP address range, specify         CIDRIP. For an overview of CIDR blocks, see the Wikipedia       article on Classless Inter-Domain Routing.
///
/// You must also associate the security group with a cluster so that clients running       on these IP addresses or the EC2 instance are authorized to connect to the cluster. For       information about managing security groups, go to Working with Security         Groups in the Amazon Redshift Cluster Management Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub cidrip: Option<String>,


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
    pub cluster_security_group_name: String,


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
    pub ec2_security_group_owner_id: Option<String>,


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
    pub ec2_security_group_name: Option<String>,

}



impl cfn_resources::CfnResource for CfnClusterSecurityGroupIngress {
    fn type_string() -> &'static str {
        "AWS::Redshift::ClusterSecurityGroupIngress"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
