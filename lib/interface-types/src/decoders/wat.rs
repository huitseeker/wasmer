//! Parse the WIT textual representation into an AST.

#![allow(unused)]

use crate::{ast::*, interpreter::Instruction};
use wast::{
    parser::{Cursor, Parse, Parser, Peek, Result},
    Id, LParen,
};

mod kw {
    pub use wast::{
        custom_keyword,
        kw::{anyref, export, f32, f64, func, i32, i64, import, param, result},
    };

    // New keywords.
    custom_keyword!(adapt);
    custom_keyword!(forward);

    // New types.
    custom_keyword!(int);
    custom_keyword!(float);
    custom_keyword!(any);
    custom_keyword!(string);
    custom_keyword!(seq);

    // Instructions.
    custom_keyword!(argument_get = "arg.get");
    custom_keyword!(call);
    custom_keyword!(call_export = "call-export");
    custom_keyword!(read_utf8 = "read-utf8");
    custom_keyword!(write_utf8 = "write-utf8");
    custom_keyword!(as_wasm = "as-wasm");
    custom_keyword!(as_interface = "as-interface");
    custom_keyword!(table_ref_add = "table-ref-add");
    custom_keyword!(table_ref_get = "table-ref-get");
    custom_keyword!(call_method = "call-method");
    custom_keyword!(make_record = "make-record");
    custom_keyword!(get_field = "get-field");
    custom_keyword!(r#const = "const");
    custom_keyword!(fold_seq = "fold-seq");
    custom_keyword!(add);
    custom_keyword!(mem_to_seq = "mem-to-seq");
    custom_keyword!(load);
    custom_keyword!(seq_new = "seq.new");
    custom_keyword!(list_push = "list.push");
    custom_keyword!(repeat_until = "repeat-until");
}

/// Issue: Uppercased keyword aren't supported for the moment.
impl Parse<'_> for InterfaceType {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut lookahead = parser.lookahead1();

        if lookahead.peek::<kw::int>() {
            parser.parse::<kw::int>()?;

            Ok(InterfaceType::Int)
        } else if lookahead.peek::<kw::float>() {
            parser.parse::<kw::float>()?;

            Ok(InterfaceType::Float)
        } else if lookahead.peek::<kw::any>() {
            parser.parse::<kw::any>()?;

            Ok(InterfaceType::Any)
        } else if lookahead.peek::<kw::string>() {
            parser.parse::<kw::string>()?;

            Ok(InterfaceType::String)
        } else if lookahead.peek::<kw::seq>() {
            parser.parse::<kw::seq>()?;

            Ok(InterfaceType::Seq)
        } else if lookahead.peek::<kw::i32>() {
            parser.parse::<kw::i32>()?;

            Ok(InterfaceType::I32)
        } else if lookahead.peek::<kw::i64>() {
            parser.parse::<kw::i64>()?;

            Ok(InterfaceType::I64)
        } else if lookahead.peek::<kw::f32>() {
            parser.parse::<kw::f32>()?;

            Ok(InterfaceType::F32)
        } else if lookahead.peek::<kw::f64>() {
            parser.parse::<kw::f64>()?;

            Ok(InterfaceType::F64)
        } else if lookahead.peek::<kw::anyref>() {
            parser.parse::<kw::anyref>()?;

            Ok(InterfaceType::AnyRef)
        } else {
            Err(lookahead.error())
        }
    }
}

