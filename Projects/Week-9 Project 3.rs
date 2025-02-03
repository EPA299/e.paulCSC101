fn main() {
    let names = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!(
        "| {:<3} | {:<25} | {:<20} | {:<15} |",
        "S/N", "Name of Commissioner", "Ministry", "Geopolitical Zone"
    );
    println!("|-----|-------------------------|----------------------|-----------------|");

    for i in 0..names.len() {
        println!(
            "| {:<3} | {:<25} | {:<20} | {:<15} |",
            i + 1,
            names[i],
            ministries[i],
            geopolitical_zones[i]
        );
    }
}
