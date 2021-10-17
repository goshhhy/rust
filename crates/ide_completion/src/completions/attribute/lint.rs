//! Completion for lints
use ide_db::helpers::generated_lints::Lint;
use syntax::ast;

use crate::{
    context::CompletionContext,
    item::{CompletionItem, CompletionItemKind, CompletionKind},
    Completions,
};

pub(super) fn complete_lint(
    acc: &mut Completions,
    ctx: &CompletionContext,
    derive_input: ast::TokenTree,
    lints_completions: &[Lint],
) {
    if let Some(existing_lints) = super::parse_comma_sep_paths(derive_input) {
        for &Lint { label, description } in lints_completions {
            let (ex_q, ex_name) = {
                // FIXME: change `Lint`'s label to not store a path in it but split the prefix off instead?
                let mut parts = label.split("::");
                let ns_or_label = match parts.next() {
                    Some(it) => it,
                    None => continue,
                };
                let label = parts.next();
                match label {
                    Some(label) => (Some(ns_or_label), label),
                    None => (None, ns_or_label),
                }
            };
            let repr_already_annotated = existing_lints
                .iter()
                .filter_map(|path| {
                    let q = path.qualifier();
                    if q.as_ref().and_then(|it| it.qualifier()).is_some() {
                        return None;
                    }
                    Some((q.and_then(|it| it.as_single_name_ref()), path.segment()?.name_ref()?))
                })
                .any(|(q, name)| {
                    let qualifier_matches = match (q, ex_q) {
                        (None, None) => true,
                        (None, Some(_)) => false,
                        (Some(_), None) => false,
                        (Some(q), Some(ns)) => q.text() == ns,
                    };
                    qualifier_matches && name.text() == ex_name
                });
            if repr_already_annotated {
                continue;
            }
            let mut item =
                CompletionItem::new(CompletionKind::Attribute, ctx.source_range(), ex_name);
            item.kind(CompletionItemKind::Attribute)
                .documentation(hir::Documentation::new(description.to_owned()));
            item.add_to(acc)
        }
    }
}
