use std::fmt::Formatter;
use std::str::FromStr;
use itertools::Itertools;

use rust_decimal::Decimal;

use crate::model::field::{FieldSet, FieldTypeMismatchError};
use crate::model::message_type::UnknownMsgTypeError;
use crate::model::BeginString;
use crate::model::generated::fields::Field;

pub struct FIX43CrackerUtils;
// parse string (35=D) into Field{35, "D"}
impl FIX43CrackerUtils {
    pub fn crack(s: &str) -> Result<Field, ()> {
        println!("crack for Field: {}", &s);
        let two_parts = s.split_once('=');
        match two_parts {
            Some((s_tag, s_value)) => {
                println!("parsing tag: {}, field: {} into Field", s_tag, s_value);
                  
                // figure out the tag
                let tag: u16 = s_tag.parse::<u16>().unwrap();
                   
                // build field using the tag & value
                match tag {
                    1 | 2 | 3 | 5 | 8 | 10 | 11 | 15 | 17 | 19 | 22 | 23 | 26 | 27 | 35 | 37 | 41 | 48 | 49 | 50 | 55 | 56 | 57 | 58 | 65 | 66 | 69 | 70 | 72 | 79 | 106 | 107 | 112 | 115 | 116 | 117 | 120 | 128 | 129 | 131 | 138 | 142 | 143 | 144 | 145 | 147 | 148 | 149 | 161 | 162 | 164 | 167 | 170 | 171 | 173 | 174 | 175 | 176 | 177 | 178 | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 196 | 198 | 214 | 217 | 220 | 221 | 222 | 233 | 234 | 235 | 239 | 243 | 250 | 255 | 256 | 257 | 262 | 278 | 280 | 282 | 283 | 284 | 288 | 289 | 299 | 302 | 305 | 306 | 307 | 309 | 310 | 311 | 312 | 320 | 322 | 324 | 335 | 336 | 337 | 347 | 372 | 375 | 376 | 379 | 390 | 391 | 392 | 400 | 444 | 448 | 455 | 456 | 458 | 459 | 461 | 463 | 466 | 467 | 471 | 472 | 474 | 476 | 478 | 479 | 482 | 488 | 489 | 491 | 493 | 494 | 496 | 498 | 499 | 500 | 501 | 505 | 508 | 509 | 511 | 513 | 521 | 523 | 524 | 526 | 527 | 535 | 536 | 543 | 545 | 548 | 551 | 553 | 554 | 556 | 568 | 571 | 572 | 574 | 578 | 579 | 583 | 584 | 593 | 594 | 595 | 597 | 598 | 599 | 600 | 601 | 602 | 603 | 605 | 606 | 608 | 609 | 617 | 620 | 625 | 628 | 635 | 644 | 649 | 654 | 655 | 659 => Ok(Field::String(tag, s_value.to_string())),

                    4 | 13 | 21 | 25 | 28 | 29 | 39 | 40 | 47 | 54 | 59 | 61 | 63 | 71 | 77 | 81 | 94 | 104 | 127 | 139 | 150 | 156 | 160 | 163 | 165 | 206 | 219 | 263 | 269 | 274 | 279 | 281 | 285 | 317 | 327 | 374 | 385 | 388 | 418 | 419 | 433 | 434 | 442 | 447 | 468 | 480 | 481 | 484 | 487 | 497 | 506 | 514 | 517 | 525 | 528 | 530 | 531 | 532 | 544 | 564 | 573 | 587 | 589 | 590 | 591 | 613 | 624 => Ok(Field::Char(tag, str::parse::<char>(s_value).unwrap())),

                    6 | 31 | 44 | 99 | 132 | 133 | 140 | 153 | 188 | 190 | 194 | 202 | 260 | 270 | 316 | 332 | 333 | 366 | 426 | 566 | 612 | 631 | 637 | 640 | 645 | 646 | 651 => Ok(Field::Decimal(tag, Decimal::from_str(s_value).unwrap())),

                    12 | 14 | 32 | 38 | 53 | 67 | 68 | 74 | 80 | 83 | 84 | 87 | 88 | 98 | 102 | 103 | 108 | 110 | 111 | 118 | 119 | 134 | 135 | 137 | 151 | 152 | 154 | 157 | 159 | 169 | 172 | 192 | 197 | 203 | 209 | 210 | 216 | 226 | 237 | 238 | 244 | 251 | 264 | 265 | 271 | 287 | 290 | 293 | 294 | 297 | 298 | 300 | 301 | 303 | 304 | 315 | 321 | 323 | 326 | 330 | 331 | 334 | 338 | 339 | 340 | 346 | 368 | 371 | 373 | 378 | 380 | 381 | 387 | 393 | 394 | 395 | 396 | 397 | 399 | 401 | 404 | 406 | 408 | 409 | 412 | 414 | 415 | 416 | 417 | 422 | 423 | 424 | 425 | 427 | 429 | 430 | 431 | 437 | 441 | 452 | 460 | 462 | 465 | 477 | 492 | 495 | 507 | 519 | 522 | 533 | 537 | 538 | 540 | 549 | 550 | 557 | 559 | 560 | 561 | 562 | 563 | 565 | 567 | 569 | 576 | 577 | 581 | 582 | 585 | 607 | 626 | 638 | 647 | 648 | 652 | 658 => Ok(Field::Int(tag, str::parse::<i32>(s_value).unwrap())),

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
impl TryFrom<&Field> for AccountField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AdvIdField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AdvRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AdvSideField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AdvTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AvgPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BeginStringField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CheckSumField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ClOrdIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CommissionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CommTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CumQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CurrencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ExecIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ExecRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for HandlInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityIDSourceField {
    pub fd: Field,
}

impl SecurityIDSourceField {
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

impl std::fmt::Display for SecurityIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityIDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for IOIidField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for IOIQltyIndField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for IOIRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIQtyField {
    pub fd: Field,
}

impl IOIQtyField {
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

impl std::fmt::Display for IOIQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOIQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for IOITransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LastCapacityField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LastPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastQtyField {
    pub fd: Field,
}

impl LastQtyField {
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

impl std::fmt::Display for LastQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LastQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MsgTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OrderIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OrderQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OrdStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OrdTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OrigClOrdIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for PriceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for Rule80AField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SenderCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SenderSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuantityField {
    pub fd: Field,
}

impl QuantityField {
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

impl std::fmt::Display for QuantityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuantityField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SideField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SymbolField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TargetCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TargetSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TextField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TimeInForceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UrgencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlmntTypField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SymbolSfxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ListIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ListSeqNoField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TotNoOrdersField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ListExecInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for RefAllocIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AvgPrxPrecisionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PositionEffectField {
    pub fd: Field,
}

impl PositionEffectField {
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

impl std::fmt::Display for PositionEffectField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PositionEffectField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocAccountField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocQtyField {
    pub fd: Field,
}

impl AllocQtyField {
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

impl std::fmt::Display for AllocQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ProcessCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for RptSeqField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CxlQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocRejCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for EmailTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for EncryptMethodField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for StopPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CxlRejReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OrdRejReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for IOIQualifierField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for IssuerField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityDescField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for HeartBtIntField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MinQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MaxFloorField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TestReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OnBehalfOfCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OnBehalfOfSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for NetMoneyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlCurrAmtField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlCurrencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DKReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DeliverToCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DeliverToSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BidPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OfferPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BidSizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OfferSizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MiscFeeAmtField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MiscFeeCurrField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MiscFeeTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for PrevClosePxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SenderLocationIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TargetLocationIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OnBehalfOfLocationIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DeliverToLocationIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SubjectField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for HeadlineField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for URLLinkField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ExecTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LeavesQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CashOrderQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocAvgPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocNetMoneyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlCurrFxRateCalcField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for NumDaysInterestField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AccruedInterestAmtField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlInstModeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocTextField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlInstIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlInstTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for EmailThreadIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlInstSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for StandInstDbTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for StandInstDbNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for StandInstDbIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlDeliveryTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlDepositoryCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlBrkrCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlInstCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecuritySettlAgentNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecuritySettlAgentCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecuritySettlAgentAcctNumField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecuritySettlAgentAcctNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecuritySettlAgentContactNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecuritySettlAgentContactPhoneField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CashSettlAgentNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CashSettlAgentCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CashSettlAgentAcctNumField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CashSettlAgentAcctNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CashSettlAgentContactNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CashSettlAgentContactPhoneField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BidSpotRateField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OfferSpotRateField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OrderQty2Field {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LastSpotRateField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocLinkIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocLinkTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecondaryOrderIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for StrikePriceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CoveredOrUncoveredField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OptAttributeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocHandlInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MaxShowField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SettlInstRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for RoutingTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for RoutingIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BenchmarkField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BenchmarkCurveCurrencyField {
    pub fd: Field,
}

impl BenchmarkCurveCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        220
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BenchmarkCurveCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BenchmarkCurveCurrencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BenchmarkCurveNameField {
    pub fd: Field,
}

impl BenchmarkCurveNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        221
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BenchmarkCurveNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BenchmarkCurveNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BenchmarkCurvePointField {
    pub fd: Field,
}

impl BenchmarkCurvePointField {
    #[must_use]
    pub const fn tag() -> u16 {
        222
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BenchmarkCurvePointField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BenchmarkCurvePointField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RepurchaseTermField {
    pub fd: Field,
}

impl RepurchaseTermField {
    #[must_use]
    pub const fn tag() -> u16 {
        226
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RepurchaseTermField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RepurchaseTermField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StipulationTypeField {
    pub fd: Field,
}

impl StipulationTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        233
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StipulationTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StipulationTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StipulationValueField {
    pub fd: Field,
}

impl StipulationValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        234
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StipulationValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StipulationValueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct YieldTypeField {
    pub fd: Field,
}

impl YieldTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        235
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for YieldTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for YieldTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotalTakedownField {
    pub fd: Field,
}

impl TotalTakedownField {
    #[must_use]
    pub const fn tag() -> u16 {
        237
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotalTakedownField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotalTakedownField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConcessionField {
    pub fd: Field,
}

impl ConcessionField {
    #[must_use]
    pub const fn tag() -> u16 {
        238
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ConcessionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ConcessionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RepoCollateralSecurityTypeField {
    pub fd: Field,
}

impl RepoCollateralSecurityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        239
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RepoCollateralSecurityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RepoCollateralSecurityTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingRepoCollateralSecurityTypeField {
    pub fd: Field,
}

impl UnderlyingRepoCollateralSecurityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        243
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingRepoCollateralSecurityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingRepoCollateralSecurityTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingRepurchaseTermField {
    pub fd: Field,
}

impl UnderlyingRepurchaseTermField {
    #[must_use]
    pub const fn tag() -> u16 {
        244
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingRepurchaseTermField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingRepurchaseTermField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegRepoCollateralSecurityTypeField {
    pub fd: Field,
}

impl LegRepoCollateralSecurityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        250
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegRepoCollateralSecurityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegRepoCollateralSecurityTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegRepurchaseTermField {
    pub fd: Field,
}

impl LegRepurchaseTermField {
    #[must_use]
    pub const fn tag() -> u16 {
        251
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegRepurchaseTermField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegRepurchaseTermField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreditRatingField {
    pub fd: Field,
}

impl CreditRatingField {
    #[must_use]
    pub const fn tag() -> u16 {
        255
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CreditRatingField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CreditRatingField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingCreditRatingField {
    pub fd: Field,
}

impl UnderlyingCreditRatingField {
    #[must_use]
    pub const fn tag() -> u16 {
        256
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCreditRatingField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCreditRatingField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegCreditRatingField {
    pub fd: Field,
}

impl LegCreditRatingField {
    #[must_use]
    pub const fn tag() -> u16 {
        257
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegCreditRatingField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegCreditRatingField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BasisFeaturePriceField {
    pub fd: Field,
}

impl BasisFeaturePriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        260
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BasisFeaturePriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BasisFeaturePriceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SubscriptionRequestTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MarketDepthField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDUpdateTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntryTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntryPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntrySizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TickDirectionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntryIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDUpdateActionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntryRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDReqRejReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntryOriginatorField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LocationIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DeskIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DeleteReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SellerDaysField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntryBuyerField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntrySellerField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MDEntryPositionNoField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DefBidSizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DefOfferSizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteStatusField {
    pub fd: Field,
}

impl QuoteStatusField {
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

impl std::fmt::Display for QuoteStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteCancelTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteEntryIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteRejectReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteResponseLevelField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteSetIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteRequestTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TotQuoteEntriesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingSecurityIDSourceField {
    pub fd: Field,
}

impl UnderlyingSecurityIDSourceField {
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

impl std::fmt::Display for UnderlyingSecurityIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSecurityIDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingIssuerField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingSecurityDescField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingSecurityIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingSecurityTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingSymbolField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingSymbolSfxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingPutOrCallField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingStrikePriceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for UnderlyingOptAttributeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityRequestTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityResponseIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityResponseTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityStatusReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SecurityTradingStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for HaltReasonCharField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BuyVolumeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SellVolumeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for HighPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LowPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AdjustmentField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TradSesReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TradingSessionIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ContraTraderField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TradSesMethodField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TradSesModeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TradSesStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for NumberOfOrdersField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MessageEncodingField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for AllocPriceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for QuoteEntryRejectReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for RefTagIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for RefMsgTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SessionRejectReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BidRequestTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ContraBrokerField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ComplianceIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ExecRestatementReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BusinessRejectRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BusinessRejectReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for GrossTradeAmtField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MsgDirectionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TotalVolumeTradedField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DiscretionInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BidIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ClientBidIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ListNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TotalNumSecuritiesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BidTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for NumTicketsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SideValue1Field {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SideValue2Field {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BidDescriptorTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BidDescriptorField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for SideValueIndField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LiquidityValueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for FairValueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ValueOfFuturesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LiquidityIndTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for OutMainCntryUIndexField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ProgRptReqsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ProgPeriodIntervalField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for IncTaxIndField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for NumBiddersField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TradeTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for BasisPxTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for TotNoStrikesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for PriceTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DayOrderQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DayCumQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for DayAvgPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for GTBookingInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ListStatusTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for NetGrossIndField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ListOrderStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ListExecInstTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for CxlRejResponseToField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ContraTradeQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for LiquidityNumSecuritiesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for MultiLegReportingTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
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
impl TryFrom<&Field> for ListStatusTextField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartyIDSourceField {
    pub fd: Field,
}

impl PartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        447
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PartyIDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartyIDField {
    pub fd: Field,
}

impl PartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        448
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PartyIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartyRoleField {
    pub fd: Field,
}

impl PartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        452
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PartyRoleField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityAltIDField {
    pub fd: Field,
}

impl SecurityAltIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        455
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityAltIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityAltIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityAltIDSourceField {
    pub fd: Field,
}

impl SecurityAltIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        456
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityAltIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityAltIDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingSecurityAltIDField {
    pub fd: Field,
}

impl UnderlyingSecurityAltIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        458
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSecurityAltIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSecurityAltIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingSecurityAltIDSourceField {
    pub fd: Field,
}

impl UnderlyingSecurityAltIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        459
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSecurityAltIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSecurityAltIDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProductField {
    pub fd: Field,
}

impl ProductField {
    #[must_use]
    pub const fn tag() -> u16 {
        460
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ProductField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ProductField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CFICodeField {
    pub fd: Field,
}

impl CFICodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        461
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CFICodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CFICodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingProductField {
    pub fd: Field,
}

impl UnderlyingProductField {
    #[must_use]
    pub const fn tag() -> u16 {
        462
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingProductField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingProductField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingCFICodeField {
    pub fd: Field,
}

impl UnderlyingCFICodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        463
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCFICodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCFICodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuantityTypeField {
    pub fd: Field,
}

impl QuantityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        465
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuantityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuantityTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BookingRefIDField {
    pub fd: Field,
}

impl BookingRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        466
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BookingRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BookingRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndividualAllocIDField {
    pub fd: Field,
}

impl IndividualAllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        467
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IndividualAllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IndividualAllocIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoundingDirectionField {
    pub fd: Field,
}

impl RoundingDirectionField {
    #[must_use]
    pub const fn tag() -> u16 {
        468
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RoundingDirectionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RoundingDirectionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StateOrProvinceOfIssueField {
    pub fd: Field,
}

impl StateOrProvinceOfIssueField {
    #[must_use]
    pub const fn tag() -> u16 {
        471
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StateOrProvinceOfIssueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StateOrProvinceOfIssueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocaleOfIssueField {
    pub fd: Field,
}

impl LocaleOfIssueField {
    #[must_use]
    pub const fn tag() -> u16 {
        472
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LocaleOfIssueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LocaleOfIssueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MailingDtlsField {
    pub fd: Field,
}

impl MailingDtlsField {
    #[must_use]
    pub const fn tag() -> u16 {
        474
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MailingDtlsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MailingDtlsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaymentRefField {
    pub fd: Field,
}

impl PaymentRefField {
    #[must_use]
    pub const fn tag() -> u16 {
        476
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PaymentRefField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PaymentRefField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DistribPaymentMethodField {
    pub fd: Field,
}

impl DistribPaymentMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        477
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DistribPaymentMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DistribPaymentMethodField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashDistribCurrField {
    pub fd: Field,
}

impl CashDistribCurrField {
    #[must_use]
    pub const fn tag() -> u16 {
        478
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashDistribCurrField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CashDistribCurrField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommCurrencyField {
    pub fd: Field,
}

impl CommCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        479
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CommCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CommCurrencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CancellationRightsField {
    pub fd: Field,
}

impl CancellationRightsField {
    #[must_use]
    pub const fn tag() -> u16 {
        480
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CancellationRightsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CancellationRightsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MoneyLaunderingStatusField {
    pub fd: Field,
}

impl MoneyLaunderingStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        481
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MoneyLaunderingStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MoneyLaunderingStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MailingInstField {
    pub fd: Field,
}

impl MailingInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        482
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MailingInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MailingInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecPriceTypeField {
    pub fd: Field,
}

impl ExecPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        484
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExecPriceTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradeReportTransTypeField {
    pub fd: Field,
}

impl TradeReportTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        487
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeReportTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeReportTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CardHolderNameField {
    pub fd: Field,
}

impl CardHolderNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        488
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CardHolderNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CardHolderNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CardNumberField {
    pub fd: Field,
}

impl CardNumberField {
    #[must_use]
    pub const fn tag() -> u16 {
        489
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CardNumberField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CardNumberField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CardIssNoField {
    pub fd: Field,
}

impl CardIssNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        491
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CardIssNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CardIssNoField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaymentMethodField {
    pub fd: Field,
}

impl PaymentMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        492
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PaymentMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PaymentMethodField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistAcctTypeField {
    pub fd: Field,
}

impl RegistAcctTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        493
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistAcctTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistAcctTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DesignationField {
    pub fd: Field,
}

impl DesignationField {
    #[must_use]
    pub const fn tag() -> u16 {
        494
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DesignationField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DesignationField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaxAdvantageTypeField {
    pub fd: Field,
}

impl TaxAdvantageTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        495
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TaxAdvantageTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TaxAdvantageTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistRejReasonTextField {
    pub fd: Field,
}

impl RegistRejReasonTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        496
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistRejReasonTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistRejReasonTextField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FundRenewWaivField {
    pub fd: Field,
}

impl FundRenewWaivField {
    #[must_use]
    pub const fn tag() -> u16 {
        497
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FundRenewWaivField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FundRenewWaivField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashDistribAgentNameField {
    pub fd: Field,
}

impl CashDistribAgentNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        498
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashDistribAgentNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CashDistribAgentNameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashDistribAgentCodeField {
    pub fd: Field,
}

impl CashDistribAgentCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        499
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashDistribAgentCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CashDistribAgentCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashDistribAgentAcctNumberField {
    pub fd: Field,
}

impl CashDistribAgentAcctNumberField {
    #[must_use]
    pub const fn tag() -> u16 {
        500
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashDistribAgentAcctNumberField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CashDistribAgentAcctNumberField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashDistribPayRefField {
    pub fd: Field,
}

impl CashDistribPayRefField {
    #[must_use]
    pub const fn tag() -> u16 {
        501
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashDistribPayRefField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CashDistribPayRefField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaymentRemitterIDField {
    pub fd: Field,
}

impl PaymentRemitterIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        505
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PaymentRemitterIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PaymentRemitterIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistStatusField {
    pub fd: Field,
}

impl RegistStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        506
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistRejReasonCodeField {
    pub fd: Field,
}

impl RegistRejReasonCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        507
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistRejReasonCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistRejReasonCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistRefIDField {
    pub fd: Field,
}

impl RegistRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        508
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistDetlsField {
    pub fd: Field,
}

impl RegistDetlsField {
    #[must_use]
    pub const fn tag() -> u16 {
        509
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistDetlsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistDetlsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistEmailField {
    pub fd: Field,
}

impl RegistEmailField {
    #[must_use]
    pub const fn tag() -> u16 {
        511
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistEmailField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistEmailField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistIDField {
    pub fd: Field,
}

impl RegistIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        513
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegistTransTypeField {
    pub fd: Field,
}

impl RegistTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        514
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RegistTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OwnershipTypeField {
    pub fd: Field,
}

impl OwnershipTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        517
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OwnershipTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OwnershipTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContAmtTypeField {
    pub fd: Field,
}

impl ContAmtTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        519
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContAmtTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ContAmtTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContAmtCurrField {
    pub fd: Field,
}

impl ContAmtCurrField {
    #[must_use]
    pub const fn tag() -> u16 {
        521
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContAmtCurrField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ContAmtCurrField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OwnerTypeField {
    pub fd: Field,
}

impl OwnerTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        522
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OwnerTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OwnerTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartySubIDField {
    pub fd: Field,
}

impl PartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        523
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PartySubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NestedPartyIDField {
    pub fd: Field,
}

impl NestedPartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        524
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NestedPartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NestedPartyIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NestedPartyIDSourceField {
    pub fd: Field,
}

impl NestedPartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        525
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NestedPartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NestedPartyIDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecondaryClOrdIDField {
    pub fd: Field,
}

impl SecondaryClOrdIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        526
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryClOrdIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryClOrdIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecondaryExecIDField {
    pub fd: Field,
}

impl SecondaryExecIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        527
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryExecIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryExecIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrderCapacityField {
    pub fd: Field,
}

impl OrderCapacityField {
    #[must_use]
    pub const fn tag() -> u16 {
        528
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderCapacityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderCapacityField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MassCancelRequestTypeField {
    pub fd: Field,
}

impl MassCancelRequestTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        530
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassCancelRequestTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassCancelRequestTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MassCancelResponseField {
    pub fd: Field,
}

impl MassCancelResponseField {
    #[must_use]
    pub const fn tag() -> u16 {
        531
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassCancelResponseField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassCancelResponseField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MassCancelRejectReasonField {
    pub fd: Field,
}

impl MassCancelRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        532
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassCancelRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassCancelRejectReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotalAffectedOrdersField {
    pub fd: Field,
}

impl TotalAffectedOrdersField {
    #[must_use]
    pub const fn tag() -> u16 {
        533
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotalAffectedOrdersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotalAffectedOrdersField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AffectedOrderIDField {
    pub fd: Field,
}

impl AffectedOrderIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        535
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AffectedOrderIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AffectedOrderIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AffectedSecondaryOrderIDField {
    pub fd: Field,
}

impl AffectedSecondaryOrderIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        536
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AffectedSecondaryOrderIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AffectedSecondaryOrderIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteTypeField {
    pub fd: Field,
}

impl QuoteTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        537
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NestedPartyRoleField {
    pub fd: Field,
}

impl NestedPartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        538
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NestedPartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NestedPartyRoleField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotalAccruedInterestAmtField {
    pub fd: Field,
}

impl TotalAccruedInterestAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        540
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotalAccruedInterestAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotalAccruedInterestAmtField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InstrRegistryField {
    pub fd: Field,
}

impl InstrRegistryField {
    #[must_use]
    pub const fn tag() -> u16 {
        543
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrRegistryField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrRegistryField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CashMarginField {
    pub fd: Field,
}

impl CashMarginField {
    #[must_use]
    pub const fn tag() -> u16 {
        544
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashMarginField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CashMarginField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NestedPartySubIDField {
    pub fd: Field,
}

impl NestedPartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        545
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NestedPartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NestedPartySubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CrossIDField {
    pub fd: Field,
}

impl CrossIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        548
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CrossIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CrossIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CrossTypeField {
    pub fd: Field,
}

impl CrossTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        549
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CrossTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CrossTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CrossPrioritizationField {
    pub fd: Field,
}

impl CrossPrioritizationField {
    #[must_use]
    pub const fn tag() -> u16 {
        550
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CrossPrioritizationField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CrossPrioritizationField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrigCrossIDField {
    pub fd: Field,
}

impl OrigCrossIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        551
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrigCrossIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrigCrossIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UsernameField {
    pub fd: Field,
}

impl UsernameField {
    #[must_use]
    pub const fn tag() -> u16 {
        553
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UsernameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UsernameField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PasswordField {
    pub fd: Field,
}

impl PasswordField {
    #[must_use]
    pub const fn tag() -> u16 {
        554
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PasswordField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PasswordField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegCurrencyField {
    pub fd: Field,
}

impl LegCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        556
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegCurrencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotalNumSecurityTypesField {
    pub fd: Field,
}

impl TotalNumSecurityTypesField {
    #[must_use]
    pub const fn tag() -> u16 {
        557
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotalNumSecurityTypesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotalNumSecurityTypesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityListRequestTypeField {
    pub fd: Field,
}

impl SecurityListRequestTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        559
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityListRequestTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityListRequestTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityRequestResultField {
    pub fd: Field,
}

impl SecurityRequestResultField {
    #[must_use]
    pub const fn tag() -> u16 {
        560
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityRequestResultField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityRequestResultField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoundLotField {
    pub fd: Field,
}

impl RoundLotField {
    #[must_use]
    pub const fn tag() -> u16 {
        561
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RoundLotField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RoundLotField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinTradeVolField {
    pub fd: Field,
}

impl MinTradeVolField {
    #[must_use]
    pub const fn tag() -> u16 {
        562
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MinTradeVolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MinTradeVolField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MultiLegRptTypeReqField {
    pub fd: Field,
}

impl MultiLegRptTypeReqField {
    #[must_use]
    pub const fn tag() -> u16 {
        563
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MultiLegRptTypeReqField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MultiLegRptTypeReqField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegPositionEffectField {
    pub fd: Field,
}

impl LegPositionEffectField {
    #[must_use]
    pub const fn tag() -> u16 {
        564
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegPositionEffectField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegPositionEffectField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegCoveredOrUncoveredField {
    pub fd: Field,
}

impl LegCoveredOrUncoveredField {
    #[must_use]
    pub const fn tag() -> u16 {
        565
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegCoveredOrUncoveredField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegCoveredOrUncoveredField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegPriceField {
    pub fd: Field,
}

impl LegPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        566
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegPriceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradSesStatusRejReasonField {
    pub fd: Field,
}

impl TradSesStatusRejReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        567
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradSesStatusRejReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradSesStatusRejReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradeRequestIDField {
    pub fd: Field,
}

impl TradeRequestIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        568
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeRequestIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeRequestIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradeRequestTypeField {
    pub fd: Field,
}

impl TradeRequestTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        569
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeRequestTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeRequestTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradeReportIDField {
    pub fd: Field,
}

impl TradeReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        571
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeReportIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradeReportRefIDField {
    pub fd: Field,
}

impl TradeReportRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        572
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeReportRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeReportRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MatchStatusField {
    pub fd: Field,
}

impl MatchStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        573
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MatchStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MatchStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MatchTypeField {
    pub fd: Field,
}

impl MatchTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        574
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MatchTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MatchTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoClearingInstructionsField {
    pub fd: Field,
}

impl NoClearingInstructionsField {
    #[must_use]
    pub const fn tag() -> u16 {
        576
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoClearingInstructionsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NoClearingInstructionsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClearingInstructionField {
    pub fd: Field,
}

impl ClearingInstructionField {
    #[must_use]
    pub const fn tag() -> u16 {
        577
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClearingInstructionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ClearingInstructionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradeInputSourceField {
    pub fd: Field,
}

impl TradeInputSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        578
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeInputSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeInputSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradeInputDeviceField {
    pub fd: Field,
}

impl TradeInputDeviceField {
    #[must_use]
    pub const fn tag() -> u16 {
        579
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeInputDeviceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeInputDeviceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccountTypeField {
    pub fd: Field,
}

impl AccountTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        581
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AccountTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AccountTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CustOrderCapacityField {
    pub fd: Field,
}

impl CustOrderCapacityField {
    #[must_use]
    pub const fn tag() -> u16 {
        582
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CustOrderCapacityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CustOrderCapacityField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClOrdLinkIDField {
    pub fd: Field,
}

impl ClOrdLinkIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        583
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClOrdLinkIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ClOrdLinkIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MassStatusReqIDField {
    pub fd: Field,
}

impl MassStatusReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        584
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassStatusReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassStatusReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MassStatusReqTypeField {
    pub fd: Field,
}

impl MassStatusReqTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        585
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassStatusReqTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassStatusReqTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSettlmntTypField {
    pub fd: Field,
}

impl LegSettlmntTypField {
    #[must_use]
    pub const fn tag() -> u16 {
        587
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSettlmntTypField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSettlmntTypField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DayBookingInstField {
    pub fd: Field,
}

impl DayBookingInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        589
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DayBookingInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DayBookingInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BookingUnitField {
    pub fd: Field,
}

impl BookingUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        590
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BookingUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BookingUnitField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PreallocMethodField {
    pub fd: Field,
}

impl PreallocMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        591
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PreallocMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PreallocMethodField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingStateOrProvinceOfIssueField {
    pub fd: Field,
}

impl UnderlyingStateOrProvinceOfIssueField {
    #[must_use]
    pub const fn tag() -> u16 {
        593
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingStateOrProvinceOfIssueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingStateOrProvinceOfIssueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingLocaleOfIssueField {
    pub fd: Field,
}

impl UnderlyingLocaleOfIssueField {
    #[must_use]
    pub const fn tag() -> u16 {
        594
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLocaleOfIssueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLocaleOfIssueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingInstrRegistryField {
    pub fd: Field,
}

impl UnderlyingInstrRegistryField {
    #[must_use]
    pub const fn tag() -> u16 {
        595
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingInstrRegistryField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingInstrRegistryField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegStateOrProvinceOfIssueField {
    pub fd: Field,
}

impl LegStateOrProvinceOfIssueField {
    #[must_use]
    pub const fn tag() -> u16 {
        597
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegStateOrProvinceOfIssueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegStateOrProvinceOfIssueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegLocaleOfIssueField {
    pub fd: Field,
}

impl LegLocaleOfIssueField {
    #[must_use]
    pub const fn tag() -> u16 {
        598
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegLocaleOfIssueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegLocaleOfIssueField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegInstrRegistryField {
    pub fd: Field,
}

impl LegInstrRegistryField {
    #[must_use]
    pub const fn tag() -> u16 {
        599
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegInstrRegistryField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegInstrRegistryField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSymbolField {
    pub fd: Field,
}

impl LegSymbolField {
    #[must_use]
    pub const fn tag() -> u16 {
        600
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSymbolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSymbolField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSymbolSfxField {
    pub fd: Field,
}

impl LegSymbolSfxField {
    #[must_use]
    pub const fn tag() -> u16 {
        601
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSymbolSfxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSymbolSfxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSecurityIDField {
    pub fd: Field,
}

impl LegSecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        602
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSecurityIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSecurityIDSourceField {
    pub fd: Field,
}

impl LegSecurityIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        603
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSecurityIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSecurityIDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSecurityAltIDField {
    pub fd: Field,
}

impl LegSecurityAltIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        605
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSecurityAltIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSecurityAltIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSecurityAltIDSourceField {
    pub fd: Field,
}

impl LegSecurityAltIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        606
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSecurityAltIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSecurityAltIDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegProductField {
    pub fd: Field,
}

impl LegProductField {
    #[must_use]
    pub const fn tag() -> u16 {
        607
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegProductField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegProductField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegCFICodeField {
    pub fd: Field,
}

impl LegCFICodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        608
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegCFICodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegCFICodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSecurityTypeField {
    pub fd: Field,
}

impl LegSecurityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        609
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSecurityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSecurityTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegStrikePriceField {
    pub fd: Field,
}

impl LegStrikePriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        612
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegStrikePriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegStrikePriceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegOptAttributeField {
    pub fd: Field,
}

impl LegOptAttributeField {
    #[must_use]
    pub const fn tag() -> u16 {
        613
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegOptAttributeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegOptAttributeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegIssuerField {
    pub fd: Field,
}

impl LegIssuerField {
    #[must_use]
    pub const fn tag() -> u16 {
        617
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegIssuerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegIssuerField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSecurityDescField {
    pub fd: Field,
}

impl LegSecurityDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        620
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSecurityDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSecurityDescField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegSideField {
    pub fd: Field,
}

impl LegSideField {
    #[must_use]
    pub const fn tag() -> u16 {
        624
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSideField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSideField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TradingSessionSubIDField {
    pub fd: Field,
}

impl TradingSessionSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        625
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradingSessionSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradingSessionSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocTypeField {
    pub fd: Field,
}

impl AllocTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        626
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HopCompIDField {
    pub fd: Field,
}

impl HopCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        628
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HopCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for HopCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MidPxField {
    pub fd: Field,
}

impl MidPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        631
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MidPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MidPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClearingFeeIndicatorField {
    pub fd: Field,
}

impl ClearingFeeIndicatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        635
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClearingFeeIndicatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ClearingFeeIndicatorField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegLastPxField {
    pub fd: Field,
}

impl LegLastPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        637
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegLastPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegLastPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PriorityIndicatorField {
    pub fd: Field,
}

impl PriorityIndicatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        638
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriorityIndicatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PriorityIndicatorField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Price2Field {
    pub fd: Field,
}

impl Price2Field {
    #[must_use]
    pub const fn tag() -> u16 {
        640
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Price2Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Price2Field {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RFQReqIDField {
    pub fd: Field,
}

impl RFQReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        644
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RFQReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RFQReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MktBidPxField {
    pub fd: Field,
}

impl MktBidPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        645
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MktBidPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MktBidPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MktOfferPxField {
    pub fd: Field,
}

impl MktOfferPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        646
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MktOfferPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MktOfferPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinBidSizeField {
    pub fd: Field,
}

impl MinBidSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        647
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MinBidSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MinBidSizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinOfferSizeField {
    pub fd: Field,
}

impl MinOfferSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        648
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MinOfferSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MinOfferSizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteStatusReqIDField {
    pub fd: Field,
}

impl QuoteStatusReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        649
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteStatusReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteStatusReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingLastPxField {
    pub fd: Field,
}

impl UnderlyingLastPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        651
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLastPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLastPxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Decimal(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnderlyingLastQtyField {
    pub fd: Field,
}

impl UnderlyingLastQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        652
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLastQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLastQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LegRefIDField {
    pub fd: Field,
}

impl LegRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        654
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContraLegRefIDField {
    pub fd: Field,
}

impl ContraLegRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        655
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContraLegRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ContraLegRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteRequestRejectReasonField {
    pub fd: Field,
}

impl QuoteRequestRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        658
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteRequestRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteRequestRejectReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SideComplianceIDField {
    pub fd: Field,
}

impl SideComplianceIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        659
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideComplianceIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideComplianceIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
