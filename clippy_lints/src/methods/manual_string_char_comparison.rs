use clippy_utils::diagnostics::span_lint_and_sugg;
use rustc_errors::Applicability;
use rustc_lint::{LateContext, LintContext, LateLintPass};
use rustc_hir::Expr;

use super::MANUAL_STRING_CHAR_COMPARISON;

// TODO: Adjust the parameters as necessary
pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>) {
    span_lint_and_sugg(cx, MANUAL_STRING_CHAR_COMPARISON, expr.span, "This manual char comparaison can be written more succinctly", "try", "".to_string(), Applicability::MachineApplicable)
}
