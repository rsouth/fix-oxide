import typing

from ..config.config import Config
from ..datatypes.field_utils import unique_field_enum_variants
from ..datatypes.datatypes import Field


class Writer:

    def __init__(self, output_file: str, config: Config, import_fields: bool):
        self.fd = self.init(output_file)
        self.config = config
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

    def write_field_enum(self, spec_versions: [str], fields: [Field]):
        self.write(f"#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]")
        self.write(f"pub enum Field {{")
        seen_enum_variants = []
        for field in fields:
            enum_variant = self.config.enum_variant_for(field)
            if enum_variant not in seen_enum_variants:
                struct_type  = self.config.struct_type_for(field)
                self.write(f"    {enum_variant}(u16, {struct_type}),")
                seen_enum_variants.append(enum_variant)
        self.write(f"}}")
        self.write_newline()

        self.write(f"impl Field {{")
        self.write(f"   pub fn crack(bs: BeginString, to_parse: &str) -> Result<Field, ()> {{")
        self.write(f"       match bs {{")

        for spec_file in spec_versions:
            spec_version = spec_file.split('.')[0]
            self.write(f"           BeginString::{spec_version.title()} => Ok(crate::model::generated::{spec_version.lower()}::{spec_version.upper()}CrackerUtils::crack(")
            self.write(f"               to_parse,")
            self.write(f"           ).unwrap()),")

        self.write(f"       }}")
        self.write(f"   }}")
        self.write(f"}}")

    def write_field_struct_impl(self, fields: [Field]):
        _seen_types = []
        _seen_enum_variants = []
        self.write(f"impl Field {{")
        self.write_newline()

        for field in fields:
            return_type = self.config.return_type_for(field)
            if self.config.should_process_type(field.type) and return_type not in _seen_types:
                method_name = self.config.method_name_for(field)
                enum_variant = self.config.enum_variant_for(field)
                # todo "if return_type.startswith ..." should be in config, and there's another place where similar is done.
                self.write(f"    ///")
                self.write(f"    /// # Errors")
                self.write(f"    ///")
                self.write(f"    pub{'' if return_type.startswith('&') else ' const'} fn as_{method_name}_safe(&self) -> Result<{return_type}, FieldTypeMismatchError> {{")
                self.write(f"        match self {{")
                self.write(f"            Field::{enum_variant}(_, v) => Ok({'' if return_type.startswith('&') else '*'}v),")
                self.write(f"            _ => Err(FieldTypeMismatchError {{}}),")
                self.write(f"        }}")
                self.write(f"    }}")
                self.write_newline()
                self.write(f"    ///")
                self.write(f"    /// # Errors")
                self.write(f"    ///")
                self.write(f"    /// # Panics")
                self.write(f"    ///")
                self.write(f"    pub fn as_{method_name}(&self) -> {return_type} {{")
                self.write(f"        match self {{")
                self.write(f"            Field::{enum_variant}(_, v) => {'' if return_type.startswith('&') else '*'}v,")
                self.write(f"            _ => panic!(""),")
                self.write(f"        }}")
                self.write(f"    }}")
                self.write_newline()
                _seen_enum_variants.append(enum_variant)
                _seen_types.append(return_type)

        # Field::tag(&self)
        self.write(f"    #[must_use]")
        self.write(f"    pub const fn tag(&self) -> u16 {{")
        self.write(f"        match self {{")
        count = 1
        _seen_enum_variants2 = []
        for variant in _seen_enum_variants:
            if variant not in _seen_enum_variants2:
                self.write(f"            {'| ' if count > 1 else ''}Field::{variant}(t, _) ")
                _seen_enum_variants2.append(variant)
            count += 1
        self.write(f"            => *t,")
        self.write(f"    }}")
        self.write(f"}}")

        # Field::to_delimited_string(&self)
        _seen_var_return_types = []
        _seen_enum_variants.clear()
        self.write_newline()
        self.write(f"    #[must_use]")
        self.write(f"    pub fn to_delimited_string(&self, separator: char) -> String {{")
        self.write(f"        match self {{")

        for field in fields:
            return_type = self.config.return_type_for(field)
            enum_variant = self.config.enum_variant_for(field)
            if self.config.should_process_type(field.type) and self.config.enum_variant_for(field) not in _seen_enum_variants:
                self.write(f"            // {return_type}")
                self.write(f"            Field::{enum_variant}(t, v) => format!(\"{{}}={{}}{{}}\", t, v, separator),")
                _seen_enum_variants.append(enum_variant)
        self.write(f"        }}")
        self.write(f"    }}")
        self.write(f"}}")
        ## ---- End Field Impl


    def write_custom_field_impls(self, fields: [Field]):
        #
        # impl generic (custom) field types (string, int etc)
        #
        seen_types = []
        # field_types = [f.type for f in fields]
        for field in fields:
            return_type = self.config.return_type_for(field)
            is_ref_return_type = field.type in ['STRING'] or return_type.startswith('&')

            enum_variant = self.config.enum_variant_for(field)
            if enum_variant not in seen_types:
                self.write(f"pub struct {enum_variant}Field {{")
                self.write(f"    tag: u16,")
                self.write(f"    fd: Field,")
                self.write(f"}}")
                self.write_newline()
                self.write(f"impl {enum_variant}Field {{")
                self.write(f"    const fn tag(&self) -> u16 {{")
                self.write(f"        self.tag")
                self.write(f"    }}")
                self.write_newline()
                self.write(f"    fn value(&self) -> {return_type} {{")
                self.write(f"        match &self.fd {{")
                self.write(f"            Field::{enum_variant}(_, v) => {'' if is_ref_return_type else '*'}v,")
                self.write(f"            _ => panic!(\"\"),")
                self.write(f"        }}")
                self.write(f"    }}")
                self.write(f"}}")
                self.write_newline()
                self.write(f"impl std::fmt::Display for {enum_variant}Field {{")
                self.write(f"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{")
                self.write(f"        write!(f, \"{{}}={{}}|\", self.tag(), self.value())")
                self.write(f"    }}")
                self.write(f"}}")

            seen_types.append(enum_variant)


    def write_field_impls(self, spec_version: str, fields: [Field]):
        ## ---- TryFrom<String> for Field
        _seen_types = []
        # enum_var -> [tag_nums...]
        _tag_num_enum_variant = {}

        # create type -> [field.tag] mapping
        for field in fields:
            if self.config.should_process_type(field.type):
                enum_type = self.config.enum_variant_for(field)
                if _tag_num_enum_variant.get(enum_type) is None:
                    _tag_num_enum_variant[enum_type] = []
                _tag_num_enum_variant.get(enum_type).append(str(field.tag))

        self.write(f"pub struct {spec_version.upper()}CrackerUtils;")
        self.write(f"// parse string (35=D) into Field{{35, \"D\"}}")
        self.write(f"impl {spec_version.upper()}CrackerUtils {{")
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
            parse_fn = self.config.parse_fn_for(enum_key)
            self.write(f"                    {' | '.join(_tag_num_enum_variant.get(enum_key))} => Ok({parse_fn}),")
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
        for field in fields:
            if self.config.should_process_type(field.type):
                return_type = self.config.return_type_for(field)
                struct_type = self.config.struct_type_for(field)
                enum_variant = self.config.enum_variant_for(field)
                self.write(f"#[derive(Debug, Clone, Eq, PartialEq)]")
                self.write(f"pub struct {field.name}Field {{")
                self.write(f"    pub fd: Field,")
                self.write(f"}}")
                self.write_newline()

                self.write(f"impl {field.name}Field {{")
                self.write(f"   pub fn new<T: Into<{struct_type}>>(value: T) -> Self {{")
                self.write(f"      {field.name}Field {{")
                self.write(f"          fd: Field::{enum_variant}({field.name}Field::tag(), value.into()),")
                self.write(f"      }}")
                self.write(f"   }}")

                self.write(f"    #[must_use]")
                self.write(f"    pub const fn tag() -> u16 {{")
                self.write(f"        {field.tag}")
                self.write(f"    }}")
                self.write_newline()

                self.write(f"    // tag_type: {field.type}")
                self.write(f"    pub fn value(&self) -> {return_type} {{")
                self.write(f"        match &self.fd {{")
                self.write(f"            Field::{enum_variant}(_, v) => {'' if return_type.startswith('&') else '*'}v,")
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

                self.write(f"impl Into<Field> for {field.name}Field {{")
                self.write(f"   fn into(self) -> Field {{")
                self.write(f"       self.fd.into()")
                self.write(f"   }}")
                self.write(f"}}")


    def write_fieldset(self, fields: [Field]):
        self.write(f"impl FieldSet {{")
        # // todo generate get_clordid(&self) etc
        #
        # // todo generate get_int(&self, tag: u16) etc.
        #

        # _seen_methods = []
        _seen_fix_types = []
        for method_deets in fields:
            if method_deets.type not in _seen_fix_types:
                method_name = self.config.method_name_for(method_deets)
                return_type = self.config.return_type_for(method_deets)
                enum_variant = self.config.enum_variant_for(method_deets)
                # dereference_type = 'v.to_string()' if return_type.startswith('&') else '*v'
                dereference_type = 'v.to_string()' if 'str' in return_type else '*v'  # if return_type.startswith('&') else 'v'
                if method_name not in _seen_fix_types:
                    self.write(f"    /// for use by custom fields")
                    self.write(f"    pub fn get_{method_name}_field(&self, tag: u16) -> Result<{enum_variant}Field, FieldTypeMismatchError> {{")
                    self.write(f"        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();")
                    self.write(f"        match f {{")
                    self.write(f"            Field::{enum_variant}(t, v) => Ok({enum_variant}Field {{")
                    self.write(f"                tag: *t,")
                    self.write(f"                fd: Field::{enum_variant}(*t, {dereference_type}),")
                    self.write(f"            }}),")
                    self.write(f"            _ => Err(FieldTypeMismatchError {{}}),")
                    self.write(f"        }}")
                    self.write(f"    }}")
                _seen_fix_types.append(method_name)

        self.write(f"}}")
