use std::fs::File;
use std::io::{self, Write};

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    let students = [
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10101001"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer Science"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical Engineering"),
            level: 100,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical Engineering"),
            level: 100,
        },
    ];

    display_students(&students);

    save_students_to_file(&students).expect("Failed to save student details to file");
}

fn display_students(students: &[Student]) {
    println!("PAU SMIS");
    println!("| Student Name   | Matric. Number | Department          | Level |");
    println!("|----------------|----------------|---------------------|-------|");
    for student in students {
        println!(
            "| {:<15} | {:<14} | {:<19} | {:>5} |",
            student.name, student.matric_number, student.department, student.level
        );
    }
}

fn save_students_to_file(students: &[Student]) -> io::Result<()> {
    let mut file = File::create("students.txt")?;
    writeln!(file, "PAU SMIS")?;
    writeln!(file, "| Student Name   | Matric. Number | Department          | Level |")?;
    writeln!(file, "|----------------|----------------|---------------------|-------|")?;
    for student in students {
        writeln!(
            file,
            "| {:<15} | {:<14} | {:<19} | {:>5} |",
            student.name, student.matric_number, student.department, student.level
        )?;
    }
    Ok(())
}
