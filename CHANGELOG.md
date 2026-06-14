# Changelog

## [1.9.0] - 2026-06-14

### Added

- **`walk` keyed iteration** — `walk $items with item by item.id { … }` attaches a key expression to each row. The optional `by <expr>` clause sits after the loop variable; `DsIter::get_key()` exposes it (`None` when absent). Backends can use the key to match rows by identity across updates instead of by position. `DsRune::inscribe_iter` gains a `key: Option<&syn::Expr>` parameter.

### Changed

- **`xrune-fmt` round-trips `by <key>`** — the formatter preserves the `with item by item.id` clause.

## [1.8.0] - 2026-06-14

### Added

- **`$` sigil on attribute values** — an attribute value accepts the same reactive sigil as control flow: `attr: $signal` (a bare path) and `attr: ${ expr }` (a block) set a `reactive: bool` flag on `DsAttr`, exposed to codegen backends. Without `$` the value parses as a plain expression and stays non-reactive. The `$` is stripped during parsing.
- **`walk ${ block }`** — `walk` now accepts a block iterable (`walk ${ rows.iter() } with item { ... }`) in addition to a bare `walk $items with item { ... }`, matching the `if` / `match` heads.

### Changed

- **`xrune-fmt` round-trips `$` on attributes** — the formatter preserves `attr: $signal` and `attr: ${ block }`, with no gap between `$` and the brace.

## [1.7.0] - 2026-06-13

### Added

- **`$` sigil marks reactive control flow** — a leading `$` on an `if` condition (`if $cond { ... }`), a `match` scrutinee (`match $expr { ... }`), or a `walk` iterable (`walk $items with item { ... }`) sets a `reactive` flag on the node, exposed via `is_reactive()`. The `$` is stripped during parsing; bare `if cond { ... }` is unchanged and stays non-reactive. `DsRune::inscribe_if`, `inscribe_iter`, and `inscribe_match` receive the flag so backends can decide whether to wrap the branch in a reactive scope.
- **`if` / `elif` / `else` branches** — `if` now accepts an optional `elif <cond> { ... }` chain (`elif` is a single keyword, not `else if`) and a terminal `else { ... }`. `DsTree` carries the chain through `else_branch: Option<DsTreeRef>`; `elif` parses as a nested `If` node and `else` as a `DsNode::Else` node whose body lives in its children. `DsRune::inscribe_if` gains an `else_branch` parameter.

### Changed

- **`xrune-fmt` round-trips `$`, `elif`, and `else`** — the formatter preserves the reactive `$` on `if` / `walk` / `match`, emits `elif` (not `else if`), and reproduces the terminal `else`.

## [1.6.0] - 2026-06-10

### Changed

- **`xrune-fmt` inlines `on` handlers** — an `on EventKind { ... }` clause now follows the attribute or enchant close on the same line (`) on Tap { ... }`, `] on Tap { ... }`) and chains across multiple handlers, with the body indented under the widget. Previously each `on` clause started its own indented line, which read as detached from the node.
- **`xrune-fmt` omits braces for childless nodes** — a widget node with no children formats without a trailing `{}` (`Text ("x")` instead of `Text ("x") {}`). Nodes with children, and `if` / `walk` / `@niche` / `match`, are unchanged. Input with explicit `{}` still parses.

## [1.5.2] - 2026-06-08

### Added

- **Callback-form `on EventKind`** — `on EventKind(args, cb)` with the trailing arg as a callable expression and no `{ ... }` body is now valid. Combinations: `on Tap(cb)` (bare callback), `on Tap(2, cb)` (count=2 callback). Body-form and callback-form coexist on the same widget.
- **`DsOn.body` is `Option<syn::Block>`** — `get_body()` returns `Option<&syn::Block>`. Hosts that consume `DsOn` decide what the trailing arg means when body is `None`.

### Fixed

- **`xrune-fmt` pretty-prints `on EventKind { ... }` body** — the body of an `on` clause used to come out as a single line with token-stream spacing (`a . b ( ) ;`). Now it runs through `prettyplease` and re-indents each statement under the call-site indent, matching how the rest of the file reads.

### Breaking

- `DsOn::get_body()` signature changes from `&syn::Block` to `Option<&syn::Block>`. Direct consumers must match on the option.

## [1.5.1] - 2026-06-07

