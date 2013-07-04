
pub mod common;
pub mod endian;
pub mod layout;
//pub mod arena;
pub mod message;
pub mod list;
pub mod serialize;

pub mod schema_capnp;

fn typeToString (t : &schema_capnp::Type::Reader) -> ~str {
    use schema_capnp::Type::Body;
    match t.getBody() {
        Body::voidType => { return ~"Void" }
        Body::boolType => { return ~"Bool" }
        Body::int8Type => { return ~"Int8" }
        Body::int16Type => { return ~"Int16" }
        Body::int32Type => { return ~"Int32" }
        Body::int64Type => { return ~"Int64" }
        Body::uint8Type => { return ~"UInt8" }
        Body::uint16Type => { return ~"UInt16" }
        Body::uint32Type => { return ~"UInt32" }
        Body::uint64Type => { return ~"UInt64" }
        Body::float32Type => { return ~"Float32" }
        Body::float64Type => { return ~"Float64" }
        Body::textType => { return ~"Text" }
        Body::dataType => { return ~"Data" }
        Body::listType(t1) => {
            let s1 = typeToString(&t1);
            return fmt!("List(%s)", s1)
        }
        Body::enumType(_) => { return ~"Enum" }
        Body::structType(_) => { return ~"Struct" }
        Body::interfaceType(_) => { return ~"Interface" }
        Body::objectType => { return ~"Object" }
    }
}

fn main() {

    use serialize::*;

    let inp = std::io::stdin();

    do InputStreamMessageReader::new(inp, message::defaultReaderOptions) | messageReader | {
        let structReader = messageReader.getRoot();

        let codeGeneratorRequest =
            schema_capnp::CodeGeneratorRequest::Reader::new(structReader);

        let requestedFilesReader = codeGeneratorRequest.getRequestedFiles();

        for std::uint::range(0, requestedFilesReader.size()) |ii| {
            std::io::println(fmt!("requested file: %x",
                                  requestedFilesReader.get(ii)));
        }

        let nodeListReader = codeGeneratorRequest.getNodes();

        for std::uint::range(0, nodeListReader.size()) |ii| {
            use schema_capnp::*;

            let nodeReader = nodeListReader.get(ii);
            let id = nodeReader.getId();
            let displayName = nodeReader.getDisplayName();
            std::io::println(fmt!("node with name: %s and id: %x", displayName, id as uint));
            std::io::println(fmt!( "  scopeId: %x", nodeReader.getScopeId() as uint));
            match nodeReader.getBody() {

                Node::Body::fileNode(_) => { }

                Node::Body::structNode(structNode) => {
                    std::io::println(fmt!("  struct node. data size: %?, pointer size: %?",
                                         structNode.getDataSectionWordSize(),
                                         structNode.getPointerSectionSize()));

                    let members = structNode.getMembers();
                    for std::uint::range(0, members.size()) |ii| {
                        let member = members.get(ii);
                        let name = member.getName();
                        match member.getBody() {
                            StructNode::Member::Body::fieldMember(field) => {
                                let offset = field.getOffset() as uint;
                                std::io::println(
                                    fmt!("    field %s : %s  at offset %u",
                                         name, typeToString(&field.getType()), offset));

                            }
                            StructNode::Member::Body::unionMember(union) => {
                                let doffset = union.getDiscriminantOffset() as uint;
                                std::io::println(
                                    fmt!("    union field %s with discriminant offset %u",
                                         name, doffset));
                            }
                        }
                    }

                }

                Node::Body::enumNode(_) => { }

                Node::Body::interfaceNode(_) => { }

                Node::Body::constNode(_) => { }

                Node::Body::annotationNode( annotationNode ) => {
                    std::io::println("  annotation node:");
                    if (annotationNode.getTargetsFile()) {
                        std::io::println("  targets file");
                    }
                    if (annotationNode.getTargetsConst()) {
                        std::io::println("  targets const");
                    }
                    // ...
                    if (annotationNode.getTargetsAnnotation()) {
                        std::io::println("  targets annotation");
                    }
                }
            }

            let nestedNodes = nodeReader.getNestedNodes();
            for std::uint::range(0, nestedNodes.size()) |ii| {
                let nestedNode = nestedNodes.get(ii);
                let id = nestedNode.getId();
                let name = nestedNode.getName();
                std::io::println(fmt!("  nested node with name: %s and id: %x",
                                      name, id as uint));
            }
        }


        0;
    }
}