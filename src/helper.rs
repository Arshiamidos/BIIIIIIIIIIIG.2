
#[allow(dead_code)]
pub fn is_greater_or_equal(s1:&String,s2:&String)->bool{

    let (s11,s22)=same_sizer(s1, s2);
    let mut is_greater_or_equal=true;
    for i in 0..s11.len() {
        if s11.chars().nth(i).unwrap() as i32 == s22.chars().nth(i).unwrap() as i32 {
            continue
        }
        if s11.chars().nth(i).unwrap() as i32 >= s22.chars().nth(i).unwrap() as i32 {
            is_greater_or_equal=true;
        }else{
            is_greater_or_equal=false;
        }
        break;
    }
    is_greater_or_equal

} 

#[allow(dead_code)]
pub fn same_sizer(s1:&String,s2:&String)->(String,String){

    let mut s22=String::from(s2);
    let mut s11=String::from(s1);

    if s1.len()>=s2.len() {
        s22=pad(s2,s1.len()-s2.len());
    }else if s1.len()<=s2.len() {
        s11=pad(s1,s2.len()-s1.len()); 
    }

    (s11,s22)


}

#[allow(dead_code)]
pub fn left_zero_kill(s:&String) ->String{
    let mut st = String::from("");
    let  still_found_zero=true;
    for _i in 0..s.len() {
        if s.chars().nth(_i).unwrap() as i32 -48 == 0 && still_found_zero{
            continue;
        }else{
            st.insert_str(0,&s[_i..]);
            break;
        }
    }
    st
}

#[allow(dead_code)]
pub fn is_greater(s1:&String,s2:&String)->bool{

    let (s11,s22)=same_sizer(s1, s2);
    let mut is_greater=true;
    for i in 0..s11.len() {
        if s11.chars().nth(i).unwrap() as i32 == s22.chars().nth(i).unwrap() as i32 {
            continue
        }
        if s11.chars().nth(i).unwrap() as i32 > s22.chars().nth(i).unwrap() as i32 {
            is_greater=true;
        }else{
            is_greater=false;
        }
        break;
    }
    is_greater

} 
#[allow(dead_code)]
pub fn pad(s:&String,i:usize) ->String{
    let mut st = String::from("");
    for _i in 0..(i) {
        st.insert_str(0,"0")
    }
    st.insert_str(st.len(), s);
    st
}

