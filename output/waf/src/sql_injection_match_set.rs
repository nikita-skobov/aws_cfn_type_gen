

/// A complex type that contains SqlInjectionMatchTuple objects, which specify the parts of web requests that you           want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header. If a 			SqlInjectionMatchSet contains more than one SqlInjectionMatchTuple object, a request needs to 			include snippets of SQL code in only one of the specified parts of the request to be considered a match.
#[derive(Default, serde::Serialize)]
pub struct CfnSqlInjectionMatchSet {


    /// 
    /// Specifies the parts of web requests that you want to inspect for snippets of malicious SQL code.
    /// 
    /// Required: No
    ///
    /// Type: List of SqlInjectionMatchTuple
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlInjectionMatchTuples")]
    pub sql_injection_match_tuples: Option<Vec<SqlInjectionMatchTuple>>,


    /// 
    /// The name, if any, of the SqlInjectionMatchSet.
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

}


/// Specifies where in a web request to look for TargetString.
#[derive(Default, serde::Serialize)]
pub struct FieldToMatch {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples-fieldtomatch.html#cfn-waf-sizeconstraintset-sizeconstraint-fieldtomatch-type
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples-fieldtomatch.html#cfn-waf-sizeconstraintset-sizeconstraint-fieldtomatch-data
    #[serde(rename = "Data")]
    pub data: Option<String>,

}


/// Specifies the part of a web request that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.
#[derive(Default, serde::Serialize)]
pub struct SqlInjectionMatchTuple {


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
    pub text_transformation: String,

}
