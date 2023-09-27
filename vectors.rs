//Una manera común de declarar e inicializar un vector es con la macro vec!.
// Declare vector, initialize with three values
let three_nums = vec![15, 3, 46];
println!("Initial vector: {:?}", three_nums);  
  
// Declare vector, value = "0", length = 5
let zeroes = vec![0; 5];
println!("Zeroes: {:?}", zeroes);

//Los vectores también se pueden crear mediante el método Vec::new(). mut a vector with mut 
// Create empty vector, declare vector mutable so it can grow and shrink
let mut fruit = Vec::new();

// Online Rust compiler to run Rust program online
// Print "Hello World!" message

fn main() {

  let mut meats = Vec::new();
  
  meats.push("chiken");
  meats.push("fish");
  meats.push("cow");
  
  let mut fruit = Vec::new();
      // Push values onto end of vector, type changes from generic `T` to String
  fruit.push("Apple");
  fruit.push("Banana");
  fruit.push("Cherry");
  fruit.push("pineaple");
  fruit.push("chocolate");
  fruit.push("burger");
  println!("Fruits: {:?}", fruit);
  println!("Meats: {:?}", meats);
  println!("Pop off {:?}",fruit.pop());
  
  println!("Fruits: {:?}", fruit);
  
  // Declare vector, initialize with three values
let mut index_vec = vec![15, 3, 46];
let three = index_vec[1];
println!("Vector: {:?}, three = {}", index_vec, three);  

// Access vector with out-of-bounds index
let beyond = index_vec[2];
println!("{}", beyond);

  }