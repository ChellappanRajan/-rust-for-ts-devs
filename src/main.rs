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



}

