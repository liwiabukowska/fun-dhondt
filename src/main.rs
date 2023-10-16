use std::{cmp, collections::HashMap};

// dhondt method
pub fn dhondt(results: &HashMap<String, f64>, seats: u64) -> Option<HashMap<String, u64>> {
    if results.is_empty() {
        return None;
    }

    let mut assigned_seats = HashMap::<String, u64>::new();

    let mut table = {
        let mut table = HashMap::<String, Vec<f64>>::new();
        for (party, &votes) in results {
            table.insert((*party).to_string(), vec![votes]);
        }
        table
    };

    for _ in 0..seats {
        let key = table
            .iter()
            .max_by(|(_, votes_a), (_, votes_b)| {
                let last_a = votes_a.last().unwrap();
                let last_b = votes_b.last().unwrap();
                last_a.partial_cmp(last_b).unwrap_or(cmp::Ordering::Equal)
            })
            .unwrap()
            .0
            .clone();

        if let Some(party_votes) = table.get_mut(&key) {
            let party_len = party_votes.len();
            let first = party_votes[0];
            party_votes.push(first / (party_len + 1) as f64);
        }

        assigned_seats.insert(key.clone(), assigned_seats.get(&key).unwrap_or(&0) + 1);
    }

    Some(assigned_seats)
}

fn main() {
    // populate dhondt from arguments : seats party1 votes1 party2 votes2 ...
    let (seats, results) = {
        let mut results = HashMap::<String, f64>::new();
        
        let mut args = std::env::args();
        args.next();
        let seats = args
            .next()
            .expect("podaj liczbe mandatow")
            .parse::<u64>()
            .unwrap();
        while let Some(party) = args.next() {
            let votes = args
                .next()
                .unwrap_or_else(|| panic!("podaj wynik partii {party}"))
                .parse::<f64>()
                .unwrap();
            results.insert(party, votes);
        }
        (seats, results)
    };

    // println!("{:?}", results);
    let assigned_seats = dhondt(&results, seats);
    println!("{:?}", assigned_seats);
}
