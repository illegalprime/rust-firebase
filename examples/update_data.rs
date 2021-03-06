extern crate firebase;

use firebase::Firebase;

fn main() {
    let david = Firebase::new("https://shining-torch-7752.firebaseio.com/users/david.json");

    let res = david.set("{\"firstName\":\"Dave\"}");

    println!("Response body: {:?}", res.body);
    println!("Response code: {:?}", res.code);
    println!("Response success: {:?}", res.is_success());

    let res = david.update("{\"firstName\":\"David\"}");

    println!("Response body: {:?}", res.body);
    println!("Response code: {:?}", res.code);
    println!("Response success: {:?}", res.is_success());
}
