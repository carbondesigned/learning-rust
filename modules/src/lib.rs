pub mod back_of_house {
    pub struct Breakfast {
       pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_resturant() {
   let mut meal = back_of_house::Breakfast::summer("beans");
   meal.toast = String::from("Wheat");
}
