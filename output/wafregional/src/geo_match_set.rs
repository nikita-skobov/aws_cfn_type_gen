/// Contains one or more countries that AWS WAF will search for.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnGeoMatchSet {
    ///
    /// An array of GeoMatchConstraint objects, which contain the country that you want AWS WAF to search for.
    ///
    /// Required: No
    ///
    /// Type: List of GeoMatchConstraint
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeoMatchConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_constraints: Option<Vec<GeoMatchConstraint>>,

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
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnGeoMatchSet {
    fn type_string(&self) -> &'static str {
        "AWS::WAFRegional::GeoMatchSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
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

/// The country from which web requests originate that you want AWS WAF to search for.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub cfn_type: GeoMatchConstraintTypeEnum,

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
    pub value: GeoMatchConstraintValueEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum GeoMatchConstraintTypeEnum {
    /// Country
    #[serde(rename = "Country")]
    Country,
}

impl Default for GeoMatchConstraintTypeEnum {
    fn default() -> Self {
        GeoMatchConstraintTypeEnum::Country
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum GeoMatchConstraintValueEnum {
    /// AD
    #[serde(rename = "AD")]
    Ad,

    /// AE
    #[serde(rename = "AE")]
    Ae,

    /// AF
    #[serde(rename = "AF")]
    Af,

    /// AG
    #[serde(rename = "AG")]
    Ag,

    /// AI
    #[serde(rename = "AI")]
    Ai,

    /// AL
    #[serde(rename = "AL")]
    Al,

    /// AM
    #[serde(rename = "AM")]
    Am,

    /// AO
    #[serde(rename = "AO")]
    Ao,

    /// AQ
    #[serde(rename = "AQ")]
    Aq,

    /// AR
    #[serde(rename = "AR")]
    Ar,

    /// AS
    #[serde(rename = "AS")]
    As,

    /// AT
    #[serde(rename = "AT")]
    At,

    /// AU
    #[serde(rename = "AU")]
    Au,

    /// AW
    #[serde(rename = "AW")]
    Aw,

    /// AX
    #[serde(rename = "AX")]
    Ax,

    /// AZ
    #[serde(rename = "AZ")]
    Az,

    /// BA
    #[serde(rename = "BA")]
    Ba,

    /// BB
    #[serde(rename = "BB")]
    Bb,

    /// BD
    #[serde(rename = "BD")]
    Bd,

    /// BE
    #[serde(rename = "BE")]
    Be,

    /// BF
    #[serde(rename = "BF")]
    Bf,

    /// BG
    #[serde(rename = "BG")]
    Bg,

    /// BH
    #[serde(rename = "BH")]
    Bh,

    /// BI
    #[serde(rename = "BI")]
    Bi,

    /// BJ
    #[serde(rename = "BJ")]
    Bj,

    /// BL
    #[serde(rename = "BL")]
    Bl,

    /// BM
    #[serde(rename = "BM")]
    Bm,

    /// BN
    #[serde(rename = "BN")]
    Bn,

    /// BO
    #[serde(rename = "BO")]
    Bo,

    /// BQ
    #[serde(rename = "BQ")]
    Bq,

    /// BR
    #[serde(rename = "BR")]
    Br,

    /// BS
    #[serde(rename = "BS")]
    Bs,

    /// BT
    #[serde(rename = "BT")]
    Bt,

    /// BV
    #[serde(rename = "BV")]
    Bv,

    /// BW
    #[serde(rename = "BW")]
    Bw,

    /// BY
    #[serde(rename = "BY")]
    By,

    /// BZ
    #[serde(rename = "BZ")]
    Bz,

    /// CA
    #[serde(rename = "CA")]
    Ca,

    /// CC
    #[serde(rename = "CC")]
    Cc,

    /// CD
    #[serde(rename = "CD")]
    Cd,

    /// CF
    #[serde(rename = "CF")]
    Cf,

    /// CG
    #[serde(rename = "CG")]
    Cg,

    /// CH
    #[serde(rename = "CH")]
    Ch,

    /// CI
    #[serde(rename = "CI")]
    Ci,

    /// CK
    #[serde(rename = "CK")]
    Ck,

    /// CL
    #[serde(rename = "CL")]
    Cl,

    /// CM
    #[serde(rename = "CM")]
    Cm,

    /// CN
    #[serde(rename = "CN")]
    Cn,

    /// CO
    #[serde(rename = "CO")]
    Co,

    /// CR
    #[serde(rename = "CR")]
    Cr,

    /// CU
    #[serde(rename = "CU")]
    Cu,

    /// CV
    #[serde(rename = "CV")]
    Cv,

    /// CW
    #[serde(rename = "CW")]
    Cw,

    /// CX
    #[serde(rename = "CX")]
    Cx,

    /// CY
    #[serde(rename = "CY")]
    Cy,

    /// CZ
    #[serde(rename = "CZ")]
    Cz,

    /// DE
    #[serde(rename = "DE")]
    De,

    /// DJ
    #[serde(rename = "DJ")]
    Dj,

    /// DK
    #[serde(rename = "DK")]
    Dk,

    /// DM
    #[serde(rename = "DM")]
    Dm,

    /// DO
    #[serde(rename = "DO")]
    Do,

    /// DZ
    #[serde(rename = "DZ")]
    Dz,

    /// EC
    #[serde(rename = "EC")]
    Ec,

    /// EE
    #[serde(rename = "EE")]
    Ee,

    /// EG
    #[serde(rename = "EG")]
    Eg,

    /// EH
    #[serde(rename = "EH")]
    Eh,

    /// ER
    #[serde(rename = "ER")]
    Er,

    /// ES
    #[serde(rename = "ES")]
    Es,

    /// ET
    #[serde(rename = "ET")]
    Et,

    /// FI
    #[serde(rename = "FI")]
    Fi,

    /// FJ
    #[serde(rename = "FJ")]
    Fj,

    /// FK
    #[serde(rename = "FK")]
    Fk,

    /// FM
    #[serde(rename = "FM")]
    Fm,

    /// FO
    #[serde(rename = "FO")]
    Fo,

    /// FR
    #[serde(rename = "FR")]
    Fr,

    /// GA
    #[serde(rename = "GA")]
    Ga,

    /// GB
    #[serde(rename = "GB")]
    Gb,

    /// GD
    #[serde(rename = "GD")]
    Gd,

    /// GE
    #[serde(rename = "GE")]
    Ge,

    /// GF
    #[serde(rename = "GF")]
    Gf,

    /// GG
    #[serde(rename = "GG")]
    Gg,

    /// GH
    #[serde(rename = "GH")]
    Gh,

    /// GI
    #[serde(rename = "GI")]
    Gi,

    /// GL
    #[serde(rename = "GL")]
    Gl,

    /// GM
    #[serde(rename = "GM")]
    Gm,

    /// GN
    #[serde(rename = "GN")]
    Gn,

    /// GP
    #[serde(rename = "GP")]
    Gp,

    /// GQ
    #[serde(rename = "GQ")]
    Gq,

    /// GR
    #[serde(rename = "GR")]
    Gr,

    /// GS
    #[serde(rename = "GS")]
    Gs,

    /// GT
    #[serde(rename = "GT")]
    Gt,

    /// GU
    #[serde(rename = "GU")]
    Gu,

    /// GW
    #[serde(rename = "GW")]
    Gw,

    /// GY
    #[serde(rename = "GY")]
    Gy,

    /// HK
    #[serde(rename = "HK")]
    Hk,

    /// HM
    #[serde(rename = "HM")]
    Hm,

    /// HN
    #[serde(rename = "HN")]
    Hn,

    /// HR
    #[serde(rename = "HR")]
    Hr,

    /// HT
    #[serde(rename = "HT")]
    Ht,

    /// HU
    #[serde(rename = "HU")]
    Hu,

    /// ID
    #[serde(rename = "ID")]
    Id,

    /// IE
    #[serde(rename = "IE")]
    Ie,

    /// IL
    #[serde(rename = "IL")]
    Il,

    /// IM
    #[serde(rename = "IM")]
    Im,

    /// IN
    #[serde(rename = "IN")]
    In,

    /// IO
    #[serde(rename = "IO")]
    Io,

    /// IQ
    #[serde(rename = "IQ")]
    Iq,

    /// IR
    #[serde(rename = "IR")]
    Ir,

    /// IS
    #[serde(rename = "IS")]
    Is,

    /// IT
    #[serde(rename = "IT")]
    It,

    /// JE
    #[serde(rename = "JE")]
    Je,

    /// JM
    #[serde(rename = "JM")]
    Jm,

    /// JO
    #[serde(rename = "JO")]
    Jo,

    /// JP
    #[serde(rename = "JP")]
    Jp,

    /// KE
    #[serde(rename = "KE")]
    Ke,

    /// KG
    #[serde(rename = "KG")]
    Kg,

    /// KH
    #[serde(rename = "KH")]
    Kh,

    /// KI
    #[serde(rename = "KI")]
    Ki,

    /// KM
    #[serde(rename = "KM")]
    Km,

    /// KN
    #[serde(rename = "KN")]
    Kn,

    /// KP
    #[serde(rename = "KP")]
    Kp,

    /// KR
    #[serde(rename = "KR")]
    Kr,

    /// KW
    #[serde(rename = "KW")]
    Kw,

    /// KY
    #[serde(rename = "KY")]
    Ky,

    /// KZ
    #[serde(rename = "KZ")]
    Kz,

    /// LA
    #[serde(rename = "LA")]
    La,

    /// LB
    #[serde(rename = "LB")]
    Lb,

    /// LC
    #[serde(rename = "LC")]
    Lc,

    /// LI
    #[serde(rename = "LI")]
    Li,

    /// LK
    #[serde(rename = "LK")]
    Lk,

    /// LR
    #[serde(rename = "LR")]
    Lr,

    /// LS
    #[serde(rename = "LS")]
    Ls,

    /// LT
    #[serde(rename = "LT")]
    Lt,

    /// LU
    #[serde(rename = "LU")]
    Lu,

    /// LV
    #[serde(rename = "LV")]
    Lv,

    /// LY
    #[serde(rename = "LY")]
    Ly,

    /// MA
    #[serde(rename = "MA")]
    Ma,

    /// MC
    #[serde(rename = "MC")]
    Mc,

    /// MD
    #[serde(rename = "MD")]
    Md,

    /// ME
    #[serde(rename = "ME")]
    Me,

    /// MF
    #[serde(rename = "MF")]
    Mf,

    /// MG
    #[serde(rename = "MG")]
    Mg,

    /// MH
    #[serde(rename = "MH")]
    Mh,

    /// MK
    #[serde(rename = "MK")]
    Mk,

    /// ML
    #[serde(rename = "ML")]
    Ml,

    /// MM
    #[serde(rename = "MM")]
    Mm,

    /// MN
    #[serde(rename = "MN")]
    Mn,

    /// MO
    #[serde(rename = "MO")]
    Mo,

    /// MP
    #[serde(rename = "MP")]
    Mp,

    /// MQ
    #[serde(rename = "MQ")]
    Mq,

    /// MR
    #[serde(rename = "MR")]
    Mr,

    /// MS
    #[serde(rename = "MS")]
    Ms,

    /// MT
    #[serde(rename = "MT")]
    Mt,

    /// MU
    #[serde(rename = "MU")]
    Mu,

    /// MV
    #[serde(rename = "MV")]
    Mv,

    /// MW
    #[serde(rename = "MW")]
    Mw,

    /// MX
    #[serde(rename = "MX")]
    Mx,

    /// MY
    #[serde(rename = "MY")]
    My,

    /// MZ
    #[serde(rename = "MZ")]
    Mz,

    /// NA
    #[serde(rename = "NA")]
    Na,

    /// NC
    #[serde(rename = "NC")]
    Nc,

    /// NE
    #[serde(rename = "NE")]
    Ne,

    /// NF
    #[serde(rename = "NF")]
    Nf,

    /// NG
    #[serde(rename = "NG")]
    Ng,

    /// NI
    #[serde(rename = "NI")]
    Ni,

    /// NL
    #[serde(rename = "NL")]
    Nl,

    /// NO
    #[serde(rename = "NO")]
    No,

    /// NP
    #[serde(rename = "NP")]
    Np,

    /// NR
    #[serde(rename = "NR")]
    Nr,

    /// NU
    #[serde(rename = "NU")]
    Nu,

    /// NZ
    #[serde(rename = "NZ")]
    Nz,

    /// OM
    #[serde(rename = "OM")]
    Om,

    /// PA
    #[serde(rename = "PA")]
    Pa,

    /// PE
    #[serde(rename = "PE")]
    Pe,

    /// PF
    #[serde(rename = "PF")]
    Pf,

    /// PG
    #[serde(rename = "PG")]
    Pg,

    /// PH
    #[serde(rename = "PH")]
    Ph,

    /// PK
    #[serde(rename = "PK")]
    Pk,

    /// PL
    #[serde(rename = "PL")]
    Pl,

    /// PM
    #[serde(rename = "PM")]
    Pm,

    /// PN
    #[serde(rename = "PN")]
    Pn,

    /// PR
    #[serde(rename = "PR")]
    Pr,

    /// PS
    #[serde(rename = "PS")]
    Ps,

    /// PT
    #[serde(rename = "PT")]
    Pt,

    /// PW
    #[serde(rename = "PW")]
    Pw,

    /// PY
    #[serde(rename = "PY")]
    Py,

    /// QA
    #[serde(rename = "QA")]
    Qa,

    /// RE
    #[serde(rename = "RE")]
    Re,

    /// RO
    #[serde(rename = "RO")]
    Ro,

    /// RS
    #[serde(rename = "RS")]
    Rs,

    /// RU
    #[serde(rename = "RU")]
    Ru,

    /// RW
    #[serde(rename = "RW")]
    Rw,

    /// SA
    #[serde(rename = "SA")]
    Sa,

    /// SB
    #[serde(rename = "SB")]
    Sb,

    /// SC
    #[serde(rename = "SC")]
    Sc,

    /// SD
    #[serde(rename = "SD")]
    Sd,

    /// SE
    #[serde(rename = "SE")]
    Se,

    /// SG
    #[serde(rename = "SG")]
    Sg,

    /// SH
    #[serde(rename = "SH")]
    Sh,

    /// SI
    #[serde(rename = "SI")]
    Si,

    /// SJ
    #[serde(rename = "SJ")]
    Sj,

    /// SK
    #[serde(rename = "SK")]
    Sk,

    /// SL
    #[serde(rename = "SL")]
    Sl,

    /// SM
    #[serde(rename = "SM")]
    Sm,

    /// SN
    #[serde(rename = "SN")]
    Sn,

    /// SO
    #[serde(rename = "SO")]
    So,

    /// SR
    #[serde(rename = "SR")]
    Sr,

    /// SS
    #[serde(rename = "SS")]
    Ss,

    /// ST
    #[serde(rename = "ST")]
    St,

    /// SV
    #[serde(rename = "SV")]
    Sv,

    /// SX
    #[serde(rename = "SX")]
    Sx,

    /// SY
    #[serde(rename = "SY")]
    Sy,

    /// SZ
    #[serde(rename = "SZ")]
    Sz,

    /// TC
    #[serde(rename = "TC")]
    Tc,

    /// TD
    #[serde(rename = "TD")]
    Td,

    /// TF
    #[serde(rename = "TF")]
    Tf,

    /// TG
    #[serde(rename = "TG")]
    Tg,

    /// TH
    #[serde(rename = "TH")]
    Th,

    /// TJ
    #[serde(rename = "TJ")]
    Tj,

    /// TK
    #[serde(rename = "TK")]
    Tk,

    /// TL
    #[serde(rename = "TL")]
    Tl,

    /// TM
    #[serde(rename = "TM")]
    Tm,

    /// TN
    #[serde(rename = "TN")]
    Tn,

    /// TO
    #[serde(rename = "TO")]
    To,

    /// TR
    #[serde(rename = "TR")]
    Tr,

    /// TT
    #[serde(rename = "TT")]
    Tt,

    /// TV
    #[serde(rename = "TV")]
    Tv,

    /// TW
    #[serde(rename = "TW")]
    Tw,

    /// TZ
    #[serde(rename = "TZ")]
    Tz,

    /// UA
    #[serde(rename = "UA")]
    Ua,

    /// UG
    #[serde(rename = "UG")]
    Ug,

    /// UM
    #[serde(rename = "UM")]
    Um,

    /// US
    #[serde(rename = "US")]
    Us,

    /// UY
    #[serde(rename = "UY")]
    Uy,

    /// UZ
    #[serde(rename = "UZ")]
    Uz,

    /// VA
    #[serde(rename = "VA")]
    Va,

    /// VC
    #[serde(rename = "VC")]
    Vc,

    /// VE
    #[serde(rename = "VE")]
    Ve,

    /// VG
    #[serde(rename = "VG")]
    Vg,

    /// VI
    #[serde(rename = "VI")]
    Vi,

    /// VN
    #[serde(rename = "VN")]
    Vn,

    /// VU
    #[serde(rename = "VU")]
    Vu,

    /// WF
    #[serde(rename = "WF")]
    Wf,

    /// WS
    #[serde(rename = "WS")]
    Ws,

    /// YE
    #[serde(rename = "YE")]
    Ye,

    /// YT
    #[serde(rename = "YT")]
    Yt,

    /// ZA
    #[serde(rename = "ZA")]
    Za,

    /// ZM
    #[serde(rename = "ZM")]
    Zm,

    /// ZW
    #[serde(rename = "ZW")]
    Zw,
}

impl Default for GeoMatchConstraintValueEnum {
    fn default() -> Self {
        GeoMatchConstraintValueEnum::Ad
    }
}

impl cfn_resources::CfnResource for GeoMatchConstraint {
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
