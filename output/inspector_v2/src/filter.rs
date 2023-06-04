/// Details about a filter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The action that is to be applied to the findings that match the filter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterAction")]
    pub filter_action: cfn_resources::StrVal,

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
    pub name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnFilterarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFilterarn;
impl CfnFilterarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnFilter {
    fn type_string(&self) -> &'static str {
        "AWS::InspectorV2::Filter"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.filter_criteria.validate()?;

        Ok(())
    }
}

/// Contains details on the time range used to filter findings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_inclusive: Option<i64>,
}

impl cfn_resources::CfnResource for DateFilter {
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

/// Details on the criteria used to define the filter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_id: Option<Vec<StringFilter>>,

    /// Details on the vulnerability score to filter findings by.
    ///
    /// Required: No
    ///
    /// Type: List of StringFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "VulnerabilitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable_packages: Option<Vec<PackageFilter>>,
}

impl cfn_resources::CfnResource for FilterCriteria {
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

/// An object that describes details of a map filter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub comparison: cfn_resources::StrVal,

    ///
    /// The tag key used in the filter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The tag value used in the filter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MapFilter {
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

/// An object that describes the details of a number filter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_inclusive: Option<f64>,
}

impl cfn_resources::CfnResource for NumberFilter {
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

/// Contains information on the details of a package filter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<StringFilter>,
}

impl cfn_resources::CfnResource for PackageFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.architecture
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.epoch.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.name.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.release.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.source_layer_hash
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.version.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that describes the details of a port range filter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PortRangeFilter {
    /// The port number the port range begins at.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BeginInclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_inclusive: Option<i64>,
}

impl cfn_resources::CfnResource for PortRangeFilter {
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

/// An object that describes the details of a string filter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub comparison: cfn_resources::StrVal,

    ///
    /// The value to filter on.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StringFilter {
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
