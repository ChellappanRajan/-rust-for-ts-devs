use std::{process::id};

use crate::shapes::Circle;

mod shapes;
use  shapes::{Area};


fn main() {
    //Variables
    let foo = 5; //Which is similar to ts const foo = 5; immutable
    let mut foo = 5; //Can be mutable
    //Shadowing
    //Rust can have same variable name with different data types    
        //let file = get_file(args); //FileHandle
        // let foo =   read_file(arg); String


    // //Control flow
        //     if(condition || condition){
        
        //     }
        //     if condition {
        
        //}


    // Loops
            for i in 0..10 { //i upto 10 but not including 10
                print!("{} ",i);
            }
            for i in 0..=10 { //i upto 10 but  including 10
                print!("{} ",i);
            }
            // while condtion{}

        // ForEver loop
            //  loop {
                
            //  }
            // for(const idx in [1,2,3].iter()) {
            //     print!("{}",idx);
            // }
            //Vec![1,2,3].iter().map(...)//process.collect::<Vec<_>>();

        
    //Functions
            fn add(a:i32,b:i32)->i32{
                 a + b
            }

    //Closures //Close over values
        // |x| {
        //     return x;
        // }

        // |x| x+1

    //Classes and methods
            // struct Foo{
                //propertiess...
                // pub properties
            // }
            // impl Foo {
                // fn this()
                // pub fn this()

                // fn this(&self)
                //fn this(&mut self )
            // }
    
    //Interfaces
            // trait Foo{
                //no properties are allowed
            //     fn method(&self)-> returnType    
            // }
            // impl Foo for myStruct {
                // ...
            // }
     
    struct Animal{
        name:String
    }

    impl Animal{
        fn eat(&self){
            print!("{} is eating",self.name);
        }
        fn breathe(&self){
            print!("{} is breathing",self.name);
        }
    }
    trait Canine {
       fn bark(&self);
       fn run(&self);
    }

    impl Canine for Animal{
        fn bark(&self) {
            print!("{} is barking",self.name);
        }
        fn run(&self) {
            print!("{} is running",self.name);
        }
    }

    //Numbers   
    //   usize,isize,f

   //There are two types String and &str
     //String heap allocated, mutable
     //&str immutable,commonly called slice
    

   //Vector
    let mut a = vec![1,2,3,4,5,6];
    a[1]; //out of bounds, panic
    a.get(1); //Safe way

    a.push(7);
    a.pop();
    
  //Tuble
  let a = (5,String::from("helo"));
   //pattern matching similar to destructring
    let (my_num,my_str) = a;
    // fn b((a,b):(i32,String)){}
    //fn v(Mystruct{x,y,..}:MyStruct){} // pattern matching will work with struct also

    struct MyStruct{
        x:i32,
        y:i32,
        eventType:String
    }
    let foo = MyStruct{
        x:10,
        y:20,
        eventType:String::from("click")
    };

    let MyStruct{x,y,..} = foo;
    let MyStruct{x,y,eventType} = foo;

    //Pattern matching with if condition
    // if let MyStruct{x,..} = foo{
    //     print!("{}",x);
    // }
    //unwrap grap inner value of result/option
    let fo = Some(1);
    let fo = fo.unwrap();

    let mut x = 5;
    let z = x; //Read only
    let y = &mut x; //Read and write 
    print!("{}",y);
     x = 10;
    print!("{}",x);
    // print!("{}",);
   //All are expression in rust
    // let foo = if true{
    //     return true;
    // }

//Excercise
    let num = vec![1,2,3];
     //Vec<_> _ - means infer type automatically
     let result:Vec<_> = vec![1,2,3].iter().map(|x| x+1).collect();
     print!("{:?}",result);
     print!("{:?}",result);

     let data: Vec<i32> =  vec![1,2,3];
      let mut foo = data.iter().map(|x| x+1);
     let mut new_vector:Vec<i32> = vec![];

      while let Some(x) = foo.next(){
        new_vector.push(x);
      }
      print!("{:?}",new_vector);
    
    let read_file = std::fs::read_to_string("files.txt").unwrap();
    read_file.lines().enumerate().filter(|(idx,va)|idx%2==0).for_each(|txt| println!("{:?}",txt));
    // read_file.lines().enumerate().filter(|(idx,va)|idx%2==0).filter(|(idx,newLin)| idx >= 2 && idx <= 4 ).for_each(|txt| println!("{:?}",txt));

//Enum

enum Color {
    Red,
    Green,
    Blue
}

impl Color{
    fn is_red(&self) -> bool{
       if let Color::Red = self{
        return true;
       }
       false
    }
    fn is_red_parts(&self)->bool{
        match self{
            Color::Blue=> false,
            Color::Red =>true,
            _ => false
        }
    }
}

let foo = Color::Blue;
print!("{}",
foo.is_red());

fn print_color(color:Color){
  match color {
     Color::Blue=> print!("Blue"),
     Color::Red=> print!("Red"),
     Color::Green=> print!("Green")
  }
}

enum Item{
    Number(usize),
    String(String)
}

// let mut new_items: Vec<Item> = vec![];

fn append(items: &mut Vec<Item>){
    items.push(Item::String("Hello".into()))
}

fn multiply(num: Option<usize>)->usize{
    return  num.unwrap_or(0);
}
fn multiply_undefined(num: Option<usize>)->Option<usize>{
    return  num.map(|x| x * 5);
}


fn practice(list:Vec<usize>,idx:usize)->usize{
    // let a = list.get(idx)?;
    // if let Some(x) = list.get(idx){
    //     return x;
    // }
    return list.get(idx).unwrap_or(&idx) * 5;
}

// if let Some(x) = std::env::args().nth(1){
//     return; x;
// }

// let arg = std::env::args().nth(1).expect("file name to be passed in");
// let file = std::fs::read_to_string(arg).expect("Error occured");
// file.lines().for_each(|x|print!("{}",x));


let rect = shapes::Rectangle{
    height:10.0,
    width:10.0,
    x:0.0,
    y:0.0
};

let cir = Circle{
    radious:2.0,
    x:0.0,
    y:0.0
};
println!("Cir Result:: {:?}",cir.area()); //This area train only visible if we import this train from shapes file
print!("Result::    {:?}",rect.area());

}

