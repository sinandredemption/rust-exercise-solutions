
struct Circle<T: num::Num> {
    radius: T,
}

struct Square<T: num::Num> {
    side: T,
}

impl<T> Circle<T> where T: num::Float + std::ops::MulAssign + Copy {
    fn new(r: T) -> Self {
        Circle { radius: r }
    }

    fn area(&self) -> T {
        let pi: T = std::f32::consts::PI.into();
        num::Float<T * self.radius * self.radius
        //std::f32::consts::PI.into<T>() * self.radius * self.radius
    }

    fn scale(&mut self, x: T) {
        self.radius *= x;
    }

    fn destroy(self) -> T {
        self.radius
    }
}

impl<T> Square<T> where T: num::Num + std::ops::MulAssign + Copy {
    fn new(r: T) -> Square<T> {
        Square { side: r }
    }

    fn area(&self) -> T {
        self.side * self.side
    }

    fn scale(&mut self, x: T) {
        self.side *= x;
    }

    fn destroy(self) -> T {
        self.side
    }
}

trait HasArea<T> {
    fn area(&self) -> T;
}
