
# https://simplabs.com/blog/2020/12/31/xml-and-rust/

import xml.etree.ElementTree as ET
root = ET.parse('FIX42.xml').getroot()

method_types = {
    'STRING': 'String',
    'CHAR': 'Char',
    'PRICE': 'Price',
    'INT': 'Int',
    'AMT': 'Amt',
    'QTY': 'Qty',
    'CURRENCY': 'Currency'
}

struct_types = {
    'STRING': 'String',
    'CHAR': 'char',
    'PRICE': 'f32',
    'INT': 'i32',
    'AMT': 'i32',
    'QTY': 'i32',
    'CURRENCY': 'String'
}

return_types = {
    'STRING': '&str',
    'CHAR': 'char',
    'PRICE': 'f32',
    'INT': 'i32',
    'AMT': 'i32',
    'QTY': 'i32',
    'CURRENCY': '&str'
}

exclude_types = [
    'MULTIPLEVALUESTRING'
]

seen_types = []

# print(f"use std::fmt::{{ Display, Formatter, Result }};")
print('\n\n\n')
print(f"#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]")
print(f"pub enum Field {{")
#
for tag_type in root.findall('fields/field'):
    tag_num  = tag_type.get('number')
    tag_name = tag_type.get('name')
    tag_type = tag_type.get('type')
    if tag_type not in exclude_types and tag_type in method_types and tag_type not in seen_types:
        print(f"    {method_types.get(tag_type)}(Tag, {struct_types.get(tag_type)}),")
        seen_types.append(tag_type)
print(f"}}")

# #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
# pub enum Field {
#     Int(Tag, i32),
#     TagNum(Tag, u128), // todo check
#     SeqNum(Tag, u128),
#
#     String(Tag, String),
#     Char(Tag, char),
# }


def old_version():
    # trait for the type
    if tag_type in method_types and tag_type not in seen_types and tag_type not in exclude_types:
        print(f"trait Has{method_types.get(tag_type)}Value {{")
        print(f"    fn get_value(&self) -> {return_types.get(tag_type)};")
        print(f"    fn get_tag(&self) -> u16;")
        print(f"}}")

    if tag_type in method_types:
        print(f"struct {tag_name}Field {{")
        print(f"    tag: u16,")
        print(f"    value: {struct_types.get(tag_type)},")
        print(f"}}")
        print(f"impl Has{method_types.get(tag_type)}Value for {tag_name}Field {{")
        print(f"    fn get_value(&self) -> {return_types.get(tag_type)} {{")
        print(f"        {'&' if return_types.get(tag_type).startswith('&') else ''}self.value")
        print(f"    }}")
        print(f"    fn get_tag(&self) -> u16 {{")
        print(f"        {tag_num}")
        print(f"    }}")
        print(f"}}")
        print(f"impl Display for {tag_name}Field {{")
        print(f"    fn fmt(&self, f: &mut Formatter<'_>) -> Result {{")
        print(f"        write!(f, \"\")")
        print(f"    }}")
        print(f"}}")

    seen_types.append(tag_type)


# root.findall('header/field')
# root.findall('messages/message')
# root.findall('trailer')
# root.findall('fields/field')


# trait HasStringValue {
#     fn get_value() -> &str;
# }
#
# struct Account {
#     tag: u16,
#     value: String
# }
#
# impl HasStringValue for Account {
#     fn get_value(&self) -> &str {
#         &self.value
#     }
# }