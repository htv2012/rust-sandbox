//! Demo: Bye default, the match on enum must be exhausted

enum WineGrapes {
    CabernetFrance,
    Tannat,
    Merlot,
}

fn main() {
    taste_wine(WineGrapes::CabernetFrance);
    taste_wine(WineGrapes::Tannat);
    taste_wine(WineGrapes::Merlot);
}

fn taste_wine(grape: WineGrapes) {
    let name: &str = match grape {
        WineGrapes::CabernetFrance => "Cabernet France",
        WineGrapes::Tannat => "Tannat",
        WineGrapes::Merlot => "Merlot",
    };
    println!("This is {} wine", name);
}