`on EventKind` syntax now attaches handlers to widgets unambiguously. The
1.5.0 form (where `on` could nest inside a widget body, leaving "is this
attached to the widget or one of its children?" ambiguous) is rejected.
1.5.0 has been yanked.

### Added

- **Form C** — inline position between attrs and children body:
  `Widget() on EventKind { body } { children }`. Multiple `on` clauses
  stack here.
- **Form B** — modifier-chain position after the widget:
  `Widget() {} on EventKind { body }`. Multiple `on` clauses chain.
  Inside a nested body, `on` attaches to the nearest preceding sibling
  widget.
- `DsWidget::get_on_handlers()` exposes the collected handlers; both
  Form B and Form C populate the same `Vec<DsOn>`.
- `DsRune::inscribe_widget` takes `on_handlers: &[DsOn]` so generators
  see every handler attached to the widget being inscribed.

### Removed

- `DsRune::inscribe_on` — replaced by the `on_handlers` parameter on
  `inscribe_widget`. Implementations no longer track on-handlers
  separately from the widget they belong to.
- `DsNode::On` variant — `on` is no longer a tree-level node.

### Fixed

- A stray top-level `on EventKind` (no preceding widget) now produces a
  parse error instead of panicking.

## [1.5.0] - 2026-06-07

DSL gains an `on EventKind(args) { body }` node that hosts can lower into
event handlers. `body` is parsed as a `syn::Block` so consumers receive a
Rust-shaped block they can splice into generated handler fns directly.

### Added

- **`on EventKind { body }` nodes** (`DsOn`) — sit alongside widget /
  niche / match / if / iter as DsTree-level nodes. Body is `syn::Block`;
  args is `Vec<syn::Expr>`; an optional single-segment `qualifier` carries
  the `Path::` prefix.
- **`on Path::EventKind { body }` qualified form** — `qualifier` is
  `Some(ident)` for the prefix; multi-segment paths (`Foo::Bar::Baz`)
  raise a parse error.
- **`on EventKind(p1, p2, ...) { body }` parameter list** — comma-separated
  expressions in the parens, exposed via `DsOn::get_args()`.
- **`DsRune::inscribe_on(qualifier, name, args, body)`** — new trait method
  every Rune impl must provide.
- **`on` joins `walk` / `with` as a custom keyword** — `is_custom_keyword`
  routes `on …` to `DsOn::parse` before the widget peek, so `on Foo` is
  not mistaken for a widget named `on`.

### Breaking

- `DsRune` adds `inscribe_on` (no default impl); existing impls must add it.

## [1.4.0] - 2026-06-07

DSL gains three new node types — niche, match, and headerless widgets — plus optional commas in the root header and unnamed positional attrs. Host runes (mirui-incant style codegen, xrune-fmt style printing) need to handle two new `DsRune` methods and an `Option`-shaped attr name.

### Added

- **`@name { ... }` niche nodes** (`DsNiche`) — anchor-routed children, parsed as `@`-prefix + ident + brace block. Expose `DsNiche::get_name()` + children.
- **`match expr { Pat => { ... } ... }` match nodes** (`DsMatch`, `DsMatchArm`) — Rust-style pattern matching inside the DSL. `DsMatch::get_scrutinee()`, `get_arms()`; each `DsMatchArm` carries its own `syn::Pat` + children sub-tree.
- **Optional empty children block on widget nodes** — `Foo (attrs)` (no `{}`) parses as a Widget with no children. `if` / `walk` / `@niche` / `match` still require their bodies — they'd be no-ops otherwise.
- **Optional commas in the root header** — `:( parent: r, world: w, :)` and the existing comma-less form both parse.
- **Multi-line root header is enforced** — `:(parent: r world: w:)` on a single line errors at parse time when there's more than one context attr; single-attr headers stay relaxed.
- **`DsRune::inscribe_niche` and `inscribe_match`** — new trait methods every Rune impl must provide.

### Changed

- **`DsAttr::name` is now `Option<syn::Ident>`** to support positional args (e.g. `Text("hi")`). The parser tries the `name: value` shape first and falls back to bare `value` with `name = None`. Existing `name: value` callers parse identically; downstream code that read `attr.name` directly needs to match on the Option.
- **`DsTree::parse` no longer demands an outer brace block on Widget and Match nodes** — they consume their own bodies. If / Iter / Niche still go through `syn::braced!`.

### Breaking

- `DsAttr.name: Option<Ident>` (was `Ident`) — every consumer that read `.name` needs a match.
- `DsRune` adds two methods (no default impls); existing impls must add `inscribe_niche` and `inscribe_match`.
- `xrune-nexus` now needs `syn` with the `full` feature (`syn::Pat` is gated behind it).

## [1.2.0] - 2026-05-08

### Added

- Enchants: optional `[expr, expr, ...]` block on widget nodes
- `DsWidget::get_enchants()` — access enchant expressions

### Breaking

- `DsRune::inscribe_widget` signature changed (added `enchants: &[syn::Expr]` parameter)

## [1.1.3] - 2026-05-07

### Added

- `xrune` crate re-exports all `xrune-nexus` public API via `pub use xrune_nexus::*`

## [1.1.2] - 2026-05-07

### Added

- `DsRoot::get_context_attrs()` — expose all context attributes to Rune implementors
- `parent` remains required, other context attrs are optional extensions

## [1.1.1] - 2026-05-07

### Added

- Renamed crates: xwrapup → xrune (xrune-sigil, xrune-nexus, xrune-incant, xrune)
- GitHub repo renamed to W-Mai/xrune

## [1.1.0] - 2026-05-07

### Added

- `DsRune` trait — pluggable codegen interface with `inscribe_*` + `seal` methods
- `decipher` function — walks DsTree AST and invokes DsRune methods
- `DefaultRune` — default backend (println-based debug output)
- Parser unit tests (12 test cases including error reporting)
- Getter methods on AST nodes (`get_children`, `get_condition`, `get_iterable`, `get_variable`)
- xtask: ci/build/test/lint/bump/publish/release
- GitHub CI workflow

### Changed

- Parser fully decoupled from codegen (removed `DsTreeToTokens` from AST nodes)
- `proc_macros::ui!` now uses `DsRune`-based decipher internally
- Crate reorganization: `xrune_derive` / `xrune_parser` / `xrune_macros` / `xrune`
- Lint uses `+stable` to match CI environment

### Removed

- `DsTreeToTokens` trait
- `ui_code_gen` module (replaced by `DsRune` backends)
- `ds_traverse` module (replaced by `ds_rune::decipher`)
