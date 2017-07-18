fn main(){
    brokenvectest(); 
}

fn vectest(){
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3]; //use the vec! macro to store initial values

    v.push(1);
    v.push(6);
    v.push(2);
    v.push(19);

    let third: &i32 = &v[2];
    let second: Option<&i32> = v.get(2);

    println!("Third: {}", third);
}

fn brokenvectest(){
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(5);

    //let nonexistent = &v[10]; //Causes a thread panic
    let nonexistent = v.get(10); //returns none
}
