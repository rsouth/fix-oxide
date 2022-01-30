from xml.etree import ElementTree as xml

from spec.config import enum_variants, exclude_types
from spec.datatypes import Field


class Parser:
    def __init__(self, doc_root: xml.Element):
        self.root = doc_root
        self.fields = []

    def parse_fields(self):
        for field in self.root.findall('fields/field'):
            field_num  = field.get('number')
            field_name = field.get('name')
            field_type = field.get('type')
            if field_type not in exclude_types and field_type in enum_variants:
                self.fields.append(Field(int(field_num), field_name, field_type))