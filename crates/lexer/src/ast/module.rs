use super::Stmt;

#[derive(Debug)]
pub enum ModuleItem {
    ModuleDecl(ModuleDecl),
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum ModuleDecl {
    Include(IncludeDecl),
    Import {
        source: SourceSpec,
        spec: ImportDecl,
    },
    Provide {
        from: Option<String>,
        specs: Vec<ProvideDecl>,
    },
    Use(UseDecl),
}

#[derive(Debug)]
pub enum IncludeDecl {
    Source(SourceSpec),
    From {
        module_ref: String,
        specs: Vec<ModuleSpec>,
    },
}

#[derive(Debug)]
pub enum ImportDecl {
    As(String),
    From(Vec<String>),
}

#[derive(Debug)]
pub struct ProvideDecl {
    pub module: ModuleSpec,
    pub hiding: Vec<String>,
}

#[derive(Debug)]
pub enum SourceSpec {
    Builtin(String),
    File(String),
    JsFile(String),
    MyGDrive(String),
    SharedGDrive(String),
}

#[derive(Debug)]
pub struct ModuleSpec {
    pub module: ModuleType,
    pub as_name: String,
}

#[derive(Debug)]
pub enum ModuleType {
    Name(String),
    Type(String),
    Module(String),
    Data(String),
}

#[derive(Debug)]
pub enum UseDecl {
    Context(String),
}
