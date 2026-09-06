use swc_core::ecma::ast::{BinaryOp, Decl, Expr, Lit, OptChainBase, Stmt};

/// The ESTree node kind of an expression, as a diagnostic names it —
/// `"CallExpression"`, `"ArrowFunctionExpression"`, `"BigIntLiteral"`.
///
/// # Why not `Expr::get_type`
///
/// `get_type` answers the *value* an expression would produce — `Known(Str)`,
/// `Unknown` — which is a different question and is `Unknown` for every
/// expression a static evaluator cannot fold. Naming those `Unsupported
/// expression: Unknown` tells an author only that something went wrong, which
/// is the one thing they already know. The kind is what points at the source.
///
/// # Why ESTree names rather than SWC's
///
/// The kind reaches an author inside a compiler diagnostic, so it is part of
/// the observable output and has to be spelled the way the ecosystem spells
/// it: the same string a parser, a linter rule, or a search of the language
/// spec uses. SWC's variant names are abbreviations of those (`Bin`, `Tpl`,
/// `TaggedTpl`) and two of its own tags diverge from ESTree
/// (`ParenthesisExpression`, `OptionalChainingExpression`), so the mapping is
/// written out rather than derived.
///
/// Three places where the two ASTs disagree about node boundaries, resolved
/// towards ESTree because that is what the name has to mean to a reader:
///
/// - A logical operator is a `LogicalExpression`, not a `BinaryExpression`.
///   SWC keeps `&&`, `||` and `??` in [`Expr::Bin`]; ESTree gives them their
///   own node, and a diagnostic about `a && b` that says `BinaryExpression` is
///   wrong.
/// - An optional chain is named by its base, so `a?.b` is an
///   `OptionalMemberExpression` and `a?.b()` an `OptionalCallExpression`.
///   SWC wraps both in one [`Expr::OptChain`].
/// - `super.x` is a `MemberExpression`. SWC gives it
///   [`Expr::SuperProp`]; ESTree makes `super` the object of an ordinary
///   member expression.
///
/// Two SWC nodes have no single ESTree spelling. [`Expr::TsConstAssertion`]
/// carries both `x as const` (`TSAsExpression`) and `<const>x`
/// (`TSTypeAssertion`); it is named for the first, which is the form
/// TypeScript code is written in. [`Expr::Invalid`] is a parse failure with no
/// node at all, and is named `"Invalid"` after the SWC variant, because there
/// is nothing else to call it.
///
/// The match is deliberately exhaustive — no wildcard arm — so a new SWC
/// expression kind fails to compile here instead of silently reporting the
/// wrong name.
pub fn get_expr_node_kind(expr: &Expr) -> &'static str {
  match expr {
    Expr::This(_) => "ThisExpression",
    Expr::Array(_) => "ArrayExpression",
    Expr::Object(_) => "ObjectExpression",
    Expr::Fn(_) => "FunctionExpression",
    Expr::Unary(_) => "UnaryExpression",
    Expr::Update(_) => "UpdateExpression",
    Expr::Bin(bin) => match bin.op {
      BinaryOp::LogicalOr | BinaryOp::LogicalAnd | BinaryOp::NullishCoalescing => {
        "LogicalExpression"
      },
      _ => "BinaryExpression",
    },
    Expr::Assign(_) => "AssignmentExpression",
    Expr::Member(_) | Expr::SuperProp(_) => "MemberExpression",
    Expr::Cond(_) => "ConditionalExpression",
    Expr::Call(_) => "CallExpression",
    Expr::New(_) => "NewExpression",
    Expr::Seq(_) => "SequenceExpression",
    Expr::Ident(_) => "Identifier",
    Expr::Lit(lit) => match lit {
      Lit::Str(_) => "StringLiteral",
      Lit::Bool(_) => "BooleanLiteral",
      Lit::Null(_) => "NullLiteral",
      Lit::Num(_) => "NumericLiteral",
      Lit::BigInt(_) => "BigIntLiteral",
      Lit::Regex(_) => "RegExpLiteral",
      Lit::JSXText(_) => "JSXText",
    },
    Expr::Tpl(_) => "TemplateLiteral",
    Expr::TaggedTpl(_) => "TaggedTemplateExpression",
    Expr::Arrow(_) => "ArrowFunctionExpression",
    Expr::Class(_) => "ClassExpression",
    Expr::Yield(_) => "YieldExpression",
    Expr::MetaProp(_) => "MetaProperty",
    Expr::Await(_) => "AwaitExpression",
    Expr::Paren(_) => "ParenthesizedExpression",
    Expr::JSXMember(_) => "JSXMemberExpression",
    Expr::JSXNamespacedName(_) => "JSXNamespacedName",
    Expr::JSXEmpty(_) => "JSXEmptyExpression",
    Expr::JSXElement(_) => "JSXElement",
    Expr::JSXFragment(_) => "JSXFragment",
    Expr::TsTypeAssertion(_) => "TSTypeAssertion",
    Expr::TsConstAssertion(_) => "TSAsExpression",
    Expr::TsNonNull(_) => "TSNonNullExpression",
    Expr::TsAs(_) => "TSAsExpression",
    Expr::TsInstantiation(_) => "TSInstantiationExpression",
    Expr::TsSatisfies(_) => "TSSatisfiesExpression",
    Expr::PrivateName(_) => "PrivateName",
    Expr::OptChain(opt_chain) => match opt_chain.base.as_ref() {
      OptChainBase::Member(_) => "OptionalMemberExpression",
      OptChainBase::Call(_) => "OptionalCallExpression",
    },
    Expr::Invalid(_) => "Invalid",
  }
}

