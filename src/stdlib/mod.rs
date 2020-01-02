pub mod list;
pub mod math;


pub mod core {
    use crate::env::{Arity, Env};
    use super::list;
    use super::math;

    pub fn core() -> Env<'static> {
        let mut e = Env::new(None);
        e.register_func("+", Arity::NoArgs, math::add_0);
        e.register_func("+", Arity::SomeArgs(2), math::add_2);
        e.register_func("+", Arity::VarArgs, math::add_varargs);
        e.register_func("-", Arity::NoArgs, math::sub_0);
        e.register_func("-", Arity::SomeArgs(1), math::sub_1);
        e.register_func("-", Arity::SomeArgs(2), math::sub_2);
        e.register_func("-", Arity::VarArgs, math::sub_varargs);
        e.register_func("*", Arity::NoArgs, math::mul_0);
        e.register_func("*", Arity::SomeArgs(2), math::mul_2);
        e.register_func("*", Arity::VarArgs, math::mul_varargs);
        e.register_func("/", Arity::NoArgs, math::div_0);
        e.register_func("/", Arity::SomeArgs(2), math::div_2);
        e.register_func("/", Arity::VarArgs, math::div_varargs);
        e.register_func("car", Arity::SomeArgs(1), list::car);
        e.register_func("cdr", Arity::SomeArgs(1), list::cdr);
        e.register_func("list", Arity::VarArgs, list::list);
        e
    }
}
