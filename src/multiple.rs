use super::helper::same_sizer;
use super::add::add;

#[allow(dead_code)]
pub fn multiple(s1:&String,s2:&String)->String{
        let (s11,s22)=same_sizer(s1, s2);
        let total:String=multiple_many_to_many(&s11,&s22);
        //println!("{} * {} = {} ",s1,s2,total);
        total
    }

#[allow(dead_code)]
pub fn multiple_one_to_many(s1:&String,s2:char)->String{
    let mut carry=0;
    let mut result=String::from("");
    for i in (0..s1.len()).rev() {

        let car:i32=(s1.chars().nth(i).unwrap() as i32 -48) * (s2 as i32 -48)+carry;

        if car>=10 {
            let mod10 =car%10;
            let div10 =car/10;
            result.insert_str(0,&mod10.to_string());
            carry=div10;
    
        }else{
            result.insert_str(0,&car.to_string());
            carry=0;
        }
        
    }
    if carry>0{
        result.insert_str(0,&carry.to_string());
    }

    result
}

#[allow(dead_code)]
pub fn multiple_10_pow_x(one_to_many:&String,pow:usize)->String{
    let mut result=String::from(one_to_many);
    for _i in 0..pow {
        result.insert_str(result.len(), "0")
    }
    result

}


#[allow(dead_code)]
pub fn multiple_many_to_many(s1:&String,s2:&String)->String{
    
       
        let mut result:String=String::from("0");
        
        let mut result_one_row;

        for i in (0..s1.len()).rev() {

            let  one_to_many=multiple_one_to_many(&s1,s2.chars().nth(i).unwrap());
            result_one_row=multiple_10_pow_x(&one_to_many,s1.len()-i-1);
            result=add(&result_one_row,&result);
        }
        result
    }
