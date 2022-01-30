import xml.etree.ElementTree as xml

# https://simplabs.com/blog/2020/12/31/xml-and-rust/
from spec.parser import Parser
from spec.writer import Writer

root = xml.parse('FIX42.xml').getroot()

parser = Parser(root)
parser.parse_fields()

writer = Writer()
writer.write_field_enum(parser.fields)
writer.write_field_impls(parser.fields)
writer.write_fieldset(parser.fields)

writer.close()



# root.findall('header/field')
# root.findall('messages/message')
# root.findall('trailer')
# root.findall('fields/field')
