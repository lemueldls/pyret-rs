use std::{collections::HashSet, fs, path::PathBuf};

use swc::{
    atoms::JsWord,
    common::{
        errors::{ColorConfig, Handler},
        sync::Lrc,
        FileName, SourceMap,
    },
    config::{Config, IsModule, JscConfig, Options},
    ecmascript::{
        ast::{self, EsVersion, Module},
        parser::{Syntax, TsConfig},
        transforms::pass::noop,
        visit::Fold,
    },
};

use once_cell::sync::Lazy;

pub static COMPILER: Lazy<Compiler> = Lazy::new(Compiler::default);

#[derive(Debug)]
pub struct Builtin {
    pub code: String,
    pub exports: HashSet<String>,
}

// pub struct Export {
//     type_ann: String,
// }

// type Exports<'a> = &'a mut HashMap<String, Export>;
type Exports<'a> = &'a mut HashSet<String>;

#[derive(Debug)]
struct Folder<'a> {
    directory: PathBuf,
    exports: Exports<'a>,
}

impl<'a> Folder<'a> {
    pub fn new(directory: PathBuf, exports: Exports<'a>) -> Self {
        Self { directory, exports }
    }

    fn add(&mut self, export: JsWord) {
        self.exports.insert(export.replace('$', "-"));
    }
}

impl Fold for Folder<'_> {
    fn fold_module(&mut self, module: Module) -> Module {
        use ast::*;

        for item in module.body.clone() {
            if let ModuleItem::ModuleDecl(declaration) = item {
                match declaration {
                    ModuleDecl::ExportDecl(export) => match export.decl {
                        Decl::Fn(function) => self.add(function.ident.sym),
                        Decl::Class(class) => self.add(class.ident.sym),
                        Decl::Var(var) => {
                            for decl in var.decls {
                                match decl.name {
                                    Pat::Ident(ident) => self.add(ident.id.sym),
                                    _ => todo!("Unimplemented"),
                                }
                            }
                        }
                        _ => todo!("Unimplemented"),
                    },
                    ModuleDecl::ExportNamed(export) => {
                        for specifier in export.specifiers {
                            if let ExportSpecifier::Named(export) = specifier {
                                match export.exported.unwrap() {
                                    ModuleExportName::Ident(ident) => self.add(ident.sym),
                                    ModuleExportName::Str(string) => self.add(string.value),
                                }
                            }
                        }
                    }
                    ModuleDecl::ExportAll(exports) => {
                        let runtime = COMPILER.compile(
                            self.directory
                                .join(exports.src.value.to_string())
                                .with_extension("ts")
                                .canonicalize()
                                .unwrap(),
                        );

                        self.exports.extend(runtime.exports);
                    }
                    _ => todo!("Unimplemented"),
                }
            }
        }

        module
    }
}

pub struct Compiler {
    compiler: swc::Compiler,
    handler: Handler,
    opts: Options,
}

impl Default for Compiler {
    fn default() -> Self {
        let cm: Lrc<SourceMap> = Default::default();
        let compiler = swc::Compiler::new(cm.clone());

        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, true, Some(cm));
        let opts = Options {
            is_module: IsModule::Bool(true),
            config: Config {
                minify: true,
                jsc: JscConfig {
                    syntax: Some(Syntax::Typescript(TsConfig::default())),
                    target: Some(EsVersion::latest()),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        Self::new(compiler, handler, opts)
    }
}

impl Compiler {
    pub fn new(compiler: swc::Compiler, handler: Handler, opts: Options) -> Self {
        Self {
            compiler,
            handler,
            opts,
        }
    }

    pub fn compile(&self, path: PathBuf) -> Builtin {
        let compiler = &self.compiler;

        let code = fs::read_to_string(path.clone()).unwrap();

        // let filename = path.file_name().unwrap().to_string_lossy().to_string();
        // let fm = compiler
        //     .cm
        //     .new_source_file(FileName::Custom(filename), code);

        // let filename = path.clone();
        // let fm = compiler.cm.new_source_file(FileName::Real(filename), code);

        let filename = PathBuf::from(path.file_name().unwrap());
        let fm = compiler.cm.new_source_file(FileName::Real(filename), code);

        let mut exports = HashSet::new();
        let folder = Folder::new(path.parent().unwrap().to_owned(), &mut exports);

        let code = compiler
            .process_js_with_custom_pass(
                fm,
                None,
                &self.handler,
                &self.opts,
                move |_| noop(),
                move |_| folder,
            )
            .unwrap()
            .code;

        Builtin { code, exports }
    }
}
