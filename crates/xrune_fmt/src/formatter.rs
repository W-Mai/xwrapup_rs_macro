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

    // Context area
    out.push_str(&indent1);
    out.push_str(":(\n");
    let indent2 = format!("{indent1}    ");
    for attr in root.get_context_attrs() {
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

    // Content tree
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
                        out.push_str(&fmt_expr(a));
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
            out.push_str(&fmt_expr(if_node.get_condition()));
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
            out.push_str(&fmt_expr(iter_node.get_iterable()));
            out.push_str(" with ");
            out.push_str(&iter_node.get_variable().to_string());
            if let Some(key) = iter_node.get_key() {
                out.push_str(" by ");
                out.push_str(&fmt_expr(key));
            }
            out.push_str(" {\n");
            for child in borrowed.get_children() {
                format_tree(child, &child_indent, out);
            }
            out.push_str(indent);
            out.push_str("}\n");
        }
        DsNode::Niche(niche_node) => {
            out.push_str(indent);
            out.push('@');
            out.push_str(&niche_node.get_name().to_string());
            out.push_str(" {\n");
            for child in borrowed.get_children() {
                format_tree(child, &child_indent, out);
            }
            out.push_str(indent);
            out.push_str("}\n");
        }
        DsNode::Match(match_node) => {
            out.push_str(indent);
            out.push_str("match ");
            if match_node.is_reactive() {
                out.push('$');
            }
            out.push_str(&fmt_expr(match_node.get_scrutinee()));
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
            out.push_str(&fmt_expr(if_node.get_condition()));
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
    let tokens = quote::quote!(#expr);
    let code = format!("const _: () = {{ let _ = {tokens}; }};");
    let Ok(file) = syn::parse_str::<syn::File>(&code) else {
        return tokens.to_string();
    };
    let formatted = prettyplease::unparse(&file);
    let Some(start) = formatted.find("let _ = ") else {
        return tokens.to_string();
    };
    let start = start + 8;
    let Some(end) = formatted[start..].find(";\n") else {
        return tokens.to_string();
    };
    let result = formatted[start..start + end].trim().to_string();

    // Re-indent multi-line results
    if result.contains('\n') && !indent.is_empty() {
        let inner_indent = format!("{indent}    ");
        result
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if i == 0 {
                    line.to_string()
                } else {
                    let trimmed = line.trim_start();
                    if trimmed == "}" || trimmed == "}," || trimmed == ")" || trimmed == ")," {
                        // Closing brace aligns with opening
                        format!("{indent}{trimmed}")
                    } else {
                        format!("{inner_indent}{trimmed}")
                    }
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        result
    }
}

fn fmt_block(block: &syn::Block, indent: &str) -> String {
    let tokens = quote::quote!(#block);
    let code = format!("fn __xrune_fmt_block_wrapper() {tokens}");
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
    let drop_outer = lines.iter().all(|l| l.is_empty() || l.starts_with("    "));

    let body_indent = format!("{indent}    ");
    let mut out = String::from("{\n");
    for line in &lines {
        if line.is_empty() {
            out.push('\n');
            continue;
        }
        let stripped: &str = if drop_outer && line.len() >= 4 {
            &line[4..]
        } else {
            line
        };
        out.push_str(&body_indent);
        out.push_str(stripped);
        out.push('\n');
    }
    out.push_str(indent);
    out.push('}');
    out
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
    fn walk_by_key_roundtrips() {
        let out = fmt(&format!(
            "{CTX}walk $items with item by item.id {{ Text (\"x\") {{}} }}\n"
        ));
        assert!(
            out.contains("with item by item.id"),
            "walk keeps the `by <key>` clause, got:\n{out}"
        );
    }
}
