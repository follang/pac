# Visitor Pattern

The `visit` module provides recursive AST traversal. It exposes:

- a `Visit<'ast>` trait with hook methods
- free functions like `visit_expression` and `visit_function_definition` that recurse into children

## The important rule

When you override a method, call the free function from `pac::visit`, not the trait method on
`self`. Calling `self.visit_*` from inside the override will recurse back into your override.

## Count function definitions

```rust
use pac::{ast, span, visit};
use pac::visit::Visit;

struct FunctionCounter {
    count: usize,
}

impl<'ast> Visit<'ast> for FunctionCounter {
    fn visit_function_definition(
        &mut self,
        node: &'ast ast::FunctionDefinition,
        span: &'ast span::Span,
    ) {
        self.count += 1;
        visit::visit_function_definition(self, node, span);
    }
}
```

## Collect identifiers from expressions

```rust
use pac::{ast, span, visit};
use pac::visit::Visit;

struct IdentifierCollector {
    names: Vec<String>,
}

impl<'ast> Visit<'ast> for IdentifierCollector {
    fn visit_identifier(&mut self, node: &'ast ast::Identifier, span: &'ast span::Span) {
        self.names.push(node.name.clone());
        visit::visit_identifier(self, node, span);
    }
}
```

## Use the visitor

```rust
use pac::driver::{parse, Config};
use pac::visit::Visit;

let parsed = parse(&Config::default(), "examples/sample.c")?;

let mut counter = FunctionCounter { count: 0 };
counter.visit_translation_unit(&parsed.unit);

println!("functions: {}", counter.count);
# Ok::<(), pac::driver::Error>(())
```

## When to override which method

- Override `visit_translation_unit` for whole-file summaries
- Override `visit_function_definition` for function-level analysis
- Override `visit_declaration` for declaration inspection
- Override `visit_expression` for expression-wide checks
- Override narrow hooks like `visit_call_expression` when you only care about one form

## Traversal style

Two common styles work well:

### Pre-order

Do work before recursing:

```rust
fn visit_expression(&mut self, node: &'ast ast::Expression, span: &'ast span::Span) {
    self.seen += 1;
    visit::visit_expression(self, node, span);
}
```

### Selective traversal

Only recurse when the node passes a filter:

```rust
fn visit_statement(&mut self, node: &'ast ast::Statement, span: &'ast span::Span) {
    if matches!(node, ast::Statement::Return(_)) {
        self.returns += 1;
    }
    visit::visit_statement(self, node, span);
}
```

## Practical advice

- Start with a broad hook like `visit_expression` while learning the tree.
- Narrow to specific hooks once you understand the shapes you care about.
- Pair the visitor with `Printer` when a subtree is unclear.
