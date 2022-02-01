use std::fmt::Formatter;
use std::str::FromStr;
use itertools::Itertools;

use rust_decimal::Decimal;

use crate::model::field::{FieldSet, FieldTypeMismatchError};
use crate::model::message_type::UnknownMsgTypeError;
use crate::model::BeginString;
use crate::model::generated::fields::Field;

pub struct FIX50SP2CrackerUtils;
// parse string (35=D) into Field{35, "D"}
impl FIX50SP2CrackerUtils {
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
                    1 | 2 | 3 | 5 | 8 | 10 | 11 | 15 | 17 | 19 | 22 | 23 | 26 | 27 | 35 | 37 | 41 | 48 | 49 | 50 | 55 | 56 | 57 | 58 | 63 | 65 | 66 | 69 | 70 | 72 | 79 | 106 | 107 | 112 | 115 | 116 | 117 | 120 | 128 | 129 | 131 | 138 | 139 | 142 | 143 | 144 | 145 | 147 | 148 | 149 | 161 | 162 | 164 | 167 | 170 | 171 | 196 | 198 | 214 | 217 | 220 | 221 | 222 | 233 | 234 | 235 | 239 | 243 | 250 | 255 | 256 | 257 | 262 | 278 | 280 | 282 | 283 | 284 | 288 | 289 | 299 | 302 | 305 | 306 | 307 | 309 | 310 | 311 | 312 | 318 | 320 | 322 | 324 | 335 | 336 | 337 | 347 | 372 | 375 | 376 | 379 | 390 | 391 | 392 | 400 | 444 | 448 | 455 | 456 | 458 | 459 | 461 | 463 | 466 | 467 | 471 | 472 | 474 | 476 | 478 | 479 | 482 | 488 | 489 | 491 | 493 | 494 | 496 | 498 | 499 | 500 | 501 | 502 | 505 | 508 | 509 | 511 | 513 | 521 | 523 | 524 | 526 | 527 | 535 | 536 | 543 | 545 | 548 | 551 | 553 | 554 | 556 | 568 | 571 | 572 | 574 | 578 | 579 | 583 | 584 | 593 | 594 | 595 | 597 | 598 | 599 | 600 | 601 | 602 | 603 | 605 | 606 | 608 | 609 | 617 | 620 | 625 | 628 | 635 | 644 | 649 | 654 | 655 | 659 | 664 | 671 | 672 | 674 | 675 | 676 | 677 | 678 | 682 | 688 | 689 | 691 | 693 | 699 | 703 | 707 | 710 | 713 | 714 | 716 | 717 | 721 | 726 | 736 | 740 | 755 | 757 | 760 | 761 | 762 | 763 | 764 | 767 | 771 | 772 | 777 | 782 | 785 | 790 | 791 | 793 | 795 | 817 | 818 | 820 | 821 | 822 | 823 | 824 | 825 | 830 | 833 | 848 | 859 | 868 | 872 | 876 | 877 | 878 | 880 | 881 | 888 | 889 | 894 | 902 | 904 | 907 | 908 | 909 | 913 | 914 | 918 | 923 | 925 | 927 | 929 | 930 | 931 | 932 | 933 | 934 | 941 | 942 | 943 | 947 | 949 | 953 | 958 | 960 | 961 | 965 | 966 | 974 | 977 | 979 | 988 | 989 | 990 | 993 | 994 | 996 | 997 | 998 | 999 | 1000 | 1001 | 1003 | 1005 | 1006 | 1007 | 1011 | 1014 | 1019 | 1022 | 1030 | 1033 | 1039 | 1040 | 1041 | 1042 | 1053 | 1055 | 1059 | 1063 | 1080 | 1096 | 1097 | 1098 | 1099 | 1103 | 1104 | 1105 | 1106 | 1113 | 1114 | 1117 | 1121 | 1126 | 1127 | 1128 | 1129 | 1130 | 1131 | 1135 | 1136 | 1137 | 1139 | 1142 | 1151 | 1154 | 1155 | 1160 | 1161 | 1163 | 1166 | 1180 | 1186 | 1191 | 1196 | 1197 | 1211 | 1214 | 1215 | 1216 | 1217 | 1219 | 1220 | 1222 | 1223 | 1227 | 1228 | 1245 | 1247 | 1248 | 1249 | 1250 | 1254 | 1256 | 1257 | 1259 | 1260 | 1262 | 1269 | 1271 | 1275 | 1279 | 1284 | 1291 | 1293 | 1294 | 1297 | 1300 | 1314 | 1315 | 1318 | 1319 | 1325 | 1326 | 1328 | 1330 | 1331 | 1332 | 1333 | 1335 | 1336 | 1337 | 1338 | 1341 | 1344 | 1346 | 1353 | 1355 | 1356 | 1363 | 1366 | 1367 | 1369 | 1371 | 1372 | 1392 | 1393 | 1394 | 1396 | 1408 | 1412 | 1415 | 1421 | 1424 | 1427 | 1433 | 1448 | 1449 | 1450 | 1453 | 1454 | 1462 | 1465 | 1466 | 1467 | 1472 | 1476 | 1497 | 1500 | 1501 => Ok(Field::String(tag, s_value.to_string())),

                    4 | 13 | 21 | 25 | 28 | 29 | 39 | 40 | 54 | 59 | 61 | 71 | 77 | 81 | 94 | 104 | 127 | 150 | 156 | 160 | 163 | 165 | 206 | 263 | 269 | 274 | 279 | 281 | 285 | 317 | 374 | 385 | 388 | 418 | 419 | 433 | 434 | 442 | 447 | 468 | 480 | 481 | 484 | 497 | 506 | 514 | 517 | 525 | 528 | 530 | 531 | 544 | 564 | 573 | 587 | 589 | 590 | 591 | 613 | 624 | 695 | 744 | 747 | 758 | 783 | 787 | 950 | 980 | 1015 | 1036 | 1046 | 1047 | 1048 | 1049 | 1050 | 1060 | 1081 | 1083 | 1084 | 1092 | 1093 | 1100 | 1101 | 1107 | 1108 | 1109 | 1111 | 1115 | 1118 | 1123 | 1124 | 1133 | 1162 | 1164 | 1193 | 1255 | 1265 | 1299 | 1308 | 1317 | 1324 | 1327 | 1391 | 1395 | 1416 | 1430 | 1463 => Ok(Field::Char(tag, str::parse::<char>(s_value).unwrap())),

                    6 | 31 | 44 | 99 | 132 | 133 | 140 | 153 | 188 | 190 | 194 | 202 | 260 | 270 | 316 | 332 | 333 | 366 | 426 | 566 | 612 | 631 | 637 | 640 | 645 | 646 | 651 | 662 | 669 | 679 | 681 | 684 | 697 | 730 | 732 | 734 | 799 | 810 | 839 | 845 | 860 | 861 | 867 | 882 | 883 | 991 | 1025 | 1095 | 1102 | 1110 | 1148 | 1149 | 1150 | 1199 | 1200 | 1202 | 1203 | 1206 | 1207 | 1208 | 1221 | 1230 | 1240 | 1261 | 1290 | 1321 | 1322 | 1340 | 1364 | 1486 => Ok(Field::Decimal(tag, Decimal::from_str(s_value).unwrap())),

