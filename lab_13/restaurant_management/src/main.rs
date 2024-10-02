mod restaurant;

fn main() {
    println!("Restaurant Management System");
    restaurant::front_of_house::hosting::hosting();
    restaurant::front_of_house::serving::serving();
}
