<p align="center">
  <img src="docs/landing/logo.svg" alt="xrune — The Grimoire" width="440">
</p>

<p align="center">
  <a href="https://github.com/W-Mai/xrune/actions"><img src="https://github.com/W-Mai/xrune/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://crates.io/crates/xrune"><img src="https://img.shields.io/crates/v/xrune.svg" alt="crates.io"></a>
  <a href="https://docs.rs/xrune"><img src="https://docs.rs/xrune/badge.svg" alt="docs.rs"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
</p>

<p align="center">
  <b><a href="https://xrune.to01.icu/">The Grimoire</a></b> ·
  <a href="https://xrune.to01.icu/book/">English docs</a> ·
  <a href="https://xrune.to01.icu/book/zh-CN/">中文文档</a> ·
  <a href="README.zh-CN.md">中文 README</a>
</p>

A declarative UI DSL proc macro framework with pluggable code generation backends.

## Features

- Declarative widget tree syntax with nested children
- Attribute expressions (any valid Rust expression as value)
- Enchants — attach arbitrary data to nodes via `[expr, ...]` syntax
- Context area with arbitrary key-value pairs
- Conditional rendering (`if` / `elif` / `else`)
- Iteration (`walk ... with ...`, optional `by <key>` for identity-based reconciliation)
- Reactive `$` sigil on control flow and attributes (`if $cond`, `match $expr`, `walk $items`, `attr: $signal` / `attr: ${ expr }`)
- Pluggable codegen via `DsRune` trait — bring your own backend

## Syntax

```rust
use xrune::ui;

fn app(parent: i32) {
    ui! {
        // Context area: arbitrary key-value pairs (each attr on its own line)
        :(
            parent: parent
            world: &mut world
        :)

        // Widget with attributes
        container (width: 480, height: 320, color: "dark") {
            header (height: 40, text: "Hello") {}

            row (direction: "horizontal") {
                button (text: "OK", grow: 1.0) {}
                button (text: "Cancel", grow: 1.0) {}
            }

            // Enchants: attach data to a node
            physics_obj (x: 100, y: 200) [
                Velocity { vx: 1, vy: 0 },
                Collider::circle(10),
            ] {}

            // Iteration
            walk items.iter() with item {
                label (text: item.name) {}
            }

            // Conditional
            if show_footer {
                footer (height: 20) {}
            }
        }
    }
}
```

## Architecture

```mermaid
block-beta
    columns 6

    A["ui! { ... }"]:6
    B["xrune-incant"]:6
    C["xrune-nexus"]:6
    D["DefaultRune"]:2 E["YourRune"]:2 F["…Rune"]:2
    G["xrune-sigil"]:6
    H["DsRoot"]:2 I["DsWidget"]:2 J["DsIf / DsIter"]:2

    style A fill:#dbeafe,stroke:#2563eb,color:#1e3a5f
    style B fill:#dbeafe,stroke:#2563eb,color:#1e3a5f
    style C fill:#dcfce7,stroke:#16a34a,color:#14532d
    style D fill:#fef3c7,stroke:#d97706,color:#78350f
    style E fill:#fef3c7,stroke:#d97706,color:#78350f
    style F fill:#fef3c7,stroke:#d97706,color:#78350f
    style G fill:#ede9fe,stroke:#7c3aed,color:#3b0764
    style H fill:#ede9fe,stroke:#7c3aed,color:#3b0764
    style I fill:#ede9fe,stroke:#7c3aed,color:#3b0764
    style J fill:#ede9fe,stroke:#7c3aed,color:#3b0764
```

## Crates

| Crate | Description |
|-------|-------------|
| [`xrune`](https://crates.io/crates/xrune) | Main entry — re-exports everything |
| [`xrune-nexus`](https://crates.io/crates/xrune-nexus) | Core: AST + DsRune trait + decipher |
| [`xrune-incant`](https://crates.io/crates/xrune-incant) | Proc macro: `ui!` invocation |
| [`xrune-sigil`](https://crates.io/crates/xrune-sigil) | Derive macro: `DsRef` |
| [`xrune-fmt`](https://crates.io/crates/xrune-fmt) | CLI formatter for `ui! { … }` blocks |

## Custom Backend

Implement `DsRune` to generate your own code:

```rust
use xrune::ds_rune::DsRune;
use xrune::ds_node::ds_attr::DsAttr;
use xrune::ds_node::ds_on::DsOn;
use xrune::ds_node::ds_match::DsMatchArm;
use xrune::ds_node::DsTreeRef;

struct MyRune { /* ... */ }

impl DsRune for MyRune {
    fn inscribe_root(&mut self, parent_expr: &syn::Expr) { /* ... */ }

    fn inscribe_widget(
        &mut self,
        name: &syn::Ident,
        attrs: &[DsAttr],
        enchants: &[syn::Expr],   // [expr, ...] attached data
        on_handlers: &[DsOn],     // every `on EventKind` clause on this widget
        children: &[DsTreeRef],
    ) { /* ... */ }

    fn inscribe_if(
        &mut self,
        condition: &syn::Expr,
        reactive: bool,             // `if $cond` sets this
        children: &[DsTreeRef],
        else_branch: Option<&DsTreeRef>,  // `elif` (nested If) / `else` (Else node)
    ) { /* ... */ }

    fn inscribe_iter(
        &mut self,
        iterable: &syn::Expr,
        variable: &syn::Ident,
        reactive: bool,                 // `walk $items` sets this
        key: Option<&syn::Expr>,        // `walk ... by item.id` sets this
        children: &[DsTreeRef],
    ) { /* ... */ }

    fn inscribe_niche(&mut self, name: &syn::Ident, children: &[DsTreeRef]) { /* ... */ }

    fn inscribe_match(
        &mut self,
        scrutinee: &syn::Expr,
        reactive: bool,             // `match $expr` sets this
        arms: &[DsMatchArm],
    ) { /* ... */ }

    fn seal(self) -> proc_macro2::TokenStream { /* ... */ }
}
```

## Context Area

The `:( … :)` block passes arbitrary context to the Rune implementation. Each attribute sits on its own line. The `parent` key is required; all others are optional and Rune-specific.

```rust
ui! {
    :(
        parent: root_entity
        world: &mut app.world
        theme: Theme::Dark
    :)
    // ...
}
```

## License

MIT
