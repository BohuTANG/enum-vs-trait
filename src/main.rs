use std::time::Instant;

enum ArithEnum {
    Sum,
    Avg,
}

impl ArithEnum {
    fn create_sum() -> Self {
        ArithEnum::Sum
    }

    fn create_avg() -> Self {
        ArithEnum::Avg
    }

    fn op(&self, a: usize, b: usize) -> usize {
        match self {
            ArithEnum::Sum => a + b,
            ArithEnum::Avg => b / a,
        }
    }
}

trait ArithTrait {
    fn sum(&self, a: usize, b: usize) -> usize;
    fn avg(&self, a: usize, b: usize) -> usize;
}

struct A {}

impl A {
    fn create() -> Self {
        A {}
    }
}

impl ArithTrait for A {
    fn sum(&self, a: usize, b: usize) -> usize {
        a + b
    }

    fn avg(&self, a: usize, b: usize) -> usize {
        b / a
    }
}

fn enum_benchmark(n: usize) {
    let sum_enum = ArithEnum::create_sum();
    let avg_enum = ArithEnum::create_avg();
    let start = Instant::now();
    let mut sum = 1;
    for i in 0..n {
        sum = sum_enum.op(sum, i);
        avg_enum.op(sum, i);
    }
    let duration = start.elapsed();
    println!("Enum, loop:{},  cost:{:?}", n, duration);
}

fn trait_benchmark(n: usize) {
    let a_trait = A::create();

    let start = Instant::now();
    let mut sum = 1;
    for i in 0..n {
        sum = a_trait.sum(sum, i);
        a_trait.avg(sum, i);
    }
    let duration = start.elapsed();
    println!("Trait, loop:{},  cost:{:?}", n, duration);
}

fn main() {
    let n = 50000000000;
    enum_benchmark(n);
    trait_benchmark(n);
}
