use super::helper::same_sizer;

pub fn add(s1:&String,s2:&String)->String{
        let (s11,s22)=same_sizer(s1, s2);
        let total:String=add_one_to_one(&s11,&s22);
        //println!("{} !! {}",s11,s22);
        //println!("{}",total);
        total
    }

pub fn add_one_to_one(s1:&String,s2:&String)->String{

    let mut carry=0;
    let mut result:String=String::from("");
    //println!("{} >> {}: {} {}",s1,s2,s1.len(),s2.len());
    for i in (0..s1.len()).rev() {

        //println!("{} ",(s2.chars().nth(i).unwrap() as i32 -48));
        let car:i32=(s1.chars().nth(i).unwrap() as i32 -48) + (s2.chars().nth(i).unwrap() as i32 -48)+carry;
        if car>=10 {
            let mod10 =car%10;
            let div10 =car/10;
            result.insert_str(0,&mod10.to_string());
            carry=div10;

        }else{
            result.insert_str(0,&car.to_string());
            carry=0;
        }
        //println!("{}",result);
    }
    if carry>0{
        result.insert_str(0,&carry.to_string());
    }
    result
}