impl<'a> Parse<'a> for Instruction<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut lookahead = parser.lookahead1();

        if lookahead.peek::<kw::argument_get>() {
            parser.parse::<kw::argument_get>()?;

            Ok(Instruction::ArgumentGet {
                index: parser.parse()?,
            })
        } else if lookahead.peek::<kw::call>() {
            parser.parse::<kw::call>()?;

            Ok(Instruction::Call {
                function_index: parser.parse::<u64>()? as usize,
            })
        } else if lookahead.peek::<kw::call_export>() {
            parser.parse::<kw::call_export>()?;

            Ok(Instruction::CallExport {
                export_name: parser.parse()?,
            })
        } else if lookahead.peek::<kw::read_utf8>() {
            parser.parse::<kw::read_utf8>()?;

            Ok(Instruction::ReadUtf8)
        } else if lookahead.peek::<kw::write_utf8>() {
            parser.parse::<kw::write_utf8>()?;

            Ok(Instruction::WriteUtf8 {
                allocator_name: parser.parse()?,
            })
        } else if lookahead.peek::<kw::as_wasm>() {
            parser.parse::<kw::as_wasm>()?;

            Ok(Instruction::AsWasm(parser.parse()?))
        } else if lookahead.peek::<kw::as_interface>() {
            parser.parse::<kw::as_interface>()?;

            Ok(Instruction::AsInterface(parser.parse()?))
        } else if lookahead.peek::<kw::table_ref_add>() {
            parser.parse::<kw::table_ref_add>()?;

            Ok(Instruction::TableRefAdd)
        } else if lookahead.peek::<kw::table_ref_get>() {
            parser.parse::<kw::table_ref_get>()?;

            Ok(Instruction::TableRefGet)
        } else if lookahead.peek::<kw::call_method>() {
            parser.parse::<kw::call_method>()?;

            Ok(Instruction::CallMethod(parser.parse()?))
        } else if lookahead.peek::<kw::make_record>() {
            parser.parse::<kw::make_record>()?;

            Ok(Instruction::MakeRecord(parser.parse()?))
        } else if lookahead.peek::<kw::get_field>() {
            parser.parse::<kw::get_field>()?;

            Ok(Instruction::GetField(parser.parse()?, parser.parse()?))
        } else if lookahead.peek::<kw::r#const>() {
            parser.parse::<kw::r#const>()?;

            Ok(Instruction::Const(parser.parse()?, parser.parse()?))
        } else if lookahead.peek::<kw::fold_seq>() {
            parser.parse::<kw::fold_seq>()?;

            Ok(Instruction::FoldSeq(parser.parse()?))
        } else if lookahead.peek::<kw::add>() {
            parser.parse::<kw::add>()?;

            Ok(Instruction::Add(parser.parse()?))
        } else if lookahead.peek::<kw::mem_to_seq>() {
            parser.parse::<kw::mem_to_seq>()?;

            Ok(Instruction::MemToSeq(parser.parse()?, parser.parse()?))
        } else if lookahead.peek::<kw::load>() {
            parser.parse::<kw::load>()?;

            Ok(Instruction::Load(parser.parse()?, parser.parse()?))
        } else if lookahead.peek::<kw::seq_new>() {
            parser.parse::<kw::seq_new>()?;

            Ok(Instruction::SeqNew(parser.parse()?))
        } else if lookahead.peek::<kw::list_push>() {
            parser.parse::<kw::list_push>()?;

            Ok(Instruction::ListPush)
        } else if lookahead.peek::<kw::repeat_until>() {
            parser.parse::<kw::repeat_until>()?;

            Ok(Instruction::RepeatUntil(parser.parse()?, parser.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

struct AtInterface;

impl Peek for AtInterface {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.reserved().map(|(string, _)| string) == Some("@interface")
    }

    fn display() -> &'static str {
        "`@interface`"
    }
}

impl Parse<'_> for AtInterface {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.step(|cursor| {
            if let Some(("@interface", rest)) = cursor.reserved() {
                return Ok((AtInterface, rest));
            }

            Err(cursor.error("expected `@interface`"))
        })
    }
}

#[derive(PartialEq, Debug)]
enum FunctionType {
    Input(Vec<InterfaceType>),
    Output(Vec<InterfaceType>),
}

impl Parse<'_> for FunctionType {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parens(|parser| {
            let mut lookahead = parser.lookahead1();

            if lookahead.peek::<kw::param>() {
                parser.parse::<kw::param>()?;

                let mut inputs = vec![];

                while !parser.is_empty() {
                    inputs.push(parser.parse()?);
                }

                Ok(FunctionType::Input(inputs))
            } else if lookahead.peek::<kw::result>() {
                parser.parse::<kw::result>()?;

                let mut outputs = vec![];

                while !parser.is_empty() {
                    outputs.push(parser.parse()?);
                }

                Ok(FunctionType::Output(outputs))
            } else {
                Err(lookahead.error())
            }
        })
    }
}

