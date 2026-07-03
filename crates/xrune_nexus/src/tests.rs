use quote::quote;
use syn::parse2;

use crate::ds_node::DsTree;
use crate::ds_node::ds_attr::{DsAttr, DsAttrs};
use crate::ds_node::node_enum::DsNode;

#[test]
fn parse_single_attr() {
    let tokens = quote! { width: 100 };
    let attr: DsAttr = syn::parse2(tokens).unwrap();
    assert_eq!(attr.name.as_ref().unwrap().to_string(), "width");
}

#[test]
fn parse_multiple_attrs() {
    let tokens = quote! { (width: 100, height: 200, color: "red") };
    let attrs: DsAttrs = syn::parse2(tokens).unwrap();
    assert_eq!(attrs.attrs.len(), 3);
    assert_eq!(attrs.attrs[0].name.as_ref().unwrap().to_string(), "width");
    assert_eq!(attrs.attrs[1].name.as_ref().unwrap().to_string(), "height");
    assert_eq!(attrs.attrs[2].name.as_ref().unwrap().to_string(), "color");
}

#[test]
fn parse_empty_attrs() {
    let tokens = quote! { () };
    let attrs: DsAttrs = syn::parse2(tokens).unwrap();
    assert_eq!(attrs.attrs.len(), 0);
}

#[test]
fn parse_no_parens_attrs() {
    let tokens = quote! { {} };
    let _tokens = tokens;
}

#[test]
fn parse_widget_node() {
    let tokens = quote! {
        button (text: "hello") {}
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Widget(w) => {
            assert_eq!(w.get_name().to_string(), "button");
            assert_eq!(w.get_attrs().attrs.len(), 1);
            assert_eq!(
                w.get_attrs().attrs[0].name.as_ref().unwrap().to_string(),
                "text"
            );
        }
        _ => panic!("Expected Widget node"),
    }
}

#[test]
fn parse_widget_no_attrs() {
    let tokens = quote! {
        container {}
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Widget(w) => {
            assert_eq!(w.get_name().to_string(), "container");
            assert_eq!(w.get_attrs().attrs.len(), 0);
        }
        _ => panic!("Expected Widget node"),
    }
}

