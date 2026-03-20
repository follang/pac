fn __parse_function_specifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<FunctionSpecifier>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_function_specifier0(__input, __state, __pos, env);
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
    }
}

fn __parse_function_specifier0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<FunctionSpecifier> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = {
                        let __choice_res = slice_eq(__input, __state, __pos, "inline");
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
                                            let __seq_res = slice_eq(__input, __state, __pos, "__inline");
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
                Matched(__pos, _) => Matched(__pos, { FunctionSpecifier::Inline }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = {
                    __state.suppress_fail += 1;
                    let res = {
                        let __seq_res = {
                            let __choice_res = slice_eq(__input, __state, __pos, "_Noreturn");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => slice_eq(__input, __state, __pos, "noreturn"),
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
                    Matched(__pos, _) => Matched(__pos, { FunctionSpecifier::Noreturn }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_alignment_specifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<AlignmentSpecifier>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_alignment_specifier0(__input, __state, __pos, env);
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
    }
}

fn __parse_alignment_specifier0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<AlignmentSpecifier> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = {
                        let __choice_res = slice_eq(__input, __state, __pos, "_Alignas");
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => slice_eq(__input, __state, __pos, "alignas"),
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
                                                                Matched(__pos, _) => Matched(__pos, { AlignmentSpecifier::Type(t) }),
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
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = {
                    __state.suppress_fail += 1;
                    let res = {
                        let __seq_res = {
                        let __choice_res = slice_eq(__input, __state, __pos, "_Alignas");
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => slice_eq(__input, __state, __pos, "alignas"),
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
                                                let __seq_res = __parse_constant_expression(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                                match __seq_res {
                                                                    Matched(__pos, _) => Matched(__pos, { AlignmentSpecifier::Constant(e) }),
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
    }
}

