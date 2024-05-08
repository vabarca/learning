
use random::Source;

fn main() {
    let mut source = random::default(42);
    let number:i32 = source.read::<i32>();
    println!("Hello, world! tu núermo es el {}", number);
}
