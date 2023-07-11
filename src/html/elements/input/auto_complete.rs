use std::borrow::Cow;
use std::fmt;
use std::marker::PhantomData;

#[derive(Hash)]
pub struct AutoComplete {
    a: Option<Cow<'static, str>>,
    b: Option<&'static str>,
    c: Option<&'static str>,
    d: Option<&'static str>,
}

impl AutoComplete {
    pub fn builder() -> AutoCompleteBuilder<Section> {
        AutoCompleteBuilder {
            a: None,
            b: None,
            c: None,
            d: None,
            marker: PhantomData,
        }
    }
}

pub struct AutoCompleteBuilder<Step> {
    a: Option<Cow<'static, str>>,
    b: Option<&'static str>,
    c: Option<&'static str>,
    d: Option<&'static str>,
    marker: PhantomData<Step>,
}

pub struct Section(());
impl AutoCompleteBuilder<Section> {
    /// Enable auto-complete, but rely on the user-agent to determine what to auto-complete.
    pub fn on(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("on"),
        }
    }

    /// Disable auto-complete.
    pub fn off(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("off"),
        }
    }

    /// The user agent should show public key credentials available via conditional mediation when
    /// the user interacts with the form control.
    pub fn webauthn(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("webauthn"),
        }
    }

    /// The field belongs to the `name`d group.
    pub fn section(self, name: impl Into<Cow<'static, str>>) -> AutoCompleteBuilder<AddrKind> {
        AutoCompleteBuilder {
            a: Some(name.into()),
            b: self.b,
            c: self.c,
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is part of the shipping address or contact information.
    pub fn shipping(self) -> AutoCompleteBuilder<TelKind> {
        AutoCompleteBuilder {
            a: self.a,
            b: Some("shipping"),
            c: self.c,
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is part of the billing address or contact information.
    pub fn billing(self) -> AutoCompleteBuilder<TelKind> {
        AutoCompleteBuilder {
            a: self.a,
            b: Some("billing"),
            c: self.c,
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is for contacting someone at their residence.
    pub fn home(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("home"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is for contacting someone at their workplace.
    pub fn work(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("work"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is for contacting someone regardless of location.
    pub fn mobile(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("mobile"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field describes a fax machine's contact details.
    pub fn fax(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("fax"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field describes a pager's or beeper's contact details.
    pub fn page(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("page"),
            d: self.d,
            marker: PhantomData,
        }
    }
}

pub struct AddrKind(());
impl AutoCompleteBuilder<AddrKind> {
    /// The field is part of the shipping address or contact information.
    pub fn shipping(self) -> AutoCompleteBuilder<TelKind> {
        AutoCompleteBuilder {
            a: self.a,
            b: Some("shipping"),
            c: self.c,
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is part of the billing address or contact information.
    pub fn billing(self) -> AutoCompleteBuilder<TelKind> {
        AutoCompleteBuilder {
            a: self.a,
            b: Some("billing"),
            c: self.c,
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is for contacting someone at their residence.
    pub fn home(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("home"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is for contacting someone at their workplace.
    pub fn work(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("work"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is for contacting someone regardless of location.
    pub fn mobile(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("mobile"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field describes a fax machine's contact details.
    pub fn fax(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("fax"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field describes a pager's or beeper's contact details.
    pub fn page(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("page"),
            d: self.d,
            marker: PhantomData,
        }
    }
}

pub struct TelKind(());
impl AutoCompleteBuilder<TelKind> {
    /// The field is for contacting someone at their residence.
    pub fn home(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("home"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is for contacting someone at their workplace.
    pub fn work(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("work"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field is for contacting someone regardless of location.
    pub fn mobile(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("mobile"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field describes a fax machine's contact details.
    pub fn fax(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("fax"),
            d: self.d,
            marker: PhantomData,
        }
    }

    /// The field describes a pager's or beeper's contact details.
    pub fn page(self) -> AutoCompleteBuilder<()> {
        AutoCompleteBuilder {
            a: self.a,
            b: self.b,
            c: Some("page"),
            d: self.d,
            marker: PhantomData,
        }
    }
}

impl<Step> AutoCompleteBuilder<Step> {
    /// Full name.
    pub fn name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("name"),
        }
    }

    /// Prefix or title (e.g. "Mr.", "Ms.", "Dr.", "Mlle").
    pub fn honorific_prefix(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("honorific-prefix"),
        }
    }

    /// Given name (in some Western cultures, also known as the first name).
    pub fn given_name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("given-name"),
        }
    }

    /// Additional names (in some Western cultures, also known as middle names, forenames other
    /// than the first name).
    pub fn additional_name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("additional-name"),
        }
    }

    /// Family name (in some Western cultures, also known as the last name or surname).
    pub fn family_name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("family-name"),
        }
    }

    /// Suffix (e.g. "Jr.", "B.Sc.", "MBASW", "II").
    pub fn honorific_suffix(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("honorific-suffix"),
        }
    }

    /// Nickname, screen name, handle: a typically short name used instead of the full name.
    pub fn nickname(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("nickname"),
        }
    }

    /// A username.
    pub fn username(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("username"),
        }
    }

    /// A new password (e.g. when creating an account or changing a password).
    pub fn new_password(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("new-password"),
        }
    }

    /// The current password for the account identified by the `username` field.
    pub fn current_password(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("current-password"),
        }
    }

    /// One-time code used for verifying user identity.
    pub fn one_time_code(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("one-time-code"),
        }
    }

    /// Job title (e.g. "Software Engineer", "Senior Vice President", "Deputy Managing Director").
    pub fn organization_title(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("organization-title"),
        }
    }

    /// Company name corresponding to the person, address, or contact information in the other
    /// fields associated with this field.
    pub fn organization(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("organization"),
        }
    }

    /// Street address (multiple lines, newlines preserved).
    pub fn street_address(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("street-address"),
        }
    }

    /// Street address line 1.
    pub fn address_line1(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("address-line1"),
        }
    }

    /// Street address line 2.
    pub fn address_line2(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("address-line2"),
        }
    }

    /// Street address line 3.
    pub fn address_line3(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("address-line3"),
        }
    }

    /// The most fine-grained administrative level, in addresses with four administrative levels.
    pub fn address_level4(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("address-level4"),
        }
    }

    /// The third administrative level, in addresses with three or more administrative levels.
    pub fn address_level3(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("address-level3"),
        }
    }

    /// The second administrative level, in addresses with two or more administrative levels; in
    /// the countries with two administrative levels, this would typically be the city, town,
    /// village, or other locality within which the relevant street address is found.
    pub fn address_level2(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("address-level2"),
        }
    }

    /// The broadest administrative level in the address, i.e. the province within which the
    /// locality is found; for example, in the US, this would be the state; in Switzerland it
    /// would be the canton; in the UK, the post town.
    pub fn address_level1(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("address-level1"),
        }
    }

    /// Country code.
    pub fn country(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("country"),
        }
    }

    /// Country name.
    pub fn country_name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("country-name"),
        }
    }

    /// Postal code, post code, ZIP code, CEDEX code (if CEDEX, append "CEDEX", and the
    /// arrondissement, if relevant, to the `address-level2` field).
    pub fn postal_code(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("postal-code"),
        }
    }

    /// Full name as given on the payment instrument.
    pub fn cc_name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-name"),
        }
    }

    /// Given name as given on the payment instrument (in some Western cultures, also known as the
    /// first name).
    pub fn cc_given_name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-given-name"),
        }
    }

    /// Additional names given on the payment instrument (in some Western cultures, also known as
    /// middle names, forenames other than the first name).
    pub fn cc_additional_name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-additional-name"),
        }
    }

    /// Family name given on the payment instrument (in some Western cultures, also known as the last
    /// name or surname).
    pub fn cc_family_name(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-family-name"),
        }
    }

    /// Code identifying the payment instrument (e.g. the credit card number).
    pub fn cc_number(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-number"),
        }
    }

    /// Expiration date of the payment instrument.
    pub fn cc_exp(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-exp"),
        }
    }

    /// Month component of the expiration date of the payment instrument.
    pub fn cc_exp_month(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-exp-month"),
        }
    }

    /// Year component of the expiration date of the payment instrument.
    pub fn cc_exp_year(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-exp-year"),
        }
    }

    /// Security code for the payment instrument (also known as the card security code (CSC), card
    /// validation code (CVC), card verification value (CVV), signature panel code (SPC), credit
    /// card ID (CCID), etc.).
    pub fn cc_csc(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-csc"),
        }
    }

    /// Type of payment instrument.
    pub fn cc_type(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("cc-type"),
        }
    }

    /// The currency that the user would prefer the transaction to use.
    pub fn transaction_currency(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("transaction-currency"),
        }
    }

    /// The amount that the user would like for the transaction (e.g. when entering a bid or sale
    /// price).
    pub fn transaction_amount(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("transaction-amount"),
        }
    }

    /// Preferred language.
    pub fn language(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("language"),
        }
    }

    /// Birthday.
    pub fn bday(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("bday"),
        }
    }

    /// Day component of birthday.
    pub fn bday_day(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("bday-day"),
        }
    }

    /// Month component of birthday.
    pub fn bday_month(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("bday-month"),
        }
    }

    /// Year component of birthday.
    pub fn bday_year(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("bday-year"),
        }
    }

    /// Gender identity (e.g. Female, Fa'afafine).
    pub fn sex(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("sex"),
        }
    }

    /// Home page or other web page corresponding to the company, person, address, or contact
    /// information in the other fields associated with this field.
    pub fn url(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("url"),
        }
    }

    /// Photograph, icon, or other image corresponding to the company, person, address, or contact
    /// information in the other fields associated with this field.
    pub fn photo(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("photo"),
        }
    }

    /// Full telephone number, including country code.
    pub fn tel(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("tel"),
        }
    }

    /// Country code component of the telephone number.
    pub fn tel_country_code(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("tel-country-code"),
        }
    }

    /// Telephone number without the county code component, with a country-internal prefix applied
    /// if applicable.
    pub fn tel_national(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("tel-national"),
        }
    }

    /// Area code component of the telephone number, with a country-internal prefix applied if
    /// applicable.
    pub fn tel_area_code(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("tel-area-code"),
        }
    }

    /// Telephone number without the country code and area code components.
    pub fn tel_local(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("tel-local"),
        }
    }

    /// First part of the component of the telephone number that follows the area code, when that
    /// component is split into two components.
    pub fn tel_local_prefix(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("tel-local-prefix"),
        }
    }

    /// Second part of the component of the telephone number that follows the area code, when that
    /// component is split into two components.
    pub fn tel_local_suffix(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("tel-local-suffix"),
        }
    }

    /// Telephone number internal extension code.
    pub fn tel_extension(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("tel-extension"),
        }
    }

    /// Email address.
    pub fn email(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("email"),
        }
    }

    /// URL representing an instant messaging protocol endpoint.
    pub fn impp(self) -> AutoComplete {
        AutoComplete {
            a: self.a,
            b: self.b,
            c: self.c,
            d: Some("impp"),
        }
    }
}

impl fmt::Display for AutoComplete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut is_first = true;

        if let Some(a) = &self.a {
            write!(f, "section-{a}")?;
            is_first = false;
        }

        if let Some(b) = self.b {
            if !is_first {
                write!(f, " ")?;
            }
            write!(f, "{b}")?;
            is_first = false;
        }

        if let Some(c) = self.c {
            if !is_first {
                write!(f, " ")?;
            }
            write!(f, "{c}")?;
            is_first = false;
        }

        if let Some(d) = self.d {
            if !is_first {
                write!(f, " ")?;
            }
            write!(f, "{d}")?;
        }

        Ok(())
    }
}
