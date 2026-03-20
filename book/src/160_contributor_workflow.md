# Contributor Workflow

This chapter records a practical workflow for changing PAC safely.

## Smallest-reproducer rule

When fixing or extending the parser, start with the smallest input that demonstrates the issue.

That input should usually be one of:

- a direct `parse::*` snippet
- a reftest file
- a preprocessed snapshot

This keeps parser work focused.

## Recommended change sequence

1. reproduce the issue with the smallest possible input
2. decide whether the right test layer is `parse_api`, reftest, or full-app style
3. inspect the AST or failure position with `Printer` or structured errors
4. patch the relevant parser module
5. rerun the focused tests
6. only then widen out to broader test coverage

## Choosing the right test layer

Use `parse_api` tests when:

- the bug is a simple grammar acceptance issue
- you only need a success/failure assertion

Use reftests when:

- tree shape matters
- printer output is the clearest regression oracle

Use preprocessed or full-app style fixtures when:

- includes or macro expansion are part of the problem
- driver behavior matters

## Grammar-oriented debugging

A good parser debugging loop is:

1. isolate the failing syntax
2. parse with the right flavor
3. inspect the closest AST shape that already works
4. patch the grammar in the most local parser file possible

This is usually better than broad speculative rewrites.

## AST changes

If you add or change an AST node, review the corresponding surfaces too:

- visitor hooks in `visit`
- printer behavior in `print`
- any book examples that describe the shape
- reftest expectations if printer output changed

## Documentation changes

If a syntax family becomes better supported, update the book at the same time.
The important places are usually:

- flavor/extension guidance
- unsupported cases
- workflows
- AST or visitor examples

That keeps the book aligned with the real parser contract.
