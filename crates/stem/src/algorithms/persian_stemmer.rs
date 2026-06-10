//! Generated from persian.sbl by Snowball 3.1.1 - https://snowballstem.org/

#![expect(non_snake_case)]
#![expect(non_upper_case_globals)]
#![expect(unused_mut)]
#![expect(unused_parens)]
#![expect(unused_variables)]
use crate::snowball::SnowballEnv;
use crate::snowball::Among;

#[derive(Clone)]
struct Context {
    i_p1: i32,
    b_remove_verb_person_endings: bool,
    b_saw_present_prefix: bool,
}

static A_0: &'static [Among<Context>; 11] = &[
    Among("", -1, 7, None),
    Among(" ", 0, 6, None),
    Among("\u{623}", 0, 4, None),
    Among("\u{624}", 0, 5, None),
    Among("\u{625}", 0, 4, None),
    Among("\u{626}", 0, 2, None),
    Among("\u{629}", 0, 3, None),
    Among("\u{643}", 0, 1, None),
    Among("\u{64A}", 0, 2, None),
    Among("\u{6C1}", 0, 3, None),
    Among("\u{200D}", 0, 6, None),
];

static A_1: &'static [Among<Context>; 2] = &[
    Among("\u{645}\u{6CC}\u{200C}", -1, 2, None),
    Among("\u{646}\u{645}\u{6CC}\u{200C}", -1, 1, None),
];

static A_2: &'static [Among<Context>; 4] = &[
    Among("\u{648}\u{627}\u{646}", -1, -1, None),
    Among("\u{633}\u{62A}\u{627}\u{646}", -1, -1, None),
    Among("\u{631}\u{627}\u{646}", -1, -1, None),
    Among("\u{633}\u{627}\u{646}", -1, -1, None),
];

static A_3: &'static [Among<Context>; 33] = &[
    Among("\u{6AF}\u{6CC}\u{644}\u{627}\u{646}", -1, 1, None),
    Among("\u{622}\u{644}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{645}\u{633}\u{644}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{633}\u{644}\u{6CC}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{627}\u{6CC}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{67E}\u{6CC}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{633}\u{627}\u{62E}\u{62A}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{631}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{642}\u{647}\u{631}\u{645}\u{627}\u{646}", 7, 1, None),
    Among("\u{6A9}\u{631}\u{645}\u{627}\u{646}", 7, 1, None),
    Among("\u{62F}\u{631}\u{645}\u{627}\u{646}", 7, 1, None),
    Among("\u{647}\u{645}\u{632}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{633}\u{627}\u{632}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{622}\u{633}\u{645}\u{627}\u{646}", -1, 1, None),
    Among("\u{6CC}\u{648}\u{646}\u{627}\u{646}", -1, 1, None),
    Among("\u{644}\u{628}\u{646}\u{627}\u{646}", -1, 1, None),
    Among("\u{627}\u{635}\u{641}\u{647}\u{627}\u{646}", -1, 1, None),
    Among("\u{67E}\u{627}\u{6CC}\u{627}\u{646}", -1, 1, None),
    Among("\u{628}\u{6CC}\u{627}\u{646}", -1, 1, None),
    Among("\u{62C}\u{631}\u{6CC}\u{627}\u{646}", -1, 1, None),
    Among("\u{627}\u{645}\u{6A9}\u{627}\u{646}", -1, 1, None),
    Among("\u{622}\u{630}\u{631}\u{628}\u{627}\u{6CC}\u{62C}\u{627}\u{646}", -1, 1, None),
    Among("\u{647}\u{645}\u{62F}\u{627}\u{646}", -1, 1, None),
    Among("\u{62E}\u{627}\u{646}\u{62F}\u{627}\u{646}", -1, 1, None),
    Among("\u{632}\u{646}\u{62F}\u{627}\u{646}", -1, 1, None),
    Among("\u{645}\u{6CC}\u{632}\u{627}\u{646}", -1, 1, None),
    Among("\u{622}\u{62A}\u{634}\u{641}\u{634}\u{627}\u{646}", -1, 1, None),
    Among("\u{646}\u{634}\u{627}\u{646}", -1, 1, None),
    Among("\u{627}\u{6CC}\u{634}\u{627}\u{646}", -1, 1, None),
    Among("\u{67E}\u{631}\u{6CC}\u{634}\u{627}\u{646}", -1, 1, None),
    Among("\u{6A9}\u{647}\u{6A9}\u{634}\u{627}\u{646}", -1, 1, None),
    Among("\u{62F}\u{631}\u{62E}\u{634}\u{627}\u{646}", -1, 1, None),
    Among("\u{633}\u{644}\u{637}\u{627}\u{646}", -1, 1, None),
];

static A_4: &'static [Among<Context>; 2] = &[
    Among("\u{627}\u{633}\u{627}\u{62A}\u{6CC}\u{62F}", -1, 2, None),
    Among("\u{627}\u{62E}\u{628}\u{627}\u{631}", -1, 1, None),
];

