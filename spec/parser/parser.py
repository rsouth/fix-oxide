from xml.etree import ElementTree as xml

from spec.datatypes.datatypes import Field
from spec.config.config import Config


class Parser:
    def __init__(self, doc_root: xml.Element, config: Config, spec_version: str):
        self.root = doc_root
        self.config = config
        self.spec_version = spec_version
        self.fields = []

    def parse_fields(self):
        for field in self.root.findall('fields/field'):
            field_num  = field.get('number')
            field_name = field.get('name')
            field_type = field.get('type')
            if self.config.is_supported(field_type) and not self.config.is_excluded(field_type):
                self.fields.append(Field(self.spec_version, int(field_num), field_name, field_type))