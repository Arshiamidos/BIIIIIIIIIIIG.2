use big2::add::add;
use big2::div::div;
use big2::multiple::multiple;

fn main() {
    
    let  s1=String::from("8723548");
    let  s2=String::from("213");
    println!("a {} / b {} = {}  :: {}",s1,s2,add(&s1,&s2),8723548+213);
    
     let  s1=String::from("213008723548");
     let  s2=String::from("213");
     println!("a {} / b {} = {}  :: {}",s1,s2,div(&s1,&s2),(213008723548 as usize)/(213 as usize));

     let  s1=String::from("2130000001");
     let  s2=String::from("213");
     println!("a {} / b {} = {}  :: {}",s1,s2,div(&s1,&s2),2130000001/213);

    
     let s1=String::from("42342351098");
     let s2=String::from("432");
    println!("a {} / b {} = {}  :: {}",s1,s2,div(&s1,&s2),(42342351098 as usize)/(432 as usize));


    let s1=String::from("9758923459723");
    let s2=String::from("209582");
    println!("a {} / b {} = {}  :: {}",s1,s2,div(&s1,&s2),(9758923459723 as usize)/209582);
    
}