static A_5: &'static [Among<Context>; 22] = &[
    Among("\u{627}\u{645}", -1, 1, None),
    Among("\u{6CC}\u{646}", -1, 1, None),
    Among("\u{62A}\u{631}\u{6CC}\u{646}", 1, 1, None),
    Among("\u{627}\u{646}", -1, 1, None),
    Among("\u{6CC}\u{627}\u{646}", 3, 1, None),
    Among("\u{628}\u{627}\u{646}", 3, 1, None),
    Among("\u{6AF}\u{627}\u{646}", 3, 1, None),
    Among("\u{627}\u{646}\u{647}", -1, 1, None),
    Among("\u{6AF}\u{627}\u{647}", -1, 1, None),
    Among("\u{627}\u{646}\u{6CC}", -1, 1, None),
    Among("\u{6CC}\u{6CC}", -1, 1, None),
    Among("\u{647}\u{627}\u{6CC}", -1, 1, None),
    Among("\u{6AF}\u{6CC}", -1, 1, None),
    Among("\u{647}\u{627}", -1, 1, None),
    Among("\u{646}\u{627}\u{6A9}", -1, 1, None),
    Among("\u{6CC}\u{62A}", -1, 1, None),
    Among("\u{627}\u{62A}", -1, 1, None),
    Among("\u{645}\u{646}\u{62F}", -1, 1, None),
    Among("\u{648}\u{627}\u{631}", -1, 1, None),
    Among("\u{6AF}\u{627}\u{631}", -1, 1, None),
    Among("\u{62A}\u{631}", -1, 2, None),
    Among("\u{627}\u{634}", -1, 1, None),
];

static A_6: &'static [Among<Context>; 8] = &[
    Among("\u{6CC}\u{645}", -1, 1, None),
    Among("\u{627}\u{6CC}\u{645}", 0, 1, None),
    Among("\u{627}\u{6CC}", -1, 1, None),
    Among("\u{627}\u{633}\u{62A}", -1, 1, None),
    Among("\u{627}\u{646}\u{62F}", -1, 1, None),
    Among("\u{6CC}\u{62F}", -1, 1, None),
    Among("\u{627}\u{6CC}\u{62F}", 5, 1, None),
    Among("\u{627}\u{633}", -1, 1, None),
];

static A_7: &'static [Among<Context>; 15] = &[
    Among("\u{645}", -1, 1, None),
    Among("\u{6CC}\u{645}", 0, 1, None),
    Among("\u{631}\u{641}\u{62A}\u{6CC}\u{645}", 1, 2, None),
    Among("\u{627}\u{645}", 0, 1, None),
    Among("\u{631}\u{641}\u{62A}\u{645}", 0, 2, None),
    Among("\u{627}\u{646}", -1, 3, None),
    Among("\u{62A}\u{647}", -1, 5, None),
    Among("\u{62F}\u{647}", -1, 4, None),
    Among("\u{646}\u{62F}\u{647}", 7, 3, None),
    Among("\u{631}\u{641}\u{62A}\u{6CC}", -1, 2, None),
    Among("\u{62F}", -1, 1, None),
    Among("\u{627}\u{646}\u{62F}", 10, 1, None),
    Among("\u{631}\u{641}\u{62A}\u{627}\u{646}\u{62F}", 11, 2, None),
    Among("\u{6CC}\u{62F}", 10, 1, None),
    Among("\u{631}\u{641}\u{62A}\u{6CC}\u{62F}", 13, 2, None),
];

fn r_Normalize_Characters(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            among_var = env.find_among(A_0, context);
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("\u{6A9}");
                }
                2 => {
                    env.slice_from("\u{6CC}");
                }
                3 => {
                    env.slice_from("\u{647}");
                }
                4 => {
                    env.slice_from("\u{627}");
                }
                5 => {
                    env.slice_from("\u{648}");
                }
                6 => {
                    env.slice_del();
                }
                7 => {
                    if env.cursor >= env.limit {
                        break 'lab1;
                    }
                    env.next_char();
                }
                _ => ()
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true
}

fn r_Prefixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if (env.cursor + 6 >= env.limit || (env.current.as_bytes()[(env.cursor + 6) as usize] as u8 != 140 as u8 && env.current.as_bytes()[(env.cursor + 6) as usize] as u8 != 226 as u8)) {
        return false;
    }

    among_var = env.find_among(A_1, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if !env.hop(2) {
                return false;
            }
            context.b_saw_present_prefix = true;
        }
        2 => {
            if !env.hop(2) {
                return false;
            }
            env.slice_del();
            context.b_saw_present_prefix = true;
        }
        _ => ()
    }
    return true
}

