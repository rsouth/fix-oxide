if __name__ == '__main__' and __package__ is None:
    from os import sys, path
    sys.path.append(path.dirname(path.dirname(path.abspath(__file__))))

import xml.etree.ElementTree as xml

# https://simplabs.com/blog/2020/12/31/xml-and-rust/
# from spec.parser.parser import Parser
from spec.parser.parser import Parser
from spec.writer.writer import Writer

def main():
    root = xml.parse('FIX42.xml').getroot()

    parser = Parser(root)
    parser.parse_fields()

    writer = Writer()
    writer.write_field_enum(parser.fields)
    writer.write_field_impls(parser.fields)
    writer.write_fieldset(parser.fields)

    writer.close()

    print("done")

# root.findall('header/field')
# root.findall('messages/message')
# root.findall('trailer')
# root.findall('fields/field')

if __name__ == "__main__":
    main()