#[derive(PartialEq, Debug)]
enum Interface<'a> {
    Export(Export<'a>),
    Type(Type<'a>),
    Import(Import<'a>),
    Adapter(Adapter<'a>),
    Forward(Forward<'a>),
}

impl<'a> Parse<'a> for Interface<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parens(|parser| {
            let mut lookahead = parser.lookahead1();

            if lookahead.peek::<AtInterface>() {
                parser.parse::<AtInterface>()?;

                let mut lookahead = parser.lookahead1();

                if lookahead.peek::<kw::export>() {
                    Ok(Interface::Export(parser.parse()?))
                } else if lookahead.peek::<kw::func>() {
                    Ok(Interface::Import(parser.parse()?))
                } else if lookahead.peek::<kw::adapt>() {
                    Ok(Interface::Adapter(parser.parse()?))
                } else if lookahead.peek::<kw::forward>() {
                    Ok(Interface::Forward(parser.parse()?))
                } else {
                    Err(lookahead.error())
                }
            } else {
                Err(lookahead.error())
            }
        })
    }
}

impl<'a> Parse<'a> for Export<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::export>()?;
        let name = parser.parse()?;

        let mut input_types = vec![];
        let mut output_types = vec![];

        while !parser.is_empty() {
            let function_type = parser.parse::<FunctionType>()?;

            match function_type {
                FunctionType::Input(mut inputs) => input_types.append(&mut inputs),
                FunctionType::Output(mut outputs) => output_types.append(&mut outputs),
            }
        }

        Ok(Export {
            name,
            input_types,
            output_types,
        })
    }
}

impl<'a> Parse<'a> for Import<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::func>()?;
        parser.parse::<Id>()?;

        let (namespace, name) = parser.parens(|parser| {
            parser.parse::<kw::import>()?;

            Ok((parser.parse()?, parser.parse()?))
        })?;
        let mut input_types = vec![];
        let mut output_types = vec![];

        while !parser.is_empty() {
            let function_type = parser.parse::<FunctionType>()?;

            match function_type {
                FunctionType::Input(mut inputs) => input_types.append(&mut inputs),
                FunctionType::Output(mut outputs) => output_types.append(&mut outputs),
            }
        }

        Ok(Import {
            namespace,
            name,
            input_types,
            output_types,
        })
    }
}

impl<'a> Parse<'a> for Adapter<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::adapt>()?;

        let (kind, namespace, name) = parser.parens(|parser| {
            let mut lookahead = parser.lookahead1();

            if lookahead.peek::<kw::import>() {
                parser.parse::<kw::import>()?;

                Ok((AdapterKind::Import, parser.parse()?, parser.parse()?))
            } else if lookahead.peek::<kw::export>() {
                parser.parse::<kw::export>()?;

                Ok((AdapterKind::Export, "", parser.parse()?))
            } else {
                Err(lookahead.error())
            }
        })?;
        let mut input_types = vec![];
        let mut output_types = vec![];
        let mut instructions = vec![];

        while !parser.is_empty() {
            if parser.peek::<LParen>() {
                let function_type = parser.parse::<FunctionType>()?;

                match function_type {
                    FunctionType::Input(mut inputs) => input_types.append(&mut inputs),
                    FunctionType::Output(mut outputs) => output_types.append(&mut outputs),
                }
            } else {
                instructions.push(parser.parse()?);
            }
        }

        Ok(match kind {
            AdapterKind::Import => Adapter::Import {
                namespace,
                name,
                input_types,
                output_types,
                instructions,
            },

            AdapterKind::Export => Adapter::Export {
                name,
                input_types,
                output_types,
                instructions,
            },

            _ => unimplemented!("Adapter of kind “helper” is not implemented yet."),
        })
    }
}

impl<'a> Parse<'a> for Forward<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::forward>()?;

        let name = parser.parens(|parser| {
            parser.parse::<kw::export>()?;

            Ok((parser.parse()?))
        })?;

        Ok(Forward { name })
    }
}

