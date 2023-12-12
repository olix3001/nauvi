/// Create a new import using js module syntax
#[macro_export]
macro_rules! import {
    ({ $($import:ident),+ } from $path:literal) => {
        $crate::module::Dependency::new(vec![$(stringify!($import).to_string()),+], $path)
    };
    ($import:ident from $path:literal) => {
        $crate::module::Dependency::new(vec![stringify!($import).to_string()], $path)
    };
}

/// Create variable declaration statement
#[macro_export]
macro_rules! var {
    // Let variable
    (let $name:ident = $initializer:expr) => {
        $crate::module::block::Statement::VarDecl {
            var_type: $crate::module::block::VarType::Let,
            name: stringify!($name).to_string(),
            initializer: Some(Box::new($initializer.into())),
        }
    };
    (let $name:ident) => {
        $crate::module::block::Statement::VarDecl {
            var_type: $crate::module::block::VarType::Let,
            name: stringify!($name).to_string(),
            initializer: None,
        }
    };

    // Const variable
    (const $name:ident = $initializer:expr) => {
        $crate::module::block::Statement::VarDecl {
            var_type: $crate::module::block::VarType::Const,
            name: stringify!($name).to_string(),
            initializer: Some(Box::new($initializer.into())),
        }
    };

    // Var variable
    (var $name:ident = $initializer:expr) => {
        $crate::module::block::Statement::VarDecl {
            var_type: $crate::module::block::VarType::Var,
            name: stringify!($name).to_string(),
            initializer: Some(Box::new($initializer.into())),
        }
    };
    (var $name:ident) => {
        $crate::module::block::Statement::VarDecl {
            var_type: $crate::module::block::VarType::Var,
            name: stringify!($name).to_string(),
            initializer: None,
        }
    };
}

/// Create new binary expression. This uses polish notation with commas (eq. + 1, 2)
#[macro_export]
macro_rules! binary {
    (+ $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "+".to_string(),
            right: Box::new($right.into()),
        }
    };
    (- $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "-".to_string(),
            right: Box::new($right.into()),
        }
    };
    (* $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "*".to_string(),
            right: Box::new($right.into()),
        }
    };
    (/ $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "/".to_string(),
            right: Box::new($right.into()),
        }
    };
    (% $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "%".to_string(),
            right: Box::new($right.into()),
        }
    };
    (== $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "==".to_string(),
            right: Box::new($right.into()),
        }
    };
    (!= $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "!=".to_string(),
            right: Box::new($right.into()),
        }
    };
    (=== $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "===".to_string(),
            right: Box::new($right.into()),
        }
    };
    (!== $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "!==".to_string(),
            right: Box::new($right.into()),
        }
    };
    (< $left:expr, $right:expr) => {
        $crate::module::block::Statement::Binary {
            left: Box::new($left.into()),
            operator: "<".to_string(),
            right: Box::new($right.into()),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::module::Dependency;

    #[test]
    fn test_import_macro() {
        let import = import!({ foo, bar, baz } from "qux");
        assert_eq!(
            import,
            Dependency::new(vec!["foo".to_string(), "bar".to_string(), "baz".to_string()], "qux")
        );

        let import = import!(foo from "bar");
        assert_eq!(import, Dependency::new(vec!["foo".to_string()], "bar"));
    }

    #[test]
    fn test_var_macro() {
        let var = var!(let foo = 42);
        assert_eq!(
            var,
            crate::module::block::Statement::VarDecl {
                var_type: crate::module::block::VarType::Let,
                name: "foo".to_string(),
                initializer: Some(Box::new(42.into())),
            }
        );

        let var = var!(let foo);
        assert_eq!(
            var,
            crate::module::block::Statement::VarDecl {
                var_type: crate::module::block::VarType::Let,
                name: "foo".to_string(),
                initializer: None,
            }
        );

        let var = var!(const foo = 42);
        assert_eq!(
            var,
            crate::module::block::Statement::VarDecl {
                var_type: crate::module::block::VarType::Const,
                name: "foo".to_string(),
                initializer: Some(Box::new(42.into())),
            }
        );

        let var = var!(var foo = 42);
        assert_eq!(
            var,
            crate::module::block::Statement::VarDecl {
                var_type: crate::module::block::VarType::Var,
                name: "foo".to_string(),
                initializer: Some(Box::new(42.into())),
            }
        );

        let var = var!(var foo);
        assert_eq!(
            var,
            crate::module::block::Statement::VarDecl {
                var_type: crate::module::block::VarType::Var,
                name: "foo".to_string(),
                initializer: None,
            }
        );
    }

    #[test]
    fn test_binary_macro() {
        let binary = binary!(+ 1, 2);
        assert_eq!(
            binary,
            crate::module::block::Statement::Binary {
                left: Box::new(1.into()),
                operator: "+".to_string(),
                right: Box::new(2.into()),
            }
        );
    }
}