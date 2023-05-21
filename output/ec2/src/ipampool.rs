

/// In IPAM, a pool is a collection of contiguous IP addresses CIDRs. Pools enable you to organize your IP addresses according to your routing and security needs. For example, if you have separate routing and security needs for development and production applications, you can create a pool for each.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIPAMPool {


    /// 
    /// The address family of the pool.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ipv4 | ipv6
    ///
    /// Update requires: Replacement
    #[serde(rename = "AddressFamily")]
    pub address_family: IPAMPoolAddressFamilyEnum,


    /// 
    /// The default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is 10.0.0.0/8 and      you enter 16 here, new allocations will default to 10.0.0.0/16.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocationDefaultNetmaskLength")]
    pub allocation_default_netmask_length: Option<i64>,


    /// 
    /// The maximum netmask length possible for CIDR allocations in this IPAM pool to be compliant. The maximum netmask length must be greater than the minimum netmask length. Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocationMaxNetmaskLength")]
    pub allocation_max_netmask_length: Option<i64>,


    /// 
    /// The minimum netmask length required for CIDR allocations in this IPAM pool to be compliant. The minimum netmask length must be less than the maximum netmask length. Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocationMinNetmaskLength")]
    pub allocation_min_netmask_length: Option<i64>,


    /// 
    /// Tags that are required for resources that use CIDRs from this IPAM pool. Resources that do not have these tags will not be allowed to allocate space from the pool. If the resources have their tags changed after they have allocated space or if the allocation tagging requirements are changed on the pool, the resource may be marked as noncompliant.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocationResourceTags")]
    pub allocation_resource_tags: Option<Vec<Tag>>,


    /// 
    /// If selected, IPAM will continuously look for resources within the CIDR range of this pool      and automatically import them as allocations into your IPAM. The CIDRs that will be allocated for     these resources must not already be allocated to other resources in order for the import to succeed. IPAM will import      a CIDR regardless of its compliance with the pool's allocation rules, so a resource might be imported and subsequently      marked as noncompliant. If IPAM discovers multiple CIDRs that overlap, IPAM will import the largest CIDR only. If IPAM      discovers multiple CIDRs with matching CIDRs, IPAM will randomly import one of them only.
    /// 
    /// A locale must be set on the pool for this feature to work.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoImport")]
    pub auto_import: Option<bool>,


    /// 
    /// Limits which service in AWS that the pool can be used in. "ec2", for example, allows users to use space for Elastic IP addresses and VPCs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ec2
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsService")]
    pub aws_service: Option<IPAMPoolAwsServiceEnum>,


    /// 
    /// The description of the IPAM pool.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The ID of the scope in which you would like to create the IPAM pool.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpamScopeId")]
    pub ipam_scope_id: String,


    /// 
    /// The locale of the IPAM pool. In IPAM, the locale is the AWS Region where you want to make an IPAM pool available for allocations. Only resources in the same Region as the locale of the pool can get IP address allocations from the pool. You can only allocate a CIDR for a VPC, for example, from an IPAM pool that shares a locale with the VPC’s Region. Note that once you choose a Locale for a pool, you cannot modify it. If you choose an AWS Region for locale that has not been configured as an operating Region for the IPAM, you'll get an error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Locale")]
    pub locale: Option<String>,


    /// 
    /// Information about the CIDRs provisioned to an IPAM pool.
    /// 
    /// Required: No
    ///
    /// Type: List of ProvisionedCidr
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedCidrs")]
    pub provisioned_cidrs: Option<Vec<ProvisionedCidr>>,


    /// 
    /// The IP address source for pools in the public scope. Only used for provisioning IP address CIDRs to pools in the public scope. Default is BYOIP. For more information, see Create IPv6 pools in the Amazon VPC IPAM User Guide.      By default, you can add only one Amazon-provided IPv6 CIDR block to a top-level IPv6 pool. For information on increasing the default limit, see Quotas for your IPAM in the Amazon VPC IPAM User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: amazon | byoip
    ///
    /// Update requires: Replacement
    #[serde(rename = "PublicIpSource")]
    pub public_ip_source: Option<IPAMPoolPublicIpSourceEnum>,


    /// 
    /// Determines if a pool is publicly advertisable. This option is not available for pools with AddressFamily set to ipv4.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PubliclyAdvertisable")]
    pub publicly_advertisable: Option<bool>,


    /// 
    /// The ID of the source IPAM pool. You can use this option to create an IPAM pool within an existing source pool.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceIpamPoolId")]
    pub source_ipam_pool_id: Option<String>,


    /// 
    /// The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value.   For example, to find all resources that have a tag with the key Owner and the value TeamA, specify tag:Owner for the filter name and TeamA for the filter value.
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
pub enum IPAMPoolAddressFamilyEnum {

