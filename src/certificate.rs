use anyhow::Context;
use openssl::{asn1, base64, bn::BigNum, hash::MessageDigest, nid::Nid, pkey::PKey, rsa::Rsa, x509::{X509NameBuilder, X509}};
use crate::{problem::Problem, utility::get_iso_country_name_from};

pub struct Certificate;

pub struct CertificateBuilder;

impl CertificateBuilder {
    pub fn build(problem: Problem)-> anyhow::Result<()>{

        let key_der_format = base64::decode_block(&problem.private_key).context("decoding private key with base64 decoder")?;
        let rsa = Rsa::private_key_from_der(&key_der_format[..]).context("creating public key form der format key")?;
        let pkey = PKey::from_rsa(rsa.clone()).context("")?;

        let iso_country_name = get_iso_country_name_from(&problem.required_data.country);

        let mut name_builder = X509NameBuilder::new().context("creating X509NameBuilder")?;
        name_builder.append_entry_by_nid(Nid::COMMONNAME, &problem.required_data.domain).context("appending domain to name_builder")?;
        name_builder.append_entry_by_nid(Nid::COUNTRYNAME, &iso_country_name).context("appending country to name_builder")?;
        let name = name_builder.build();


        let mut builder = X509::builder().context("creating x509 builder")?;

        builder.set_version(2).context("setting x.509 version")?;

        builder.set_subject_name(&name).context("setting x.509 subject name")?;
        builder.set_issuer_name(&name).context("setting x.509 issuer name")?;

        builder.set_pubkey(&pkey).context("setting x.509 public key")?;

        let serial_number = BigNum::from_hex_str(&problem.required_data.serial_number)
        .context("converting big num from hex")?.to_asn1_integer().context("converting bignum to to_asn1_integer")?;
        builder.set_serial_number(&serial_number).context("setting x.509 serial number")?;

        let not_before = asn1::Asn1Time::days_from_now(0).context("")?;
        let not_after = asn1::Asn1Time::days_from_now(365).context("")?;

        builder.set_not_before(&not_before)?;
        builder.set_not_after(&not_after)?;

        builder.sign(&pkey, MessageDigest::sha256()).context("siging the certificate with private key")?;

        let certificate = builder.build();

        let der = certificate.to_der().context("converting certificate to pem")?;

        let base64_der = base64::encode_block(&der[..]);

        println!("{}", base64_der);

        Ok(())

    }
}







