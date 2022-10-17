fn main() {
    println!("Hello, world!");
    let _x = 1; // underscore 로 warning 을 무시할 수 있다.
    let 한글 = "한글"; // 한글 변수도 가능해졌다.
}

fn integer() {
    let unsigned: u8 = 123;
    let unsigned16 :u16 = 123;
    // 서로 다른 타입의 정수는 더할 수 없음
    unsigned + unsigned16;
}
