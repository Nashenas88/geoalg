use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    fmt::{self, Debug, Display},
    ops::Mul,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Base(u8);

impl Base {
    pub fn e1() -> Self {
        Base(1)
    }
    pub fn e2() -> Self {
        Base(2)
    }
    pub fn e3() -> Self {
        Base(3)
    }
    pub fn e4() -> Self {
        Base(4)
    }
}

macro_rules! base_fmt {
    ($t:ty) => {
        impl $t for Base {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "{}", match self {
                    Base(1) => "e₁",
                    Base(2) => "e₂",
                    Base(3) => "e₃",
                    Base(4) => "e₄",
                    v => panic!("Unexpected var value: {}", v),
                })
            }
        }
    };
}

base_fmt!(Display);
base_fmt!(Debug);

#[derive(Eq, Ord)]
struct Bases(Vec<Base>);

impl Bases {
    fn simplify(self) -> (bool, Vec<Base>) {
        let mut v = self.0;
        if v.len() <= 1 {
            return (false, v);
        }

        let mut moves = 0;

        let mut i = 0;
        while i < v.len() - 1 {
            let mut j = 0;
            while j < v.len() - i - 1 {
                let vj1 = v[j];
                let vj2 = v[j + 1];
                if vj1 > vj2 {
                    v[j] = vj2;
                    v[j + 1] = vj1;
                    moves += 1;
                }

                j += 1;
            }

            i += 1;
        }

        i = 0;
        while i < v.len() - 1 {
            if v[i] == v[i + 1] {
                v.remove(i);
                v.remove(i);

                if i >= v.len() {
                    break;
                }

                continue;
            }

            i += 1;
        }

        (moves % 2 != 0, v)
    }
}

impl PartialEq for Bases {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for (&l, &r) in self.0.iter().zip(other.0.iter()) {
            if l != r {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for Bases {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.len() < other.0.len() {
            return Some(Ordering::Less);
        } else if self.0.len() > other.0.len() {
            return Some(Ordering::Greater);
        }

        for (&l, &r) in self.0.iter().zip(other.0.iter()) {
            if l < r {
                return Some(Ordering::Less);
            } else if l > r {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Equal)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Var(u8);

impl Var {
    pub fn a() -> Self {
        Var(1)
    }
    pub fn b() -> Self {
        Var(2)
    }
    pub fn c() -> Self {
        Var(3)
    }
    pub fn d() -> Self {
        Var(4)
    }
}

macro_rules! var_fmt {
    ($t:ty) => {
        impl $t for Var {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "{}", match self {
                    Var(1) => "a",
                    Var(2) => "b",
                    Var(3) => "c",
                    Var(4) => "d",
                    v => panic!("Unexpected var value: {}", v),
                })
            }
        }
    };
}

var_fmt!(Display);
var_fmt!(Debug);

#[derive(Clone, Debug)]
pub struct VarParam {
    negative: bool,
    scalars: Vec<char>,
    var: Option<Var>,
}

impl VarParam {
    pub fn from_scalar(scalar: char) -> Self {
        Self {
            negative: false,
            scalars: vec![scalar],
            var: None,
        }
    }

    pub fn from_var(var: Var) -> Self {
        Self {
            negative: false,
            scalars: vec![],
            var: Some(var),
        }
    }

    fn simplify(&mut self) {
        self.scalars.sort();
    }
}

impl<'a> Mul<&'a VarParam> for &'a VarParam {
    type Output = VarParam;

    fn mul(self, other: &VarParam) -> Self::Output {
        let var = match (self.var, other.var) {
            (v @ Some(_), None)
            | (None, v @ Some(_)) => v,
            (Some(v1), Some(v2)) if v1 == v2 => Some(v1),
            (None, None) => None,
            (Some(v1), Some(v2)) => panic!("Did not expect to mult two different vars: {} {}", v1, v2),
        };

        VarParam {
            negative: self.negative ^ other.negative,
            scalars: self.scalars.iter().cloned()
                .chain(other.scalars.iter().cloned())
                .collect(),
            var,
        }
    }
}

impl Display for VarParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negative {
            write!(f, "-")?;
        }

        for scalar in &self.scalars {
            write!(f, "{}", scalar)?;
        }

