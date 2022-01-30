# Mapping on FIX types to Rust Field return types
return_types = {
    'STRING': '&str',
    'CHAR': 'char',
    'PRICE': 'Decimal',
    'INT': 'i32',
    'AMT': 'i32',
    'QTY': 'i32',
    'CURRENCY': '&str'
}

# Mapping FIX types to Rust Field enum variant types
# e.g.  enum Field { String(Tag, <struct_type>), Int(Tag, <struct_type>) }
struct_types = {
    'STRING': 'String',
    'CHAR': 'char',
    'PRICE': 'Decimal',
    'INT': 'i32',
    'AMT': 'i32',
    'QTY': 'i32',
    'CURRENCY': 'String'
}

# Mapping FIX types to method names
# e.g. get_<str> get_<char>
method_names = {
    'STRING': 'str',
    'CHAR': 'char',
    'PRICE': 'decimal',
    'INT': 'i32',
    'AMT': 'i32',
    'QTY': 'i32',
    'CURRENCY': 'str'
}

# Mapping FIX types to enum variants
# Field::String, Field::Char
enum_variants = {
    'STRING': 'String',
    'CHAR': 'Char',
    'PRICE': 'Decimal',
    'INT': 'Int',
    'AMT': 'Int',
    'QTY': 'Int',
    'CURRENCY': 'String'
}

# Exclude FIX types not (yet?) supported
exclude_types = [
    'MULTIPLEVALUESTRING', 'DATA'
]

# enum_type -> parsing function string
# e.g. String -> Self::String(tag, s_value.to_string())
#      Int    -> Self::Int(tag, str::parse::<i32>(s_value).unwrap())
enum_variants_parse_function = {
    'String': 'Self::String(tag, s_value.to_string())',
    'Char': 'Self::Char(tag, str::parse::<char>(s_value).unwrap())',
    'Decimal': 'Self::Decimal(tag, Decimal::from_str(s_value).unwrap())',
    'Int': 'Self::Int(tag, str::parse::<i32>(s_value).unwrap())',
    'Currency': 'Self::String(tag, s_value.to_string())',
}