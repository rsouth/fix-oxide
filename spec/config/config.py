from spec.datatypes.datatypes import Field

DEFAULT_KEY = 'default'
ERROR = "ERROR"

def _init_return_types():
    return {
        'STRING': {DEFAULT_KEY: '&str'},
        'CHAR': {
            DEFAULT_KEY: 'char',
            'fix40': '&str',
            'fix41': '&str'
        },
        'PRICE': {DEFAULT_KEY: '&Decimal'},
        'INT': {DEFAULT_KEY: 'i32'},
        'AMT': {DEFAULT_KEY: 'i32'},
        'QTY': {DEFAULT_KEY: 'i32'},
        'CURRENCY': {DEFAULT_KEY: '&str'},
    }

# todo review these mappings; may need types other than i32 (Decimal/other)
def _init_struct_types():
    return {
        'STRING': {DEFAULT_KEY: 'String'},
        # todo char fix < 4.2
        'CHAR': {
            DEFAULT_KEY: 'char',
            'fix40': 'String',
            'fix41': 'String'
        },
        'PRICE': {DEFAULT_KEY: 'Decimal'},
        'INT': {DEFAULT_KEY: 'i32'},
        'AMT': {DEFAULT_KEY: 'i32'},
        'QTY': {DEFAULT_KEY: 'i32'},
        'CURRENCY': {DEFAULT_KEY: 'String'},
    }


def _init_method_names():
    return {
        'STRING': {DEFAULT_KEY: 'str'},
        # todo char < fix 4.2
        'CHAR': {
            DEFAULT_KEY: 'char',
            'fix40': 'str',
            'fix41': 'str'
        },
        'PRICE': {DEFAULT_KEY: 'decimal'},
        'INT': {DEFAULT_KEY: 'i32'},
        'AMT': {DEFAULT_KEY: 'i32'},
        'QTY': {DEFAULT_KEY: 'i32'},
        'CURRENCY': {DEFAULT_KEY: 'str'},
    }


def _init_enum_variant():
    return {
        'STRING': {DEFAULT_KEY: 'String'},
        'CHAR': {
            DEFAULT_KEY: 'Char',
            'fix40': 'String',
            'fix41': 'String'
        },
        'PRICE': {DEFAULT_KEY: 'Decimal'},
        'INT': {DEFAULT_KEY: 'Int'},
        'AMT': {DEFAULT_KEY: 'Int'},
        'QTY': {DEFAULT_KEY: 'Int'},
        'CURRENCY': {DEFAULT_KEY: 'String'},
    }

# Enum variant to parsing function
def _init_parse_functions():
    return {
        'String': 'Field::String(tag, s_value.to_string())',
        'Char': 'Field::Char(tag, str::parse::<char>(s_value).unwrap())',
        'Decimal': 'Field::Decimal(tag, Decimal::from_str(s_value).unwrap())',
        'Int': 'Field::Int(tag, str::parse::<i32>(s_value).unwrap())',
        'Currency': 'Field::String(tag, s_value.to_string())',
    }

class Config:
    def __init__(self):
        self._return_types = _init_return_types()
        self._struct_types = _init_struct_types()
        self._method_names = _init_method_names()
        self._enum_varient = _init_enum_variant()
        self._parse_functn = _init_parse_functions()

    def return_type_for(self, field: Field) -> str:
        """ Maps the FIX type (STRING, CHAR...) to the Rust return type """
        ret_types = self._return_types.get(field.type)
        return ret_types.get(field.version, ret_types.get(DEFAULT_KEY, ERROR))

    def struct_type_for(self, field: Field) -> str:
        """ Maps the FIX type (STRING, CHAR...) to the Rust type """
        st_types = self._struct_types.get(field.type)
        return st_types.get(field.version, st_types.get(DEFAULT_KEY, ERROR))

    def method_name_for(self, field: Field) -> str:
        m_name = self._method_names.get(field.type)
        return m_name.get(field.version, m_name.get(DEFAULT_KEY, ERROR))

    def enum_variant_for(self, field: Field) -> str:
        m_name = self._enum_varient.get(field.type)
        return m_name.get(field.version, m_name.get(DEFAULT_KEY, ERROR))

    def all_enum_variants_for(self, field: Field) -> [str]:
        m_name = self._enum_varient.get(field.type)
        return list(m_name.values())

    def parse_fn_for(self, enum_type: str):
        return self._parse_functn.get(enum_type, ERROR)

    def is_excluded(self, fix_type: str) -> bool:
        return fix_type in [ 'MULTIPLEVALUESTRING', 'DATA' ]

    def should_process_type(self, fix_type: str):
        return self.is_supported(fix_type) and not self.is_excluded(fix_type)

    def is_supported(self, fix_type: str):
        return fix_type in self._struct_types
