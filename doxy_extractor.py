from dataclasses import dataclass, field
import re
import os

@dataclass
class DocEntry:
    declaration: str = ''           # Kind of documentation declaration
    prototype: str = ''             # The prototype declaration of the function/struct
    name: str = ''                  # Name of the function/struct
    brief_description: str =''      # Brief/short description (@brief)
    extended_description: str = ''  # Appended description following @brief
    version_since: str = ''         # Version when the feature was added (@since)
    version_current: str = ''       # Version when the feature was last updated (@current)
    date: str = ''                  # Date of the last update (@date)
    example: str = ''               # Example code/application (@code ... @endcode)
    # Additional notes for the documentation (@note)
    notes: list[str] = field(default_factory=list) 
    # References to related documentation entries (@see)
    see_notes: dict[str, str] = field(default_factory=dict)
    return_type: str = ''           # For functions, type of the return value
    return_description: str = ''    # For functions, details of the return value (@return/s)
    # For functions, dictionary of param-doc pairs (@params)
    parameters: dict[str, dict[str, str]] = field(default_factory=dict) 
    # For structs, dictionary of member-doc pairs (inline)
    members: dict[str, dict[str, str]] = field(default_factory=dict)
    # List of authors (@author)
    authors: list[str] = field(default_factory=list)

Docs = dict[str, list[DocEntry | None]]

def extractor(input_dir, subdirs, extensions):
    """
    Traverses through the target subdirs under input_dir
    and extracts doxygen comment blocks with function/struct
    details into a dictionary with metadata.

    Parameters
    ----------
    input_dir   :   string
                    Target path to read recursively.
    subdirs     :   list
                    List of directories to search and parse.
    extensions  :   list
                    List of file extensions to read.
    Returns
    -------
    dict
        A dictionary containing all function/struct details
        with corresponding doxygen comments and metadata.
    """

    valid_comment_tags = ['@brief', '@param', '@author', '@since', '@version', '@date', '@code', '@endcode', '@note', '@see']
    dirs_re = [re.compile(f"{re.escape(input_dir)}/{re.escape(subdir)}") for subdir in subdirs]
    comment_re = re.compile(r"/\*\*(.*?)\*/", re.DOTALL)
    func_re = re.compile(r"([\w\s\*\_]+?)\s+(\**\w+)\s*\((.*?)\)\s*;", re.MULTILINE)
    struct_re = re.compile(r"\s*typedef\s+struct\s+\{(.*?)\}\s*(\w+);", re.DOTALL)
    struct_doc_re = re.compile(r"\s*(\w+)\s*(\**\w+);\s*/\*\*<(.*?)\*/", re.MULTILINE)
    
    docs: Docs = {}
    for (pwd, _, files) in os.walk(input_dir):
        if not any([dir_re.match(pwd) for dir_re in dirs_re]):
            continue # Skip dirs not matching target subdirs
        
        # Filter files matching target extensions
        target_files = [file for file in files if file.split('.')[-1] in extensions]
        for file in target_files:
            docs[file] = [] # Initialize entry

            with open(os.path.join(pwd, file), 'r', encoding='utf-8') as f:
                content = f.read()

            # Iterate through matched comment blocks
            for comment_block in comment_re.finditer(content):
                # Split by @ to capture tag-value pairs separately
                comment_end_idx = comment_block.end()
                comment_fields = comment_block.group(1).split('\n')
                comment_fields = [re.sub(r"^\s\*\s|^\s\*$", '', comment_field) for comment_field in comment_fields]
                # Maintain the @ delimiter to avoid false-positive matching
                comment_fields = ['@'+comment_field for comment_field in '\n'.join(comment_fields).split('@') if comment_field.strip('\n')]

                # Initialize entry
                doc_entry = DocEntry()

                # Extract comment block fields
                for comment_field in comment_fields:
                    comment_parts = re.match(r"^(@\w+)\s*\n*(.*)", comment_field, re.DOTALL)
                    if not comment_parts:
                        continue

                    comment_tag = comment_parts.group(1)
                    comment_value = comment_parts.group(2).strip('\n')

                    comment_tag_is_valid = comment_tag in valid_comment_tags
                    if comment_tag_is_valid:
                        if comment_tag == '@brief':
                            description_parts = comment_value.split('\n\n')
                            doc_entry.brief_description = description_parts[0]
                            doc_entry.extended_description = '\n'.join(description_parts[1:])
                        elif comment_tag == '@param':
                            comment_param_parts = comment_value.split(' ')
                            comment_param_name = comment_param_parts[0]
                            comment_param_description = ' '.join(comment_param_parts[1:])
                            doc_entry.parameters[comment_param_name] = {}
                            doc_entry.parameters[comment_param_name]['description'] = comment_param_description
                        elif comment_tag in ['@return', '@returns']:
                            doc_entry.return_description = comment_value
                        elif comment_tag == '@see':
                            note_parts = comment_value.split(' ')
                            doc_entry.see_notes[note_parts[0]] = ' '.join(note_parts[1:])
                        elif comment_tag == '@note':
                            doc_entry.notes.append(comment_value)
                        elif comment_tag == '@author':
                            doc_entry.authors.append(comment_value)
                        elif comment_tag == '@since':
                            doc_entry.version_since = comment_value
                        elif comment_tag == '@version':
                            doc_entry.version_current = comment_value
                        elif comment_tag == '@date':
                            doc_entry.date = comment_value
                        elif comment_tag == '@code':
                            doc_entry.example = comment_value
        
                # Content following the comment block for detection of declarations
                content_after = content[comment_end_idx:]
                content_next_line = content_after.split('\n')
                if len(content_next_line) > 1:
                    content_next_line = content_next_line[1]
                else:
                    content_next_line = ''

                # Match function prototype and extract details (if applicable)
                func_prototype = func_re.match(content_next_line)
                if func_prototype:
                    doc_entry.declaration = 'function'
                    doc_entry.prototype = func_prototype.group(0)
                    func_return_pointers = func_prototype.group(2).count('*')
                    doc_entry.name = func_prototype.group(2).replace('*', '')
                    doc_entry.return_type = func_prototype.group(1) + ('*' * func_return_pointers)
                    for func_param in func_prototype.group(3).split(','):
                        func_param_parts = func_param.split(' ')
                        func_param_pointers = func_param_parts[-1].count('*')
                        func_param_type = ' '.join(func_param_parts[:-1]) + ('*' * func_param_pointers)
                        func_param_name = func_param_parts[-1].replace('*', '')
                        doc_entry.parameters[func_param_name]['dtype'] = func_param_type.strip()
                
                # Match struct definition and extract details (if applicable)
                struct_def = struct_re.match(content_after)
                if struct_def:
                    doc_entry.declaration = 'struct'
                    doc_entry.prototype = re.sub(r"/.*/", '', struct_def.group(0).strip('\n'))
                    doc_entry.name = struct_def.group(2)
                    struct_members = struct_def.group(1).strip().split('\n')
                    for struct_member in struct_members:
                        struct_member_parts = struct_doc_re.match(struct_member)
                        if struct_member_parts:
                            struct_member_pointers = struct_member_parts.group(2).count('*')
                            struct_member_type = struct_member_parts.group(1) + '*' * struct_member_pointers
                            struct_member_name = struct_member_parts.group(2).replace('*', '')
                            struct_member_description = struct_member_parts.group(3).strip()
                            doc_entry.members[struct_member_name] = {}
                            doc_entry.members[struct_member_name]['dtype'] = struct_member_type
                            doc_entry.members[struct_member_name]['description'] = struct_member_description

                if doc_entry.declaration:
                    docs[file].append(doc_entry)
    return docs
