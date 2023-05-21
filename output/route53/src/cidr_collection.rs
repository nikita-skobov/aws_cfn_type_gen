/// Creates a CIDR collection in the current AWS account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCidrCollection {
    ///
    /// A complex type that contains information about the list of CIDR locations.
    ///
    /// Required: No
    ///
    /// Type: List of Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "Locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,

    ///
    /// The name of a CIDR collection.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [0-9A-Za-z_\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnCidrCollection {
    fn type_string(&self) -> &'static str {
        "AWS::Route53::CidrCollection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies the list of CIDR blocks for a CIDR location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Location {
    ///
    /// List of CIDR blocks.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrList")]
    pub cidr_list: Vec<String>,

    ///
    /// The CIDR collection location name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16
    ///
    /// Pattern: [0-9A-Za-z_\-\*]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocationName")]
    pub location_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Location {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.cidr_list;

        if the_val.len() > 1000 as _ {
            return Err(format!(
                "Max validation failed on field 'cidr_list'. {} is greater than 1000",
                the_val.len()
            ));
        }

        let the_val = &self.location_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 16 as _ {
                return Err(format!(
                    "Max validation failed on field 'location_name'. {} is greater than 16",
                    s.len()
                ));
            }
        }

        let the_val = &self.location_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'location_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
