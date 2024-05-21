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
        self.age
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
*/

/*
CHAPTER 16
 

enum UserRole{
    BASIC,
    ADMIN
}

enum WebSite{
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}

fn main()
{
    let role = UserRole::BASIC;
    let website = WebSite::INSTAGRAM(String::from("@mariano"));
}

fn hasAccess(user_role: UserRole) -> bool{
    match user_role{
        UserRole::ADMIN => true,
        UserRole::BASIC => false,
    }
}

fn goToWebsite(website: WebSite) -> None {
    match website{
        WebSite::URL => ,
        WebSite::INSTAGRAM => ,
        WebSite::FACEBOOK => ,
        WebSite::LINKEDIN => ,
    }
}
*/

/*
CHAPTER 17
 

fn main()
{
    let nombre: Option<String> = Some("Julio".to_string());

    match nombre {
        None => println!("Nombre no indicado"),
        Some(nombre) => println!("{}", nombre),
    }
}



struct User {
    city: String,
    name: String,
    age: Option<i32>,
    id: Option<i32>,
    id2: Option<i32>,
}

impl User{
    fn get_age(&self)-> Option<i32> {
        self.age
    }
}

impl User{
    fn get_id(&self)-> i32 {
        self.id.unwrap_or_default()
    }
}

impl User{
    fn get_id2(&self)-> i32 {
        if self.id2.is_none(){
            0
        }
        else{
            self.id2.unwrap()
        }
    }
}


fn main()
{
 let nuevo = User{
    city: "Valencia".to_string(),
    name: "julio".to_string(),
    age: Some(12),
    id: Some(1),
    id2: Some(2),
 };

 let age: Option<i32> = nuevo.get_age();
 let id:i32 = nuevo.get_id();
 let id2:i32 = nuevo.get_id2();


 match age {
    Some(age) => println!("{} {} {} {} {}", nuevo.city, nuevo.name, age, id, id2),
    _ => (),
 };

}

*/
/*
CHAPTER 17

struct Point<T, V>{
    x:T,
    y:T,
    z:i32,
    data:V,
}

fn main(){

    let point_a:Point<i32, String> = Point{
        x:5,
        y:6,
        z:0,
        data:"12".to_string(),
    };

    let point_b:Point<f64, i32> = Point{
        x:5.0,
        y:6.0,
        z:0,
        data:1,
    };

    let point_c:Point<i32, String> = Point{
        x:5,
        y:6,
        z:0,
        data:"12".to_string(),
    };


    calcular_cosas(point_a, point_c)
}

fn calcular_cosas<C>(p1: Point<T, V>, p2: Point<T, V>){
    let a: T = p1.x;
    let b: T = p2.y;

    return 15.0;

}

*/

/*
CHAPTER 19


fn main(){


    let edad: Option<i32> = Some(16);

    if edad.es_mayor_de_edad(){
        println!("True");
    }else {
        println!("False");
    }

}

trait LicenciaConducir {
    fn es_mayor_de_edad(&self)-> bool;
}

impl LicenciaConducir for Option<i32> {
    fn es_mayor_de_edad(&self)-> bool {
        match self {
            Some(edad) => edad > &18,
            None => false,
        }
    }
}

*/


/*
CHAPTER 20
*/

fn main(){
    
}