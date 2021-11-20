use swc_common::Span;
use swc_ecma_ast::Stmt;

// pub(crate) fn span_of_stmt(stmt: &Stmt) -> &Span {
//     match stmt {
//         Stmt::Block(block_stmt) => &block_stmt.span,
//         Stmt::Empty(empty_stmt) => &empty_stmt.span,
//         Stmt::Debugger(debugger_stmt) => &debugger_stmt.span,
//         Stmt::With(with_stmt) => &with_stmt.span,
//         Stmt::Return(return_stmt) => &return_stmt.span,
//         Stmt::Labeled(labeled_stmt) => &labeled_stmt.span,
//         Stmt::Break(break_stmt) => &break_stmt.span,
//         Stmt::Continue(continue_stmt) => &continue_stmt.span,
//         Stmt::If(_) => {}
//         Stmt::Switch(_) => {}
//         Stmt::Throw(_) => {}
//         Stmt::Try(_) => {}
//         Stmt::While(_) => {}
//         Stmt::DoWhile(_) => {}
//         Stmt::For(_) => {}
//         Stmt::ForIn(_) => {}
//         Stmt::ForOf(_) => {}
//         Stmt::Decl(_) => {}
//         Stmt::Expr(_) => {}
//     }
// }