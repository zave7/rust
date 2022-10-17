use std::mem::size_of; // import

fn main() {
    println!("Hello, world!");
    let _x = 1; // underscore 로 warning 을 무시할 수 있다.
    let 한글 = "한글"; // 한글 변수도 가능해졌다.
    integer();
    char();
    casting();
    float();
}

fn integer() {
    println!("integer");
    let unsigned: u8 = 123;
    let unsigned16 :u16 = 123;
    // 서로 다른 타입의 정수는 더할 수 없음
    // 묵시적 타입 캐스팅이 되지 않는다.
    // unsigned + unsigned16;
    let numberWithType = 0u8; // 숫자 끝에 타입 지정 가능
    let numberWithUnderscore = 1___2___3___i32; // 숫자 사이 및 타입 사이 언더스코어 삽입 가능
    println!("{} {}", numberWithType, numberWithUnderscore);
}

fn char() {
    println!("char");
    let letter = 'a'; // 4 bytes
    let cat_face = '😹'; // char 는 유니코드
    println!("Size of a char: {}", size_of::<char>());
    println!("Size of string containing '&&&&': {}", "&&&&".len()); // len() 은 byte 수 를 구한다.
    println!("Size of string containing '🕊🕊️': {}", "🕊🕊️".len());
}

fn casting() {
    println!("casting");
    // casting = simple type change

    let my_number: u16 = 8;
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16;
    let char = '1' as u8;
    println!("casting char {}", char)

}

fn float() {
    let my_number = 9.; // . 만 찍어도 float 타입이 됨
    let other_number = 9;
    // 서로 다른 타입 연산 불가능
    // println!("{}", my_number + other_number);
    println!("{}", my_number as i32 + other_number);
}