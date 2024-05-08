/*
CHAPTER 12
fn main() 
{
    let mut x:u8 = 2;
    println!("Hello, world! tu núermo es el {}", x);
    x = 10;
    println!("Heºllo, world! tu núermo es el {}", x);
    const LIKE_A_DEFINE:i32 = 3;
    println!("Heºllo, world! tu núermo es el {}", LIKE_A_DEFINE);
}
*/

/*
CHAPTER 13
fn main() 
{
    //Integer
    let entero: i8 = 23;
    let entero2: u8 = 40;
    let entero3Ç: i8 = -40;

    //Integer literals
    let decimal = 98_224;
    let hex = 0xff;
    let octal = 0o777;
    let binary =0b1111_0000;

    //Float
    let float1 = 5.0;
    let float32: f32 = 12.42;

    //Boolean
    let verdader = true;
    let falso: bool = false;

    //Character
    let character = 'a';
    let emoji = '😎';

    //Compound types


    //Tuplas
    let tupla = ('h', 23, -45, 0.2222);
    let tupla2: (char, i64, i32, f64) = ('h', 23, -45, 0.2222);

    let (x,y,z,w) = tupla;

    println!("El valor de x es {}", x);
    println!("El primer valor de la tupla es {}", tupla.1);

    //Array
    let vector = [1,2,3,4,5];
    println!("El primer valor del vector es {}", vector[1]);
    let vector2:[i32; 5] = [1,2,3,4,5];

    //String slide
    let nombre: &str= "Julio Cesar";
    let apellido: String = "Julio".to_string();
    let mut domicilio = String::new(); //Puede crecer y se aloja en el heap
    domicilio = "acequia de la cadena".to_string();

}
*/
/*
CHAPTER 14

fn print_number(nb:i32){
    println!("Number {}", nb);
}

fn print_number_by_reference(nb:&i32){
    println!("Number {}", *nb);
}

fn expresion() -> i32
{
    8
}

fn main()
{
    let nb = expresion();
    print_number(nb);
    print_number_by_reference(&nb);
    print_number_by_reference(&16);
    say_hello_with_string("TT".to_string());
    say_hello("TT2");
}

fn say_hello(name: &str)
{
    println!("Hello {}", name);

}

fn say_hello_with_string(name: String)
{
    println!("Hola {}", name);
}
*/
/*
CHAPTER 15
*/


//Struct
struct User {
    name: String,
    email: String,
    age: u32,
    active: bool
}

//tupple structs
struct Point(i32, i32, i32);

//Add methods to the struct
impl User {
    fn get_age(&self) -> u32
    {
        self.age;
    }
}


fn main()
{
    let mut user = User { name: "TT".to_string(), email: String::from("vabarca@kk.com"), age: 45, active: true};
    print_user(&user);
    user.active = false;

    let user2 = create_user(String::from("Marco Aurelio"), 45);
    print_user(&user2);

    let user3 = User{
        name: String::from("Quetzalcoatl"),
        ..user // the rest of the parameters are equal to user parameter values
    };

    print_user(&user3);

    let point_a = Point(0,2,3);
    println!("X:{}, Y:{}, Z{}", point_a.0, point_a.1, point_a.2);
}

fn create_user(name:String, age:u32) -> User{

    return User { name, email: "tbd".to_string(), age, active: true };
}

fn print_user(user: &User)
{
    println!("User {}, age {}, email {}", user.name, user.age, user.email);
    println!("Age {}", user.get_age());
}