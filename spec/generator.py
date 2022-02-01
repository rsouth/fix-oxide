import os.path
import sys
if __name__ == '__main__' and __package__ is None:
    from os import sys, path
    sys.path.append(path.dirname(path.dirname(path.abspath(__file__))))

import xml.etree.ElementTree as xml

# https://simplabs.com/blog/2020/12/31/xml-and-rust/
# from spec.parser.parser import Parser
from spec.parser.parser import Parser
from spec.writer.writer import Writer

print(f"cargo:warning=Number of arguments: {len(sys.argv)}")
print(f"cargo:warning=Argument List: {str(sys.argv)}")

SPECS_DIR_NOT_FOUND=-1
OUTPUT_DIR_NOT_FOUND=-2

specs_path = sys.argv[1]
if not os.path.exists(specs_path):
    exit(SPECS_DIR_NOT_FOUND)

output_path = sys.argv[2]
if not os.path.exists(output_path):
    exit(OUTPUT_DIR_NOT_FOUND)


def looks_like_fix_specs(file: str):
    return file.startswith("FIX") and file.endswith('.xml')


def main(spec_file: str) -> Parser:
    spec_version = spec_file.split('.')[0]
    print(f"Spec version {spec_version} in {spec_file}")
    # exit(123)

    root = xml.parse(spec_file).getroot()

    parser = Parser(root)
    parser.parse_fields()

    writer = Writer(output_path + '/' + spec_version + '.rs', True)
    # writer.write_field_enum(parser.fields)
    writer.write_field_impls(spec_version, parser.fields)
    # writer.write_fieldset(parser.fields)
    writer.close()

    print("done")
    return parser


if __name__ == "__main__":
    import os
    xml_files = [file for file in os.listdir('.') if looks_like_fix_specs(file)]
    parsers = {}
    for spec in xml_files:
        parsers[spec] = main(spec)

    writer = Writer(output_path + '/fields.rs', False)

    all_fields = []
    for p in parsers:
        prs = parsers.get(p)
        for f in prs.fields:
            if f not in all_fields:
                all_fields.append(f)

    writer.write_field_enum(xml_files, all_fields)
    writer.write_field_struct_impl(all_fields)
    writer.write_fieldset(all_fields)
    writer.write_custom_field_impls(all_fields)
    writer.close()













# root.findall('header/field')
# root.findall('messages/message')
# root.findall('trailer')
# root.findall('fields/field')
