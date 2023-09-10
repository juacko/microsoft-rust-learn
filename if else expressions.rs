fn main() {
  if 1==1 {
  println!("True, the numbers are equal."); // 
} else {
  println!("False, the numbers are not equal.");
}

//A diferencia de la mayoría de los demás lenguajes, los bloques de if en Rust también pueden actuar como expresiones. Todos los bloques de ejecución de las ramas de condición deben devolver el mismo tipo para que se compile el código.
let formal = true;
let greeting = if formal { // if used here as an expression
  "Good day to you."     // return a String
} else {
  "Hey!"                 // return a String
};
println!("{}", greeting)   // prints "Good day to you."

  let num = 500; // num variable can be set at some point in the program
  let out_of_range: bool;
  if num < 0 {
      out_of_range = true;
  } else if num == 0 {
      out_of_range = true;
  } else if num > 512 {
      out_of_range = true;
  } else {
      out_of_range = false;
  }
}

