use serde::{Serialize, Deserialize};
use validator::Validate;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::header::Festschreibung;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Buchung {
    /// Umsatz/Betrag für den Datensatz
    /// z.B.: 1234567890,12
    /// Betrag muss immer positiv sein.
    pub umsatz: f64,
    /// Soll-/Haben-Kennzeichnung
    /// bezieht sich auf das Feld #7
    /// Konto
    /// S = SOLL (default)
    /// H = HABEN
    pub soll_haben_kennzeichen: SollHabenKennzeichen,
    /// ISO-Code der Währung
    /// #22 aus Header = default
    pub wkz_umsatz: String,
    /// Wenn Umsatz in Fremdwährung bei #1 angegeben wird
    /// #004, 005 und 006 sind zu übergeben
    /// z.B.: 1234,123456
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kurs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_umsatz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wkz_basis_umsatz: Option<String>,
    pub konto: u32,
    pub gegenkonto: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_schlüssel: Option<u32>,
    /// nur Tag und Monat werden ausgewertet.
    /// Das Jahr wird immer aus
    /// dem Feld 13 des Headers ermittelt
    pub beleg_datum: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belegfeld1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belegfeld2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skonto: Option<f64>,
    #[validate(length(min = 0, max = 60))]
    pub buchungstext: Option<String>,
    #[validate(range(min = 0, max = 1))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postensperre: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diverse_adressnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geschäftspartner_bank: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sachverhalt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zinssperre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_art1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_inhalt1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_art2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_inhalt2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_art3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_inhalt3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_art4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_inhalt4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_art5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_inhalt5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_art6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_inhalt6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_art7: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_inhalt7: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_art8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beleg_info_inhalt8: Option<String>,
    #[validate(length(min = 0, max = 36))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kost1_kostenstelle: Option<String>,
    #[validate(length(min = 0, max = 36))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kost2_kostenstelle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kost_menge: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_ustid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_steuersatz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abweichende_versteuerungsart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sachverhalt_l_l: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funktionsergänzung_l_l: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_49_hauptfunktiontyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_49_hauptfunktionsnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_49_funktionsergänzung: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art1: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt1: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art2: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt2: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art3: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt3: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art4: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt4: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art5: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt5: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art6: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt6: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art7: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt7: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art8: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt8: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art9: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt9: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art10: Option<String>,    
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt10: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art11: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt11: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art12: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt12: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art13: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt13: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art14: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt14: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art15: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt15: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art16: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt16: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art17: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt17: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art18: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt18: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art19: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt19: Option<String>,
    #[validate(length(min = 0, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_art20: Option<String>,
    #[validate(length(min = 0, max = 210))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zusatzinformation_inhalt20: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stück: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gewicht: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zahlweise: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forderungsart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veranlagungsjahr: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zugeordnete_fälligkeit: Option<String>,
    // #[validate(range(min = 1, max = 2))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skontotyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auftragsnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buchungstyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ust_schlüssel_anzahlung: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_mitgliedstaat_anzahlung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sachverhalt_l_l_anzahlung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_steuersatz_anzahlung: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erlöskonto_anzahlung: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub herkunft_kz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leerfeld: Option<String>,
    /// Format TTMMJJJJ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kost_datum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_mandatsreferenz: Option<String>,
    /// Gültige Werte: 0, 1.
    /// 1 = Skontosperre
    /// 0 = Keine Skontosperre
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skontosperre: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gesellschaftername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beteiligtennummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifikationsnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeichennummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postensperre_bis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bezeichnung_so_bil_sachverhalt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kennzeichen_so_bil_buchung: Option<String>,
    /// leer=nichtdefiniert;wird  automatisch festgeschrieben
    /// 0 = keine Festschreibung
    /// 1 = Festschreibung
    /// Hat ein Buchungssatz in diesem Feld den Inhalt 1, so wird der
    /// gesamte Stapel nach dem Import festgeschrieben.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub festschreibung: Option<Festschreibung>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leistungsdatum: Option<String>,
    /// Format TTMMJJJJ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum_zuord_steuerperiode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fälligkeit: Option<String>,
    /// G oder 1 = Generalumkehr
    /// 0 = keine Generalumkehr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generalumkehr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steuersatz: Option<f64>,
    /// Beispiel: DE für Deutschland
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abrechnungsreferenz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvv_position: Option<String>,
    /// Die USt-IdNr. besteht aus
    /// - 2-stelligen Länderkürzel (siehe Dok.-Nr. 1080169)
    /// Ausnahme Griechenland: Das Länderkürzel lautet EL)
    /// - 13-stelliger USt-IdNr.
    /// - Beispiel: DE133546770. Die USt-IdNr kann auch Buchstaben haben, z.B.: bei Österreich
    /// Detaillierte Informationen zur Erfassung von EU-Informationen im Buchungssatz: Dok.-Nr: 9211462. 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_ustid_ursprung: Option<String>,
    /// Nur für entsprechende EU-Buchungen:
    /// Der im EU-Ursprungsland gültige Steuersatz.
    /// Beispiel: 12,12
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_steuersatz_ursprung: Option<f64>,
}

