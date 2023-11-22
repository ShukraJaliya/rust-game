
use std::io;
use std::env;

enum Reactions{
    Correct,
    Lost,
    Close, 
    Unknown
}

enum Country {
    Algeria, Angola, Benin, Botswana, BurkinaFaso, Burundi, CaboVerde, Cameroon, CentralAfricanRepublic,Chad,
    Comoros, DRCongo, Egypt, EquatorialGuinea, Eritrea, Eswatini, Ethiopia, Gabon,Gambia, Ghana,
    Guinea, GuineaBissau, IvoryCoast, Kenya, Lesotho, Liberia, Libya, Madagascar, Malawi,Mali, Mauritania,
     Mauritius, Morocco, Mozambique, Namibia, Niger, Nigeria,Rwanda, SaoTomeandPrincipe, Senegal, Seychelles,SierraLeone,
    Somalia, SouthAfrica, SouthSudan, Sudan, Tanzania,Togo,Tunisia, Uganda, Zambia, Zimbabwe, NOtknown
}

enum Region {
    North,
    West,
    Central,
    East,
    South,
}
//access the enums

fn matching(guess_country: &str) -> Country{
    let user_input_lowercase = guess_country.to_lowercase();


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
            "southsudan"=>Country::SouthSudan,
            "sudan"=>Country::Sudan,
            "tanzania"=>Country::Tanzania,
            "togo"=>Country::Togo,
            "tunisia"=>Country::Tunisia,
            "uganda"=>Country::Uganda,
            "zambia"=>Country::Zambia,
            "zimbawe"=>Country::Zimbabwe,
            _ => Country::NOtknown

    }
}
fn matchinggueses(user_guess: &str, actual_country: &str) -> Reactions {
    if user_guess.trim().to_lowercase() == actual_country.to_lowercase() {
        Reactions::Correct
    } else {
        Reactions::Unknown
    }
}
fn handle_reaction(user_reaction: Reactions) {
    match user_reaction {
        Reactions::Correct => println!("Congratulations, you got it."),
        Reactions::Lost => println!("Sorry, maybe next time."),
        Reactions::Close => println!("So close, try again."),
        Reactions::Unknown => println!("Unknown reaction."),
    }
}



fn main() {

    let acutal_country = "uganda";
    println!("Please Enter a guess");
    println!("Guess a country from East Africa");

    let mut userguess = String::new(); // dynamic string to hold user input

    let country = matching(&mut userguess);
    for _ in  0..5 {
        io::stdin()
        .read_line(&mut userguess)
        .expect("failed to read line");
        println!("You've guessed: {}", userguess);
        let reaction = matchinggueses(& mut userguess, acutal_country);
        handle_reaction(reaction);

        userguess.clear();
    }
    
}
