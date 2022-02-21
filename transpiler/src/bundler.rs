use crate::utils::serialize_name;

use std::collections::{HashMap, HashSet};

use pyret_runtime::{Graph, Result, TROVE};

pub struct Bundler {
    pub modules: HashMap<String, HashSet<String>>,
}

impl Default for Bundler {
    fn default() -> Self {
        Self::new(HashSet::from(["global".to_owned()]))
    }
}

impl Bundler {
    pub fn new(modules: HashSet<String>) -> Self {
        let modules = modules
            .iter()
            .map(|module| {
                let exports = TROVE.get(module).unwrap().exports.to_owned();

                (module.to_owned(), exports)
            })
            .collect();

        Self { modules }
    }

    pub fn bundle(&self, code: String) -> Result<String> {
        let mut code = code;

        let mut modules = HashSet::new();

        for (module, imports) in self.modules.iter() {
            modules.insert(module.to_owned());

            let imports: Vec<String> = imports
                .iter()
                .map(|import| serialize_name(import.to_owned()))
                .collect();

            code.insert_str(
                0,
                &format!("import {{{}}} from \"{}\";", imports.join(","), module),
            );
        }

        let graph = Graph::new(code, modules);
        let bundle = graph.bundle()?;

        Ok(bundle)
    }
}
