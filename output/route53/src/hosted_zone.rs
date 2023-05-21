

/// Creates a new public or private hosted zone. You create records in a public hosted 			zone to define how you want to route traffic on the internet for a domain, such as 			example.com, and its subdomains (apex.example.com, acme.example.com). You create records 			in a private hosted zone to define how you want to route traffic for a domain and its 			subdomains within one or more Amazon Virtual Private Clouds (Amazon VPCs).
///
/// For more information about charges for hosted zones, see Amazon Route 53 Pricing.
///
/// Note the following:
///
/// When you submit a CreateHostedZone request, the initial status of the 			hosted zone is PENDING. For public hosted zones, this means that the NS and 			SOA records are not yet available on all Route 53 DNS servers. When the NS and 			SOA records are available, the status of the zone changes to INSYNC.
///
/// The CreateHostedZone request requires the caller to have an 				ec2:DescribeVpcs permission.
#[derive(Default, serde::Serialize)]
pub struct CfnHostedZone {


    /// 
    /// Creates a configuration for DNS query logging. After you create a query logging 			configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs 			log group.
    /// 
    /// DNS query logs contain information about the queries that Route 53 receives for a 			specified public hosted zone, such as the following:
    /// 
    /// Route 53 edge location that responded to the DNS query               Domain or subdomain that was requested               DNS record type, such as A or AAAA               DNS response code, such as NoError or 					ServFail
    /// 
    /// Log Group and Resource Policy                  Before you create a query logging configuration, perform the following 						operations.          NoteIf you create a query logging configuration using the Route 53 							console, Route 53 performs these operations automatically.                                                       Create a CloudWatch Logs log group, and make note of the ARN, 								which you specify when you create a query logging configuration. 								Note the following:                                                                                    You must create the log group in the us-east-1 										region.                              You must use the same AWS account to create 										the log group and the hosted zone that you want to configure 										query logging for.                              When you create log groups for query logging, we recommend 										that you use a consistent prefix, for example:                                 /aws/route53/hosted zone 											name                                                In the next step, you'll create a resource policy, which 										controls access to one or more log groups and the associated 											AWS resources, such as Route 53 hosted 										zones. There's a limit on the number of resource policies 										that you can create, so we recommend that you use a 										consistent prefix so you can use the same resource policy 										for all the log groups that you create for query 										logging.                                      Create a CloudWatch Logs resource policy, and give it the 								permissions that Route 53 needs to create log streams and to send 								query logs to log streams. For the value of Resource, 								specify the ARN for the log group that you created in the previous 								step. To use the same resource policy for all the CloudWatch Logs 								log groups that you created for query logging configurations, 								replace the hosted zone name with *, for 								example:                           arn:aws:logs:us-east-1:123412341234:log-group:/aws/route53/*                         To avoid the confused deputy problem, a security issue where an 								entity without a permission for an action can coerce a 								more-privileged entity to perform it, you can optionally limit the 								permissions that a service has to a resource in a resource-based 								policy by supplying the following values:                                                                      For aws:SourceArn, supply the hosted zone ARN 										used in creating the query logging configuration. For 										example, aws:SourceArn: 											arn:aws:route53:::hostedzone/hosted zone 										ID.                              For aws:SourceAccount, supply the account ID 										for the account that creates the query logging 										configuration. For example, 											aws:SourceAccount:111111111111.                           For more information, see The confused 									deputy problem in the                AWS 									IAM User Guide.             NoteYou can't use the CloudWatch console to create or edit a 									resource policy. You must use the CloudWatch API, one of the 										AWS SDKs, or the AWS CLI.                                  Log Streams and Edge Locations                  When Route 53 finishes creating the configuration for DNS query logging, 						it does the following:                                                       Creates a log stream for an edge location the first time that the 								edge location responds to DNS queries for the specified hosted zone. 								That log stream is used to log all queries that Route 53 responds to 								for that edge location.                        Begins to send query logs to the applicable log stream.                     The name of each log stream is in the following format:                                 hosted zone ID/edge location 								code                              The edge location code is a three-letter code and an arbitrarily assigned 						number, for example, DFW3. The three-letter code typically corresponds with 						the International Air Transport Association airport code for an airport near 						the edge location. (These abbreviations might change in the future.) For a 						list of edge locations, see "The Route 53 Global Network" on the Route 53 Product Details 						page.                       Queries That Are Logged                  Query logs contain only the queries that DNS resolvers forward to Route 						53. If a DNS resolver has already cached the response to a query (such as 						the IP address for a load balancer for example.com), the resolver will 						continue to return the cached response. It doesn't forward another query to 						Route 53 until the TTL for the corresponding resource record set expires. 						Depending on how many DNS queries are submitted for a resource record set, 						and depending on the TTL for that resource record set, query logs might 						contain information about only one query out of every several thousand 						queries that are submitted to DNS. For more information about how DNS works, 						see Routing 							Internet Traffic to Your Website or Web Application in the 							Amazon Route 53 Developer Guide.                       Log File Format                  For a list of the values in each query log and the format of each value, 						see Logging DNS 							Queries in the Amazon Route 53 Developer 							Guide.                       Pricing                  For information about charges for query logs, see Amazon CloudWatch Pricing.                       How to Stop Logging                  If you want Route 53 to stop sending query logs to CloudWatch Logs, delete 						the query logging configuration. For more information, see DeleteQueryLoggingConfig.
    /// 
    /// Required: No
    ///
    /// Type: QueryLoggingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryLoggingConfig")]
    pub query_logging_config: Option<QueryLoggingConfig>,


