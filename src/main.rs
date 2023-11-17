
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
    Comoros, DRCongo, Djibouti, Egypt, EquatorialGuinea, Eritrea, Eswatini, Ethiopia, Gabon,Gambia, Ghana,
    Guinea, GuineaBissau, IvoryCoast, Kenya, Lesotho, Liberia, Libya, Madagascar, Malawi,Mali, Mauritania,
     Mauritius, Morocco, Mozambique, Namibia, Niger, Nigeria,Rwanda, SaoTomeandPrincipe, Senegal, Seychelles,SierraLeone,
    Somalia, SouthAfrica, SouthSudan, Sudan, Tanzania,Togo,Tunisia, Uganda, Zambia, Zimbabwe,
}

enum Region {
    North,
    West,
    Central,
    East,
    South,
}
//access the enums
fn handle_reaction(user_reaction: Reactions) {
    match user_reaction {
        Reactions::Correct => println!("Congratulations, you got it."),
        Reactions::Lost => println!("Sorry, maybe next time."),
        Reactions::Close => println!("So close, try again."),
        Reactions::Unknown => println!("Unknown reaction."),
    }
}

fn matching(guess_country: &str) -> Region{
    match guess_country.as_str(){
        Country::Algeria => Region::North,
        Country::Angola => 
        Country::Benin
        Country::Botswana
        | Country::BurkinaFaso
        | Country::Burundi
        | Country::CaboVerde
        | Country::Cameroon
        | Country::CentralAfricanRepublic
        | Country::Chad
        | Country::Comoros
        | Country::DRCongo
        | Country::Djibouti
        | Country::Egypt
        | Country::EquatorialGuinea
        | Country::Eritrea
        | Country::Eswatini
        | Country::Ethiopia
        | Country::Gabon
        | Country::Gambia
        | Country::Ghana
        | Country::Guinea
        | Country::GuineaBissau
        | Country::IvoryCoast
        | Country::Kenya
        | Country::Lesotho
        | Country::Liberia
        | Country::Libya
        | Country::Madagascar
        | Country::Malawi
        | Country::Mali
        | Country::Mauritania
        | Country::Mauritius
        | Country::Morocco
        | Country::Mozambique
        | Country::Namibia
        | Country::Niger
        | Country::Nigeria
        | Country::Rwanda
        | Country::SaoTomeandPrincipe
        | Country::Senegal
        | Country::Seychelles
        | Country::SierraLeone
        | Country::Somalia
        | Country::SouthAfrica
        | Country::SouthSudan
        | Country::Sudan
        | Country::Tanzania
        | Country::Togo
        | Country::Tunisia
        | Country::Uganda=>Region::East,
        | Country::Zambia=> Region::Central,
        | Country::Zimbabwe=> Region::North,

    }

}

fn main() {

    let acutal_country = "Uganda";
    println!("Please Enter a guess");
    println!("Guess a country from East Africa");

    let mut userguess = String::new(); // dynamic string to hold user input

    for _ in  0..5 {
        io::stdin()
        .read_line(&mut userguess)
        .expect("failed to read line");
        println!("You've guessed: {}", userguess);
        userguess.clear();
    }
    //assignes reaction to variable for easy access
    let user_reaction= Reactions::Correct;

    if userguess = 
 //watsuapsaj ax

}
