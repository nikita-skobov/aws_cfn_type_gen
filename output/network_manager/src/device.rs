/// Specifies a device.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDevice {
    ///
    /// A description of the device.
    ///
    /// Constraints: Maximum length of 256 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The ID of the global network.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,

    ///
    /// The site location.
    ///
    /// Required: No
    ///
    /// Type: Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<Location>,

    ///
    /// The model of the device.
    ///
    /// Constraints: Maximum length of 128 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Model")]
    pub model: Option<String>,

    ///
    /// The serial number of the device.
    ///
    /// Constraints: Maximum length of 128 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

    ///
    /// The site ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SiteId")]
    pub site_id: Option<String>,

    ///
    /// The tags for the device.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The device type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

    ///
    /// The vendor of the device.
    ///
    /// Constraints: Maximum length of 128 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,
}

impl cfn_resources::CfnResource for CfnDevice {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::Device"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'description'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.global_network_id;

        if the_val.len() > 50 as _ {
            return Err(format!(
                "Max validation failed on field 'global_network_id'. {} is greater than 50",
                the_val.len()
            ));
        }

        let the_val = &self.global_network_id;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'global_network_id'. {} is less than 0",
                the_val.len()
            ));
        }

        self.location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.model {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'model'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.model {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'model'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.serial_number {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'serial_number'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.serial_number {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'serial_number'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.site_id {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'site_id'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.site_id {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'site_id'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.cfn_type {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'cfn_type'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.cfn_type {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'cfn_type'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.vendor {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'vendor'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.vendor {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'vendor'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Describes a location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Location {
    ///
    /// The physical address.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<String>,

    ///
    /// The latitude.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Latitude")]
    pub latitude: Option<String>,

    ///
    /// The longitude.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Longitude")]
    pub longitude: Option<String>,
}

impl cfn_resources::CfnResource for Location {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.address {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'address'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.address {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'address'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.latitude {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'latitude'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.latitude {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'latitude'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.longitude {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'longitude'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.longitude {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'longitude'. {} is less than 0",
                    the_val.len()
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
