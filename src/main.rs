mod types;
mod discovery;
mod scanner;
mod parser;
mod writer;

use std::{fs::{self, File}, io::{self}, path::{Path, PathBuf}};
use crate::{
	discovery::get_headers, parser::parse_doc_block, scanner::get_doc_blocks, types::DocItem, writer::{write_enum, write_func, write_struct}
};

fn main() -> io::Result<()> {
    let libdir = std::env::args().nth(1).expect("library directory not specified");
    let outdir = std::env::args().nth(2).expect("output directory not specified");

    let libpath = Path::new(&libdir);
    let outpath = Path::new(&outdir);

    if !libpath.exists() { panic!("library directory does not exist"); }
    if !outpath.exists() { panic!("output directory does not exist"); }


    let mut headers = Vec::<PathBuf>::new();
    get_headers(libpath, &mut headers)?;

    for header in headers {
        let mut module = String::new();
        let mut submodule = String::new();

        if let Some(str) = header.to_str() {
            if let Some((rest, filename)) = str.rsplit_once('/') {
                if let Some((_, parent)) = rest.rsplit_once('/') {
                    module = filename.replace(".h", "").to_string();
                    submodule = parent.to_string();
                }
            }
        }

        let dir = outpath.join(submodule).join(module);
        fs::create_dir_all(&dir)?;

        let doc_blocks = get_doc_blocks(&header)?;
        for block in doc_blocks {
            if let Some(doc_item) = parse_doc_block(&block) {
                match doc_item {
                    DocItem::Enum(item) => write_enum(item, &dir)?,
                    DocItem::Struct(item) => write_struct(item, &dir)?,
                    DocItem::Function(item) => write_func(item, &dir)?
                }
            }
        }
    }

    Ok(())
}
