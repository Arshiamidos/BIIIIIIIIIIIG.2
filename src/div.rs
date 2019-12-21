
use super::helper::same_sizer;
use super::helper::left_zero_kill;
use super::helper::is_greater;
use super::helper::is_greater_or_equal;
use super::add::add;
use super::sub::sub;
use super::multiple::multiple;

#[allow(dead_code)]
pub fn div(s1:&String,s2:&String)->String{
        let (mut s11, mut s22)=same_sizer(s1, s2);
        s11=left_zero_kill(&s11);
        s22=left_zero_kill(&s22);
       // println!("{} / {}",s11,s22);
        let total:String=div_many_to_many(&s11,&s22);
        total
        
}

#[allow(dead_code)]
pub fn div_chunks(s1:&String,s2:&String)->(String,String){
    

    let mut divider  =String::from("0");
    let mut reminder ;
    
   // println!("{} {}",s1,s2);
    while is_greater_or_equal(&s1, &multiple(&s2,&divider)) {

        divider=add(&divider,&String::from("1"));
        
        if is_greater(&divider, &String::from("11") ){
            panic!("error in divider");
        }
    }
    if is_greater_or_equal(&s1, &multiple(&s2,&divider)){
        divider=add(&divider,&String::from("1") );
    }

    divider=left_zero_kill(&sub(&divider, &String::from("1")));
    reminder=sub(&s1,&multiple(&divider, &s2));
    
    if divider.len()==0 {
       return (String::from("0"),reminder)
    }
    
    (divider,reminder)
}



#[allow(dead_code)]
pub fn div_many_to_many(divee:&String,s2:&String)->String{

        let mut result:String=String::from("0");
        let mut s1_copy=String::from(divee);
        let mut s1=String::from("");
        let mut space    = 0;

        //println!("| s1 {} | s2 {} |  | | result {} >> s1_copy {} ",s1,s2,result, s1_copy);

        while &s1_copy.len() >= &s2.len() {
            //is_greater(&s1_copy, &s2) {
            if s2.len()+space > s1_copy.len() {
                break;
            }

            s1=String::from(&s1_copy[0..s2.len()+space]);

         //   println!("new s1 is {}",s1);
            
            let (divider,reminder)=div_chunks(&s1,&s2);

            
            result.insert_str(result.len(), &divider.to_string() );

            if is_greater_or_equal(&String::from("0"), &reminder){
              //  println!("{}",">>>>>>>>>>>>>>>");
                s1_copy=format!("{}{}",
                    &String::from("0").repeat(s2.len()+space),       
                    &s1_copy[s2.len()+space..]); 
        
            }else{

                s1_copy=format!("{}{}",
                    &reminder[&reminder.len()-s2.len()-space..],       
                    &s1_copy[s2.len()+space..]); 
                    
            }

            space+=1;
           
            //delta+=1;
          // println!(" s1 {} | s2 {} | divider {} | reminder {} | result {} >> s1_copy {} space {} ",s1,s2,divider,&reminder[&reminder.len()-s2.len()..],result, s1_copy,space);
        }

        result
    }