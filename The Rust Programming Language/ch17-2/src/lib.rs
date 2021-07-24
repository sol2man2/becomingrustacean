// Using Trait Objects That Allow for Values of Different Types
//
// Defining a Trait for Common Behavior

pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

// pub struct Screen<T: Draw> {
//   pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//   T: Draw,
// {
//   pub fn run(&self) {
//     for component in self.components.iter() {
//       component.draw();
//     }
//   }
// }

// Implementing the Trait

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    // code to actually draw a button
  }
}

// move on to lib.rs

// Trait Objects Perform Dynamic Dispatch
// Object Safety Is Required for Trait Objects
// A trait is object safe if all the methods defined in the trait have the following properties:
// 1. The return type isn’t Self.
// 2. There are no generic type parameters.

// pub trait Clone {
//   fn clone(&self) -> Self;
// }

// pub struct Screen {
//   pub components: Vec<Box<dyn Clone>>,
// }