fn r_Delete_ZWNJ(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            'golab2: loop {
                let v_2 = env.cursor;
                'lab3: loop {
                    env.bra = env.cursor;
                    if !env.eq_s(&"\u{200C}") {
                        break 'lab3;
                    }
                    env.ket = env.cursor;
                    env.slice_del();
                    env.cursor = v_2;
                    break 'golab2;
                }
                env.cursor = v_2;
                if env.cursor >= env.limit {
                    break 'lab1;
                }
                env.next_char();
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_Protect_Lexical_AN(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        if !r_AN_Exception(env, context) {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab1: loop {
        if (env.cursor - 5 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 134 as u8) {
            break 'lab1;
        }

        if env.find_among_b(A_2, context) == 0 {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    return true
}

fn r_AN_Exception(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if (env.cursor - 7 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 134 as u8) {
        return false;
    }

    if env.find_among_b(A_3, context) == 0 {
        return false;
    }
    if env.cursor > env.limit_backward {
        return false;
    }
    return true
}

fn r_Irregular_Noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 9 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 175 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 177 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("\u{62E}\u{628}\u{631}");
        }
        2 => {
            env.slice_from("\u{627}\u{633}\u{62A}\u{627}\u{62F}");
        }
        _ => ()
    }
    return true
}

fn r_Stem_Noun_or_Adjective(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !r_Irregular_Noun(env, context) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if env.cursor < context.i_p1 {
            return false;
        }
        let v_2 = env.limit_backward;
        env.limit_backward = context.i_p1;
        env.ket = env.cursor;
        among_var = env.find_among_b(A_5, context);
        if among_var == 0 {
            env.limit_backward = v_2;
            return false;
        }
        env.bra = env.cursor;
        match among_var {
            1 => {
                env.slice_del();
            }
            2 => {
                if env.cursor <= env.limit_backward {
                    env.limit_backward = v_2;
                    return false;
                }
                env.slice_del();
            }
            _ => ()
        }
        env.limit_backward = v_2;
        break 'lab0;
    }
    return true
}

fn r_Stem_Verb(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            env.ket = env.cursor;
            if env.find_among_b(A_6, context) == 0 {
                break 'lab1;
            }
            env.bra = env.cursor;
            if !r_R1(env, context) {
                break 'lab1;
            }
            env.slice_del();
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        env.ket = env.cursor;
        among_var = env.find_among_b(A_7, context);
        if among_var == 0 {
            return false;
        }
        env.bra = env.cursor;
        match among_var {
            1 => {
                if !context.b_remove_verb_person_endings {
                    return false;
                }
                if !r_R1(env, context) {
                    return false;
                }
                env.slice_del();
            }
            2 => {
                env.slice_from("\u{631}\u{641}\u{62A}");
            }
            3 => {
                if !r_R1(env, context) {
                    return false;
                }
                env.slice_del();
                context.b_remove_verb_person_endings = true;
            }
            4 => {
                if env.cursor <= env.limit_backward {
                    return false;
                }
                env.slice_from("\u{62F}");
                context.b_remove_verb_person_endings = true;
            }
            5 => {
                if env.cursor <= env.limit_backward {
                    return false;
                }
                env.slice_from("\u{62A}");
                context.b_remove_verb_person_endings = true;
            }
            _ => ()
        }
        break 'lab0;
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p1: 0,
        b_remove_verb_person_endings: false,
        b_saw_present_prefix: false,
    };
    context.b_saw_present_prefix = false;
    let v_1 = env.cursor;
    r_Normalize_Characters(env, context);
    env.cursor = v_1;
    let v_2 = env.cursor;
    r_Prefixes(env, context);
    env.cursor = v_2;
    let v_3 = env.cursor;
    r_Delete_ZWNJ(env, context);
    env.cursor = v_3;
    context.i_p1 = env.limit;
    let v_4 = env.cursor;
    'lab0: loop {
        if !env.hop(3) {
            break 'lab0;
        }
        context.i_p1 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_4;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    'replab1: loop{
        let v_5 = env.limit - env.cursor;
        'lab2: for _ in 0..1 {
            let v_6 = env.limit - env.cursor;
            context.b_remove_verb_person_endings = false;
            'lab3: loop {
                if !context.b_saw_present_prefix {
                    break 'lab3;
                }
                context.b_remove_verb_person_endings = true;
                break 'lab3;
            }
            if !r_Protect_Lexical_AN(env, context) {
                break 'lab2;
            }
            'lab4: loop {
                let v_7 = env.limit - env.cursor;
                'lab5: loop {
                    if !r_Stem_Noun_or_Adjective(env, context) {
                        break 'lab5;
                    }
                    break 'lab4;
                }
                env.cursor = env.limit - v_7;
                if !r_Stem_Verb(env, context) {
                    break 'lab2;
                }
                break 'lab4;
            }
            env.cursor = env.limit - v_6;
            continue 'replab1;
        }
        env.cursor = env.limit - v_5;
        break 'replab1;
    }
    env.cursor = env.limit_backward;
    return true
}