#[test]
fn parse_nested_widgets() {
    let tokens = quote! {
        div (width: 100) {
            button (text: "ok") {}
            label (content: "hi") {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Widget(w) => {
            assert_eq!(w.get_name().to_string(), "div");
        }
        _ => panic!("Expected Widget"),
    }
    // Children count - need access to children field
    // Currently children is private, we'd need a getter
}

#[test]
fn parse_if_node() {
    let tokens = quote! {
        if show_footer {
            footer (height: 20) {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::If(_) => {} // OK
        _ => panic!("Expected If node"),
    }
}

#[test]
fn parse_walk_node() {
    let tokens = quote! {
        walk items with item {
            label (text: "x") {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Iter(_) => {} // OK
        _ => panic!("Expected Iter node"),
    }
}

#[test]
fn parse_expr_attr_value() {
    // Attribute values can be arbitrary expressions
    let tokens = quote! { height: 100 + A * 2 };
    let attr: DsAttr = syn::parse2(tokens).unwrap();
    assert_eq!(attr.name.as_ref().unwrap().to_string(), "height");
    // Value is a complex expression - just verify it parsed
}
#[test]
fn header_optional_no_prefix() {
    let tokens = quote! { div (width: 100) {} };
    let result = syn::parse2::<crate::ds_node::DsRoot>(tokens);
    assert!(result.is_ok(), "header is optional; runes fill parent");
    let root = result.unwrap();
    assert!(root.get_context_attrs().is_empty());
}

#[test]
fn parse_code_block_top_level() {
    let tokens = quote! { ${ let x = 1; } };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::CodeBlock(code) => {
            assert!(code.get_tokens().to_string().contains("let x = 1"));
        }
        other => panic!("expected CodeBlock, got {other:?}"),
    }
}

#[test]
fn parse_code_block_between_widgets() {
    let tokens = quote! {
        Root {
            Child {}
            ${ let pad = 12; }
            OtherChild {}
        }
    };
    let root: DsTree = parse2(tokens).unwrap();
    let kids = root.get_children();
    assert_eq!(kids.len(), 3);
    assert!(matches!(kids[1].borrow().get_node(), DsNode::CodeBlock(_)));
}

#[test]
fn parse_code_block_multi_statement() {
    let tokens = quote! {
        ${
            let x = 1;
            let y = 2;
            let z = x + y;
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::CodeBlock(code) => {
            let s = code.get_tokens().to_string();
            assert!(s.contains("let x = 1"));
            assert!(s.contains("let y = 2"));
            assert!(s.contains("let z = x + y"));
        }
        other => panic!("expected multi-statement CodeBlock, got {other:?}"),
    }
}

#[test]
fn parse_code_block_empty() {
    let tokens = quote! { ${ } };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::CodeBlock(code) => {
            assert!(code.get_tokens().is_empty());
        }
        other => panic!("expected empty CodeBlock, got {other:?}"),
    }
}

#[test]
fn parse_niche_with_body_widgets() {
    let tokens = quote! {
        @header {
            Text ("Untitled")
            Icon (name: "warn")
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Niche(n) => {
            assert_eq!(n.get_name().to_string(), "header");
        }
        other => panic!("expected Niche, got {other:?}"),
    }
    assert_eq!(tree.get_children().len(), 2);
}

#[test]
fn parse_nested_widget_with_niche_refs() {
    let tokens = quote! {
        Column (grow: 1.0) {
            View (bg_color: Primary) {
                @header
            }
            View (grow: 1.0) {
                @body
            }
            View {
                @footer
            }
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    let column_kids = tree.get_children();
    assert_eq!(column_kids.len(), 3);
    for child in column_kids {
        let borrowed = child.borrow();
        assert!(matches!(borrowed.get_node(), DsNode::Widget(_)));
        let grandkids = borrowed.get_children();
        assert_eq!(grandkids.len(), 1);
        assert!(matches!(grandkids[0].borrow().get_node(), DsNode::Niche(_)));
    }
}

#[test]
fn parse_code_block_inside_if_body() {
    let tokens = quote! {
        if cond {
            ${ let pad = 12; }
            Widget {}
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    assert!(matches!(tree.get_node(), DsNode::If(_)));
    let kids = tree.get_children();
    assert_eq!(kids.len(), 2);
    assert!(matches!(kids[0].borrow().get_node(), DsNode::CodeBlock(_)));
}

#[test]
fn parse_code_block_inside_match_arm() {
    let tokens = quote! {
        match state {
            State::Loading => {
                ${ let msg = "loading"; }
                Spinner()
            }
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    assert!(matches!(tree.get_node(), DsNode::Match(_)));
}

#[test]
fn parse_code_block_inside_niche_body() {
    let tokens = quote! {
        @header {
            ${ let title = "Default"; }
            Text (title)
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    assert!(matches!(tree.get_node(), DsNode::Niche(_)));
    let kids = tree.get_children();
    assert_eq!(kids.len(), 2);
    assert!(matches!(kids[0].borrow().get_node(), DsNode::CodeBlock(_)));
}

#[test]
fn parse_multiple_code_blocks_in_sequence() {
    let tokens = quote! {
        Widget {
            ${ let a = 1; }
            ${ let b = 2; }
            Child {}
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    let kids = tree.get_children();
    assert_eq!(kids.len(), 3);
    assert!(matches!(kids[0].borrow().get_node(), DsNode::CodeBlock(_)));
    assert!(matches!(kids[1].borrow().get_node(), DsNode::CodeBlock(_)));
    assert!(matches!(kids[2].borrow().get_node(), DsNode::Widget(_)));
}

#[test]
fn parse_code_block_with_closures_and_nested_braces() {
    let tokens = quote! {
        ${
            let f = |x: i32| { x + 1 };
            let r = if true { 1 } else { 2 };
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::CodeBlock(code) => {
            let s = code.get_tokens().to_string();
            assert!(s.contains("| x"));
            assert!(s.contains("if true"));
        }
        other => panic!("expected CodeBlock, got {other:?}"),
    }
}

#[test]
fn reactive_if_with_dollar_expr_not_confused_with_code_block() {
    let tokens = quote! {
        if ${ state == State::Ready } {
            Widget {}
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::If(if_node) => {
            assert!(
                if_node.is_reactive(),
                "`if ${{ expr }}` must resolve as reactive-if, not a top-level code block"
            );
        }
        other => panic!("expected reactive If, got {other:?}"),
    }
    assert_eq!(tree.get_children().len(), 1);
}

#[test]
fn reactive_walk_with_dollar_expr_not_confused_with_code_block() {
    let tokens = quote! {
        walk ${ items.iter() } with i {
            Row (index: i)
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Iter(iter) => {
            assert!(
                iter.is_reactive(),
                "reactive walk must not be misparsed as code block"
            );
        }
        other => panic!("expected reactive Iter, got {other:?}"),
    }
}

#[test]
fn reactive_match_with_dollar_expr_not_confused_with_code_block() {
    let tokens = quote! {
        match ${ state.get() } {
            State::Loading => { Spinner () }
            _ => { Content () }
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Match(m) => {
            assert!(
                m.is_reactive(),
                "reactive match must not be misparsed as code block"
            );
        }
        other => panic!("expected reactive Match, got {other:?}"),
    }
}

#[test]
fn code_block_and_reactive_dollar_coexist() {
    // Statement-position ${} → CodeBlock;
    // condition-position ${} → reactive prefix.
    let tokens = quote! {
        Widget {
            ${ let pad = 12; }
            if ${ visible.get() } {
                Child (padding: pad)
            }
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    let kids = tree.get_children();
    assert_eq!(kids.len(), 2);
    assert!(matches!(kids[0].borrow().get_node(), DsNode::CodeBlock(_)));
    match kids[1].borrow().get_node() {
        DsNode::If(if_node) => assert!(if_node.is_reactive()),
        other => panic!("expected reactive If as sibling of CodeBlock, got {other:?}"),
    }
}

#[test]
fn parse_niche_inside_walk_body() {
    let tokens = quote! {
        walk items with it {
            @slot { Row (data: it) }
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    assert!(matches!(tree.get_node(), DsNode::Iter(_)));
    let kids = tree.get_children();
    assert_eq!(kids.len(), 1);
    assert!(matches!(kids[0].borrow().get_node(), DsNode::Niche(_)));
}

#[test]
fn parse_code_block_inside_walk() {
    let tokens = quote! {
        walk items with i {
            ${ let key = i * 2; }
            Row (index: i)
        }
    };
    let tree: DsTree = parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Iter(_) => {}
        other => panic!("expected Iter, got {other:?}"),
    }
    let kids = tree.get_children();
    assert_eq!(kids.len(), 2);
    assert!(matches!(kids[0].borrow().get_node(), DsNode::CodeBlock(_)));
    assert!(matches!(kids[1].borrow().get_node(), DsNode::Widget(_)));
}

#[test]
fn root_no_header_produces_unit_parent() {
    let tokens = quote! { Widget (x: 1) {} };
    let root = parse2::<crate::ds_node::DsRoot>(tokens).unwrap();
    assert!(root.get_context_attrs().is_empty());
    let parent = root.get_parent();
    let s = quote!(#parent).to_string();
    assert_eq!(s, "()", "no header → parent expr is unit");
}

#[test]
fn root_header_without_parent_attr() {
    let tokens = quote! { :(world: w:) Widget {} };
    let root = parse2::<crate::ds_node::DsRoot>(tokens).unwrap();
    assert_eq!(root.get_context_attrs().len(), 1);
    let parent = root.get_parent();
    let s = quote!(#parent).to_string();
    assert_eq!(s, "()", "header without parent attr falls back to unit");
}

#[test]
fn header_present_without_parent_attr() {
    let tokens = quote! { :(foo: 123:) div {} };
    let result = syn::parse2::<crate::ds_node::DsRoot>(tokens);
    assert!(result.is_ok(), "unnamed parent attr is filled with unit");
    let root = result.unwrap();
    assert_eq!(root.get_context_attrs().len(), 1);
}

#[test]
fn parse_widget_no_braces() {
    let tokens = quote! { Image (path: "x.png") };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Widget(w) => {
            assert_eq!(w.get_name().to_string(), "Image");
            assert_eq!(w.get_attrs().attrs.len(), 1);
        }
        _ => panic!("Expected Widget"),
    }
    assert_eq!(tree.get_children().len(), 0);
}

#[test]
fn parse_widget_empty_braces_still_works() {
    let tokens = quote! { Image (path: "x.png") {} };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Widget(_) => {}
        _ => panic!("Expected Widget"),
    }
    assert_eq!(tree.get_children().len(), 0);
}

#[test]
fn error_if_without_body() {
    let tokens = quote! { if show_footer };
    let result = syn::parse2::<DsTree>(tokens);
    assert!(result.is_err());
}

#[test]
fn error_walk_without_body() {
    let tokens = quote! { walk items with x };
    let result = syn::parse2::<DsTree>(tokens);
    assert!(result.is_err());
}

#[test]
fn parse_niche_node() {
    let tokens = quote! {
        @header { Text (text: "hi") {} }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Niche(n) => assert_eq!(n.get_name().to_string(), "header"),
        _ => panic!("Expected Niche node"),
    }
    assert_eq!(tree.get_children().len(), 1);
}

#[test]
fn parse_niche_multiple_children() {
    let tokens = quote! {
        @body { Text (text: "a") {} Text (text: "b") {} }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Niche(_) => {}
        _ => panic!("Expected Niche"),
    }
    assert_eq!(tree.get_children().len(), 2);
}

#[test]
fn niche_without_body_is_a_bare_reference() {
    let tokens = quote! { @header };
    let tree = syn::parse2::<DsTree>(tokens).expect("bare @name is legal");
    match tree.get_node() {
        DsNode::Niche(n) => {
            assert_eq!(n.get_name().to_string(), "header");
            assert!(!n.is_declaration(), "single-@ is a fill site, not a decl");
        }
        other => panic!("expected Niche, got {other:?}"),
    }
    assert!(tree.get_children().is_empty());
}

#[test]
fn parse_niche_declaration_bare() {
    let tokens = quote! { @@header };
    let tree = syn::parse2::<DsTree>(tokens).expect("`@@name` parses");
    match tree.get_node() {
        DsNode::Niche(n) => {
            assert_eq!(n.get_name().to_string(), "header");
            assert!(n.is_declaration(), "double-@@ is a declaration");
        }
        other => panic!("expected Niche, got {other:?}"),
    }
    assert!(tree.get_children().is_empty());
}

#[test]
fn parse_niche_declaration_with_body() {
    let tokens = quote! {
        @@body { Text ("fallback") {} }
    };
    let tree = syn::parse2::<DsTree>(tokens).expect("`@@name { fallback }` parses");
    match tree.get_node() {
        DsNode::Niche(n) => {
            assert!(n.is_declaration());
            assert_eq!(n.get_name().to_string(), "body");
        }
        other => panic!("expected Niche, got {other:?}"),
    }
    assert_eq!(
        tree.get_children().len(),
        1,
        "fallback body child preserved"
    );
}

#[test]
fn niche_decl_and_fill_share_a_scope() {
    let tokens = quote! {
        Card () {
            @@header
            @body { Text ("hi") {} }
        }
    };
    let tree: DsTree = syn::parse2(tokens).expect("mixed decl + fill parses");
    let children = tree.get_children();
    assert_eq!(children.len(), 2, "two niche children");
    let (decl_is_declaration, fill_is_declaration) = {
        let a = children[0].borrow();
        let b = children[1].borrow();
        let d = match a.get_node() {
            DsNode::Niche(n) => n.is_declaration(),
            other => panic!("expected first child Niche, got {other:?}"),
        };
        let f = match b.get_node() {
            DsNode::Niche(n) => n.is_declaration(),
            other => panic!("expected second child Niche, got {other:?}"),
        };
        (d, f)
    };
    assert!(decl_is_declaration, "first child is @@decl");
    assert!(!fill_is_declaration, "second child is @fill");
}

#[test]
fn parse_match_node() {
    let tokens = quote! {
        match state {
            State::Loading => { Spinner () }
            State::Ready => { Content (text: "ok") }
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Match(m) => {
            assert_eq!(m.get_arms().len(), 2);
        }
        _ => panic!("Expected Match node"),
    }
}

#[test]
fn parse_match_with_binding() {
    let tokens = quote! {
        match state {
            State::Ready(d) => { Content (text: "x") }
            _ => { Empty () }
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Match(m) => {
            assert_eq!(m.get_arms().len(), 2);
            let arms = m.get_arms();
            assert_eq!(arms[0].get_children().len(), 1);
            assert_eq!(arms[1].get_children().len(), 1);
        }
        _ => panic!("Expected Match"),
    }
}

#[test]
fn error_match_without_body() {
    let tokens = quote! { match x };
    let result = syn::parse2::<DsTree>(tokens);
    assert!(result.is_err());
}

#[test]
fn root_header_accepts_commas() {
    let tokens = quote! {
        :(
            parent: root,
            world: w,
        :)
        Foo {}
    };
    let result = syn::parse2::<crate::ds_node::DsRoot>(tokens);
    assert!(result.is_ok(), "trailing-comma form must parse");
}

#[test]
fn root_header_accepts_no_commas() {
    let tokens = quote! {
        :(
            parent: root
            world: w
        :)
        Foo {}
    };
    let result = syn::parse2::<crate::ds_node::DsRoot>(tokens);
    assert!(result.is_ok(), "no-comma form must still parse");
}

#[test]
fn form_c_on_after_attrs_before_body() {
    let tokens = quote! {
        Slider (min: 0, max: 100)
            on Tap { fire_a() }
            on ValueChanged(2) { fire_b() }
            {}
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    let widget = match tree.get_node() {
        DsNode::Widget(w) => w,
        _ => panic!("Expected Widget node"),
    };
    let handlers = widget.get_on_handlers();
    assert_eq!(handlers.len(), 2, "two on handlers in Form C");
    assert_eq!(handlers[0].get_name().to_string(), "Tap");
    assert_eq!(handlers[1].get_name().to_string(), "ValueChanged");
    assert_eq!(handlers[1].get_args().len(), 1);
}

#[test]
fn form_c_qualified_event_name() {
    let tokens = quote! {
        Slider (min: 0, max: 100)
            on Slider::ValueChanged { persist(*new) }
            {}
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    let widget = match tree.get_node() {
        DsNode::Widget(w) => w,
        _ => panic!("Expected Widget"),
    };
    let handlers = widget.get_on_handlers();
    assert_eq!(handlers.len(), 1);
    assert_eq!(handlers[0].get_qualifier().unwrap().to_string(), "Slider",);
    assert_eq!(handlers[0].get_name().to_string(), "ValueChanged");
}

#[test]
fn form_b_on_after_widget_body_at_root() {
    let tokens = quote! {
        View () {} on Tap { fire() }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    let widget = match tree.get_node() {
        DsNode::Widget(w) => w,
        _ => panic!("Expected Widget"),
    };
    let handlers = widget.get_on_handlers();
    assert_eq!(
        handlers.len(),
        1,
        "Form B: trailing on attaches to root widget"
    );
    assert_eq!(handlers[0].get_name().to_string(), "Tap");
}

#[test]
fn form_b_chained_modifiers() {
    let tokens = quote! {
        Button (text: "x") {}
            on Tap { fire_a() }
            on LongPress { fire_b() }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    let widget = match tree.get_node() {
        DsNode::Widget(w) => w,
        _ => panic!("Expected Widget"),
    };
    let handlers = widget.get_on_handlers();
    assert_eq!(
        handlers.len(),
        2,
        "Form B chain: two on modifiers attach to same widget"
    );
    assert_eq!(handlers[0].get_name().to_string(), "Tap");
    assert_eq!(handlers[1].get_name().to_string(), "LongPress");
}

#[test]
fn form_b_inside_nested_body_attaches_to_previous_sibling() {
    let tokens = quote! {
        Container () {
            Child () {} on Tap { handle() }
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    let container = match tree.get_node() {
        DsNode::Widget(w) => w,
        _ => panic!("Expected Container Widget"),
    };
    assert_eq!(
        container.get_on_handlers().len(),
        0,
        "Container itself has no on handler"
    );
    let children = tree.get_children();
    assert_eq!(children.len(), 1, "Container has one child Child");
    let child_borrow = children[0].borrow();
    let child = match child_borrow.get_node() {
        DsNode::Widget(w) => w,
        _ => panic!("Expected Child Widget"),
    };
    assert_eq!(
        child.get_on_handlers().len(),
        1,
        "on attaches to nearest preceding sibling Child, not Container",
    );
}

#[test]
fn form_b_plus_c_mixed_on_same_widget() {
    let tokens = quote! {
        View ()
            on Tap { a() }
            {}
            on LongPress { b() }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    let widget = match tree.get_node() {
        DsNode::Widget(w) => w,
        _ => panic!("Expected Widget"),
    };
    let handlers = widget.get_on_handlers();
    assert_eq!(handlers.len(), 2, "Form B + Form C handlers all stick");
    assert_eq!(handlers[0].get_name().to_string(), "Tap");
    assert_eq!(handlers[1].get_name().to_string(), "LongPress");
}

#[test]
fn error_form_a_on_inside_body_rejected() {
    let tokens = quote! {
        View () {
            on Tap { fire() }
        }
    };
    let result = syn::parse2::<DsTree>(tokens);
    assert!(
        result.is_err(),
        "Form A (on nested inside body without preceding sibling) must fail"
    );
}

#[test]
fn error_on_at_root_without_widget() {
    let tokens = quote! { on Tap { fire() } };
    let result = syn::parse2::<DsTree>(tokens);
    assert!(
        result.is_err(),
        "bare `on` with no preceding widget must fail"
    );
}

#[test]
fn error_form_c_no_body() {
    let tokens = quote! {
        View () on Tap { fire() }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    let widget = match tree.get_node() {
        DsNode::Widget(w) => w,
        _ => panic!("Expected Widget"),
    };
    assert_eq!(widget.get_on_handlers().len(), 1);
    assert_eq!(
        tree.get_children().len(),
        0,
        "Form C without children body parses as widget with empty body"
    );
}

#[test]
fn error_on_no_braces_in_handler() {
    let tokens = quote! {
        View () on Tap call_me() {}
    };
    let result = syn::parse2::<DsTree>(tokens);
    assert!(result.is_err(), "on EventKind without {{}} body must fail",);
}

#[test]
fn error_on_multi_qualifier_segment() {
    let tokens = quote! {
        View () on Foo::Bar::Baz { x() } {}
    };
    let result = syn::parse2::<DsTree>(tokens);
    assert!(result.is_err(), "multi-segment qualifier must fail",);
}

#[test]
fn parse_if_else() {
    let tokens = quote! {
        if cond {
            a (height: 10) {}
        } else {
            b (height: 20) {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    let else_b = tree.get_else_branch().expect("else branch present");
    assert!(
        matches!(else_b.borrow().get_node(), DsNode::Else),
        "terminal else is an Else node",
    );
}

#[test]
fn parse_if_elif_else_chain() {
    let tokens = quote! {
        if a {
            x (height: 10) {}
        } elif b {
            y (height: 10) {}
        } else {
            z (height: 10) {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    // elif is a nested If node carrying its own else (the terminal else)
    let elif = tree.get_else_branch().expect("elif branch");
    let elif_b = elif.borrow();
    assert!(
        matches!(elif_b.get_node(), DsNode::If(_)),
        "elif is an If node"
    );
    let tail = elif_b.get_else_branch().expect("elif's else");
    assert!(
        matches!(tail.borrow().get_node(), DsNode::Else),
        "chain ends in Else"
    );
}

#[test]
fn parse_reactive_if() {
    let tokens = quote! {
        if $cond {
            a (height: 10) {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::If(n) => assert!(n.is_reactive(), "$ marks the if reactive"),
        _ => panic!("expected If node"),
    }
}

#[test]
fn parse_reactive_walk() {
    let tokens = quote! {
        walk $items with item {
            a (height: 10) {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Iter(n) => assert!(n.is_reactive(), "$ marks the walk reactive"),
        _ => panic!("expected Iter node"),
    }
}

#[test]
fn parse_walk_by_key() {
    let tokens = quote! {
        walk $items with item by item.id {
            a (height: 10) {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Iter(n) => {
            assert!(n.get_key().is_some(), "by clause captures a key expr");
        }
        _ => panic!("expected Iter node"),
    }
}

#[test]
fn parse_walk_without_by_has_no_key() {
    let tokens = quote! {
        walk items with item {
            a (height: 10) {}
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Iter(n) => assert!(n.get_key().is_none(), "no by clause -> no key"),
        _ => panic!("expected Iter node"),
    }
}

#[test]
fn parse_reactive_match() {
    let tokens = quote! {
        match $state {
            0 => { a (height: 10) {} }
            _ => { b (height: 10) {} }
        }
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Match(n) => assert!(n.is_reactive(), "$ marks the match reactive"),
        _ => panic!("expected Match node"),
    }
}

#[test]
fn parse_reactive_attr() {
    let tokens = quote! {
        a (bg_color: $signal, width: ${ pick(x.get()) }, height: 10) {}
    };
    let tree: DsTree = syn::parse2(tokens).unwrap();
    match tree.get_node() {
        DsNode::Widget(w) => {
            let attrs = &w.get_attrs().attrs;
            assert!(attrs[0].reactive, "$path attr is reactive");
            assert!(attrs[1].reactive, "${{block}} attr is reactive");
            assert!(!attrs[2].reactive, "bare attr is not reactive");
        }
        _ => panic!("expected Widget node"),
    }
}