                    12 | 14 | 32 | 38 | 53 | 67 | 68 | 74 | 80 | 82 | 83 | 84 | 87 | 88 | 98 | 102 | 103 | 108 | 110 | 111 | 118 | 119 | 134 | 135 | 137 | 151 | 152 | 154 | 157 | 159 | 169 | 172 | 192 | 197 | 201 | 203 | 209 | 210 | 216 | 226 | 237 | 238 | 244 | 251 | 264 | 265 | 271 | 287 | 290 | 293 | 294 | 297 | 298 | 300 | 301 | 303 | 304 | 315 | 321 | 323 | 326 | 327 | 330 | 331 | 334 | 338 | 339 | 340 | 346 | 368 | 371 | 373 | 378 | 380 | 381 | 387 | 393 | 394 | 395 | 396 | 397 | 399 | 401 | 404 | 406 | 408 | 409 | 412 | 414 | 415 | 416 | 417 | 422 | 423 | 424 | 425 | 427 | 429 | 430 | 431 | 437 | 441 | 452 | 460 | 462 | 477 | 487 | 492 | 495 | 507 | 519 | 522 | 532 | 533 | 537 | 538 | 540 | 549 | 550 | 557 | 559 | 560 | 561 | 562 | 563 | 565 | 567 | 569 | 577 | 581 | 582 | 585 | 607 | 626 | 638 | 647 | 648 | 652 | 658 | 660 | 661 | 663 | 665 | 666 | 668 | 673 | 680 | 685 | 686 | 687 | 690 | 692 | 694 | 698 | 704 | 705 | 706 | 708 | 709 | 712 | 718 | 722 | 723 | 724 | 725 | 727 | 728 | 729 | 731 | 733 | 737 | 738 | 741 | 742 | 745 | 746 | 748 | 749 | 750 | 751 | 752 | 759 | 766 | 770 | 773 | 774 | 775 | 776 | 780 | 784 | 786 | 788 | 792 | 794 | 796 | 798 | 800 | 803 | 805 | 807 | 808 | 812 | 813 | 814 | 815 | 819 | 826 | 827 | 828 | 829 | 832 | 835 | 836 | 837 | 838 | 840 | 841 | 842 | 843 | 844 | 846 | 847 | 851 | 853 | 854 | 855 | 856 | 857 | 858 | 863 | 865 | 871 | 875 | 879 | 884 | 885 | 886 | 890 | 891 | 892 | 895 | 896 | 899 | 900 | 901 | 903 | 905 | 906 | 910 | 911 | 919 | 920 | 921 | 922 | 924 | 926 | 928 | 935 | 937 | 939 | 940 | 944 | 945 | 946 | 951 | 954 | 959 | 963 | 964 | 970 | 971 | 973 | 975 | 982 | 983 | 985 | 986 | 992 | 1002 | 1008 | 1009 | 1013 | 1020 | 1021 | 1023 | 1024 | 1032 | 1034 | 1037 | 1038 | 1043 | 1044 | 1051 | 1054 | 1056 | 1061 | 1064 | 1070 | 1072 | 1074 | 1075 | 1082 | 1085 | 1086 | 1087 | 1088 | 1089 | 1090 | 1094 | 1112 | 1119 | 1122 | 1138 | 1140 | 1144 | 1146 | 1147 | 1152 | 1153 | 1156 | 1157 | 1159 | 1167 | 1168 | 1169 | 1170 | 1172 | 1173 | 1174 | 1176 | 1178 | 1179 | 1192 | 1194 | 1195 | 1198 | 1209 | 1210 | 1224 | 1225 | 1229 | 1231 | 1246 | 1268 | 1270 | 1273 | 1274 | 1287 | 1295 | 1298 | 1302 | 1303 | 1304 | 1305 | 1306 | 1313 | 1316 | 1320 | 1323 | 1343 | 1347 | 1348 | 1349 | 1354 | 1358 | 1361 | 1365 | 1368 | 1373 | 1374 | 1375 | 1376 | 1377 | 1378 | 1385 | 1386 | 1388 | 1390 | 1400 | 1406 | 1407 | 1409 | 1411 | 1417 | 1418 | 1419 | 1420 | 1422 | 1423 | 1425 | 1426 | 1428 | 1429 | 1431 | 1432 | 1434 | 1435 | 1436 | 1437 | 1438 | 1439 | 1440 | 1441 | 1442 | 1443 | 1444 | 1446 | 1447 | 1464 | 1470 | 1471 | 1473 | 1477 | 1478 | 1479 | 1481 | 1482 | 1484 | 1485 | 1487 | 1489 | 1490 | 1498 | 1502 | 1503 | 1617 => Ok(Field::Int(tag, str::parse::<i32>(s_value).unwrap())),

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
pub struct IOIIDField {
    pub fd: Field,
}

impl IOIIDField {
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

impl std::fmt::Display for IOIIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOIIDField {
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
pub struct SettlTypeField {
    pub fd: Field,
}

impl SettlTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        63
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlTypeField {
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
pub struct AvgPxPrecisionField {
    pub fd: Field,
}

impl AvgPxPrecisionField {
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

impl std::fmt::Display for AvgPxPrecisionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AvgPxPrecisionField {
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
impl TryFrom<&Field> for NoRptsField {
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

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
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
            Field::String(_, _) => Ok(Self { fd: value.clone() }),
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
impl TryFrom<&Field> for PutOrCallField {
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
pub struct TotNoQuoteEntriesField {
    pub fd: Field,
}

impl TotNoQuoteEntriesField {
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

impl std::fmt::Display for TotNoQuoteEntriesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNoQuoteEntriesField {
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
impl TryFrom<&Field> for UnderlyingCurrencyField {
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
pub struct HaltReasonIntField {
    pub fd: Field,
}

impl HaltReasonIntField {
    #[must_use]
    pub const fn tag() -> u16 {
        327
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HaltReasonIntField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for HaltReasonIntField {
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
pub struct TotNoRelatedSymField {
    pub fd: Field,
}

impl TotNoRelatedSymField {
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

impl std::fmt::Display for TotNoRelatedSymField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNoRelatedSymField {
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
pub struct BidTradeTypeField {
    pub fd: Field,
}

impl BidTradeTypeField {
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

impl std::fmt::Display for BidTradeTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BidTradeTypeField {
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

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
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
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
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
pub struct CardIssNumField {
    pub fd: Field,
}

impl CardIssNumField {
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

impl std::fmt::Display for CardIssNumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CardIssNumField {
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
pub struct CashDistribAgentAcctNameField {
    pub fd: Field,
}

impl CashDistribAgentAcctNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        502
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashDistribAgentAcctNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CashDistribAgentAcctNameField {
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
pub struct RegistDtlsField {
    pub fd: Field,
}

impl RegistDtlsField {
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

impl std::fmt::Display for RegistDtlsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RegistDtlsField {
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

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
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
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
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
pub struct TotNoSecurityTypesField {
    pub fd: Field,
}

impl TotNoSecurityTypesField {
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

impl std::fmt::Display for TotNoSecurityTypesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNoSecurityTypesField {
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
pub struct LegSettlTypeField {
    pub fd: Field,
}

impl LegSettlTypeField {
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

impl std::fmt::Display for LegSettlTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSettlTypeField {
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AcctIDSourceField {
    pub fd: Field,
}

impl AcctIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        660
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AcctIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AcctIDSourceField {
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
pub struct AllocAcctIDSourceField {
    pub fd: Field,
}

impl AllocAcctIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        661
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocAcctIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocAcctIDSourceField {
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
pub struct BenchmarkPriceField {
    pub fd: Field,
}

impl BenchmarkPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        662
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BenchmarkPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BenchmarkPriceField {
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
pub struct BenchmarkPriceTypeField {
    pub fd: Field,
}

impl BenchmarkPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        663
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BenchmarkPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BenchmarkPriceTypeField {
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
pub struct ConfirmIDField {
    pub fd: Field,
}

impl ConfirmIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        664
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ConfirmIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ConfirmIDField {
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
pub struct ConfirmStatusField {
    pub fd: Field,
}

impl ConfirmStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        665
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ConfirmStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ConfirmStatusField {
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
pub struct ConfirmTransTypeField {
    pub fd: Field,
}

impl ConfirmTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        666
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ConfirmTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ConfirmTransTypeField {
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
pub struct DeliveryFormField {
    pub fd: Field,
}

impl DeliveryFormField {
    #[must_use]
    pub const fn tag() -> u16 {
        668
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeliveryFormField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DeliveryFormField {
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
pub struct LastParPxField {
    pub fd: Field,
}

impl LastParPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        669
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastParPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LastParPxField {
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
pub struct LegAllocAccountField {
    pub fd: Field,
}

impl LegAllocAccountField {
    #[must_use]
    pub const fn tag() -> u16 {
        671
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegAllocAccountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegAllocAccountField {
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
pub struct LegIndividualAllocIDField {
    pub fd: Field,
}

impl LegIndividualAllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        672
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegIndividualAllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegIndividualAllocIDField {
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
pub struct LegAllocQtyField {
    pub fd: Field,
}

impl LegAllocQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        673
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegAllocQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegAllocQtyField {
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
pub struct LegAllocAcctIDSourceField {
    pub fd: Field,
}

impl LegAllocAcctIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        674
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegAllocAcctIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegAllocAcctIDSourceField {
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
pub struct LegSettlCurrencyField {
    pub fd: Field,
}

impl LegSettlCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        675
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSettlCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSettlCurrencyField {
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
pub struct LegBenchmarkCurveCurrencyField {
    pub fd: Field,
}

impl LegBenchmarkCurveCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        676
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegBenchmarkCurveCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegBenchmarkCurveCurrencyField {
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
pub struct LegBenchmarkCurveNameField {
    pub fd: Field,
}

impl LegBenchmarkCurveNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        677
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegBenchmarkCurveNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegBenchmarkCurveNameField {
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
pub struct LegBenchmarkCurvePointField {
    pub fd: Field,
}

impl LegBenchmarkCurvePointField {
    #[must_use]
    pub const fn tag() -> u16 {
        678
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegBenchmarkCurvePointField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegBenchmarkCurvePointField {
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
pub struct LegBenchmarkPriceField {
    pub fd: Field,
}

impl LegBenchmarkPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        679
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegBenchmarkPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegBenchmarkPriceField {
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
pub struct LegBenchmarkPriceTypeField {
    pub fd: Field,
}

impl LegBenchmarkPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        680
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegBenchmarkPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegBenchmarkPriceTypeField {
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
pub struct LegBidPxField {
    pub fd: Field,
}

impl LegBidPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        681
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegBidPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegBidPxField {
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
pub struct LegIOIQtyField {
    pub fd: Field,
}

impl LegIOIQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        682
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegIOIQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegIOIQtyField {
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
pub struct LegOfferPxField {
    pub fd: Field,
}

impl LegOfferPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        684
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegOfferPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegOfferPxField {
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
pub struct LegOrderQtyField {
    pub fd: Field,
}

impl LegOrderQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        685
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegOrderQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegOrderQtyField {
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
pub struct LegPriceTypeField {
    pub fd: Field,
}

impl LegPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        686
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegPriceTypeField {
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
pub struct LegQtyField {
    pub fd: Field,
}

impl LegQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        687
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegQtyField {
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
pub struct LegStipulationTypeField {
    pub fd: Field,
}

impl LegStipulationTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        688
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegStipulationTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegStipulationTypeField {
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
pub struct LegStipulationValueField {
    pub fd: Field,
}

impl LegStipulationValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        689
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegStipulationValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegStipulationValueField {
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
pub struct LegSwapTypeField {
    pub fd: Field,
}

impl LegSwapTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        690
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSwapTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSwapTypeField {
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
pub struct PoolField {
    pub fd: Field,
}

impl PoolField {
    #[must_use]
    pub const fn tag() -> u16 {
        691
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PoolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PoolField {
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
pub struct QuotePriceTypeField {
    pub fd: Field,
}

impl QuotePriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        692
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuotePriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuotePriceTypeField {
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
pub struct QuoteRespIDField {
    pub fd: Field,
}

impl QuoteRespIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        693
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteRespIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteRespIDField {
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
pub struct QuoteRespTypeField {
    pub fd: Field,
}

impl QuoteRespTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        694
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteRespTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteRespTypeField {
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
pub struct QuoteQualifierField {
    pub fd: Field,
}

impl QuoteQualifierField {
    #[must_use]
    pub const fn tag() -> u16 {
        695
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteQualifierField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteQualifierField {
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
pub struct YieldRedemptionPriceField {
    pub fd: Field,
}

impl YieldRedemptionPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        697
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for YieldRedemptionPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for YieldRedemptionPriceField {
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
pub struct YieldRedemptionPriceTypeField {
    pub fd: Field,
}

impl YieldRedemptionPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        698
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for YieldRedemptionPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for YieldRedemptionPriceTypeField {
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
pub struct BenchmarkSecurityIDField {
    pub fd: Field,
}

impl BenchmarkSecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        699
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BenchmarkSecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BenchmarkSecurityIDField {
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
pub struct PosTypeField {
    pub fd: Field,
}

impl PosTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        703
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosTypeField {
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
pub struct LongQtyField {
    pub fd: Field,
}

impl LongQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        704
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LongQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LongQtyField {
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
pub struct ShortQtyField {
    pub fd: Field,
}

impl ShortQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        705
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ShortQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ShortQtyField {
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
pub struct PosQtyStatusField {
    pub fd: Field,
}

impl PosQtyStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        706
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosQtyStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosQtyStatusField {
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
pub struct PosAmtTypeField {
    pub fd: Field,
}

impl PosAmtTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        707
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosAmtTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosAmtTypeField {
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
pub struct PosAmtField {
    pub fd: Field,
}

impl PosAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        708
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosAmtField {
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
pub struct PosTransTypeField {
    pub fd: Field,
}

impl PosTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        709
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosTransTypeField {
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
pub struct PosReqIDField {
    pub fd: Field,
}

impl PosReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        710
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosReqIDField {
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
pub struct PosMaintActionField {
    pub fd: Field,
}

impl PosMaintActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        712
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosMaintActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosMaintActionField {
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
pub struct OrigPosReqRefIDField {
    pub fd: Field,
}

impl OrigPosReqRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        713
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrigPosReqRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrigPosReqRefIDField {
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
pub struct PosMaintRptRefIDField {
    pub fd: Field,
}

impl PosMaintRptRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        714
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosMaintRptRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosMaintRptRefIDField {
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
pub struct SettlSessIDField {
    pub fd: Field,
}

impl SettlSessIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        716
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlSessIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlSessIDField {
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
pub struct SettlSessSubIDField {
    pub fd: Field,
}

impl SettlSessSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        717
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlSessSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlSessSubIDField {
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
pub struct AdjustmentTypeField {
    pub fd: Field,
}

impl AdjustmentTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        718
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdjustmentTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AdjustmentTypeField {
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
pub struct PosMaintRptIDField {
    pub fd: Field,
}

impl PosMaintRptIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        721
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosMaintRptIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosMaintRptIDField {
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
pub struct PosMaintStatusField {
    pub fd: Field,
}

impl PosMaintStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        722
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosMaintStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosMaintStatusField {
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
pub struct PosMaintResultField {
    pub fd: Field,
}

impl PosMaintResultField {
    #[must_use]
    pub const fn tag() -> u16 {
        723
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosMaintResultField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosMaintResultField {
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
pub struct PosReqTypeField {
    pub fd: Field,
}

impl PosReqTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        724
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosReqTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosReqTypeField {
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
pub struct ResponseTransportTypeField {
    pub fd: Field,
}

impl ResponseTransportTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        725
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ResponseTransportTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ResponseTransportTypeField {
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
pub struct ResponseDestinationField {
    pub fd: Field,
}

impl ResponseDestinationField {
    #[must_use]
    pub const fn tag() -> u16 {
        726
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ResponseDestinationField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ResponseDestinationField {
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
pub struct TotalNumPosReportsField {
    pub fd: Field,
}

impl TotalNumPosReportsField {
    #[must_use]
    pub const fn tag() -> u16 {
        727
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotalNumPosReportsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotalNumPosReportsField {
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
pub struct PosReqResultField {
    pub fd: Field,
}

impl PosReqResultField {
    #[must_use]
    pub const fn tag() -> u16 {
        728
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosReqResultField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosReqResultField {
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
pub struct PosReqStatusField {
    pub fd: Field,
}

impl PosReqStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        729
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PosReqStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PosReqStatusField {
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
pub struct SettlPriceField {
    pub fd: Field,
}

impl SettlPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        730
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlPriceField {
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
pub struct SettlPriceTypeField {
    pub fd: Field,
}

impl SettlPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        731
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlPriceTypeField {
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
pub struct UnderlyingSettlPriceField {
    pub fd: Field,
}

impl UnderlyingSettlPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        732
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSettlPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSettlPriceField {
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
pub struct UnderlyingSettlPriceTypeField {
    pub fd: Field,
}

impl UnderlyingSettlPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        733
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSettlPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSettlPriceTypeField {
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
pub struct PriorSettlPriceField {
    pub fd: Field,
}

impl PriorSettlPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        734
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriorSettlPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PriorSettlPriceField {
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
pub struct AllocSettlCurrencyField {
    pub fd: Field,
}

impl AllocSettlCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        736
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocSettlCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocSettlCurrencyField {
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
pub struct AllocSettlCurrAmtField {
    pub fd: Field,
}

impl AllocSettlCurrAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        737
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocSettlCurrAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocSettlCurrAmtField {
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
pub struct InterestAtMaturityField {
    pub fd: Field,
}

impl InterestAtMaturityField {
    #[must_use]
    pub const fn tag() -> u16 {
        738
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InterestAtMaturityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InterestAtMaturityField {
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
pub struct LegPoolField {
    pub fd: Field,
}

impl LegPoolField {
    #[must_use]
    pub const fn tag() -> u16 {
        740
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegPoolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegPoolField {
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
pub struct AllocInterestAtMaturityField {
    pub fd: Field,
}

impl AllocInterestAtMaturityField {
    #[must_use]
    pub const fn tag() -> u16 {
        741
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocInterestAtMaturityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocInterestAtMaturityField {
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
pub struct AllocAccruedInterestAmtField {
    pub fd: Field,
}

impl AllocAccruedInterestAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        742
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocAccruedInterestAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocAccruedInterestAmtField {
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
pub struct AssignmentMethodField {
    pub fd: Field,
}

impl AssignmentMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        744
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AssignmentMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AssignmentMethodField {
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
pub struct AssignmentUnitField {
    pub fd: Field,
}

impl AssignmentUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        745
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AssignmentUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AssignmentUnitField {
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
pub struct OpenInterestField {
    pub fd: Field,
}

impl OpenInterestField {
    #[must_use]
    pub const fn tag() -> u16 {
        746
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OpenInterestField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OpenInterestField {
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
pub struct ExerciseMethodField {
    pub fd: Field,
}

impl ExerciseMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        747
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExerciseMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExerciseMethodField {
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
pub struct TotNumTradeReportsField {
    pub fd: Field,
}

impl TotNumTradeReportsField {
    #[must_use]
    pub const fn tag() -> u16 {
        748
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNumTradeReportsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNumTradeReportsField {
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
pub struct TradeRequestResultField {
    pub fd: Field,
}

impl TradeRequestResultField {
    #[must_use]
    pub const fn tag() -> u16 {
        749
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeRequestResultField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeRequestResultField {
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
pub struct TradeRequestStatusField {
    pub fd: Field,
}

impl TradeRequestStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        750
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeRequestStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeRequestStatusField {
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
pub struct TradeReportRejectReasonField {
    pub fd: Field,
}

impl TradeReportRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        751
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeReportRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeReportRejectReasonField {
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
pub struct SideMultiLegReportingTypeField {
    pub fd: Field,
}

impl SideMultiLegReportingTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        752
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideMultiLegReportingTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideMultiLegReportingTypeField {
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
pub struct AllocReportIDField {
    pub fd: Field,
}

impl AllocReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        755
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocReportIDField {
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
pub struct Nested2PartyIDField {
    pub fd: Field,
}

impl Nested2PartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        757
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested2PartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested2PartyIDField {
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
pub struct Nested2PartyIDSourceField {
    pub fd: Field,
}

impl Nested2PartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        758
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested2PartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested2PartyIDSourceField {
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
pub struct Nested2PartyRoleField {
    pub fd: Field,
}

impl Nested2PartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        759
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested2PartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested2PartyRoleField {
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
pub struct Nested2PartySubIDField {
    pub fd: Field,
}

impl Nested2PartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        760
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested2PartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested2PartySubIDField {
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
pub struct BenchmarkSecurityIDSourceField {
    pub fd: Field,
}

impl BenchmarkSecurityIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        761
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BenchmarkSecurityIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BenchmarkSecurityIDSourceField {
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
pub struct SecuritySubTypeField {
    pub fd: Field,
}

impl SecuritySubTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        762
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecuritySubTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecuritySubTypeField {
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
pub struct UnderlyingSecuritySubTypeField {
    pub fd: Field,
}

impl UnderlyingSecuritySubTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        763
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSecuritySubTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSecuritySubTypeField {
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
pub struct LegSecuritySubTypeField {
    pub fd: Field,
}

impl LegSecuritySubTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        764
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegSecuritySubTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegSecuritySubTypeField {
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
pub struct AllowableOneSidednessValueField {
    pub fd: Field,
}

impl AllowableOneSidednessValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        766
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllowableOneSidednessValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllowableOneSidednessValueField {
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
pub struct AllowableOneSidednessCurrField {
    pub fd: Field,
}

impl AllowableOneSidednessCurrField {
    #[must_use]
    pub const fn tag() -> u16 {
        767
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllowableOneSidednessCurrField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllowableOneSidednessCurrField {
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
pub struct TrdRegTimestampTypeField {
    pub fd: Field,
}

impl TrdRegTimestampTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        770
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TrdRegTimestampTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TrdRegTimestampTypeField {
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
pub struct TrdRegTimestampOriginField {
    pub fd: Field,
}

impl TrdRegTimestampOriginField {
    #[must_use]
    pub const fn tag() -> u16 {
        771
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TrdRegTimestampOriginField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TrdRegTimestampOriginField {
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
pub struct ConfirmRefIDField {
    pub fd: Field,
}

impl ConfirmRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        772
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ConfirmRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ConfirmRefIDField {
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
pub struct ConfirmTypeField {
    pub fd: Field,
}

impl ConfirmTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        773
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ConfirmTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ConfirmTypeField {
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
pub struct ConfirmRejReasonField {
    pub fd: Field,
}

impl ConfirmRejReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        774
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ConfirmRejReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ConfirmRejReasonField {
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
pub struct BookingTypeField {
    pub fd: Field,
}

impl BookingTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        775
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BookingTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BookingTypeField {
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
pub struct IndividualAllocRejCodeField {
    pub fd: Field,
}

impl IndividualAllocRejCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        776
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IndividualAllocRejCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IndividualAllocRejCodeField {
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
pub struct SettlInstMsgIDField {
    pub fd: Field,
}

impl SettlInstMsgIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        777
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstMsgIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlInstMsgIDField {
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
pub struct AllocSettlInstTypeField {
    pub fd: Field,
}

impl AllocSettlInstTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        780
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocSettlInstTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocSettlInstTypeField {
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
pub struct SettlPartyIDField {
    pub fd: Field,
}

impl SettlPartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        782
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlPartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlPartyIDField {
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
pub struct SettlPartyIDSourceField {
    pub fd: Field,
}

impl SettlPartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        783
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlPartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlPartyIDSourceField {
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
pub struct SettlPartyRoleField {
    pub fd: Field,
}

impl SettlPartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        784
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlPartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlPartyRoleField {
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
pub struct SettlPartySubIDField {
    pub fd: Field,
}

impl SettlPartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        785
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlPartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlPartySubIDField {
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
pub struct SettlPartySubIDTypeField {
    pub fd: Field,
}

impl SettlPartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        786
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlPartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlPartySubIDTypeField {
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
pub struct DlvyInstTypeField {
    pub fd: Field,
}

impl DlvyInstTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        787
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DlvyInstTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DlvyInstTypeField {
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
pub struct TerminationTypeField {
    pub fd: Field,
}

impl TerminationTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        788
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TerminationTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TerminationTypeField {
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
pub struct OrdStatusReqIDField {
    pub fd: Field,
}

impl OrdStatusReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        790
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrdStatusReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrdStatusReqIDField {
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
pub struct SettlInstReqIDField {
    pub fd: Field,
}

impl SettlInstReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        791
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlInstReqIDField {
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
pub struct SettlInstReqRejCodeField {
    pub fd: Field,
}

impl SettlInstReqRejCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        792
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlInstReqRejCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlInstReqRejCodeField {
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
pub struct SecondaryAllocIDField {
    pub fd: Field,
}

impl SecondaryAllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        793
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryAllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryAllocIDField {
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
pub struct AllocReportTypeField {
    pub fd: Field,
}

impl AllocReportTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        794
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocReportTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocReportTypeField {
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
pub struct AllocReportRefIDField {
    pub fd: Field,
}

impl AllocReportRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        795
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocReportRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocReportRefIDField {
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
pub struct AllocCancReplaceReasonField {
    pub fd: Field,
}

impl AllocCancReplaceReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        796
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocCancReplaceReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocCancReplaceReasonField {
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
pub struct AllocAccountTypeField {
    pub fd: Field,
}

impl AllocAccountTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        798
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocAccountTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocAccountTypeField {
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
pub struct OrderAvgPxField {
    pub fd: Field,
}

impl OrderAvgPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        799
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderAvgPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderAvgPxField {
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
pub struct OrderBookingQtyField {
    pub fd: Field,
}

impl OrderBookingQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        800
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderBookingQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderBookingQtyField {
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
pub struct PartySubIDTypeField {
    pub fd: Field,
}

impl PartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        803
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PartySubIDTypeField {
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
pub struct NestedPartySubIDTypeField {
    pub fd: Field,
}

impl NestedPartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        805
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NestedPartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NestedPartySubIDTypeField {
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
pub struct Nested2PartySubIDTypeField {
    pub fd: Field,
}

impl Nested2PartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        807
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested2PartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested2PartySubIDTypeField {
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
pub struct AllocIntermedReqTypeField {
    pub fd: Field,
}

impl AllocIntermedReqTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        808
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocIntermedReqTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocIntermedReqTypeField {
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
pub struct UnderlyingPxField {
    pub fd: Field,
}

impl UnderlyingPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        810
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingPxField {
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
pub struct ApplQueueMaxField {
    pub fd: Field,
}

impl ApplQueueMaxField {
    #[must_use]
    pub const fn tag() -> u16 {
        812
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplQueueMaxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplQueueMaxField {
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
pub struct ApplQueueDepthField {
    pub fd: Field,
}

impl ApplQueueDepthField {
    #[must_use]
    pub const fn tag() -> u16 {
        813
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplQueueDepthField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplQueueDepthField {
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
pub struct ApplQueueResolutionField {
    pub fd: Field,
}

impl ApplQueueResolutionField {
    #[must_use]
    pub const fn tag() -> u16 {
        814
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplQueueResolutionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplQueueResolutionField {
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
pub struct ApplQueueActionField {
    pub fd: Field,
}

impl ApplQueueActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        815
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplQueueActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplQueueActionField {
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
pub struct AltMDSourceIDField {
    pub fd: Field,
}

impl AltMDSourceIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        817
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AltMDSourceIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AltMDSourceIDField {
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
pub struct SecondaryTradeReportIDField {
    pub fd: Field,
}

impl SecondaryTradeReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        818
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryTradeReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryTradeReportIDField {
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
pub struct AvgPxIndicatorField {
    pub fd: Field,
}

impl AvgPxIndicatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        819
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AvgPxIndicatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AvgPxIndicatorField {
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
pub struct TradeLinkIDField {
    pub fd: Field,
}

impl TradeLinkIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        820
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeLinkIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeLinkIDField {
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
pub struct OrderInputDeviceField {
    pub fd: Field,
}

impl OrderInputDeviceField {
    #[must_use]
    pub const fn tag() -> u16 {
        821
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderInputDeviceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderInputDeviceField {
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
pub struct UnderlyingTradingSessionIDField {
    pub fd: Field,
}

impl UnderlyingTradingSessionIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        822
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingTradingSessionIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingTradingSessionIDField {
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
pub struct UnderlyingTradingSessionSubIDField {
    pub fd: Field,
}

impl UnderlyingTradingSessionSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        823
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingTradingSessionSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingTradingSessionSubIDField {
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
pub struct TradeLegRefIDField {
    pub fd: Field,
}

impl TradeLegRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        824
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeLegRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeLegRefIDField {
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
pub struct ExchangeRuleField {
    pub fd: Field,
}

impl ExchangeRuleField {
    #[must_use]
    pub const fn tag() -> u16 {
        825
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExchangeRuleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExchangeRuleField {
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
pub struct TradeAllocIndicatorField {
    pub fd: Field,
}

impl TradeAllocIndicatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        826
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeAllocIndicatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeAllocIndicatorField {
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
pub struct ExpirationCycleField {
    pub fd: Field,
}

impl ExpirationCycleField {
    #[must_use]
    pub const fn tag() -> u16 {
        827
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExpirationCycleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExpirationCycleField {
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
pub struct TrdTypeField {
    pub fd: Field,
}

impl TrdTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        828
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TrdTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TrdTypeField {
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
pub struct TrdSubTypeField {
    pub fd: Field,
}

impl TrdSubTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        829
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TrdSubTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TrdSubTypeField {
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
pub struct TransferReasonField {
    pub fd: Field,
}

impl TransferReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        830
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TransferReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TransferReasonField {
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
pub struct TotNumAssignmentReportsField {
    pub fd: Field,
}

impl TotNumAssignmentReportsField {
    #[must_use]
    pub const fn tag() -> u16 {
        832
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNumAssignmentReportsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNumAssignmentReportsField {
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
pub struct AsgnRptIDField {
    pub fd: Field,
}

impl AsgnRptIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        833
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AsgnRptIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AsgnRptIDField {
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
pub struct PegMoveTypeField {
    pub fd: Field,
}

impl PegMoveTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        835
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegMoveTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegMoveTypeField {
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
pub struct PegOffsetTypeField {
    pub fd: Field,
}

impl PegOffsetTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        836
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegOffsetTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegOffsetTypeField {
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
pub struct PegLimitTypeField {
    pub fd: Field,
}

impl PegLimitTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        837
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegLimitTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegLimitTypeField {
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
pub struct PegRoundDirectionField {
    pub fd: Field,
}

impl PegRoundDirectionField {
    #[must_use]
    pub const fn tag() -> u16 {
        838
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegRoundDirectionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegRoundDirectionField {
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
pub struct PeggedPriceField {
    pub fd: Field,
}

impl PeggedPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        839
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PeggedPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PeggedPriceField {
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
pub struct PegScopeField {
    pub fd: Field,
}

impl PegScopeField {
    #[must_use]
    pub const fn tag() -> u16 {
        840
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegScopeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegScopeField {
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
pub struct DiscretionMoveTypeField {
    pub fd: Field,
}

impl DiscretionMoveTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        841
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DiscretionMoveTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DiscretionMoveTypeField {
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
pub struct DiscretionOffsetTypeField {
    pub fd: Field,
}

impl DiscretionOffsetTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        842
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DiscretionOffsetTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DiscretionOffsetTypeField {
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
pub struct DiscretionLimitTypeField {
    pub fd: Field,
}

impl DiscretionLimitTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        843
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DiscretionLimitTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DiscretionLimitTypeField {
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
pub struct DiscretionRoundDirectionField {
    pub fd: Field,
}

impl DiscretionRoundDirectionField {
    #[must_use]
    pub const fn tag() -> u16 {
        844
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DiscretionRoundDirectionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DiscretionRoundDirectionField {
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
pub struct DiscretionPriceField {
    pub fd: Field,
}

impl DiscretionPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        845
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DiscretionPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DiscretionPriceField {
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
pub struct DiscretionScopeField {
    pub fd: Field,
}

impl DiscretionScopeField {
    #[must_use]
    pub const fn tag() -> u16 {
        846
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DiscretionScopeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DiscretionScopeField {
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
pub struct TargetStrategyField {
    pub fd: Field,
}

impl TargetStrategyField {
    #[must_use]
    pub const fn tag() -> u16 {
        847
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetStrategyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TargetStrategyField {
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
pub struct TargetStrategyParametersField {
    pub fd: Field,
}

impl TargetStrategyParametersField {
    #[must_use]
    pub const fn tag() -> u16 {
        848
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetStrategyParametersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TargetStrategyParametersField {
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
pub struct LastLiquidityIndField {
    pub fd: Field,
}

impl LastLiquidityIndField {
    #[must_use]
    pub const fn tag() -> u16 {
        851
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastLiquidityIndField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LastLiquidityIndField {
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
pub struct ShortSaleReasonField {
    pub fd: Field,
}

impl ShortSaleReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        853
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ShortSaleReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ShortSaleReasonField {
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
pub struct QtyTypeField {
    pub fd: Field,
}

impl QtyTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        854
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QtyTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QtyTypeField {
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
pub struct SecondaryTrdTypeField {
    pub fd: Field,
}

impl SecondaryTrdTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        855
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryTrdTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryTrdTypeField {
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
pub struct TradeReportTypeField {
    pub fd: Field,
}

impl TradeReportTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        856
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeReportTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeReportTypeField {
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
pub struct AllocNoOrdersTypeField {
    pub fd: Field,
}

impl AllocNoOrdersTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        857
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocNoOrdersTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocNoOrdersTypeField {
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
pub struct SharedCommissionField {
    pub fd: Field,
}

impl SharedCommissionField {
    #[must_use]
    pub const fn tag() -> u16 {
        858
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SharedCommissionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SharedCommissionField {
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
pub struct ConfirmReqIDField {
    pub fd: Field,
}

impl ConfirmReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        859
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ConfirmReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ConfirmReqIDField {
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
pub struct AvgParPxField {
    pub fd: Field,
}

impl AvgParPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        860
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AvgParPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AvgParPxField {
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
pub struct ReportedPxField {
    pub fd: Field,
}

impl ReportedPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        861
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ReportedPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ReportedPxField {
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
pub struct OrderCapacityQtyField {
    pub fd: Field,
}

impl OrderCapacityQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        863
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderCapacityQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderCapacityQtyField {
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
pub struct EventTypeField {
    pub fd: Field,
}

impl EventTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        865
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EventTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EventTypeField {
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
pub struct EventPxField {
    pub fd: Field,
}

impl EventPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        867
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EventPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EventPxField {
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
pub struct EventTextField {
    pub fd: Field,
}

impl EventTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        868
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EventTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EventTextField {
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
pub struct InstrAttribTypeField {
    pub fd: Field,
}

impl InstrAttribTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        871
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrAttribTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrAttribTypeField {
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
pub struct InstrAttribValueField {
    pub fd: Field,
}

impl InstrAttribValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        872
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrAttribValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrAttribValueField {
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
pub struct CPProgramField {
    pub fd: Field,
}

impl CPProgramField {
    #[must_use]
    pub const fn tag() -> u16 {
        875
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CPProgramField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CPProgramField {
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
pub struct CPRegTypeField {
    pub fd: Field,
}

impl CPRegTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        876
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CPRegTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CPRegTypeField {
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
pub struct UnderlyingCPProgramField {
    pub fd: Field,
}

impl UnderlyingCPProgramField {
    #[must_use]
    pub const fn tag() -> u16 {
        877
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCPProgramField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCPProgramField {
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
pub struct UnderlyingCPRegTypeField {
    pub fd: Field,
}

impl UnderlyingCPRegTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        878
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCPRegTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCPRegTypeField {
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
pub struct UnderlyingQtyField {
    pub fd: Field,
}

impl UnderlyingQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        879
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingQtyField {
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
pub struct TrdMatchIDField {
    pub fd: Field,
}

impl TrdMatchIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        880
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TrdMatchIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TrdMatchIDField {
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
pub struct SecondaryTradeReportRefIDField {
    pub fd: Field,
}

impl SecondaryTradeReportRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        881
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryTradeReportRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryTradeReportRefIDField {
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
pub struct UnderlyingDirtyPriceField {
    pub fd: Field,
}

impl UnderlyingDirtyPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        882
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingDirtyPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingDirtyPriceField {
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
pub struct UnderlyingEndPriceField {
    pub fd: Field,
}

impl UnderlyingEndPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        883
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingEndPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingEndPriceField {
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
pub struct UnderlyingStartValueField {
    pub fd: Field,
}

impl UnderlyingStartValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        884
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingStartValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingStartValueField {
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
pub struct UnderlyingCurrentValueField {
    pub fd: Field,
}

impl UnderlyingCurrentValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        885
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCurrentValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCurrentValueField {
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
pub struct UnderlyingEndValueField {
    pub fd: Field,
}

impl UnderlyingEndValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        886
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingEndValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingEndValueField {
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
pub struct UnderlyingStipTypeField {
    pub fd: Field,
}

impl UnderlyingStipTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        888
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingStipTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingStipTypeField {
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
pub struct UnderlyingStipValueField {
    pub fd: Field,
}

impl UnderlyingStipValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        889
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingStipValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingStipValueField {
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
pub struct MaturityNetMoneyField {
    pub fd: Field,
}

impl MaturityNetMoneyField {
    #[must_use]
    pub const fn tag() -> u16 {
        890
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaturityNetMoneyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MaturityNetMoneyField {
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
pub struct MiscFeeBasisField {
    pub fd: Field,
}

impl MiscFeeBasisField {
    #[must_use]
    pub const fn tag() -> u16 {
        891
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MiscFeeBasisField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MiscFeeBasisField {
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
pub struct TotNoAllocsField {
    pub fd: Field,
}

impl TotNoAllocsField {
    #[must_use]
    pub const fn tag() -> u16 {
        892
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNoAllocsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNoAllocsField {
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
pub struct CollReqIDField {
    pub fd: Field,
}

impl CollReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        894
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollReqIDField {
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
pub struct CollAsgnReasonField {
    pub fd: Field,
}

impl CollAsgnReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        895
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollAsgnReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollAsgnReasonField {
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
pub struct CollInquiryQualifierField {
    pub fd: Field,
}

impl CollInquiryQualifierField {
    #[must_use]
    pub const fn tag() -> u16 {
        896
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollInquiryQualifierField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollInquiryQualifierField {
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
pub struct MarginExcessField {
    pub fd: Field,
}

impl MarginExcessField {
    #[must_use]
    pub const fn tag() -> u16 {
        899
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MarginExcessField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MarginExcessField {
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
pub struct TotalNetValueField {
    pub fd: Field,
}

impl TotalNetValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        900
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotalNetValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotalNetValueField {
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
pub struct CashOutstandingField {
    pub fd: Field,
}

impl CashOutstandingField {
    #[must_use]
    pub const fn tag() -> u16 {
        901
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CashOutstandingField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CashOutstandingField {
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
pub struct CollAsgnIDField {
    pub fd: Field,
}

impl CollAsgnIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        902
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollAsgnIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollAsgnIDField {
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
pub struct CollAsgnTransTypeField {
    pub fd: Field,
}

impl CollAsgnTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        903
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollAsgnTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollAsgnTransTypeField {
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
pub struct CollRespIDField {
    pub fd: Field,
}

impl CollRespIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        904
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollRespIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollRespIDField {
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
pub struct CollAsgnRespTypeField {
    pub fd: Field,
}

impl CollAsgnRespTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        905
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollAsgnRespTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollAsgnRespTypeField {
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
pub struct CollAsgnRejectReasonField {
    pub fd: Field,
}

impl CollAsgnRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        906
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollAsgnRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollAsgnRejectReasonField {
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
pub struct CollAsgnRefIDField {
    pub fd: Field,
}

impl CollAsgnRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        907
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollAsgnRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollAsgnRefIDField {
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
pub struct CollRptIDField {
    pub fd: Field,
}

impl CollRptIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        908
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollRptIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollRptIDField {
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
pub struct CollInquiryIDField {
    pub fd: Field,
}

impl CollInquiryIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        909
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollInquiryIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollInquiryIDField {
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
pub struct CollStatusField {
    pub fd: Field,
}

impl CollStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        910
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollStatusField {
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
pub struct TotNumReportsField {
    pub fd: Field,
}

impl TotNumReportsField {
    #[must_use]
    pub const fn tag() -> u16 {
        911
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNumReportsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNumReportsField {
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
pub struct AgreementDescField {
    pub fd: Field,
}

impl AgreementDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        913
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AgreementDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AgreementDescField {
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
pub struct AgreementIDField {
    pub fd: Field,
}

impl AgreementIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        914
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AgreementIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AgreementIDField {
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
pub struct AgreementCurrencyField {
    pub fd: Field,
}

impl AgreementCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        918
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AgreementCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AgreementCurrencyField {
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
pub struct DeliveryTypeField {
    pub fd: Field,
}

impl DeliveryTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        919
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeliveryTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DeliveryTypeField {
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
pub struct EndAccruedInterestAmtField {
    pub fd: Field,
}

impl EndAccruedInterestAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        920
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EndAccruedInterestAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EndAccruedInterestAmtField {
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
pub struct StartCashField {
    pub fd: Field,
}

impl StartCashField {
    #[must_use]
    pub const fn tag() -> u16 {
        921
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StartCashField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StartCashField {
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
pub struct EndCashField {
    pub fd: Field,
}

impl EndCashField {
    #[must_use]
    pub const fn tag() -> u16 {
        922
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EndCashField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EndCashField {
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
pub struct UserRequestIDField {
    pub fd: Field,
}

impl UserRequestIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        923
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UserRequestIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UserRequestIDField {
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
pub struct UserRequestTypeField {
    pub fd: Field,
}

impl UserRequestTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        924
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UserRequestTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UserRequestTypeField {
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
pub struct NewPasswordField {
    pub fd: Field,
}

impl NewPasswordField {
    #[must_use]
    pub const fn tag() -> u16 {
        925
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NewPasswordField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NewPasswordField {
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
pub struct UserStatusField {
    pub fd: Field,
}

impl UserStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        926
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UserStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UserStatusField {
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
pub struct UserStatusTextField {
    pub fd: Field,
}

impl UserStatusTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        927
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UserStatusTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UserStatusTextField {
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
pub struct StatusValueField {
    pub fd: Field,
}

impl StatusValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        928
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StatusValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StatusValueField {
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
pub struct StatusTextField {
    pub fd: Field,
}

impl StatusTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        929
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StatusTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StatusTextField {
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
pub struct RefCompIDField {
    pub fd: Field,
}

impl RefCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        930
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefCompIDField {
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
pub struct RefSubIDField {
    pub fd: Field,
}

impl RefSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        931
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefSubIDField {
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
pub struct NetworkResponseIDField {
    pub fd: Field,
}

impl NetworkResponseIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        932
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NetworkResponseIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NetworkResponseIDField {
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
pub struct NetworkRequestIDField {
    pub fd: Field,
}

impl NetworkRequestIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        933
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NetworkRequestIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NetworkRequestIDField {
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
pub struct LastNetworkResponseIDField {
    pub fd: Field,
}

impl LastNetworkResponseIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        934
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastNetworkResponseIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LastNetworkResponseIDField {
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
pub struct NetworkRequestTypeField {
    pub fd: Field,
}

impl NetworkRequestTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        935
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NetworkRequestTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NetworkRequestTypeField {
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
pub struct NetworkStatusResponseTypeField {
    pub fd: Field,
}

impl NetworkStatusResponseTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        937
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NetworkStatusResponseTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NetworkStatusResponseTypeField {
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
pub struct TrdRptStatusField {
    pub fd: Field,
}

impl TrdRptStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        939
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TrdRptStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TrdRptStatusField {
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
pub struct AffirmStatusField {
    pub fd: Field,
}

impl AffirmStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        940
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AffirmStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AffirmStatusField {
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
pub struct UnderlyingStrikeCurrencyField {
    pub fd: Field,
}

impl UnderlyingStrikeCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        941
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingStrikeCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingStrikeCurrencyField {
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
pub struct LegStrikeCurrencyField {
    pub fd: Field,
}

impl LegStrikeCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        942
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegStrikeCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegStrikeCurrencyField {
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
pub struct TimeBracketField {
    pub fd: Field,
}

impl TimeBracketField {
    #[must_use]
    pub const fn tag() -> u16 {
        943
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TimeBracketField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TimeBracketField {
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
pub struct CollActionField {
    pub fd: Field,
}

impl CollActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        944
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollActionField {
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
pub struct CollInquiryStatusField {
    pub fd: Field,
}

impl CollInquiryStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        945
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollInquiryStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollInquiryStatusField {
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
pub struct CollInquiryResultField {
    pub fd: Field,
}

impl CollInquiryResultField {
    #[must_use]
    pub const fn tag() -> u16 {
        946
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollInquiryResultField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollInquiryResultField {
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
pub struct StrikeCurrencyField {
    pub fd: Field,
}

impl StrikeCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        947
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrikeCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StrikeCurrencyField {
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
pub struct Nested3PartyIDField {
    pub fd: Field,
}

impl Nested3PartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        949
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested3PartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested3PartyIDField {
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
pub struct Nested3PartyIDSourceField {
    pub fd: Field,
}

impl Nested3PartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        950
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested3PartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested3PartyIDSourceField {
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
pub struct Nested3PartyRoleField {
    pub fd: Field,
}

impl Nested3PartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        951
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested3PartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested3PartyRoleField {
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
pub struct Nested3PartySubIDField {
    pub fd: Field,
}

impl Nested3PartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        953
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested3PartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested3PartySubIDField {
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
pub struct Nested3PartySubIDTypeField {
    pub fd: Field,
}

impl Nested3PartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        954
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested3PartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested3PartySubIDTypeField {
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
pub struct StrategyParameterNameField {
    pub fd: Field,
}

impl StrategyParameterNameField {
    #[must_use]
    pub const fn tag() -> u16 {
        958
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrategyParameterNameField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StrategyParameterNameField {
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
pub struct StrategyParameterTypeField {
    pub fd: Field,
}

impl StrategyParameterTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        959
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrategyParameterTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StrategyParameterTypeField {
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
pub struct StrategyParameterValueField {
    pub fd: Field,
}

impl StrategyParameterValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        960
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrategyParameterValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StrategyParameterValueField {
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
pub struct HostCrossIDField {
    pub fd: Field,
}

impl HostCrossIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        961
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HostCrossIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for HostCrossIDField {
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
pub struct MDReportIDField {
    pub fd: Field,
}

impl MDReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        963
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDReportIDField {
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
pub struct SecurityReportIDField {
    pub fd: Field,
}

impl SecurityReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        964
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityReportIDField {
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
pub struct SecurityStatusField {
    pub fd: Field,
}

impl SecurityStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        965
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityStatusField {
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
pub struct SettleOnOpenFlagField {
    pub fd: Field,
}

impl SettleOnOpenFlagField {
    #[must_use]
    pub const fn tag() -> u16 {
        966
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettleOnOpenFlagField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettleOnOpenFlagField {
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
pub struct PositionLimitField {
    pub fd: Field,
}

impl PositionLimitField {
    #[must_use]
    pub const fn tag() -> u16 {
        970
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PositionLimitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PositionLimitField {
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
pub struct NTPositionLimitField {
    pub fd: Field,
}

impl NTPositionLimitField {
    #[must_use]
    pub const fn tag() -> u16 {
        971
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NTPositionLimitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NTPositionLimitField {
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
pub struct UnderlyingCashAmountField {
    pub fd: Field,
}

impl UnderlyingCashAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        973
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCashAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCashAmountField {
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
pub struct UnderlyingCashTypeField {
    pub fd: Field,
}

impl UnderlyingCashTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        974
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCashTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCashTypeField {
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
pub struct UnderlyingSettlementTypeField {
    pub fd: Field,
}

impl UnderlyingSettlementTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        975
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSettlementTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSettlementTypeField {
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
pub struct ContIntRptIDField {
    pub fd: Field,
}

impl ContIntRptIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        977
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContIntRptIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ContIntRptIDField {
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
pub struct InputSourceField {
    pub fd: Field,
}

impl InputSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        979
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InputSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InputSourceField {
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
pub struct SecurityUpdateActionField {
    pub fd: Field,
}

impl SecurityUpdateActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        980
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityUpdateActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityUpdateActionField {
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
pub struct ExpirationQtyTypeField {
    pub fd: Field,
}

impl ExpirationQtyTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        982
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExpirationQtyTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExpirationQtyTypeField {
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
pub struct ExpQtyField {
    pub fd: Field,
}

impl ExpQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        983
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExpQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExpQtyField {
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
pub struct UnderlyingPayAmountField {
    pub fd: Field,
}

impl UnderlyingPayAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        985
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingPayAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingPayAmountField {
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
pub struct UnderlyingCollectAmountField {
    pub fd: Field,
}

impl UnderlyingCollectAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        986
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCollectAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCollectAmountField {
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
pub struct UnderlyingSettlementStatusField {
    pub fd: Field,
}

impl UnderlyingSettlementStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        988
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSettlementStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSettlementStatusField {
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
pub struct SecondaryIndividualAllocIDField {
    pub fd: Field,
}

impl SecondaryIndividualAllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        989
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryIndividualAllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryIndividualAllocIDField {
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
pub struct LegReportIDField {
    pub fd: Field,
}

impl LegReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        990
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegReportIDField {
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
pub struct RndPxField {
    pub fd: Field,
}

impl RndPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        991
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RndPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RndPxField {
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
pub struct IndividualAllocTypeField {
    pub fd: Field,
}

impl IndividualAllocTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        992
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IndividualAllocTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IndividualAllocTypeField {
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
pub struct AllocCustomerCapacityField {
    pub fd: Field,
}

impl AllocCustomerCapacityField {
    #[must_use]
    pub const fn tag() -> u16 {
        993
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocCustomerCapacityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocCustomerCapacityField {
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
pub struct TierCodeField {
    pub fd: Field,
}

impl TierCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        994
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TierCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TierCodeField {
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
pub struct UnitOfMeasureField {
    pub fd: Field,
}

impl UnitOfMeasureField {
    #[must_use]
    pub const fn tag() -> u16 {
        996
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnitOfMeasureField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnitOfMeasureField {
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
pub struct TimeUnitField {
    pub fd: Field,
}

impl TimeUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        997
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TimeUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TimeUnitField {
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
pub struct UnderlyingUnitOfMeasureField {
    pub fd: Field,
}

impl UnderlyingUnitOfMeasureField {
    #[must_use]
    pub const fn tag() -> u16 {
        998
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingUnitOfMeasureField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingUnitOfMeasureField {
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
pub struct LegUnitOfMeasureField {
    pub fd: Field,
}

impl LegUnitOfMeasureField {
    #[must_use]
    pub const fn tag() -> u16 {
        999
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegUnitOfMeasureField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegUnitOfMeasureField {
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
pub struct UnderlyingTimeUnitField {
    pub fd: Field,
}

impl UnderlyingTimeUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1000
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingTimeUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingTimeUnitField {
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
pub struct LegTimeUnitField {
    pub fd: Field,
}

impl LegTimeUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1001
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegTimeUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegTimeUnitField {
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
pub struct AllocMethodField {
    pub fd: Field,
}

impl AllocMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1002
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocMethodField {
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
pub struct TradeIDField {
    pub fd: Field,
}

impl TradeIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1003
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeIDField {
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
pub struct SideTradeReportIDField {
    pub fd: Field,
}

impl SideTradeReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1005
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideTradeReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideTradeReportIDField {
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
pub struct SideFillStationCdField {
    pub fd: Field,
}

impl SideFillStationCdField {
    #[must_use]
    pub const fn tag() -> u16 {
        1006
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideFillStationCdField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideFillStationCdField {
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
pub struct SideReasonCdField {
    pub fd: Field,
}

impl SideReasonCdField {
    #[must_use]
    pub const fn tag() -> u16 {
        1007
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideReasonCdField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideReasonCdField {
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
pub struct SideTrdSubTypField {
    pub fd: Field,
}

impl SideTrdSubTypField {
    #[must_use]
    pub const fn tag() -> u16 {
        1008
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideTrdSubTypField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideTrdSubTypField {
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
pub struct SideLastQtyField {
    pub fd: Field,
}

impl SideLastQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1009
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideLastQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideLastQtyField {
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
pub struct MessageEventSourceField {
    pub fd: Field,
}

impl MessageEventSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1011
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MessageEventSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MessageEventSourceField {
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
pub struct SideTrdRegTimestampTypeField {
    pub fd: Field,
}

impl SideTrdRegTimestampTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1013
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideTrdRegTimestampTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideTrdRegTimestampTypeField {
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
pub struct SideTrdRegTimestampSrcField {
    pub fd: Field,
}

impl SideTrdRegTimestampSrcField {
    #[must_use]
    pub const fn tag() -> u16 {
        1014
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideTrdRegTimestampSrcField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideTrdRegTimestampSrcField {
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
pub struct AsOfIndicatorField {
    pub fd: Field,
}

impl AsOfIndicatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        1015
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AsOfIndicatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AsOfIndicatorField {
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
pub struct InstrumentPartyIDField {
    pub fd: Field,
}

impl InstrumentPartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1019
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrumentPartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrumentPartyIDField {
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
pub struct TradeVolumeField {
    pub fd: Field,
}

impl TradeVolumeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1020
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeVolumeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeVolumeField {
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
pub struct MDBookTypeField {
    pub fd: Field,
}

impl MDBookTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1021
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDBookTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDBookTypeField {
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
pub struct MDFeedTypeField {
    pub fd: Field,
}

impl MDFeedTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1022
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDFeedTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDFeedTypeField {
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
pub struct MDPriceLevelField {
    pub fd: Field,
}

impl MDPriceLevelField {
    #[must_use]
    pub const fn tag() -> u16 {
        1023
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDPriceLevelField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDPriceLevelField {
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
pub struct MDOriginTypeField {
    pub fd: Field,
}

impl MDOriginTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1024
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDOriginTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDOriginTypeField {
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
pub struct FirstPxField {
    pub fd: Field,
}

impl FirstPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        1025
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FirstPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FirstPxField {
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
pub struct ReceivedDeptIDField {
    pub fd: Field,
}

impl ReceivedDeptIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1030
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ReceivedDeptIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ReceivedDeptIDField {
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
pub struct OrderHandlingInstSourceField {
    pub fd: Field,
}

impl OrderHandlingInstSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1032
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderHandlingInstSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderHandlingInstSourceField {
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
pub struct DeskTypeField {
    pub fd: Field,
}

impl DeskTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1033
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeskTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DeskTypeField {
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
pub struct DeskTypeSourceField {
    pub fd: Field,
}

impl DeskTypeSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1034
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeskTypeSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DeskTypeSourceField {
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
pub struct ExecAckStatusField {
    pub fd: Field,
}

impl ExecAckStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        1036
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecAckStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExecAckStatusField {
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
pub struct UnderlyingDeliveryAmountField {
    pub fd: Field,
}

impl UnderlyingDeliveryAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1037
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingDeliveryAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingDeliveryAmountField {
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
pub struct UnderlyingCapValueField {
    pub fd: Field,
}

impl UnderlyingCapValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        1038
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingCapValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingCapValueField {
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
pub struct UnderlyingSettlMethodField {
    pub fd: Field,
}

impl UnderlyingSettlMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1039
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSettlMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSettlMethodField {
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
pub struct SecondaryTradeIDField {
    pub fd: Field,
}

impl SecondaryTradeIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1040
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryTradeIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryTradeIDField {
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
pub struct FirmTradeIDField {
    pub fd: Field,
}

impl FirmTradeIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1041
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FirmTradeIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FirmTradeIDField {
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
pub struct SecondaryFirmTradeIDField {
    pub fd: Field,
}

impl SecondaryFirmTradeIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1042
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryFirmTradeIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryFirmTradeIDField {
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
pub struct CollApplTypeField {
    pub fd: Field,
}

impl CollApplTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1043
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CollApplTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CollApplTypeField {
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
pub struct UnderlyingAdjustedQuantityField {
    pub fd: Field,
}

impl UnderlyingAdjustedQuantityField {
    #[must_use]
    pub const fn tag() -> u16 {
        1044
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingAdjustedQuantityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingAdjustedQuantityField {
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
pub struct UnderlyingFXRateCalcField {
    pub fd: Field,
}

impl UnderlyingFXRateCalcField {
    #[must_use]
    pub const fn tag() -> u16 {
        1046
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingFXRateCalcField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingFXRateCalcField {
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
pub struct AllocPositionEffectField {
    pub fd: Field,
}

impl AllocPositionEffectField {
    #[must_use]
    pub const fn tag() -> u16 {
        1047
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocPositionEffectField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocPositionEffectField {
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
pub struct DealingCapacityField {
    pub fd: Field,
}

impl DealingCapacityField {
    #[must_use]
    pub const fn tag() -> u16 {
        1048
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DealingCapacityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DealingCapacityField {
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
pub struct InstrmtAssignmentMethodField {
    pub fd: Field,
}

impl InstrmtAssignmentMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1049
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrmtAssignmentMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrmtAssignmentMethodField {
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
pub struct InstrumentPartyIDSourceField {
    pub fd: Field,
}

impl InstrumentPartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1050
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrumentPartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrumentPartyIDSourceField {
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
pub struct InstrumentPartyRoleField {
    pub fd: Field,
}

impl InstrumentPartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1051
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrumentPartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrumentPartyRoleField {
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
pub struct InstrumentPartySubIDField {
    pub fd: Field,
}

impl InstrumentPartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1053
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrumentPartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrumentPartySubIDField {
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
pub struct InstrumentPartySubIDTypeField {
    pub fd: Field,
}

impl InstrumentPartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1054
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for InstrumentPartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for InstrumentPartySubIDTypeField {
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
pub struct PositionCurrencyField {
    pub fd: Field,
}

impl PositionCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1055
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PositionCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PositionCurrencyField {
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
pub struct CalculatedCcyLastQtyField {
    pub fd: Field,
}

impl CalculatedCcyLastQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1056
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CalculatedCcyLastQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CalculatedCcyLastQtyField {
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
pub struct UnderlyingInstrumentPartyIDField {
    pub fd: Field,
}

impl UnderlyingInstrumentPartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1059
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingInstrumentPartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingInstrumentPartyIDField {
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
pub struct UnderlyingInstrumentPartyIDSourceField {
    pub fd: Field,
}

impl UnderlyingInstrumentPartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1060
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingInstrumentPartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingInstrumentPartyIDSourceField {
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
pub struct UnderlyingInstrumentPartyRoleField {
    pub fd: Field,
}

impl UnderlyingInstrumentPartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1061
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingInstrumentPartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingInstrumentPartyRoleField {
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
pub struct UnderlyingInstrumentPartySubIDField {
    pub fd: Field,
}

impl UnderlyingInstrumentPartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1063
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingInstrumentPartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingInstrumentPartySubIDField {
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
pub struct UnderlyingInstrumentPartySubIDTypeField {
    pub fd: Field,
}

impl UnderlyingInstrumentPartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1064
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingInstrumentPartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingInstrumentPartySubIDTypeField {
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
pub struct MDQuoteTypeField {
    pub fd: Field,
}

impl MDQuoteTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1070
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDQuoteTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDQuoteTypeField {
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
pub struct SideGrossTradeAmtField {
    pub fd: Field,
}

impl SideGrossTradeAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        1072
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideGrossTradeAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideGrossTradeAmtField {
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
pub struct LegCalculatedCcyLastQtyField {
    pub fd: Field,
}

impl LegCalculatedCcyLastQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1074
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegCalculatedCcyLastQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegCalculatedCcyLastQtyField {
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
pub struct LegGrossTradeAmtField {
    pub fd: Field,
}

impl LegGrossTradeAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        1075
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegGrossTradeAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegGrossTradeAmtField {
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
pub struct RefOrderIDField {
    pub fd: Field,
}

impl RefOrderIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1080
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefOrderIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefOrderIDField {
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
pub struct RefOrderIDSourceField {
    pub fd: Field,
}

impl RefOrderIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1081
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefOrderIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefOrderIDSourceField {
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
pub struct SecondaryDisplayQtyField {
    pub fd: Field,
}

impl SecondaryDisplayQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1082
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryDisplayQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryDisplayQtyField {
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
pub struct DisplayWhenField {
    pub fd: Field,
}

impl DisplayWhenField {
    #[must_use]
    pub const fn tag() -> u16 {
        1083
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DisplayWhenField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DisplayWhenField {
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
pub struct DisplayMethodField {
    pub fd: Field,
}

impl DisplayMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1084
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DisplayMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DisplayMethodField {
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
pub struct DisplayLowQtyField {
    pub fd: Field,
}

impl DisplayLowQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1085
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DisplayLowQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DisplayLowQtyField {
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
pub struct DisplayHighQtyField {
    pub fd: Field,
}

impl DisplayHighQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1086
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DisplayHighQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DisplayHighQtyField {
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
pub struct DisplayMinIncrField {
    pub fd: Field,
}

impl DisplayMinIncrField {
    #[must_use]
    pub const fn tag() -> u16 {
        1087
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DisplayMinIncrField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DisplayMinIncrField {
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
pub struct RefreshQtyField {
    pub fd: Field,
}

impl RefreshQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1088
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefreshQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefreshQtyField {
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
pub struct MatchIncrementField {
    pub fd: Field,
}

impl MatchIncrementField {
    #[must_use]
    pub const fn tag() -> u16 {
        1089
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MatchIncrementField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MatchIncrementField {
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
pub struct MaxPriceLevelsField {
    pub fd: Field,
}

impl MaxPriceLevelsField {
    #[must_use]
    pub const fn tag() -> u16 {
        1090
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaxPriceLevelsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MaxPriceLevelsField {
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
pub struct PriceProtectionScopeField {
    pub fd: Field,
}

impl PriceProtectionScopeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1092
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriceProtectionScopeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PriceProtectionScopeField {
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
pub struct LotTypeField {
    pub fd: Field,
}

impl LotTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1093
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LotTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LotTypeField {
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
pub struct PegPriceTypeField {
    pub fd: Field,
}

impl PegPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1094
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegPriceTypeField {
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
pub struct PeggedRefPriceField {
    pub fd: Field,
}

impl PeggedRefPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1095
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PeggedRefPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PeggedRefPriceField {
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
pub struct PegSecurityIDSourceField {
    pub fd: Field,
}

impl PegSecurityIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1096
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegSecurityIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegSecurityIDSourceField {
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
pub struct PegSecurityIDField {
    pub fd: Field,
}

impl PegSecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1097
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegSecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegSecurityIDField {
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
pub struct PegSymbolField {
    pub fd: Field,
}

impl PegSymbolField {
    #[must_use]
    pub const fn tag() -> u16 {
        1098
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegSymbolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegSymbolField {
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
pub struct PegSecurityDescField {
    pub fd: Field,
}

impl PegSecurityDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        1099
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PegSecurityDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PegSecurityDescField {
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
pub struct TriggerTypeField {
    pub fd: Field,
}

impl TriggerTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1100
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerTypeField {
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
pub struct TriggerActionField {
    pub fd: Field,
}

impl TriggerActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        1101
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerActionField {
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
pub struct TriggerPriceField {
    pub fd: Field,
}

impl TriggerPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1102
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerPriceField {
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
pub struct TriggerSymbolField {
    pub fd: Field,
}

impl TriggerSymbolField {
    #[must_use]
    pub const fn tag() -> u16 {
        1103
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerSymbolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerSymbolField {
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
pub struct TriggerSecurityIDField {
    pub fd: Field,
}

impl TriggerSecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1104
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerSecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerSecurityIDField {
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
pub struct TriggerSecurityIDSourceField {
    pub fd: Field,
}

impl TriggerSecurityIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1105
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerSecurityIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerSecurityIDSourceField {
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
pub struct TriggerSecurityDescField {
    pub fd: Field,
}

impl TriggerSecurityDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        1106
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerSecurityDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerSecurityDescField {
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
pub struct TriggerPriceTypeField {
    pub fd: Field,
}

impl TriggerPriceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1107
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerPriceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerPriceTypeField {
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
pub struct TriggerPriceTypeScopeField {
    pub fd: Field,
}

impl TriggerPriceTypeScopeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1108
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerPriceTypeScopeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerPriceTypeScopeField {
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
pub struct TriggerPriceDirectionField {
    pub fd: Field,
}

impl TriggerPriceDirectionField {
    #[must_use]
    pub const fn tag() -> u16 {
        1109
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerPriceDirectionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerPriceDirectionField {
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
pub struct TriggerNewPriceField {
    pub fd: Field,
}

impl TriggerNewPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1110
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerNewPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerNewPriceField {
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
pub struct TriggerOrderTypeField {
    pub fd: Field,
}

impl TriggerOrderTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1111
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerOrderTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerOrderTypeField {
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
pub struct TriggerNewQtyField {
    pub fd: Field,
}

impl TriggerNewQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1112
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerNewQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerNewQtyField {
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
pub struct TriggerTradingSessionIDField {
    pub fd: Field,
}

impl TriggerTradingSessionIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1113
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerTradingSessionIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerTradingSessionIDField {
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
pub struct TriggerTradingSessionSubIDField {
    pub fd: Field,
}

impl TriggerTradingSessionSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1114
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TriggerTradingSessionSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TriggerTradingSessionSubIDField {
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
pub struct OrderCategoryField {
    pub fd: Field,
}

impl OrderCategoryField {
    #[must_use]
    pub const fn tag() -> u16 {
        1115
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderCategoryField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderCategoryField {
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
pub struct RootPartyIDField {
    pub fd: Field,
}

impl RootPartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1117
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RootPartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RootPartyIDField {
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
pub struct RootPartyIDSourceField {
    pub fd: Field,
}

impl RootPartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1118
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RootPartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RootPartyIDSourceField {
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
pub struct RootPartyRoleField {
    pub fd: Field,
}

impl RootPartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1119
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RootPartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RootPartyRoleField {
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
pub struct RootPartySubIDField {
    pub fd: Field,
}

impl RootPartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1121
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RootPartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RootPartySubIDField {
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
pub struct RootPartySubIDTypeField {
    pub fd: Field,
}

impl RootPartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1122
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RootPartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RootPartySubIDTypeField {
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
pub struct TradeHandlingInstrField {
    pub fd: Field,
}

impl TradeHandlingInstrField {
    #[must_use]
    pub const fn tag() -> u16 {
        1123
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradeHandlingInstrField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradeHandlingInstrField {
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
pub struct OrigTradeHandlingInstrField {
    pub fd: Field,
}

impl OrigTradeHandlingInstrField {
    #[must_use]
    pub const fn tag() -> u16 {
        1124
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrigTradeHandlingInstrField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrigTradeHandlingInstrField {
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
pub struct OrigTradeIDField {
    pub fd: Field,
}

impl OrigTradeIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1126
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrigTradeIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrigTradeIDField {
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
pub struct OrigSecondaryTradeIDField {
    pub fd: Field,
}

impl OrigSecondaryTradeIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1127
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrigSecondaryTradeIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrigSecondaryTradeIDField {
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
pub struct ApplVerIDField {
    pub fd: Field,
}

impl ApplVerIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1128
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplVerIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplVerIDField {
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
pub struct CstmApplVerIDField {
    pub fd: Field,
}

impl CstmApplVerIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1129
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CstmApplVerIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CstmApplVerIDField {
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
pub struct RefApplVerIDField {
    pub fd: Field,
}

impl RefApplVerIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1130
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefApplVerIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefApplVerIDField {
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
pub struct RefCstmApplVerIDField {
    pub fd: Field,
}

impl RefCstmApplVerIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1131
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefCstmApplVerIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefCstmApplVerIDField {
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
pub struct ExDestinationIDSourceField {
    pub fd: Field,
}

impl ExDestinationIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1133
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExDestinationIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExDestinationIDSourceField {
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
pub struct RptSysField {
    pub fd: Field,
}

impl RptSysField {
    #[must_use]
    pub const fn tag() -> u16 {
        1135
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RptSysField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RptSysField {
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
pub struct AllocClearingFeeIndicatorField {
    pub fd: Field,
}

impl AllocClearingFeeIndicatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        1136
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocClearingFeeIndicatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocClearingFeeIndicatorField {
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
pub struct DefaultApplVerIDField {
    pub fd: Field,
}

impl DefaultApplVerIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1137
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DefaultApplVerIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DefaultApplVerIDField {
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
pub struct DisplayQtyField {
    pub fd: Field,
}

impl DisplayQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1138
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DisplayQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DisplayQtyField {
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
pub struct ExchangeSpecialInstructionsField {
    pub fd: Field,
}

impl ExchangeSpecialInstructionsField {
    #[must_use]
    pub const fn tag() -> u16 {
        1139
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExchangeSpecialInstructionsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExchangeSpecialInstructionsField {
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
pub struct MaxTradeVolField {
    pub fd: Field,
}

impl MaxTradeVolField {
    #[must_use]
    pub const fn tag() -> u16 {
        1140
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaxTradeVolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MaxTradeVolField {
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
pub struct MatchAlgorithmField {
    pub fd: Field,
}

impl MatchAlgorithmField {
    #[must_use]
    pub const fn tag() -> u16 {
        1142
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MatchAlgorithmField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MatchAlgorithmField {
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
pub struct ImpliedMarketIndicatorField {
    pub fd: Field,
}

impl ImpliedMarketIndicatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        1144
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ImpliedMarketIndicatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ImpliedMarketIndicatorField {
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
pub struct MinPriceIncrementAmountField {
    pub fd: Field,
}

impl MinPriceIncrementAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1146
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MinPriceIncrementAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MinPriceIncrementAmountField {
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
pub struct UnitOfMeasureQtyField {
    pub fd: Field,
}

impl UnitOfMeasureQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1147
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnitOfMeasureQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnitOfMeasureQtyField {
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
pub struct LowLimitPriceField {
    pub fd: Field,
}

impl LowLimitPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1148
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LowLimitPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LowLimitPriceField {
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
pub struct HighLimitPriceField {
    pub fd: Field,
}

impl HighLimitPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1149
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HighLimitPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for HighLimitPriceField {
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
pub struct TradingReferencePriceField {
    pub fd: Field,
}

impl TradingReferencePriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1150
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradingReferencePriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradingReferencePriceField {
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
pub struct SecurityGroupField {
    pub fd: Field,
}

impl SecurityGroupField {
    #[must_use]
    pub const fn tag() -> u16 {
        1151
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityGroupField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityGroupField {
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
pub struct LegNumberField {
    pub fd: Field,
}

impl LegNumberField {
    #[must_use]
    pub const fn tag() -> u16 {
        1152
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegNumberField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegNumberField {
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
pub struct SettlementCycleNoField {
    pub fd: Field,
}

impl SettlementCycleNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        1153
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlementCycleNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlementCycleNoField {
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
pub struct SideCurrencyField {
    pub fd: Field,
}

impl SideCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1154
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideCurrencyField {
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
pub struct SideSettlCurrencyField {
    pub fd: Field,
}

impl SideSettlCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1155
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideSettlCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideSettlCurrencyField {
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
pub struct ApplExtIDField {
    pub fd: Field,
}

impl ApplExtIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1156
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplExtIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplExtIDField {
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
pub struct CcyAmtField {
    pub fd: Field,
}

impl CcyAmtField {
    #[must_use]
    pub const fn tag() -> u16 {
        1157
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CcyAmtField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CcyAmtField {
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
pub struct SettlObligModeField {
    pub fd: Field,
}

impl SettlObligModeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1159
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlObligModeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlObligModeField {
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
pub struct SettlObligMsgIDField {
    pub fd: Field,
}

impl SettlObligMsgIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1160
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlObligMsgIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlObligMsgIDField {
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
pub struct SettlObligIDField {
    pub fd: Field,
}

impl SettlObligIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1161
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlObligIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlObligIDField {
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
pub struct SettlObligTransTypeField {
    pub fd: Field,
}

impl SettlObligTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1162
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlObligTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlObligTransTypeField {
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
pub struct SettlObligRefIDField {
    pub fd: Field,
}

impl SettlObligRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1163
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlObligRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlObligRefIDField {
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
pub struct SettlObligSourceField {
    pub fd: Field,
}

impl SettlObligSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1164
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlObligSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlObligSourceField {
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
pub struct QuoteMsgIDField {
    pub fd: Field,
}

impl QuoteMsgIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1166
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteMsgIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteMsgIDField {
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
pub struct QuoteEntryStatusField {
    pub fd: Field,
}

impl QuoteEntryStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        1167
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteEntryStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteEntryStatusField {
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
pub struct TotNoCxldQuotesField {
    pub fd: Field,
}

impl TotNoCxldQuotesField {
    #[must_use]
    pub const fn tag() -> u16 {
        1168
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNoCxldQuotesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNoCxldQuotesField {
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
pub struct TotNoAccQuotesField {
    pub fd: Field,
}

impl TotNoAccQuotesField {
    #[must_use]
    pub const fn tag() -> u16 {
        1169
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNoAccQuotesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNoAccQuotesField {
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
pub struct TotNoRejQuotesField {
    pub fd: Field,
}

impl TotNoRejQuotesField {
    #[must_use]
    pub const fn tag() -> u16 {
        1170
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNoRejQuotesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNoRejQuotesField {
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
pub struct RespondentTypeField {
    pub fd: Field,
}

impl RespondentTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1172
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RespondentTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RespondentTypeField {
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
pub struct MDSubBookTypeField {
    pub fd: Field,
}

impl MDSubBookTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1173
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDSubBookTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDSubBookTypeField {
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
pub struct SecurityTradingEventField {
    pub fd: Field,
}

impl SecurityTradingEventField {
    #[must_use]
    pub const fn tag() -> u16 {
        1174
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityTradingEventField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityTradingEventField {
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
pub struct StatsTypeField {
    pub fd: Field,
}

impl StatsTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1176
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StatsTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StatsTypeField {
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
pub struct MDSecSizeTypeField {
    pub fd: Field,
}

impl MDSecSizeTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1178
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDSecSizeTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDSecSizeTypeField {
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
pub struct MDSecSizeField {
    pub fd: Field,
}

impl MDSecSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1179
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDSecSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDSecSizeField {
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
pub struct ApplIDField {
    pub fd: Field,
}

impl ApplIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1180
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplIDField {
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
pub struct SecurityXMLSchemaField {
    pub fd: Field,
}

impl SecurityXMLSchemaField {
    #[must_use]
    pub const fn tag() -> u16 {
        1186
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityXMLSchemaField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityXMLSchemaField {
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
pub struct PriceUnitOfMeasureField {
    pub fd: Field,
}

impl PriceUnitOfMeasureField {
    #[must_use]
    pub const fn tag() -> u16 {
        1191
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriceUnitOfMeasureField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PriceUnitOfMeasureField {
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
pub struct PriceUnitOfMeasureQtyField {
    pub fd: Field,
}

impl PriceUnitOfMeasureQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1192
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriceUnitOfMeasureQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PriceUnitOfMeasureQtyField {
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
pub struct SettlMethodField {
    pub fd: Field,
}

impl SettlMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1193
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlMethodField {
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
pub struct ExerciseStyleField {
    pub fd: Field,
}

impl ExerciseStyleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1194
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExerciseStyleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExerciseStyleField {
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
pub struct OptPayoutAmountField {
    pub fd: Field,
}

impl OptPayoutAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1195
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OptPayoutAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OptPayoutAmountField {
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
pub struct PriceQuoteMethodField {
    pub fd: Field,
}

impl PriceQuoteMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1196
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriceQuoteMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PriceQuoteMethodField {
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
pub struct ValuationMethodField {
    pub fd: Field,
}

impl ValuationMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1197
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ValuationMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ValuationMethodField {
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
pub struct ListMethodField {
    pub fd: Field,
}

impl ListMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1198
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ListMethodField {
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
pub struct CapPriceField {
    pub fd: Field,
}

impl CapPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1199
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CapPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CapPriceField {
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
pub struct FloorPriceField {
    pub fd: Field,
}

impl FloorPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1200
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FloorPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FloorPriceField {
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
pub struct StartStrikePxRangeField {
    pub fd: Field,
}

impl StartStrikePxRangeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1202
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StartStrikePxRangeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StartStrikePxRangeField {
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
pub struct EndStrikePxRangeField {
    pub fd: Field,
}

impl EndStrikePxRangeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1203
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EndStrikePxRangeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EndStrikePxRangeField {
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
pub struct StartTickPriceRangeField {
    pub fd: Field,
}

impl StartTickPriceRangeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1206
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StartTickPriceRangeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StartTickPriceRangeField {
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
pub struct EndTickPriceRangeField {
    pub fd: Field,
}

impl EndTickPriceRangeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1207
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EndTickPriceRangeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EndTickPriceRangeField {
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
pub struct TickIncrementField {
    pub fd: Field,
}

impl TickIncrementField {
    #[must_use]
    pub const fn tag() -> u16 {
        1208
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TickIncrementField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TickIncrementField {
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
pub struct TickRuleTypeField {
    pub fd: Field,
}

impl TickRuleTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1209
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TickRuleTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TickRuleTypeField {
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
pub struct NestedInstrAttribTypeField {
    pub fd: Field,
}

impl NestedInstrAttribTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1210
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NestedInstrAttribTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NestedInstrAttribTypeField {
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
pub struct NestedInstrAttribValueField {
    pub fd: Field,
}

impl NestedInstrAttribValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        1211
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NestedInstrAttribValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NestedInstrAttribValueField {
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
pub struct DerivativeSymbolField {
    pub fd: Field,
}

impl DerivativeSymbolField {
    #[must_use]
    pub const fn tag() -> u16 {
        1214
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSymbolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSymbolField {
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
pub struct DerivativeSymbolSfxField {
    pub fd: Field,
}

impl DerivativeSymbolSfxField {
    #[must_use]
    pub const fn tag() -> u16 {
        1215
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSymbolSfxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSymbolSfxField {
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
pub struct DerivativeSecurityIDField {
    pub fd: Field,
}

impl DerivativeSecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1216
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityIDField {
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
pub struct DerivativeSecurityIDSourceField {
    pub fd: Field,
}

impl DerivativeSecurityIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1217
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityIDSourceField {
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
pub struct DerivativeSecurityAltIDField {
    pub fd: Field,
}

impl DerivativeSecurityAltIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1219
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityAltIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityAltIDField {
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
pub struct DerivativeSecurityAltIDSourceField {
    pub fd: Field,
}

impl DerivativeSecurityAltIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1220
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityAltIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityAltIDSourceField {
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
pub struct SecondaryLowLimitPriceField {
    pub fd: Field,
}

impl SecondaryLowLimitPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1221
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryLowLimitPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryLowLimitPriceField {
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
pub struct MaturityRuleIDField {
    pub fd: Field,
}

impl MaturityRuleIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1222
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaturityRuleIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MaturityRuleIDField {
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
pub struct StrikeRuleIDField {
    pub fd: Field,
}

impl StrikeRuleIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1223
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrikeRuleIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StrikeRuleIDField {
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
pub struct LegUnitOfMeasureQtyField {
    pub fd: Field,
}

impl LegUnitOfMeasureQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1224
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegUnitOfMeasureQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegUnitOfMeasureQtyField {
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
pub struct DerivativeOptPayAmountField {
    pub fd: Field,
}

impl DerivativeOptPayAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1225
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeOptPayAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeOptPayAmountField {
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
pub struct ProductComplexField {
    pub fd: Field,
}

impl ProductComplexField {
    #[must_use]
    pub const fn tag() -> u16 {
        1227
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ProductComplexField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ProductComplexField {
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
pub struct DerivativeProductComplexField {
    pub fd: Field,
}

impl DerivativeProductComplexField {
    #[must_use]
    pub const fn tag() -> u16 {
        1228
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeProductComplexField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeProductComplexField {
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
pub struct MaturityMonthYearIncrementField {
    pub fd: Field,
}

impl MaturityMonthYearIncrementField {
    #[must_use]
    pub const fn tag() -> u16 {
        1229
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaturityMonthYearIncrementField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MaturityMonthYearIncrementField {
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
pub struct SecondaryHighLimitPriceField {
    pub fd: Field,
}

impl SecondaryHighLimitPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1230
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryHighLimitPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryHighLimitPriceField {
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
pub struct MinLotSizeField {
    pub fd: Field,
}

impl MinLotSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1231
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MinLotSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MinLotSizeField {
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
pub struct SecondaryTradingReferencePriceField {
    pub fd: Field,
}

impl SecondaryTradingReferencePriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1240
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryTradingReferencePriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryTradingReferencePriceField {
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
pub struct TradingCurrencyField {
    pub fd: Field,
}

impl TradingCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1245
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradingCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradingCurrencyField {
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
pub struct DerivativeProductField {
    pub fd: Field,
}

impl DerivativeProductField {
    #[must_use]
    pub const fn tag() -> u16 {
        1246
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeProductField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeProductField {
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
pub struct DerivativeSecurityGroupField {
    pub fd: Field,
}

impl DerivativeSecurityGroupField {
    #[must_use]
    pub const fn tag() -> u16 {
        1247
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityGroupField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityGroupField {
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
pub struct DerivativeCFICodeField {
    pub fd: Field,
}

impl DerivativeCFICodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1248
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeCFICodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeCFICodeField {
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
pub struct DerivativeSecurityTypeField {
    pub fd: Field,
}

impl DerivativeSecurityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1249
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityTypeField {
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
pub struct DerivativeSecuritySubTypeField {
    pub fd: Field,
}

impl DerivativeSecuritySubTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1250
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecuritySubTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecuritySubTypeField {
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
pub struct DerivativeSettleOnOpenFlagField {
    pub fd: Field,
}

impl DerivativeSettleOnOpenFlagField {
    #[must_use]
    pub const fn tag() -> u16 {
        1254
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSettleOnOpenFlagField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSettleOnOpenFlagField {
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
pub struct DerivativeInstrmtAssignmentMethodField {
    pub fd: Field,
}

impl DerivativeInstrmtAssignmentMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1255
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrmtAssignmentMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrmtAssignmentMethodField {
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
pub struct DerivativeSecurityStatusField {
    pub fd: Field,
}

impl DerivativeSecurityStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        1256
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityStatusField {
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
pub struct DerivativeInstrRegistryField {
    pub fd: Field,
}

impl DerivativeInstrRegistryField {
    #[must_use]
    pub const fn tag() -> u16 {
        1257
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrRegistryField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrRegistryField {
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
pub struct DerivativeStateOrProvinceOfIssueField {
    pub fd: Field,
}

impl DerivativeStateOrProvinceOfIssueField {
    #[must_use]
    pub const fn tag() -> u16 {
        1259
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeStateOrProvinceOfIssueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeStateOrProvinceOfIssueField {
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
pub struct DerivativeLocaleOfIssueField {
    pub fd: Field,
}

impl DerivativeLocaleOfIssueField {
    #[must_use]
    pub const fn tag() -> u16 {
        1260
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeLocaleOfIssueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeLocaleOfIssueField {
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
pub struct DerivativeStrikePriceField {
    pub fd: Field,
}

impl DerivativeStrikePriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1261
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeStrikePriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeStrikePriceField {
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
pub struct DerivativeStrikeCurrencyField {
    pub fd: Field,
}

impl DerivativeStrikeCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1262
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeStrikeCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeStrikeCurrencyField {
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
pub struct DerivativeOptAttributeField {
    pub fd: Field,
}

impl DerivativeOptAttributeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1265
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeOptAttributeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeOptAttributeField {
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
pub struct DerivativeMinPriceIncrementAmountField {
    pub fd: Field,
}

impl DerivativeMinPriceIncrementAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1268
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeMinPriceIncrementAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeMinPriceIncrementAmountField {
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
pub struct DerivativeUnitOfMeasureField {
    pub fd: Field,
}

impl DerivativeUnitOfMeasureField {
    #[must_use]
    pub const fn tag() -> u16 {
        1269
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeUnitOfMeasureField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeUnitOfMeasureField {
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
pub struct DerivativeUnitOfMeasureQtyField {
    pub fd: Field,
}

impl DerivativeUnitOfMeasureQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1270
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeUnitOfMeasureQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeUnitOfMeasureQtyField {
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
pub struct DerivativeTimeUnitField {
    pub fd: Field,
}

impl DerivativeTimeUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1271
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeTimeUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeTimeUnitField {
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
pub struct DerivativePositionLimitField {
    pub fd: Field,
}

impl DerivativePositionLimitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1273
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativePositionLimitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativePositionLimitField {
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
pub struct DerivativeNTPositionLimitField {
    pub fd: Field,
}

impl DerivativeNTPositionLimitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1274
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeNTPositionLimitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeNTPositionLimitField {
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
pub struct DerivativeIssuerField {
    pub fd: Field,
}

impl DerivativeIssuerField {
    #[must_use]
    pub const fn tag() -> u16 {
        1275
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeIssuerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeIssuerField {
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
pub struct DerivativeSecurityDescField {
    pub fd: Field,
}

impl DerivativeSecurityDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        1279
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityDescField {
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
pub struct DerivativeSecurityXMLSchemaField {
    pub fd: Field,
}

impl DerivativeSecurityXMLSchemaField {
    #[must_use]
    pub const fn tag() -> u16 {
        1284
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSecurityXMLSchemaField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSecurityXMLSchemaField {
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
pub struct DerivativeEventTypeField {
    pub fd: Field,
}

impl DerivativeEventTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1287
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeEventTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeEventTypeField {
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
pub struct DerivativeEventPxField {
    pub fd: Field,
}

impl DerivativeEventPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        1290
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeEventPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeEventPxField {
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
pub struct DerivativeEventTextField {
    pub fd: Field,
}

impl DerivativeEventTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        1291
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeEventTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeEventTextField {
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
pub struct DerivativeInstrumentPartyIDField {
    pub fd: Field,
}

impl DerivativeInstrumentPartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1293
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrumentPartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrumentPartyIDField {
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
pub struct DerivativeInstrumentPartyIDSourceField {
    pub fd: Field,
}

impl DerivativeInstrumentPartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1294
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrumentPartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrumentPartyIDSourceField {
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
pub struct DerivativeInstrumentPartyRoleField {
    pub fd: Field,
}

impl DerivativeInstrumentPartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1295
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrumentPartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrumentPartyRoleField {
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
pub struct DerivativeInstrumentPartySubIDField {
    pub fd: Field,
}

impl DerivativeInstrumentPartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1297
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrumentPartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrumentPartySubIDField {
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
pub struct DerivativeInstrumentPartySubIDTypeField {
    pub fd: Field,
}

impl DerivativeInstrumentPartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1298
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrumentPartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrumentPartySubIDTypeField {
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
pub struct DerivativeExerciseStyleField {
    pub fd: Field,
}

impl DerivativeExerciseStyleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1299
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeExerciseStyleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeExerciseStyleField {
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
pub struct MarketSegmentIDField {
    pub fd: Field,
}

impl MarketSegmentIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1300
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MarketSegmentIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MarketSegmentIDField {
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
pub struct MaturityMonthYearIncrementUnitsField {
    pub fd: Field,
}

impl MaturityMonthYearIncrementUnitsField {
    #[must_use]
    pub const fn tag() -> u16 {
        1302
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaturityMonthYearIncrementUnitsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MaturityMonthYearIncrementUnitsField {
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
pub struct MaturityMonthYearFormatField {
    pub fd: Field,
}

impl MaturityMonthYearFormatField {
    #[must_use]
    pub const fn tag() -> u16 {
        1303
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaturityMonthYearFormatField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MaturityMonthYearFormatField {
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
pub struct StrikeExerciseStyleField {
    pub fd: Field,
}

impl StrikeExerciseStyleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1304
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrikeExerciseStyleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StrikeExerciseStyleField {
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
pub struct SecondaryPriceLimitTypeField {
    pub fd: Field,
}

impl SecondaryPriceLimitTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1305
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecondaryPriceLimitTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecondaryPriceLimitTypeField {
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
pub struct PriceLimitTypeField {
    pub fd: Field,
}

impl PriceLimitTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1306
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PriceLimitTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PriceLimitTypeField {
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
pub struct ExecInstValueField {
    pub fd: Field,
}

impl ExecInstValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        1308
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecInstValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExecInstValueField {
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
pub struct DerivativeInstrAttribTypeField {
    pub fd: Field,
}

impl DerivativeInstrAttribTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1313
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrAttribTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrAttribTypeField {
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
pub struct DerivativeInstrAttribValueField {
    pub fd: Field,
}

impl DerivativeInstrAttribValueField {
    #[must_use]
    pub const fn tag() -> u16 {
        1314
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeInstrAttribValueField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeInstrAttribValueField {
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
pub struct DerivativePriceUnitOfMeasureField {
    pub fd: Field,
}

impl DerivativePriceUnitOfMeasureField {
    #[must_use]
    pub const fn tag() -> u16 {
        1315
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativePriceUnitOfMeasureField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativePriceUnitOfMeasureField {
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
pub struct DerivativePriceUnitOfMeasureQtyField {
    pub fd: Field,
}

impl DerivativePriceUnitOfMeasureQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1316
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativePriceUnitOfMeasureQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativePriceUnitOfMeasureQtyField {
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
pub struct DerivativeSettlMethodField {
    pub fd: Field,
}

impl DerivativeSettlMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1317
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeSettlMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeSettlMethodField {
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
pub struct DerivativePriceQuoteMethodField {
    pub fd: Field,
}

impl DerivativePriceQuoteMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1318
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativePriceQuoteMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativePriceQuoteMethodField {
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
pub struct DerivativeValuationMethodField {
    pub fd: Field,
}

impl DerivativeValuationMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1319
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeValuationMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeValuationMethodField {
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
pub struct DerivativeListMethodField {
    pub fd: Field,
}

impl DerivativeListMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1320
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeListMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeListMethodField {
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
pub struct DerivativeCapPriceField {
    pub fd: Field,
}

impl DerivativeCapPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1321
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeCapPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeCapPriceField {
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
pub struct DerivativeFloorPriceField {
    pub fd: Field,
}

impl DerivativeFloorPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1322
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeFloorPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeFloorPriceField {
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
pub struct DerivativePutOrCallField {
    pub fd: Field,
}

impl DerivativePutOrCallField {
    #[must_use]
    pub const fn tag() -> u16 {
        1323
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativePutOrCallField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativePutOrCallField {
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
pub struct ListUpdateActionField {
    pub fd: Field,
}

impl ListUpdateActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        1324
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListUpdateActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ListUpdateActionField {
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
pub struct ParentMktSegmIDField {
    pub fd: Field,
}

impl ParentMktSegmIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1325
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ParentMktSegmIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ParentMktSegmIDField {
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
pub struct TradingSessionDescField {
    pub fd: Field,
}

impl TradingSessionDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        1326
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradingSessionDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradingSessionDescField {
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
pub struct TradSesUpdateActionField {
    pub fd: Field,
}

impl TradSesUpdateActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        1327
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradSesUpdateActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradSesUpdateActionField {
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
pub struct RejectTextField {
    pub fd: Field,
}

impl RejectTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        1328
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RejectTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RejectTextField {
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
pub struct UnderlyingLegSymbolField {
    pub fd: Field,
}

impl UnderlyingLegSymbolField {
    #[must_use]
    pub const fn tag() -> u16 {
        1330
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSymbolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSymbolField {
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
pub struct UnderlyingLegSymbolSfxField {
    pub fd: Field,
}

impl UnderlyingLegSymbolSfxField {
    #[must_use]
    pub const fn tag() -> u16 {
        1331
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSymbolSfxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSymbolSfxField {
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
pub struct UnderlyingLegSecurityIDField {
    pub fd: Field,
}

impl UnderlyingLegSecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1332
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSecurityIDField {
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
pub struct UnderlyingLegSecurityIDSourceField {
    pub fd: Field,
}

impl UnderlyingLegSecurityIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1333
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSecurityIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSecurityIDSourceField {
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
pub struct UnderlyingLegSecurityAltIDField {
    pub fd: Field,
}

impl UnderlyingLegSecurityAltIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1335
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSecurityAltIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSecurityAltIDField {
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
pub struct UnderlyingLegSecurityAltIDSourceField {
    pub fd: Field,
}

impl UnderlyingLegSecurityAltIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1336
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSecurityAltIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSecurityAltIDSourceField {
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
pub struct UnderlyingLegSecurityTypeField {
    pub fd: Field,
}

impl UnderlyingLegSecurityTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1337
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSecurityTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSecurityTypeField {
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
pub struct UnderlyingLegSecuritySubTypeField {
    pub fd: Field,
}

impl UnderlyingLegSecuritySubTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1338
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSecuritySubTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSecuritySubTypeField {
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
pub struct UnderlyingLegStrikePriceField {
    pub fd: Field,
}

impl UnderlyingLegStrikePriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1340
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegStrikePriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegStrikePriceField {
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
pub struct UnderlyingLegSecurityExchangeField {
    pub fd: Field,
}

impl UnderlyingLegSecurityExchangeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1341
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSecurityExchangeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSecurityExchangeField {
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
pub struct UnderlyingLegPutOrCallField {
    pub fd: Field,
}

impl UnderlyingLegPutOrCallField {
    #[must_use]
    pub const fn tag() -> u16 {
        1343
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegPutOrCallField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegPutOrCallField {
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
pub struct UnderlyingLegCFICodeField {
    pub fd: Field,
}

impl UnderlyingLegCFICodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1344
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegCFICodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegCFICodeField {
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
pub struct ApplReqIDField {
    pub fd: Field,
}

impl ApplReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1346
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplReqIDField {
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
pub struct ApplReqTypeField {
    pub fd: Field,
}

impl ApplReqTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1347
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplReqTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplReqTypeField {
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
pub struct ApplResponseTypeField {
    pub fd: Field,
}

impl ApplResponseTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1348
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplResponseTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplResponseTypeField {
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
pub struct ApplTotalMessageCountField {
    pub fd: Field,
}

impl ApplTotalMessageCountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1349
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplTotalMessageCountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplTotalMessageCountField {
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
pub struct ApplResponseIDField {
    pub fd: Field,
}

impl ApplResponseIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1353
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplResponseIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplResponseIDField {
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
pub struct ApplResponseErrorField {
    pub fd: Field,
}

impl ApplResponseErrorField {
    #[must_use]
    pub const fn tag() -> u16 {
        1354
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplResponseErrorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplResponseErrorField {
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
pub struct RefApplIDField {
    pub fd: Field,
}

impl RefApplIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1355
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefApplIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefApplIDField {
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
pub struct ApplReportIDField {
    pub fd: Field,
}

impl ApplReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1356
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplReportIDField {
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
pub struct LegPutOrCallField {
    pub fd: Field,
}

impl LegPutOrCallField {
    #[must_use]
    pub const fn tag() -> u16 {
        1358
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegPutOrCallField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegPutOrCallField {
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
pub struct TotNoFillsField {
    pub fd: Field,
}

impl TotNoFillsField {
    #[must_use]
    pub const fn tag() -> u16 {
        1361
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TotNoFillsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TotNoFillsField {
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
pub struct FillExecIDField {
    pub fd: Field,
}

impl FillExecIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1363
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FillExecIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FillExecIDField {
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
pub struct FillPxField {
    pub fd: Field,
}

impl FillPxField {
    #[must_use]
    pub const fn tag() -> u16 {
        1364
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FillPxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FillPxField {
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
pub struct FillQtyField {
    pub fd: Field,
}

impl FillQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1365
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FillQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FillQtyField {
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
pub struct LegAllocIDField {
    pub fd: Field,
}

impl LegAllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1366
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegAllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegAllocIDField {
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
pub struct LegAllocSettlCurrencyField {
    pub fd: Field,
}

impl LegAllocSettlCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1367
    }

    // tag_type: CURRENCY
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegAllocSettlCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegAllocSettlCurrencyField {
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
pub struct TradSesEventField {
    pub fd: Field,
}

impl TradSesEventField {
    #[must_use]
    pub const fn tag() -> u16 {
        1368
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradSesEventField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradSesEventField {
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
pub struct MassActionReportIDField {
    pub fd: Field,
}

impl MassActionReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1369
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassActionReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassActionReportIDField {
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
pub struct NotAffectedOrderIDField {
    pub fd: Field,
}

impl NotAffectedOrderIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1371
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NotAffectedOrderIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NotAffectedOrderIDField {
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
pub struct NotAffOrigClOrdIDField {
    pub fd: Field,
}

impl NotAffOrigClOrdIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1372
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NotAffOrigClOrdIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NotAffOrigClOrdIDField {
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
pub struct MassActionTypeField {
    pub fd: Field,
}

impl MassActionTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1373
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassActionTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassActionTypeField {
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
pub struct MassActionScopeField {
    pub fd: Field,
}

impl MassActionScopeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1374
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassActionScopeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassActionScopeField {
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
pub struct MassActionResponseField {
    pub fd: Field,
}

impl MassActionResponseField {
    #[must_use]
    pub const fn tag() -> u16 {
        1375
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassActionResponseField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassActionResponseField {
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
pub struct MassActionRejectReasonField {
    pub fd: Field,
}

impl MassActionRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        1376
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MassActionRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MassActionRejectReasonField {
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
pub struct MultilegModelField {
    pub fd: Field,
}

impl MultilegModelField {
    #[must_use]
    pub const fn tag() -> u16 {
        1377
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MultilegModelField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MultilegModelField {
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
pub struct MultilegPriceMethodField {
    pub fd: Field,
}

impl MultilegPriceMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1378
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MultilegPriceMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MultilegPriceMethodField {
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
pub struct ContingencyTypeField {
    pub fd: Field,
}

impl ContingencyTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1385
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContingencyTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ContingencyTypeField {
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
pub struct ListRejectReasonField {
    pub fd: Field,
}

impl ListRejectReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        1386
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListRejectReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ListRejectReasonField {
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
pub struct TrdRepPartyRoleField {
    pub fd: Field,
}

impl TrdRepPartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1388
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TrdRepPartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TrdRepPartyRoleField {
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
pub struct TradePublishIndicatorField {
    pub fd: Field,
}

impl TradePublishIndicatorField {
    #[must_use]
    pub const fn tag() -> u16 {
        1390
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TradePublishIndicatorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TradePublishIndicatorField {
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
pub struct UnderlyingLegOptAttributeField {
    pub fd: Field,
}

impl UnderlyingLegOptAttributeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1391
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegOptAttributeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegOptAttributeField {
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
pub struct UnderlyingLegSecurityDescField {
    pub fd: Field,
}

impl UnderlyingLegSecurityDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        1392
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingLegSecurityDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingLegSecurityDescField {
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
pub struct MarketReqIDField {
    pub fd: Field,
}

impl MarketReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1393
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MarketReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MarketReqIDField {
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
pub struct MarketReportIDField {
    pub fd: Field,
}

impl MarketReportIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1394
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MarketReportIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MarketReportIDField {
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
pub struct MarketUpdateActionField {
    pub fd: Field,
}

impl MarketUpdateActionField {
    #[must_use]
    pub const fn tag() -> u16 {
        1395
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MarketUpdateActionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MarketUpdateActionField {
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
pub struct MarketSegmentDescField {
    pub fd: Field,
}

impl MarketSegmentDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        1396
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MarketSegmentDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MarketSegmentDescField {
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
pub struct EncryptedPasswordMethodField {
    pub fd: Field,
}

impl EncryptedPasswordMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1400
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EncryptedPasswordMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EncryptedPasswordMethodField {
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
pub struct RefApplExtIDField {
    pub fd: Field,
}

impl RefApplExtIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1406
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefApplExtIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefApplExtIDField {
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
pub struct DefaultApplExtIDField {
    pub fd: Field,
}

impl DefaultApplExtIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1407
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DefaultApplExtIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DefaultApplExtIDField {
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
pub struct DefaultCstmApplVerIDField {
    pub fd: Field,
}

impl DefaultCstmApplVerIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1408
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DefaultCstmApplVerIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DefaultCstmApplVerIDField {
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
pub struct SessionStatusField {
    pub fd: Field,
}

impl SessionStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        1409
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SessionStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SessionStatusField {
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
pub struct Nested4PartySubIDTypeField {
    pub fd: Field,
}

impl Nested4PartySubIDTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1411
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested4PartySubIDTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested4PartySubIDTypeField {
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
pub struct Nested4PartySubIDField {
    pub fd: Field,
}

impl Nested4PartySubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1412
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested4PartySubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested4PartySubIDField {
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
pub struct Nested4PartyIDField {
    pub fd: Field,
}

impl Nested4PartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1415
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested4PartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested4PartyIDField {
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
pub struct Nested4PartyIDSourceField {
    pub fd: Field,
}

impl Nested4PartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1416
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested4PartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested4PartyIDSourceField {
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
pub struct Nested4PartyRoleField {
    pub fd: Field,
}

impl Nested4PartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1417
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Nested4PartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Nested4PartyRoleField {
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
pub struct LegLastQtyField {
    pub fd: Field,
}

impl LegLastQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1418
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegLastQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegLastQtyField {
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
pub struct UnderlyingExerciseStyleField {
    pub fd: Field,
}

impl UnderlyingExerciseStyleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1419
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingExerciseStyleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingExerciseStyleField {
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
pub struct LegExerciseStyleField {
    pub fd: Field,
}

impl LegExerciseStyleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1420
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegExerciseStyleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegExerciseStyleField {
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
pub struct LegPriceUnitOfMeasureField {
    pub fd: Field,
}

impl LegPriceUnitOfMeasureField {
    #[must_use]
    pub const fn tag() -> u16 {
        1421
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegPriceUnitOfMeasureField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegPriceUnitOfMeasureField {
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
pub struct LegPriceUnitOfMeasureQtyField {
    pub fd: Field,
}

impl LegPriceUnitOfMeasureQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1422
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegPriceUnitOfMeasureQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegPriceUnitOfMeasureQtyField {
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
pub struct UnderlyingUnitOfMeasureQtyField {
    pub fd: Field,
}

impl UnderlyingUnitOfMeasureQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1423
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingUnitOfMeasureQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingUnitOfMeasureQtyField {
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
pub struct UnderlyingPriceUnitOfMeasureField {
    pub fd: Field,
}

impl UnderlyingPriceUnitOfMeasureField {
    #[must_use]
    pub const fn tag() -> u16 {
        1424
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingPriceUnitOfMeasureField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingPriceUnitOfMeasureField {
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
pub struct UnderlyingPriceUnitOfMeasureQtyField {
    pub fd: Field,
}

impl UnderlyingPriceUnitOfMeasureQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        1425
    }

    // tag_type: QTY
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingPriceUnitOfMeasureQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingPriceUnitOfMeasureQtyField {
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
pub struct ApplReportTypeField {
    pub fd: Field,
}

impl ApplReportTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1426
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ApplReportTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ApplReportTypeField {
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
pub struct SideExecIDField {
    pub fd: Field,
}

impl SideExecIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1427
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideExecIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideExecIDField {
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
pub struct OrderDelayField {
    pub fd: Field,
}

impl OrderDelayField {
    #[must_use]
    pub const fn tag() -> u16 {
        1428
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderDelayField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderDelayField {
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
pub struct OrderDelayUnitField {
    pub fd: Field,
}

impl OrderDelayUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1429
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderDelayUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderDelayUnitField {
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
pub struct VenueTypeField {
    pub fd: Field,
}

impl VenueTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1430
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for VenueTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for VenueTypeField {
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
pub struct RefOrdIDReasonField {
    pub fd: Field,
}

impl RefOrdIDReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        1431
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefOrdIDReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefOrdIDReasonField {
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
pub struct OrigCustOrderCapacityField {
    pub fd: Field,
}

impl OrigCustOrderCapacityField {
    #[must_use]
    pub const fn tag() -> u16 {
        1432
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrigCustOrderCapacityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrigCustOrderCapacityField {
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
pub struct RefApplReqIDField {
    pub fd: Field,
}

impl RefApplReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1433
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefApplReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefApplReqIDField {
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
pub struct ModelTypeField {
    pub fd: Field,
}

impl ModelTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1434
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ModelTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ModelTypeField {
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
pub struct ContractMultiplierUnitField {
    pub fd: Field,
}

impl ContractMultiplierUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1435
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ContractMultiplierUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ContractMultiplierUnitField {
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
pub struct LegContractMultiplierUnitField {
    pub fd: Field,
}

impl LegContractMultiplierUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1436
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegContractMultiplierUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegContractMultiplierUnitField {
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
pub struct UnderlyingContractMultiplierUnitField {
    pub fd: Field,
}

impl UnderlyingContractMultiplierUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1437
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingContractMultiplierUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingContractMultiplierUnitField {
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
pub struct DerivativeContractMultiplierUnitField {
    pub fd: Field,
}

impl DerivativeContractMultiplierUnitField {
    #[must_use]
    pub const fn tag() -> u16 {
        1438
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeContractMultiplierUnitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeContractMultiplierUnitField {
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
pub struct FlowScheduleTypeField {
    pub fd: Field,
}

impl FlowScheduleTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1439
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FlowScheduleTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FlowScheduleTypeField {
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
pub struct LegFlowScheduleTypeField {
    pub fd: Field,
}

impl LegFlowScheduleTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1440
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LegFlowScheduleTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LegFlowScheduleTypeField {
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
pub struct UnderlyingFlowScheduleTypeField {
    pub fd: Field,
}

impl UnderlyingFlowScheduleTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1441
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingFlowScheduleTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingFlowScheduleTypeField {
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
pub struct DerivativeFlowScheduleTypeField {
    pub fd: Field,
}

impl DerivativeFlowScheduleTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1442
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DerivativeFlowScheduleTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DerivativeFlowScheduleTypeField {
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
pub struct FillLiquidityIndField {
    pub fd: Field,
}

impl FillLiquidityIndField {
    #[must_use]
    pub const fn tag() -> u16 {
        1443
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for FillLiquidityIndField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for FillLiquidityIndField {
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
pub struct SideLiquidityIndField {
    pub fd: Field,
}

impl SideLiquidityIndField {
    #[must_use]
    pub const fn tag() -> u16 {
        1444
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideLiquidityIndField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideLiquidityIndField {
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
pub struct RateSourceField {
    pub fd: Field,
}

impl RateSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1446
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RateSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RateSourceField {
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
pub struct RateSourceTypeField {
    pub fd: Field,
}

impl RateSourceTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1447
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RateSourceTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RateSourceTypeField {
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
pub struct ReferencePageField {
    pub fd: Field,
}

impl ReferencePageField {
    #[must_use]
    pub const fn tag() -> u16 {
        1448
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ReferencePageField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ReferencePageField {
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
pub struct RestructuringTypeField {
    pub fd: Field,
}

impl RestructuringTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1449
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RestructuringTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RestructuringTypeField {
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
pub struct SeniorityField {
    pub fd: Field,
}

impl SeniorityField {
    #[must_use]
    pub const fn tag() -> u16 {
        1450
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SeniorityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SeniorityField {
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
pub struct UnderlyingRestructuringTypeField {
    pub fd: Field,
}

impl UnderlyingRestructuringTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1453
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingRestructuringTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingRestructuringTypeField {
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
pub struct UnderlyingSeniorityField {
    pub fd: Field,
}

impl UnderlyingSeniorityField {
    #[must_use]
    pub const fn tag() -> u16 {
        1454
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingSeniorityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingSeniorityField {
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
pub struct TargetPartyIDField {
    pub fd: Field,
}

impl TargetPartyIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1462
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetPartyIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TargetPartyIDField {
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
pub struct TargetPartyIDSourceField {
    pub fd: Field,
}

impl TargetPartyIDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1463
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetPartyIDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TargetPartyIDSourceField {
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
pub struct TargetPartyRoleField {
    pub fd: Field,
}

impl TargetPartyRoleField {
    #[must_use]
    pub const fn tag() -> u16 {
        1464
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetPartyRoleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TargetPartyRoleField {
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
pub struct SecurityListIDField {
    pub fd: Field,
}

impl SecurityListIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1465
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityListIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityListIDField {
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
pub struct SecurityListRefIDField {
    pub fd: Field,
}

impl SecurityListRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1466
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityListRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityListRefIDField {
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
pub struct SecurityListDescField {
    pub fd: Field,
}

impl SecurityListDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        1467
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityListDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityListDescField {
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
pub struct SecurityListTypeField {
    pub fd: Field,
}

impl SecurityListTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1470
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityListTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityListTypeField {
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
pub struct SecurityListTypeSourceField {
    pub fd: Field,
}

impl SecurityListTypeSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1471
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityListTypeSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityListTypeSourceField {
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
pub struct NewsIDField {
    pub fd: Field,
}

impl NewsIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1472
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NewsIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NewsIDField {
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
pub struct NewsCategoryField {
    pub fd: Field,
}

impl NewsCategoryField {
    #[must_use]
    pub const fn tag() -> u16 {
        1473
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NewsCategoryField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NewsCategoryField {
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
pub struct NewsRefIDField {
    pub fd: Field,
}

impl NewsRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1476
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NewsRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NewsRefIDField {
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
pub struct NewsRefTypeField {
    pub fd: Field,
}

impl NewsRefTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1477
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NewsRefTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NewsRefTypeField {
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
pub struct StrikePriceDeterminationMethodField {
    pub fd: Field,
}

impl StrikePriceDeterminationMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1478
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrikePriceDeterminationMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StrikePriceDeterminationMethodField {
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
pub struct StrikePriceBoundaryMethodField {
    pub fd: Field,
}

impl StrikePriceBoundaryMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1479
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StrikePriceBoundaryMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StrikePriceBoundaryMethodField {
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
pub struct UnderlyingPriceDeterminationMethodField {
    pub fd: Field,
}

impl UnderlyingPriceDeterminationMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1481
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UnderlyingPriceDeterminationMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UnderlyingPriceDeterminationMethodField {
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
pub struct OptPayoutTypeField {
    pub fd: Field,
}

impl OptPayoutTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1482
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OptPayoutTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OptPayoutTypeField {
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
pub struct ComplexEventTypeField {
    pub fd: Field,
}

impl ComplexEventTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1484
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ComplexEventTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ComplexEventTypeField {
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
pub struct ComplexOptPayoutAmountField {
    pub fd: Field,
}

impl ComplexOptPayoutAmountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1485
    }

    // tag_type: AMT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ComplexOptPayoutAmountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ComplexOptPayoutAmountField {
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
pub struct ComplexEventPriceField {
    pub fd: Field,
}

impl ComplexEventPriceField {
    #[must_use]
    pub const fn tag() -> u16 {
        1486
    }

    // tag_type: PRICE
    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ComplexEventPriceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ComplexEventPriceField {
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
pub struct ComplexEventPriceBoundaryMethodField {
    pub fd: Field,
}

impl ComplexEventPriceBoundaryMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        1487
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ComplexEventPriceBoundaryMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ComplexEventPriceBoundaryMethodField {
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
pub struct ComplexEventPriceTimeTypeField {
    pub fd: Field,
}

impl ComplexEventPriceTimeTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1489
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ComplexEventPriceTimeTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ComplexEventPriceTimeTypeField {
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
pub struct ComplexEventConditionField {
    pub fd: Field,
}

impl ComplexEventConditionField {
    #[must_use]
    pub const fn tag() -> u16 {
        1490
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ComplexEventConditionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ComplexEventConditionField {
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
pub struct StreamAsgnReqIDField {
    pub fd: Field,
}

impl StreamAsgnReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1497
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StreamAsgnReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StreamAsgnReqIDField {
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
pub struct StreamAsgnReqTypeField {
    pub fd: Field,
}

impl StreamAsgnReqTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1498
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StreamAsgnReqTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StreamAsgnReqTypeField {
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
pub struct MDStreamIDField {
    pub fd: Field,
}

impl MDStreamIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1500
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MDStreamIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MDStreamIDField {
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
pub struct StreamAsgnRptIDField {
    pub fd: Field,
}

impl StreamAsgnRptIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        1501
    }

    // tag_type: STRING
    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StreamAsgnRptIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StreamAsgnRptIDField {
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
pub struct StreamAsgnRejReasonField {
    pub fd: Field,
}

impl StreamAsgnRejReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        1502
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StreamAsgnRejReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StreamAsgnRejReasonField {
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
pub struct StreamAsgnAckTypeField {
    pub fd: Field,
}

impl StreamAsgnAckTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1503
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StreamAsgnAckTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StreamAsgnAckTypeField {
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
pub struct StreamAsgnTypeField {
    pub fd: Field,
}

impl StreamAsgnTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        1617
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StreamAsgnTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for StreamAsgnTypeField {
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