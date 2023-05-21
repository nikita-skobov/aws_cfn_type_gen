/// The AWS::GuardDuty::Filter resource specifies a new filter defined by          the provided findingCriteria.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFilter {
    ///
    /// Specifies the action that is to be applied to the findings that match the filter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ARCHIVE | NOOP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: FilterActionEnum,

    ///
    /// The description of the filter. Valid characters include alphanumeric characters, and    special characters such as hyphen, period, colon, underscore, parentheses ({ },     [ ], and ( )), forward slash, horizontal tab, vertical tab,    newline, form feed, return, and whitespace.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: String,

    ///
    /// The ID of the detector belonging to the GuardDuty account that you want to create a filter    for.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Update requires: Replacement
    #[serde(rename = "DetectorId")]
    pub detector_id: String,

    ///
    /// Represents the criteria to be used in the filter for querying findings.
    ///
    /// Required: Yes
    ///
    /// Type: FindingCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "FindingCriteria")]
    pub finding_criteria: FindingCriteria,

    ///
    /// The name of the filter. Valid characters include period (.), underscore (_), dash (-), and    alphanumeric characters. A whitespace is considered to be an invalid character.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Specifies the position of the filter in the list of current filters. Also          specifies the order in which this filter is applied to the findings. The minimum          value for this property is 1 and the maximum is 100.
    ///
    /// By default, filters may not be created in the same order as they are ranked. To          ensure that the filters are created in the expected order, you can use an optional          attribute, DependsOn, with the following syntax: "DependsOn":[             "ObjectName" ].
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rank")]
    pub rank: i64,

    ///
    /// The tags to be added to a new filter resource. Each tag consists of a key and an          optional value, both of which you define.
    ///
    /// For more information, see Tag.
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
pub enum FilterActionEnum {
    /// ARCHIVE
    #[serde(rename = "ARCHIVE")]
    Archive,

    /// NOOP
    #[serde(rename = "NOOP")]
    Noop,
}

impl Default for FilterActionEnum {
    fn default() -> Self {
        FilterActionEnum::Archive
    }
}

