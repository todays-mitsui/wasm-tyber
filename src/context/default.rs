use crate::context::Context;
use crate::expr;
use crate::func;

impl Default for Context {
    fn default() -> Self {
        Context::from(vec![
            func::new("i", vec!["x"], "x"),
            func::new("k", vec!["x", "y"], "x"),
            func::new(
                "s",
                vec!["x", "y", "z"],
                expr::a(expr::a("x", "z"), expr::a("y", "z")),
            ),
            func::new("TRUE", Vec::<&str>::new(), expr::l("x", expr::l("y", "x"))),
            func::new("FALSE", Vec::<&str>::new(), expr::l("x", expr::l("y", "y"))),
            func::new(
                "IF",
                vec!["PRED", "THEN", "ELSE"],
                expr::a(expr::a("PRED", "THEN"), "ELSE"),
            ),
            func::new("NOT", vec!["x"], expr::a(expr::a("x", "FALSE"), "TRUE")),
            func::new("AND", vec!["x", "y"], expr::a(expr::a("x", "y"), "FALSE")),
            func::new("OR", vec!["x", "y"], expr::a(expr::a("x", "TRUE"), "y")),
            func::new(
                "XOR",
                vec!["x", "y"],
                expr::a(expr::a("x", expr::a("NOT", "y")), "y"),
            ),
            func::new(
                "CONS",
                vec!["x", "y"],
                expr::l("f", expr::a(expr::a("f", "x"), "y")),
            ),
            func::new("CAR", vec!["x"], expr::a("x", "TRUE")),
            func::new("CDR", vec!["x"], expr::a("x", "FALSE")),
            func::new("NIL", Vec::<&str>::new(), "FALSE"),
            func::new(
                "IS_NIL",
                vec!["x"],
                expr::a(expr::a("x", expr::l("_", "FALSE")), "TRUE"),
            ),
            func::new(
                "Y",
                vec!["f"],
                expr::a(
                    expr::l("x", expr::a("f", expr::a("x", "x"))),
                    expr::l("x", expr::a("f", expr::a("x", "x"))),
                ),
            ),
            func::new(
                "Z",
                vec!["f"],
                expr::a(
                    expr::l(
                        "x",
                        expr::a("f", expr::l("y", expr::a(expr::a("x", "x"), "y"))),
                    ),
                    expr::l(
                        "x",
                        expr::a("f", expr::l("y", expr::a(expr::a("x", "x"), "y"))),
                    ),
                ),
            ),
            func::new(
                "IS_ZERO",
                vec!["n"],
                expr::a(expr::a("n", expr::l("_", "FALSE")), "TRUE"),
            ),
            func::new(
                "SUCC",
                vec!["n"],
                expr::l(
                    "f",
                    expr::l("x", expr::a("f", expr::a(expr::a("n", "f"), "x"))),
                ),
            ),
            func::new(
                "ADD",
                vec!["m", "n"],
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a(expr::a("m", "f"), expr::a(expr::a("n", "f"), "x")),
                    ),
                ),
            ),
            func::new(
                "MUL",
                vec!["m", "n"],
                expr::l("f", expr::a("m", expr::a("n", "f"))),
            ),
            func::new("POW", vec!["m", "n"], expr::a("n", "m")),
            func::new(
                "PRED",
                vec!["n"],
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a(
                            expr::a(
                                expr::a(
                                    "n",
                                    expr::l("g", expr::l("h", expr::a("h", expr::a("g", "f")))),
                                ),
                                expr::l("u", "x"),
                            ),
                            expr::l("u", "u"),
                        ),
                    ),
                ),
            ),
            func::new("SUB", vec!["m", "n"], expr::a(expr::a("n", "PRED"), "m")),
            func::new(
                "GTE",
                vec!["m", "n"],
                expr::a("IS_ZERO", expr::a(expr::a("SUB", "n"), "m")),
            ),
            func::new(
                "LTE",
                vec!["m", "n"],
                expr::a("IS_ZERO", expr::a(expr::a("SUB", "m"), "n")),
            ),
            func::new(
                "EQ",
                vec!["m", "n"],
                expr::a(
                    expr::a("AND", expr::a(expr::a("GTE", "m"), "n")),
                    expr::a(expr::a("LTE", "m"), "n"),
                ),
            ),
            func::new("0", Vec::<&str>::new(), expr::l("f", expr::l("x", "x"))),
            func::new(
                "1",
                Vec::<&str>::new(),
                expr::l("f", expr::l("x", expr::a("f", "x"))),
            ),
            func::new(
                "2",
                Vec::<&str>::new(),
                expr::l("f", expr::l("x", expr::a("f", expr::a("f", "x")))),
            ),
            func::new(
                "3",
                Vec::<&str>::new(),
                expr::l(
                    "f",
                    expr::l("x", expr::a("f", expr::a("f", expr::a("f", "x")))),
                ),
            ),
            func::new(
                "4",
                Vec::<&str>::new(),
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a("f", expr::a("f", expr::a("f", expr::a("f", "x")))),
                    ),
                ),
            ),
            func::new(
                "5",
                Vec::<&str>::new(),
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a(
                            "f",
                            expr::a("f", expr::a("f", expr::a("f", expr::a("f", "x")))),
                        ),
                    ),
                ),
            ),
            func::new(
                "6",
                Vec::<&str>::new(),
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a(
                            "f",
                            expr::a(
                                "f",
                                expr::a("f", expr::a("f", expr::a("f", expr::a("f", "x")))),
                            ),
                        ),
                    ),
                ),
            ),
            func::new(
                "7",
                Vec::<&str>::new(),
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a(
                            "f",
                            expr::a(
                                "f",
                                expr::a(
                                    "f",
                                    expr::a("f", expr::a("f", expr::a("f", expr::a("f", "x")))),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            func::new(
                "8",
                Vec::<&str>::new(),
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a(
                            "f",
                            expr::a(
                                "f",
                                expr::a(
                                    "f",
                                    expr::a(
                                        "f",
                                        expr::a("f", expr::a("f", expr::a("f", expr::a("f", "x")))),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            func::new(
                "9",
                Vec::<&str>::new(),
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a(
                            "f",
                            expr::a(
                                "f",
                                expr::a(
                                    "f",
                                    expr::a(
                                        "f",
                                        expr::a(
                                            "f",
                                            expr::a(
                                                "f",
                                                expr::a("f", expr::a("f", expr::a("f", "x"))),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            func::new(
                "10",
                Vec::<&str>::new(),
                expr::l(
                    "f",
                    expr::l(
                        "x",
                        expr::a(
                            "f",
                            expr::a(
                                "f",
                                expr::a(
                                    "f",
                                    expr::a(
                                        "f",
                                        expr::a(
                                            "f",
                                            expr::a(
                                                "f",
                                                expr::a(
                                                    "f",
                                                    expr::a("f", expr::a("f", expr::a("f", "x"))),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ])
    }
}