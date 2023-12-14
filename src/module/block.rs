/// Block of code in a module / function.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// Indentation level of the block.
    pub indent: usize,
    /// The statements in the block.
    pub statements: Vec<Statement>,
}

/// Statement for a block.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Raw line of code.
    Raw(String),
    /// Literal value.
    Literal {
        /// The value of the literal.
        value: String
    },
    /// Variable declaration.
    VarDecl {
        /// The type of the variable.
        var_type: VarType,
        /// The name of the variable.
        name: String,
        /// Initializer expression.
        initializer: Option<Box<Statement>>
    },
    /// Binary expression.
    Binary {
        /// The left side of the expression.
        left: Box<Statement>,
        /// The operator of the expression.
        operator: String,
        /// The right side of the expression.
        right: Box<Statement>
    },
    /// Block of code.
    Block(Box<Block>)
}

/// The type of a variable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VarType {
    Let, Const, Var
}

impl Statement {
    /// Create js code for the statement.
    pub fn generate(&self) -> String {
        match self {
            Statement::Raw(code) => code.clone(),
            Statement::Literal { value } => value.clone(),
            Statement::VarDecl { var_type, name, initializer } => {
                let var_type = match var_type {
                    VarType::Let => "let",
                    VarType::Const => "const",
                    VarType::Var => "var"
                };
                let initializer = match initializer {
                    Some(initializer) => format!(" = {}", initializer.generate()),
                    None => "".to_string()
                };
                format!("{} {}{}", var_type, name, initializer)
            },
            Statement::Binary { left, operator, right } => {
                format!("({} {} {})", left.generate(), operator, right.generate())
            }
            Statement::Block(block) => {
                block.generate()
            }
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Block {
    /// Create a new block.
    pub fn new(indent: usize) -> Self {
        Self {
            indent,
            statements: Vec::new(),
        }
    }

    /// Add a statement to the block.
    pub fn stmt(&mut self, statement: Statement) -> &mut Self {
        self.statements.push(statement);
        self
    }

    /// Add raw code to the block.
    pub fn raw(&mut self, code: &str) -> &mut Self {
        self.stmt(Statement::Raw(code.to_string()))
    }

    /// Add a variable declaration to the block.
    pub fn var_decl(&mut self, var_type: VarType, name: &str, initializer: Option<Statement>) -> &mut Self {
        self.stmt(Statement::VarDecl {
            var_type,
            name: name.to_string(),
            initializer: match initializer {
                Some(initializer) => Some(initializer.into()),
                None => None
            }
        })
    }

    /// Add a literal to the block.
    pub fn literal(&mut self, value: impl Into<Statement>) -> &mut Self {
        let value = value.into();
        if let Statement::Literal { value } = value {
            self.stmt(Statement::Literal { value })
        } else {
            panic!("Expected literal statement")
        }
    }

    /// Add a binary expression to the block.
    pub fn binary(&mut self, left: impl Into<Statement>, operator: &str, right: impl Into<Statement>) -> &mut Self {
        self.stmt(Statement::Binary {
            left: Box::new(left.into()),
            operator: operator.to_string(),
            right: Box::new(right.into())
        })
    }

    /// Generate the block's code.
    pub fn generate(&self) -> String {
        let mut code = String::new();

        for statement in &self.statements {
            code.push_str(&format!("{}{}\n", "    ".repeat(self.indent), statement.generate()));
        }

        code
    }
}

#[cfg(test)]
mod tests {
    use crate::module::block::{Block, Statement, VarType};

    #[test]
    fn test_raw_stmt() {
        let mut block = Block::new(0);
        block.stmt(Statement::Raw("foo".to_string()));
        assert_eq!(block.generate(), "foo\n");
    }

    #[test]
    fn test_var_decl_stmt() {
        let mut block = Block::new(0);
        block.stmt(Statement::VarDecl {
            var_type: VarType::Let,
            name: "foo".to_string(),
            initializer: None
        });
        assert_eq!(block.generate(), "let foo\n");
    }
}

impl From<&str> for Statement {
    fn from(code: &str) -> Self {
        Statement::Literal { value: format!("'{}'", code) }
    }
}

impl From<String> for Statement {
    fn from(code: String) -> Self {
        Statement::Literal { value: format!("'{}'", code) }
    }
}

impl From<i32> for Statement {
    fn from(code: i32) -> Self {
        Statement::Literal { value: code.to_string() }
    }
}

impl From<f32> for Statement {
    fn from(code: f32) -> Self {
        Statement::Literal { value: code.to_string() }
    }
}