impl Default for Buchung {
    fn default() -> Self {
        Buchung {
            umsatz: 0.0,
            soll_haben_kennzeichen: SollHabenKennzeichen::default(),
            wkz_umsatz: "EUR".to_string(),
            kurs: None,
            basis_umsatz: None,
            wkz_basis_umsatz: None,
            konto: 0,
            gegenkonto: 0,
            bu_schlüssel: None,
            beleg_datum: chrono::Local::today().naive_local(),
            belegfeld1: None,
            belegfeld2: None,
            skonto: None,
            buchungstext: None,
            postensperre: None,
            diverse_adressnummer: None,
            geschäftspartner_bank: None,
            sachverhalt: None,
            zinssperre: None,
            beleg_link: None,
            beleg_info_art1: None,
            beleg_info_inhalt1: None,
            beleg_info_art2: None,
            beleg_info_inhalt2: None,
            beleg_info_art3: None,
            beleg_info_inhalt3: None,
            beleg_info_art4: None,
            beleg_info_inhalt4: None,
            beleg_info_art5: None,
            beleg_info_inhalt5: None,
            beleg_info_art6: None,
            beleg_info_inhalt6: None,
            beleg_info_art7: None,
            beleg_info_inhalt7: None,
            beleg_info_art8: None,
            beleg_info_inhalt8: None,
            kost1_kostenstelle: None,
            kost2_kostenstelle: None,
            kost_menge: None,
            eu_ustid: None,
            eu_steuersatz: None,
            abweichende_versteuerungsart: None,
            sachverhalt_l_l: None,
            funktionsergänzung_l_l: None,
            bu_49_hauptfunktiontyp: None,
            bu_49_hauptfunktionsnummer: None,
            bu_49_funktionsergänzung: None,
            zusatzinformation_art1: None,
            zusatzinformation_inhalt1: None,
            zusatzinformation_art2: None,
            zusatzinformation_inhalt2: None,
            zusatzinformation_art3: None,
            zusatzinformation_inhalt3: None,
            zusatzinformation_art4: None,
            zusatzinformation_inhalt4: None,
            zusatzinformation_art5: None,
            zusatzinformation_inhalt5: None,
            zusatzinformation_art6: None,
            zusatzinformation_inhalt6: None,
            zusatzinformation_art7: None,
            zusatzinformation_inhalt7: None,
            zusatzinformation_art8: None,
            zusatzinformation_inhalt8: None,
            zusatzinformation_art9: None,
            zusatzinformation_inhalt9: None,
            zusatzinformation_art10: None,    
            zusatzinformation_inhalt10: None,
            zusatzinformation_art11: None,
            zusatzinformation_inhalt11: None,
            zusatzinformation_art12: None,
            zusatzinformation_inhalt12: None,
            zusatzinformation_art13: None,
            zusatzinformation_inhalt13: None,
            zusatzinformation_art14: None,
            zusatzinformation_inhalt14: None,
            zusatzinformation_art15: None,
            zusatzinformation_inhalt15: None,
            zusatzinformation_art16: None,
            zusatzinformation_inhalt16: None,
            zusatzinformation_art17: None,
            zusatzinformation_inhalt17: None,
            zusatzinformation_art18: None,
            zusatzinformation_inhalt18: None,
            zusatzinformation_art19: None,
            zusatzinformation_inhalt19: None,
            zusatzinformation_art20: None,
            zusatzinformation_inhalt20: None,
            stück: None,
            gewicht: None,
            zahlweise: None,
            forderungsart: None,
            veranlagungsjahr: None,
            zugeordnete_fälligkeit: None,
            skontotyp: None,
            auftragsnummer: None,
            buchungstyp: None,
            ust_schlüssel_anzahlung: None,
            eu_mitgliedstaat_anzahlung: None,
            sachverhalt_l_l_anzahlung: None,
            eu_steuersatz_anzahlung: None,
            erlöskonto_anzahlung: None,
            herkunft_kz: None,
            leerfeld: None,
            kost_datum: None,
            sepa_mandatsreferenz: None,
            skontosperre: None,
            gesellschaftername: None,
            beteiligtennummer: None,
            identifikationsnummer: None,
            zeichennummer: None,
            postensperre_bis: None,
            bezeichnung_so_bil_sachverhalt: None,
            kennzeichen_so_bil_buchung: None,
            festschreibung: None,
            leistungsdatum: None,
            datum_zuord_steuerperiode: None,
            fälligkeit: None,
            generalumkehr: None,
            steuersatz: None,
            land: None,
            abrechnungsreferenz: None,
            bvv_position: None,
            eu_ustid_ursprung: None,
            eu_steuersatz_ursprung: None,
        }
    }
}

