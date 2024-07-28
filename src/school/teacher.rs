pub mod female_student {
    pub fn num() -> i32 {
        10
    }
}

pub fn total() -> i32 {
    let stu_num1 = crate::school::student::num();
    let female_stu = self::female_student::num();
    let stu_num2 = super::student::num();


    use  crate::school::worker::male;

    let male_num = male::num();
    let eqa = stu_num1 == stu_num2;
    println!("eqa {}",eqa);
    stu_num1 + female_stu + male_num
}