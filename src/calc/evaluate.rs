use super::apply::apply;
use super::arity::arity;
use crate::context::Context;
use crate::expr::{self, Expr};

#[derive(Debug, Clone, PartialEq)]
pub struct EvalSteps<'a> {
    expr: Expr,
    stack: Stack<'a>,
    context: &'a Context,
    step: Step,
}

/// 簡約のステップ
/// 最左最外簡約を行うために LeftTree → RightTree の順に簡約を試みる
/// 式全体を簡約し終えて正規形を得たら Done となる、それ以上簡約するべきものは何も無い
#[derive(Debug, Clone, PartialEq)]
enum Step {
    LeftTree,
    RightTree(usize),
    Done,
}

impl EvalSteps<'_> {
    pub fn new(expr: Expr, context: &Context) -> EvalSteps {
        EvalSteps {
            expr,
            stack: Stack::new(),
            context,
            step: Step::LeftTree,
        }
    }

    pub fn eval_last(&mut self, limit: usize) -> (Option<Expr>, bool) {
        assert!(0 < limit);

        if let Some(mut e) = self.next() {
            for _ in 0..limit - 1 {
                if let Some(next) = self.next() {
                    e = next;
                } else {
                    return (Some(e), false);
                }
            }

            // TODO: ここの true は嘘をつくことがある、peekable で先読みして正しい結果を返すように変える
            (Some(e), true)
        } else {
            (None, false)
        }
    }

    fn expr(&self) -> Expr {
        let mut expr = self.expr.clone();

        for arg in self.stack.all() {
            expr = expr::a(expr, arg.expr());
        }

        expr
    }
}

impl Iterator for EvalSteps<'_> {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        match self.step {
            Step::LeftTree => self.left_tree(),
            Step::RightTree(n) => self.right_tree(n),
            Step::Done => None,
        }
    }
}

impl EvalSteps<'_> {
    fn left_tree(&mut self) -> Option<Expr> {
        while let Expr::Apply { lhs, rhs } = self.expr.clone() {
            self.expr = *lhs;
            self.stack.push(EvalSteps::new(*rhs, self.context));
        }

        let maybe_args = arity(self.context, &self.expr)
            .filter(|a| *a >= 1 || self.stack.len() >= 1)
            .and_then(|a| self.stack.pop(a));

        if let Some(args) = maybe_args {
            let result = apply(
                &self.context,
                &mut self.expr,
                args.iter().map(|arg| arg.expr()).collect(),
            );
            assert!(result.is_ok());

            Some(self.expr())
        } else {
            self.step = Step::RightTree(0);

            self.next()
        }
    }

    fn right_tree(&mut self, n: usize) -> Option<Expr> {
        match self.stack.nth(n) {
            // スタックの n 番目の枝を取得し、その枝の簡約を試みる
            Some(step) => match step.next() {
                Some(_) => Some(self.expr()),

                // n 番目の枝が簡約済みなら、n+1 番目の枝へ進む
                None => {
                    self.step = Step::RightTree(n + 1);
                    self.next()
                }
            },

            // n がスタックの長さを超えているなら、もう簡約するべきものは何も無い
            None => {
                self.step = Step::Done;
                self.next()
            }
        }
    }
}

// ========================================================================== //

#[derive(Debug, Clone, PartialEq)]
struct Stack<'a>(Vec<EvalSteps<'a>>);