/// The ESTree node kind of a statement, as a diagnostic names it —
/// `"ForStatement"`, `"SwitchStatement"`, `"FunctionDeclaration"`.
///
/// The statement half of [`get_expr_node_kind`], spelled the same way and for
/// the same reason: the kind reaches an author inside a diagnostic, so it has
/// to be the word a parser, a linter rule or the language spec would use.
///
/// The match is deliberately exhaustive — no wildcard arm — so a new SWC
/// statement kind fails to compile here instead of silently reporting the
/// wrong name.
pub fn get_stmt_node_kind(stmt: &Stmt) -> &'static str {
  match stmt {
    Stmt::Block(_) => "BlockStatement",
    Stmt::Empty(_) => "EmptyStatement",
    Stmt::Debugger(_) => "DebuggerStatement",
    Stmt::With(_) => "WithStatement",
    Stmt::Return(_) => "ReturnStatement",
    Stmt::Labeled(_) => "LabeledStatement",
    Stmt::Break(_) => "BreakStatement",
    Stmt::Continue(_) => "ContinueStatement",
    Stmt::If(_) => "IfStatement",
    Stmt::Switch(_) => "SwitchStatement",
    Stmt::Throw(_) => "ThrowStatement",
    Stmt::Try(_) => "TryStatement",
    Stmt::While(_) => "WhileStatement",
    Stmt::DoWhile(_) => "DoWhileStatement",
    Stmt::For(_) => "ForStatement",
    Stmt::ForIn(_) => "ForInStatement",
    Stmt::ForOf(_) => "ForOfStatement",
    Stmt::Expr(_) => "ExpressionStatement",
    Stmt::Decl(decl) => match decl {
      Decl::Class(_) => "ClassDeclaration",
      Decl::Fn(_) => "FunctionDeclaration",
      Decl::Var(_) => "VariableDeclaration",
      Decl::Using(_) => "VariableDeclaration",
      Decl::TsInterface(_) => "TSInterfaceDeclaration",
      Decl::TsTypeAlias(_) => "TSTypeAliasDeclaration",
      Decl::TsEnum(_) => "TSEnumDeclaration",
      Decl::TsModule(_) => "TSModuleDeclaration",
    },
  }
}

#[cfg(test)]
#[path = "tests/swc_test.rs"]
mod tests;
