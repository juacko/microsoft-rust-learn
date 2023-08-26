fn main() {

  let mut a_number =10;
  println!("The number is{}.", a_number);
  let a_word = "Ten";

  a_number = 15;
  
  println!("The number is {}.", a_number);
  println!("The word is {}.", a_word);
  
  let shadow-number = 5;
  let shadow-number = shadow-number + 5;
  let shadow-number = shadow-number * 2;

  println!("The shadow number is {}.", shadow-number);

  let number: u32 = 14;
  println!("The number is {}.", number);

  //define tuple with 3 length
  let tuple_e = ('E', 5i32, true);
  
  //Use the tuple indexing  and show the values of the elements
  println!("Is '{}' the {}th  letter  of the alphabet ? {}", tuple_e.0,tuple_e.1,tuple_e.2);

}



