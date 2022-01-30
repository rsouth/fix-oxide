use std::fmt::Formatter;
use std::str::FromStr;
use itertools::Itertools;

use rust_decimal::Decimal;

use crate::model::field::{FieldSet, FieldTypeMismatchError};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Field {
    String(u16, String),
    Char(u16, char),
    Decimal(u16, Decimal),
    Int(u16, i32),
}

impl Field {

    ///
    /// # Errors
    ///
    pub fn as_str_safe(&self) -> Result<&str, FieldTypeMismatchError> {
        match self {
            Field::String(_, v) => Ok(v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    ///
    /// # Errors
    ///
    /// # Panics
    ///
    pub fn as_str(&self) -> &str {
        match self {
            Field::String(_, v) => v,
            _ => panic!(),
        }
    }

    ///
    /// # Errors
    ///
    pub const fn as_char_safe(&self) -> Result<char, FieldTypeMismatchError> {
        match self {
            Field::Char(_, v) => Ok(*v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    ///
    /// # Errors
    ///
    /// # Panics
    ///
    pub fn as_char(&self) -> char {
        match self {
            Field::Char(_, v) => *v,
            _ => panic!(),
        }
    }

    ///
    /// # Errors
    ///
    pub const fn as_decimal_safe(&self) -> Result<Decimal, FieldTypeMismatchError> {
        match self {
            Field::Decimal(_, v) => Ok(*v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    ///
    /// # Errors
    ///
    /// # Panics
    ///
    pub fn as_decimal(&self) -> Decimal {
        match self {
            Field::Decimal(_, v) => *v,
            _ => panic!(),
        }
    }

    ///
    /// # Errors
    ///
    pub const fn as_i32_safe(&self) -> Result<i32, FieldTypeMismatchError> {
        match self {
            Field::Int(_, v) => Ok(*v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    ///
    /// # Errors
    ///
    /// # Panics
    ///
    pub fn as_i32(&self) -> i32 {
        match self {
            Field::Int(_, v) => *v,
            _ => panic!(),
        }
    }

    #[must_use]
    pub const fn tag(&self) -> u16 {
        match self {
            Field::String(t, _) 
            | Field::Char(t, _) 
            | Field::Decimal(t, _) 
            | Field::Int(t, _) 
            => *t,
    }
}

    #[must_use]
    pub fn to_delimited_string(&self, separator: char) -> String {
        match self {
            // &str
            Field::String(t, v) => format!("{}={}{}", t, v, separator),
            // char
            Field::Char(t, v) => format!("{}={}{}", t, v, separator),
            // Decimal
            Field::Decimal(t, v) => format!("{}={}{}", t, v, separator),
            // i32
            Field::Int(t, v) => format!("{}={}{}", t, v, separator),
        }
    }
}
// parse string (35=D) into Field{35, "D"}
impl TryFrom<String> for Field {
    type Error = (); // todo error type...
      
    fn try_from(s: String) -> Result<Self, Self::Error> {
        println!("From<String> for Field: {}", &s);
        let two_parts = s.split_once('=');
        match two_parts {
            Some((s_tag, s_value)) => {
                println!("parsing tag: {}, field: {} into Field", s_tag, s_value);
                  
                // figure out the tag
                let tag: u16 = s_tag.parse::<u16>().unwrap();
                   
                // build field using the tag & value
                match tag {
                    1 | 2 | 3 | 5 | 8 | 10 | 11 | 15 | 17 | 19 | 22 | 23 | 26 | 27 | 35 | 37 | 41 | 46 | 48 | 49 | 50 | 55 | 56 | 57 | 58 | 65 | 66 | 69 | 70 | 72 | 76 | 79 | 86 | 92 | 105 | 106 | 107 | 109 | 112 | 115 | 116 | 117 | 120 | 128 | 129 | 131 | 138 | 142 | 143 | 144 | 145 | 147 | 148 | 149 | 161 | 162 | 164 | 166 | 167 | 170 | 171 | 173 | 174 | 175 | 176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 196 | 198 | 214 | 217 | 262 | 278 | 280 | 282 | 283 | 284 | 288 | 289 | 299 | 302 | 305 | 306 | 307 | 309 | 310 | 311 | 312 | 318 | 320 | 322 | 324 | 335 | 336 | 337 | 347 | 372 | 375 | 376 | 379 | 390 | 391 | 392 | 400 | 421 | 439 | 440 | 444 => Ok(Self::String(tag, s_value.to_string())),

                    4 | 13 | 20 | 21 | 24 | 25 | 28 | 29 | 39 | 40 | 47 | 54 | 59 | 61 | 63 | 71 | 77 | 81 | 94 | 104 | 125 | 127 | 139 | 150 | 156 | 160 | 163 | 165 | 206 | 219 | 263 | 269 | 274 | 279 | 281 | 285 | 286 | 291 | 292 | 317 | 327 | 374 | 385 | 388 | 418 | 419 | 433 | 434 | 442 => Ok(Self::Char(tag, str::parse::<char>(s_value).unwrap())),

                    6 | 31 | 44 | 99 | 132 | 133 | 140 | 153 | 188 | 190 | 194 | 202 | 270 | 316 | 332 | 333 | 366 | 426 => Ok(Self::Decimal(tag, Decimal::from_str(s_value).unwrap())),

                    7 | 9 | 12 | 14 | 16 | 32 | 33 | 34 | 36 | 38 | 45 | 53 | 67 | 68 | 73 | 74 | 78 | 80 | 82 | 83 | 84 | 85 | 87 | 88 | 98 | 102 | 103 | 108 | 110 | 111 | 118 | 119 | 124 | 134 | 135 | 136 | 137 | 146 | 151 | 152 | 154 | 157 | 159 | 169 | 172 | 192 | 197 | 199 | 201 | 203 | 204 | 209 | 210 | 215 | 216 | 264 | 265 | 267 | 268 | 271 | 287 | 290 | 293 | 294 | 295 | 296 | 297 | 298 | 300 | 301 | 303 | 304 | 315 | 319 | 321 | 323 | 326 | 330 | 331 | 334 | 338 | 339 | 340 | 346 | 368 | 369 | 371 | 373 | 378 | 380 | 381 | 382 | 383 | 384 | 386 | 387 | 393 | 394 | 395 | 396 | 397 | 398 | 399 | 401 | 404 | 406 | 408 | 409 | 412 | 414 | 415 | 416 | 417 | 420 | 422 | 423 | 424 | 425 | 427 | 428 | 429 | 430 | 431 | 437 | 441 => Ok(Self::Int(tag, str::parse::<i32>(s_value).unwrap())),

                    _ => Err(()),
                }
            }
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccountField {
    pub fd: Field,
}

impl AccountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AccountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvIdField {
    pub fd: Field,
}

impl AdvIdField {
    #[must_use]
    pub const fn tag() -> u16 {
        2
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdvIdField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvRefIDField {
    pub fd: Field,
}

impl AdvRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        3
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdvRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvSideField {
    pub fd: Field,
}

impl AdvSideField {
    #[must_use]
    pub const fn tag() -> u16 {
        4
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdvSideField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvTransTypeField {
    pub fd: Field,
}

impl AdvTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        5
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdvTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AvgPxField {
    pub fd: Field,
}

impl AvgPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        6
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AvgPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BeginSeqNoField {
    pub fd: Field,
}

impl BeginSeqNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        7
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BeginSeqNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BeginStringField {
    pub fd: Field,
}

impl BeginStringField {
    #[must_use]
    pub const fn tag() -> u16 {
        8
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BeginStringField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BodyLengthField {
    pub fd: Field,
}

impl BodyLengthField {
    #[must_use]
    pub const fn tag() -> u16 {
        9
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BodyLengthField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CheckSumField {
    pub fd: Field,
}

impl CheckSumField {
    #[must_use]
    pub const fn tag() -> u16 {
        10
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CheckSumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClOrdIDField {
    pub fd: Field,
}

impl ClOrdIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        11
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClOrdIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommissionField {
    pub fd: Field,
}

impl CommissionField {
    #[must_use]
    pub const fn tag() -> u16 {
        12
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CommissionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommTypeField {
    pub fd: Field,
}

impl CommTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        13
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CommTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CumQtyField {
    pub fd: Field,
}

impl CumQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        14
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CumQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CurrencyField {
    pub fd: Field,
}

impl CurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        15
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EndSeqNoField {
    pub fd: Field,
}

impl EndSeqNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        16
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EndSeqNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecIDField {
    pub fd: Field,
}

impl ExecIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        17
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecRefIDField {
    pub fd: Field,
}

impl ExecRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        19
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecTransTypeField {
    pub fd: Field,
}

impl ExecTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        20
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HandlInstField {
    pub fd: Field,
}

impl HandlInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        21
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HandlInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IDSourceField {
    pub fd: Field,
}

impl IDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        22
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIidField {
    pub fd: Field,
}

impl IOIidField {
    #[must_use]
    pub const fn tag() -> u16 {
        23
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIidField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIOthSvcField {
    pub fd: Field,
}

impl IOIOthSvcField {
    #[must_use]
    pub const fn tag() -> u16 {
        24
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIOthSvcField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIQltyIndField {
    pub fd: Field,
}

impl IOIQltyIndField {
    #[must_use]
    pub const fn tag() -> u16 {
        25
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIQltyIndField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIRefIDField {
    pub fd: Field,
}

impl IOIRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        26
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOISharesField {
    pub fd: Field,
}

impl IOISharesField {
    #[must_use]
    pub const fn tag() -> u16 {
        27
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOISharesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOITransTypeField {
    pub fd: Field,
}

impl IOITransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        28
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOITransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastCapacityField {
    pub fd: Field,
}

impl LastCapacityField {
    #[must_use]
    pub const fn tag() -> u16 {
        29
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastCapacityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastPxField {
    pub fd: Field,
}

impl LastPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        31
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastSharesField {
    pub fd: Field,
}

impl LastSharesField {
    #[must_use]
    pub const fn tag() -> u16 {
        32
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastSharesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinesOfTextField {
    pub fd: Field,
}

impl LinesOfTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        33
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LinesOfTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MsgSeqNumField {
    pub fd: Field,
}

impl MsgSeqNumField {
    #[must_use]
    pub const fn tag() -> u16 {
        34
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MsgSeqNumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MsgTypeField {
    pub fd: Field,
}

impl MsgTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        35
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MsgTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NewSeqNoField {
    pub fd: Field,
}

impl NewSeqNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        36
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NewSeqNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrderIDField {
    pub fd: Field,
}

impl OrderIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        37
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrderQtyField {
    pub fd: Field,
}

impl OrderQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        38
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrdStatusField {
    pub fd: Field,
}

impl OrdStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        39
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrdStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrdTypeField {
    pub fd: Field,
}

impl OrdTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        40
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrdTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrigClOrdIDField {
    pub fd: Field,
}

impl OrigClOrdIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        41
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrigClOrdIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PriceField {
    pub fd: Field,
}

impl PriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        44
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RefSeqNumField {
    pub fd: Field,
}

impl RefSeqNumField {
    #[must_use]
    pub const fn tag() -> u16 {
        45
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefSeqNumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RelatdSymField {
    pub fd: Field,
}

impl RelatdSymField {
    #[must_use]
    pub const fn tag() -> u16 {
        46
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RelatdSymField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rule80AField {
    pub fd: Field,
}

impl Rule80AField {
    #[must_use]
    pub const fn tag() -> u16 {
        47
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Rule80AField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityIDField {
    pub fd: Field,
}

impl SecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        48
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SenderCompIDField {
    pub fd: Field,
}

impl SenderCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        49
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SenderCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SenderSubIDField {
    pub fd: Field,
}

impl SenderSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        50
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SenderSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SharesField {
    pub fd: Field,
}

impl SharesField {
    #[must_use]
    pub const fn tag() -> u16 {
        53
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SharesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SideField {
    pub fd: Field,
}

impl SideField {
    #[must_use]
    pub const fn tag() -> u16 {
        54
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolField {
    pub fd: Field,
}

impl SymbolField {
    #[must_use]
    pub const fn tag() -> u16 {
        55
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SymbolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetCompIDField {
    pub fd: Field,
}

impl TargetCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        56
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetSubIDField {
    pub fd: Field,
}

impl TargetSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        57
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextField {
    pub fd: Field,
}

impl TextField {
    #[must_use]
    pub const fn tag() -> u16 {
        58
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeInForceField {
    pub fd: Field,
}

impl TimeInForceField {
    #[must_use]
    pub const fn tag() -> u16 {
        59
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TimeInForceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UrgencyField {
    pub fd: Field,
}

impl UrgencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        61
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UrgencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlmntTypField {
    pub fd: Field,
}

impl SettlmntTypField {
    #[must_use]
    pub const fn tag() -> u16 {
        63
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlmntTypField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolSfxField {
    pub fd: Field,
}

impl SymbolSfxField {
    #[must_use]
    pub const fn tag() -> u16 {
        65
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SymbolSfxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListIDField {
    pub fd: Field,
}

impl ListIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        66
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListSeqNoField {
    pub fd: Field,
}

impl ListSeqNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        67
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListSeqNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotNoOrdersField {
    pub fd: Field,
}

impl TotNoOrdersField {
    #[must_use]
    pub const fn tag() -> u16 {
        68
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNoOrdersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListExecInstField {
    pub fd: Field,
}

impl ListExecInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        69
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListExecInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocIDField {
    pub fd: Field,
}

impl AllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        70
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocTransTypeField {
    pub fd: Field,
}

impl AllocTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        71
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RefAllocIDField {
    pub fd: Field,
}

impl RefAllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        72
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefAllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoOrdersField {
    pub fd: Field,
}

impl NoOrdersField {
    #[must_use]
    pub const fn tag() -> u16 {
        73
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoOrdersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AvgPrxPrecisionField {
    pub fd: Field,
}

impl AvgPrxPrecisionField {
    #[must_use]
    pub const fn tag() -> u16 {
        74
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AvgPrxPrecisionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecBrokerField {
    pub fd: Field,
}

impl ExecBrokerField {
    #[must_use]
    pub const fn tag() -> u16 {
        76
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecBrokerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenCloseField {
    pub fd: Field,
}

impl OpenCloseField {
    #[must_use]
    pub const fn tag() -> u16 {
        77
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OpenCloseField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoAllocsField {
    pub fd: Field,
}

impl NoAllocsField {
    #[must_use]
    pub const fn tag() -> u16 {
        78
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoAllocsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocAccountField {
    pub fd: Field,
}

impl AllocAccountField {
    #[must_use]
    pub const fn tag() -> u16 {
        79
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocAccountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocSharesField {
    pub fd: Field,
}

impl AllocSharesField {
    #[must_use]
    pub const fn tag() -> u16 {
        80
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocSharesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessCodeField {
    pub fd: Field,
}

impl ProcessCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        81
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ProcessCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoRptsField {
    pub fd: Field,
}

impl NoRptsField {
    #[must_use]
    pub const fn tag() -> u16 {
        82
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoRptsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RptSeqField {
    pub fd: Field,
}

impl RptSeqField {
    #[must_use]
    pub const fn tag() -> u16 {
        83
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RptSeqField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CxlQtyField {
    pub fd: Field,
}

impl CxlQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        84
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CxlQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoDlvyInstField {
    pub fd: Field,
}

impl NoDlvyInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        85
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoDlvyInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DlvyInstField {
    pub fd: Field,
}

impl DlvyInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        86
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DlvyInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocStatusField {
    pub fd: Field,
}

impl AllocStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        87
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocRejCodeField {
    pub fd: Field,
}

impl AllocRejCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        88
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocRejCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BrokerOfCreditField {
    pub fd: Field,
}

impl BrokerOfCreditField {
    #[must_use]
    pub const fn tag() -> u16 {
        92
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BrokerOfCreditField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmailTypeField {
    pub fd: Field,
}

impl EmailTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        94
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EmailTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EncryptMethodField {
    pub fd: Field,
}

impl EncryptMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        98
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EncryptMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StopPxField {
    pub fd: Field,
}

impl StopPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        99
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StopPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CxlRejReasonField {
    pub fd: Field,
}

impl CxlRejReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        102
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CxlRejReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrdRejReasonField {
    pub fd: Field,
}

impl OrdRejReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        103
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrdRejReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIQualifierField {
    pub fd: Field,
}

impl IOIQualifierField {
    #[must_use]
    pub const fn tag() -> u16 {
        104
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIQualifierField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WaveNoField {
    pub fd: Field,
}

impl WaveNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        105
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for WaveNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IssuerField {
    pub fd: Field,
}

impl IssuerField {
    #[must_use]
    pub const fn tag() -> u16 {
        106
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IssuerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityDescField {
    pub fd: Field,
}

impl SecurityDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        107
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HeartBtIntField {
    pub fd: Field,
}

impl HeartBtIntField {
    #[must_use]
    pub const fn tag() -> u16 {
        108
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HeartBtIntField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClientIDField {
    pub fd: Field,
}

impl ClientIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        109
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClientIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinQtyField {
    pub fd: Field,
}

impl MinQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        110
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MinQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MaxFloorField {
    pub fd: Field,
}

impl MaxFloorField {
    #[must_use]
    pub const fn tag() -> u16 {
        111
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaxFloorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestReqIDField {
    pub fd: Field,
}

impl TestReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        112
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TestReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OnBehalfOfCompIDField {
    pub fd: Field,
}

impl OnBehalfOfCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        115
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OnBehalfOfCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OnBehalfOfSubIDField {
    pub fd: Field,
}

impl OnBehalfOfSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        116
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OnBehalfOfSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteIDField {
    pub fd: Field,
}

impl QuoteIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        117
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NetMoneyField {
    pub fd: Field,
}

impl NetMoneyField {
    #[must_use]
    pub const fn tag() -> u16 {
        118
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NetMoneyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlCurrAmtField {
    pub fd: Field,
}

impl SettlCurrAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        119
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlCurrAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlCurrencyField {
    pub fd: Field,
}

impl SettlCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        120
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoExecsField {
    pub fd: Field,
}

impl NoExecsField {
    #[must_use]
    pub const fn tag() -> u16 {
        124
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoExecsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CxlTypeField {
    pub fd: Field,
}

impl CxlTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        125
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CxlTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DKReasonField {
    pub fd: Field,
}

impl DKReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        127
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DKReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeliverToCompIDField {
    pub fd: Field,
}

impl DeliverToCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        128
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeliverToCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeliverToSubIDField {
    pub fd: Field,
}

impl DeliverToSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        129
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeliverToSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteReqIDField {
    pub fd: Field,
}

impl QuoteReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        131
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidPxField {
    pub fd: Field,
}

impl BidPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        132
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OfferPxField {
    pub fd: Field,
}

impl OfferPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        133
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OfferPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidSizeField {
    pub fd: Field,
}

impl BidSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        134
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OfferSizeField {
    pub fd: Field,
}

impl OfferSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        135
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OfferSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoMiscFeesField {
    pub fd: Field,
}

impl NoMiscFeesField {
    #[must_use]
    pub const fn tag() -> u16 {
        136
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoMiscFeesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MiscFeeAmtField {
    pub fd: Field,
}

impl MiscFeeAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        137
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MiscFeeAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MiscFeeCurrField {
    pub fd: Field,
}

impl MiscFeeCurrField {
    #[must_use]
    pub const fn tag() -> u16 {
        138
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MiscFeeCurrField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MiscFeeTypeField {
    pub fd: Field,
}

impl MiscFeeTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        139
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MiscFeeTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrevClosePxField {
    pub fd: Field,
}

impl PrevClosePxField {
    #[must_use]
    pub const fn tag() -> u16 {
        140
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PrevClosePxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SenderLocationIDField {
    pub fd: Field,
}

impl SenderLocationIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        142
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SenderLocationIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetLocationIDField {
    pub fd: Field,
}

impl TargetLocationIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        143
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetLocationIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OnBehalfOfLocationIDField {
    pub fd: Field,
}

impl OnBehalfOfLocationIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        144
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OnBehalfOfLocationIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeliverToLocationIDField {
    pub fd: Field,
}

impl DeliverToLocationIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        145
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeliverToLocationIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoRelatedSymField {
    pub fd: Field,
}

impl NoRelatedSymField {
    #[must_use]
    pub const fn tag() -> u16 {
        146
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoRelatedSymField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubjectField {
    pub fd: Field,
}

impl SubjectField {
    #[must_use]
    pub const fn tag() -> u16 {
        147
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SubjectField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HeadlineField {
    pub fd: Field,
}

impl HeadlineField {
    #[must_use]
    pub const fn tag() -> u16 {
        148
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HeadlineField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct URLLinkField {
    pub fd: Field,
}

impl URLLinkField {
    #[must_use]
    pub const fn tag() -> u16 {
        149
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for URLLinkField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecTypeField {
    pub fd: Field,
}

impl ExecTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        150
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LeavesQtyField {
    pub fd: Field,
}

impl LeavesQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        151
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LeavesQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashOrderQtyField {
    pub fd: Field,
}

impl CashOrderQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        152
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashOrderQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocAvgPxField {
    pub fd: Field,
}

impl AllocAvgPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        153
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocAvgPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocNetMoneyField {
    pub fd: Field,
}

impl AllocNetMoneyField {
    #[must_use]
    pub const fn tag() -> u16 {
        154
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocNetMoneyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlCurrFxRateCalcField {
    pub fd: Field,
}

impl SettlCurrFxRateCalcField {
    #[must_use]
    pub const fn tag() -> u16 {
        156
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlCurrFxRateCalcField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NumDaysInterestField {
    pub fd: Field,
}

impl NumDaysInterestField {
    #[must_use]
    pub const fn tag() -> u16 {
        157
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NumDaysInterestField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccruedInterestAmtField {
    pub fd: Field,
}

impl AccruedInterestAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        159
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AccruedInterestAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlInstModeField {
    pub fd: Field,
}

impl SettlInstModeField {
    #[must_use]
    pub const fn tag() -> u16 {
        160
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstModeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocTextField {
    pub fd: Field,
}

impl AllocTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        161
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlInstIDField {
    pub fd: Field,
}

impl SettlInstIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        162
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlInstTransTypeField {
    pub fd: Field,
}

impl SettlInstTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        163
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmailThreadIDField {
    pub fd: Field,
}

impl EmailThreadIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        164
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EmailThreadIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlInstSourceField {
    pub fd: Field,
}

impl SettlInstSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        165
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlLocationField {
    pub fd: Field,
}

impl SettlLocationField {
    #[must_use]
    pub const fn tag() -> u16 {
        166
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlLocationField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityTypeField {
    pub fd: Field,
}

impl SecurityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        167
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StandInstDbTypeField {
    pub fd: Field,
}

impl StandInstDbTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        169
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StandInstDbTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StandInstDbNameField {
    pub fd: Field,
}

impl StandInstDbNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        170
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StandInstDbNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StandInstDbIDField {
    pub fd: Field,
}

impl StandInstDbIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        171
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StandInstDbIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlDeliveryTypeField {
    pub fd: Field,
}

impl SettlDeliveryTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        172
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlDeliveryTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlDepositoryCodeField {
    pub fd: Field,
}

impl SettlDepositoryCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        173
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlDepositoryCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlBrkrCodeField {
    pub fd: Field,
}

impl SettlBrkrCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        174
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlBrkrCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlInstCodeField {
    pub fd: Field,
}

impl SettlInstCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        175
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecuritySettlAgentNameField {
    pub fd: Field,
}

impl SecuritySettlAgentNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        176
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecuritySettlAgentNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecuritySettlAgentCodeField {
    pub fd: Field,
}

impl SecuritySettlAgentCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        177
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecuritySettlAgentCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecuritySettlAgentAcctNumField {
    pub fd: Field,
}

impl SecuritySettlAgentAcctNumField {
    #[must_use]
    pub const fn tag() -> u16 {
        178
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecuritySettlAgentAcctNumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecuritySettlAgentAcctNameField {
    pub fd: Field,
}

impl SecuritySettlAgentAcctNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        179
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecuritySettlAgentAcctNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecuritySettlAgentContactNameField {
    pub fd: Field,
}

impl SecuritySettlAgentContactNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        180
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecuritySettlAgentContactNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecuritySettlAgentContactPhoneField {
    pub fd: Field,
}

impl SecuritySettlAgentContactPhoneField {
    #[must_use]
    pub const fn tag() -> u16 {
        181
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecuritySettlAgentContactPhoneField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashSettlAgentNameField {
    pub fd: Field,
}

impl CashSettlAgentNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        182
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashSettlAgentNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashSettlAgentCodeField {
    pub fd: Field,
}

impl CashSettlAgentCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        183
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashSettlAgentCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashSettlAgentAcctNumField {
    pub fd: Field,
}

impl CashSettlAgentAcctNumField {
    #[must_use]
    pub const fn tag() -> u16 {
        184
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashSettlAgentAcctNumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashSettlAgentAcctNameField {
    pub fd: Field,
}

impl CashSettlAgentAcctNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        185
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashSettlAgentAcctNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashSettlAgentContactNameField {
    pub fd: Field,
}

impl CashSettlAgentContactNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        186
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashSettlAgentContactNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashSettlAgentContactPhoneField {
    pub fd: Field,
}

impl CashSettlAgentContactPhoneField {
    #[must_use]
    pub const fn tag() -> u16 {
        187
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashSettlAgentContactPhoneField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidSpotRateField {
    pub fd: Field,
}

impl BidSpotRateField {
    #[must_use]
    pub const fn tag() -> u16 {
        188
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidSpotRateField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OfferSpotRateField {
    pub fd: Field,
}

impl OfferSpotRateField {
    #[must_use]
    pub const fn tag() -> u16 {
        190
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OfferSpotRateField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrderQty2Field {
    pub fd: Field,
}

impl OrderQty2Field {
    #[must_use]
    pub const fn tag() -> u16 {
        192
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderQty2Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastSpotRateField {
    pub fd: Field,
}

impl LastSpotRateField {
    #[must_use]
    pub const fn tag() -> u16 {
        194
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastSpotRateField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocLinkIDField {
    pub fd: Field,
}

impl AllocLinkIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        196
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocLinkIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocLinkTypeField {
    pub fd: Field,
}

impl AllocLinkTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        197
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocLinkTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecondaryOrderIDField {
    pub fd: Field,
}

impl SecondaryOrderIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        198
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryOrderIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoIOIQualifiersField {
    pub fd: Field,
}

impl NoIOIQualifiersField {
    #[must_use]
    pub const fn tag() -> u16 {
        199
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoIOIQualifiersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PutOrCallField {
    pub fd: Field,
}

impl PutOrCallField {
    #[must_use]
    pub const fn tag() -> u16 {
        201
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PutOrCallField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StrikePriceField {
    pub fd: Field,
}

impl StrikePriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        202
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrikePriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CoveredOrUncoveredField {
    pub fd: Field,
}

impl CoveredOrUncoveredField {
    #[must_use]
    pub const fn tag() -> u16 {
        203
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CoveredOrUncoveredField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CustomerOrFirmField {
    pub fd: Field,
}

impl CustomerOrFirmField {
    #[must_use]
    pub const fn tag() -> u16 {
        204
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CustomerOrFirmField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OptAttributeField {
    pub fd: Field,
}

impl OptAttributeField {
    #[must_use]
    pub const fn tag() -> u16 {
        206
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OptAttributeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocHandlInstField {
    pub fd: Field,
}

impl AllocHandlInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        209
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocHandlInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MaxShowField {
    pub fd: Field,
}

impl MaxShowField {
    #[must_use]
    pub const fn tag() -> u16 {
        210
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaxShowField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlInstRefIDField {
    pub fd: Field,
}

impl SettlInstRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        214
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoRoutingIDsField {
    pub fd: Field,
}

impl NoRoutingIDsField {
    #[must_use]
    pub const fn tag() -> u16 {
        215
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoRoutingIDsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoutingTypeField {
    pub fd: Field,
}

impl RoutingTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        216
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RoutingTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoutingIDField {
    pub fd: Field,
}

impl RoutingIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        217
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RoutingIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BenchmarkField {
    pub fd: Field,
}

impl BenchmarkField {
    #[must_use]
    pub const fn tag() -> u16 {
        219
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BenchmarkField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDReqIDField {
    pub fd: Field,
}

impl MDReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        262
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubscriptionRequestTypeField {
    pub fd: Field,
}

impl SubscriptionRequestTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        263
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SubscriptionRequestTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MarketDepthField {
    pub fd: Field,
}

impl MarketDepthField {
    #[must_use]
    pub const fn tag() -> u16 {
        264
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MarketDepthField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDUpdateTypeField {
    pub fd: Field,
}

impl MDUpdateTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        265
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDUpdateTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoMDEntryTypesField {
    pub fd: Field,
}

impl NoMDEntryTypesField {
    #[must_use]
    pub const fn tag() -> u16 {
        267
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoMDEntryTypesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoMDEntriesField {
    pub fd: Field,
}

impl NoMDEntriesField {
    #[must_use]
    pub const fn tag() -> u16 {
        268
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoMDEntriesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntryTypeField {
    pub fd: Field,
}

impl MDEntryTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        269
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntryTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntryPxField {
    pub fd: Field,
}

impl MDEntryPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        270
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntryPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntrySizeField {
    pub fd: Field,
}

impl MDEntrySizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        271
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntrySizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TickDirectionField {
    pub fd: Field,
}

impl TickDirectionField {
    #[must_use]
    pub const fn tag() -> u16 {
        274
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TickDirectionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntryIDField {
    pub fd: Field,
}

impl MDEntryIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        278
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntryIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDUpdateActionField {
    pub fd: Field,
}

impl MDUpdateActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        279
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDUpdateActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntryRefIDField {
    pub fd: Field,
}

impl MDEntryRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        280
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntryRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDReqRejReasonField {
    pub fd: Field,
}

impl MDReqRejReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        281
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDReqRejReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntryOriginatorField {
    pub fd: Field,
}

impl MDEntryOriginatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        282
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntryOriginatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocationIDField {
    pub fd: Field,
}

impl LocationIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        283
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LocationIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeskIDField {
    pub fd: Field,
}

impl DeskIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        284
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeskIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeleteReasonField {
    pub fd: Field,
}

impl DeleteReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        285
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeleteReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenCloseSettleFlagField {
    pub fd: Field,
}

impl OpenCloseSettleFlagField {
    #[must_use]
    pub const fn tag() -> u16 {
        286
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OpenCloseSettleFlagField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SellerDaysField {
    pub fd: Field,
}

impl SellerDaysField {
    #[must_use]
    pub const fn tag() -> u16 {
        287
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SellerDaysField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntryBuyerField {
    pub fd: Field,
}

impl MDEntryBuyerField {
    #[must_use]
    pub const fn tag() -> u16 {
        288
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntryBuyerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntrySellerField {
    pub fd: Field,
}

impl MDEntrySellerField {
    #[must_use]
    pub const fn tag() -> u16 {
        289
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntrySellerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MDEntryPositionNoField {
    pub fd: Field,
}

impl MDEntryPositionNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        290
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDEntryPositionNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FinancialStatusField {
    pub fd: Field,
}

impl FinancialStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        291
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FinancialStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CorporateActionField {
    pub fd: Field,
}

impl CorporateActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        292
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CorporateActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefBidSizeField {
    pub fd: Field,
}

impl DefBidSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        293
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DefBidSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefOfferSizeField {
    pub fd: Field,
}

impl DefOfferSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        294
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DefOfferSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoQuoteEntriesField {
    pub fd: Field,
}

impl NoQuoteEntriesField {
    #[must_use]
    pub const fn tag() -> u16 {
        295
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoQuoteEntriesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoQuoteSetsField {
    pub fd: Field,
}

impl NoQuoteSetsField {
    #[must_use]
    pub const fn tag() -> u16 {
        296
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoQuoteSetsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteAckStatusField {
    pub fd: Field,
}

impl QuoteAckStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        297
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteAckStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteCancelTypeField {
    pub fd: Field,
}

impl QuoteCancelTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        298
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteCancelTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteEntryIDField {
    pub fd: Field,
}

impl QuoteEntryIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        299
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteEntryIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteRejectReasonField {
    pub fd: Field,
}

impl QuoteRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        300
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteResponseLevelField {
    pub fd: Field,
}

impl QuoteResponseLevelField {
    #[must_use]
    pub const fn tag() -> u16 {
        301
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteResponseLevelField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteSetIDField {
    pub fd: Field,
}

impl QuoteSetIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        302
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteSetIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteRequestTypeField {
    pub fd: Field,
}

impl QuoteRequestTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        303
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteRequestTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotQuoteEntriesField {
    pub fd: Field,
}

impl TotQuoteEntriesField {
    #[must_use]
    pub const fn tag() -> u16 {
        304
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotQuoteEntriesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingIDSourceField {
    pub fd: Field,
}

impl UnderlyingIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        305
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingIssuerField {
    pub fd: Field,
}

impl UnderlyingIssuerField {
    #[must_use]
    pub const fn tag() -> u16 {
        306
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingIssuerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingSecurityDescField {
    pub fd: Field,
}

impl UnderlyingSecurityDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        307
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSecurityDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingSecurityIDField {
    pub fd: Field,
}

impl UnderlyingSecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        309
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingSecurityTypeField {
    pub fd: Field,
}

impl UnderlyingSecurityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        310
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSecurityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingSymbolField {
    pub fd: Field,
}

impl UnderlyingSymbolField {
    #[must_use]
    pub const fn tag() -> u16 {
        311
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSymbolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingSymbolSfxField {
    pub fd: Field,
}

impl UnderlyingSymbolSfxField {
    #[must_use]
    pub const fn tag() -> u16 {
        312
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSymbolSfxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingPutOrCallField {
    pub fd: Field,
}

impl UnderlyingPutOrCallField {
    #[must_use]
    pub const fn tag() -> u16 {
        315
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingPutOrCallField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingStrikePriceField {
    pub fd: Field,
}

impl UnderlyingStrikePriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        316
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingStrikePriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingOptAttributeField {
    pub fd: Field,
}

impl UnderlyingOptAttributeField {
    #[must_use]
    pub const fn tag() -> u16 {
        317
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingOptAttributeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingCurrencyField {
    pub fd: Field,
}

impl UnderlyingCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        318
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RatioQtyField {
    pub fd: Field,
}

impl RatioQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        319
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RatioQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityReqIDField {
    pub fd: Field,
}

impl SecurityReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        320
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityRequestTypeField {
    pub fd: Field,
}

impl SecurityRequestTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        321
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityRequestTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityResponseIDField {
    pub fd: Field,
}

impl SecurityResponseIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        322
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityResponseIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityResponseTypeField {
    pub fd: Field,
}

impl SecurityResponseTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        323
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityResponseTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityStatusReqIDField {
    pub fd: Field,
}

impl SecurityStatusReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        324
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityStatusReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityTradingStatusField {
    pub fd: Field,
}

impl SecurityTradingStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        326
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityTradingStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HaltReasonCharField {
    pub fd: Field,
}

impl HaltReasonCharField {
    #[must_use]
    pub const fn tag() -> u16 {
        327
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HaltReasonCharField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BuyVolumeField {
    pub fd: Field,
}

impl BuyVolumeField {
    #[must_use]
    pub const fn tag() -> u16 {
        330
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BuyVolumeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SellVolumeField {
    pub fd: Field,
}

impl SellVolumeField {
    #[must_use]
    pub const fn tag() -> u16 {
        331
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SellVolumeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HighPxField {
    pub fd: Field,
}

impl HighPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        332
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HighPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LowPxField {
    pub fd: Field,
}

impl LowPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        333
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LowPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdjustmentField {
    pub fd: Field,
}

impl AdjustmentField {
    #[must_use]
    pub const fn tag() -> u16 {
        334
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdjustmentField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradSesReqIDField {
    pub fd: Field,
}

impl TradSesReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        335
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradSesReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradingSessionIDField {
    pub fd: Field,
}

impl TradingSessionIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        336
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradingSessionIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContraTraderField {
    pub fd: Field,
}

impl ContraTraderField {
    #[must_use]
    pub const fn tag() -> u16 {
        337
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContraTraderField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradSesMethodField {
    pub fd: Field,
}

impl TradSesMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        338
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradSesMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradSesModeField {
    pub fd: Field,
}

impl TradSesModeField {
    #[must_use]
    pub const fn tag() -> u16 {
        339
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradSesModeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradSesStatusField {
    pub fd: Field,
}

impl TradSesStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        340
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradSesStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NumberOfOrdersField {
    pub fd: Field,
}

impl NumberOfOrdersField {
    #[must_use]
    pub const fn tag() -> u16 {
        346
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NumberOfOrdersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MessageEncodingField {
    pub fd: Field,
}

impl MessageEncodingField {
    #[must_use]
    pub const fn tag() -> u16 {
        347
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MessageEncodingField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocPriceField {
    pub fd: Field,
}

impl AllocPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        366
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteEntryRejectReasonField {
    pub fd: Field,
}

impl QuoteEntryRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        368
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteEntryRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastMsgSeqNumProcessedField {
    pub fd: Field,
}

impl LastMsgSeqNumProcessedField {
    #[must_use]
    pub const fn tag() -> u16 {
        369
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastMsgSeqNumProcessedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RefTagIDField {
    pub fd: Field,
}

impl RefTagIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        371
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefTagIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RefMsgTypeField {
    pub fd: Field,
}

impl RefMsgTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        372
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefMsgTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SessionRejectReasonField {
    pub fd: Field,
}

impl SessionRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        373
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SessionRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidRequestTransTypeField {
    pub fd: Field,
}

impl BidRequestTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        374
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidRequestTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContraBrokerField {
    pub fd: Field,
}

impl ContraBrokerField {
    #[must_use]
    pub const fn tag() -> u16 {
        375
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContraBrokerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComplianceIDField {
    pub fd: Field,
}

impl ComplianceIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        376
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ComplianceIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecRestatementReasonField {
    pub fd: Field,
}

impl ExecRestatementReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        378
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecRestatementReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessRejectRefIDField {
    pub fd: Field,
}

impl BusinessRejectRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        379
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BusinessRejectRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessRejectReasonField {
    pub fd: Field,
}

impl BusinessRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        380
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BusinessRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GrossTradeAmtField {
    pub fd: Field,
}

impl GrossTradeAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        381
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for GrossTradeAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoContraBrokersField {
    pub fd: Field,
}

impl NoContraBrokersField {
    #[must_use]
    pub const fn tag() -> u16 {
        382
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoContraBrokersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MaxMessageSizeField {
    pub fd: Field,
}

impl MaxMessageSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        383
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaxMessageSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoMsgTypesField {
    pub fd: Field,
}

impl NoMsgTypesField {
    #[must_use]
    pub const fn tag() -> u16 {
        384
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoMsgTypesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MsgDirectionField {
    pub fd: Field,
}

impl MsgDirectionField {
    #[must_use]
    pub const fn tag() -> u16 {
        385
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MsgDirectionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoTradingSessionsField {
    pub fd: Field,
}

impl NoTradingSessionsField {
    #[must_use]
    pub const fn tag() -> u16 {
        386
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoTradingSessionsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotalVolumeTradedField {
    pub fd: Field,
}

impl TotalVolumeTradedField {
    #[must_use]
    pub const fn tag() -> u16 {
        387
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotalVolumeTradedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DiscretionInstField {
    pub fd: Field,
}

impl DiscretionInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        388
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DiscretionInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidIDField {
    pub fd: Field,
}

impl BidIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        390
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClientBidIDField {
    pub fd: Field,
}

impl ClientBidIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        391
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClientBidIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNameField {
    pub fd: Field,
}

impl ListNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        392
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotalNumSecuritiesField {
    pub fd: Field,
}

impl TotalNumSecuritiesField {
    #[must_use]
    pub const fn tag() -> u16 {
        393
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotalNumSecuritiesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidTypeField {
    pub fd: Field,
}

impl BidTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        394
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NumTicketsField {
    pub fd: Field,
}

impl NumTicketsField {
    #[must_use]
    pub const fn tag() -> u16 {
        395
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NumTicketsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SideValue1Field {
    pub fd: Field,
}

impl SideValue1Field {
    #[must_use]
    pub const fn tag() -> u16 {
        396
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideValue1Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SideValue2Field {
    pub fd: Field,
}

impl SideValue2Field {
    #[must_use]
    pub const fn tag() -> u16 {
        397
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideValue2Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoBidDescriptorsField {
    pub fd: Field,
}

impl NoBidDescriptorsField {
    #[must_use]
    pub const fn tag() -> u16 {
        398
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoBidDescriptorsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidDescriptorTypeField {
    pub fd: Field,
}

impl BidDescriptorTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        399
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidDescriptorTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidDescriptorField {
    pub fd: Field,
}

impl BidDescriptorField {
    #[must_use]
    pub const fn tag() -> u16 {
        400
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidDescriptorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SideValueIndField {
    pub fd: Field,
}

impl SideValueIndField {
    #[must_use]
    pub const fn tag() -> u16 {
        401
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideValueIndField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LiquidityValueField {
    pub fd: Field,
}

impl LiquidityValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        404
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LiquidityValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FairValueField {
    pub fd: Field,
}

impl FairValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        406
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FairValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValueOfFuturesField {
    pub fd: Field,
}

impl ValueOfFuturesField {
    #[must_use]
    pub const fn tag() -> u16 {
        408
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ValueOfFuturesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LiquidityIndTypeField {
    pub fd: Field,
}

impl LiquidityIndTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        409
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LiquidityIndTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OutMainCntryUIndexField {
    pub fd: Field,
}

impl OutMainCntryUIndexField {
    #[must_use]
    pub const fn tag() -> u16 {
        412
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OutMainCntryUIndexField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProgRptReqsField {
    pub fd: Field,
}

impl ProgRptReqsField {
    #[must_use]
    pub const fn tag() -> u16 {
        414
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ProgRptReqsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProgPeriodIntervalField {
    pub fd: Field,
}

impl ProgPeriodIntervalField {
    #[must_use]
    pub const fn tag() -> u16 {
        415
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ProgPeriodIntervalField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IncTaxIndField {
    pub fd: Field,
}

impl IncTaxIndField {
    #[must_use]
    pub const fn tag() -> u16 {
        416
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IncTaxIndField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NumBiddersField {
    pub fd: Field,
}

impl NumBiddersField {
    #[must_use]
    pub const fn tag() -> u16 {
        417
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NumBiddersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradeTypeField {
    pub fd: Field,
}

impl TradeTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        418
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BasisPxTypeField {
    pub fd: Field,
}

impl BasisPxTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        419
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BasisPxTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoBidComponentsField {
    pub fd: Field,
}

impl NoBidComponentsField {
    #[must_use]
    pub const fn tag() -> u16 {
        420
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoBidComponentsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CountryField {
    pub fd: Field,
}

impl CountryField {
    #[must_use]
    pub const fn tag() -> u16 {
        421
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CountryField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotNoStrikesField {
    pub fd: Field,
}

impl TotNoStrikesField {
    #[must_use]
    pub const fn tag() -> u16 {
        422
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNoStrikesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PriceTypeField {
    pub fd: Field,
}

impl PriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        423
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DayOrderQtyField {
    pub fd: Field,
}

impl DayOrderQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        424
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DayOrderQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DayCumQtyField {
    pub fd: Field,
}

impl DayCumQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        425
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DayCumQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DayAvgPxField {
    pub fd: Field,
}

impl DayAvgPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        426
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DayAvgPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GTBookingInstField {
    pub fd: Field,
}

impl GTBookingInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        427
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for GTBookingInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoStrikesField {
    pub fd: Field,
}

impl NoStrikesField {
    #[must_use]
    pub const fn tag() -> u16 {
        428
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoStrikesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListStatusTypeField {
    pub fd: Field,
}

impl ListStatusTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        429
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListStatusTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NetGrossIndField {
    pub fd: Field,
}

impl NetGrossIndField {
    #[must_use]
    pub const fn tag() -> u16 {
        430
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NetGrossIndField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListOrderStatusField {
    pub fd: Field,
}

impl ListOrderStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        431
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListOrderStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListExecInstTypeField {
    pub fd: Field,
}

impl ListExecInstTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        433
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListExecInstTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CxlRejResponseToField {
    pub fd: Field,
}

impl CxlRejResponseToField {
    #[must_use]
    pub const fn tag() -> u16 {
        434
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CxlRejResponseToField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContraTradeQtyField {
    pub fd: Field,
}

impl ContraTradeQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        437
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContraTradeQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClearingFirmField {
    pub fd: Field,
}

impl ClearingFirmField {
    #[must_use]
    pub const fn tag() -> u16 {
        439
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClearingFirmField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClearingAccountField {
    pub fd: Field,
}

impl ClearingAccountField {
    #[must_use]
    pub const fn tag() -> u16 {
        440
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClearingAccountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LiquidityNumSecuritiesField {
    pub fd: Field,
}

impl LiquidityNumSecuritiesField {
    #[must_use]
    pub const fn tag() -> u16 {
        441
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LiquidityNumSecuritiesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MultiLegReportingTypeField {
    pub fd: Field,
}

impl MultiLegReportingTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        442
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MultiLegReportingTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListStatusTextField {
    pub fd: Field,
}

impl ListStatusTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        444
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListStatusTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
pub struct StringField {
    tag: u16,
    fd: Field,
}

impl StringField {
    const fn tag(&self) -> u16 {
        self.tag
    }

    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StringField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}
pub struct CharField {
    tag: u16,
    fd: Field,
}

impl CharField {
    const fn tag(&self) -> u16 {
        self.tag
    }

    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CharField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}
pub struct DecimalField {
    tag: u16,
    fd: Field,
}

impl DecimalField {
    const fn tag(&self) -> u16 {
        self.tag
    }

    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DecimalField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}
pub struct IntField {
    tag: u16,
    fd: Field,
}

impl IntField {
    const fn tag(&self) -> u16 {
        self.tag
    }

    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IntField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}
impl FieldSet {
    /// for use by custom fields
    pub fn get_str_field(&self, tag: u16) -> Result<StringField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::String(t, v) => Ok(StringField {
                tag: *t,
                fd: Field::String(*t, v.to_string()),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
    /// for use by custom fields
    pub fn get_char_field(&self, tag: u16) -> Result<CharField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::Char(t, v) => Ok(CharField {
                tag: *t,
                fd: Field::Char(*t, *v),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
    /// for use by custom fields
    pub fn get_decimal_field(&self, tag: u16) -> Result<DecimalField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::Decimal(t, v) => Ok(DecimalField {
                tag: *t,
                fd: Field::Decimal(*t, *v),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
    /// for use by custom fields
    pub fn get_i32_field(&self, tag: u16) -> Result<IntField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::Int(t, v) => Ok(IntField {
                tag: *t,
                fd: Field::Int(*t, *v),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
}
