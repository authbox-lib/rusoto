#!/usr/bin/env python
import json
import re
import sys
from html2text import html2text
from pprint import pprint

shapes = {}
operations = {}
metadata = {}

# map botocore primitives to rust primitives
primitive_types = {
    'string': 'String',
    'timestamp': 'String',
    'integer': 'i32',
    'long': 'i64',
    'float': 'f32',
    'double': 'f64',
    'blob': 'Vec<u8>',
    'boolean': 'bool'
}

# rust code to pull primitive types from XML
primitive_parsers = {
    'string': 'try!(characters(stack))',
    'timestamp': 'try!(characters(stack))',
    'integer': 'i32::from_str(try!(characters(stack)).as_ref()).unwrap()',
    'double': 'f32::from_str(try!(characters(stack)).as_ref()).unwrap()',
    'blob': 'try!(characters(stack)).into_bytes()',
    'boolean': 'bool::from_str(try!(characters(stack)).as_ref()).unwrap()'
}

# rust code to write primitive types to a string
primitive_writers = {
    'string': 'obj',
    'timestamp': 'obj',
    'integer': '&obj.to_string()',
    'double': '&obj.to_string()',
    'blob': 'str::from_utf8(&obj).unwrap()',
    'boolean': '&obj.to_string()',
}


