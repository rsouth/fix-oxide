import typing
from typing import TextIO

from ..config.config import return_types, struct_types, enum_variants, exclude_types, method_names
from ..datatypes.field_utils import FieldUtils
from ..datatypes.datatypes import Field


class Writer:

    def __init__(self, output_file: str, import_fields: bool):
        self.fd = self.init(output_file)
        self._write_imports(import_fields)

    def init(self, output_file: str) -> typing.TextIO:
        fd = open(output_file.lower(), "w", newline='\n')
        return fd

    def _write_imports(self, import_fields: bool):
        self.write("use std::fmt::Formatter;")
        self.write("use std::str::FromStr;")
        self.write("use itertools::Itertools;")
        self.write_newline()

        self.write("use rust_decimal::Decimal;")
        self.write_newline()

        self.write("use crate::model::field::{FieldSet, FieldTypeMismatchError};")
        self.write("use crate::model::message_type::UnknownMsgTypeError;")
        self.write("use crate::model::BeginString;")

        if import_fields:
            self.write("use crate::model::generated::fields::Field;")
        self.write_newline()

    def close(self):
        self.fd.close()

    def write(self, output: str):
        self.fd.write(output)
        self.fd.write('\n')

    def write_newline(self):
        self.fd.write('\n')

    def write_field_enum(self, xml_files: [str], fields: [Field]):
        self.write(f"#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]")
        self.write(f"pub enum Field {{")
        for field_type in FieldUtils.unique_field_enum_variants(fields):
                self.write(f"    {enum_variants.get(field_type)}(u16, {struct_types.get(field_type)}),")
        self.write(f"}}")
        self.write_newline()

        self.write(f"impl Field {{")
        self.write(f"   pub fn crack(bs: BeginString, to_parse: &str) -> Result<Field, ()> {{")
        self.write(f"       match bs {{")

        for spec_file in xml_files:
            spec_version = spec_file.split('.')[0]
            self.write(f"           BeginString::{spec_version.title()} => Ok(crate::model::generated::{spec_version.lower()}::{spec_version.upper()}CrackerUtils::crack(")
            self.write(f"               to_parse,")
            self.write(f"           ).unwrap()),")

        self.write(f"       }}")
        # self.write(f"       Err(())")
        self.write(f"   }}")
        self.write(f"}}")

    def write_field_struct_impl(self, fields: [Field]):
        _seen_types = []
        _seen_enum_variants = []
        self.write(f"impl Field {{")
        self.write_newline()

        for field in fields:
            return_type = return_types.get(field.type)
            if field.type not in exclude_types and return_type not in _seen_types:
                self.write(f"    ///")
                self.write(f"    /// # Errors")
                self.write(f"    ///")
                self.write(f"    pub{'' if return_type.startswith('&') else ' const'} fn as_{method_names.get(field.type)}_safe(&self) -> Result<{return_type}, FieldTypeMismatchError> {{")
                self.write(f"        match self {{")
                self.write(f"            Field::{enum_variants.get(field.type)}(_, v) => Ok({'' if return_type.startswith('&') else '*'}v),")
                self.write(f"            _ => Err(FieldTypeMismatchError {{}}),")
                self.write(f"        }}")
                self.write(f"    }}")
                self.write_newline()
                self.write(f"    ///")
                self.write(f"    /// # Errors")
                self.write(f"    ///")
                self.write(f"    /// # Panics")
                self.write(f"    ///")
                self.write(f"    pub fn as_{method_names.get(field.type)}(&self) -> {return_type} {{")
                self.write(f"        match self {{")
                self.write(f"            Field::{enum_variants.get(field.type)}(_, v) => {'' if return_type.startswith('&') else '*'}v,")
                self.write(f"            _ => panic!(""),")
                self.write(f"        }}")
                self.write(f"    }}")
                self.write_newline()
                _seen_enum_variants.append(enum_variants.get(field.type))
                _seen_types.append(return_type)

        # Field::tag(&self)
        self.write(f"    #[must_use]")
        self.write(f"    pub const fn tag(&self) -> u16 {{")
        self.write(f"        match self {{")
        count = 1
        _seen_enum_variants = []
        for variant in enum_variants.values():
            if variant not in _seen_enum_variants:
                self.write(f"            {'| ' if count > 1 else ''}Field::{variant}(t, _) ")
                _seen_enum_variants.append(variant)
            count += 1
        self.write(f"            => *t,")
        self.write(f"    }}")
        self.write(f"}}")

        # Field::to_delimited_string(&self)
        _seen_var_return_types = []
        self.write_newline()
        self.write(f"    #[must_use]")
        self.write(f"    pub fn to_delimited_string(&self, separator: char) -> String {{")
        self.write(f"        match self {{")

        _seen_enum_variants.clear()
        for field in fields:
            return_type = return_types.get(field.type)
            if field.type not in exclude_types and field.type in enum_variants and enum_variants.get(field.type) not in _seen_enum_variants:
                self.write(f"            // {return_type}")
                self.write(f"            Field::{enum_variants.get(field.type)}(t, v) => format!(\"{{}}={{}}{{}}\", t, v, separator),")
                _seen_enum_variants.append(enum_variants.get(field.type))
        self.write(f"        }}")
        self.write(f"    }}")
        self.write(f"}}")
        ## ---- End Field Impl


    def write_custom_field_impls(self, fields: [Field]):
        #
        # impl generic (custom) field types (string, int etc)
        #
        seen_types = []
        for field_type in enum_variants.keys():
            is_ref_return_type = field_type in ['STRING']
            if enum_variants.get(field_type) not in seen_types:
                self.write(f"pub struct {enum_variants.get(field_type)}Field {{")
                self.write(f"    tag: u16,")
                self.write(f"    fd: Field,")
                self.write(f"}}")
                self.write_newline()
                self.write(f"impl {enum_variants.get(field_type)}Field {{")
                self.write(f"    const fn tag(&self) -> u16 {{")
                self.write(f"        self.tag")
                self.write(f"    }}")
                self.write_newline()
                self.write(f"    fn value(&self) -> {return_types.get(field_type)} {{")
                self.write(f"        match &self.fd {{")
                self.write(f"            Field::{enum_variants.get(field_type)}(_, v) => {'' if is_ref_return_type else '*'}v,")
                self.write(f"            _ => panic!(\"\"),")
                self.write(f"        }}")
                self.write(f"    }}")
                self.write(f"}}")
                self.write_newline()
                self.write(f"impl std::fmt::Display for {enum_variants.get(field_type)}Field {{")
                self.write(f"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{")
                self.write(f"        write!(f, \"{{}}={{}}|\", self.tag(), self.value())")
                self.write(f"    }}")
                self.write(f"}}")
            seen_types.append(enum_variants.get(field_type))


    def write_field_impls(self, spec_version: str, fields: [Field]):
        ## ---- TryFrom<String> for Field
        _seen_types = []
        # enum_var -> [tag_nums...]
        _tag_num_enum_variant = {}
        for field in fields:
            from spec.config.config import enum_variants_parse_function
            if field.type not in exclude_types and field.type in enum_variants:
                enum_type = enum_variants.get(field.type)
                if _tag_num_enum_variant.get(enum_type) is None:
                    _tag_num_enum_variant[enum_type] = []
                _tag_num_enum_variant.get(enum_type).append(str(field.tag))

        self.write(f"pub struct {spec_version}CrackerUtils;")
        self.write(f"// parse string (35=D) into Field{{35, \"D\"}}")
        self.write(f"impl {spec_version}CrackerUtils {{")
        self.write(f"    pub fn crack(s: &str) -> Result<Field, ()> {{")
        self.write(f"        println!(\"crack for Field: {{}}\", &s);")
        self.write(f"        let two_parts = s.split_once('=');")
        self.write(f"        match two_parts {{")
        self.write(f"            Some((s_tag, s_value)) => {{")
        self.write(f"                println!(\"parsing tag: {{}}, field: {{}} into Field\", s_tag, s_value);")
        self.write(f"                  ")
        self.write(f"                // figure out the tag")
        self.write(f"                let tag: u16 = s_tag.parse::<u16>().unwrap();")
        self.write(f"                   ")
        self.write(f"                // build field using the tag & value")
        self.write(f"                match tag {{")
        for enum_key in _tag_num_enum_variant.keys():
            self.write(f"                    {' | '.join(_tag_num_enum_variant.get(enum_key))} => Ok({enum_variants_parse_function.get(enum_key)}),")
            self.write_newline()
        self.write(f"                    _ => Err(()),")
        self.write(f"                }}")
        self.write(f"            }}")
        self.write(f"            None => Err(()),")
        self.write(f"        }}")
        self.write(f"    }}")
        self.write(f"}}")

        ##
        ## Generate Field Types
        ##

        self.write_newline()
        enum_type_return_type = {}
        for field in fields:
            return_type = return_types.get(field.type)
            if field.type not in exclude_types and field.type in enum_variants:
                self.write(f"#[derive(Debug, Clone, Eq, PartialEq)]")
                self.write(f"pub struct {field.name}Field {{")
                self.write(f"    pub fd: Field,")
                self.write(f"}}")
                self.write_newline()

                self.write(f"impl {field.name}Field {{")
                self.write(f"    #[must_use]")
                self.write(f"    pub const fn tag() -> u16 {{")
                self.write(f"        {field.tag}")
                self.write(f"    }}")
                self.write_newline()

                self.write(f"    // tag_type: {field.type}")
                self.write(f"    fn value(&self) -> {return_types.get(field.type)} {{")
                self.write(f"        match &self.fd {{")
                self.write(f"            Field::{enum_variants.get(field.type)}(_, v) => {'' if return_type.startswith('&') else '*'}v,")
                self.write(f"            _ => panic!(\"\"),")
                self.write(f"        }}")
                self.write(f"    }}")
                self.write(f"}}")

                self.write_newline()
                self.write(f"impl std::fmt::Display for {field.name}Field {{")
                self.write(f"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{")
                self.write(f"        write!(f, \"{{}}={{}}|\", Self::tag(), self.value())")
                self.write(f"    }}")
                self.write(f"}}")

                ##
                self.write(f"impl TryFrom<&Field> for {field.name}Field {{")
                self.write(f"    type Error = UnknownMsgTypeError;")
                self.write_newline()
                self.write(f"    fn try_from(value: &Field) -> Result<Self, Self::Error> {{")
                self.write(f"        match value {{")

                self.write(f"            Field::{enum_variants.get(field.type)}(_, _) => Ok(Self {{ fd: value.clone() }}),")

                self.write(f"            _ => Err(UnknownMsgTypeError {{")
                self.write(f"                val: value.to_string(),")
                self.write(f"            }}),")
                self.write(f"        }}")
                self.write(f"    }}")
                self.write(f"}}")
                ##
                enum_type_return_type[enum_variants.get(field.type)] = return_types.get(field.type)



    def write_fieldset(self, fields: [Field]):
        self.write(f"impl FieldSet {{")
        # // todo generate get_clordid(&self) etc
        #
        # // todo generate get_int(&self, tag: u16) etc.
        #

        _seen_methods = []
        for method_deets in method_names.keys():  # k: STRING, v: str
            method_name = method_names.get(method_deets)
            dereference_type = 'v.to_string()' if method_name == 'str' else '*v'
            if method_name not in _seen_methods:
                self.write(f"    /// for use by custom fields")
                self.write(f"    pub fn get_{method_name}_field(&self, tag: u16) -> Result<{enum_variants.get(method_deets)}Field, FieldTypeMismatchError> {{")
                self.write(f"        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();")
                self.write(f"        match f {{")
                self.write(f"            Field::{enum_variants.get(method_deets)}(t, v) => Ok({enum_variants.get(method_deets)}Field {{")
                self.write(f"                tag: *t,")
                self.write(f"                fd: Field::{enum_variants.get(method_deets)}(*t, {dereference_type}),")
                self.write(f"            }}),")
                self.write(f"            _ => Err(FieldTypeMismatchError {{}}),")
                self.write(f"        }}")
                self.write(f"    }}")
            _seen_methods.append(method_name)

        self.write(f"}}")
