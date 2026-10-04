struct dealerType {
    id: u32,
    chips: u32,
}

struct playerType {
    id: u32,
    name: String,
    chips: u32
}



pub fn create_dealer() -> dealerType {
    let dealer = dealerType {
        id: 1,  
        chips: 1000
    };

    return dealer;
}

pub fn create_player(entered_name: String) -> playerType {
    let player = playerType {
        id: 2,
        name: entered_name,
        chips: 1000
    };

    return player;
}