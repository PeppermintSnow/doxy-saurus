use crate::types::{DocBlock, DocItem, DocMeta, EnumItem, Field, FunctionItem, Member, Param, Reference, Return, StructItem};

#[derive(PartialEq)]
#[derive(Debug)]
enum Tags {
    None,
    Brief,
    Details,
    Param,
    Return,
    Note,
    See,
    Author,
    Since,
    Version,
    Date
}

fn clean_str(str: &str) -> String {
    String::from(str)
        .replace("<", "\\<")
        .replace(">", "\\>")
}

pub fn parse_doc_block(block: &DocBlock) -> Option<DocItem> {
    let sig = clean_str(&block.signature.trim());
    let doc = clean_str(&block.doc);

    if let Some((mut meta, mut func)) = parse_doc(&doc) {
        meta.signature = sig.to_string();
        if !sig.starts_with("typedef") {
            func.meta = meta;
            parse_func_sig(&sig, &mut func);
            Some(DocItem::Function(func))
        } else if sig.starts_with("typedef enum") {
            let mut r#enum = EnumItem { 
                meta, 
                members: Vec::new()
            };

            parse_enum_sig(&sig, &mut r#enum);
            Some(DocItem::Enum(r#enum))
        } else if sig.starts_with("typedef struct") {
            let mut r#struct = StructItem { 
                meta, 
                fields: Vec::new()
            };

            parse_struct_sig(&sig, &mut r#struct);
            Some(DocItem::Struct(r#struct))
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_doc(doc: &str) -> Option<(DocMeta, FunctionItem)> {
    let mut meta = DocMeta::default();
    let mut func = FunctionItem::default();

    let mut last_tag = Tags::None;
    let mut buf = String::new();

    for raw_line in doc.lines() {
        let line = raw_line
            .trim_start_matches("*")
            .trim();

        if line.starts_with("@") && let Some((tag, desc)) = line.split_once(' ') {
            if !buf.is_empty() { save_and_flush(&mut meta, &mut func, &last_tag, &mut buf) }

            if tag == "@brief" { last_tag = Tags::Brief }
            else if tag == "@param" { last_tag = Tags::Param }
            else if tag == "@return" { last_tag = Tags::Return }
            else if tag == "@note" { last_tag = Tags::Note }
            else if tag == "@see" { last_tag = Tags::See }
            else if tag == "@author" { last_tag = Tags::Author }
            else if tag == "@since" { last_tag = Tags::Since }
            else if tag == "@version" { last_tag = Tags::Version }
            else if tag == "@date" { last_tag = Tags::Date }

            buf.push_str(desc);

            continue;
        } else if last_tag == Tags::Brief && line.is_empty() {
            save_and_flush(&mut meta, &mut func, &last_tag, &mut buf);
            last_tag = Tags::Details;
            continue;
        }

        if !line.is_empty() && line != "/" {
            buf.push(' ');
            buf.push_str(line);
        }

    }
    save_and_flush(&mut meta, &mut func, &last_tag, &mut buf);

    Some((meta, func))
}

fn save_and_flush(meta: &mut DocMeta, func: &mut FunctionItem, tag: &Tags, buf: &mut String) {
    let trimmed = buf.trim();
    match tag {
        Tags::Brief => meta.short_desc = trimmed.to_string(),
        Tags::Details => meta.long_desc = trimmed.to_string(),
        Tags::Param => {
            if let Some((name, desc)) = trimmed.split_once(' ') {
                func.params.push(Param { 
                    r#type: String::new(),
                    name: name.to_string(), 
                    desc: desc.to_string()
                });
            }
        },
        Tags::Return => func.r#return = Return {r#type: String::new(), desc: trimmed.to_string()},
        Tags::Note => meta.notes.push(trimmed.to_string()),
        Tags::See => {
            if let Some((subj, desc)) = trimmed.split_once(' ') {
                meta.references.push(Reference { 
                    subject: subj.to_string(), 
                    desc: desc.to_string()
                });
            }
        },
        Tags::Author => meta.author = trimmed.to_string(),
        Tags::Since => meta.since_ver = trimmed.to_string(),
        Tags::Version => meta.latest_ver = trimmed.to_string(),
        Tags::Date => meta.created_date = trimmed.to_string(),
        _ => println!("Warning: {} detected, but tag is not valid", trimmed)
    }

    buf.clear();
}

fn parse_func_sig(sig: &str, func: &mut FunctionItem) {
    if let Some((rest, params)) = sig.split_once('(') {
        if let Some((return_type, name)) = rest.rsplit_once(' ') {
            let mut return_type = String::from(return_type);
            let mut name = String::from(name);

            let ptr_count = name.chars().filter(|c| *c == '*').count();
            if ptr_count > 0 {
                return_type.push(' ');
                return_type.push_str(&"*".repeat(ptr_count));
                name = name[ptr_count..name.len()].to_string();
            }

            func.r#return.r#type = return_type.to_string();
            func.meta.identifier.original = name.to_string();
            func.meta.identifier.alias = name.to_string();

            let params = params[0..params.len() - 2].split(",");
            for (param, signature) in func.params.iter_mut().zip(params) {
                let signature = signature.trim();
                if let Some((param_type, param_name)) = signature.rsplit_once(" ") {
                    let mut param_type = String::from(param_type);
                    let mut param_name = String::from(param_name);

                    let ptr_count = param_name.chars().filter(|c| *c == '*').count();
                    if ptr_count > 0 {
                        param_type.push(' ');
                        param_type.push_str(&"*".repeat(ptr_count));
                        param_name = param_name[ptr_count..param_name.len()].to_string();
                    }

                    if param.name == param_name {
                        param.r#type = param_type;
                    }
                }
            }
        }
    }
}

fn parse_enum_sig(sig: &str, r#enum: &mut EnumItem) {
    let mut member_index = 0;
    for raw_line in sig.lines() {
        let line = raw_line.trim();

        if line.starts_with("typedef enum") {
            if let Some((_typedef, identifier)) = line.split_once("typedef enum") {
                if let Some((identifier, _)) = identifier.split_once("{") {
                    let identifier = identifier.trim();
                    if !identifier.is_empty() {
                        r#enum.meta.identifier.original = identifier.to_string();
                    }
                }
            }
        }

        if line.contains("/**<") {
            if let Some((dec, doc)) = line.rsplit_once("/**< ") {
                let dec = dec.replace(",", "");
                let dec = dec.trim();
                let doc = doc[0..doc.len() - 2].trim();

                let identifier: String;
                let value: String;
                if let Some((name, val)) = dec.split_once("=") {
                    identifier = name.to_string();
                    value = val.trim().to_string();
                } else {
                    identifier = dec.to_string();
                    value = member_index.to_string();
                }

                r#enum.members.push(Member {
                    value: value, 
                    name: identifier, 
                    desc: doc.to_string()
                });

                member_index += 1;
            }
        }

        if line.ends_with(";") {
            if let Some((_brace, alias)) = line.rsplit_once("}") {
                let alias = alias[0..alias.len() - 1].trim();
                r#enum.meta.identifier.alias = alias.to_string();
            }
        }
    }
}

fn parse_struct_sig(sig: &str, r#struct: &mut StructItem) {
    for raw_line in sig.lines() {
        let line = raw_line.trim();

        if line.starts_with("typedef struct") {
            if let Some((_typedef, identifier)) = line.split_once("typedef struct") {
                if let Some((identifier, _)) = identifier.split_once("{") {
                    let identifier = identifier.trim();
                    if !identifier.is_empty() {
                        r#struct.meta.identifier.original = identifier.to_string();
                    }
                }
            }
        }

        if line.contains("/**<") {
            if let Some((dec, desc)) = line.rsplit_once("/**< ") {
                let dec = dec.replace(",", "");
                let dec = dec.trim();
                let desc = desc[0..desc.len() - 2].trim().to_string();

                if let Some((r#type, name)) = dec.rsplit_once(" ") {
                    let mut r#type = String::from(r#type);
                    let mut name = String::from(name);

                    let ptr_count = name.chars().filter(|c| *c == '*').count();
                    if ptr_count > 0 {
                        r#type.push(' ');
                        r#type.push_str(&"*".repeat(ptr_count));
                        name = name[ptr_count..name.len() - 1].to_string();
                    }

                    r#struct.fields.push(Field {
                        r#type,
                        name, 
                        desc
                    });
                }
            }
        }

        if line.ends_with(";") {
            if let Some((_brace, alias)) = line.rsplit_once("}") {
                let alias = alias[0..alias.len() - 1].trim();
                r#struct.meta.identifier.alias = alias.to_string();
            }
        }
    }
}
