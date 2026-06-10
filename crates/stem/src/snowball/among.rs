use crate::snowball::SnowballEnv;

pub type SomeType<'a, T> = Option<&'a (dyn Fn(&mut SnowballEnv<'_>, &mut T) -> bool + Sync)>;

pub struct Among<T: 'static>(pub &'static str, pub i32, pub i32, pub SomeType<'static, T>);
