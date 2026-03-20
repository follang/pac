fn __parse_unary_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_unary_expression0(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { Box::new(e) }),
            Failed => Failed,
        }
    }
}

fn __parse_unary_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = __parse_postfix_expression0(__input, __state, __pos, env);
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = __parse_unary_prefix(__input, __state, __pos, env);
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = __parse_unary_cast(__input, __state, __pos, env);
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = __parse_sizeof_expression(__input, __state, __pos, env);
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __choice_res = __parse_alignof_expression(__input, __state, __pos, env);
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                                let __seq_res = {
                                                    let __seq_res = {
                                                        __state.suppress_fail += 1;
                                                        let __assert_res = __parse_gnu_guard(__input, __state, __pos, env);
                                                        __state.suppress_fail -= 1;
                                                        match __assert_res {
                                                            Matched(_, __value) => Matched(__pos, __value),
                                                            Failed => Failed,
                                                        }
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res = {
                                                                __state.suppress_fail += 1;
                                                                let res = {
                                                                    let __seq_res = slice_eq(__input, __state, __pos, "__extension__");
                                                                    match __seq_res {
                                                                        Matched(__pos, e) => {
                                                                            let __seq_res = {
                                                                                __state.suppress_fail += 1;
                                                                                let __assert_res = if __input.len() > __pos {
                                                                                    let (__ch, __next) = char_range_at(__input, __pos);
                                                                                    match __ch {
                                                                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                                                    }
                                                                                } else {
                                                                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                                                                };
                                                                                __state.suppress_fail -= 1;
                                                                                match __assert_res {
                                                                                    Failed => Matched(__pos, ()),
                                                                                    Matched(..) => Failed,
                                                                                }
                                                                            };
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => Matched(__pos, { e }),
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                };
                                                                __state.suppress_fail -= 1;
                                                                res
                                                            };
                                                            match __seq_res {
                                                                Matched(__pos, e) => Matched(__pos, { e }),
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                };
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = __parse_unary_expression0(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, e) => Matched(__pos, { e }),
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn __parse_unary_prefix<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_unary_prefix_inner(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, n) => Matched(__pos, { Expression::UnaryOperator(Box::new(n)) }),
            Failed => Failed,
        }
    }
}

fn __parse_unary_prefix_inner<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<UnaryOperatorExpression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_prefix_operator(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, op) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_unary_expression(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, e) => Matched(__pos, { UnaryOperatorExpression { operator: op, operand: e } }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_prefix_operator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<UnaryOperator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = slice_eq(__input, __state, __pos, "++");
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { UnaryOperator::PreIncrement }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = slice_eq(__input, __state, __pos, "--");
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { UnaryOperator::PreDecrement }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_unary_cast<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_unary_cast_inner(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, n) => Matched(__pos, { Expression::UnaryOperator(Box::new(n)) }),
            Failed => Failed,
        }
    }
}

fn __parse_unary_cast_inner<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<UnaryOperatorExpression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_unary_operator(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, op) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_cast_expression(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, e) => Matched(__pos, { UnaryOperatorExpression { operator: op, operand: e } }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_unary_operator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<UnaryOperator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = slice_eq(__input, __state, __pos, "&");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let __assert_res = slice_eq(__input, __state, __pos, "&");
                        __state.suppress_fail -= 1;
                        match __assert_res {
                            Failed => Matched(__pos, ()),
                            Matched(..) => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { UnaryOperator::Address }),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "*");
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { UnaryOperator::Indirection }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let __seq_res = slice_eq(__input, __state, __pos, "+");
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { UnaryOperator::Plus }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = {
                                    let __seq_res = slice_eq(__input, __state, __pos, "-");
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { UnaryOperator::Minus }),
                                        Failed => Failed,
                                    }
                                };
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __choice_res = {
                                            let __seq_res = slice_eq(__input, __state, __pos, "~");
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { UnaryOperator::Complement }),
                                                Failed => Failed,
                                            }
                                        };
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "!");
                                                match __seq_res {
                                                    Matched(__pos, _) => Matched(__pos, { UnaryOperator::Negate }),
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn __parse_sizeof_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_sizeof_ty_expression(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, n) => Matched(__pos, { Expression::SizeOfTy(n) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = __parse_sizeof_val_expression(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, n) => Matched(__pos, { Expression::SizeOfVal(n) }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_sizeof_ty_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<SizeOfTy>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_sizeof_ty_expression0(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { Box::new(e) }),
            Failed => Failed,
        }
    }
}

fn __parse_sizeof_ty_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<SizeOfTy> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "sizeof");
                match __seq_res {
                    Matched(__pos, e) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                            };
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { e }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            __state.suppress_fail -= 1;
            res
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, "(");
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse_type_name(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, t) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                        match __seq_res {
                                                            Matched(__pos, _) => Matched(__pos, { SizeOfTy(t) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_sizeof_val_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<SizeOfVal>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_sizeof_val_expression0(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { Box::new(e) }),
            Failed => Failed,
        }
    }
}

fn __parse_sizeof_val_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<SizeOfVal> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "sizeof");
                match __seq_res {
                    Matched(__pos, e) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                            };
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { e }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            __state.suppress_fail -= 1;
            res
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_unary_expression(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, e) => Matched(__pos, { SizeOfVal(e) }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_alignof_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_alignof(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { Expression::AlignOf(e) }),
            Failed => Failed,
        }
    }
}

fn __parse_alignof<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<AlignOf>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_alignof0(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { Box::new(e) }),
            Failed => Failed,
        }
    }
}

fn __parse_alignof0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<AlignOf> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = {
                    let __choice_res = slice_eq(__input, __state, __pos, "_Alignof");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "alignof");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let __assert_res = __parse_gnu_guard(__input, __state, __pos, env);
                                __state.suppress_fail -= 1;
                                match __assert_res {
                                    Matched(_, __value) => Matched(__pos, __value),
                                    Failed => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = slice_eq(__input, __state, __pos, "__alignof");
                                        match __seq_res {
                                            Matched(__pos, _) => match slice_eq(__input, __state, __pos, "__") {
                                                Matched(__newpos, _) => Matched(__newpos, ()),
                                                Failed => Matched(__pos, ()),
                                            },
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, e) => Matched(__pos, { e }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                            }
                        }
                    }
                };
                match __seq_res {
                    Matched(__pos, e) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                            };
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { e }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            __state.suppress_fail -= 1;
            res
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, "(");
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse_type_name(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, t) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                        match __seq_res {
                                                            Matched(__pos, _) => Matched(__pos, { AlignOf(Box::new(t)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

