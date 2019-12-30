use crate::types::{Args, Res, Val};

pub fn car(args: Args) -> Res {
    let l = args[0].unwrap_list()?;
    if l.len() > 0 {
        Ok(l[0].clone())
    } else {
        Ok(Val::List(Vec::new()))
    }
}

pub fn cdr(args: Args) -> Res {
    let l = args[0].unwrap_list()?;
    if l.len() > 1 {
        let mut l2 = Vec::new();
        for v in l[1..].iter() {
            l2.push(v.clone());
        }
        Ok(Val::List(l2))
    } else {
        Ok(Val::List(Vec::new()))
    }
}

pub fn list(args: Args) -> Res {
    let mut l = Vec::new();
    for v in args.iter() {
        l.push(v.clone());
    }
    Ok(Val::List(l))
}

#[cfg(test)]
mod tests {
    use crate::errors::ErrType;
    use crate::types::Val;
    use crate::stdlib::list::*;

    #[test]
    fn test_car() -> Result<(), ErrType> {
        let a = Val::Char('a');
        let b = Val::Int(1);
        let l = Val::List(vec![a, b]);
        let res = car(vec![l])?;
        assert_eq!(res, Val::Char('a'));
        Ok(())
    }

    #[test]
    fn test_car_short() -> Result<(), ErrType> {
        let l = Val::List(Vec::new());
        let res = car(vec![l])?;
        assert_eq!(res, Val::List(Vec::new()));
        Ok(())
    }

    #[test]
    fn test_cdr() -> Result<(), ErrType> {
        let a = Val::Char('a');
        let b = Val::Char('b');
        let c = Val::Char('c');
        let l = Val::List(vec![a, b, c]);
        let expected = Val::List(vec![Val::Char('b'), Val::Char('c')]);
        let res = cdr(vec![l])?;
        assert_eq!(res, expected);
        Ok(())
    }

    #[test]
    fn test_cdr_short() -> Result<(), ErrType> {
        let l = Val::List(Vec::new());
        let res = cdr(vec![l])?;
        assert_eq!(res, Val::List(Vec::new()));
        Ok(())
    }

    #[test]
    fn test_list() -> Result<(), ErrType> {
        let l = vec![Val::Char('a'), Val::Char('b')];
        let l2 = list(l)?;
        let expected = Val::List(vec![Val::Char('a'), Val::Char('b')]);
        assert_eq!(l2, expected);
        Ok(())
    }
}