impl cfn_resources::CfnResource for CfnFilter {
    fn type_string(&self) -> &'static str {
        "AWS::GuardDuty::Filter"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.description;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'description'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.description;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'description'. {} is less than 0",
                the_val.len()
            ));
        }

        let the_val = &self.detector_id;

        if the_val.len() > 300 as _ {
            return Err(format!(
                "Max validation failed on field 'detector_id'. {} is greater than 300",
                the_val.len()
            ));
        }

        let the_val = &self.detector_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'detector_id'. {} is less than 1",
                the_val.len()
            ));
        }

        self.finding_criteria.validate()?;

        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 3 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 3",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Specifies the condition to apply to a single field when filtering through findings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Condition {
    ///
    /// Represents the equal condition to apply to a single field when querying for          findings.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Eq")]
    pub eq: Option<Vec<String>>,

    ///
    /// Represents an equal       condition to be applied to    a single field when querying for findings.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Equals")]
    pub equals: Option<Vec<String>>,

    ///
    /// Represents a greater than condition to be applied to a single field    when querying for findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "GreaterThan")]
    pub greater_than: Option<i64>,

    ///
    /// Represents a greater than or equal condition to be applied to a    single field when querying for findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "GreaterThanOrEqual")]
    pub greater_than_or_equal: Option<i64>,

    ///
    /// Represents a greater than condition to be applied to a single field    when querying for findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Gt")]
    pub gt: Option<i64>,

    ///
    /// Represents the greater than or equal condition to apply to a single field when          querying for findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Gte")]
    pub gte: Option<i64>,

    ///
    /// Represents a less than condition to be applied to a single field when    querying for findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "LessThan")]
    pub less_than: Option<i64>,

    ///
    /// Represents a less than or equal condition to be applied to a single    field when querying for findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "LessThanOrEqual")]
    pub less_than_or_equal: Option<i64>,

    ///
    /// Represents the less than condition to apply to a single field when querying for          findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lt")]
    pub lt: Option<i64>,

    ///
    /// Represents the less than or equal condition to apply to a single field when          querying for findings.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lte")]
    pub lte: Option<i64>,

    ///
    /// Represents the not equal condition to apply to a single field when querying for          findings.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Neq")]
    pub neq: Option<Vec<String>>,

    ///
    /// Represents a not equal       condition to be applied    to a single field when querying for findings.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotEquals")]
    pub not_equals: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for Condition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Represents a map of finding properties that match specified conditions and values          when querying findings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FindingCriteria {
    ///
    /// Represents a map of finding properties that match specified conditions and values          when querying findings.
    ///
    /// For a mapping of JSON criterion to their console equivalent see Finding criteria. The following are the available criterion:
    ///
    /// accountId                      region                      confidence                      id                      resource.accessKeyDetails.accessKeyId                      resource.accessKeyDetails.principalId                      resource.accessKeyDetails.userName                      resource.accessKeyDetails.userType                      resource.instanceDetails.iamInstanceProfile.id                      resource.instanceDetails.imageId                      resource.instanceDetails.instanceId                      resource.instanceDetails.outpostArn                      resource.instanceDetails.networkInterfaces.ipv6Addresses                      resource.instanceDetails.networkInterfaces.privateIpAddresses.privateIpAddress                      resource.instanceDetails.networkInterfaces.publicDnsName                      resource.instanceDetails.networkInterfaces.publicIp                      resource.instanceDetails.networkInterfaces.securityGroups.groupId                      resource.instanceDetails.networkInterfaces.securityGroups.groupName                      resource.instanceDetails.networkInterfaces.subnetId                      resource.instanceDetails.networkInterfaces.vpcId                      resource.instanceDetails.tags.key                      resource.instanceDetails.tags.value                      resource.resourceType                      service.action.actionType                      service.action.awsApiCallAction.api                      service.action.awsApiCallAction.callerType                      service.action.awsApiCallAction.errorCode                      service.action.awsApiCallAction.remoteIpDetails.city.cityName                      service.action.awsApiCallAction.remoteIpDetails.country.countryName                      service.action.awsApiCallAction.remoteIpDetails.ipAddressV4                      service.action.awsApiCallAction.remoteIpDetails.organization.asn                      service.action.awsApiCallAction.remoteIpDetails.organization.asnOrg                      service.action.awsApiCallAction.serviceName                      service.action.dnsRequestAction.domain                      service.action.networkConnectionAction.blocked                      service.action.networkConnectionAction.connectionDirection                      service.action.networkConnectionAction.localPortDetails.port                      service.action.networkConnectionAction.protocol                      service.action.networkConnectionAction.localIpDetails.ipAddressV4                      service.action.networkConnectionAction.remoteIpDetails.city.cityName                      service.action.networkConnectionAction.remoteIpDetails.country.countryName                      service.action.networkConnectionAction.remoteIpDetails.ipAddressV4                      service.action.networkConnectionAction.remoteIpDetails.organization.asn                      service.action.networkConnectionAction.remoteIpDetails.organization.asnOrg                      service.action.networkConnectionAction.remotePortDetails.port                      service.additionalInfo.threatListName                      service.archived             When this attribute is set to TRUE, only archived findings are listed.                When it's set to FALSE, only unarchived findings are listed. When this                attribute is not set, all existing findings are listed.                      service.resourceRole                      severity                      type                      updatedAt             Type: ISO 8601 string format: YYYY-MM-DDTHH:MM:SS.SSSZ or                YYYY-MM-DDTHH:MM:SSZ depending on whether the value contains                milliseconds.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Criterion")]
    pub criterion: Option<serde_json::Value>,

    ///
    /// Specifies the condition to be applied to a single field when filtering through          findings.
    ///
    /// Required: No
    ///
    /// Type: Condition
    ///
    /// Update requires: No interruption
    #[serde(rename = "ItemType")]
    pub item_type: Option<Condition>,
}

impl cfn_resources::CfnResource for FindingCriteria {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.item_type
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
