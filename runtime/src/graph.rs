use crate::TROVE;

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use swc::{
    common::{sync::Lrc, FileName, Globals, SourceMap, Span},
    ecmascript::{
        ast::{EsVersion, KeyValueProp},
        codegen::{text_writer::JsWriter, Config, Emitter},
        parser::{lexer, Parser, StringInput, Syntax},
    },
};
use swc_bundler::{Bundler, Hook, Load, ModuleData, ModuleRecord, ModuleType, Resolve};

pub struct BundleLoader<'a> {
    cm: Lrc<SourceMap>,
    graph: &'a Graph,
}

impl<'a> BundleLoader<'a> {
    pub fn new(graph: &'a Graph, cm: Lrc<SourceMap>) -> Self {
        BundleLoader { cm, graph }
    }
}

impl Load for BundleLoader<'_> {
    fn load(&self, file: &FileName) -> Result<ModuleData> {
        if let FileName::Custom(filename) = file {
            let fm = self.cm.new_source_file(
                file.to_owned(),
                self.graph
                    .modules
                    .get(filename)
                    .unwrap_or_else(|| unreachable!("File not found: {}", filename))
                    .to_owned(),
            );

            let lexer = lexer::Lexer::new(
                Syntax::Es(Default::default()),
                EsVersion::latest(),
                StringInput::from(&*fm),
                None,
            );
            let mut parser = Parser::new_from(lexer);
            let module = parser.parse_module().unwrap();

            Ok(ModuleData {
                fm,
                module,
                helpers: Default::default(),
            })
        } else {
            unreachable!("Received request for unsupported filename {:?}", file)
        }
    }
}

pub struct BundleHook;

impl Hook for BundleHook {
    fn get_import_meta_props(
        &self,
        _span: Span,
        _module_record: &ModuleRecord,
    ) -> Result<Vec<KeyValueProp>> {
        // let mut value = module_record.file_name.into();
        // value.pop();
        // value.remove(0);

        Ok(vec![
            // ast::KeyValueProp {
            //     key: ast::PropName::Ident(ast::Ident::new("url".into(), span)),
            //     value: Box::new(ast::Expr::Lit(ast::Lit::Str(ast::Str {
            //         span,
            //         value: value.into(),
            //         kind: ast::StrKind::Synthesized,
            //         has_escape: false,
            //     }))),
            // },
            // ast::KeyValueProp {
            //     key: ast::PropName::Ident(ast::Ident::new("main".into(), span)),
            //     value: Box::new(if module_record.is_entry {
            //         ast::Expr::Member(ast::MemberExpr {
            //             span,
            //             obj: ast::ExprOrSuper::Expr(Box::new(ast::Expr::MetaProp(
            //                 ast::MetaPropExpr {
            //                     meta: ast::Ident::new("import".into(), span),
            //                     prop: ast::Ident::new("meta".into(), span),
            //                 },
            //             ))),
            //             prop: Box::new(ast::Expr::Ident(ast::Ident::new("main".into(), span))),
            //             computed: false,
            //         })
            //     } else {
            //         ast::Expr::Lit(ast::Lit::Bool(ast::Bool { span, value: false }))
            //     }),
            // },
        ])
    }
}

#[derive(Debug)]
pub struct Graph {
    pub modules: HashMap<String, String>,
}

impl Graph {
    const NAME: &'static str = "<pyret>";

    pub fn new(code: String, modules: HashSet<String>) -> Self {
        let mut modules = modules;
        modules.insert("global".to_owned());

        let mut modules: HashMap<String, String> = modules
            .iter()
            .map(|import| (import.to_owned(), TROVE.get(import).unwrap().code.clone()))
            .collect();

        modules.insert(Self::NAME.to_owned(), code);

        Self { modules }
    }

    pub fn bundle(&self) -> Result<String> {
        let globals = Globals::new();
        let cm: Lrc<SourceMap> = Default::default();
        let loader = BundleLoader::new(self, cm.clone());
        let hook = Box::new(BundleHook);

        let mut bundler = Bundler::new(
            &globals,
            cm.clone(),
            loader,
            self,
            swc_bundler::Config {
                require: false,
                module: ModuleType::Iife,
                ..Default::default()
            },
            hook,
        );

        let file = FileName::Custom(Self::NAME.to_owned());

        let mut entries = HashMap::new();
        entries.insert(Self::NAME.to_owned(), file);

        let output = bundler
            .bundle(entries)
            .context("Unable to output bundle during Graph::bundle().")?;

        let mut buf = Vec::new();
        let mut map = Vec::new();

        {
            let mut emitter = Emitter {
                cfg: Config { minify: false },
                cm: cm.clone(),
                comments: None,
                wr: Box::new(JsWriter::new(cm, "\n", &mut buf, Some(&mut map))),
            };

            emitter
                .emit_module(&output[0].module)
                .context("Unable to emit bundle during Graph::bundle().")?;
        }

        let src = String::from_utf8(buf).context("Emitted bundle is an invalid utf-8 string.")?;

        // {
        //     let mut buf = Vec::new();

        //     cm.build_source_map_from(&mut map, None)
        //         .to_writer(&mut buf)?;
        // }

        Ok(src)
    }
}

impl Resolve for Graph {
    fn resolve(&self, referrer: &FileName, specifier: &str) -> Result<FileName> {
        match referrer {
            FileName::Custom(..) => Ok(FileName::Custom(specifier.to_owned())),
            _ => unreachable!(
                "An unexpected referrer was passed when bundling: {:?}",
                referrer
            ),
        }
    }
}
