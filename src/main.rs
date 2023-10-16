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

const SEJM_DISTRICTS: [(&str, i32); 41]= [
    ("Powiaty: bolesławiecki, głogowski, jaworski, jeleniogórski, kamiennogórski, legnicki, lubański, lubiński, lwówecki, polkowicki, zgorzelecki, złotoryjski, Miasta na prawach powiatu: Jelenia Góra, Legnica", 12),
    ("Powiaty: dzierżoniowski, kłodzki, świdnicki, wałbrzyski, ząbkowicki", 8),
    ("Powiaty: górowski, milicki, oleśnicki, oławski, strzeliński, średzki, trzebnicki, wołowski, wrocławski, Miasto na prawach powiatu: Wrocław", 14),
    ("Powiaty: bydgoski, inowrocławski, mogileński, nakielski, sępoleński, świecki, tucholski, żniński, Miasto na prawach powiatu: Bydgoszcz", 12),
    ("Powiaty: aleksandrowski, brodnicki, chełmiński, golubsko-dobrzyński, grudziądzki, lipnowski, radziejowski, rypiński, toruński, wąbrzeski, włocławski, Miasta na prawach powiatu: Grudziądz, Toruń, Włocławek", 13),
    ("Powiaty: janowski, kraśnicki, lubartowski, lubelski, łęczyński, łukowski, opolski, puławski, rycki, świdnicki, Miasto na prawach powiatu: Lublin", 15),
    ("Powiaty: bialski, biłgorajski, chełmski, hrubieszowski, krasnostawski, parczewski, radzyński, tomaszowski, włodawski, zamojski, Miasta na prawach powiatu: Biała Podlaska, Chełm, Zamość", 12),
    ("Województwo lubuskie", 12),
    ("Powiaty: brzeziński, łódzki wschodni, Miasto na prawach powiatu: Łódź", 10),
    ("Powiaty: bełchatowski, opoczyński, piotrkowski, radomszczański, rawski, skierniewicki, tomaszowski, Miasta na prawach powiatu: Piotrków Trybunalski, Skierniewice", 9),
    ("Powiaty: kutnowski, łaski, łęczycki, łowicki, pabianicki, pajęczański, poddębicki, sieradzki, wieluński, wieruszowski, zduńskowolski, zgierski", 12),
    ("Powiaty: chrzanowski, myślenicki, oświęcimski, suski, wadowicki", 8),
    ("Powiaty: krakowski, miechowski , olkuski, Miasto na prawach powiatu: Kraków", 14),
    ("Powiaty: gorlicki, limanowski, nowosądecki, nowotarski, tatrzański, Miasto na prawach powiatu: Nowy Sącz", 10),
    ("Powiaty: bocheński, brzeski, dąbrowski, proszowicki, tarnowski, wielicki, Miasto na prawach powiatu: Tarnów", 9),
    ("Powiaty: ciechanowski, gostyniński, mławski, płocki, płoński, przasnyski, sierpecki, sochaczewski, żuromiński, żyrardowski, Miasto na prawach powiatu: Płock", 10),
    ("Powiaty: białobrzeski, grójecki, kozienicki, lipski, przysuski, radomski, szydłowiecki, zwoleński, Miasto na prawach powiatu: Radom", 9),
    ("Powiaty: garwoliński, łosicki, makowski, miński, ostrołęcki, ostrowski, pułtuski, siedlecki, sokołowski, węgrowski, wyszkowski, Miasta na prawach powiatu: Ostrołęka, Siedlce", 12),
    ("Miasto na prawach powiatu: Warszawa (oraz obwody głosowania za granicą)", 20),
    ("Powiaty: grodziski, legionowski, nowodworski, otwocki, piaseczyński, pruszkowski, warszawski zachodni, wołomiński", 12),
    ("Województwo opolskie", 12),
    ("Powiaty: bieszczadzki, brzozowski, jarosławski, jasielski, krośnieński, leski, lubaczowski, przemyski, przeworski, sanocki, Miasta na prawach powiatu: Krosno, Przemyśl", 11),
    ("Powiaty: dębicki, kolbuszowski, leżajski, łańcucki, mielecki, niżański, ropczycko-sędziszowski, rzeszowski, stalowowolski, strzyżowski, tarnobrzeski, Miasta na prawach powiatu: Rzeszów, Tarnobrzeg", 15),
    ("Województwo podlaskie", 14),
    ("Powiaty: gdański, kwidzyński, malborski, nowodworski, starogardzki, sztumski, tczewski, Miasta na prawach powiatu: Gdańsk, Sopot (oraz obwody głosowania na polskich statkach morskich)", 12),
    ("Powiaty: bytowski, chojnicki, człuchowski, kartuski, kościerski, lęborski, pucki, słupski, wejherowski, Miasta na prawach powiatu: Gdynia, Słupsk (oraz obwody głosowania na polskich statkach morskich)", 14),
    ("Powiaty: bielski, cieszyński, pszczyński, żywiecki, Miasto na prawach powiatu: Bielsko-Biała", 9),
    ("Powiaty: częstochowski, kłobucki, lubliniecki, myszkowski, Miasto na prawach powiatu: Częstochowa", 7),
    ("Powiaty: gliwicki, tarnogórski, Miasta na prawach powiatu: Bytom, Gliwice, Zabrze", 9),
    ("Powiaty: mikołowski, raciborski, rybnicki, wodzisławski, Miasta na prawach powiatu: Jastrzębie-Zdrój, Rybnik, Żory", 9),
    ("Powiaty: bieruńsko-lędziński, Miasta na prawach powiatu: Chorzów, Katowice, Mysłowice, Piekary Śląskie, Ruda Śląska, Siemianowice Śląskie, Świętochłowice, Tychy", 12),
    ("Powiaty: będziński, zawierciański, Miasta na prawach powiatu: Dąbrowa Górnicza, Jaworzno, Sosnowiec", 9),
    ("Województwo świętokrzyskie", 16),
    ("Powiaty: bartoszycki, braniewski, działdowski, elbląski, iławski, lidzbarski, nowomiejski, ostródzki, Miasto na prawach powiatu: Elbląg", 8),
    ("Powiaty: ełcki, giżycki, gołdapski, kętrzyński, mrągowski, nidzicki, olecki, olsztyński, piski, szczycieński, węgorzewski, Miasto na prawach powiatu: Olsztyn", 10),
    ("Powiaty: gostyński, jarociński, kaliski, kępiński, kościański, krotoszyński, leszczyński, ostrowski, ostrzeszowski, pleszewski, rawicki, Miasta na prawach powiatu: Kalisz, Leszno", 12),
    ("Powiaty: gnieźnieński, kolski, koniński, słupecki, średzki, śremski, turecki, wrzesiński, Miasto na prawach powiatu: Konin", 9),
    ("Powiaty: chodzieski, czarnkowsko-trzcianecki, grodziski, międzychodzki, nowotomyski, obornicki, pilski, szamotulski, wągrowiecki, wolsztyński, złotowski", 9),
    ("Powiat: poznański, Miasto na prawach powiatu: Poznań", 10),
    ("Powiaty: białogardzki, choszczeński, drawski, kołobrzeski, koszaliński, sławieński, szczecinecki, świdwiński, wałecki, Miasto na prawach powiatu: Koszalin", 8),
    ("Powiaty: goleniowski, gryficki, gryfiński, kamieński, łobeski, myśliborski, policki, pyrzycki, stargardzki, Miasta na prawach powiatu: Szczecin, Świnoujście (oraz obwody głosowania na polskich statkach morskich)", 12),
];

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
    let assigned_seats: Option<HashMap<String, u64>> = dhondt(&results, seats);
    println!("{:?}", assigned_seats);

    // for each district calculate number of seats with dhondt method
    let mut districts: HashMap<String, HashMap<String, u64>> = HashMap::new();
    for (district, seats) in SEJM_DISTRICTS.iter() {
        let assigned_seats = dhondt(&results, *seats as u64).unwrap();
        districts.insert(district.to_string(), assigned_seats);
    }

    // println!("{:?}", districts);

    // calculate total number of seats for each party
    let mut total_seats: HashMap<String, u64> = HashMap::new();
    for (_, assigned_seats) in districts.iter() {
        for (party, seats) in assigned_seats.iter() {
            *total_seats.entry(party.to_string()).or_insert(0) += seats;
        }
    }

    println!("{:?}", total_seats);

}
