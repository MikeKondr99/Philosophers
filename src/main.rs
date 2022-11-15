use philosophers::*;

const N: usize = 5;
fn main() {
    println!("Метод первого:");
    dining::<N>(|i| {
        if i == 0 {
            TakeFirst::Left
        } else {
            TakeFirst::Right
        }
    });
    println!("Метод четных:");
    dining::<N>(|i| {
        if i % 2 == 0 {
            TakeFirst::Left
        } else {
            TakeFirst::Right
        }
    });
    println!("Метод deadlock:");
    dining::<N>(|_| TakeFirst::Left);
}
