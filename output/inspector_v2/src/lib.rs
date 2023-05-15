
pub mod cfn_filter {

#[derive(serde::Serialize, Default)]
pub struct CfnFilter {
    /// Findings filter name.
    #[serde(rename = "Name")]
    pub name: String,
    /// Findings filter criteria.
    #[serde(rename = "FilterCriteria")]
    pub filter_criteria: FilterCriteria,
    /// Findings filter description.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Findings filter action.
    #[serde(rename = "FilterAction")]
    pub filter_action: FilterAction,

}

pub type Timestamp = usize;pub type MapComparison = String;
#[derive(serde::Serialize, Default)]
pub struct PackageFilter {
    #[serde(rename = "Name")]
    pub name: Option<StringFilter>,
    #[serde(rename = "Release")]
    pub release: Option<StringFilter>,
    #[serde(rename = "Architecture")]
    pub architecture: Option<StringFilter>,
    #[serde(rename = "SourceLayerHash")]
    pub source_layer_hash: Option<StringFilter>,
    #[serde(rename = "Version")]
    pub version: Option<StringFilter>,
    #[serde(rename = "Epoch")]
    pub epoch: Option<NumberFilter>,

}

#[derive(serde::Serialize, Default)]
pub struct MapFilter {
    #[serde(rename = "Key")]
    pub key: Option<MapKey>,
    #[serde(rename = "Value")]
    pub value: Option<MapValue>,
    #[serde(rename = "Comparison")]
    pub comparison: MapComparison,

}
pub type Port = usize;
#[derive(serde::Serialize, Default)]
pub struct DateFilter {
    #[serde(rename = "EndInclusive")]
    pub end_inclusive: Option<Timestamp>,
    #[serde(rename = "StartInclusive")]
    pub start_inclusive: Option<Timestamp>,

}
pub type MapValue = String;
#[derive(serde::Serialize, Default)]
pub struct StringFilterList {

}

#[derive(serde::Serialize, Default)]
pub struct PortRangeFilterList {

}
pub type StringComparison = String;
#[derive(serde::Serialize, Default)]
pub struct DateFilterList {

}

#[derive(serde::Serialize, Default)]
pub struct StringFilter {
    #[serde(rename = "Value")]
    pub value: StringInput,
    #[serde(rename = "Comparison")]
    pub comparison: StringComparison,

}
pub type FilterAction = String;pub type StringInput = String;
#[derive(serde::Serialize, Default)]
pub struct PortRangeFilter {
    #[serde(rename = "BeginInclusive")]
    pub begin_inclusive: Option<Port>,
    #[serde(rename = "EndInclusive")]
    pub end_inclusive: Option<Port>,

}

#[derive(serde::Serialize, Default)]
pub struct PackageFilterList {

}

#[derive(serde::Serialize, Default)]
pub struct NumberFilterList {

}

#[derive(serde::Serialize, Default)]
pub struct FilterCriteria {
    #[serde(rename = "FindingArn")]
    pub finding_arn: Option<StringFilterList>,
    #[serde(rename = "ComponentType")]
    pub component_type: Option<StringFilterList>,
    #[serde(rename = "ComponentId")]
    pub component_id: Option<StringFilterList>,
    #[serde(rename = "FindingType")]
    pub finding_type: Option<StringFilterList>,
    #[serde(rename = "RelatedVulnerabilities")]
    pub related_vulnerabilities: Option<StringFilterList>,
    #[serde(rename = "NetworkProtocol")]
    pub network_protocol: Option<StringFilterList>,
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<StringFilterList>,
    #[serde(rename = "EcrImageArchitecture")]
    pub ecr_image_architecture: Option<StringFilterList>,
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<StringFilterList>,
    #[serde(rename = "Title")]
    pub title: Option<StringFilterList>,
    #[serde(rename = "VulnerabilitySource")]
    pub vulnerability_source: Option<StringFilterList>,
    #[serde(rename = "Ec2InstanceSubnetId")]
    pub ec2_instance_subnet_id: Option<StringFilterList>,
    #[serde(rename = "VendorSeverity")]
    pub vendor_severity: Option<StringFilterList>,
    #[serde(rename = "Severity")]
    pub severity: Option<StringFilterList>,
    #[serde(rename = "EcrImageRegistry")]
    pub ecr_image_registry: Option<StringFilterList>,
    #[serde(rename = "VulnerabilityId")]
    pub vulnerability_id: Option<StringFilterList>,
    #[serde(rename = "FindingStatus")]
    pub finding_status: Option<StringFilterList>,
    #[serde(rename = "EcrImageHash")]
    pub ecr_image_hash: Option<StringFilterList>,
    #[serde(rename = "EcrImageTags")]
    pub ecr_image_tags: Option<StringFilterList>,
    #[serde(rename = "PortRange")]
    pub port_range: Option<PortRangeFilterList>,
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<StringFilterList>,
    #[serde(rename = "ResourceTags")]
    pub resource_tags: Option<MapFilterList>,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<DateFilterList>,
    #[serde(rename = "EcrImagePushedAt")]
    pub ecr_image_pushed_at: Option<DateFilterList>,
    #[serde(rename = "InspectorScore")]
    pub inspector_score: Option<NumberFilterList>,
    #[serde(rename = "VulnerablePackages")]
    pub vulnerable_packages: Option<PackageFilterList>,
    #[serde(rename = "Ec2InstanceVpcId")]
    pub ec2_instance_vpc_id: Option<StringFilterList>,
    #[serde(rename = "Ec2InstanceImageId")]
    pub ec2_instance_image_id: Option<StringFilterList>,
    #[serde(rename = "LastObservedAt")]
    pub last_observed_at: Option<DateFilterList>,
    #[serde(rename = "EcrImageRepositoryName")]
    pub ecr_image_repository_name: Option<StringFilterList>,
    #[serde(rename = "FirstObservedAt")]
    pub first_observed_at: Option<DateFilterList>,

}
pub type MapKey = String;
#[derive(serde::Serialize, Default)]
pub struct NumberFilter {
    #[serde(rename = "LowerInclusive")]
    pub lower_inclusive: Option<f64>,
    #[serde(rename = "UpperInclusive")]
    pub upper_inclusive: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct MapFilterList {

}


}