impl<'a> Stack<'a> {
    fn new() -> Stack<'a> {
        Stack(Vec::new())
    }

    fn push(&mut self, expr: EvalSteps<'a>) {
        self.0.push(expr);
    }

    fn pop(&mut self, n: usize) -> Option<Vec<EvalSteps>> {
        let length = self.len();

        if length >= n {
            Some(self.0.drain(length - n..).rev().collect())
        } else {
            None
        }
    }

    fn all(&self) -> Vec<EvalSteps> {
        let mut all = self.0.clone();
        all.reverse();
        all
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    /// 末尾から数えて n 番目の要素を取得する
    fn nth(&mut self, n: usize) -> Option<&mut EvalSteps<'a>> {
        let len = self.0.len();
        if n >= len {
            None
        } else {
            self.0.get_mut(len - n - 1)
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::func;

    fn setup() -> Context {
        let i = func::new("i", vec!["x"], "x");
        let k = func::new("k", vec!["x", "y"], "x");
        let s = func::new(
            "s",
            vec!["x", "y", "z"],
            expr::a(expr::a("x", "z"), expr::a("y", "z")),
        );

        let _true = func::new("TRUE", Vec::<&str>::new(), expr::a("k", "i"));
        let _false = func::new("FALSE", Vec::<&str>::new(), "k");

        Context::from(vec![i, k, s, _true, _false])
    }

    #[test]
    fn test_eval_steps_lambda_i() {
        let context = Context::new();

        let i = expr::l("x", "x");
        let expr = expr::a(i, ":a");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next().map(|e| e.to_string()),
            Some(expr::s("a")).map(|e| e.to_string())
        );
        assert_eq!(steps.next().map(|e| e.to_string()), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_1() {
        let context = Context::new();

        let k = expr::l("x", expr::l("y", "x"));
        let expr = expr::a(k, ":a");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(expr::l("y", ":a")));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_2() {
        let context = Context::new();

        let k = expr::l("x", expr::l("y", "x"));
        let expr = expr::a(expr::a(k, ":a"), ":b");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(expr::a(expr::l("y", ":a"), ":b")));
        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_true_1() {
        let context = setup();

        let expr = expr::v("TRUE");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_true_2() {
        let context = setup();

        let expr = expr::a(":a", "TRUE");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_true_3() {
        let context = setup();

        let expr = expr::a(expr::a("TRUE", ":a"), ":b");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next(),
            Some(expr::a(expr::a(expr::a("k", "i"), ":a"), ":b"))
        );
        assert_eq!(steps.next(), Some(expr::a("i", ":b")));
        assert_eq!(steps.next(), Some(":b".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_i() {
        let context = setup();

        let expr = expr::a("i", ":a");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_k_1() {
        let context = setup();

        let expr = expr::a("k", ":a");

        let mut steps = EvalSteps::new(expr, &context);

        // k の arity が2なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_k_2() {
        let context = setup();

        let expr = expr::a(expr::a("k", ":a"), ":b");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_1() {
        let context = setup();

        let expr = expr::a("s", ":a");

        let mut steps = EvalSteps::new(expr, &context);

        // s の arity が3なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_2() {
        let context = setup();

        let expr = expr::a(expr::a("s", ":a"), ":b");

        let mut steps = EvalSteps::new(expr, &context);

        // s の arity が3なのに対して引数を2つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_3() {
        let context = setup();

        let expr = expr::a(expr::a(expr::a("s", ":a"), ":b"), ":c");

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next(),
            Some(expr::a(expr::a(":a", ":c"), expr::a(":b", ":c")))
        );
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_skk() {
        let context = setup();

        let expr = expr::a(expr::a(expr::a("s", "k"), "k"), ":a");

        let steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.last(), Some(":a".into()));
    }

    #[test]
    fn test_eval_steps_right_tree_1() {
        let context = setup();

        // `:a``k:b:c
        let expr = expr::a(expr::s("a"), expr::a(expr::a("k", ":b"), ":c"));

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(expr::a(":a", ":b")));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_right_tree_2() {
        let context = setup();

        // ```:a`i:b`i:c
        let expr = expr::a(expr::a(":a", expr::a("i", ":b")), expr::a("i", ":c"));

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next(),
            Some(expr::a(expr::a(":a", ":b"), expr::a("i", ":c")))
        );
        assert_eq!(steps.next(), Some(expr::a(expr::a(":a", ":b"), ":c")));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps() {
        let context = setup();

        // ```s^x.`x:a^x.`x:b:c
        let expr = expr::a(
            expr::a(
                expr::a("s", expr::l("x", expr::a("x", ":a"))),
                expr::l("x", expr::a("x", ":b")),
            ),
            ":c",
        );

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next().map(|e| e.to_string()),
            // ``^x.`x:a:c`^x.`x:b:c
            Some(expr::a(
                expr::a(expr::l("x", expr::a("x", ":a")), ":c"),
                expr::a(expr::l("x", expr::a("x", ":b")), ":c")
            ))
            .map(|e| e.to_string())
        );
        assert_eq!(
            steps.next().map(|e| e.to_string()),
            // ``:c:a`^x.`x:b:c
            Some(expr::a(
                expr::a(":c", ":a"),
                expr::a(expr::l("x", expr::a("x", ":b")), ":c")
            ))
            .map(|e| e.to_string())
        );
        assert_eq!(
            steps.next().map(|e| e.to_string()),
            // ``:c:a`:c:b
            Some(expr::a(expr::a(":c", ":a"), expr::a(":c", ":b"))).map(|e| e.to_string())
        );
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_stack_pop() {
        let context = Context::new();
        let mut stack = Stack(vec![
            EvalSteps::new(expr::v("x"), &context),
            EvalSteps::new(expr::v("y"), &context),
        ]);

        assert_eq!(stack.len(), 2);

        stack.push(EvalSteps::new(expr::v("z"), &context));

        assert_eq!(stack.len(), 3);

        assert_eq!(
            stack.pop(2),
            Some(vec![
                EvalSteps::new(expr::v("z"), &context),
                EvalSteps::new(expr::v("y"), &context)
            ])
        );

        assert_eq!(stack.len(), 1);

        assert_eq!(
            stack.pop(1),
            Some(vec![EvalSteps::new(expr::v("x"), &context)])
        );

        assert_eq!(stack.len(), 0);

        assert_eq!(stack.pop(1), None);
    }

    #[test]
    fn test_stack_all() {
        let context = Context::new();
        let stack = Stack(vec![
            EvalSteps::new(expr::v("x"), &context),
            EvalSteps::new(expr::v("y"), &context),
            EvalSteps::new(expr::v("z"), &context),
        ]);
        assert_eq!(
            stack.all(),
            vec![
                EvalSteps::new(expr::v("z"), &context),
                EvalSteps::new(expr::v("y"), &context),
                EvalSteps::new(expr::v("x"), &context),
            ]
        );

        let stack0 = Stack(vec![]);
        assert_eq!(stack0.all(), vec![]);
    }

    #[test]
    fn test_stack_nth() {
        let context = Context::new();
        let mut stack = Stack(vec![
            EvalSteps::new(expr::v("x"), &context),
            EvalSteps::new(expr::v("y"), &context),
            EvalSteps::new(expr::v("z"), &context),
        ]);

        assert_eq!(
            stack.nth(0),
            Some(&mut EvalSteps::new(expr::v("z"), &context))
        );
        assert_eq!(
            stack.nth(1),
            Some(&mut EvalSteps::new(expr::v("y"), &context))
        );
        assert_eq!(
            stack.nth(2),
            Some(&mut EvalSteps::new(expr::v("x"), &context))
        );
        assert_eq!(stack.nth(3), None);
    }

    #[test]
    fn test_eval_last_1() {
        let context = setup();

        let expr = ":a".into();
        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.eval_last(42), (None, false));
    }

    #[test]
    fn test_eval_last_2() {
        let context = setup();

        let expr = expr::a("i", expr::a("i", expr::a("i", expr::a("i", ":a"))));
        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.eval_last(42), (Some(":a".into()), false));
    }

    #[test]
    fn test_eval_last_3() {
        let context = setup();

        let expr = expr::a("i", expr::a("i", expr::a("i", expr::a("i", ":a"))));
        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.eval_last(3), (Some(expr::a("i", ":a")), true));
    }
}