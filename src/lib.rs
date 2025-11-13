use serde::{Serialize, Deserialize};
use validator::Validate;
use std::fmt::Display;
use std::fmt::Formatter;
use header::Header;
use buchung::Buchung;
use debkred::DebKred;
#[macro_use]
extern crate lazy_static;

pub mod header;
pub mod buchung;
pub mod debkred;

#[derive(Clone, Debug, PartialEq, Validate, Serialize, Deserialize)]
pub struct Buchungsstapel {
    pub header: Header,
    pub buchungen: Vec<Buchung>,
}

impl Default for Buchungsstapel{
    fn default() -> Self {
        Buchungsstapel {
            header: Header::default(),
            buchungen: Vec::new(),
        }
    }
}

impl Display for Buchungsstapel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}",self.header))?;
        for buchung in &self.buchungen {
            buchung.fmt(f)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Buchungsstapel {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split;
        if value.contains("\r\n") {
            split = value.split("\r\n");
        }else{
            split = value.split("\n");
        }
        let vec: Vec<&str> = split.collect();
        
        let header_str: &str = vec.get(0).unwrap();
        let header = Header::try_from(header_str).unwrap();

        let mut buchungen: Vec<Buchung> = Vec::new();
        for input in vec.iter().skip(2) {
            let input2: String = input.to_string();
            let input3: &str = &input2.replace(',',".");
            if !input3.is_empty() {
                let buchung = Buchung::try_from(input3).unwrap();
                buchungen.push(buchung);
            }
        }

        let stapel = Buchungsstapel{
            header,
            buchungen,
        };
        Ok(stapel)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebKred_Stamm{
    header: Header,
    debitoren_kreditoren: Vec<DebKred>,
}

impl Default for DebKred_Stamm{
    fn default() -> Self {
        DebKred_Stamm {
            header: Header::default(),
            debitoren_kreditoren: Vec::new(),
        }
    }
}

impl Display for DebKred_Stamm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}",self.header))?;
        for debkred in &self.debitoren_kreditoren {
            debkred.fmt(f)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for DebKred_Stamm {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split;
        if value.contains("\r\n") {
            split = value.split("\r\n");
        }else{
            split = value.split("\n");
        }
        let vec: Vec<&str> = split.collect();
        
        let header_str: &str = vec.get(0).unwrap();
        let header = Header::try_from(header_str).unwrap();

        let mut debitoren_kreditoren: Vec<DebKred> = Vec::new();
        for input in vec.iter().skip(2) {
            let input2: String = input.to_string();
            let input3: &str = &input2.replace(',',".");
            if !input3.is_empty() {
                let debkred = DebKred::try_from(input3).unwrap();
                debitoren_kreditoren.push(debkred);
            }
        }

        let stapel = DebKred_Stamm{
            header,
            debitoren_kreditoren,
        };
        Ok(stapel)
    }
}

#[test]
fn valid_header() {
    use header::Kennzeichen;
    let str = r#""EXTF";510;21;"Buchungsstapel";7;20211106165314647;;"";"";"";1000;1;20190101;4;20190101;20191231;"";"";;;;"";;"";;;"";;;"";"""#;
    
    let result = Header::try_from(str);
    assert!(result.is_ok());
    let header = result.unwrap();
    header.validate().unwrap();
    assert_eq!(header.kennzeichen, Kennzeichen::EXTF);
    assert_eq!(header.versionsnummer, 510);
    assert_eq!(header.format_kategorie, 21);
    assert_eq!(header.format_name, "Buchungsstapel");
    assert_eq!(header.format_version, 7);
    assert_eq!(header.erzeugt_am, chrono::NaiveDateTime::parse_from_str("20211106165314647","%Y%m%d%H%M%S%3f").unwrap());
    assert_eq!(header.beraternummer, 1000);
    assert_eq!(header.mandantennummer, 1);
    assert_eq!(header.wj_beginn, chrono::NaiveDate::from_ymd(2019,1,1));
    assert_eq!(header.sachkontenlänge, 4);
    assert_eq!(header.datum_von, chrono::NaiveDate::from_ymd(2019,1,1));
    assert_eq!(header.datum_bis, chrono::NaiveDate::from_ymd(2019,12,31));
    assert_eq!(header.buchungstyp, None);
    assert_eq!(header.rechnungslegungszweck, None);
    assert_eq!(header.festschreibung, None);
}

#[test]
fn full_cycle_header() {
    let header = Header{
        format_name: "Buchungsstapel".to_string(),
        erzeugt_am: chrono::Local::now().naive_local(),
        beraternummer: 1000,
        mandantennummer: 1,
        wj_beginn: chrono::NaiveDate::from_ymd(2019,1,1),
        sachkontenlänge: 4,
        datum_von: chrono::NaiveDate::from_ymd(2019,1,1),
        datum_bis: chrono::NaiveDate::from_ymd(2019,12,31),
        ..Default::default()
    };
    let data = format!("{}",header);
    println!("{}",data);
    let mut header2 = Header::try_from(data.as_str()).unwrap();
    //need to overwrite ezeugt_am because of the higher precision
    header2.erzeugt_am = header.erzeugt_am.clone();
    println!("{}",header2);
    assert_eq!(header, header2);
    assert_eq!(format!("{}",header),format!("{}",header2));
}
#[test]
#[should_panic]
fn invalid_header() {
    let str = r#""INVALIDFORMAT";510;21;"Buchungsstapel";7;20211106165314647;;"";"";"";1000;1;20190101;4;20190101;20191231;"";"";;;;"";;"";;;"";;;"";"""#;
    let result = Header::try_from(str);
    assert!(result.is_ok());
    let header = result.unwrap();
    header.validate().unwrap();
}

#[test]
fn einzelbuchung() {
    use buchung::SollHabenKennzeichen;
    use header::Kennzeichen;

    let header = Header{
        kennzeichen: Kennzeichen::EXTF,
        versionsnummer: 700,
        format_kategorie: 21,
        format_name: "Buchungsstapel".to_string(),
        format_version: 7,
        erzeugt_am: chrono::Local::now().naive_local(),
        beraternummer: 1000,
        mandantennummer: 1,
        wj_beginn: chrono::NaiveDate::from_ymd(2019,1,1),
        sachkontenlänge: 4,
        datum_von: chrono::NaiveDate::from_ymd(2019,1,1),
        datum_bis: chrono::NaiveDate::from_ymd(2019,12,31),
        ..Default::default()
    };
    let buchung = Buchung{
        soll_haben_kennzeichen: SollHabenKennzeichen::Soll,
        umsatz: 100.0,
        beleg_datum: chrono::NaiveDate::from_ymd(2000, 2, 29),
        konto: 1800,
        gegenkonto: 1420,
        buchungstext: Some("zahlung 123".to_string()),
        ..Default::default()
    };
    let stapel = Buchungsstapel{
        header: header,
        buchungen: vec![buchung],
    };
    let _str = format!("{}", stapel);
}

#[test]
fn test_extf_buchungstapel() {
    use std::io::Read;
    let mut f = std::fs::File::open("./test-data/EXTF_Buchungsstapel.csv").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    //length in windows encoding
    assert_eq!(buffer.len(), 21514);
    let (cow, encoding_used, had_errors) = encoding_rs::WINDOWS_1252.decode(&buffer);
    assert_eq!(had_errors, false);
    assert_eq!(encoding_used, encoding_rs::WINDOWS_1252);
    //length in utf-8 encoding
    assert_eq!(cow.len(), 21556);
    let str: String = cow.to_string();
    println!("{}", str);
    println!("done.");
    let _stapel: Buchungsstapel = Buchungsstapel::try_from(str.as_str()).unwrap();
    println!("{:?}", _stapel);
}

#[test]
fn test_sage_export_buchungstapel() {
    let input = r#""EXTF";510;21;"Buchungsstapel";7;20211106165344208;;"";"";"";1000;1;20170101;4;20170101;20171231;"";"";;;;"";;"";;;"";;;"";""
Umsatz (ohne Soll/Haben-Kz);Soll/Haben-Kennzeichen;WKZ Umsatz;Kurs;Basis-Umsatz;WKZ Basis-Umsatz;Konto;Gegenkonto (ohne BU-Schlüssel);BU-Schlüssel;Belegdatum;Belegfeld 1;Belegfeld 2;Skonto;Buchungstext;Postensperre;Diverse Adressnummer;Geschäftspartnerbank;Sachverhalt;Zinssperre;Beleglink;Beleginfo - Art 1;Beleginfo - Inhalt 1;Beleginfo - Art 2;Beleginfo - Inhalt 2;Beleginfo - Art 3;Beleginfo - Inhalt 3;Beleginfo - Art 4;Beleginfo - Inhalt 4;Beleginfo - Art 5;Beleginfo - Inhalt 5;Beleginfo - Art 6;Beleginfo - Inhalt 6;Beleginfo - Art 7;Beleginfo - Inhalt 7;Beleginfo - Art 8;Beleginfo - Inhalt 8;KOST1 - Kostenstelle;KOST2 - Kostenstelle;Kost-Menge;EU-Land u. UStID;EU-Steuersatz;Abw. Versteuerungsart;Sachverhalt L+L;Funktionsergänzung L+L;BU 49 Hauptfunktionstyp;BU 49 Hauptfunktionsnummer;BU 49 Funktionsergänzung;Zusatzinformation - Art 1;Zusatzinformation- Inhalt 1;Zusatzinformation - Art 2;Zusatzinformation- Inhalt 2;Zusatzinformation - Art 3;Zusatzinformation- Inhalt 3;Zusatzinformation - Art 4;Zusatzinformation- Inhalt 4;Zusatzinformation - Art 5;Zusatzinformation- Inhalt 5;Zusatzinformation - Art 6;Zusatzinformation- Inhalt 6;Zusatzinformation - Art 7;Zusatzinformation- Inhalt 7;Zusatzinformation - Art 8;Zusatzinformation- Inhalt 8;Zusatzinformation - Art 9;Zusatzinformation- Inhalt 9;Zusatzinformation - Art 10;Zusatzinformation- Inhalt 10;Zusatzinformation - Art 11;Zusatzinformation- Inhalt 11;Zusatzinformation - Art 12;Zusatzinformation- Inhalt 12;Zusatzinformation - Art 13;Zusatzinformation- Inhalt 13;Zusatzinformation - Art 14;Zusatzinformation- Inhalt 14;Zusatzinformation - Art 15;Zusatzinformation- Inhalt 15;Zusatzinformation - Art 16;Zusatzinformation- Inhalt 16;Zusatzinformation - Art 17;Zusatzinformation- Inhalt 17;Zusatzinformation - Art 18;Zusatzinformation- Inhalt 18;Zusatzinformation - Art 19;Zusatzinformation- Inhalt 19;Zusatzinformation - Art 20;Zusatzinformation- Inhalt 20;Stück;Gewicht;Zahlweise;Forderungsart;Veranlagungsjahr;Zugeordnete Fälligkeit;Skontotyp;Auftragsnummer;Buchungstyp;USt-Schlüssel (Anzahlungen);EU-Land (Anzahlungen);Sachverhalt L+L (Anzahlungen);EU-Steuersatz (Anzahlungen);Erlöskonto (Anzahlungen);Herkunft-Kz;Buchungs GUID;KOST-Datum;SEPA-Mandatsreferenz;Skontosperre;Gesellschaftername;Beteiligtennummer;Identifikationsnummer;Zeichnernummer;Postensperre bis;Bezeichnung SoBil-Sachverhalt;Kennzeichen SoBil-Buchung;Festschreibung;Leistungsdatum;Datum Zuord. Steuerperiode
25000,00;"S";"EUR";;;"";1800;1460;"";0610;"1";"";;"1";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;
25000,00;"S";"EUR";;;"";1460;2900;"";1712;"2";"";;"2";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;
25000,00;"S";"EUR";;;"";2900;1460;"";1712;"3";"";;"Storno von Journal";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;
25000,00;"S";"EUR";;;"";1460;2900;"";0610;"4";"";;"2";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;
25000,00;"S";"EUR";;;"";1810;1800;"";2410;"5";"";;"3";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;"#;
    let stapel: Buchungsstapel = Buchungsstapel::try_from(input).unwrap();
    println!("{:?}",stapel);
    println!("{}", serde_json::to_string_pretty(&stapel).unwrap());
    // panic!("x");
}

#[test]
fn test_sage_export_buchung(){
    let input = r#"389,92;"H";"EUR";;;"";70013;670;"";2912;"118";"280118";;"20171229_cyberport";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;"#;
    let input2 = r#"74,08;"H";"EUR";;;"";70013;1406;"";2912;"118";"280118";;"20171229_cyberport";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;"#;
    let input3 = r#"464,00;"H";"EUR";;;"";1810;70013;"";2912;"118";"";;"20171229_cyberport";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;"#;    
    let _b1 = Buchung::try_from(input).unwrap();
    let _b2 = Buchung::try_from(input2).unwrap();
    let _b3 = Buchung::try_from(input3).unwrap();
}

#[test]
fn test_extf_debkred() {
    use std::io::Read;
    let mut f = std::fs::File::open("./test-data/EXTF_DebKred_Stamm.csv").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    //length in windows encoding
    assert_eq!(buffer.len(), 10294);
    let (cow, encoding_used, had_errors) = encoding_rs::WINDOWS_1252.decode(&buffer);
    assert_eq!(had_errors, false);
    assert_eq!(encoding_used, encoding_rs::WINDOWS_1252);
    //length in utf-8 encoding
    assert_eq!(cow.len(), 10371);
    let str: String = cow.to_string();
    println!("{}", str);
    println!("done.");
    let stammdaten: DebKred_Stamm = DebKred_Stamm::try_from(str.as_str()).unwrap();
    let first = stammdaten.debitoren_kreditoren.get(0).unwrap();
    assert_eq!(first.konto, 10000);
}
