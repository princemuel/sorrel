//! Generated from nepali.sbl by Snowball 3.1.1 - https://snowballstem.org/

#![expect(non_snake_case)]
#![expect(non_upper_case_globals)]
#![expect(unused_mut)]
#![expect(unused_parens)]
#![expect(unused_variables)]
use crate::snowball::{Among, SnowballEnv};

#[derive(Clone)]
struct Context {}

static A_0: &'static [Among<Context>; 17] = &[
    Among("\u{915}\u{940}", -1, 2, None),
    Among("\u{932}\u{93E}\u{907}", -1, 1, None),
    Among("\u{932}\u{947}", -1, 1, None),
    Among("\u{932}\u{93E}\u{908}", -1, 1, None),
    Among("\u{915}\u{948}", -1, 2, None),
    Among("\u{938}\u{901}\u{917}\u{948}", -1, 1, None),
    Among("\u{92E}\u{948}", -1, 1, None),
    Among("\u{915}\u{94B}", -1, 2, None),
    Among("\u{938}\u{901}\u{917}", -1, 1, None),
    Among("\u{938}\u{902}\u{917}", -1, 1, None),
    Among("\u{92E}\u{93E}\u{930}\u{94D}\u{92B}\u{924}", -1, 1, None),
    Among("\u{930}\u{924}", -1, 1, None),
    Among("\u{915}\u{93E}", -1, 2, None),
    Among("\u{92E}\u{93E}", -1, 1, None),
    Among("\u{926}\u{94D}\u{935}\u{93E}\u{930}\u{93E}", -1, 1, None),
    Among("\u{915}\u{93F}", -1, 2, None),
    Among("\u{92A}\u{91B}\u{93F}", -1, 1, None),
];

static A_1: &'static [Among<Context>; 3] =
    &[Among("\u{901}", -1, 1, None), Among("\u{902}", -1, 1, None), Among("\u{948}", -1, 2, None)];

