use anyhow::Context;
use openssl::{base64, nid::Nid, pkey::PKey, rsa::Rsa, x509::X509NameBuilder};
use crate::{problem::{self, Problem, RequiredData}, utility::get_iso_country_name_from};

pub struct Certificate;

pub struct CertificateBuilder;

impl CertificateBuilder {
    pub fn build(problem: Problem)-> anyhow::Result<()>{

        let key_der_format = base64::decode_block(&problem.private_key).context("decoding private key with base64 decoder")?;
        let rsa = Rsa::private_key_from_der(&key_der_format[..]).context("creating public key form der format key")?;
        let public_key = PKey::from_rsa(rsa.clone()).context("")?;

        let iso_country_name = get_iso_country_name_from(&problem.required_data.country);

        let mut name_builder = X509NameBuilder::new().context("creating X509NameBuilder")?;
        name_builder.append_entry_by_nid(Nid::COMMONNAME, &problem.required_data.domain).context("appending domain to name_builder")?;
        name_builder.append_entry_by_nid(Nid::COUNTRYNAME, &iso_country_name).context("appending country to name_builder")?;
        let name = name_builder.build();

        Ok(())

    }
}







