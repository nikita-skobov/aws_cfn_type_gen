

/// The AWS::Athena::NamedQuery resource specifies an Amazon Athena saved query, where QueryString contains the SQL query statements that       make up the query.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNamedQuery {


    /// 
    /// The database to which the query belongs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// The query description.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The query name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The SQL statements that make up the query.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 262144
    ///
    /// Update requires: Replacement
    #[serde(rename = "QueryString")]
    pub query_string: String,


    /// 
    /// The name of the workgroup that contains the named query.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [a-zA-Z0-9._-]{1,128}
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkGroup")]
    pub work_group: Option<String>,

}



impl cfn_resources::CfnResource for CfnNamedQuery {
    fn type_string(&self) -> &'static str {
        "AWS::Athena::NamedQuery"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.database;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'database'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.database;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'database'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.description {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 128", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
        let the_val = &self.query_string;

        if the_val.len() > 262144 as _ {
            return Err(format!("Max validation failed on field 'query_string'. {} is greater than 262144", the_val.len()));
        }

        
        let the_val = &self.query_string;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'query_string'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}