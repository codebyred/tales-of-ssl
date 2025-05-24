# OpenSSL NID (Numerical Identifier)

In **OpenSSL**, a **NID** (short for *Numerical Identifier*) is an internal numeric constant that uniquely identifies various types of objects, such as:

- **Algorithms** (e.g., `SHA256`, `RSA`)
- **Object Identifiers (OIDs)**
- **X.509 Attribute Types** (e.g., `commonName`, `countryName`)
- **Elliptic Curves** (e.g., `prime256v1`)

OpenSSL maintains a table that maps these NIDs to string names and OIDs.

## Common NID Values

| Name           | NID | OID                          |
|----------------|-----|------------------------------|
| `rsaEncryption`| 6   | 1.2.840.113549.1.1.1          |
| `sha256`       | 672 | 2.16.840.1.101.3.4.2.1        |
| `prime256v1`   | 415 | 1.2.840.10045.3.1.7           |
| `commonName`   | 13  | 2.5.4.3                       |
| `countryName`  | 14  | 2.5.4.6                       |
