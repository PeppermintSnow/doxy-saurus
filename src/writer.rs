use std::{fs::File, io::{self, Write}, path::PathBuf};

use crate::types::{DocMeta, EnumItem, FunctionItem, StructItem};

fn write_meta_header(meta: &DocMeta, buf: &mut String) {
    let title = &meta.identifier.alias;

    let full_desc = format!(
        "{} {}", 
        &meta.short_desc,
        &meta.long_desc
    );

    let tags = format!(
        "{}, added {}, updated {}",
        &title,
        &meta.since_ver,
        &meta.latest_ver,
    );

    let keywords = format!(
        "ml-in-c, machine learning, {}",
        &title
    );

    let mut parsed_signature = String::new();
    for (index, line) in meta.signature.lines().enumerate() {
        if index > 0 && index < meta.signature.lines().count() - 1 {
            let mut parsed_line = String::from(line);

            if let Some(bpos) =  line.find("/") {
                parsed_line = line[0..bpos].trim().to_string();
            }

            parsed_signature.push_str(&format!("\n    {}", parsed_line));
        } else {
            parsed_signature.push_str(&format!("\n{}", line));
        }
    }

    let sidebar_position = if meta.signature.starts_with("typedef struct") {
        1
    } else if meta.signature.starts_with("typedef enum") {
        2
    } else {
        3
    };

    buf.extend([
        "---\n",
        &format!("title: \"{}\"\n", &title),
        &format!("description: \"{}\"\n", &full_desc.trim()),
        &format!("sidebar_position: \"{}\"\n", &sidebar_position.to_string()),
        &format!("tags: [{}]\n", &tags),
        &format!("keywords: [{}]\n", &keywords),
        "last_update:\n",
        &format!("  date: {}\n", &meta.created_date),
        &format!("  author: {}\n", &meta.author),
        "---\n\n",
        &format!("```c\n{}\n```\n\n", parsed_signature.trim()),
        &format!("{}\n\n", &meta.short_desc),
        &format!("{}\n\n", &meta.long_desc),
        &format!(":::info\n\nLast updated in version **{}**\n\n:::\n\n", &meta.latest_ver),
    ]);

    if &meta.latest_ver != &meta.since_ver {
        buf.push_str(&format!(
            ":::info\n\nAdded in version **{}**\n\n:::\n\n", 
            &meta.since_ver
        ));
    }
}

fn write_meta_footer(meta: &DocMeta, buf: &mut String) {
    if !meta.notes.is_empty() {
        buf.push_str("\n:::note\n\n");
        for note in &meta.notes {
            buf.push_str(&format!("- {}\n", note));
        }
        buf.push_str("\n:::\n");
    }

    if !meta.references.is_empty() {
        buf.push_str("\n:::tip see also\n\n");
        for reference in &meta.references {
            buf.push_str(&format!(
                "- [**`{}`**]({}) {}\n", 
                reference.subject,
                reference.subject.replace("()", ""),
                reference.desc
            ));
        }
        buf.push_str("\n:::\n");
    }
}

fn write_md(buf: &str, dir: &PathBuf, filename: &str) -> io::Result<()> {
    let filename = format!("{}.md", filename);
    let path = dir.join(filename);

    let mut file = File::create(path)?;
    file.write(buf.as_bytes())?;

    Ok(())
}

pub fn write_func(item: FunctionItem, dir: &PathBuf) -> io::Result<()> {
    let mut buf = String::new();

    write_meta_header(&item.meta, &mut buf);

    buf.push_str("## Parameters\n\n");
    for param in item.params {
        let space = if param.r#type.ends_with('*') { "" } else { " " };

        buf.push_str(&format!(
            "- **`{}{}{}`** ← _{}_  \n",
            param.r#type,
            space,
            param.name,
            param.desc
        ));
    }

    buf.extend([
        "## Return\n\n",
        &format!("- **`{}`** → {}", item.r#return.r#type, item.r#return.desc)
    ]);

    write_meta_footer(&item.meta, &mut buf);

    write_md(&buf, dir, &item.meta.identifier.alias)?;

    Ok(())
}

pub fn write_enum(item: EnumItem, dir: &PathBuf) -> io::Result<()> {
    let mut buf = String::new();

    write_meta_header(&item.meta, &mut buf);

    buf.push_str("## Members\n\n");
    for member in item.members {
        buf.push_str(&format!(
            "- **`{} = {}`** ← _{}_  \n",
            member.name,
            member.value,
            member.desc
        ));
    }

    write_meta_footer(&item.meta, &mut buf);

    write_md(&buf, dir, &item.meta.identifier.alias)?;

    Ok(())
}

pub fn write_struct(item: StructItem, dir: &PathBuf) -> io::Result<()> {
    let mut buf = String::new();

    write_meta_header(&item.meta, &mut buf);

    buf.push_str("## Properties\n\n");
    for field in item.fields {
        let space = if field.r#type.ends_with('*') { "" } else { " " };

        buf.push_str(&format!(
            "- **`{}{}{}`** ← _{}_  \n",
            field.r#type,
            space,
            field.name,
            field.desc
        ));
    }

    write_meta_footer(&item.meta, &mut buf);

    write_md(&buf, dir, &item.meta.identifier.alias)?;

    Ok(())
}
