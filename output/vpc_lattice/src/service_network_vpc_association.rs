/// Associates a VPC with a service network. When you associate a VPC with the service network,  it enables all the resources within that VPC to be clients and communicate with other services in  the service network. For more information, see Manage VPC associations in the Amazon VPC Lattice User Guide.
///
/// You can't use this operation if there is a disassociation in progress. If the association  fails, retry by deleting the association and recreating it.
///
/// As a result of this operation, the association gets created in the service network account  and the VPC owner account.
///
/// If you add a security group to the service network and VPC association, the association must  continue to always have at least one security group. You can add or edit security groups at any  time. However, to remove all security groups, you must first delete the association and recreate  it without security groups.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnServiceNetworkVpcAssociation {
    ///
    /// The IDs of the security groups. Security groups aren't added by default. You can add a  security group to apply network level controls to control which resources in a VPC are allowed to  access the service network and its services. For more information, see Control traffic to   resources using security groups in the Amazon VPC User  Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// The ID or Amazon Resource Name (ARN) of the service network. You must use the ARN when the  resources specified in the operation are in different accounts.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceNetworkIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The tags for the association.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the VPC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_identifier: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnServiceNetworkVpcAssociationarn,

    #[serde(skip_serializing)]
    pub att_created_at: CfnServiceNetworkVpcAssociationcreatedat,

    #[serde(skip_serializing)]
    pub att_id: CfnServiceNetworkVpcAssociationid,

    #[serde(skip_serializing)]
    pub att_service_network_arn: CfnServiceNetworkVpcAssociationservicenetworkarn,

    #[serde(skip_serializing)]
    pub att_service_network_id: CfnServiceNetworkVpcAssociationservicenetworkid,

    #[serde(skip_serializing)]
    pub att_service_network_name: CfnServiceNetworkVpcAssociationservicenetworkname,

    #[serde(skip_serializing)]
    pub att_status: CfnServiceNetworkVpcAssociationstatus,

    #[serde(skip_serializing)]
    pub att_vpc_id: CfnServiceNetworkVpcAssociationvpcid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkVpcAssociationarn;
impl CfnServiceNetworkVpcAssociationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkVpcAssociationcreatedat;
impl CfnServiceNetworkVpcAssociationcreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkVpcAssociationid;
impl CfnServiceNetworkVpcAssociationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkVpcAssociationservicenetworkarn;
impl CfnServiceNetworkVpcAssociationservicenetworkarn {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceNetworkArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkVpcAssociationservicenetworkid;
impl CfnServiceNetworkVpcAssociationservicenetworkid {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceNetworkId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkVpcAssociationservicenetworkname;
impl CfnServiceNetworkVpcAssociationservicenetworkname {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceNetworkName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkVpcAssociationstatus;
impl CfnServiceNetworkVpcAssociationstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkVpcAssociationvpcid;
impl CfnServiceNetworkVpcAssociationvpcid {
    pub fn att_name(&self) -> &'static str {
        r#"VpcId"#
    }
}

impl cfn_resources::CfnResource for CfnServiceNetworkVpcAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::VpcLattice::ServiceNetworkVpcAssociation"
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
