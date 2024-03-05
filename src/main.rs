#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
struct PaypalCsv {
    Date : String,
    Nom : String,
    #[serde(rename = "Avant commission")]
    Montant : String,
}

#[derive(Debug, serde::Serialize)]
struct Paypal {
    date : String,
    nom : String,
    montant : String
}

fn read_paypal(file_path: &str) -> Vec<Paypal> {
    let mut tab = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b',').from_reader(file);
    for result in rdr.deserialize::<PaypalCsv>() {
        let r = result.unwrap();
        if r.Montant.starts_with('-') {
            tab.push(Paypal {
                date : r.Date,
                nom : r.Nom,
                montant : r.Montant[1..].to_string()
            });
        }
    }
    tab
}

fn main() {
    let file = std::fs::File::create("titi.csv").unwrap();
    let t = read_paypal("toto.csv");
    let mut wtr = csv::Writer::from_writer(file);
    for r in t {
        println!("date:{} montant:{} nom:{}",r.date,r.montant,r.nom);
        wtr.serialize(r).unwrap();
    }
    wtr.flush().unwrap();
}
