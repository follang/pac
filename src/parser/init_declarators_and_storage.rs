fn __parse_declaration_init_declarators<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<InitDeclarator>>> {
    #![allow(non_snake_case, unused)]
    {
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
                            let __seq_res = __parse_init_declarator(__input, __state, __pos, env);
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
            Matched(__repeat_pos, __repeat_value)
        };
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { e }),
            Failed => Failed,
        }
    }
}

fn __parse_declaration_type_declarators<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<InitDeclarator>>> {
    #![allow(non_snake_case, unused)]
    {
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
                            let __seq_res = __parse_type_declarator(__input, __state, __pos, env);
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
            Matched(__repeat_pos, __repeat_value)
        };
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { e }),
            Failed => Failed,
        }
    }
}

fn __parse_init_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<InitDeclarator> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_init_declarator_declarator(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, d) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = match {
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
                                    let __seq_res = __parse_init_declarator_gnu(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, e) => Matched(__pos, { e }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                            Failed => Matched(__pos, None),
                        };
                        match __seq_res {
                            Matched(__pos, e) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = match {
                                            let __seq_res = Matched(__pos, __pos);
                                            match __seq_res {
                                                Matched(__pos, l) => {
                                                    let __seq_res = __parse_init_declarator_init(__input, __state, __pos, env);
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
                                        } {
                                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                            Failed => Matched(__pos, None),
                                        };
                                        match __seq_res {
                                            Matched(__pos, i) => Matched(__pos, { InitDeclarator { declarator: with_ext(d, e), initializer: i } }),
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

fn __parse_init_declarator_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Declarator>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_declarator(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, d) => Matched(__pos, {
                env.handle_declarator(&d, Symbol::Identifier);
                d
            }),
            Failed => Failed,
        }
    }
}

fn __parse_init_declarator_init<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Initializer> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "=");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_initializer(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, i) => Matched(__pos, { i }),
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

fn __parse_init_declarator_gnu<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<Extension>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = match __parse_asm_label(__input, __state, __pos, env) {
            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
            Failed => Matched(__pos, None),
        };
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_attribute_specifier_list(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, a) => Matched(__pos, { l.into_iter().chain(a).collect() }),
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

fn __parse_type_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<InitDeclarator> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_declarator(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, d) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = match {
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
                                    let __seq_res = __parse_init_declarator_gnu(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, e) => Matched(__pos, { e }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                            Failed => Matched(__pos, None),
                        };
                        match __seq_res {
                            Matched(__pos, e) => Matched(__pos, {
                                env.handle_declarator(&d, Symbol::Typename);
                                InitDeclarator { declarator: with_ext(d, e), initializer: None }
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
}

fn __parse_storage_class_specifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<StorageClassSpecifier>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_storage_class_specifier0(__input, __state, __pos, env);
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

fn __parse_storage_class_specifier0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<StorageClassSpecifier> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "extern");
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
                Matched(__pos, _) => Matched(__pos, { StorageClassSpecifier::Extern }),
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
                            let __seq_res = slice_eq(__input, __state, __pos, "static");
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
                        Matched(__pos, _) => Matched(__pos, { StorageClassSpecifier::Static }),
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
                                        let __choice_res = slice_eq(__input, __state, __pos, "_Thread_local");
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => slice_eq(__input, __state, __pos, "thread_local"),
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
                                Matched(__pos, _) => Matched(__pos, { StorageClassSpecifier::ThreadLocal }),
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
                                            let __seq_res = slice_eq(__input, __state, __pos, "auto");
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
                                        Matched(__pos, _) => Matched(__pos, { StorageClassSpecifier::Auto }),
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
                                                    let __seq_res = slice_eq(__input, __state, __pos, "register");
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
                                                Matched(__pos, _) => Matched(__pos, { StorageClassSpecifier::Register }),
                                                Failed => Failed,
                                            }
                                        };
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                            let __seq_res = {
                                                __state.suppress_fail += 1;
                                                let res = {
                                                    let __seq_res = slice_eq(__input, __state, __pos, "constexpr");
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
                                                Matched(__pos, _) => Matched(__pos, { StorageClassSpecifier::Constexpr }),
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

fn __parse_storage_class_typedef<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<StorageClassSpecifier>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_storage_class_typedef0(__input, __state, __pos, env);
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

fn __parse_storage_class_typedef0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<StorageClassSpecifier> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "typedef");
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
            Matched(__pos, _) => Matched(__pos, { StorageClassSpecifier::Typedef }),
            Failed => Failed,
        }
    }
}

