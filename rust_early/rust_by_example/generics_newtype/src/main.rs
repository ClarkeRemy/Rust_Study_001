struct Years(i64);
struct Days(i64);
struct Container(i32, i32);

// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for
// aliases).
trait Contains {
    type A;
    type B;

    //fn contains(&self, _: &A, _: &B) -> bool;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B.`
    fn last(&self) -> i32; // Doesn't explicitly require `A` or `B.`
}

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}
impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}
// old
// impl Contains<i32, i32> for Container {
impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // True if the numbers stored are equal.
    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 {
        self.0
    }
    // Grab the last number.
    fn last(&self) -> i32 {
        self.1
    }
}
fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}
// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance.
// without associated types
//fn difference<A, B, C>(container: &C) -> i32
//where
//    C: Contains<A, B>,
//
// with
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}
fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));

    let years = Years(42);
    let years_as_primitive: i64 = years.0;

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
