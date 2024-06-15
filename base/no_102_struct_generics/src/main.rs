/* 结构体泛型 */
fn main() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 'a', y: 'c' };
    let p3 = p.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Point<T, F> {
    x: T,
    y: F,
}

impl<T, F> Point<T, F> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}