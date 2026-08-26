/*Student Gradebook
Store students and their grades.
Calculate averages and determine the highest-performing student. */

struct Student {
    name: String,
    grades: Vec<f64>
}

struct GradeBook {
    students : Vec<Student>,
}

impl GradeBook {
    fn new() -> GradeBook {
        GradeBook{
            students: Vec::new(),
        }
        
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn average(&self, student: &Student) ->f64 {
        student.grades.iter().sum::<f64>() / student.grades.len() as f64
    }

    fn highest_performer(&self) -> &Student {
        self.students
            .iter()
            .max_by(|a, b| {
                self.average(a)
                    .partial_cmp(&self.average(b))
                    .unwrap()
            })
            .unwrap()
    
    }
}
 fn main() {
    let student = Student {
        name: "Kendy".to_string(),
        grades: vec![90.0, 85.0, 95.0]
    };

    let student2 = Student {
        name: "Derrick".to_string(),
        grades: vec![80.0, 95.0, 90.0]
    };

    let student3 = Student {
        name: "John".to_string(),
        grades: vec![70.0, 95.0, 95.0]
    };

    let mut gradebook = GradeBook::new();

    gradebook.add_student(student);
    gradebook.add_student(student2);
    gradebook.add_student(student3);

    for student in &gradebook.students {
        println!("{}'s average is {:.2}", student.name, gradebook.average(student));
    }

    let highest = gradebook.highest_performer();

    println!("The highest performer is {} with an average of {}", highest.name, gradebook.average(highest));

 }