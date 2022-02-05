import sys
if __name__ == '__main__' and __package__ is None:
    from os import sys, path
    sys.path.append(path.dirname(path.dirname(path.abspath(__file__))))

import xml.etree.ElementTree as xml

# https://simplabs.com/blog/2020/12/31/xml-and-rust/

from os import path, listdir

# from spec.parser.parser import Parser
from config.config import Config
from spec.parser.parser import Parser
from spec.writer.writer import Writer

print(f"cargo:warning=Number of arguments: {len(sys.argv)}")
print(f"cargo:warning=Argument List: {str(sys.argv)}")

SPECS_DIR_NOT_FOUND=-1
OUTPUT_DIR_NOT_FOUND=-2

specs_path = sys.argv[1]
if not path.exists(specs_path):
    exit(SPECS_DIR_NOT_FOUND)

output_path = sys.argv[2]
if not path.exists(output_path):
    exit(OUTPUT_DIR_NOT_FOUND)


def looks_like_fix_specs(file: str):
    return file.startswith("FIX") and file.endswith('.xml')


def parse_and_write_fix_specs(config: Config, spec_file: str, spec_version: str) -> Parser:
    print(f"Spec version {spec_version} in {spec_file}")

    root = xml.parse(spec_file).getroot()
    parser = Parser(root, config, spec_version)
    parser.parse_fields()

    writer = Writer(output_path + '/' + spec_version + '.rs', config, import_fields=True)
    # writer.write_field_enum(parser.fields)
    writer.write_field_impls(spec_version, parser.fields)
    # writer.write_fieldset(parser.fields)
    writer.close()

    print("done")
    return parser


def all_fix_spec_files() -> [str]:
    return [file for file in listdir('.') if looks_like_fix_specs(file)]


def parse_and_write_fields(config: Config, parsers: [Parser]):
    all_fields = []
    for p in parsers:
        prs = parsers.get(p)
        for f in prs.fields:
            if f not in all_fields:
                all_fields.append(f)

    writer = Writer(output_path + '/fields.rs', config, import_fields=False)
    writer.write_field_enum(parsers.keys(), all_fields)
    writer.write_field_struct_impl(all_fields)
    writer.write_fieldset(all_fields)
    writer.write_custom_field_impls(all_fields)
    writer.close()


if __name__ == "__main__":
    config = Config()

    # Parse + generate Rust for all FIX specs
    parsers = {}
    for spec_file in all_fix_spec_files():
        spec_version = spec_file.split('.')[0].lower()  # e.g. fix42, fix50sp1
        parsers[spec_file] = parse_and_write_fix_specs(config, spec_file, spec_version)

    # generate Field type + impls
    parse_and_write_fields(config, parsers)











# root.findall('header/field')
# root.findall('messages/message')
# root.findall('trailer')
# root.findall('fields/field')