# convert CamelCase to snake_case
# Also unapolagetically overloaded to prevent collisions with Rust keywords like "type".
# TODO: stop that, use function specifically for that.
def c_to_s(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    s2 = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
    if s2 == 'type':
        # prepend something informative
        s2 = 'foo_' + s2
    return s2


def documentation(shape, indent=""):
    if 'documentation' in shape:
        markdown = html2text(shape['documentation'])
        markdown = re.sub(r"\n+$", r"", markdown)
        markdown = re.sub(r"\n+", r"\n" + indent + "/// ", markdown)
        print indent + "/// " + markdown


# generate a rust declaration for a botocore shape
def rust_type(name, shape):
    shape_type = shape['type']

    documentation(shape)

    if shape_type == "structure":
        rust_struct(name, shape)
    else:
        if shape_type in primitive_types:
            rust_type = primitive_types[shape_type]
        elif shape_type == 'map':
            rust_type = "HashMap<" + shape['key']['shape'] + "," + shape['value']['shape'] + ">"
        elif shape_type == 'list':
            rust_type = "Vec<" + shape['member']['shape'] + ">"
        else:
            raise Exception("unrecognised type %s. supported types are %s" % (
                shape_type, primitive_types.keys() + ['map', 'list', 'struct']))
        # a String is already a String in rust
        if name != 'String':
            print "pub type " + name + " = " + rust_type + ";"


# generate a rust declaration for a botocore structure shape
def rust_struct(name, shape):
    print "#[derive(Debug, Default)]";
    if shape['members']:
        # print "MEMBERS:" + name
        print "pub struct " + name + " {"
        for (mname, member) in shape['members'].iteritems():
            if 'documentation' in member:
                documentation(member, "\t")
            rust_type = member['shape']

            if rust_type == 'Message':
                # print "foooo shape"
                rust_type = sys.argv[2] + rust_type

            if not is_required(shape, mname):
                rust_type = "Option<" + rust_type + ">"
            print "\tpub " + c_to_s(mname) + ": " + rust_type + ","
        print "}\n"
    else:
        print "pub struct " + name + ";\n"


# generate rust code to encode a botocore shape into a map of query parameters
# TODO: swap this over to putting things into the request headers
def param_writer(name, shape):
    print "/// Write " + name + " contents to a SignedRequest via headers"
    print 'struct ' + name + 'Writer;'
    print 'impl ' + name + 'Writer {'
    print '\tfn write_params(request: &mut request, name: &str, obj: &' + name + ') {'

    shape_type = shape['type']

    if shape_type in primitive_writers:
        print '\t\trequest.add_header(name, ' + primitive_writers[shape_type] + ');'
    # print '\t\tparams.put(name, ' + primitive_writers[shape_type] + ');'
    elif shape_type == 'list':
        list_writer(shape)
    elif shape_type == 'map':
        map_writer(shape)
    elif shape_type == 'structure':
        struct_writer(shape)

    print '\t}'
    print '}'


# guts of the param_writer for struct shapes
def struct_writer(shape):
    for (name, member) in shape['members'].iteritems():
        location_name = get_location_name(name, member)

        if not is_required(shape, name):
            print "\t\tif let Some(ref obj) = obj." + c_to_s(name) + " {"
            print '\t\t\t request.add_header("' + location_name + '", obj);'
            # print '\t\t\t' + member['shape'] + 'Writer::write_params(params, &(prefix.to_string() + "' + location_name + '"), obj);'
            print "\t\t}"
        else:
            # TODO: oh this needs to switch to headers, too, for some things:
            print '\t\t' + member[
                'shape'] + 'Writer::write_params(params, &(prefix.to_string() + "' + location_name + '"), &obj.' + c_to_s(
                name) + ');'


# guts of the param_writer for list shapes
def list_writer(shape):
    print "\t\tlet mut index = 1;"
    print "\t\tfor element in obj.iter() {"
    print "\t\t\tlet key = &format!(\"{}.{}\", name, index);"
    print "\t\t\t" + shape['member']['shape'] + "Writer::write_params(params, key, &element);"
    print "\t\t\tindex += 1;"
    print "\t\t}"


# guts of the param_writer for map shapes
def map_writer(shape):
    print "\t\tlet mut index = 1;"
    print "\t\tfor (key,value) in obj {"
    print "\t\t\tlet prefix = &format!(\"{}.{}\", name, index);"
    print "\t\t\t" + shape['key']['shape'] + "Writer::write_params(params, &format!(\"{}.{}\", prefix, \"" + shape_name(
        shape['key']) + "\"), &key);"
    print "\t\t\t" + shape['value'][
        'shape'] + "Writer::write_params(params, &format!(\"{}.{}\", prefix, \"" + shape_name(
        shape['value']) + "\"), &value);"
    print "\t\t\tindex += 1;"
    print "\t\t}"


# generate rust code to parse a botocore shape from XML
# TODO: take in a request and an XML stack, we may need both
def type_parser(name, shape):
    shape_type = shape['type']

    if 'location' in shape and shape['location'] == 'header':
        print '\t\t// should look in header'
    if 'location' in shape and shape['location'] == 'payload':
        print '\t\t// should look in payload xml'

    print '//typeparser'
    print "\n/// Parse " + name + " from response"
    print 'struct ' + name + 'Parser;'
    print 'impl ' + name + 'Parser {'
    print '\tfn parse_response<\'a, T: Peek + Next>(tag_name: &str, response: &Response stack: &mut T) -> Result<' + name + ', XmlParseError> {'

    if shape_type == 'map':
        map_parser(shape)
    elif shape_type == 'list':
        list_parser(shape)
    else:
        print '\t\ttry!(start_element(tag_name, stack));'
        if shape_type in primitive_parsers:
            print '\t\t // primitive_parser'
            print "\t\tlet obj = " + primitive_parsers[shape_type] + ";"
        elif shape_type == 'structure':
            struct_parser(name, shape)
        print '\t\ttry!(end_element(tag_name, stack));'

    print '\t\tOk(obj)'
    print '\t}'
    print '}'


def shape_name(shape):
    if 'locationName' in shape:
        return shape['locationName']
    else:
        return shape['shape']


# guts of the XML parser for map shapes
def map_parser(shape):
    print '\t\t // map_parser'
    print "\t\tlet mut obj = HashMap::new();"
    print "\t\twhile try!(peek_at_name(stack)) == tag_name {"
    print "\t\t\ttry!(start_element(tag_name, stack));"
    print "\t\t\tlet key = try!(" + shape['key']['shape'] + "Parser::parse_xml(\"" + shape_name(
        shape['key']) + "\", stack));"
    print "\t\t\tlet value = try!(" + shape['value']['shape'] + "Parser::parse_xml(\"" + shape_name(
        shape['value']) + "\", stack));"
    print "\t\t\tobj.insert(key, value);"
    print "\t\t\ttry!(end_element(tag_name, stack));"
    print "\t\t}"


# guts of the XML parser for list shapes
def list_parser(shape):
    print '\t\t // list_parser'
    print "\t\tlet mut obj = Vec::new();";
    print "\t\twhile try!(peek_at_name(stack)) == \"" + shape_name(shape['member']) + "\" {"
    print "\t\t\tobj.push(try!(" + shape['member']['shape'] + "Parser::parse_xml(\"" + shape_name(
        shape['member']) + "\", stack)));"
    print "\t\t}"


def is_required(shape, field_name):
    if not 'required' in shape:
        return True;
    else:
        return 'required' in shape and field_name in shape['required']


# guts of the XML parser for struct shapes
def struct_parser(name, shape):
    # if this is a request type it's input only and we don't need a parser
    print '\t\t // struct_parser'
    children = shape['members']
    print '\t\tlet mut obj = ' + name + '::default();'
    if children:
        print '\t\tloop {'
        print '\t\t\tlet current_name = try!(peek_at_name(stack));'

        for (cname, child) in children.iteritems():
            parse_struct_child(cname, child, is_required(shape, cname))
        print '\t\t\tbreak;\n\t\t}'


# get the name that should be used for a child element when encoding/decoding
def get_location_name(name, child):
    child_shape = shapes[child['shape']]

    # list elements aren't wrapped in a parent tag, so use their member name
    if child_shape['type'] == 'list':
        tag_name = shape_name(child_shape['member'])
    else:
        if 'locationName' in child:
            tag_name = child['locationName']
        else:
            tag_name = name

    return tag_name


# Need to figure out how to explain this well
def get_location(name, child):
    if 'location' in child:
        return child['location']  # should always be header or uri
    else:
        return 'payload'


# XML parser code to pull a single struct element from XML
def parse_struct_child(name, child, required):
    tag_name = get_location_name(name, child)

    print '\t\t\t// heylisten etc: location for ' + name + ' is ' + get_location(name, child) + '.'

    if get_location(name, child) == 'header':
        parse_stmt = 'try!(response.Headers.get(\"' + tag_name + '\"))'
    # parse_stmt = 'try!(' + child['shape'] + 'Parser::parse_header("' + tag_name + '", stack))'
    else:
        parse_stmt = 'try!(' + child['shape'] + 'Parser::parse_xml("' + tag_name + '", stack))'

    if not required:
        parse_stmt = "Some(" + parse_stmt + ")"

    print '\t\t\tif current_name == "' + tag_name + '" {'
    print '\t\t\t\tobj.' + c_to_s(name) + ' = ' + parse_stmt + ';'
    print '\t\t\t\tcontinue;'
    print '\t\t\t}'


# determine the rust output type for a botocore operation
def get_output_type(operation):
    if 'output' in operation:
        return operation['output']['shape']
    else:
        return "()"


# generate rust code to sign and execute an HTTP request for a botocore operation
def request_method(operation):
    http = operation['http']

    output_type = get_output_type(operation)
    documentation(operation, "\t")

    # This feels so hacky to get around scoping of these in the else block:
    input_name = ''
    input_type = ''

    if not ('input' in operation):
        print "\tpub fn " + c_to_s(operation['name']) + "(&self"") -> Result<" + output_type + ", AWSError> {"
    else:
        input_name = operation['input']['shape']
        input_type = shapes[input_name]
        print "\tpub fn " + c_to_s(
            operation['name']) + "(&self, input: &" + input_name + ") -> Result<" + output_type + ", AWSError> {"

    # Most of these go to just the bucket
    print '\t\tlet mut uri = String::from("/");'
    print '\t\turi = uri +  &input.key.to_string();'
    print '\t\tlet mut request = SignedRequest::new("' + http['method'] + '", "' + metadata[
        'endpointPrefix'] + '", &self.region, &uri);\n'

    if ('input' in operation):
        print '\t\t' + input_name + 'Writer::write_params(&mut request, \"\", &input);'

    print '\t\tlet hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";'
    print '\t\trequest.set_hostname(Some(hostname));\n'
    print '\t\tlet result = request.sign_and_execute(&self.creds);'
    print '\t\tlet status = result.status.to_u16();\n'

    print '\t\tmatch status {'
    print '\t\t\t200 => { '
    if output_type == '()':
        print '\t\t\t\tOk(())'
    else:
        print '\t\t\t\tlet mut reader = EventReader::new(result);'
        print '\t\t\t\tlet mut stack = XmlResponseFromAws::new(reader.events().peekable());'
        print '\t\t\t\tstack.next(); // xml start tag'
        print '\t\t\t\tstack.next();\n'
        print '\t\t\t\tOk(try!(' + output_type + 'Parser::parse_response("' + output_type + '", &result, &mut stack)))'
    print '\t\t\t}'

    print '\t\t\t_ => { '
    print '\t\t\t\tlet mut body = String::new();'
    print '\t\t\t\tresult.read_to_string(&mut body).unwrap();'
    print '\t\t\t\tprintln!("Error response body: {}", body);'

    print '\t\t\t\tErr(AWSError::new("S3 error in ' + c_to_s(operation['name']) + '"))'
    print '\t\t\t }'

    print '\t\t}'
    print "\t}"


def generate_client():
    client_name = sys.argv[2]

    print "pub struct " + client_name + "<'a> {"
    print "\tcreds: &'a AWSCredentials,"
    print "\tregion: &'a str"
    print "}\n"

    print "impl<'a> " + client_name + "<'a> { "
    print "\tpub fn new(creds: &'a AWSCredentials, region: &'a str) -> " + client_name + "<'a> {"
    print "\t\t" + client_name + " { creds: creds, region: region }"
    print "\t}"

    for (name, operation) in operations.iteritems():
        request_method(operation)

    print "}"


def pretty(d, indent=0):
    for key, value in d.iteritems():
        print '\t' * indent + str(key)
        if isinstance(value, dict):
            pretty(value, indent + 1)
        else:
            print '\t' * (indent + 1) + str(value)


def main():
    with open(sys.argv[1]) as data_file:
        service = json.load(data_file)

        print "use std::collections::HashMap;"
        print "use std::str;"
        print "use std::error::Error;"
        global shapes
        global metadata
        global operations
        shapes = service['shapes']
        metadata = service['metadata']
        operations = service['operations']

        for (name, shape) in shapes.iteritems():
            # don't pass in reserved Rust keywords.
            if name == 'Message' or name == 'Error':
                # print "REASSIGNING"
                name = sys.argv[2] + name

            # no need to make a parser for an outgoing request
            if not name.endswith('Request'):
                type_parser(name, shape)

            rust_type(name, shape)

            # no need to make a parameter writer for a type coming back from AWS
            if not name.endswith('Output') and not name.endswith('Result'):
                # print '//superlisten: ' + name + ' doesn\'t end with Output or Result so gets a paramwriter\n'
                param_writer(name, shape)

        generate_client()


if __name__ == "__main__": main()
