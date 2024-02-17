use std::collections::HashMap;

// groups errors with the same error type
pub struct RunningErrors(HashMap<String, Vec<String>>);

impl RunningErrors {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_err(&mut self, err_type: &String, err_details: String) {
        if !self.0.contains_key(err_type) {
            self.0.insert(err_type.clone(), Vec::new());
        }
        // unwrap is safe, since we insert if key does not already exist
        self.0.get_mut(err_type).unwrap().push(err_details);
    }

    pub fn print_errs(&mut self) {
        if !self.0.is_empty() {
            for (os_err, human_errs) in self.0.iter() {
                eprintln!("{} {}", console::style("Fatal:").red(), os_err);
                for err in human_errs {
                    eprintln!("  {}", err);
                }
            }
        }
    }
}
