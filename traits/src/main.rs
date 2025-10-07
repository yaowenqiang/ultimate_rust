use std::any::Any;
use std::fmt;
use std::fmt::Debug;
use std::ops::Add;

struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Animal: Debug {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat;
#[derive(Debug)]
struct Dog;
impl Animal for Cat {
    fn speak(&self) {
        // 喵
        println!("Meow");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
    println!("{animal:?}");
}

fn make_animal() -> impl Animal {
    Cat
}

trait DowncastableAnimal {
    fn speak(&self) {
        println!("No idea")
    }
    fn as_any(&self) -> &dyn Any;
}

struct Tortoise;
impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
fn main() {
    let cat = Cat;
    cat.speak();

    let dog = Dog;
    dog.speak();

    speak_twice(&cat);

    let animal = make_animal();

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(cat), Box::new(dog)];

    animals.iter().for_each(|animal| {
        animal.speak();
    });

    let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];

    for animal in more_animals {
        if let Some(t) = animal.as_any().downcast_ref::<Tortoise>() {
            println!("I am a tortoise!");
        }
    }

    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 1.0, y: 2.0 };
    let c = a + b;
    println!("{:?}", c);
}
