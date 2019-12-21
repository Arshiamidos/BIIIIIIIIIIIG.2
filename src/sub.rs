    use super::helper::same_sizer;
    use super::helper::is_greater;
   
    #[allow(dead_code)]
    pub fn sub_one_to_one(s1:&String,s2:&String)->String{
        let mut carry=0;
        let mut result:String=String::from("");
      
        for i in (0..s1.len()).rev() {
    
    
            let mut car:i32=(s1.chars().nth(i).unwrap() as i32 -48-carry) - (s2.chars().nth(i).unwrap() as i32 -48);
    
            if car<0{
                car=(s1.chars().nth(i).unwrap() as i32 + 10 -48-carry) - (s2.chars().nth(i).unwrap() as i32 -48);
                
                result.insert_str(0,&car.to_string());
                carry=1;
                
            }else{
                result.insert_str(0,&car.to_string());
                carry=0;
            }
    
            //println!("{}",result);
        }
       result
    }
    #[allow(dead_code)]
    pub fn sub(s1:&String,s2:&String)->String{
        let (s11,s22)=same_sizer(s1, s2);
        let mut total:String;
        if is_greater(&s1,&s2) {
            total=sub_one_to_one(&s11,&s22);
        }else{
            total=sub_one_to_one(&s22,&s11);
        }
        total
       
    }