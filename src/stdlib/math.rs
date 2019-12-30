use crate::types::{Args, Res, Val};

pub fn add_0(_: Args) -> Res {
    Ok(Val::Int(0))
}

pub fn add_2(args: Args) -> Res {
    let a = args[0].unwrap_int()?;
    let b = args[1].unwrap_int()?;
    Ok(Val::Int(a + b))
}

pub fn add_varargs(args: Args) -> Res {
    args.into_iter().fold(
        Ok(Val::Int(0)),
        |acc, x| {
            let a = acc.unwrap();
            let a = a.unwrap_int()?;
            let x = x.unwrap_int()?;
            Ok(Val::Int(a + x))
        }
    )
}

pub fn sub_0(_: Args) -> Res {
    Ok(Val::Int(0))
}

pub fn sub_1(args: Args) -> Res {
    let a = args[0].unwrap_int()?;
    Ok(Val::Int(a * -1))
}

pub fn sub_2(args: Args) -> Res {
    let a = args[0].unwrap_int()?;
    let b = args[1].unwrap_int()?;
    Ok(Val::Int(a - b))
}

pub fn sub_varargs(args: Args) -> Res {
    let mut val = 0;
    for (i, x) in args.into_iter().enumerate() {
        let v = x.unwrap_int()?;
        if i == 0 {
            val = *v;
        } else {
            val = val - v;
        }
    }
    Ok(Val::Int(val))
}

pub fn mul_0(_: Args) -> Res {
    Ok(Val::Int(1))
}

pub fn mul_2(args: Args) -> Res {
    let a = args[0].unwrap_int()?;
    let b = args[1].unwrap_int()?;
    Ok(Val::Int(a * b))
}

pub fn mul_varargs(args: Args) -> Res {
    args.into_iter().fold(
        Ok(Val::Int(1)),
        |acc, x| {
            let a = acc.unwrap();
            let a = a.unwrap_int()?;
            let x = x.unwrap_int()?;
            Ok(Val::Int(a * x))
        }
    )
}

pub fn div_0(_: Args) -> Res {
    Ok(Val::Int(1))
}

pub fn div_2(args: Args) -> Res {
    let a = args[0].unwrap_int()?;
    let b = args[1].unwrap_int()?;
    Ok(Val::Int(a / b))
}

pub fn div_varargs(args: Args) -> Res {
    let mut val = 0;
    for (i, x) in args.into_iter().enumerate() {
        let v = x.unwrap_int()?;
        if i == 0 {
            val = *v;
        } else {
            val = val / v;
        }
    }
    Ok(Val::Int(val))
}

#[cfg(test)]
mod test {
    use crate::errors::ErrType;
    use crate::types::{Val};
    use crate::stdlib::math::*;

    #[test]
    fn test_add_2() -> Result<(), ErrType> {
        let res = add_2(vec![Val::Int(1), Val::Int(2)])?;
        assert_eq!(res, Val::Int(3));
        Ok(())
    }

    #[test]
    fn test_add_varargs() -> Result<(), ErrType> {
        let args = vec![Val::Int(1), Val::Int(2), Val::Int(3)];
        let res = add_varargs(args)?;
        assert_eq!(res, Val::Int(6));
        Ok(())
    }

    #[test]
    fn test_sub_1() -> Result<(), ErrType> {
        let args = vec![Val::Int(1)];
        let res = sub_1(args)?;
        assert_eq!(res, Val::Int(-1));
        Ok(())
    }

    #[test]
    fn test_sub_2() -> Result<(), ErrType> {
        let args = vec![Val::Int(3), Val::Int(1)];
        let res = sub_2(args)?;
        assert_eq!(res, Val::Int(2));
        Ok(())
    }

    #[test]
    fn test_sub_varargs() -> Result<(), ErrType> {
        let args = vec![Val::Int(10), Val::Int(5), Val::Int(2)];
        let res = sub_varargs(args)?;
        assert_eq!(res, Val::Int(3));
        Ok(())
    }

    #[test]
    fn test_mul_2() -> Result<(), ErrType> {
        let args = vec![Val::Int(2), Val::Int(3)];
        let res = mul_2(args)?;
        assert_eq!(res, Val::Int(6));
        Ok(())
    }

    #[test]
    fn test_mul_varargs() -> Result<(), ErrType> {
        let args = vec![Val::Int(2), Val::Int(3), Val::Int(4)];
        let res = mul_varargs(args)?;
        assert_eq!(res, Val::Int(24));
        Ok(())
    }

    #[test]
    fn test_div_2() -> Result<(), ErrType> {
        let args = vec![Val::Int(6), Val::Int(2)];
        let res = div_2(args)?;
        assert_eq!(res, Val::Int(3));
        Ok(())
    }

    #[test]
    fn test_div_varargs() -> Result<(), ErrType> {
        let args = vec![Val::Int(12), Val::Int(2), Val::Int(3)];
        let res = div_varargs(args)?;
        assert_eq!(res, Val::Int(2));
        Ok(())
    }
}
