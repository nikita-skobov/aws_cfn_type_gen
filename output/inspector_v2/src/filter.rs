

/// Details about a filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFilter {


    /// 
    /// A description of the filter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The action that is to be applied to the findings that match the filter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterAction")]
    pub filter_action: String,


    /// 
    /// Details on the filter criteria associated with this filter.
    /// 
    /// Required: Yes
    ///
    /// Type: FilterCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterCriteria")]
    pub filter_criteria: FilterCriteria,


    /// 
    /// The name of the filter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}



impl cfn_resources::CfnResource for CfnFilter {
    fn type_string() -> &'static str {
        "AWS::InspectorV2::Filter"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains details on the time range used to filter findings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DateFilter {


    /// 
    /// A timestamp representing the end of the time period filtered on.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndInclusive")]
    pub end_inclusive: Option<i64>,


    /// 
    /// A timestamp representing the start of the time period filtered on.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartInclusive")]
    pub start_inclusive: Option<i64>,

}




/// Details on the criteria used to define the filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterCriteria {


    /// 
    /// Details of the AWS account IDs used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<Vec<StringFilter>>,


    /// 
    /// Details of the component IDs used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentId")]
    pub component_id: Option<Vec<StringFilter>>,


    /// 
    /// Details of the component types used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentType")]
    pub component_type: Option<Vec<StringFilter>>,


    /// 
    /// Details of the Amazon EC2 instance image IDs used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ec2InstanceImageId")]
    pub ec2_instance_image_id: Option<Vec<StringFilter>>,


    /// 
    /// Details of the Amazon EC2 instance subnet IDs used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ec2InstanceSubnetId")]
    pub ec2_instance_subnet_id: Option<Vec<StringFilter>>,


    /// 
    /// Details of the Amazon EC2 instance VPC IDs used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ec2InstanceVpcId")]
    pub ec2_instance_vpc_id: Option<Vec<StringFilter>>,


    /// 
    /// Details of the Amazon ECR image architecture types used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcrImageArchitecture")]
    pub ecr_image_architecture: Option<Vec<StringFilter>>,


    /// 
    /// Details of the Amazon ECR image hashes used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcrImageHash")]
    pub ecr_image_hash: Option<Vec<StringFilter>>,


    /// 
    /// Details on the Amazon ECR image push date and time used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of DateFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcrImagePushedAt")]
    pub ecr_image_pushed_at: Option<Vec<DateFilter>>,


    /// 
    /// Details on the Amazon ECR registry used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcrImageRegistry")]
    pub ecr_image_registry: Option<Vec<StringFilter>>,


    /// 
    /// Details on the name of the Amazon ECR repository used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcrImageRepositoryName")]
    pub ecr_image_repository_name: Option<Vec<StringFilter>>,


    /// 
    /// The tags attached to the Amazon ECR container image.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcrImageTags")]
    pub ecr_image_tags: Option<Vec<StringFilter>>,


    /// 
    /// Details on the finding ARNs used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "FindingArn")]
    pub finding_arn: Option<Vec<StringFilter>>,


    /// 
    /// Details on the finding status types used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "FindingStatus")]
    pub finding_status: Option<Vec<StringFilter>>,


    /// 
    /// Details on the finding types used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "FindingType")]
    pub finding_type: Option<Vec<StringFilter>>,


    /// 
    /// Details on the date and time a finding was first seen used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of DateFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirstObservedAt")]
    pub first_observed_at: Option<Vec<DateFilter>>,


    /// 
    /// The Amazon Inspector score to filter on.
    /// 
    /// Required: No
    ///
    /// Type: List of NumberFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "InspectorScore")]
    pub inspector_score: Option<Vec<NumberFilter>>,


    /// 
    /// Details on the date and time a finding was last seen used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of DateFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastObservedAt")]
    pub last_observed_at: Option<Vec<DateFilter>>,