impl<'a> Parse<'a> for Interfaces<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut interfaces: Interfaces = Default::default();

        while !parser.is_empty() {
            let interface = parser.parse::<Interface>()?;

            match interface {
                Interface::Export(export) => interfaces.exports.push(export),
                Interface::Type(ty) => interfaces.types.push(ty),
                Interface::Import(import) => interfaces.imports.push(import),
                Interface::Adapter(adapter) => interfaces.adapters.push(adapter),
                Interface::Forward(forward) => interfaces.forwards.push(forward),
            }
        }

        Ok(interfaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wast::parser::{self, ParseBuffer};

    fn buffer(input: &str) -> ParseBuffer {
        ParseBuffer::new(input).expect("Failed to build the parser buffer.")
    }

    #[test]
    fn test_interface_type() {
        let inputs = vec![
            "int", "float", "any", "string", "seq", "i32", "i64", "f32", "f64", "anyref",
        ];
        let outputs = vec![
            InterfaceType::Int,
            InterfaceType::Float,
            InterfaceType::Any,
            InterfaceType::String,
            InterfaceType::Seq,
            InterfaceType::I32,
            InterfaceType::I64,
            InterfaceType::F32,
            InterfaceType::F64,
            InterfaceType::AnyRef,
        ];

        assert_eq!(inputs.len(), outputs.len());

        for (input, output) in inputs.iter().zip(outputs.iter()) {
            assert_eq!(
                &parser::parse::<InterfaceType>(&buffer(input)).unwrap(),
                output
            );
        }
    }

    #[test]
    fn test_instructions() {
        let inputs = vec![
            "arg.get 7",
            "call 7",
            r#"call-export "foo""#,
            "read-utf8",
            r#"write-utf8 "foo""#,
            "as-wasm int",
            "as-interface anyref",
            "table-ref-add",
            "table-ref-get",
            "call-method 7",
            "make-record int",
            "get-field int 7",
            "const i32 7",
            "fold-seq 7",
            "add int",
            r#"mem-to-seq int "foo""#,
            r#"load int "foo""#,
            "seq.new int",
            "list.push",
            "repeat-until 1 2",
        ];
        let outputs = vec![
            Instruction::ArgumentGet { index: 7 },
            Instruction::Call { function_index: 7 },
            Instruction::CallExport { export_name: "foo" },
            Instruction::ReadUtf8,
            Instruction::WriteUtf8 {
                allocator_name: "foo",
            },
            Instruction::AsWasm(InterfaceType::Int),
            Instruction::AsInterface(InterfaceType::AnyRef),
            Instruction::TableRefAdd,
            Instruction::TableRefGet,
            Instruction::CallMethod(7),
            Instruction::MakeRecord(InterfaceType::Int),
            Instruction::GetField(InterfaceType::Int, 7),
            Instruction::Const(InterfaceType::I32, 7),
            Instruction::FoldSeq(7),
            Instruction::Add(InterfaceType::Int),
            Instruction::MemToSeq(InterfaceType::Int, "foo"),
            Instruction::Load(InterfaceType::Int, "foo"),
            Instruction::SeqNew(InterfaceType::Int),
            Instruction::ListPush,
            Instruction::RepeatUntil(1, 2),
        ];

        assert_eq!(inputs.len(), outputs.len());

        for (input, output) in inputs.iter().zip(outputs.iter()) {
            assert_eq!(
                &parser::parse::<Instruction>(&buffer(input)).unwrap(),
                output
            );
        }
    }

    #[test]
    fn test_param_empty() {
        let input = buffer("(param)");
        let output = FunctionType::Input(vec![]);

        assert_eq!(parser::parse::<FunctionType>(&input).unwrap(), output);
    }

    #[test]
    fn test_param() {
        let input = buffer("(param i32 string)");
        let output = FunctionType::Input(vec![InterfaceType::I32, InterfaceType::String]);

        assert_eq!(parser::parse::<FunctionType>(&input).unwrap(), output);
    }

    #[test]
    fn test_result_empty() {
        let input = buffer("(result)");
        let output = FunctionType::Output(vec![]);

        assert_eq!(parser::parse::<FunctionType>(&input).unwrap(), output);
    }

    #[test]
    fn test_result() {
        let input = buffer("(result i32 string)");
        let output = FunctionType::Output(vec![InterfaceType::I32, InterfaceType::String]);

        assert_eq!(parser::parse::<FunctionType>(&input).unwrap(), output);
    }

    #[test]
    fn test_export_with_no_param_no_result() {
        let input = buffer(r#"(@interface export "foo")"#);
    }

    #[test]
    fn test_export_with_some_param_no_result() {
        let input = buffer(r#"(@interface export "foo" (param i32))"#);
        let output = Interface::Export(Export {
            name: "foo",
            input_types: vec![InterfaceType::I32],
            output_types: vec![],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_export_with_no_param_some_result() {
        let input = buffer(r#"(@interface export "foo" (result i32))"#);
        let output = Interface::Export(Export {
            name: "foo",
            input_types: vec![],
            output_types: vec![InterfaceType::I32],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_export_with_some_param_some_result() {
        let input = buffer(r#"(@interface export "foo" (param string) (result i32 i32))"#);
        let output = Interface::Export(Export {
            name: "foo",
            input_types: vec![InterfaceType::String],
            output_types: vec![InterfaceType::I32, InterfaceType::I32],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_export_escaped_name() {
        let input = buffer(r#"(@interface export "fo\"o")"#);
        let output = Interface::Export(Export {
            name: r#"fo"o"#,
            input_types: vec![],
            output_types: vec![],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_import_with_no_param_no_result() {
        let input = buffer(r#"(@interface func $ns_foo (import "ns" "foo"))"#);
        let output = Interface::Import(Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![],
            output_types: vec![],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_import_with_some_param_no_result() {
        let input = buffer(r#"(@interface func $ns_foo (import "ns" "foo") (param i32))"#);
        let output = Interface::Import(Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![InterfaceType::I32],
            output_types: vec![],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_import_with_no_param_some_result() {
        let input = buffer(r#"(@interface func $ns_foo (import "ns" "foo") (result i32))"#);
        let output = Interface::Import(Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![],
            output_types: vec![InterfaceType::I32],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_import_with_some_param_some_result() {
        let input = buffer(
            r#"(@interface func $ns_foo (import "ns" "foo") (param string) (result i32 i32))"#,
        );
        let output = Interface::Import(Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![InterfaceType::String],
            output_types: vec![InterfaceType::I32, InterfaceType::I32],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_adapter_import() {
        let input =
            buffer(r#"(@interface adapt (import "ns" "foo") (param i32 i32) (result i32))"#);
        let output = Interface::Adapter(Adapter::Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![InterfaceType::I32, InterfaceType::I32],
            output_types: vec![InterfaceType::I32],
            instructions: vec![],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_adapter_export() {
        let input = buffer(r#"(@interface adapt (export "foo") (param i32 i32) (result i32))"#);
        let output = Interface::Adapter(Adapter::Export {
            name: "foo",
            input_types: vec![InterfaceType::I32, InterfaceType::I32],
            output_types: vec![InterfaceType::I32],
            instructions: vec![],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }
    #[test]
    fn test_forward() {
        let input = buffer(r#"(@interface forward (export "foo"))"#);
        let output = Interface::Forward(Forward { name: "foo" });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_interfaces() {
        let input = buffer(
            r#"(@interface export "foo"
  (param i32))

(@interface export "bar")

(@interface func $ns_foo (import "ns" "foo")
  (result i32))

(@interface func $ns_bar (import "ns" "bar"))

(@interface adapt (import "ns" "foo")
  (param i32)
  arg.get 42)

(@interface adapt (export "bar")
  arg.get 42)

(@interface forward (export "main"))"#,
        );
        let output = Interfaces {
            exports: vec![
                Export {
                    name: "foo",
                    input_types: vec![InterfaceType::I32],
                    output_types: vec![],
                },
                Export {
                    name: "bar",
                    input_types: vec![],
                    output_types: vec![],
                },
            ],
            types: vec![],
            imports: vec![
                Import {
                    namespace: "ns",
                    name: "foo",
                    input_types: vec![],
                    output_types: vec![InterfaceType::I32],
                },
                Import {
                    namespace: "ns",
                    name: "bar",
                    input_types: vec![],
                    output_types: vec![],
                },
            ],
            adapters: vec![
                Adapter::Import {
                    namespace: "ns",
                    name: "foo",
                    input_types: vec![InterfaceType::I32],
                    output_types: vec![],
                    instructions: vec![Instruction::ArgumentGet { index: 42 }],
                },
                Adapter::Export {
                    name: "bar",
                    input_types: vec![],
                    output_types: vec![],
                    instructions: vec![Instruction::ArgumentGet { index: 42 }],
                },
            ],
            forwards: vec![Forward { name: "main" }],
        };

        assert_eq!(parser::parse::<Interfaces>(&input).unwrap(), output);
    }
}
