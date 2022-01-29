
# https://simplabs.com/blog/2020/12/31/xml-and-rust/

import xml.etree.ElementTree as ET
root = ET.parse('FIX42.xml').getroot()

enum_variants = {
    'STRING': 'String',
    'CHAR': 'Char',
    'PRICE': 'Price',
    'INT': 'Int',
    'AMT': 'Int',
    'QTY': 'Int',
    'CURRENCY': 'Currency'
}

method_names = {
    'STRING': 'str',
    'CHAR': 'char',
    'PRICE': 'decimal',
    'INT': 'i32',
    'AMT': 'i32',
    'QTY': 'i32',
    'CURRENCY': 'str'
}

struct_types = {
    'STRING': 'String',
    'CHAR': 'char',
    'PRICE': 'Decimal',
    'INT': 'i32',
    'AMT': 'i32',
    'QTY': 'i32',
    'CURRENCY': 'String'
}

return_types = {
    'STRING': '&str',
    'CHAR': 'char',
    'PRICE': 'Decimal',
    'INT': 'i32',
    'AMT': 'i32',
    'QTY': 'i32',
    'CURRENCY': '&str'
}

exclude_types = [
    'MULTIPLEVALUESTRING'
]

seen_types = []

##
## Field enum
##
print('\n\n\n')
print(f"#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]")
print(f"pub enum Field {{")
for tag_type in root.findall('fields/field'):
    tag_num  = tag_type.get('number')
    tag_name = tag_type.get('name')
    tag_type = tag_type.get('type')
    if tag_type not in exclude_types and tag_type in enum_variants and tag_type not in seen_types:
        print(f"    {enum_variants.get(tag_type)}(Tag, {struct_types.get(tag_type)}),")
        seen_types.append(tag_type)
print(f"}}")

##
## Field impl
##
seen_types.clear()
seen_enum_variants = []
print(f"impl Field {{")
print()

for tag_type in root.findall('fields/field'):
    tag_num  = tag_type.get('number')
    tag_name = tag_type.get('name')
    tag_type = tag_type.get('type')
    return_type = return_types.get(tag_type)
    if tag_type not in exclude_types and tag_type in enum_variants and return_type not in seen_types:
        print(f"    ///")
        print(f"    /// # Errors")
        print(f"    ///")
        print(f"    pub fn as_{method_names.get(tag_type)}_safe(&self) -> Result<{return_types.get(tag_type)}, FieldTypeMismatchError> {{")
        print(f"        match self {{")
        print(f"            Field::{enum_variants.get(tag_type)}(_, v) => Ok(v),")
        print(f"            _ => Err(FieldTypeMismatchError {{}}),")
        print(f"        }}")
        print(f"    }}")
        print()
        print(f"    ///")
        print(f"    /// # Errors")
        print(f"    ///")
        print(f"    /// # Panics")
        print(f"    ///")
        # todo i32 can be `const fn`
        print(f"    pub fn as_{method_names.get(tag_type)}(&self) -> {return_type} {{")
        print(f"        match self {{")
        print(f"            Field::{enum_variants.get(tag_type)}(_, v) => v,")
        print(f"            _ => panic!(""),")
        print(f"        }}")
        print(f"    }}")
        print()
        seen_enum_variants.append(enum_variants.get(tag_type))
        seen_types.append(return_type)

# Field::tag(&self)
print(f"    #[must_use]")
print(f"    pub const fn tag(&self) -> u16 {{")
print(f"        match self {{")
variant_count = len(enum_variants)
count = 1
for variant in enum_variants.values():
    print(f"            {'| ' if count > 1 else ''}Field::{variant}(t, _){' => *t,' if count == variant_count else ''}")
    count += 1
print(f"    }}")

# Field::to_delimited_string(&self)

seen_var_return_types = []
print()
print(f"    #[must_use]")
print(f"    pub fn to_delimited_string(&self, separator: char) -> String {{")
print(f"        match self {{")

for tag_type in root.findall('fields/field'):
    tag_num  = tag_type.get('number')
    tag_name = tag_type.get('name')
    tag_type = tag_type.get('type')
    return_type = return_types.get(tag_type)
    if tag_type not in exclude_types and tag_type in enum_variants and return_type not in seen_var_return_types:
        print(f"            // {return_type}")
        print(f"            Field::{enum_variants.get(tag_type)}(t, v) => format!(\"{{}}={{}}{{}}\", t, v, separator),")
        seen_var_return_types.append(return_type)
print(f"        }}")
print(f"    }}")


print(f"}}")
## ---- End Field Impl


## ---- TryFrom<String> for Field

# enum_type -> parsing function string
# e.g. String -> Self::String(tag, s_value.to_string())
#      Int    -> Self::Int(tag, str::parse::<i32>(s_value).unwrap())
enum_variants_parse_function = {
    'String': 'Self::String(tag, s_value.to_string())',
    'Char': 'Self::Char(tag, str::parse::<char>(s_value).unwrap())',
    'Price': 'Self::Price(tag, Decimal::from_str(s_value).unwrap())',
    'Int': 'Self::Int(tag, str::parse::<i32>(s_value).unwrap())',
    'Currency': 'Err(())',
}

seen_types.clear()
# enum_var -> [tag_nums...]
tag_num_enum_variant = {}
for tag_type in root.findall('fields/field'):
    tag_num  = tag_type.get('number')
    tag_name = tag_type.get('name')
    tag_type = tag_type.get('type')
    return_type = return_types.get(tag_type)
    if tag_type not in exclude_types and tag_type in enum_variants:
        if tag_num_enum_variant.get(enum_variants.get(tag_type)) is None:
            tag_num_enum_variant[enum_variants.get(tag_type)] = []
        tag_num_enum_variant.get(enum_variants.get(tag_type)).append(tag_num)

print(f"// parse string (35=D) into Field{{35, \"D\"}}")
print(f"impl TryFrom<String> for Field {{")
print(f"    type Error = (); // todo error type...")
print(f"      ")
print(f"    fn try_from(s: String) -> Result<Self, Self::Error> {{")
print(f"        println!(\"From<String> for Field: {{}}\", &s);")
print(f"        let two_parts = s.split_once('=');")
print(f"        match two_parts {{")
print(f"            Some((s_tag, s_value)) => {{")
print(f"                println!(\"parsing tag: {{}}, field: {{}} into Field\", s_tag, s_value);")
print(f"                  ")
print(f"                // figure out the tag")
print(f"                let tag: u16 = s_tag.parse::<u16>().unwrap();")
print(f"                   ")
print(f"                // build field using the tag & value")
print(f"                match tag {{")
for enum_key in tag_num_enum_variant.keys():
    print(f"                    {' | '.join(tag_num_enum_variant.get(enum_key))} => Ok({enum_variants_parse_function.get(enum_key)}),")
print(f"                    _ => Err(()),")
print(f"                }}")
print(f"            }}")
print(f"            None => Err(()),")
print(f"        }}")
print(f"    }}")
print(f"}}")













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