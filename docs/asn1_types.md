# ASN.1 Data Types

ASN.1 (Abstract Syntax Notation One) is a standard interface description language used for defining data structures for serialization and deserialization. It is commonly used in areas such as cryptography and X.509 certificates.

## Common ASN.1 Types

| Type                         | Description                              |
| ---------------------------- | ---------------------------------------- |
| `BOOLEAN`                    | True/False                               |
| `INTEGER`                    | Arbitrary precision signed integer       |
| `BIT STRING`                 | A string of bits                         |
| `OCTET STRING`               | A string of bytes                        |
| `NULL`                       | Null value                               |
| `OBJECT IDENTIFIER`          | For referencing standard identifiers     |
| `UTF8String`                 | UTF-8 encoded string                     |
| `IA5String`                  | ASCII string (used for DNS, email, etc.) |
| `PrintableString`            | Restricted character set string          |
| `UTCTime`, `GeneralizedTime` | Date/time formats                        |
| `SEQUENCE`                   | Ordered list of elements                 |
| `SET`                        | Unordered set of elements                |

