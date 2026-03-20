fn __parse_primary_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_primary_expression0(__input, __state, __pos, env);
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

fn __parse_primary_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_string_literal(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, a) => Matched(__pos, { Expression::StringLiteral(Box::new(a)) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = {
                        let __seq_res = Matched(__pos, __pos);
                        match __seq_res {
                            Matched(__pos, l) => {
                                let __seq_res = __parse_constant(__input, __state, __pos, env);
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
                        Matched(__pos, a) => Matched(__pos, { Expression::Constant(Box::new(a)) }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let res = {
                                    let __seq_res = slice_eq(__input, __state, __pos, "true");
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
                                Matched(__pos, _) => Matched(__pos, {
                                    Expression::Constant(Box::new(Node::new(Constant::Integer(Integer {
                                        base: IntegerBase::Decimal,
                                        number: "1".into(),
                                        suffix: IntegerSuffix { size: IntegerSize::Int, unsigned: false, imaginary: false },
                                    }), Span::none())))
                                }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                        let __choice_res = {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let res = {
                                    let __seq_res = {
                                        let __choice_res = slice_eq(__input, __state, __pos, "false");
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => slice_eq(__input, __state, __pos, "nullptr"),
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
                                Matched(__pos, _) => Matched(__pos, {
                                    Expression::Constant(Box::new(Node::new(Constant::Integer(Integer {
                                        base: IntegerBase::Decimal,
                                        number: "0".into(),
                                        suffix: IntegerSuffix { size: IntegerSize::Int, unsigned: false, imaginary: false },
                                    }), Span::none())))
                                }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                        let __choice_res = {
                            let __seq_res = __parse_identifier(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, a) => Matched(__pos, { Expression::Identifier(Box::new(a)) }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = {
                                    let __seq_res = slice_eq(__input, __state, __pos, "(");
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = __parse_expression0(__input, __state, __pos, env);
                                                    match __seq_res {
                                                        Matched(__pos, a) => {
                                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                                            match __seq_res {
                                                                Matched(__pos, _) => {
                                                                    let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => Matched(__pos, { a }),
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
                                };
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __choice_res = {
                                            let __seq_res = {
                                                let __seq_res = Matched(__pos, __pos);
                                                match __seq_res {
                                                    Matched(__pos, l) => {
                                                        let __seq_res = __parse_generic_selection(__input, __state, __pos, env);
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
                                                Matched(__pos, a) => Matched(__pos, { Expression::GenericSelection(Box::new(a)) }),
                                                Failed => Failed,
                                            }
                                        };
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
                                                        let __seq_res = __parse_gnu_primary_expression(__input, __state, __pos, env);
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

fn __parse_generic_selection<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<GenericSelection> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "_Generic");
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
                                        let __seq_res = __parse_assignment_expression(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, e) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = slice_eq(__input, __state, __pos, ",");
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = {
                                                                            let __seq_res = {
                                                                                let mut __repeat_pos = __pos;
                                                                                let mut __repeat_value = vec![];
                                                                                loop {
                                                                                    let __pos = __repeat_pos;
                                                                                    let __pos = if __repeat_value.len() > 0 {
                                                                                        let __sep_res = {
                                                                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                            match __seq_res {
                                                                                                Matched(__pos, _) => {
                                                                                                    let __seq_res = slice_eq(__input, __state, __pos, ",");
                                                                                                    match __seq_res {
                                                                                                        Matched(__pos, _) => __parse__(__input, __state, __pos, env),
                                                                                                        Failed => Failed,
                                                                                                    }
                                                                                                }
                                                                                                Failed => Failed,
                                                                                            }
                                                                                        };
                                                                                        match __sep_res {
                                                                                            Matched(__newpos, _) => __newpos,
                                                                                            Failed => break,
                                                                                        }
                                                                                    } else {
                                                                                        __pos
                                                                                    };
                                                                                    let __step_res = {
                                                                                        let __seq_res = Matched(__pos, __pos);
                                                                                        match __seq_res {
                                                                                            Matched(__pos, l) => {
                                                                                                let __seq_res = __parse_generic_association(__input, __state, __pos, env);
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
                                                                                    match __step_res {
                                                                                        Matched(__newpos, __value) => {
                                                                                            __repeat_pos = __newpos;
                                                                                            __repeat_value.push(__value);
                                                                                        }
                                                                                        Failed => {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                if __repeat_value.len() >= 1 {
                                                                                    Matched(__repeat_pos, __repeat_value)
                                                                                } else {
                                                                                    Failed
                                                                                }
                                                                            };
                                                                            match __seq_res {
                                                                                Matched(__pos, e) => Matched(__pos, { e }),
                                                                                Failed => Failed,
                                                                            }
                                                                        };
                                                                        match __seq_res {
                                                                            Matched(__pos, a) => {
                                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                match __seq_res {
                                                                                    Matched(__pos, _) => {
                                                                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                                                        match __seq_res {
                                                                                            Matched(__pos, _) => Matched(__pos, { GenericSelection { expression: e, associations: a } }),
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

fn __parse_generic_association<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<GenericAssociation> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_type_name(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, t) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, ":");
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = __parse_assignment_expression(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, e) => Matched(__pos, {
                                                    let span = Span::span(t.span.start, e.span.end);
                                                    GenericAssociation::Type(Node::new(GenericAssociationType { type_name: t, expression: e }, span))
                                                }),
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
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = {
                    __state.suppress_fail += 1;
                    let res = {
                        let __seq_res = slice_eq(__input, __state, __pos, "default");
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
                                let __seq_res = slice_eq(__input, __state, __pos, ":");
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = __parse_assignment_expression(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, e) => Matched(__pos, { GenericAssociation::Default(e) }),
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
    }
}

