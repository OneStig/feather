use std::env;
use std::fs;
use std::collections::HashMap;
use serde::Deserialize;

macro_rules! create_currency_formats {
    ($($code:expr => $format:expr),*) => {{
        let mut map = HashMap::new();
        $(
            map.insert($code.to_string(), $format.to_string());
        )*
        map
    }};
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExchangeRates {
    pub conversion_rates: HashMap<String, f64>,
}

const EXCHANGE_FILE: &str = "exchange.json";
const EXCHANGE_API: &str = "https://v6.exchangerate-api.com/v6/{}/latest/USD";

async fn load_json() -> Result<ExchangeRates, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(EXCHANGE_FILE)?;
    let rates: ExchangeRates = serde_json::from_str(&data)?;
    Ok(rates)
}

pub async fn refresh_json() -> Result<ExchangeRates, Box<dyn std::error::Error>> {
    let response = reqwest::get(
        EXCHANGE_API.replace("{}", &env::var("EXCHANGERATE_TOKEN").expect("Exchange rate token missing"))
    ).await?.text().await?;
    let rates: ExchangeRates = serde_json::from_str(&response)?;
    fs::write(EXCHANGE_FILE, response)?;
    Ok(rates)
}

pub async fn load_exchange_rates() -> Result<(ExchangeRates, HashMap<String, String>), Box<dyn std::error::Error>> {
    let currency_formats: HashMap<String, String> = create_currency_formats!(
        "USD" => "${}",
        "AED" => "{} د.إ",
        "AFN" => "{} AFN",
        "ALL" => "{} ALL",
        "AMD" => "{} AMD",
        "ANG" => "{} ƒ",
        "AOA" => "{} AOA",
        "ARS" => "ARG${}",
        "AUD" => "A${}",
        "AWG" => "{} ƒ",
        "AZN" => "{} ₼",
        "BAM" => "{} KM",
        "BBD" => "${}",
        "BDT" => "{} ৳",
        "BGN" => "{} BGN",
        "BHD" => "{} .د.ب",
        "BIF" => "{} BIF",
        "BMD" => "${}",
        "BND" => "{} $",
        "BOB" => "{} Bs",
        "BRL" => "R${}",
        "BSD" => "${}",
        "BTN" => "{} Nu.",
        "BWP" => "{} P",
        "BYN" => "{} Br",
        "BZD" => "${}",
        "CAD" => "C${}",
        "CDF" => "{} CDF",
        "CHF" => "CHF{}",
        "CLP" => "CLP${}",
        "CNY" => "¥{}",
        "COP" => "COL${}",
        "CRC" => "{} ₡",
        "CUP" => "{} CUP",
        "CVE" => "{} CVE",
        "CZK" => "{} Kč",
        "DJF" => "{} DJF",
        "DKK" => "{} kr",
        "DOP" => "{} RD$",
        "DZD" => "{} DZD",
        "EGP" => "{} E£",
        "ERN" => "{} Nfk",
        "ETB" => "{} Br",
        "EUR" => "€{}",
        "FJD" => "{} FJ$",
        "FKP" => "{} £",
        "FOK" => "{} kr",
        "GBP" => "£{}",
        "GEL" => "{} ₾",
        "GGP" => "£{}",
        "GHS" => "{} ₵",
        "GIP" => "{} £",
        "GMD" => "{} D",
        "GNF" => "{} FG",
        "GTQ" => "{} Q",
        "GYD" => "{} GY$",
        "HKD" => "HK${}",
        "HNL" => "{} L",
        "HRK" => "{} kn",
        "HTG" => "{} G",
        "HUF" => "{} Ft",
        "IDR" => "{} Rp",
        "ILS" => "₪{}",
        "IMP" => "£{}",
        "INR" => "₹{}",
        "IQD" => "{} ع.د",
        "IRR" => "{} ﷼",
        "ISK" => "{} kr",
        "JEP" => "£{}",
        "JMD" => "{} J$",
        "JOD" => "{} د.ا",
        "JPY" => "¥{}",
        "KES" => "{} KSh",
        "KGS" => "{} с",
        "KHR" => "{} ៛",
        "KID" => "A${}",
        "KMF" => "{} CF",
        "KRW" => "₩{}",
        "KWD" => "{} د.ك",
        "KYD" => "{} CI$",
        "KZT" => "{} ₸",
        "LAK" => "{} ₭",
        "LBP" => "{} ل.ل",
        "LKR" => "{} Rs",
        "LRD" => "{} L$",
        "LSL" => "{} L",
        "LYD" => "{} ل.د",
        "MAD" => "{} MAD",
        "MDL" => "{} MDL",
        "MGA" => "{} Ar",
        "MKD" => "{} ден",
        "MMK" => "{} K",
        "MNT" => "{} ₮",
        "MOP" => "{} MOP$",
        "MRU" => "{} UM",
        "MUR" => "{} ₨",
        "MVR" => "{} ރ.",
        "MWK" => "{} MK",
        "MXN" => "MX${}",
        "MYR" => "RM{}",
        "MZN" => "{} MT",
        "NAD" => "{} N$",
        "NGN" => "{} ₦",
        "NIO" => "{} C$",
        "NOK" => "{} kr",
        "NPR" => "{} NPR",
        "NZD" => "NZ${}",
        "OMR" => "{} ر.ع.",
        "PAB" => "${}",
        "PEN" => "{} S/",
        "PGK" => "{} PGK",
        "PHP" => "₱{}",
        "PKR" => "{} Rs",
        "PLN" => "{} zł",
        "PYG" => "{} ₲",
        "QAR" => "{} ر.ق",
        "RON" => "{} L",
        "RSD" => "{} din",
        "RUB" => "{} ₽",
        "RWF" => "{} FRw",
        "SAR" => "﷼{}",
        "SBD" => "{} SI$",
        "SCR" => "{} ₨",
        "SDG" => "{} ج.س.",
        "SEK" => "{} kr",
        "SGD" => "S${}",
        "SHP" => "{} £",
        "SLE" => "{} Le",
        "SLL" => "{} Le",
        "SOS" => "{} Sh",
        "SRD" => "{} SR$",
        "SSP" => "{} £",
        "STN" => "{} Db",
        "SYP" => "{} £S",
        "SZL" => "{} E",
        "THB" => "฿{}",
        "TJS" => "{} ЅМ",
        "TMT" => "{} T",
        "TND" => "{} د.ت",
        "TOP" => "{} T$",
        "TRY" => "₺{}",
        "TTD" => "{} TT$",
        "TVD" => "A${}",
        "TWD" => "NT${}",
        "TZS" => "{} TSh",
        "UAH" => "{} ₴",
        "UGX" => "{} USh",
        "UYU" => "{} $U",
        "UZS" => "{} сўм",
        "VES" => "{} Bs.S",
        "VND" => "{} ₫",
        "VUV" => "{} VT",
        "WST" => "{} WS$",
        "XAF" => "{} FCFA",
        "XCD" => "{} EC$",
        "XDR" => "{} XDR",
        "XOF" => "{} CFA",
        "XPF" => "{} CFP",
        "YER" => "{} ﷼",
        "ZAR" => "R{}",
        "ZMW" => "{} ZK",
        "ZWL" => "{} Z$"
    );

    let rates: ExchangeRates = match load_json().await {
        Ok(rates) => {
            println!("Loaded local {}", EXCHANGE_FILE);
            rates
        },
        Err(_) => {
            println!("Could not find {}", EXCHANGE_FILE);
            match refresh_json().await {
                Ok(rates) => {
                    println!("Wrote new {}", EXCHANGE_FILE);
                    rates
                },
                Err(e) => {
                    println!("Failed to fetch exchange rates");
                    return Err(e);
                }
            }
        }
    };

    Ok((rates, currency_formats))
}