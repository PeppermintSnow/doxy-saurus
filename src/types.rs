pub struct DocBlock {
    pub doc: String,
    pub signature: String
}

pub enum DocItem {
    Function(FunctionItem),
    Struct(StructItem),
    Enum(EnumItem)
}

#[derive(Default)]
#[derive(Debug)]
pub struct DocMeta {
    pub signature: String,
    pub identifier: Identifier,
    pub short_desc: String,
    pub long_desc: String,
    pub author: String,
    pub since_ver: String,
    pub latest_ver: String,
    pub created_date: String,
    pub notes: Vec<String>,
    pub references: Vec<Reference>,
}

#[derive(Default)]
#[derive(Debug)]
pub struct Identifier {
    pub original: String,
    pub alias: String
}

#[derive(Default)]
#[derive(Debug)]
pub struct Reference {
    pub subject: String,
    pub desc: String
}

#[derive(Default)]
#[derive(Debug)]
pub struct FunctionItem {
    pub meta: DocMeta,
    pub params: Vec<Param>,
    pub r#return: Return
}

#[derive(Default)]
pub struct EnumItem {
    pub meta: DocMeta,
    pub members: Vec<Member>
}

#[derive(Default)]
pub struct StructItem {
    pub meta: DocMeta,
    pub fields: Vec<Field>
}

#[derive(Debug)]
pub struct Param {
    pub r#type: String,
    pub name: String,
    pub desc: String
}

pub struct Field {
    pub r#type: String,
    pub name: String,
    pub desc: String,
}

pub struct Member {
    pub value: String,
    pub name: String,
    pub desc: String
}

#[derive(Default)]
#[derive(Debug)]
pub struct Return {
    pub r#type: String,
    pub desc: String
}
