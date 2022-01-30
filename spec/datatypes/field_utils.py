from ..config.config import enum_variants
from ..datatypes.datatypes import Field


class FieldUtils:
    @staticmethod
    def unique_field_enum_variants(fields: [Field]) -> [str]:
        _seen_field_types = []
        _found_field_types = []
        for field in fields:
            field_type = enum_variants.get(field.type)
            if field_type not in _seen_field_types:
                _found_field_types.append(field.type)
            _seen_field_types.append(field_type)
        return _found_field_types