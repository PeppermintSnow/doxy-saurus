def category_page_generator(docs, filename, title):
    """
    Parses a list of dictionaries from the doxy_extractor.extractor() 
    function and turns it into markdown strings stylized for docusaurus.

    Parameters
    ----------
    docs    :   list
                List of dictionaries from doxy_extractor.extractor().
    filename:   string 
                Name of which the content will be saved under.
    title   :   string
                Displayed title in the page.
    Returns
    -------
    dict
        A dictionary containing markdown file metadata and
        content.
    """

    sorted_docs_list = sorted(docs, key=lambda doc: doc.name)
    md_list = []
    md_list.extend(["---", f"title: \"{title}\"", "---\n"]);
    for related_doc in sorted_docs_list:
        if related_doc.declaration == 'function':
            related_params_formatted = [f"`{param['dtype']} {param_name}` "
                                        for param_name, param in related_doc.parameters.items()]
            md_list.extend([f"- [**`{related_doc.name}`**]({related_doc.name}) → `{related_doc.return_type}`  ",
                            f"_{related_doc.brief_description}_  ",
                            f"**Params**: {' '.join(related_params_formatted)}\n"])

    return ({'group': docs[0].parent_dir, 
             'subgroup': filename.split("_")[-1].replace('.h', ''),
             'filename': f"{filename}.md",
             'content': '\n'.join(md_list)
             })
