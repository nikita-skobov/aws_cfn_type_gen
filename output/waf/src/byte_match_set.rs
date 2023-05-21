

/// The AWS::WAF::ByteMatchSet resource creates an AWS WAF ByteMatchSet that identifies a part of a web request that you want to inspect.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnByteMatchSet {


    /// 
    /// The name of the ByteMatchSet. You can't change Name after you create a ByteMatchSet.
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
    /// Specifies the bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings.
    /// 
    /// Required: No
    ///
    /// Type: List of ByteMatchTuple
    ///
    /// Update requires: No interruption
    #[serde(rename = "ByteMatchTuples")]
    pub byte_match_tuples: Option<Vec<ByteMatchTuple>>,

}



impl cfn_resources::CfnResource for CfnByteMatchSet {
    fn type_string() -> &'static str {
        "AWS::WAF::ByteMatchSet"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies where in a web request to look for TargetString.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FieldToMatch {


    /// 
    /// When the value of Type is HEADER, enter the name of the header that you want AWS WAF to search, 			for example, User-Agent or Referer. The name of the header is not case sensitive.
    /// 
    /// When the value of Type is SINGLE_QUERY_ARG, enter the name of the parameter that you want AWS WAF to search, 	    for example, UserName or SalesRegion. The parameter name is not case sensitive.
    /// 
    /// If the value of Type is any other value, omit Data.
    /// 
    /// Required: No
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
    /// HEADER: A specified request header, for example, the value of the User-Agent or Referer header. 				If you choose HEADER for the type, specify the name of the header in Data.                        METHOD: The HTTP method, which indicated the type of operation that the request is asking the origin to perform.          Amazon CloudFront supports the following methods: DELETE, GET, HEAD, OPTIONS, PATCH, 				POST, and PUT.                        QUERY_STRING: A query string, which is the part of a URL that appears after a ? character, if any.                        URI: The part of a web request that identifies a resource, for example, /images/daily-ad.jpg.                        BODY: The part of a request that contains any additional data that you want to send to your web server 				as the HTTP request body, such as data from a form. The request body immediately follows the request headers. 				Note that only the first 8192 bytes of the request body are forwarded to AWS WAF for inspection. 				To allow or block requests based on the length of the body, you can create a size constraint set.       		       		        SINGLE_QUERY_ARG: The parameter in the query string that you will inspect, such as UserName or SalesRegion. The maximum length for SINGLE_QUERY_ARG is 30 characters. 		      		       		        ALL_QUERY_ARGS: Similar to SINGLE_QUERY_ARG, but rather than inspecting a single parameter, AWS WAF will inspect all parameters within the query for the value or regex pattern that you specify in 		       TargetString.
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



/// The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ByteMatchTuple {


    /// 
    /// Within the portion of a web request that you want to search (for example, in the query string, if any), specify where you want AWS WAF to search. Valid values include the following:
    /// 
    /// CONTAINS
    /// 
    /// The specified part of the web request must include the value of TargetString, but the location doesn't matter.
    /// 
    /// CONTAINS_WORD
    /// 
    /// The specified part of the web request must include the value of TargetString, and 			TargetString must contain only alphanumeric characters or underscore (A-Z, a-z, 0-9, or _). In addition, 			TargetString must be a word, which means one of the following:
    /// 
    /// TargetString exactly matches the value of the specified part of the web request, such as the value of a 					header.                        TargetString is at the beginning of the specified part of the web request and is followed by a character 					other than an alphanumeric character or underscore (_), for example, BadBot;.                        TargetString is at the end of the specified part of the web request and is preceded by a character 					other than an alphanumeric character or underscore (_), for example, ;BadBot.                        TargetString is in the middle of the specified part of the web request and is preceded and followed 					by characters other than alphanumeric characters or underscore (_), for example, -BadBot;.
    /// 
    /// EXACTLY
    /// 
    /// The value of the specified part of the web request must exactly match the value of TargetString.
    /// 
    /// STARTS_WITH
    /// 
    /// The value of TargetString must appear at the beginning of the specified part of the web request.
    /// 
    /// ENDS_WITH
    /// 
    /// The value of TargetString must appear at the end of the specified part of the web request.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONTAINS | CONTAINS_WORD | ENDS_WITH | EXACTLY | STARTS_WITH
    ///
    /// Update requires: No interruption
    #[serde(rename = "PositionalConstraint")]
    pub positional_constraint: ByteMatchTuplePositionalConstraintEnum,


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
    /// The base64-encoded value that AWS WAF searches for. AWS CloudFormation sends this value to AWS WAF without encoding it.
    /// 
    /// You must specify this property or the TargetString property.
    /// 
    /// AWS WAF searches for this value in a specific part of web requests, which you define in the FieldToMatch property.
    /// 
    /// Valid values depend on the Type value in the FieldToMatch property. For example, for a METHOD type, you must specify HTTP methods such as DELETE, GET, HEAD, OPTIONS, PATCH, POST, and PUT.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetStringBase64")]
    pub target_string_base64: Option<String>,


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
    pub text_transformation: ByteMatchTupleTextTransformationEnum,


    /// 
    /// The value that you want AWS WAF to search for. AWS WAF searches for the specified string in the part of web requests that you 			specified in FieldToMatch. The maximum length of the value is 50 bytes.
    /// 
    /// You must specify this property or the TargetStringBase64 property.
    /// 
    /// Valid values depend on the values that you specified for FieldToMatch:
    /// 
    /// HEADER: The value that you want AWS WAF to search for in the request header that you specified in          FieldToMatch, for example, the value of the User-Agent or Referer header.                        METHOD: The HTTP method, which indicates the type of operation specified in the request.          Amazon CloudFront supports the following methods: DELETE, GET, HEAD, OPTIONS, 				PATCH, POST, and PUT.                        QUERY_STRING: The value that you want AWS WAF to search for in the query string, which is the part 				of a URL that appears after a ? character.                        URI: The value that you want AWS WAF to search for in the part of a URL that identifies a resource, 				for example, /images/daily-ad.jpg.                        BODY: The part of a request that contains any additional data that you want to send to your web server 				as the HTTP request body, such as data from a form. The request body immediately follows the request headers. 				Note that only the first 8192 bytes of the request body are forwarded to AWS WAF for inspection. 				To allow or block requests based on the length of the body, you can create a size constraint set.        		       		        SINGLE_QUERY_ARG: The parameter in the query string that you will inspect, such as UserName or SalesRegion. The maximum length for SINGLE_QUERY_ARG is 30 characters. 		      		                ALL_QUERY_ARGS: Similar to SINGLE_QUERY_ARG, but instead of        inspecting a single parameter, AWS WAF inspects all parameters within the query        string for the value or regex pattern that you specify in        TargetString.
    /// 
    /// If TargetString includes alphabetic characters A-Z and a-z, note that the value is case sensitive.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetString")]
    pub target_string: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ByteMatchTupleTextTransformationEnum {

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

impl Default for ByteMatchTupleTextTransformationEnum {
    fn default() -> Self {
        ByteMatchTupleTextTransformationEnum::Cmdline
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ByteMatchTuplePositionalConstraintEnum {

    /// CONTAINS
    #[serde(rename = "CONTAINS")]
    Contains,

    /// CONTAINS_WORD
    #[serde(rename = "CONTAINS_WORD")]
    Containsword,

    /// ENDS_WITH
    #[serde(rename = "ENDS_WITH")]
    Endswith,

    /// EXACTLY
    #[serde(rename = "EXACTLY")]
    Exactly,

    /// STARTS_WITH
    #[serde(rename = "STARTS_WITH")]
    Startswith,

}

impl Default for ByteMatchTuplePositionalConstraintEnum {
    fn default() -> Self {
        ByteMatchTuplePositionalConstraintEnum::Contains
    }
}

