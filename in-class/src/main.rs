
fn double(x:i32) -> i32 {
   // return x*2;
   {
    let y = 10;
    x*2*y
   }

}

fn main(){
    println!("Double {} equals to {}",5,double(5));
}
