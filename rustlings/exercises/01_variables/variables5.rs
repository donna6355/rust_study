fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3; //SHADOWING, assign new type to same var name
    println!("Number plus two is: {}", number + 2);
}