    /// ipv4
    #[serde(rename = "ipv4")]
    Ipv4,

    /// ipv6
    #[serde(rename = "ipv6")]
    Ipv6,

}

impl Default for IPAMPoolAddressFamilyEnum {
    fn default() -> Self {
        IPAMPoolAddressFamilyEnum::Ipv4
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum IPAMPoolAwsServiceEnum {

    /// ec2
    #[serde(rename = "ec2")]
    Ec2,

}

impl Default for IPAMPoolAwsServiceEnum {
    fn default() -> Self {
        IPAMPoolAwsServiceEnum::Ec2
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum IPAMPoolPublicIpSourceEnum {

    /// amazon
    #[serde(rename = "amazon")]
    Amazon,

    /// byoip
    #[serde(rename = "byoip")]
    Byoip,

}

impl Default for IPAMPoolPublicIpSourceEnum {
    fn default() -> Self {
        IPAMPoolPublicIpSourceEnum::Amazon
    }
}


impl cfn_resources::CfnResource for CfnIPAMPool {
    fn type_string() -> &'static str {
        "AWS::EC2::IPAMPool"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.allocation_default_netmask_length {

        if *the_val > 128 as _ {
            return Err(format!("Max validation failed on field 'allocation_default_netmask_length'. {} is greater than 128", the_val));
        }

        }
        
        if let Some(the_val) = &self.allocation_default_netmask_length {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'allocation_default_netmask_length'. {} is less than 0", the_val));
        }

        }
        
        if let Some(the_val) = &self.allocation_max_netmask_length {

        if *the_val > 128 as _ {
            return Err(format!("Max validation failed on field 'allocation_max_netmask_length'. {} is greater than 128", the_val));
        }

        }
        
        if let Some(the_val) = &self.allocation_max_netmask_length {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'allocation_max_netmask_length'. {} is less than 0", the_val));
        }

        }
        
        if let Some(the_val) = &self.allocation_min_netmask_length {

        if *the_val > 128 as _ {
            return Err(format!("Max validation failed on field 'allocation_min_netmask_length'. {} is greater than 128", the_val));
        }

        }
        
        if let Some(the_val) = &self.allocation_min_netmask_length {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'allocation_min_netmask_length'. {} is less than 0", the_val));
        }

        }
        
        Ok(())
    }
}

/// The CIDR provisioned to the IPAM pool. A CIDR is a representation of an IP address and its associated network mask (or netmask)      and refers to a range of IP addresses. An IPv4 CIDR example is 10.24.34.0/23. An IPv6 CIDR example is 2001:DB8::/32.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProvisionedCidr {


    /// 
    /// The CIDR provisioned to the IPAM pool. A CIDR is a representation of an IP address and its associated network mask (or netmask)      and refers to a range of IP addresses. An IPv4 CIDR example is 10.24.34.0/23. An IPv6 CIDR example is 2001:DB8::/32.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidr")]
    pub cidr: String,

}



impl cfn_resources::CfnResource for ProvisionedCidr {
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