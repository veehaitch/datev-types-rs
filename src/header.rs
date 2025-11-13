use serde::{Serialize, Deserialize};
use validator::Validate;
use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

lazy_static! {
  static ref FORMATNAME: Regex = Regex::new(r#"^(Buchungsstapel|Wiederkehrende Buchungen|Debitoren/Kreditoren|Sachkontenbeschriftungen|Zahlungsbedingungen|Diverse Adressen)$"#).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
  pub kennzeichen: Kennzeichen,
  pub versionsnummer: u32,
  /// 16 = Debitoren-/Kreditoren
  /// 20 = Sachkontenbeschriftungen
  /// 21 = Buchungsstapel
  /// 46 = Zahlungsbedingungen
  /// 48 = Diverse Adressen
  /// 65 = Wiederkehrende Buchungen
  pub format_kategorie: u16,
  #[validate(regex = "FORMATNAME")]
  pub format_name: String,
  /// Debitoren-/Kreditoren = 5
  /// Sachkontenbeschriftungen = 3
  /// Buchungsstapel = 12
  /// Zahlungsbedingungen = 2
  /// Wiederkehrende Buchungen = 4
  /// Diverse Adressen = 2
  pub format_version: u16,
  /// Zeitstempel:
  /// YYYYMMDDHHMMSSFFF
  pub erzeugt_am: NaiveDateTime,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld1: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld2: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld3: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld4: Option<String>,
  /// Bereich 1001-9999999
  pub beraternummer: u32,
  /// Bereich 1-99999
  pub mandantennummer: u32,
  /// Wirtschaftsjahresbeginn
  /// Format: YYYYMMDD
  pub wj_beginn: NaiveDate,
  /// Nummernlänge der Sachkonten.
  /// Wert muss beim Import mit Konfiguration des Mandats in der DATEV App übereinstimmen.
  pub sachkontenlänge: u32,
  /// Beginn der Periode des Stapels
  /// Format: YYYYMMDD
  pub datum_von: NaiveDate,
  /// Ende der Periode des Stapels
  /// Format: YYYYMMDD
  pub datum_bis: NaiveDate,
  /// Bezeichnung des Stapels
  /// z.B. „Rechnungsausgang 09/2019“
  #[validate(length(min = 0, max = 30))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bezeichnung: Option<String>,
  /// Kürzel in Großbuchstaben des Bearbeiters
  /// z.B. "MM" für Max Mustermann
  #[validate(length(min = 0, max = 2))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub diktatkürzel: Option<String>,
  /// 1 = Finanzbuchführung (default)
  /// 2 = Jahresabschluss
  #[serde(skip_serializing_if = "Option::is_none")]
  pub buchungstyp: Option<BuchungsTyp>,
  /// 0 = unabhängig (default)
  /// 30 = Steuerrecht
  /// 40 = Kalkulatorik
  /// 50 = Handelsrecht
  /// 64 = IFRS
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rechnungslegungszweck: Option<u8>,
  /// 0 = keine Festschreibung
  /// 1 = Festschreibung (default)
  // #[serde(default = "default_festschreibung")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub festschreibung: Option<Festschreibung>,
  /// ISO-Code der Währung "EUR" = default
  #[validate(length(min = 0, max = 3))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wkz: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld5: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub derivatskennzeichen: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld6: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld7: Option<String>,
  /// Sachkontenrahmen der für die Bewegungsdaten verwendet wurde
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sachkontenrahmen: Option<String>,
  /// Falls eine spezielle DATEV Branchenlösung genutzt wird.
  #[validate(length(min = 0, max = 4))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id_der_branchenlösung: Option<String>, 	
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld8: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub leerfeld9: Option<String>,
  /// Verarbeitungskennzeichen der abgebenden Anwendung
  // z.B. „09/2019“
  #[validate(length(min = 0, max = 16))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub anwendungsinformation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum BuchungsTyp {
  Finanzbuchführung,
  Jahresabschluss,
}

impl Default for Header {
  fn default() -> Self {
      Header{
          kennzeichen: Kennzeichen::EXTF,
          versionsnummer: 700,
          format_kategorie: 21,
          format_name: String::from("Buchungsstapel"),
          format_version: 12,
          erzeugt_am: chrono::Local::now().naive_local(),
          leerfeld1: None,
          leerfeld2: None,
          leerfeld3: None,
          leerfeld4: None,
          beraternummer: 0,
          mandantennummer: 0,
          wj_beginn: NaiveDate::from_ymd(2000, 1, 1),
          sachkontenlänge: 0,
          datum_von: NaiveDate::from_ymd(2000, 1, 1),
          datum_bis: NaiveDate::from_ymd(2000, 12, 31),
          bezeichnung: None,
          diktatkürzel: None,
          buchungstyp: None,
          rechnungslegungszweck: None,
          festschreibung: None,
          wkz: None,
          leerfeld5: None,
          derivatskennzeichen: None,
          leerfeld6: None,
          leerfeld7: None,
          sachkontenrahmen: None,
          id_der_branchenlösung: None,
          leerfeld8: None,
          leerfeld9: None,
          anwendungsinformation: None,
      }
  }
}

impl Display for Header {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      write!(f, r#""{kennzeichen}";{versionsnummer};{format_kategorie};"{format_name}";{format_version};{erzeugt_am};{leerfeld1};{leerfeld2};{leerfeld3};{leerfeld4};{beraternummer};{mandantennummer};{wj_beginn};{sachkontenlänge};{datum_von};{datum_bis};{bezeichnung};{diktatkürzel};{buchungstyp};{rechnungslegungszweck};{festschreibung};{wkz};{leerfeld5};{derivatskennzeichen};{leerfeld6};{leerfeld7};{sachkontenrahmen};{id_der_branchenlösung};{leerfeld8};{leerfeld9};{anwendungsinformation}{newline}{desc}{newline}"#,
          kennzeichen=self.kennzeichen,
          versionsnummer=self.versionsnummer,
          format_kategorie=self.format_kategorie,
          format_name=self.format_name,
          format_version=self.format_version,
          erzeugt_am=self.erzeugt_am.format("%Y%m%d%H%M%S%3f"),
          leerfeld1=match &self.leerfeld1{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          leerfeld2=match &self.leerfeld2{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          leerfeld3=match &self.leerfeld3{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          leerfeld4=match &self.leerfeld4{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          beraternummer=self.beraternummer,
          mandantennummer=self.mandantennummer,
          wj_beginn=self.wj_beginn.format("%Y%m%d").to_string(),
          sachkontenlänge=self.sachkontenlänge,
          datum_von=self.datum_von.format("%Y%m%d").to_string(),
          datum_bis=self.datum_bis.format("%Y%m%d").to_string(),
          bezeichnung=match &self.bezeichnung{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          diktatkürzel=match &self.diktatkürzel{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          buchungstyp = match self.buchungstyp {
              Some(BuchungsTyp::Finanzbuchführung) => "1",
              Some(BuchungsTyp::Jahresabschluss) => "2",
              _ => "",
          },
          rechnungslegungszweck = match self.rechnungslegungszweck {
              Some(0) => "0",
              Some(30) => "30",
              Some(40) => "40",
              Some(50) => "50",
              Some(64) => "64",
              None => "",
              _ => "",
          },
          festschreibung= match self.festschreibung {
              Some(Festschreibung::KeineFestschreibung) => "0",
              Some(Festschreibung::Festschreibung) => "1",
              None => "",
          },
          wkz = match &self.wkz {
              Some(val) => val.to_string(),
              None => "".to_string(),
          },
          leerfeld5=match &self.leerfeld5{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          derivatskennzeichen=match &self.derivatskennzeichen{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          leerfeld6=match &self.leerfeld6{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          leerfeld7=match &self.leerfeld7{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          sachkontenrahmen=match &self.sachkontenrahmen{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          id_der_branchenlösung=match &self.id_der_branchenlösung{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          leerfeld8=match &self.leerfeld8{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          leerfeld9=match &self.leerfeld9{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          anwendungsinformation=match &self.anwendungsinformation{
            Some(x) => format!("{}", x),
            None => String::from(""),
          },
          newline = "\n",
          desc = "Umsatz (ohne Soll/Haben-Kz);Soll/Haben-Kennzeichen;WKZ Umsatz;Kurs;Basis-Umsatz;WKZ Basis-Umsatz;Konto;Gegenkonto (ohne BU-Schlüssel);BU-Schlüssel;Belegdatum;Belegfeld 1;Belegfeld 2;Skonto;Buchungstext;Postensperre;Diverse Adressnummer;Geschäftspartnerbank;Sachverhalt;Zinssperre;Beleglink;Beleginfo - Art 1;Beleginfo - Inhalt 1;Beleginfo - Art 2;Beleginfo - Inhalt 2;Beleginfo - Art 3;Beleginfo - Inhalt 3;Beleginfo - Art 4;Beleginfo - Inhalt 4;Beleginfo - Art 5;Beleginfo - Inhalt 5;Beleginfo - Art 6;Beleginfo - Inhalt 6;Beleginfo - Art 7;Beleginfo - Inhalt 7;Beleginfo - Art 8;Beleginfo - Inhalt 8;KOST1 - Kostenstelle;KOST2 - Kostenstelle;Kost-Menge;EU-Land u. UStID (Bestimmung);EU-Steuersatz (Bestimmung);Abw. Versteuerungsart;Sachverhalt L+L;Funktionsergänzung L+L;BU 49 Hauptfunktionstyp;BU 49 Hauptfunktionsnummer;BU 49 Funktionsergänzung;Zusatzinformation - Art 1;Zusatzinformation- Inhalt 1;Zusatzinformation - Art 2;Zusatzinformation- Inhalt 2;Zusatzinformation - Art 3;Zusatzinformation- Inhalt 3;Zusatzinformation - Art 4;Zusatzinformation- Inhalt 4;Zusatzinformation - Art 5;Zusatzinformation- Inhalt 5;Zusatzinformation - Art 6;Zusatzinformation- Inhalt 6;Zusatzinformation - Art 7;Zusatzinformation- Inhalt 7;Zusatzinformation - Art 8;Zusatzinformation- Inhalt 8;Zusatzinformation - Art 9;Zusatzinformation- Inhalt 9;Zusatzinformation - Art 10;Zusatzinformation- Inhalt 10;Zusatzinformation - Art 11;Zusatzinformation- Inhalt 11;Zusatzinformation - Art 12;Zusatzinformation- Inhalt 12;Zusatzinformation - Art 13;Zusatzinformation- Inhalt 13;Zusatzinformation - Art 14;Zusatzinformation- Inhalt 14;Zusatzinformation - Art 15;Zusatzinformation- Inhalt 15;Zusatzinformation - Art 16;Zusatzinformation- Inhalt 16;Zusatzinformation - Art 17;Zusatzinformation- Inhalt 17;Zusatzinformation - Art 18;Zusatzinformation- Inhalt 18;Zusatzinformation - Art 19;Zusatzinformation- Inhalt 19;Zusatzinformation - Art 20;Zusatzinformation- Inhalt 20;Stück;Gewicht;Zahlweise;Forderungsart;Veranlagungsjahr;Zugeordnete Fälligkeit;Skontotyp;Auftragsnummer;Buchungstyp;USt-Schlüssel (Anzahlungen);EU-Land (Anzahlungen);Sachverhalt L+L (Anzahlungen);EU-Steuersatz (Anzahlungen);Erlöskonto (Anzahlungen);Herkunft-Kz;Buchungs GUID;KOST-Datum;SEPA-Mandatsreferenz;Skontosperre;Gesellschaftername;Beteiligtennummer;Identifikationsnummer;Zeichnernummer;Postensperre bis;Bezeichnung SoBil-Sachverhalt;Kennzeichen SoBil-Buchung;Festschreibung;Leistungsdatum;Datum Zuord. Steuerperiode;Fälligkeit;Generalumkehr (GU);Steuersatz;Land;Abrechnungsreferenz;BVV-Position;EU-Land u. UStID (Ursprung);EU-Steuersatz (Ursprung);Abw. Skontokonto",
      )
  }
}

impl TryFrom<&str> for Header {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
      let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
          .has_headers(false).from_reader(value.as_bytes());
      
      //manual way
      let mut iter = rdr.records();
      if let Some(result) = iter.next() {
          let record = result.unwrap();
          let mut header = Header::default();
          //add values
          if let Some(val) = record.get(0) {
            if val.len() > 0 {
              header.kennzeichen = match val {
                "EXTF" => Kennzeichen::EXTF,
                "DTVF" => Kennzeichen::DTVF,
                _ => return Err("Kennzeichen ist weder EXTF noch DTVF"),
              };
            }
          }
          if let Some(val) = record.get(1) {
              header.versionsnummer = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(2) {
              header.format_kategorie = val.parse::<u16>().unwrap();
          }
          if let Some(val) = record.get(3) {
              header.format_name = val.to_string();
          }
          if let Some(val) = record.get(4) {
              header.format_version = val.parse::<u16>().unwrap();
          }
          if let Some(val) = record.get(5) {
              header.erzeugt_am = NaiveDateTime::parse_from_str(val, "%Y%m%d%H%M%S%3f").unwrap();
          }
          if let Some(val) = record.get(6) {
              if val.len() > 0 {
                header.leerfeld1 = Some(val.to_string());
              }
          }
          if let Some(val) = record.get(7) {
            if val.len() > 0 {
              header.leerfeld2 = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(8) {
            if val.len() > 0 {
              header.leerfeld3 = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(9) {
            if val.len() > 0 {
              header.leerfeld4 = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(10) {
              header.beraternummer = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(11) {
              header.mandantennummer = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(12) {
              header.wj_beginn = NaiveDate::parse_from_str(val, "%Y%m%d").unwrap();
          }
          if let Some(val) = record.get(13) {
              header.sachkontenlänge = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(14) {
            if val.len() > 0 {
              header.datum_von = NaiveDate::parse_from_str(val, "%Y%m%d").unwrap();
            }
          }
          if let Some(val) = record.get(15) {
            if val.len() > 0 {
              header.datum_bis = NaiveDate::parse_from_str(val, "%Y%m%d").unwrap();
            }
          }
          if let Some(val) = record.get(16) {
            if val.len() > 0 {
              header.bezeichnung = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(17) {
            if val.len() > 0 {
              header.diktatkürzel = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(18) {
              header.buchungstyp = match val {
                  "1" => Some(BuchungsTyp::Finanzbuchführung),
                  "2" => Some(BuchungsTyp::Jahresabschluss),
                  _ => None,
              };
          }
          if let Some(val) = record.get(19) {
              header.rechnungslegungszweck = match val {
                  "0" => Some(0),
                  "30" => Some(30),
                  "40" => Some(40),
                  "50" => Some(50),
                  "64" => Some(64),
                  _ => None,
              };
          }
          if let Some(val) = record.get(20) {
              header.festschreibung = match val {
                  "0" => Some(Festschreibung::KeineFestschreibung),
                  "1" => Some(Festschreibung::Festschreibung),
                  _ => None,
              };
          }
          if let Some(val) = record.get(21) {
              if !val.is_empty() {
                  header.wkz = Some(val.to_string());
              } else {
                  header.wkz = None;
              }
          }
          if let Some(val) = record.get(22) {
            if val.len() > 0 {
              header.leerfeld5 = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(23) {
            if val.len() > 0 {
              header.derivatskennzeichen = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(24) {
            if val.len() > 0 {
              header.leerfeld6 = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(25) {
            if val.len() > 0 {
              header.leerfeld7 = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(26) {
            if val.len() > 0 {
              header.sachkontenrahmen = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(27) {
            if val.len() > 0 {
              header.id_der_branchenlösung = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(28) {
            if val.len() > 0 {
              header.leerfeld8 = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(29) {
            if val.len() > 0 {
              header.leerfeld9 = Some(val.to_string());
            }
          }
          if let Some(val) = record.get(30) {
            if val.len() > 0 {
              header.anwendungsinformation = Some(val.to_string());
            }
          }
          Ok(header)
      } else {
          Err("No header found")
      }
  }
}

#[derive(Clone, PartialEq, Debug, Eq, Serialize, Deserialize)]
pub enum Festschreibung{
  KeineFestschreibung,
  Festschreibung,
}
impl Default for Festschreibung {
  fn default() -> Self {
      Festschreibung::Festschreibung
  }
}
impl Display for Festschreibung {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      write!(f, "{}", match self {
          Festschreibung::KeineFestschreibung => "0",
          Festschreibung::Festschreibung => "1",
      })
  }
}

#[derive(Clone, PartialEq, Debug, Eq, Serialize, Deserialize)]
pub enum Kennzeichen{
  /// externe Daten
  EXTF,
  /// datev Daten
  DTVF,
}

impl Default for Kennzeichen {
  fn default() -> Self {
      Kennzeichen::EXTF
  }
}

impl Display for Kennzeichen {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      write!(f, "{}", match self {
          Kennzeichen::EXTF => "EXTF",
          Kennzeichen::DTVF => "DTVF",
      })
  }
}