        if let Some(v) = self.var {
            write!(f, "{:?}", v)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Param {
    var_params: Vec<VarParam>,
    bases: Vec<Base>,
}

impl Param {
    pub fn new(vp: VarParam, bases: Vec<Base>) -> Self {
        Self {
            var_params: vec![vp],
            bases,
        }
    }

    fn simplify(&mut self) {
        // println!("simplifying: {}", self);
        let mut bases = vec![];
        std::mem::swap(&mut self.bases, &mut bases);
        let (negative, bases) = Bases(bases).simplify();
        self.bases = bases;

        for var_param in &mut self.var_params {
            var_param.negative = var_param.negative ^ negative;
            var_param.simplify();
        }

        self.var_params.sort_by_key(|vp| vp.var);
        // println!(" -> {}", self);
    }

    fn compress(&mut self) {
        self.var_params.sort_by(|lvp, rvp| {
            match <Option<Var> as Ord>::cmp(&lvp.var, &rvp.var) {
                Ordering::Equal => {},
                o => return o,
            }

            for (ls, rs) in lvp.scalars.iter().zip(rvp.scalars.iter()) {
                match <char as Ord>::cmp(ls, rs) {
                    Ordering::Equal => {},
                    o => return o,
                }
            }

            Ordering::Equal
        });
    }

    fn pow(n: u8) -> char {
        match n {
            0 => '⁰',
            1 => '¹',
            2 => '²',
            3 => '³',
            4 => '⁴',
            5 => '⁵',
            6 => '⁶',
            7 => '⁷',
            8 => '⁸',
            9 => '⁹',
            _ => panic!("Only supporting up to 9: {}", n),
        }
    }

    fn get_scalars(vp: &mut VarParam, mult: &mut u8) -> String {
        let mut fin = String::new();

        let mut j = 0;
        let mut pow = 1;
        let mut unprinted = false;
        while j < vp.scalars.len() - 1 {
            let j1 = vp.scalars[j];
            let j2 = vp.scalars[j + 1];
            if j1 == j2 {
                pow += 1;
                vp.scalars.remove(j + 1);
                unprinted = true;
                continue;
            }

            unprinted = false;

            if vp.negative {
                fin += &format!(" - ");
            } else {
                fin += &format!(" + ");
            }

            if *mult != 1 {
                fin += &format!("{}", mult);
                *mult = 1;
            }

            fin += &format!("{}", j1);

            if pow > 1 {
                fin += &format!("{}", Self::pow(pow));
                pow = 1;
            }

            j += 1;
        }
        if j < vp.scalars.len() {
            if unprinted {
                if vp.negative {
                    fin += &format!(" - ");
                } else {
                    fin += &format!(" + ");
                }
            }

            fin += &format!("{}", vp.scalars[j]);

            if pow > 1 {
                fin += &format!("{}", Self::pow(pow));
            }
        }

        fin
    }

    fn into_final(mut self) -> String {
        let mut i = 0;
        let mut fin = String::new();
        fin += "((";

        let mut mult = 1;
        while i < self.var_params.len() - 1 {
            let mut vp1 = self.var_params[i].clone();
            let vp2 = self.var_params[i + 1].clone();
            if vp1.var == vp2.var && vp1.scalars == vp2.scalars {
                if vp1.negative == vp2.negative {
                    mult *= 2;
                }

                self.var_params.remove(i + 1);
                continue;
            }

            fin += &Self::get_scalars(&mut vp1, &mut mult);

            if vp1.var != vp2.var {
                fin += ")";
                if let Some(v) = vp1.var {
                    fin += &format!("{}", v);
                }
                fin += " + (";
            }

            i += 1;
        }

        if i < self.var_params.len() {
            fin += &Self::get_scalars(&mut self.var_params[i], &mut mult);
        }
        fin += ")";
        for b in &self.bases {
            fin += &format!("{}", b);
        }

        fin
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.var_params.len() > 1 {
            write!(f, "(")?;
        }

        write!(f, "{}", self.var_params[0])?;
        for var_param in self.var_params.iter().skip(1) {
            write!(f, " + {}", var_param)?;
        }
        if self.var_params.len() > 1 {
            write!(f, ")")?;
        }

        for base in &self.bases {
            write!(f, "{:?}", base)?;
        }

        Ok(())
    }
}

impl<'a> Mul<&'a Param> for &'a Param {
    type Output = Param;

    fn mul(self, other: &Param) -> Self::Output {
        let mut var_params = vec![];

        for rp in &self.var_params {
            for lp in &other.var_params {
                var_params.push(rp * lp);
            }
        }

        Param {
            var_params,
            bases: self.bases.iter().cloned()
                .chain(other.bases.iter().cloned())
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Multivector {
    params: Vec<Param>,
}

impl Multivector {
    pub fn from_rotor(a: char, b: char, c: char, d: char, e: char, f: char, g: char, h: char) -> Self {
        Self {
            params: vec![
                Param::new(VarParam::from_scalar(a), vec![]),
                Param::new(VarParam::from_scalar(b), vec![Base::e1(), Base::e2()]),
                Param::new(VarParam::from_scalar(c), vec![Base::e1(), Base::e3()]),
                Param::new(VarParam::from_scalar(d), vec![Base::e1(), Base::e4()]),
                Param::new(VarParam::from_scalar(e), vec![Base::e2(), Base::e3()]),
                Param::new(VarParam::from_scalar(f), vec![Base::e2(), Base::e4()]),
                Param::new(VarParam::from_scalar(g), vec![Base::e3(), Base::e4()]),
                Param::new(VarParam::from_scalar(h), vec![Base::e1(), Base::e2(), Base::e3(), Base::e4()])
            ]
        }
    }

    pub fn from_vector(a: Var, b: Var, c: Var, d: Var) -> Self {
        Self {
            params: vec![
                Param {
                    var_params: vec![VarParam::from_var(a)],
                    bases: vec![Base::e1()],
                },
                Param {
                    var_params: vec![VarParam::from_var(b)],
                    bases: vec![Base::e2()],
                },
                Param {
                    var_params: vec![VarParam::from_var(c)],
                    bases: vec![Base::e3()],
                },
                Param {
                    var_params: vec![VarParam::from_var(d)],
                    bases: vec![Base::e4()],
                },
            ]
        }
    }

    pub fn simplify(&mut self) {
        for param in &mut self.params {
            param.simplify();
        }

        self.params.sort_by_key(|p| Bases(p.bases.clone()));
        self.compress();
    }

    pub fn compress(&mut self) {
        let mut i = 0;
        while i < self.params.len() - 1 {
            let lb = Bases(self.params[i].bases.clone());
            let rb = Bases(self.params[i + 1].bases.clone());

            if lb == rb {
                let mut rps = vec![];
                std::mem::swap(&mut self.params[i + 1].var_params, &mut rps);
                self.params[i].var_params.append(&mut rps);
                self.params.remove(i + 1);

                continue;
            }

            i += 1;
        }

        for param in &mut self.params {
            param.compress();
        }
    }

    pub fn reverse(&self) -> Self {
        let mut rotor = self.clone();

        for param in rotor.params.iter_mut().skip(1) {
            if param.bases.len() != 2 {
                continue;
            }

            for var_param in param.var_params.iter_mut() {
                var_param.negative = !var_param.negative;
            }
        }

        rotor
    }

    pub fn into_final(self) -> String {
        let mut fin = String::new();

        for param in self.params.into_iter() {
            fin += &format!("{}\n", param.into_final());
        }

        fin
    }
}

impl Display for Multivector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.params.len() == 0 || self.params.iter().map(|p| p.var_params.len()).sum::<usize>() == 0 {
            return Ok(());
        }

        let write_param = |f: &mut fmt::Formatter, param: &Param| -> fmt::Result {
            if param.var_params.len() > 1 {
                write!(f, "(")?;
            }

            let first_var_param = &param.var_params[0];

            if first_var_param.negative {
                write!(f, "-")?;
            }

            let write_var_param = |f: &mut fmt::Formatter, var_param: &VarParam| -> fmt::Result {
                for s in &var_param.scalars {
                    write!(f, "{}", s)?;
                }

                if let Some(v) = var_param.var {
                    write!(f, "{}", v)?;
                }

                Ok(())
            };

            write_var_param(f, first_var_param)?;

            for var_param in param.var_params.iter().skip(1) {
                write!(f, " ")?;
                if var_param.negative {
                    write!(f, "-")?;
                } else {
                    write!(f, "+")?;
                }
                write!(f, " ")?;

                write_var_param(f, var_param)?;
            }

            if param.var_params.len() > 1 {
                write!(f, ")")?;
            }

            for base in &param.bases {
                write!(f, "{}", base)?;
            }

            Ok(())
        };

        write!(f, "[")?;
        write_param(f, &self.params[0])?;
        for param in self.params.iter().skip(1) {
            if param.var_params.len() == 0 {
                continue;
            }

            write!(f, " + ")?;
            write_param(f, param)?;
        }
        write!(f, "]")
    }
}

impl Mul<Multivector> for Multivector {
    type Output = Multivector;
    fn mul(self, other: Multivector) -> Self::Output {
        let mut params = vec![];

        for rp in &self.params {
            for lp in &other.params {
                params.push(rp * lp);
            }
        }

        Multivector {
            params,
        }
    }
}

impl<'a> Mul<&'a Multivector> for &'a Multivector {
    type Output = Multivector;
    fn mul(self, other: &Multivector) -> Self::Output {
        let mut params = vec![];

        for rp in &self.params {
            for lp in &other.params {
                params.push(rp * lp);
            }
        }

        Multivector {
            params,
        }
    }
}
