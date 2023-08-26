// Classic struct with named fields
struct Student {
  name: String,
  level: u8,
  remote: bool
}

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

fn main() {
  // Instantiate a classic struct, specify the fields in random order
  let student_1 = Student {
      name: String::from("Constance Sharma"),
      remote: true,
      level: 2
  };

  // Instantiate a tuple struct, pass the values in the same order as the types are defined
  let grades_1 = Grades('A', 'A', 'B', 'A', 3.75);

  // Instantiate another classic struct, specify the field values in order
  let student_2 = Student {
      name: String::from("Dyson Tan"),
      level: 5,
      remote: false
  };

  // Instantiate another tuple struct, pass the values in the same order as defined
  let grades_2 = Grades('B', 'A', 'A', 'C', 3.25);

  // Show the student information
  println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
           student_1.name, student_1.level, student_1.remote, grades_1.0, grades_1.1, grades_1.2, grades_1.3, grades_1.4);

  println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
           student_2.name, student_2.level, student_2.remote, grades_2.0, grades_2.1, grades_2.2, grades_2.3, grades_2.4);
  
}