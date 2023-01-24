/* —————————————————————————————————————————— Generics —————————————————————————————————————————— */

/* 
	———————————————————— program 10—11 ————————————————————
	A method that uses generic types different from its struct’s definition
*/
// struct Point<X1, Y1> {
// 	x: X1,
// 	y: Y1,
// }
    
// impl<X1, Y1> Point<X1, Y1> { // Generics X1 & X2 following impl tell the rust compiler that X1 & X2 after Point are generics, not concrete paremeters, thus allowing us to reference them in our impl block.
// 	fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
// 	    Point {
// 		x: self.x,
// 		y: other.y,
// 	    }
// 	}
// }
    
// fn main() {
// 	let p1 = Point { x: 5, y: 10.4 };
// 	let p2 = Point { x: "Hello", y: 'c' };
    
// 	let p3 = p1.mixup(p2);
    
// 	println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }
    

/* 
———————————————————— program 10—9 ————————————————————
	Implementing a method named x on the Point<T> struct that returns a reference to the x field of type
*/

// struct Point<T> {
// 	x: T,
// 	y: T,
// }
    
// impl<T> Point<T> { 
// 	// we have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>
// 	// declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
// 	fn x(&self) -> &T {
// 	    	&self.x
// 	}
// }
    
// fn main() {
// 	let p = Point { x: 5, y: 10 };
    
// 	println!("p.x = {}", p.x());
// }


/*	
	———————————————————— program 10—X ————————————————————

#![allow(unused)]
fn main() {
	enum Option<T> {
		Some(T),
		None,
	}

	enum Result<T, E> {
    		Ok(T),
    		Err(E),
	}
}
*/


/*	
	———————————————————— program 10—8 ————————————————————
 	A Point<T, U> generic over two types so that x and y can be values of different types
*/
// struct Point<T, U> {
// 	x: T,
// 	y: U,
// }
//     
// fn main() {
// 	let both_integer = Point { x: 5, y: 10 };
// 	let both_float = Point { x: 1.0, y: 4.0 };
// 	let integer_and_float = Point { x: 5, y: 4.0 }; // this will compile now since we defined Point with 2 generics
// }

/*	
	———————————————————— program 10—6 ————————————————————
 	A Point<T> struct that holds x and y values of type T
*/
// struct Point<T> {
// 	x: T,
// 	y: T,
// }
    
// fn main() {
// 	let integer = Point { x: 5, y: 10 };
// 	let float = Point { x: 1.0, y: 4.0 };

// 	// If we create an instance of a Point<T> that has values of different types, our code won’t compile
// 	// The fields x and y must be the same type because both have the same generic data type T
// 	let wont_work = Point { x: 5, y: 4.0 }; // this won't compile
// }


/* 	
	———————————————————— program 10—5 ————————————————————
	Shows the combined largest function definition using the generic data type in its signature.
	The listing also shows how we can call the function with either a slice of i32 values or char values.
*/

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
// 	let mut largest = &list[0];
    
// 	for item in list {
// 	    if item > largest {
// 		largest = item;
// 	    }
// 	}
    
// 	largest
// }

// fn main() {
// 	let number_list = vec![34, 50, 25, 100, 65];
    
// 	let result = largest(&number_list);
// 	println!("The largest number is {}", result);
    
// 	let char_list = vec!['y', 'm', 'a', 'q'];
    
// 	let result = largest(&char_list);
// 	println!("The largest char is {}", result);
// }

/* —————————————— Removing Duplication by Extracting a Function ——————————————
=> Generics allow us to replace specific types with a placeholder that represents multiple types
=> recognize duplicated code to extract into Generics

For example, say we had two functions: one that finds the largest item in a slice of i32 values, & one that finds the largest item in a slice of char values. How would we eliminate that duplication?
*/


/* 	
	program 10—4:
	two functions that both find the largest value in a slice. 
	Two functions that differ only in their names and the types in their signatures
*/

// fn largest_i32(list: &[i32]) -> &i32 {
// 	let mut largest = &list[0];
    
// 	for item in list {
// 	    if item > largest {
// 		largest = item;
// 	    }
// 	}
    
// 	largest
// }
    
// fn largest_char(list: &[char]) -> &char {
// 	let mut largest = &list[0];
    
// 	for item in list {
// 	    if item > largest {
// 		largest = item;
// 	    }
// 	}
    
// 	largest
// }
    
// fn main() {
// 	let number_list = vec![34, 50, 25, 100, 65];
    
// 	let result = largest_i32(&number_list);
// 	println!("The largest number is {}", result);
// 	assert_eq!(*result, 100);
    
// 	let char_list = vec!['y', 'm', 'a', 'q'];
    
// 	let result = largest_char(&char_list);
// 	println!("The largest char is {}", result);
// 	assert_eq!(*result, 'y');
// }
    