    /// 
    /// Details on the ingress source addresses used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkProtocol")]
    pub network_protocol: Option<Vec<StringFilter>>,


    /// 
    /// Details on the port ranges used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of PortRangeFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRange")]
    pub port_range: Option<Vec<PortRangeFilter>>,


    /// 
    /// Details on the related vulnerabilities used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelatedVulnerabilities")]
    pub related_vulnerabilities: Option<Vec<StringFilter>>,


    /// 
    /// Details on the resource IDs used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<Vec<StringFilter>>,


    /// 
    /// Details on the resource tags used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of MapFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTags")]
    pub resource_tags: Option<Vec<MapFilter>>,


    /// 
    /// Details on the resource types used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<Vec<StringFilter>>,


    /// 
    /// Details on the severity used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Severity")]
    pub severity: Option<Vec<StringFilter>>,


    /// 
    /// Details on the finding title used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Title")]
    pub title: Option<Vec<StringFilter>>,


    /// 
    /// Details on the date and time a finding was last updated at used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of DateFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<Vec<DateFilter>>,


    /// 
    /// Details on the vendor severity used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "VendorSeverity")]
    pub vendor_severity: Option<Vec<StringFilter>>,


    /// 
    /// Details on the vulnerability ID used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "VulnerabilityId")]
    pub vulnerability_id: Option<Vec<StringFilter>>,


    /// Details on the vulnerability score to filter findings by.
    ///
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "VulnerabilitySource")]
    pub vulnerability_source: Option<Vec<StringFilter>>,


    /// 
    /// Details on the vulnerable packages used to filter findings.
    /// 
    /// Required: No
    ///
    /// Type: List of PackageFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "VulnerablePackages")]
    pub vulnerable_packages: Option<Vec<PackageFilter>>,

}




/// An object that describes details of a map filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MapFilter {


    /// 
    /// The operator to use when comparing values in the filter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comparison")]
    pub comparison: String,


    /// 
    /// The tag key used in the filter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The tag value used in the filter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}




/// An object that describes the details of a number filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NumberFilter {


    /// 
    /// The lowest number to be included in the filter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LowerInclusive")]
    pub lower_inclusive: Option<f64>,


    /// 
    /// The highest number to be included in the filter.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpperInclusive")]
    pub upper_inclusive: Option<f64>,

}




/// Contains information on the details of a package filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PackageFilter {


    /// 
    /// An object that contains details on the package architecture type to filter on.
    /// 
    /// Required: No
    ///
    /// Type: StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Architecture")]
    pub architecture: Option<StringFilter>,


    /// 
    /// An object that contains details on the package epoch to filter on.
    /// 
    /// Required: No
    ///
    /// Type: NumberFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Epoch")]
    pub epoch: Option<NumberFilter>,


    /// 
    /// An object that contains details on the name of the package to filter on.
    /// 
    /// Required: No
    ///
    /// Type: StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<StringFilter>,


    /// 
    /// An object that contains details on the package release to filter on.
    /// 
    /// Required: No
    ///
    /// Type: StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Release")]
    pub release: Option<StringFilter>,


    /// 
    /// An object that contains details on the source layer hash to filter on.
    /// 
    /// Required: No
    ///
    /// Type: StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceLayerHash")]
    pub source_layer_hash: Option<StringFilter>,


    /// 
    /// The package version to filter on.
    /// 
    /// Required: No
    ///
    /// Type: StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<StringFilter>,

}




/// An object that describes the details of a port range filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PortRangeFilter {


    /// The port number the port range begins at.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BeginInclusive")]
    pub begin_inclusive: Option<i64>,


    /// 
    /// The port number the port range ends at.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndInclusive")]
    pub end_inclusive: Option<i64>,

}




/// An object that describes the details of a string filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StringFilter {


    /// 
    /// The operator to use when comparing values in the filter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comparison")]
    pub comparison: String,


    /// 
    /// The value to filter on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


