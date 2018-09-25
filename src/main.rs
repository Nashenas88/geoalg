extern crate geoalg;

use geoalg::{Multivector, Var};

fn main() {
    let rotor = Multivector::from_rotor('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H');
    let vector = Multivector::from_vector(Var::a(), Var::b(), Var::c(), Var::d());

    println!("r = {}", rotor);
    println!("v = {}", vector);
    println!("v' = r⁻¹vr");
    let ri = rotor.reverse();
    println!("r⁻¹ = {}", ri);

    println!();

    let mut multiplied = &ri * &vector;
    println!("v' = {} {}", multiplied, rotor);
    multiplied.simplify();
    println!("simplified:\n{} {}", multiplied, rotor);

    println!();

    let mut finished = &multiplied * &rotor;
    println!("v' = {}", finished);
    finished.simplify();

    println!("\nsimplified:\nv' = {}", finished);
    println!("--\nsuper simplified:\nv' = {}", finished.into_final());

    let rotor2 = Multivector::from_rotor('Z', 'Y', 'X', 'W', 'V', 'U', 'T', 'S');
    let mut mult = &rotor * &rotor2;
    println!("m = {}", mult);
    mult.simplify();
    println!("m = {}", mult);
    println!("m =\n{}", mult.into_final());
}