    /// 
    /// Adds, edits, or deletes tags for a health check or a hosted zone.
    /// 
    /// For information about using tags for cost allocation, see Using Cost Allocation 				Tags in the         AWS Billing and Cost Management User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of HostedZoneTag
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedZoneTags")]
    pub hosted_zone_tags: Option<Vec<HostedZoneTag>>,


    /// 
    /// A complex type that contains an optional comment.
    /// 
    /// If you don't want to specify a comment, omit the HostedZoneConfig and Comment elements.
    /// 
    /// Required: No
    ///
    /// Type: HostedZoneConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedZoneConfig")]
    pub hosted_zone_config: Option<HostedZoneConfig>,


    /// 
    /// The name of the domain. Specify a fully qualified domain name, for example, www.example.com. 			The trailing dot is optional; Amazon Route 53 assumes that the domain name is fully qualified. This means that Route 53 treats 			www.example.com (without a trailing dot) and www.example.com. (with a trailing dot) as identical.
    /// 
    /// If you're creating a public hosted zone, this is the name you have registered with your DNS registrar. If your domain name 			is registered with a registrar other than Route 53, change the name servers for your domain to the set of NameServers that 			are returned by the Fn::GetAtt intrinsic function.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Private hosted zones: A complex type that contains information about the VPCs that are 			associated with the specified hosted zone.
    /// 
    /// NoteFor public hosted zones, omit VPCs, VPCId, and VPCRegion.
    /// 
    /// Required: No
    ///
    /// Type: List of VPC
    ///
    /// Update requires: No interruption
    #[serde(rename = "VPCs")]
    pub vpcs: Option<Vec<VPC>>,

}


/// A complex type that contains information about a configuration for DNS query 			logging.
#[derive(Default, serde::Serialize)]
pub struct QueryLoggingConfig {


    /// 
    /// The Amazon Resource Name (ARN) of the CloudWatch Logs log group that Amazon Route 53 			is publishing logs to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: String,

}


/// A complex type that contains an optional comment about your hosted zone. If you don't want to specify a comment, omit both the 			HostedZoneConfig and Comment elements.
#[derive(Default, serde::Serialize)]
pub struct HostedZoneConfig {


    /// 
    /// Any comments that you want to include about the hosted zone.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

}


/// Private hosted zones only: A complex type that contains information about an Amazon VPC. Route 53 Resolver 			uses the records in the private hosted zone to route traffic in that VPC.
#[derive(Default, serde::Serialize)]
pub struct VPC {


    /// 
    /// Private hosted zones only: The ID of an Amazon VPC.
    /// 
    /// NoteFor public hosted zones, omit VPCs, VPCId, and VPCRegion.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VPCId")]
    pub vpcid: String,


    /// 
    /// Private hosted zones only: The region that an Amazon VPC was created in.
    /// 
    /// NoteFor public hosted zones, omit VPCs, VPCId, and VPCRegion.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: af-south-1 | ap-east-1 | ap-northeast-1 | ap-northeast-2 | ap-northeast-3 | ap-south-1 | ap-southeast-1 | ap-southeast-2 | ap-southeast-3 | ca-central-1 | cn-north-1 | eu-central-1 | eu-north-1 | eu-south-1 | eu-west-1 | eu-west-2 | eu-west-3 | me-south-1 | sa-east-1 | us-east-1 | us-east-2 | us-gov-east-1 | us-gov-west-1 | us-iso-east-1 | us-iso-west-1 | us-isob-east-1 | us-west-1 | us-west-2
    ///
    /// Update requires: No interruption
    #[serde(rename = "VPCRegion")]
    pub vpcregion: String,

}


/// A complex type that contains information about a tag that you want to add or edit for 			the specified health check or hosted zone.
#[derive(Default, serde::Serialize)]
pub struct HostedZoneTag {


    /// 
    /// The value of Value depends on the operation that you want to 			perform:
    /// 
    /// Add a tag to a health check or hosted zone: 						Value is the value that you want to give the new tag.                        Edit a tag: Value is the new 					value that you want to assign the tag.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The value of Key depends on the operation that you want to 			perform:
    /// 
    /// Add a tag to a health check or hosted zone: 						Key is the name that you want to give the new tag.                        Edit a tag: Key is the name of 					the tag that you want to change the Value for.                         Delete a key: Key is the name 					of the tag you want to remove.                        Give a name to a health check: Edit the 					default Name tag. In the Amazon Route 53 console, the list of your 					health checks includes a Name column that lets 					you see the name that you've given to each health check.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}
