
/*
fn intro_to_idea(){
    
    pub struct Rectangle{
        pub width: f64,
        pub height: f64,
     }

    impl Rectangle {
         fn get_area(&self) -> f64 {
             self.width * self.height
         }
    }


    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
         fn get_area(&self) -> f64 {
             self.radius * self.radius * 3.14 as f64
         }
     }

     let rec = Rectangle {width:5.0,height:8.0};
     let circle = Circle {radius: 5.0};

     println!("Area of a rectangle {}", rec.get_area());
     println!("Area of a circle {}", circle.get_area());

    // let shapes: Vec<????> = vec![rec, circle]; 
    // unfortunately doesn't work
    Even tho Rectangle and circle use the same get_area() method we are unable to group the together since they are different types
    Vector must contain elements of the same type
}
fn main(){
    intro_to_idea();
}
    */

fn same_method_same_logical_entity() {
    //bind different data types with same behaviour, as one logical unit
    pub trait AreaInfo {
        fn get_area(&self)->f64;
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl AreaInfo for Rectangle {
        fn get_area(&self) -> f64 {
            self.width * self.height
        }
    }

    pub struct Circle {
        pub radius: f64, 
    }

    impl AreaInfo for Circle {
        fn get_area(&self) -> f64 {
            self.radius * self.radius * 3.14 as f64
        }
    }

    let rec = Rectangle {width: 5.0, height: 8.0};
    let cir = Circle {radius: 5.0};

    println!("Area of Rectangle {}", rec.get_area());
    println!("Area of Circle {}",cir.get_area());

    let shapes: Vec<&dyn AreaInfo> = vec![&rec, &cir];
    //dyn = dynamic word

    for shape in shapes.iter() {
        println!("{}",shape.get_area());
    }
}
fn main(){
    same_method_same_logical_entity();
}