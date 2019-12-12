// struct Tweet {
//     username: String,
//     content: String,
// }

// struct NewsArticle {
//     author: String,
//     content: String,
// }

// pub trait Summary {
//     fn summarize_author (&self) -> String;
//     fn summarize(&self) -> String {
//         format!("This is a default implementation {}", self.summarize_author())
//     }
// }

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }

// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.author)
//     }
// }

// fn notify (item: impl Summary) {
//     println!("{}", item.summarize());
// }

// fn main() {
//     let tweet_1 = Tweet {
//         username:String::from("Jeff"),
//         content:String::from("It's cold outside"),
//     };

//     let news_article1 = NewsArticle {
//         author:String::from("Jordan"),
//         content:String::from("12 rules for life"),
//     };

//     let s = String::from("Hello world");
//     notify(tweet_1);
//     notify(news_article1);
// }


// #[derive(Debug)]
// struct Vehicle {
//     EngineCC: String,
//     Color: String,
//     Model: String,
// }

// pub trait VehicleTypes {
//     fn  Car(&self) -> String;
//     fn Truck(&self, Loading:String) -> String;
// }

// impl Vehicle {
//     pub fn create_vehicle(EngineCC:String,Color:String,Model:String) -> Vehicle{
//         Vehicle{
//             EngineCC,
//             Color,
//             Model,
//         }
//     }

// }

// impl VehicleTypes for Vehicle {
//     fn Car(&self) ->String{
//         format!("Engine CC {}, Color: {}, Model: {}",
//     self.EngineCC,
//     self.Color,
//     self.Model,
//     )
//     }

//     fn Truck(&self,Loading:String) ->String{
//         format!("Engine CC {}, Color: {}, Model: {}, Loading: true",
//     self.EngineCC,
//     self.Color,
//     self.Model,
//     )
//     }
// }

// fn main() {
//     let v_01 = Vehicle::create_vehicle{
//         "5000".to_string(),
//         "blue".to_string(),
//         "2015".to_string(),
//     };

//     let v_02 = Vehicle::create_vehicle{
//         "800".to_string(),
//         "red".to_string(),
//         "2015".to_string(),
//     };

//     println!("The first vehicle type is {}", v_01.Car());
//     println!("The Second vehicle type is {}", v_02.Truck());
// }

// struct Tweet {
//     username: String,
//     content: String,
// }

// struct NewsArticle {
//     author: String,
//     content: String,
// }

// pub trait Summary {
//     fn summarize_author (&self) -> String;
//     fn summarize(&self) -> String {
//         format!("This is a default implementation {}", self.summarize_author())
//     }
// }

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }

// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.author)
//     }
// } 

// fn summarizeable() -> impl Summary {
//     Tweet {
//         username:String::from("A"),
//         content:String::from("B"),
//     }
//  }

//  fn main() {
//     let return_value = summarizeable();
//     println!("{}", return_value.summarize());
//  }

// #[derive(Debug)]
// struct Driver {
//     Name: String,
//     Age: String,
//     Licensce: String,
// }

// pub trait DriverTypes {
//     fn  Car_Driver(&self) -> String;
//     fn Truck_Driver(&self) -> String;
// }

// impl Driver {
//     pub fn create_driver(Name:String,Age:String,Licensce:String) -> Driver{
//         Driver{
//             Name,
//             Age,
//             Licence,
//         }
//     }

// }

// impl DriverTypes for Driver {
//     fn Car_Driver(&self) ->String{
//         format!("Name {}, Age: {}, Licensce: {}",
//     self.Name,
//     self.Age,
//     self.Licence,
//     )
//     }

//     fn Truck_Driver(&self) ->String{
//         format!("Name {}, Age: {}, Licensce: {}",
//     self.Name,
//     self.Age,
//     self.Licence,
//     )
//     }
// }

// fn main() {
//      let d_01 = Driver:create_driver {
//          "Faheen".to_string(),
//          "23".to_string(),
//          "M.Car".to_string(),
//      };

//     let d_02 = Driver:create_driver {
//         "Zain".to_string(),
//         "23".to_string(),
//         "Truck_type".to_string(),
//     };

//     println!("The first driver type is {}", d_01.Car_Driver());
//     println!("The Second vehicle type is {}", d_02.Truck_Driver());
// }

fn main() {
    let integer_list = vec![1,5,9,0];
    let char_list = vec!['h', 'k', 'd'];
    println!("{}",largest_item(&integer_list));
    println!("{}",largest_item(&char_list));
}

fn largest_item <T: PartialOrd + Copy> (item: &[T]) -> T {
    let mut largest = item[0];
    for &number in item.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}