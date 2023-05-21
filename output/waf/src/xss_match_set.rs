

/// A complex type that contains XssMatchTuple objects, which specify the parts of web requests that you           want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header. If a 			XssMatchSet contains more than one XssMatchTuple object, a request needs to 			include cross-site scripting attacks in only one of the specified parts of the request to be considered a match.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnXssMatchSet {


    /// 
    /// The name, if any, of the XssMatchSet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Specifies the parts of web requests that you want to inspect for cross-site scripting attacks.
    /// 
    /// Required: Yes
    ///
    /// Type: List of XssMatchTuple
    ///
    /// Update requires: No interruption
    #[serde(rename = "XssMatchTuples")]
    pub xss_match_tuples: Vec<XssMatchTuple>,

}



impl cfn_resources::CfnResource for CfnXssMatchSet {
    fn type_string() -> &'static str {
        "AWS::WAF::XssMatchSet"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.name;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The part of a web request that you want to inspect, such as a specified header or a query string.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FieldToMatch {


    /// 
    /// When the value of Type is HEADER, enter the name of the header that you want AWS WAF to search, 			for example, User-Agent or Referer. The name of the header is not case sensitive.
    /// 
    /// When the value of Type is SINGLE_QUERY_ARG, enter the name of the parameter that you want AWS WAF to search, 	    for example, UserName or SalesRegion. The parameter name is not case sensitive.
    /// 
    /// If the value of Type is any other value, omit Data.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Data")]
    pub data: Option<String>,


    /// 
    /// The part of the web request that you want AWS WAF to search for a specified string. Parts of a request that you can search include the following:
    /// 
    /// HEADER: A specified request header, for example, the value of the User-Agent or Referer header. 				If you choose HEADER for the type, specify the name of the header in Data.                        METHOD: The HTTP method, which indicated the type of operation that the request is asking the origin to perform.          Amazon CloudFront supports the following methods: DELETE, GET, HEAD, OPTIONS, PATCH, 				POST, and PUT.                        QUERY_STRING: A query string, which is the part of a URL that appears after a ? character, if any.                        URI: The part of a web request that identifies a resource, for example, /images/daily-ad.jpg.                        BODY: The part of a request that contains any additional data that you want to send to your web server 				as the HTTP request body, such as data from a form. The request body immediately follows the request headers. 				Note that only the first 8192 bytes of the request body are forwarded to AWS WAF for inspection. 				To allow or block requests based on the length of the body, you can create a size constraint set.        		       		        SINGLE_QUERY_ARG: The parameter in the query string that you will inspect, such as UserName or SalesRegion. The maximum length for SINGLE_QUERY_ARG is 30 characters. 		      		       		        ALL_QUERY_ARGS: Similar to SINGLE_QUERY_ARG, but rather than inspecting a single parameter, AWS WAF will inspect all parameters within the query for the value or regex pattern that you specify in 		       TargetString.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL_QUERY_ARGS | BODY | HEADER | METHOD | QUERY_STRING | SINGLE_QUERY_ARG | URI
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: FieldToMatchTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FieldToMatchTypeEnum {

    /// ALL_QUERY_ARGS
    #[serde(rename = "ALL_QUERY_ARGS")]
    Allqueryargs,

    /// BODY
    #[serde(rename = "BODY")]
    Body,

    /// HEADER
    #[serde(rename = "HEADER")]
    Header,

    /// METHOD
    #[serde(rename = "METHOD")]
    Method,

    /// QUERY_STRING
    #[serde(rename = "QUERY_STRING")]
    Querystring,

    /// SINGLE_QUERY_ARG
    #[serde(rename = "SINGLE_QUERY_ARG")]
    Singlequeryarg,

    /// URI
    #[serde(rename = "URI")]
    Uri,

}

impl Default for FieldToMatchTypeEnum {
    fn default() -> Self {
        FieldToMatchTypeEnum::Allqueryargs
    }
}


impl cfn_resources::CfnResource for FieldToMatch {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.data {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'data'. {} is greater than 128", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.data {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'data'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Specifies the part of a web request that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct XssMatchTuple {


    /// 
    /// The part of a web request that you want to inspect, such as a specified header or a query string.
    /// 
    /// Required: Yes
    ///
    /// Type: FieldToMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,


    /// 
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF.          If you specify a transformation, AWS WAF performs the transformation on FieldToMatch before inspecting it for a match.
    /// 
    /// You can only specify a single type of TextTransformation.
    /// 
    /// CMD_LINE
    /// 
    /// When you're concerned that attackers are injecting an operating system command line     command and using unusual formatting to disguise some or all of the command, use this     option to perform the following transformations:
    /// 
    /// Delete the following characters: \ " ' ^               Delete spaces before the following characters: / (               Replace the following characters with a space: , ;               Replace multiple spaces with one space               Convert uppercase letters (A-Z) to lowercase (a-z)
    /// 
    /// COMPRESS_WHITE_SPACE
    /// 
    /// Use this option to replace the following characters with a space character (decimal 32):
    /// 
    /// \f, formfeed, decimal 12               \t, tab, decimal 9               \n, newline, decimal 10               \r, carriage return, decimal 13               \v, vertical tab, decimal 11               non-breaking space, decimal 160
    /// 
    /// COMPRESS_WHITE_SPACE also replaces multiple spaces with one space.
    /// 
    /// HTML_ENTITY_DECODE
    /// 
    /// Use this option to replace HTML-encoded characters with unencoded characters. HTML_ENTITY_DECODE performs 			the following operations:
    /// 
    /// Replaces (ampersand)quot; with "                       Replaces (ampersand)nbsp; with a non-breaking space, decimal 160               Replaces (ampersand)lt; with a "less than" symbol               Replaces (ampersand)gt; with >                       Replaces characters that are represented in hexadecimal format, (ampersand)#xhhhh;, with the corresponding 				characters               Replaces characters that are represented in decimal format, (ampersand)#nnnn;, with the corresponding 				characters
    /// 
    /// LOWERCASE
    /// 
    /// Use this option to convert uppercase letters (A-Z) to lowercase (a-z).
    /// 
    /// URL_DECODE
    /// 
    /// Use this option to decode a URL-encoded value.
    /// 
    /// NONE
    /// 
    /// Specify NONE if you don't want to perform any text transformations.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CMD_LINE | COMPRESS_WHITE_SPACE | HTML_ENTITY_DECODE | LOWERCASE | NONE | URL_DECODE
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextTransformation")]
    pub text_transformation: XssMatchTupleTextTransformationEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum XssMatchTupleTextTransformationEnum {

    /// CMD_LINE
    #[serde(rename = "CMD_LINE")]
    Cmdline,

    /// COMPRESS_WHITE_SPACE
    #[serde(rename = "COMPRESS_WHITE_SPACE")]
    Compresswhitespace,

    /// HTML_ENTITY_DECODE
    #[serde(rename = "HTML_ENTITY_DECODE")]
    Htmlentitydecode,

    /// LOWERCASE
    #[serde(rename = "LOWERCASE")]
    Lowercase,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// URL_DECODE
    #[serde(rename = "URL_DECODE")]
    Urldecode,

}

impl Default for XssMatchTupleTextTransformationEnum {
    fn default() -> Self {
        XssMatchTupleTextTransformationEnum::Cmdline
    }
}


impl cfn_resources::CfnResource for XssMatchTuple {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.field_to_match.validate()?;

        Ok(())
    }
}