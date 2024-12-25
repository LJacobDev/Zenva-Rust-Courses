fn main() {
    //exercise on functions

    let student_scores = [85, 90, 78, 92, 88];

    for score in student_scores {

        let grade = get_letter_grade(score);

        println!("{score}, {grade}");
    }
}

fn get_letter_grade(score: i32) -> char {
    let grade: char;
    if score >= 90 {
        grade = 'A';
    } else if score >= 80 {
        grade = 'B';
    } else if score >= 70 {
        grade = 'C';
    } else if score >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }
    grade
}
