
use std::io;
use std::process;

#[derive(PartialEq)]
enum Country {
    Algeria, Angola, Benin, Botswana, BurkinaFaso, Burundi, CaboVerde, Cameroon, CentralAfricanRepublic,Chad,
    Comoros, DRCongo, Egypt, EquatorialGuinea, Eritrea, Eswatini, Ethiopia, Gabon,Gambia, Ghana,
    Guinea, GuineaBissau, IvoryCoast, Kenya, Lesotho, Liberia, Libya, Madagascar, Malawi,Mali, Mauritania,
     Mauritius, Morocco, Mozambique, Namibia, Niger, Nigeria,Rwanda, SaoTomeandPrincipe, Senegal, Seychelles,SierraLeone,
    Somalia, SouthAfrica, SouthSudan, Sudan, Tanzania,Togo,Tunisia, Uganda, Zambia, Zimbabwe, Notknown
}
#[derive(PartialEq)]
enum Region {
    North,
    West,
    Central,
    East,
    South,
    Where
}
//access the enums
fn matchingregion(guess_region: &str) -> Region{
    let _user_input_lowercase = guess_region.to_lowercase();
    match guess_region{
        "north"=>Region::North,
        "east"=>Region::East,
        "west"=>Region::West,
        "south"=>Region::South,
        "central"=>Region::Central,
        _=> Region::Where
    }
}

//matching countries to user gueses 
fn matching(guess_country: &str) -> Country{
    let _user_input_lowercase = guess_country.to_lowercase();

    match  guess_country{
            "algeria" => Country::Algeria,
            "angola" =>Country::Angola,
            "benin" =>Country::Benin,
            "botswana" =>Country::Botswana,
            "bukinafaso" =>Country::BurkinaFaso,
            "burundi" =>Country::Burundi,
            "caboverde" =>Country::CaboVerde,
            "camerron" =>Country::Cameroon,
            "cenralafricarepublic" =>Country::CentralAfricanRepublic,
            "chad" =>Country::Chad,
            "comors" =>Country::Comoros,
            "drcongo" =>Country::DRCongo,
            "egypt" =>Country::Egypt,
            "equatorialguinea" =>Country::EquatorialGuinea ,
            "eritiera" =>Country::Eritrea,
            "eswatini" =>Country::Eswatini,
            "ethiopis" =>Country::Ethiopia,
            "gabon" =>Country::Gabon,
            "gambia" =>Country::Gambia,
            "ghana" =>Country::Ghana,
            "guinea" =>Country::Guinea,
            "gineabissau" =>Country::GuineaBissau,
            "ivorycoast" =>Country::IvoryCoast,
            "kenya" =>Country::Kenya,
            "lesotho" =>Country::Lesotho,
            "liberia" =>Country::Liberia,
            "libya" =>Country::Libya,
            "madagascar" =>Country::Madagascar,
            "malawi" =>Country::Malawi,
            "mali" =>Country::Mali,
            "mauritania" =>Country::Mauritania,
            "mauritius" =>Country::Mauritius,
            "morocco" =>Country::Morocco,
            "mozambique" =>Country::Mozambique,
            "namibia" =>Country::Namibia,
            "niger" =>Country::Niger,
            "nigeria"=>Country::Nigeria,
            "rwanda"=>Country::Rwanda,
            "saotomeandprincipel" =>Country::SaoTomeandPrincipe,
            "senegal"=>Country::Senegal,
            "seychelles"=>Country::Seychelles,
            "sierraleone"=>Country::SierraLeone,
            "somalia"=>Country::Somalia,
            "southafrica"=> Country::SouthAfrica,
            "southsudan"=>Country::SouthSudan,
            "sudan"=>Country::Sudan,
            "tanzania"=>Country::Tanzania,
            "togo"=>Country::Togo,
            "tunisia"=>Country::Tunisia,
            "uganda"=>Country::Uganda,
            "zambia"=>Country::Zambia,
            "zimbawe"=>Country::Zimbabwe,
            _ => Country::Notknown

    }
}



fn main() {


    println!("\nThere are 5 levels to this game \n");
    println!("FOr this first level, You only get  5  tries \n");
    println!("if country has two words, write as one eg. South Africa write as southafrica\n");
    println!("Are you ready? \n");

    println!("Start \n");

    println!("--------------Level 1----------------\n");

    println!("\nPlease Enter a Country");
    println!("Guess what country in Africa Shukra is from");

    for _ in 0..5 {//user gets 5 tries 
        let mut user_guess = String::new(); // dynamic string to hold user input
        io::stdin()//reads the user input for country
            .read_line(&mut user_guess)
            .expect("failed to read line");

        let mut user_guess = user_guess.trim().to_lowercase();
        println!("\nYou've guessed: {}\n", user_guess);

        match matching(&user_guess) { //in those tries, it matches the user input to specific out put.
            Country::Uganda => {
                println!("YAY!!!!!!!, that's right. How did you know!\n");
                println!("----------------Level 2----------------\n");
                println!("Guess what region. You have 3 tries");
                println!("Chose Either North, East, South or West\n");


            for _ in 0..3 {
                let mut user_region = String::new(); // dynamic string to hold user input
                io::stdin()
                .read_line(&mut user_region)
                .expect("failed to read line");

            let mut user_region = user_region.trim().to_lowercase();

                match matchingregion(&user_region){
                    Region::East=>{
                        println!("YAY!!!!!! You have done it again\n");
                        break;//Ends programmm once answer is correct 
                    }
                    Region::Where => {
                        println!(" Where is that .TRY AGAIN\n");
                        user_region.clear();
                    }
                    _=>{
                        println!("Try again.\n");
                        user_region.clear();

                    }

                }
        
            }println!("Sorry, run out of attempts");
            process::exit(0); //exits program when out of attems for region
            
            } 
            Country::Notknown => {
                println!("Where is that? TRY AGAIN\n");
                user_guess.clear();
            }
            _ => {
                println!("On the right track. it is in Africa but Try again\n");
                user_guess.clear();
            }
        
        }
    }println!("Sorry, run out of attempts");
        
    }

 













