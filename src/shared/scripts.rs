use std::{collections::HashSet, path::Path};

use super::{datum::Datum, errors::RunningErrors, path::style_path};

pub enum ScriptFn {
    Derive,
    Plot,
    Sort,
    Migrate,
}

impl ScriptFn {
    fn to_str(&self) -> &'static str {
        match self {
            ScriptFn::Derive => "derive",
            ScriptFn::Plot => "plot",
            ScriptFn::Sort => "sort",
            ScriptFn::Migrate => "migrate",
        }
    }

    fn iter() -> std::slice::Iter<'static, ScriptFn> {
        static SCRIPT_FNS: [ScriptFn; 4] = [
            ScriptFn::Derive,
            ScriptFn::Plot,
            ScriptFn::Sort,
            ScriptFn::Migrate,
        ];
        SCRIPT_FNS.iter()
    }
}

// lets us reuse a scripting configuration for multiple function calls
pub struct RhaiSpace<'a> {
    pub engine: rhai::Engine,
    pub ast: rhai::AST,
    pub scope: rhai::Scope<'a>,
}

impl RhaiSpace<'_> {
    pub fn new(scripts_path: &Path) -> Self {
        // make engine with ability to manip datum
        let mut engine: rhai::Engine = rhai::Engine::new();
        engine.register_type::<Datum>();

        // parse script
        let script = std::fs::read_to_string(scripts_path)
            .expect(format!("Should read {}", style_path(scripts_path, "scripts file")).as_str());
        let ast = engine.compile(&script).expect(
            format!(
                "{} is a valid rhai script",
                style_path(scripts_path, "scripts file")
            )
            .as_str(),
        );

        // ensure the script meets our API
        let mut missing_fn_set = HashSet::<&str>::new();
        for script_fn in ScriptFn::iter() {
            missing_fn_set.insert(script_fn.to_str());
        }
        for function in ast.iter_functions() {
            // unlikely this iter is every going to be on a scale where performance matters
            for script_fn in ScriptFn::iter() {
                let script_fn_str = script_fn.to_str();
                if function.name.eq(script_fn_str) {
                    missing_fn_set.remove(script_fn_str);
                }
            }
        }
        if !missing_fn_set.is_empty() {
            let err_type = "Rhai script does not satisfy API".to_string();
            let mut running_errors = RunningErrors::new();
            for missing_fn in missing_fn_set {
                running_errors.add_err(
                    &err_type,
                    format!(
                        "Did not implement {} function",
                        console::style(missing_fn).cyan()
                    ),
                )
            }
            running_errors.print_errs();
        }

        // make shared scope
        let scope = rhai::Scope::new();

        RhaiSpace { engine, ast, scope }
    }

    pub fn call_fn<T: Clone + 'static>(&mut self, name: ScriptFn, args: impl rhai::FuncArgs) -> T {
        self
            .engine
            .call_fn::<T>(&mut self.scope, &self.ast, name.to_str(), args)
            .expect(format!("Could run {} fn", console::style(name.to_str()).magenta()).as_str())
    }
}
