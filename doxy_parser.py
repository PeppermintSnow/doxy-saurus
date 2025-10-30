def parser(docs):
    """
    Parses a list of dictionaries from the doxy_extractor.extractor() 
    function and turns it into markdown strings stylized for docusaurus.

    Parameters
    ----------
    docs    :   list
                List of dictionaries from doxy_extractor.extractor().
    Returns
    -------
    list[dict]
        A list of dictionaries containing markdown file metadata and
        content.
    """

    md_entries = []
    for filename, docs_list in docs.items():
        for doc_entry in docs_list:
            # Store buffer as an array to avoid indentation quirks
            md_list = []

            doc_dict = vars(doc_entry)
            declaration, prototype, name, brief_description, extended_description, \
            version_since, version_current, date, example, notes, see_notes, \
            return_type, return_description, parameters, members, authors \
            = [doc_dict[key] for key in doc_dict.keys()]

            tags = [filename,
                    declaration.title(),
                    f"Added {version_since}",
                    f"Updated {version_current}"]
            keywords = ["ML-in-C", 
                        "machine learning", 
                        filename,
                        name,
                        f"{declaration} {name}",
                        f"{name} in {filename}"]

            full_description = f"{brief_description} {extended_description}"
            md_list.extend(["---",
                            f"title: \"{name}\"",
                            f"description: \"{full_description.strip()}\"",
                            f"tags: {tags}",
                            f"keywords: {keywords}",
                            "last_update:",
                            f"  date: {date}",
                            f"  author: {authors[0]}",
                            "---\n"])

            md_list.extend([f"```c\n{prototype}\n```\n",
                            f"{brief_description}\n",
                            f"{extended_description}\n",
                            f":::info\n\nLast updated in version **{version_current}**\n\n:::\n" \
                            if version_since != version_current else "",
                            f":::info\n\nAdded in version **{version_since}**\n\n:::\n"
                            ])

            if declaration == 'struct':
                md_list.append("## Struct Members\n")
                for member_name, member in members.items():
                    md_list.apppend(f"- `{member['dtype']} **{member_name}**` ← _{member['description']}_")
                if len(docs_list) > 1:
                    md_list.append("\n## Related Functions")
                    for related_doc in docs_list:
                        if related_doc.declaration == 'function':
                            related_params_formatted = [f"`{param['dtype']} {param_name}` "
                                                        for param_name, param in related_doc.parameters.items()]
                            md_list.extend([f"- [**`{related_doc.name}`**]({related_doc.name}) → `{related_doc.return_type}`  ",
                                            f"_{related_doc.brief_description}_  ",
                                            f"**Params**:  \n{' '.join(related_params_formatted)}\n"])
            elif declaration == 'function':
                md_list.append("## Parameters\n")
                for parameter_name, parameter in parameters.items():
                    md_list.append(f"- `{parameter['dtype']} **{parameter_name}**` ← _{parameter['description']}_  ")
                md_list.extend(["## Return\n",
                                f"- **`{return_type}`**",
                                f"**→** _{return_description}_" if return_description else ""
                                ])
            
            if len(notes) > 0:
                md_list.append("\n:::note\n")
                for note in notes:
                    md_list.append(f"- {note}")
                md_list.append("\n:::")

            if len(see_notes) > 0:
                md_list.append("\n:::tip see also\n")
                for note in see_notes:
                    md_list.append(f"- [{note}]({note.replace('()', '')})")
                md_list.append("\n:::")
            
            md_entries.append({'group': filename.replace('.h', ''),
                               'filename': f"{name}.md",
                               'content': '\n'.join(md_list)
                               })
    return md_entries;
