from ..config.config import Config
from ..datatypes.datatypes import Field


def unique_field_enum_variants(config: Config, fields: [Field]) -> [str]:
    _seen_field_types = []
    _found_field_types = []
    for field in fields:
        field_type = config.enum_variant_for(field.type)
        if field_type not in _seen_field_types:
            _found_field_types.append(field.type)
        _seen_field_types.append(field_type)
    return _found_field_types