use std::{fs, io, path::Path};
use crate::types::DocBlock;

enum State {
    Scanning,
    CollectingDoc,
    CollectingSig
}

pub fn get_doc_blocks(header: &Path) -> io::Result<Vec<DocBlock>> {
    let content = fs::read_to_string(header)?;
    
    let mut blocks = Vec::<DocBlock>::new();
    let mut curr_doc = String::new();
    let mut curr_sig = String::new();
    let mut state = State::Scanning;

    for raw_line in content.lines() {
        let line = raw_line.trim();

        match state {
            State::Scanning => {
                if line.starts_with("/**") {
                    state = State::CollectingDoc;
                }
            }
            State::CollectingDoc => {
                curr_doc.push_str(line);
                curr_doc.push('\n');

                if line.ends_with("*/") {
                    state = State::CollectingSig;
                }
            }
            State::CollectingSig => {
                curr_sig.push_str(line);
                curr_sig.push('\n');

                if line.ends_with(";") {
                    blocks.push(DocBlock {
                        doc: curr_doc.clone(),
                        signature: curr_sig.clone()
                    });

                    curr_doc.clear();
                    curr_sig.clear();
                    state = State::Scanning;
                }
            }
        }
    }

    Ok(blocks)
}
