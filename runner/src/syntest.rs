use syn::visit::Visit;
use syn::{Expr, ExprMethodCall, File, Lifetime, Type};

/// A syntest validation violation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Violation {
    pub rule: String,
    pub message: String,
}

/// Validate user code against the given syntest rules.
///
/// Returns an empty vec if all rules pass, or a list of violations.
pub fn validate(code: &str, rules: &[String]) -> Result<Vec<Violation>, String> {
    let syntax_tree: File =
        syn::parse_file(code).map_err(|e| format!("Failed to parse code: {e}"))?;

    let mut violations = Vec::new();

    for rule in rules {
        match rule.as_str() {
            "no_unwrap" => check_no_unwrap(&syntax_tree, &mut violations),
            "no_clone" => check_no_clone(&syntax_tree, &mut violations),
            "must_use_match" => check_must_use_match(&syntax_tree, &mut violations),
            "must_use_lifetime" => check_must_use_lifetime(&syntax_tree, &mut violations),
            other => {
                violations.push(Violation {
                    rule: other.to_string(),
                    message: format!("Unknown syntest rule: {other}"),
                });
            }
        }
    }

    Ok(violations)
}

/// Visitor that checks for `.unwrap()` method calls.
struct UnwrapVisitor {
    found: bool,
}

impl<'ast> Visit<'ast> for UnwrapVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        if node.method == "unwrap" {
            self.found = true;
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

fn check_no_unwrap(file: &File, violations: &mut Vec<Violation>) {
    let mut visitor = UnwrapVisitor { found: false };
    visitor.visit_file(file);
    if visitor.found {
        violations.push(Violation {
            rule: "no_unwrap".to_string(),
            message: "Code must not use .unwrap(). Use proper error handling instead.".to_string(),
        });
    }
}

/// Visitor that checks for `.clone()` method calls.
struct CloneVisitor {
    found: bool,
}

impl<'ast> Visit<'ast> for CloneVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        if node.method == "clone" {
            self.found = true;
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

fn check_no_clone(file: &File, violations: &mut Vec<Violation>) {
    let mut visitor = CloneVisitor { found: false };
    visitor.visit_file(file);
    if visitor.found {
        violations.push(Violation {
            rule: "no_clone".to_string(),
            message: "Code must not use .clone(). Use references or ownership transfer instead."
                .to_string(),
        });
    }
}

/// Visitor that checks for at least one `match` expression.
struct MatchVisitor {
    found: bool,
}

impl<'ast> Visit<'ast> for MatchVisitor {
    fn visit_expr(&mut self, node: &'ast Expr) {
        if matches!(node, Expr::Match(_)) {
            self.found = true;
        }
        syn::visit::visit_expr(self, node);
    }
}

fn check_must_use_match(file: &File, violations: &mut Vec<Violation>) {
    let mut visitor = MatchVisitor { found: false };
    visitor.visit_file(file);
    if !visitor.found {
        violations.push(Violation {
            rule: "must_use_match".to_string(),
            message: "Code must contain at least one match expression.".to_string(),
        });
    }
}

/// Visitor that checks for at least one lifetime annotation.
struct LifetimeVisitor {
    found: bool,
}

impl<'ast> Visit<'ast> for LifetimeVisitor {
    fn visit_lifetime(&mut self, node: &'ast Lifetime) {
        // Ignore the implicit 'static and '_ lifetimes for this check
        let ident = node.ident.to_string();
        if ident != "_" {
            self.found = true;
        }
        syn::visit::visit_lifetime(self, node);
    }

    fn visit_type(&mut self, node: &'ast Type) {
        // Also check type references for lifetime annotations
        syn::visit::visit_type(self, node);
    }
}

fn check_must_use_lifetime(file: &File, violations: &mut Vec<Violation>) {
    let mut visitor = LifetimeVisitor { found: false };
    visitor.visit_file(file);
    if !visitor.found {
        violations.push(Violation {
            rule: "must_use_lifetime".to_string(),
            message: "Code must contain at least one lifetime annotation (e.g., 'a, 'b)."
                .to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_unwrap_passes() {
        let code = r#"
            fn main() {
                let x: Option<i32> = Some(5);
                let y = x.unwrap_or(0);
            }
        "#;
        let violations = validate(code, &["no_unwrap".to_string()]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_no_unwrap_fails() {
        let code = r#"
            fn main() {
                let x: Option<i32> = Some(5);
                let y = x.unwrap();
            }
        "#;
        let violations = validate(code, &["no_unwrap".to_string()]).unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule, "no_unwrap");
    }

    #[test]
    fn test_no_clone_passes() {
        let code = r#"
            fn main() {
                let x = String::from("hello");
                let y = &x;
            }
        "#;
        let violations = validate(code, &["no_clone".to_string()]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_no_clone_fails() {
        let code = r#"
            fn main() {
                let x = String::from("hello");
                let y = x.clone();
            }
        "#;
        let violations = validate(code, &["no_clone".to_string()]).unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule, "no_clone");
    }

    #[test]
    fn test_must_use_match_passes() {
        let code = r#"
            fn main() {
                let x: Option<i32> = Some(5);
                match x {
                    Some(v) => println!("{}", v),
                    None => println!("none"),
                }
            }
        "#;
        let violations = validate(code, &["must_use_match".to_string()]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_must_use_match_fails() {
        let code = r#"
            fn main() {
                let x: Option<i32> = Some(5);
                if let Some(v) = x {
                    println!("{}", v);
                }
            }
        "#;
        let violations = validate(code, &["must_use_match".to_string()]).unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule, "must_use_match");
    }

    #[test]
    fn test_must_use_lifetime_passes() {
        let code = r#"
            fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
                if x.len() > y.len() { x } else { y }
            }
        "#;
        let violations = validate(code, &["must_use_lifetime".to_string()]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_must_use_lifetime_fails() {
        let code = r#"
            fn add(x: i32, y: i32) -> i32 {
                x + y
            }
        "#;
        let violations = validate(code, &["must_use_lifetime".to_string()]).unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule, "must_use_lifetime");
    }

    #[test]
    fn test_multiple_rules() {
        let code = r#"
            fn main() {
                let x = String::from("hello");
                let y = x.clone();
                let z: Option<&str> = Some("world");
                let w = z.unwrap();
            }
        "#;
        let rules = vec![
            "no_unwrap".to_string(),
            "no_clone".to_string(),
            "must_use_match".to_string(),
        ];
        let violations = validate(code, &rules).unwrap();
        assert_eq!(violations.len(), 3);
    }
}
