use rand::Rng;

fn main() {
    const NUM_OF_DOORS:u128 = 3;//number of doors of every simulated game

    let mut total_games:u128 = 0;//this gets incremented ever loop iteration
    let mut total_wins_without_switching:u128 = 0;//this only gets incremented when a player that doesn't switch wins a game
    let mut total_wins_with_switching:   u128 = 0;//this only gets incremented when a player that switches wins a game

    loop {//start simulating a game
        //this is the turn for the player that does not switch
        total_wins_without_switching = if rand::thread_rng().gen_range(0,NUM_OF_DOORS) == rand::thread_rng().gen_range(0,NUM_OF_DOORS) { //was the fist guess the car?
            total_wins_without_switching + 1 //yes! we won, because we stuck to our decision of picking the first door!
        } else {
            total_wins_without_switching //alas, we picked the wrong door, but didn't switch :(
        };

        //this is the turn for the player that switches
        total_wins_with_switching = if rand::thread_rng().gen_range(0,NUM_OF_DOORS) != rand::thread_rng().gen_range(0,NUM_OF_DOORS) {//was the first guess the car?
            total_wins_with_switching +1//no! and we switched to the door that was not our fist guess, so we won!
        } else {
            total_wins_with_switching//yes, but we chose to switch :( we failed
        };
        total_games += 1;
        println!("no switch wins: {:.2}%, switch wins: {:.2}%",//prints numbers with maximum of two decimal places
                 (total_wins_without_switching as f64 / total_games as f64) * 100.0,//percentage of wins without switching
                 (total_wins_with_switching as f64 / total_games as f64) * 100.0);//percentage of wins with switching
    }
}