static A_2: &'static [Among<Context>; 91] = &[
    Among("\u{947}\u{915}\u{940}", -1, 1, None),
    Among("\u{90F}\u{915}\u{940}", -1, 1, None),
    Among("\u{907}\u{90F}\u{915}\u{940}", 1, 1, None),
    Among("\u{93F}\u{90F}\u{915}\u{940}", 1, 1, None),
    Among("\u{926}\u{947}\u{916}\u{940}", -1, 1, None),
    Among("\u{925}\u{940}", -1, 1, None),
    Among("\u{926}\u{940}", -1, 1, None),
    Among("\u{91B}\u{941}", -1, 1, None),
    Among("\u{947}\u{91B}\u{941}", 7, 1, None),
    Among("\u{928}\u{947}\u{91B}\u{941}", 8, 1, None),
    Among("\u{90F}\u{91B}\u{941}", 7, 1, None),
    Among("\u{928}\u{941}", -1, 1, None),
    Among("\u{939}\u{930}\u{941}", -1, 1, None),
    Among("\u{939}\u{930}\u{942}", -1, 1, None),
    Among("\u{91B}\u{947}", -1, 1, None),
    Among("\u{925}\u{947}", -1, 1, None),
    Among("\u{928}\u{947}", -1, 1, None),
    Among("\u{947}\u{915}\u{948}", -1, 1, None),
    Among("\u{928}\u{947}\u{915}\u{948}", 17, 1, None),
    Among("\u{90F}\u{915}\u{948}", -1, 1, None),
    Among("\u{926}\u{948}", -1, 1, None),
    Among("\u{907}\u{926}\u{948}", 20, 1, None),
    Among("\u{93F}\u{926}\u{948}", 20, 1, None),
    Among("\u{947}\u{915}\u{94B}", -1, 1, None),
    Among("\u{928}\u{947}\u{915}\u{94B}", 23, 1, None),
    Among("\u{90F}\u{915}\u{94B}", -1, 1, None),
    Among("\u{907}\u{90F}\u{915}\u{94B}", 25, 1, None),
    Among("\u{93F}\u{90F}\u{915}\u{94B}", 25, 1, None),
    Among("\u{926}\u{94B}", -1, 1, None),
    Among("\u{907}\u{926}\u{94B}", 28, 1, None),
    Among("\u{93F}\u{926}\u{94B}", 28, 1, None),
    Among("\u{92F}\u{94B}", -1, 1, None),
    Among("\u{907}\u{92F}\u{94B}", 31, 1, None),
    Among("\u{925}\u{94D}\u{92F}\u{94B}", 31, 1, None),
    Among("\u{92D}\u{92F}\u{94B}", 31, 1, None),
    Among("\u{93F}\u{92F}\u{94B}", 31, 1, None),
    Among("\u{925}\u{93F}\u{92F}\u{94B}", 35, 1, None),
    Among("\u{926}\u{93F}\u{92F}\u{94B}", 35, 1, None),
    Among("\u{91B}\u{94C}", -1, 1, None),
    Among("\u{907}\u{91B}\u{94C}", 38, 1, None),
    Among("\u{947}\u{91B}\u{94C}", 38, 1, None),
    Among("\u{928}\u{947}\u{91B}\u{94C}", 40, 1, None),
    Among("\u{90F}\u{91B}\u{94C}", 38, 1, None),
    Among("\u{93F}\u{91B}\u{94C}", 38, 1, None),
    Among("\u{92F}\u{94C}", -1, 1, None),
    Among("\u{91B}\u{94D}\u{92F}\u{94C}", 44, 1, None),
    Among("\u{925}\u{94D}\u{92F}\u{94C}", 44, 1, None),
    Among("\u{925}\u{93F}\u{92F}\u{94C}", 44, 1, None),
    Among("\u{91B}\u{928}\u{94D}", -1, 1, None),
    Among("\u{907}\u{91B}\u{928}\u{94D}", 48, 1, None),
    Among("\u{947}\u{91B}\u{928}\u{94D}", 48, 1, None),
    Among("\u{928}\u{947}\u{91B}\u{928}\u{94D}", 50, 1, None),
    Among("\u{90F}\u{91B}\u{928}\u{94D}", 48, 1, None),
    Among("\u{93F}\u{91B}\u{928}\u{94D}", 48, 1, None),
    Among("\u{932}\u{93E}\u{928}\u{94D}", -1, 1, None),
    Among("\u{91B}\u{93F}\u{928}\u{94D}", -1, 1, None),
    Among("\u{925}\u{93F}\u{928}\u{94D}", -1, 1, None),
    Among("\u{92A}\u{930}\u{94D}", -1, 1, None),
    Among("\u{907}\u{938}\u{94D}", -1, 1, None),
    Among("\u{925}\u{93F}\u{907}\u{938}\u{94D}", 58, 1, None),
    Among("\u{91B}\u{947}\u{938}\u{94D}", -1, 1, None),
    Among("\u{939}\u{94B}\u{938}\u{94D}", -1, 1, None),
    Among("\u{91B}\u{938}\u{94D}", -1, 1, None),
    Among("\u{907}\u{91B}\u{938}\u{94D}", 62, 1, None),
    Among("\u{947}\u{91B}\u{938}\u{94D}", 62, 1, None),
    Among("\u{928}\u{947}\u{91B}\u{938}\u{94D}", 64, 1, None),
    Among("\u{90F}\u{91B}\u{938}\u{94D}", 62, 1, None),
    Among("\u{93F}\u{91B}\u{938}\u{94D}", 62, 1, None),
    Among("\u{93F}\u{938}\u{94D}", -1, 1, None),
    Among("\u{925}\u{93F}\u{938}\u{94D}", 68, 1, None),
    Among("\u{925}\u{93F}\u{90F}", -1, 1, None),
    Among("\u{91B}", -1, 1, None),
    Among("\u{907}\u{91B}", 71, 1, None),
    Among("\u{947}\u{91B}", 71, 1, None),
    Among("\u{928}\u{947}\u{91B}", 73, 1, None),
    Among("\u{939}\u{941}\u{928}\u{947}\u{91B}", 74, 1, None),
    Among("\u{939}\u{941}\u{928}\u{94D}\u{91B}", 71, 1, None),
    Among("\u{907}\u{928}\u{94D}\u{91B}", 71, 1, None),
    Among("\u{93F}\u{928}\u{94D}\u{91B}", 71, 1, None),
    Among("\u{90F}\u{91B}", 71, 1, None),
    Among("\u{93F}\u{91B}", 71, 1, None),
    Among("\u{947}\u{915}\u{93E}", -1, 1, None),
    Among("\u{928}\u{947}\u{915}\u{93E}", 81, 1, None),
    Among("\u{90F}\u{915}\u{93E}", -1, 1, None),
    Among("\u{907}\u{90F}\u{915}\u{93E}", 83, 1, None),
    Among("\u{93F}\u{90F}\u{915}\u{93E}", 83, 1, None),
    Among("\u{926}\u{93E}", -1, 1, None),
    Among("\u{907}\u{926}\u{93E}", 86, 1, None),
    Among("\u{93F}\u{926}\u{93E}", 86, 1, None),
    Among("\u{926}\u{947}\u{916}\u{93F}", -1, 1, None),
    Among("\u{92E}\u{93E}\u{925}\u{93F}", -1, 1, None),
];

fn r_remove_category_1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => 'lab0: loop {
            'lab1: loop {
                if !env.eq_s_b(&"\u{90F}") {
                    break 'lab1;
                }
                break 'lab0;
            }
            'lab2: loop {
                if !env.eq_s_b(&"\u{947}") {
                    break 'lab2;
                }
                break 'lab0;
            }
            env.slice_del();
            break 'lab0;
        },
        _ => (),
    }
    return true;
}

fn r_remove_category_2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward
        || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 4 as u8
        || ((262 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1)
            == 0)
    {
        return false;
    }

    among_var = env.find_among_b(A_1, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            'lab0: loop {
                'lab1: loop {
                    if !env.eq_s_b(&"\u{92F}\u{94C}") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                'lab2: loop {
                    if !env.eq_s_b(&"\u{91B}\u{94C}") {
                        break 'lab2;
                    }
                    break 'lab0;
                }
                'lab3: loop {
                    if !env.eq_s_b(&"\u{928}\u{94C}") {
                        break 'lab3;
                    }
                    break 'lab0;
                }
                if !env.eq_s_b(&"\u{925}\u{947}") {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        2 => {
            if !env.eq_s_b(&"\u{924}\u{94D}\u{930}") {
                return false;
            }
            env.slice_del();
        }
        _ => (),
    }
    return true;
}

fn r_remove_category_3(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_2, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {};
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    r_remove_category_1(env, context);
    env.cursor = env.limit - v_1;
    'replab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: for _ in 0..1 {
            let v_3 = env.limit - env.cursor;
            r_remove_category_2(env, context);
            env.cursor = env.limit - v_3;
            if !r_remove_category_3(env, context) {
                break 'lab1;
            }
            continue 'replab0;
        }
        env.cursor = env.limit - v_2;
        break 'replab0;
    }
    env.cursor = env.limit_backward;
    return true;
}
