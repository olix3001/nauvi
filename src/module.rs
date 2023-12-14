use std::{path::PathBuf, io::Write, ops::{DerefMut, Deref}};

pub mod block;

/// Struct that represents a js module (file).
pub struct Module {
    /// The name of the module (file without extension).
    pub name: String,
    /// The dependencies of the module.
    pub dependencies: Vec<Dependency>,
    /// Main block of the module.
    pub main_block: block::Block,
}

impl Module {
    /// Create a new module.
    pub fn create(name: &str) -> Self {
        Self {
            name: name.to_string(),
            dependencies: Vec::new(),
            main_block: block::Block::new(0),
        }
    }

    /// Add a dependency to the module.
    pub fn dep(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }
    /// Add multiple dependencies to the module.
    pub fn deps(&mut self, dependencies: Vec<Dependency>) {
        self.dependencies.extend(dependencies);
    }

    /// Generate the module's code and write it to a file.
    /// Returns the path of the file that was written to.
    pub fn generate(&self, path: &PathBuf) -> PathBuf {
        // If path is a directory, append the module's name to the path.
        let path = if path.is_dir() {
            path.join(format!("{}.js", self.name))
        } else {
            path.clone()
        };

        let file = std::fs::File::create(&path).unwrap();
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(self.generate_code_string().as_bytes()).unwrap();

        path
    }

    /// Generate the module's code and write it to any output.
    pub fn generate_to(&self, mut output: impl std::io::Write) {
        // Imports
        for dependency in &self.dependencies {
            output.write_all(&format!(
                "import {{ {} }} from '{}';\n",
                dependency.imports.join(", "),
                dependency.path
            ).as_bytes()).unwrap();
        }

        // Main block
        output.write_all(&self.main_block.generate().as_bytes()).unwrap();
    }

    /// Generate the module's code.
    pub fn generate_code_string(&self) -> String {
        let mut code = String::new();

        // Add the imports.
        for dependency in &self.dependencies {
            code.push_str(&format!(
                "import {{ {} }} from '{}';\n",
                dependency.imports.join(", "),
                dependency.path
            ));
        }

        // Add the main block.
        code.push_str(&self.main_block.generate());

        code
    }
}

impl Deref for Module {
    type Target = block::Block;

    fn deref(&self) -> &Self::Target {
        &self.main_block
    }
}

impl DerefMut for Module {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.main_block
    }
}

/// Module dependency.
#[derive(Debug, Clone, PartialEq)]
pub struct Dependency {
    /// List of imported things (eg. `import { foo, bar } from 'baz'` would be `["foo", "bar"]`).
    pub imports: Vec<String>,
    /// Path of the dependency (eg. `import { foo, bar } from 'baz'` would be `"baz"`).
    pub path: String,
}

impl Dependency {
    /// Create a new dependency.
    pub fn new(imports: Vec<String>, path: &str) -> Self {
        Self {
            imports,
            path: path.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_with_dependencies() {
        let mut module = Module::create("foo");
        module.dep(Dependency::new(vec!["foo".to_string()], "bar"));
        module.dep(Dependency::new(vec!["bar".to_string()], "baz"));

        assert_eq!(
            module.generate_code_string(),
            "import { foo } from 'bar';\nimport { bar } from 'baz';\n"
        );
    }

    #[test]
    fn test_module_with_main_block() {
        let mut module = Module::create("foo");
        module.stmt(block::Statement::Raw("foo".to_string()));
        module.dep(Dependency::new(vec!["foo".to_string()], "bar"));

        assert_eq!(module.generate_code_string(), "import { foo } from 'bar';\nfoo\n");
    }
}