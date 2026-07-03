use quote::ToTokens;
use xrune_nexus::ds_node::ds_attr::DsAttr;
use xrune_nexus::ds_node::node_enum::DsNode;
use xrune_nexus::ds_node::{DsRoot, DsTreeRef};

/// Format xrune DSL content (inside ui! { ... }) using the real parser
pub fn format_dsl(input: &str, base_indent: &str) -> Option<String> {
    let tokens: proc_macro2::TokenStream = input.parse().ok()?;
    let root: DsRoot = syn::parse2(tokens).ok()?;

    let mut out = String::new();
    let indent1 = format!("{base_indent}    ");

    let attrs = root.get_context_attrs();
    if !attrs.is_empty() {
        out.push_str(&indent1);
        out.push_str(":(\n");
        let indent2 = format!("{indent1}    ");
        for attr in attrs {
            out.push_str(&indent2);
            if let Some(n) = &attr.name {
                out.push_str(&n.to_string());
                out.push_str(": ");
            }
            out.push_str(&fmt_expr(&attr.value));
            out.push('\n');
        }
        out.push_str(&indent1);
        out.push_str(":)\n\n");
    }

    let content = root.get_content();
    format_tree(&content, &indent1, &mut out);

    Some(out)
}

fn format_tree(tree: &DsTreeRef, indent: &str, out: &mut String) {
    let borrowed = tree.borrow();
    let child_indent = format!("{indent}    ");

    match borrowed.get_node() {
        DsNode::Root(_) => {
            for child in borrowed.get_children() {
                format_tree(child, indent, out);
            }
        }
        DsNode::Widget(widget) => {
            out.push_str(indent);
            let name_str = widget.get_name().to_string();
            out.push_str(&name_str);
            out.push(' ');

            // Attributes
            format_attrs(&widget.get_attrs().attrs, indent, name_str.len(), out);

            // Enchants
            let enchants = widget.get_enchants();
            if !enchants.is_empty() {
                out.push_str(" [\n");
                for enchant in enchants {
                    out.push_str(&child_indent);
                    out.push_str(&fmt_expr_indented(enchant, &child_indent));
                    out.push_str(",\n");
                }
                out.push_str(indent);
                out.push(']');
            }

            for on in widget.get_on_handlers() {
                out.push(' ');
                out.push_str("on ");
                if let Some(q) = on.get_qualifier() {
                    out.push_str(&q.to_string());
                    out.push_str("::");
                }
                out.push_str(&on.get_name().to_string());
                let args = on.get_args();
                if !args.is_empty() {
                    out.push('(');
                    for (i, a) in args.iter().enumerate() {
                        if i > 0 {
                            out.push_str(", ");
                        }
                        out.push_str(&fmt_expr_indented(a, indent));
                    }
                    out.push(')');
                }
                if let Some(body) = on.get_body() {
                    out.push(' ');
                    out.push_str(&fmt_block(body, indent));
                }
            }

            let children = borrowed.get_children();
            let has_on = !widget.get_on_handlers().is_empty();
            if children.is_empty() {
                out.push('\n');
            } else if has_on {
                out.push('\n');
                out.push_str(indent);
                out.push_str("{\n");
                for child in children {
                    format_tree(child, &child_indent, out);
                }
                out.push_str(indent);
                out.push_str("}\n");
            } else {
                out.push_str(" {\n");
                for child in children {
                    format_tree(child, &child_indent, out);
                }
                out.push_str(indent);
                out.push_str("}\n");
            }
        }
        DsNode::If(if_node) => {
            out.push_str(indent);
            out.push_str("if ");
            if if_node.is_reactive() {
                out.push('$');
            }
            out.push_str(&fmt_expr_indented(if_node.get_condition(), indent));
            out.push_str(" {\n");
            for child in borrowed.get_children() {
                format_tree(child, &child_indent, out);
            }
            out.push_str(indent);
            out.push('}');
            format_else_branch(borrowed.get_else_branch(), indent, out);
            out.push('\n');
        }
        DsNode::Else => {
            for child in borrowed.get_children() {
                format_tree(child, &child_indent, out);
            }
        }
        DsNode::Iter(iter_node) => {
            out.push_str(indent);
            out.push_str("walk ");
            if iter_node.is_reactive() {
                out.push('$');
            }
            out.push_str(&fmt_expr_indented(iter_node.get_iterable(), indent));
            out.push_str(" with ");
            out.push_str(&iter_node.get_variable().to_string());
            if let Some(key) = iter_node.get_key() {
                out.push_str(" by ");
                out.push_str(&fmt_expr_indented(key, indent));
            }
            out.push_str(" {\n");
            for child in borrowed.get_children() {
                format_tree(child, &child_indent, out);
            }
            out.push_str(indent);
            out.push_str("}\n");
        }
        DsNode::Niche(niche_node) => {
            let kids = borrowed.get_children();
            out.push_str(indent);
            out.push_str(if niche_node.is_declaration() {
                "@@"
            } else {
                "@"
            });
            out.push_str(&niche_node.get_name().to_string());
            if kids.is_empty() {
                out.push('\n');
            } else {
                out.push_str(" {\n");
                for child in kids {
                    format_tree(child, &child_indent, out);
                }
                out.push_str(indent);
                out.push_str("}\n");
            }
        }
        DsNode::Match(match_node) => {
            out.push_str(indent);
            out.push_str("match ");
            if match_node.is_reactive() {
                out.push('$');
            }
            out.push_str(&fmt_expr_indented(match_node.get_scrutinee(), indent));
            out.push_str(" {\n");
            let arm_indent = format!("{child_indent}    ");
            for arm in match_node.get_arms() {
                let pat = arm.get_pat();
                out.push_str(&child_indent);
                out.push_str(&quote::quote!(#pat).to_string());
                out.push_str(" => {\n");
                for child in arm.get_children() {
                    format_tree(child, &arm_indent, out);
                }
                out.push_str(&child_indent);
                out.push_str("}\n");
            }
            out.push_str(indent);
            out.push_str("}\n");
        }
        DsNode::CodeBlock(code) => {
            let formatted = fmt_code_block(code.get_tokens(), indent);
            out.push_str(indent);
            out.push('$');
            out.push_str(&formatted);
            out.push('\n');
        }
    }
}

const MAX_LINE_WIDTH: usize = 100;

fn format_attrs(attrs: &[DsAttr], indent: &str, name_len: usize, out: &mut String) {
    if attrs.is_empty() {
        out.push_str("()");
        return;
    }

    // Check if original was multiline (first attr and last attr on different lines)
    let first_line = attrs
        .first()
        .map(|a| match &a.name {
            Some(n) => n.span().start().line,
            None => syn::spanned::Spanned::span(&a.value).start().line,
        })
        .unwrap_or(0);
    let last_line = attrs
        .last()
        .map(|a| {
            a.value
                .to_token_stream()
                .into_iter()
                .last()
                .map(|t| t.span().end().line)
                .unwrap_or(first_line)
        })
        .unwrap_or(first_line);
    let was_multiline = last_line > first_line;

    // Build all attr strings
    let attr_indent = format!("{indent}    ");
    let attr_strs: Vec<String> = attrs
        .iter()
        .map(|attr| {
            let sigil = if attr.reactive { "$" } else { "" };
            let value = format!("{sigil}{}", fmt_expr_indented(&attr.value, &attr_indent));
            match &attr.name {
                Some(n) => format!("{n}: {value}"),
                None => value,
            }
        })
        .collect();

    let single_line = attr_strs.join(", ");
    let total_len = indent.len() + name_len + 1 + single_line.len() + 1 + 3;

    // Use multiline if: original was multiline OR exceeds max width
    if was_multiline || total_len > MAX_LINE_WIDTH {
        let attr_indent = format!("{indent}    ");
        out.push_str("(\n");
        for (i, s) in attr_strs.iter().enumerate() {
            out.push_str(&attr_indent);
            out.push_str(s);
            if i + 1 < attr_strs.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str(indent);
        out.push(')');
    } else {
        out.push('(');
        out.push_str(&single_line);
        out.push(')');
    }
}

/// Append an `if`'s `elif`/`else` tail after its closing brace.
fn format_else_branch(branch: Option<&DsTreeRef>, indent: &str, out: &mut String) {
    let Some(branch) = branch else {
        return;
    };
    let b = branch.borrow();
    let child_indent = format!("{indent}    ");
    match b.get_node() {
        DsNode::If(if_node) => {
            out.push_str(" elif ");
            if if_node.is_reactive() {
                out.push('$');
            }
            out.push_str(&fmt_expr_indented(if_node.get_condition(), indent));
            out.push_str(" {\n");
            for child in b.get_children() {
                format_tree(child, &child_indent, out);
            }
            out.push_str(indent);
            out.push('}');
            format_else_branch(b.get_else_branch(), indent, out);
        }
        _ => {
            out.push_str(" else {\n");
            for child in b.get_children() {
                format_tree(child, &child_indent, out);
            }
            out.push_str(indent);
            out.push('}');
        }
    }
}

fn fmt_expr(expr: &syn::Expr) -> String {
    fmt_expr_indented(expr, "")
}

/// Format a syn::Expr with re-indentation for multi-line output
fn fmt_expr_indented(expr: &syn::Expr, indent: &str) -> String {
    // A `${ block }` value. A single-statement block whose content fits on one
    // line stays inline (`{ expr }`); anything longer formats as a real block
    // so its body sits one level under the `{`.
    if let syn::Expr::Block(b) = expr {
        if let [syn::Stmt::Expr(inner, None)] = b.block.stmts.as_slice() {
            let one_line = fmt_expr_indented(inner, indent);
            if !one_line.contains('\n') {
                return format!("{{ {one_line} }}");
            }
        }
        return fmt_block(&b.block, indent);
    }

    let tokens = quote::quote!(#expr);
    // Wrap in a fn so prettyplease indents the expression's continuation lines
    // by a clean 4 spaces per level (the `const _ = { let _ = … }` wrapper adds
    // an extra level, which over-indents nested struct/call literals).
    let code = format!("fn __xrune_fmt_expr_wrapper() {{ {tokens} }}");
    let Ok(file) = syn::parse_str::<syn::File>(&code) else {
        return tokens.to_string();
    };
    let formatted = prettyplease::unparse(&file);
    let Some(open) = formatted.find('{') else {
        return tokens.to_string();
    };
    let Some(close) = formatted.rfind('}') else {
        return tokens.to_string();
    };
    if close <= open {
        return tokens.to_string();
    }
    let inner = formatted[open + 1..close].trim_matches('\n');
    let lines: Vec<&str> = inner.lines().map(|l| l.trim_end()).collect();

    if lines.len() <= 1 || indent.is_empty() {
        return lines.join("\n").trim_start().to_string();
    }

    // Line 0 stays bare — the caller already wrote `indent` at the insertion
    // point (e.g. after `text: ${` or `walk `).
    rebase(&lines, indent, true)
}

/// Re-indent prettyplease body lines: shift the whole block to `indent` while
/// keeping prettyplease's exact relative nesting (which is NOT a uniform 4-step
/// ladder). Subtracts the minimum lead, then re-prefixes with `indent`.
/// `line0_bare`: line 0 gets no prefix (caller pre-wrote indent) when true.
fn rebase(lines: &[&str], indent: &str, line0_bare: bool) -> String {
    let base = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if line.is_empty() {
                return String::new();
            }
            let lead = line.len() - line.trim_start().len();
            let body = &line[lead..];
            if i == 0 && line0_bare {
                body.to_string()
            } else {
                format!("{indent}{}{body}", " ".repeat(lead - base))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn fmt_block(block: &syn::Block, indent: &str) -> String {
    let tokens = quote::quote!(#block);
    fmt_wrapped_body(&tokens, indent).unwrap_or_else(|| tokens.to_string())
}

fn fmt_code_block(tokens: &proc_macro2::TokenStream, indent: &str) -> String {
    let wrapped = quote::quote!({ #tokens });
    fmt_wrapped_body(&wrapped, indent).unwrap_or_else(|| format!("{{ {tokens} }}"))
}

fn fmt_wrapped_body(brace_tokens: &proc_macro2::TokenStream, indent: &str) -> Option<String> {
    let code = format!("fn __xrune_fmt_block_wrapper() {brace_tokens}");
    let file = syn::parse_str::<syn::File>(&code).ok()?;
    let formatted = prettyplease::unparse(&file);
    let open = formatted.find('{')?;
    let close = formatted.rfind('}')?;
    if close <= open {
        return None;
    }
    let inner = formatted[open + 1..close].trim_matches('\n');
    let lines: Vec<&str> = inner.lines().map(|l| l.trim_end()).collect();
    let body_indent = format!("{indent}    ");
    let body = rebase(&lines, &body_indent, false);
    if body.is_empty() {
        Some("{ }".to_string())
    } else if !body.contains('\n') {
        let single = body.trim();
        Some(format!("{{ {single} }}"))
    } else {
        Some(format!("{{\n{body}\n{indent}}}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(s: &str) -> String {
        format_dsl(s, "").unwrap()
    }

    const CTX: &str = "
:(
parent: parent
world: world
:)

";

    #[test]
    fn empty_header_produces_no_context_block() {
        let out = fmt("Widget {}");
        assert!(
            !out.contains(":("),
            "no context attrs → no `:( :)` header. got:\n{out}"
        );
    }

    #[test]
    fn multiline_code_block_preserves_lines() {
        let out = fmt(&format!(
            "{CTX}Widget {{ ${{
                let a = 1;
                let b = 2;
            }} @body }}"
        ));
        assert!(
            out.contains("${\n") && out.contains("let a = 1;") && out.contains("let b = 2;"),
            "multi-line ${{...}} keeps its structure. got:\n{out}"
        );
    }

    #[test]
    fn bare_niche_reference_stays_bare() {
        let out = fmt(&format!("{CTX}Card {{ @header @body }}"));
        assert!(
            out.contains("@header\n") && !out.contains("@header {"),
            "bare @name should not gain a body block. got:\n{out}"
        );
    }

    #[test]
    fn niche_with_body_keeps_braces() {
        let out = fmt(&format!("{CTX}Card {{ @header {{ Text (\"hi\") }} }}"));
        assert!(
            out.contains("@header {"),
            "@name with body keeps the block. got:\n{out}"
        );
    }

    #[test]
    fn niche_declaration_bare_prints_double_at() {
        let out = fmt(&format!("{CTX}Card {{ @@header @@body }}"));
        assert!(
            out.contains("@@header\n") && out.contains("@@body\n"),
            "@@decl should emit `@@name` bare, not `@name` or `@@name {{}}`. got:\n{out}"
        );
        assert!(
            !out.contains("@@header {"),
            "bare @@decl must not gain empty braces. got:\n{out}"
        );
    }

    #[test]
    fn niche_declaration_with_fallback_keeps_braces() {
        let out = fmt(&format!(
            "{CTX}Card {{ @@body {{ Text (\"fallback\") {{}} }} }}"
        ));
        assert!(
            out.contains("@@body {"),
            "@@decl with fallback body keeps the block. got:\n{out}"
        );
    }

    #[test]
    fn childless_node_omits_braces() {
        let out = fmt(&format!(
            "{CTX}Text (\"hi\") {{}}
"
        ));
        assert!(
            out.contains(
                "Text (\"hi\")
"
            ),
            "childless should drop braces, got:
{out}"
        );
        assert!(
            !out.contains("Text (\"hi\") {}"),
            "should not emit empty braces, got:
{out}"
        );
    }

    #[test]
    fn node_with_children_keeps_braces() {
        let out = fmt(&format!(
            "{CTX}Column (grow: 1.0) {{ Text (\"x\") {{}} }}
"
        ));
        assert!(
            out.contains(
                "Column (grow: 1.0) {
"
            ),
            "got:
{out}"
        );
        assert!(
            out.contains(
                "    }
"
            ),
            "closing brace, got:
{out}"
        );
    }

    #[test]
    fn on_handler_childless_no_trailing_braces() {
        let out = fmt(&format!(
            "{CTX}View (a: 1.0) on Tap {{ foo(); }} {{}}
"
        ));
        // on-handler present, no children -> no trailing {}
        assert!(
            !out.contains("} {}"),
            "on-handler childless should not emit trailing braces, got:
{out}"
        );
    }

    #[test]
    fn on_handler_with_children_indent_aligned() {
        let out = fmt(&format!(
            "{CTX}View (a: 1.0) on Tap {{ foo(); }} {{ Text (\"d\") {{}} }}
"
        ));
        // children { must NOT be "} {" glued; must align under the widget indent
        assert!(
            !out.contains(
                "} {
"
            ),
            "children brace should not glue to on-handler close, got:
{out}"
        );
        // child Text is childless -> no braces
        assert!(
            out.contains(
                "Text (\"d\")
"
            ),
            "nested childless drops braces, got:
{out}"
        );
    }

    #[test]
    fn on_handler_follows_enchant_close_same_line() {
        let out = fmt(&format!(
            "{CTX}View (a: 1.0) [ X ] on Tap {{ foo(); }} {{}}
"
        ));
        assert!(
            out.contains("] on Tap {"),
            "on must follow ] on same line, got:
{out}"
        );
        assert!(
            !out.contains(
                "]
                on Tap"
            ),
            "on must not start a new indented line, got:
{out}"
        );
    }

    #[test]
    fn on_handler_follows_attr_close_same_line() {
        let out = fmt(&format!(
            "{CTX}View (a: 1.0) on Tap {{ foo(); }} {{}}
"
        ));
        assert!(
            out.contains("View (a: 1.0) on Tap {"),
            "on must follow ) on same line, got:
{out}"
        );
    }

    #[test]
    fn multi_on_handlers_chain_same_line() {
        let out = fmt(&format!(
            "{CTX}View (a: 1.0) on DragMove {{ a(); }} on DragEnd {{ b(); }} {{}}
"
        ));
        assert!(
            out.contains("} on DragEnd {"),
            "second on must follow first close same line, got:
{out}"
        );
    }

    #[test]
    fn reactive_if_keeps_dollar() {
        let out = fmt(&format!("{CTX}if $cond {{ Text (\"x\") {{}} }}\n"));
        assert!(out.contains("if $cond"), "must keep $ sigil, got:\n{out}");
    }

    #[test]
    fn reactive_walk_and_match_keep_dollar() {
        let walk = fmt(&format!(
            "{CTX}walk $items with item {{ Text (\"x\") {{}} }}\n"
        ));
        assert!(walk.contains("walk $items"), "walk keeps $, got:\n{walk}");
        let m = fmt(&format!(
            "{CTX}match $state {{ 0 => {{ Text (\"x\") {{}} }} _ => {{ Text (\"y\") {{}} }} }}\n"
        ));
        assert!(m.contains("match $state"), "match keeps $, got:\n{m}");
    }

    #[test]
    fn if_elif_else_roundtrip() {
        let out = fmt(&format!(
            "{CTX}if a {{ Text (\"x\") {{}} }} elif b {{ Text (\"y\") {{}} }} else {{ Text (\"z\") {{}} }}\n"
        ));
        assert!(
            out.contains("elif b"),
            "keeps elif (not else if), got:\n{out}"
        );
        assert!(out.contains("} else {"), "keeps terminal else, got:\n{out}");
    }

    #[test]
    fn reactive_attr_keeps_dollar() {
        let path = fmt(&format!("{CTX}View (bg_color: $signal) {{}}\n"));
        assert!(
            path.contains("bg_color: $signal"),
            "bare $path attr keeps $, got:\n{path}"
        );
        let block = fmt(&format!(
            "{CTX}View (bg_color: ${{ pick(x.get()) }}) {{}}\n"
        ));
        assert!(
            block.contains("bg_color: ${"),
            "${{block}} attr keeps $ with no gap, got:\n{block}"
        );
    }

    #[test]
    fn reactive_walk_block_keeps_dollar() {
        let out = fmt(&format!(
            "{CTX}walk ${{ rows.iter() }} with item {{ Text (\"x\") {{}} }}\n"
        ));
        assert!(
            out.contains("walk ${"),
            "walk ${{block}} keeps $ with no gap, got:\n{out}"
        );
    }

    #[test]
    fn reactive_block_attr_preserves_nesting() {
        let out = fmt(&format!(
            "{CTX}Text (text: ${{ format!(\"value: {{}}   changes: {{}}   drags: {{}}\", s.get().last_value, s.get().changes, s.get().drags) }}, id: \"stats_label\", height: 30) {{}}\n"
        ));
        let format_line = out
            .lines()
            .find(|l| l.trim_start().starts_with("format!("))
            .unwrap_or_else(|| panic!("no format!( line, got:\n{out}"));
        let arg_line = out
            .lines()
            .find(|l| l.trim_start().starts_with("\"value:"))
            .unwrap_or_else(|| panic!("no arg line, got:\n{out}"));
        let lead = |l: &str| l.len() - l.trim_start().len();
        assert!(
            lead(arg_line) > lead(format_line),
            "args must be deeper than format!(, got:\n{out}"
        );
        // The `${` block body sits exactly one level (4) under its `text: ${`
        // attr line — not two, which is the const-wrapper over-indent bug.
        let attr_line = out
            .lines()
            .find(|l| l.trim_start().starts_with("text: ${"))
            .unwrap_or_else(|| panic!("no `text: ${{` line, got:\n{out}"));
        assert_eq!(
            lead(format_line),
            lead(attr_line) + 4,
            "format!( must be one level under the attr, not over-indented, got:\n{out}"
        );
    }

    #[test]
    fn walk_by_key_roundtrips() {
        let out = fmt(&format!(
            "{CTX}walk $items with item by item.id {{ Text (\"x\") {{}} }}\n"
        ));
        assert!(
            out.contains("with item by item.id"),
            "walk keeps the `by <key>` clause, got:\n{out}"
        );
    }

    #[test]
    fn nested_call_enchant_keeps_relative_nesting() {
        let out = fmt(&format!(
            "{CTX}Column (grow: 1.0) [ GaussRadius(Tween::new(Fixed::from_int(0), Fixed::from_int(3), MS, ease::lin, PlayMode::PingPong).into()) ] {{ Text (\"x\") {{}} }}\n"
        ));
        let lead = |needle: &str| {
            let l = out
                .lines()
                .find(|l| l.trim_start().starts_with(needle))
                .unwrap_or_else(|| panic!("no `{needle}` line, got:\n{out}"));
            l.len() - l.trim_start().len()
        };
        assert_eq!(
            lead("Tween::new("),
            lead("GaussRadius(") + 4,
            "inner call one level under outer, got:\n{out}"
        );
        assert!(
            lead("Fixed::from_int(0)") > lead("Tween::new("),
            "call args deeper than the call, got:\n{out}"
        );
    }

    #[test]
    fn short_walk_block_stays_inline() {
        let out = fmt(&format!(
            "{CTX}walk ${{ rows.get() }} with card {{ Text (\"x\") {{}} }}\n"
        ));
        assert!(
            out.contains("walk ${ rows.get() } with card"),
            "a short ${{ block }} iterable stays on one line, got:\n{out}"
        );
    }

    #[test]
    fn multiline_walk_block_iterable_indents_under_walk() {
        // Long enough that prettyplease wraps the iterable across lines.
        let out = fmt(&format!(
            "{CTX}walk ${{ really_long_source_name.get().iter().filter(|row| row.is_enabled && row.visible).cloned().collect::<alloc::vec::Vec<_>>() }} with card {{ Text (\"x\") {{}} }}\n"
        ));
        let walk_lead = out
            .lines()
            .find(|l| l.trim_start() == "walk ${")
            .map(|l| l.len() - l.trim_start().len())
            .unwrap_or_else(|| panic!("expected multi-line `walk ${{`, got:\n{out}"));
        let body = out
            .lines()
            .find(|l| l.trim_start().starts_with("really_long_source_name"))
            .unwrap_or_else(|| panic!("no iterable body line, got:\n{out}"));
        let body_lead = body.len() - body.trim_start().len();
        assert!(
            body_lead > walk_lead,
            "wrapped iterable body indents under walk, not column 0, got:\n{out}"
        );
    }
}
