

/// Contains one or more countries that AWS WAF will search for.
#[derive(Default, serde::Serialize)]
pub struct CfnGeoMatchSet {


    /// 
    /// A friendly name or description of the AWS::WAFRegional::GeoMatchSet. You can't change the name of an GeoMatchSet after you create it.
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
    /// An array of GeoMatchConstraint objects, which contain the country that you want AWS WAF to search for.
    /// 
    /// Required: No
    ///
    /// Type: List of GeoMatchConstraint
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeoMatchConstraints")]
    pub geo_match_constraints: Option<Vec<GeoMatchConstraint>>,

}


/// The country from which web requests originate that you want AWS WAF to search for.
#[derive(Default, serde::Serialize)]
pub struct GeoMatchConstraint {


    /// 
    /// The type of geographical area you want AWS WAF to search for. Currently Country is the only valid value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Country
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The country that you want AWS WAF to search for.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AD | AE | AF | AG | AI | AL | AM | AO | AQ | AR | AS | AT | AU | AW | AX | AZ | BA | BB | BD | BE | BF | BG | BH | BI | BJ | BL | BM | BN | BO | BQ | BR | BS | BT | BV | BW | BY | BZ | CA | CC | CD | CF | CG | CH | CI | CK | CL | CM | CN | CO | CR | CU | CV | CW | CX | CY | CZ | DE | DJ | DK | DM | DO | DZ | EC | EE | EG | EH | ER | ES | ET | FI | FJ | FK | FM | FO | FR | GA | GB | GD | GE | GF | GG | GH | GI | GL | GM | GN | GP | GQ | GR | GS | GT | GU | GW | GY | HK | HM | HN | HR | HT | HU | ID | IE | IL | IM | IN | IO | IQ | IR | IS | IT | JE | JM | JO | JP | KE | KG | KH | KI | KM | KN | KP | KR | KW | KY | KZ | LA | LB | LC | LI | LK | LR | LS | LT | LU | LV | LY | MA | MC | MD | ME | MF | MG | MH | MK | ML | MM | MN | MO | MP | MQ | MR | MS | MT | MU | MV | MW | MX | MY | MZ | NA | NC | NE | NF | NG | NI | NL | NO | NP | NR | NU | NZ | OM | PA | PE | PF | PG | PH | PK | PL | PM | PN | PR | PS | PT | PW | PY | QA | RE | RO | RS | RU | RW | SA | SB | SC | SD | SE | SG | SH | SI | SJ | SK | SL | SM | SN | SO | SR | SS | ST | SV | SX | SY | SZ | TC | TD | TF | TG | TH | TJ | TK | TL | TM | TN | TO | TR | TT | TV | TW | TZ | UA | UG | UM | US | UY | UZ | VA | VC | VE | VG | VI | VN | VU | WF | WS | YE | YT | ZA | ZM | ZW
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}
