# TALES OF SSL
A solution of hackattic task tales of ssl problem where you have to generate a self-signed x.509 certificate
from a DER base64 encoded private key with the required data.

## Features
* **Requested handler to automatically fetch problem and submit solution**

    `reqwest` and `tokio` crates are used to asynchronously fetch data

* **Visual indicator to show fetching problem and submitting solution**

    `indicatif` is used to show progress spinner indicating problem fetch and submission status

* **Openssl self signed certification genration**

    `openssl` crate is used to generate a base64 der encoded x.509 certificate

## Documentation

[ASN1 TYPES](docs/asn1_types.md)

[OPENSSL NID](docs/openssl_nid.md)

[ENCODING](docs/encoding.md)

[COMPRESSION](docs/compression.md)

## Tools
- anyhow
- reqwest
- tokio
- serde
- dotenvy
- serde_json
- openssl
- indicatif
- hex

## Usage
Clone the github repo

```git clone https://github.com/codebyred/tales-of-ssl.git```

Open the folder

Add a .env file and REMOTE and you own ACCESS_TOKEN

<pre>REMOTE=https://hackattic.com/challenges/tales_of_ssl
ACCESS_TOKEN=</pre>


Build the project

```cargo build```

Then run

```cargo run```

This will automatically fetch the problem and submit the solution


