extern crate geoalg;

use geoalg::{Multivector, Var};

fn main() {
    let rotor = Multivector::from_rotor('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H');
    let vector = Multivector::from_vector(Var::a(), Var::b(), Var::c(), Var::d());

    println!("{}", rotor);
    println!("{}", vector);

    let ri = rotor.inverse();

    let mut multiplied = &ri * &vector;
    println!("{}", multiplied);
    multiplied.simplify();
    println!("{}", multiplied);

    let mut finished = &multiplied * &rotor;
    println!("{}", finished);
    finished.simplify();

    println!("\n{}", finished);
    println!("--\n{}", finished.into_final());

    // let rotor = Rotor {
    //     params: vec![
    //         Param {
    //             var_params: vec![VarParam::from_scalar('A')],
    //             bases: vec![],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('B')],
    //             bases: vec![Base(1), Base(2)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('C')],
    //             bases: vec![Base(1), Base(3)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('D')],
    //             bases: vec![Base(1), Base(4)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('E')],
    //             bases: vec![Base(2), Base(3)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('F')],
    //             bases: vec![Base(2), Base(4)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('G')],
    //             bases: vec![Base(3), Base(4)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('H')],
    //             bases: vec![Base(1), Base(2), Base(3), Base(4)],
    //         }
    //     ]
    // };

    // let rotor2 = Rotor {
    //     params: vec![
    //         Param {
    //             var_params: vec![VarParam::from_scalar('Z')],
    //             bases: vec![],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('Y')],
    //             bases: vec![Base(1), Base(2)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('X')],
    //             bases: vec![Base(2), Base(3)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('W')],
    //             bases: vec![Base(1), Base(3)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('V')],
    //             bases: vec![Base(1), Base(4)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('U')],
    //             bases: vec![Base(2), Base(4)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('T')],
    //             bases: vec![Base(3), Base(4)],
    //         },
    //         Param {
    //             var_params: vec![VarParam::from_scalar('S')],
    //             bases: vec![Base(1), Base(2), Base(3), Base(4)],
    //         }
    //     ]
    // };

    // let mut mult = rotor * rotor2;
    // println!("{}", mult);
    // mult.simplify();
    // println!("{}", mult);
    // println!("{}", mult.into_final());
}