impl Display for Buchung{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{umsatz};{soll_haben_kennzeichen};{wkz_umsatz};{kurs};{basis_umsatz};{wkz_basis_umsatz};{konto};{gegenkonto};{bu_schlüssel};{beleg_datum};{belegfeld1};{belegfeld2};{skonto};{buchungstext};{posten_sperre};{diverse_adressnummer};{geschäftspartner_bank};{sachverhalt};{zinssperre};{beleg_link};{beleg_info_art1};{beleg_info_inhalt1};{beleg_info_art2};{beleg_info_inhalt2};{beleg_info_art3};{beleg_info_inhalt3};{beleg_info_art4};{beleg_info_inhalt4};{beleg_info_art5};{beleg_info_inhalt5};{beleg_info_art6};{beleg_info_inhalt6};{beleg_info_art7};{beleg_info_inhalt7};{beleg_info_art8};{beleg_info_inhalt8};{kost1_kostenstelle};{kost2_kostenstelle};{kost_menge};{eu_ustid};{eu_steuersatz};{abweichende_versteuerungsart};{sachverhalt_l_l};{funktionsergänzung_l_l};{bu_49_hauptfunktiontyp};{bu_49_hauptfunktionsnummer};{bu_49_funktionsergänzung};{zusatzinformation_art1};{zusatzinformation_inhalt1};{zusatzinformation_art2};{zusatzinformation_inhalt2};{zusatzinformation_art3};{zusatzinformation_inhalt3};{zusatzinformation_art4};{zusatzinformation_inhalt4};{zusatzinformation_art5};{zusatzinformation_inhalt5};{zusatzinformation_art6};{zusatzinformation_inhalt6};{zusatzinformation_art7};{zusatzinformation_inhalt7};{zusatzinformation_art8};{zusatzinformation_inhalt8};{zusatzinformation_art9};{zusatzinformation_inhalt9};{zusatzinformation_art10};{zusatzinformation_inhalt10};{zusatzinformation_art11};{zusatzinformation_inhalt11};{zusatzinformation_art12};{zusatzinformation_inhalt12};{zusatzinformation_art13};{zusatzinformation_inhalt13};{zusatzinformation_art14};{zusatzinformation_inhalt14};{zusatzinformation_art15};{zusatzinformation_inhalt15};{zusatzinformation_art16};{zusatzinformation_inhalt16};{zusatzinformation_art17};{zusatzinformation_inhalt17};{zusatzinformation_art18};{zusatzinformation_inhalt18};{zusatzinformation_art19};{zusatzinformation_inhalt19};{zusatzinformation_art20};{zusatzinformation_inhalt20};{stück};{gewicht};{zahlweise};{forderungsart};{veranlagungsjahr};{zugeordnete_fälligkeit};{skontotyp};{auftragsnummer};{buchungstyp};{ust_schlüssel_anzahlung};{eu_mitgliedstaat_anzahlung};{sachverhalt_l_l_anzahlung};{eu_steuersatz_anzahlung};{erlöskonto_anzahlung};{herkunft_kz};{leerfeld};{kost_datum};{sepa_mandatsreferenz};{skontosperre};{gesellschaftername};{beteiligtennummer};{identifikationsnummer};{zeichennummer};{postensperre_bis};{bezeichnung_so_bil_sachverhalt};{kennzeichen_so_bil_buchung};{festschreibung};{leistungsdatum};{datum_zuord_steuerperiode};{fälligkeit};{generalumkehr};{steuersatz};{land};{abrechnungsreferenz};{bvv_position};{eu_ustid_ursprung};{eu_steuersatz_ursprung}{newline}"#,
        umsatz = format!("{:.2}", self.umsatz).replace(".", ","),
        soll_haben_kennzeichen = self.soll_haben_kennzeichen,
        wkz_umsatz = self.wkz_umsatz,
        kurs = match self.kurs {
            Some(val) => val.to_string().replace(".", ","),
            None => "".to_string(),
        },
        basis_umsatz = match self.basis_umsatz {
            Some(val) => format!("{:.2}", val).replace(".", ","),
            None => "".to_string(),
        },
        wkz_basis_umsatz = match &self.wkz_basis_umsatz{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        konto = self.konto,
        gegenkonto = self.gegenkonto,
        bu_schlüssel = match self.bu_schlüssel {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_datum = self.beleg_datum.format("%d%m").to_string(),
        belegfeld1 = match &self.belegfeld1 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        belegfeld2 = match &self.belegfeld2 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        skonto = match self.skonto {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        buchungstext = match &self.buchungstext {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        posten_sperre = match self.postensperre {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        diverse_adressnummer = match &self.diverse_adressnummer{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        geschäftspartner_bank = match &self.geschäftspartner_bank{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        sachverhalt = match &self.sachverhalt{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zinssperre = match &self.zinssperre{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_link = match &self.beleg_link{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_art1 = match &self.beleg_info_art1{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_inhalt1 = match &self.beleg_info_inhalt1{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_art2 = match &self.beleg_info_art2{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_inhalt2 = match &self.beleg_info_inhalt2{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_art3 = match &self.beleg_info_art3{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_inhalt3 = match &self.beleg_info_inhalt3{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_art4 = match &self.beleg_info_art4{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_inhalt4 = match &self.beleg_info_inhalt4{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_art5 = match &self.beleg_info_art5{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_inhalt5 = match &self.beleg_info_inhalt5{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_art6 = match &self.beleg_info_art6{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_inhalt6 = match &self.beleg_info_inhalt6{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_art7 = match &self.beleg_info_art7{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_inhalt7 = match &self.beleg_info_inhalt7{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_art8 = match &self.beleg_info_art8{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_info_inhalt8 = match &self.beleg_info_inhalt8{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kost1_kostenstelle = match &self.kost1_kostenstelle {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kost2_kostenstelle = match &self.kost2_kostenstelle {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kost_menge = match self.kost_menge {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_ustid = match &self.eu_ustid {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_steuersatz = match self.eu_steuersatz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        abweichende_versteuerungsart = match &self.abweichende_versteuerungsart{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        sachverhalt_l_l = match &self.sachverhalt_l_l{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        funktionsergänzung_l_l = match &self.funktionsergänzung_l_l{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        bu_49_hauptfunktiontyp = match &self.bu_49_hauptfunktiontyp{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        bu_49_hauptfunktionsnummer = match &self.bu_49_hauptfunktionsnummer{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        bu_49_funktionsergänzung = match &self.bu_49_funktionsergänzung{
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art1 = match &self.zusatzinformation_art1 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt1 = match &self.zusatzinformation_inhalt1 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art2 = match &self.zusatzinformation_art2 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt2 = match &self.zusatzinformation_inhalt2 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art3 = match &self.zusatzinformation_art3 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt3 = match &self.zusatzinformation_inhalt3 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art4 = match &self.zusatzinformation_art4 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt4 = match &self.zusatzinformation_inhalt4 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art5 = match &self.zusatzinformation_art5 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt5 = match &self.zusatzinformation_inhalt5 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art6 = match &self.zusatzinformation_art6 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt6 = match &self.zusatzinformation_inhalt6 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art7 = match &self.zusatzinformation_art7 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt7 = match &self.zusatzinformation_inhalt7 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art8 = match &self.zusatzinformation_art8 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt8 = match &self.zusatzinformation_inhalt8 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art9 = match &self.zusatzinformation_art9 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt9 = match &self.zusatzinformation_inhalt9 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art10 = match &self.zusatzinformation_art10 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt10 = match &self.zusatzinformation_inhalt10 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art11 = match &self.zusatzinformation_art11 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt11 = match &self.zusatzinformation_inhalt11 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art12 = match &self.zusatzinformation_art12 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt12 = match &self.zusatzinformation_inhalt12 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art13 = match &self.zusatzinformation_art13 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt13 = match &self.zusatzinformation_inhalt13 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art14 = match &self.zusatzinformation_art14 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt14 = match &self.zusatzinformation_inhalt14 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art15 = match &self.zusatzinformation_art15 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt15 = match &self.zusatzinformation_inhalt15 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art16 = match &self.zusatzinformation_art16 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt16 = match &self.zusatzinformation_inhalt16 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art17 = match &self.zusatzinformation_art17 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt17 = match &self.zusatzinformation_inhalt17 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art18 = match &self.zusatzinformation_art18 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt18 = match &self.zusatzinformation_inhalt18 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art19 = match &self.zusatzinformation_art19 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt19 = match &self.zusatzinformation_inhalt19 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art20 = match &self.zusatzinformation_art20 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt20 = match &self.zusatzinformation_inhalt20 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        stück = match self.stück {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        gewicht = match self.gewicht {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zahlweise = match &self.zahlweise {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        forderungsart = match &self.forderungsart {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        veranlagungsjahr = match self.veranlagungsjahr {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zugeordnete_fälligkeit = match &self.zugeordnete_fälligkeit {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        skontotyp = match &self.skontotyp {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        auftragsnummer = match &self.auftragsnummer {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        buchungstyp = match &self.buchungstyp {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        ust_schlüssel_anzahlung = match self.ust_schlüssel_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_mitgliedstaat_anzahlung = match &self.eu_mitgliedstaat_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        sachverhalt_l_l_anzahlung = match &self.sachverhalt_l_l_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_steuersatz_anzahlung = match self.eu_steuersatz_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        erlöskonto_anzahlung = match self.erlöskonto_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        herkunft_kz = match &self.herkunft_kz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        leerfeld = match &self.leerfeld {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kost_datum = match &self.kost_datum {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        sepa_mandatsreferenz = match &self.sepa_mandatsreferenz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        skontosperre = match self.skontosperre {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        gesellschaftername = match &self.gesellschaftername {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beteiligtennummer = match &self.beteiligtennummer {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        identifikationsnummer = match &self.identifikationsnummer {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zeichennummer = match &self.zeichennummer {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        postensperre_bis = match &self.postensperre_bis {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        bezeichnung_so_bil_sachverhalt = match &self.bezeichnung_so_bil_sachverhalt {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kennzeichen_so_bil_buchung = match &self.kennzeichen_so_bil_buchung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        festschreibung = match &self.festschreibung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        leistungsdatum = match &self.leistungsdatum {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        datum_zuord_steuerperiode = match &self.datum_zuord_steuerperiode {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        fälligkeit = match &self.fälligkeit {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        generalumkehr = match &self.generalumkehr {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        steuersatz = match self.steuersatz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        land = match &self.land {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        abrechnungsreferenz = match &self.abrechnungsreferenz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        bvv_position = match &self.bvv_position {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_ustid_ursprung = match &self.eu_ustid_ursprung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_steuersatz_ursprung = match self.eu_steuersatz_ursprung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        newline = "\n",
        )      
    }
}

impl TryFrom<&str> for Buchung {
    type Error = &'static str;
  
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut rdr = csv::ReaderBuilder::new().delimiter(b';').flexible(true)
            .has_headers(false).from_reader(value.as_bytes());

        let mut iter = rdr.records();
        if let Some(result) = iter.next() {
            let record = result.unwrap();
            let mut buchung = Buchung::default();
            //add values
            if let Some(val) = record.get(0) {
                let fixed_decimal = val.replace(',', ".");
                buchung.umsatz = fixed_decimal.parse().unwrap();
            }
            if let Some(val) = record.get(1) {
                buchung.soll_haben_kennzeichen = match val{
                    "H" => SollHabenKennzeichen::Haben,
                    "S" => SollHabenKennzeichen::Soll,
                    _ => SollHabenKennzeichen::default(),
                };
            }
            if let Some(val) = record.get(2) {
                buchung.wkz_umsatz = val.to_string();
            }
            if let Some(val) = record.get(3) {
                if !val.is_empty() {
                    let fixed_decimal = val.replace(',', ".");
                    buchung.kurs = Some(fixed_decimal.parse().unwrap());
                }
            }
            if let Some(val) = record.get(4) {
                if !val.is_empty() {
                    let fixed_decimal = val.replace(',', ".");
                    buchung.basis_umsatz = Some(fixed_decimal.parse().unwrap());
                }
            }
            if let Some(val) = record.get(5) {
                buchung.wkz_basis_umsatz = Some(val.to_string());
            }
            if let Some(val) = record.get(6) {
                buchung.konto = val.parse().unwrap();
            }
            if let Some(val) = record.get(7) {
                buchung.gegenkonto = val.parse().unwrap();
            }
            if let Some(val) = record.get(8) {
                if !val.is_empty() {
                    buchung.bu_schlüssel = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(9) {
                //TODO find correct year
                let x = format!("{}-2021", val);
                buchung.beleg_datum = chrono::NaiveDate::parse_from_str(&x, "%d%m-%Y").unwrap();
            }
            if let Some(val) = record.get(10) {
                buchung.belegfeld1 = Some(val.to_string());
            }
            if let Some(val) = record.get(11) {
                buchung.belegfeld2 = Some(val.to_string());
            }
            if let Some(val) = record.get(12) {
                if !val.is_empty() {
                    let fixed_decimal = val.replace(',', ".");
                    buchung.skonto = Some(fixed_decimal.parse().unwrap());
                }
            }
            if let Some(val) = record.get(13) {
                buchung.buchungstext = Some(val.to_string());
            }
            if let Some(val) = record.get(14) {
                if !val.is_empty() {
                    buchung.postensperre = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(15) {
                buchung.diverse_adressnummer = Some(val.to_string());
            }
            if let Some(val) = record.get(16) {
                buchung.geschäftspartner_bank = Some(val.to_string());
            }
            if let Some(val) = record.get(17) {
                buchung.sachverhalt = Some(val.to_string());
            }
            if let Some(val) = record.get(18) {
                buchung.zinssperre = Some(val.to_string());
            }
            if let Some(val) = record.get(19) {
                buchung.beleg_link = Some(val.to_string());
            }
            if let Some(val) = record.get(20) {
                buchung.beleg_info_art1 = Some(val.to_string());
            }
            if let Some(val) = record.get(21) {
                buchung.beleg_info_inhalt1 = Some(val.to_string());
            }
            if let Some(val) = record.get(22) {
                buchung.beleg_info_art2 = Some(val.to_string());
            }
            if let Some(val) = record.get(23) {
                buchung.beleg_info_inhalt2 = Some(val.to_string());
            }
            if let Some(val) = record.get(24) {
                buchung.beleg_info_art3 = Some(val.to_string());
            }
            if let Some(val) = record.get(25) {
                buchung.beleg_info_inhalt3 = Some(val.to_string());
            }
            if let Some(val) = record.get(26) {
                buchung.beleg_info_art4 = Some(val.to_string());
            }
            if let Some(val) = record.get(27) {
                buchung.beleg_info_inhalt4 = Some(val.to_string());
            }
            if let Some(val) = record.get(28) {
                buchung.beleg_info_art5 = Some(val.to_string());
            }
            if let Some(val) = record.get(29) {
                buchung.beleg_info_inhalt5 = Some(val.to_string());
            }
            if let Some(val) = record.get(30) {
                buchung.beleg_info_art6 = Some(val.to_string());
            }
            if let Some(val) = record.get(31) {
                buchung.beleg_info_inhalt6 = Some(val.to_string());
            }
            if let Some(val) = record.get(32) {
                buchung.beleg_info_art7 = Some(val.to_string());
            }
            if let Some(val) = record.get(33) {
                buchung.beleg_info_inhalt7 = Some(val.to_string());
            }
            if let Some(val) = record.get(34) {
                buchung.beleg_info_art8 = Some(val.to_string());
            }
            if let Some(val) = record.get(35) {
                buchung.beleg_info_inhalt8 = Some(val.to_string());
            }
            if let Some(val) = record.get(36) {
                buchung.kost1_kostenstelle = Some(val.to_string());
            }
            if let Some(val) = record.get(37) {
                buchung.kost2_kostenstelle = Some(val.to_string());
            }
            if let Some(val) = record.get(38) {
                if !val.is_empty() {
                    buchung.kost_menge = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(39) {
                buchung.eu_ustid = Some(val.to_string());
            }
            if let Some(val) = record.get(40) {
                if !val.is_empty() {
                    buchung.eu_steuersatz = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(41) {
                buchung.abweichende_versteuerungsart = Some(val.to_string());
            }
            if let Some(val) = record.get(42) {
                buchung.sachverhalt_l_l = Some(val.to_string());
            }
            if let Some(val) = record.get(43) {
                buchung.funktionsergänzung_l_l = Some(val.to_string());
            }
            if let Some(val) = record.get(44) {
                buchung.bu_49_hauptfunktiontyp = Some(val.to_string());
            }
            if let Some(val) = record.get(45) {
                buchung.bu_49_hauptfunktionsnummer = Some(val.to_string());
            }
            if let Some(val) = record.get(46) {
                buchung.bu_49_funktionsergänzung = Some(val.to_string());
            }
            if let Some(val) = record.get(47) {
                buchung.zusatzinformation_art1 = Some(val.to_string());
            }
            if let Some(val) = record.get(48) {
                buchung.zusatzinformation_inhalt1 = Some(val.to_string());
            }
            if let Some(val) = record.get(49) {
                buchung.zusatzinformation_art2 = Some(val.to_string());
            }
            if let Some(val) = record.get(50) {
                buchung.zusatzinformation_inhalt2 = Some(val.to_string());
            }
            if let Some(val) = record.get(51) {
                buchung.zusatzinformation_art3 = Some(val.to_string());
            }
            if let Some(val) = record.get(52) {
                buchung.zusatzinformation_inhalt3 = Some(val.to_string());
            }
            if let Some(val) = record.get(53) {
                buchung.zusatzinformation_art4 = Some(val.to_string());
            }
            if let Some(val) = record.get(54) {
                buchung.zusatzinformation_inhalt4 = Some(val.to_string());
            }
            if let Some(val) = record.get(55) {
                buchung.zusatzinformation_art5 = Some(val.to_string());
            }
            if let Some(val) = record.get(56) {
                buchung.zusatzinformation_inhalt5 = Some(val.to_string());
            }
            if let Some(val) = record.get(57) {
                buchung.zusatzinformation_art6 = Some(val.to_string());
            }
            if let Some(val) = record.get(58) {
                buchung.zusatzinformation_inhalt6 = Some(val.to_string());
            }
            if let Some(val) = record.get(59) {
                buchung.zusatzinformation_art7 = Some(val.to_string());
            }
            if let Some(val) = record.get(60) {
                buchung.zusatzinformation_inhalt7 = Some(val.to_string());
            }
            if let Some(val) = record.get(61) {
                buchung.zusatzinformation_art8 = Some(val.to_string());
            }
            if let Some(val) = record.get(62) {
                buchung.zusatzinformation_inhalt8 = Some(val.to_string());
            }
            if let Some(val) = record.get(63) {
                buchung.zusatzinformation_art9 = Some(val.to_string());
            }
            if let Some(val) = record.get(64) {
                buchung.zusatzinformation_inhalt9 = Some(val.to_string());
            }
            if let Some(val) = record.get(65) {
                buchung.zusatzinformation_art10 = Some(val.to_string());
            }
            if let Some(val) = record.get(66) {
                buchung.zusatzinformation_inhalt10 = Some(val.to_string());
            }
            if let Some(val) = record.get(67) {
                buchung.zusatzinformation_art11 = Some(val.to_string());
            }
            if let Some(val) = record.get(68) {
                buchung.zusatzinformation_inhalt11 = Some(val.to_string());
            }
            if let Some(val) = record.get(69) {
                buchung.zusatzinformation_art12 = Some(val.to_string());
            }
            if let Some(val) = record.get(70) {
                buchung.zusatzinformation_inhalt12 = Some(val.to_string());
            }
            if let Some(val) = record.get(71) {
                buchung.zusatzinformation_art13 = Some(val.to_string());
            }
            if let Some(val) = record.get(72) {
                buchung.zusatzinformation_inhalt13 = Some(val.to_string());
            }
            if let Some(val) = record.get(73) {
                buchung.zusatzinformation_art14 = Some(val.to_string());
            }
            if let Some(val) = record.get(74) {
                buchung.zusatzinformation_inhalt14 = Some(val.to_string());
            }
            if let Some(val) = record.get(75) {
                buchung.zusatzinformation_art15 = Some(val.to_string());
            }
            if let Some(val) = record.get(76) {
                buchung.zusatzinformation_inhalt15 = Some(val.to_string());
            }
            if let Some(val) = record.get(77) {
                buchung.zusatzinformation_art16 = Some(val.to_string());
            }
            if let Some(val) = record.get(78) {
                buchung.zusatzinformation_inhalt16 = Some(val.to_string());
            }
            if let Some(val) = record.get(79) {
                buchung.zusatzinformation_art17 = Some(val.to_string());
            }
            if let Some(val) = record.get(80) {
                buchung.zusatzinformation_inhalt17 = Some(val.to_string());
            }
            if let Some(val) = record.get(81) {
                buchung.zusatzinformation_art18 = Some(val.to_string());
            }
            if let Some(val) = record.get(82) {
                buchung.zusatzinformation_inhalt18 = Some(val.to_string());
            }
            if let Some(val) = record.get(83) {
                buchung.zusatzinformation_art19 = Some(val.to_string());
            }
            if let Some(val) = record.get(84) {
                buchung.zusatzinformation_inhalt19 = Some(val.to_string());
            }
            if let Some(val) = record.get(85) {
                buchung.zusatzinformation_art20 = Some(val.to_string());
            }
            if let Some(val) = record.get(86) {
                buchung.zusatzinformation_inhalt20 = Some(val.to_string());
            }
            if let Some(val) = record.get(87) {
                if !val.is_empty() {
                    buchung.stück = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(88) {
                if !val.is_empty() {
                    buchung.gewicht = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(89) {
                buchung.zahlweise = Some(val.to_string());
            }
            if let Some(val) = record.get(90) {
                buchung.forderungsart = Some(val.to_string());
            }
            if let Some(val) = record.get(91) {
                buchung.veranlagungsjahr = val.parse().ok();
            }
            if let Some(val) = record.get(92) {
                buchung.zugeordnete_fälligkeit = Some(val.to_string());
            }
            if let Some(val) = record.get(93) {
                buchung.skontotyp = Some(val.to_string());
            }
            if let Some(val) = record.get(94) {
                buchung.auftragsnummer = Some(val.to_string());
            }
            if let Some(val) = record.get(95) {
                buchung.buchungstyp = Some(val.to_string());
            }
            if let Some(val) = record.get(96) {
                if !val.is_empty() {
                    buchung.ust_schlüssel_anzahlung = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(97) {
                buchung.eu_mitgliedstaat_anzahlung = Some(val.to_string());
            }
            if let Some(val) = record.get(98) {
                buchung.sachverhalt_l_l_anzahlung = Some(val.to_string());
            }
            if let Some(val) = record.get(99) {
                if !val.is_empty() {
                    buchung.eu_steuersatz_anzahlung = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(100) {
                if !val.is_empty() {
                    buchung.erlöskonto_anzahlung = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(101) {
                buchung.herkunft_kz = Some(val.to_string());
            }
            if let Some(val) = record.get(102) {
                buchung.leerfeld = Some(val.to_string());
            }
            if let Some(val) = record.get(103) {
                buchung.kost_datum = Some(val.to_string());
            }
            if let Some(val) = record.get(104) {
                buchung.sepa_mandatsreferenz = Some(val.to_string());
            }
            if let Some(val) = record.get(105) {
                if !val.is_empty() {
                    buchung.skontosperre = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(106) {
                buchung.gesellschaftername = Some(val.to_string());
            }
            if let Some(val) = record.get(107) {
                buchung.beteiligtennummer = Some(val.to_string());
            }
            if let Some(val) = record.get(108) {
                buchung.identifikationsnummer = Some(val.to_string());
            }
            if let Some(val) = record.get(109) {
                buchung.zeichennummer = Some(val.to_string());
            }
            if let Some(val) = record.get(110) {
                buchung.postensperre_bis = Some(val.to_string());
            }
            if let Some(val) = record.get(111) {
                buchung.bezeichnung_so_bil_sachverhalt = Some(val.to_string());
            }
            if let Some(val) = record.get(112) {
                buchung.kennzeichen_so_bil_buchung = Some(val.to_string());
            }
            if let Some(val) = record.get(113) {
                buchung.festschreibung = match val {
                    "0" => Some(Festschreibung::KeineFestschreibung),
                    "1" => Some(Festschreibung::Festschreibung),
                    _ => None,
                };
            }
            if let Some(val) = record.get(114) {
                buchung.leistungsdatum = Some(val.to_string());
            }
            if let Some(val) = record.get(115) {
                buchung.datum_zuord_steuerperiode = Some(val.to_string());
            }
            if let Some(val) = record.get(116) {
                buchung.fälligkeit = Some(val.to_string());
            }
            if let Some(val) = record.get(117) {
                buchung.generalumkehr = Some(val.to_string());
            }
            if let Some(val) = record.get(118) {
                if !val.is_empty() {
                    buchung.steuersatz = Some(val.parse().unwrap());
                }
            }
            if let Some(val) = record.get(119) {
                buchung.land = Some(val.to_string());
            }
            if let Some(val) = record.get(120) {
                buchung.abrechnungsreferenz = Some(val.to_string());
            }
            if let Some(val) = record.get(121) {
                buchung.bvv_position = Some(val.to_string());
            }
            if let Some(val) = record.get(122) {
                buchung.eu_ustid_ursprung = Some(val.to_string());
            }
            if let Some(val) = record.get(123) {
                if !val.is_empty() {
                    buchung.eu_steuersatz_ursprung = Some(val.parse().unwrap());
                }
            }
            Ok(buchung)
        }else{
            Err("content not recognised")
        }   
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum SollHabenKennzeichen{
  Soll,
  Haben,
}
impl Default for SollHabenKennzeichen {
  fn default() -> Self {
      SollHabenKennzeichen::Soll
  }
}
impl Display for SollHabenKennzeichen {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      write!(f, "{}", match self {
          SollHabenKennzeichen::Soll => "S",
          SollHabenKennzeichen::Haben => "H",
      })
  }
}
