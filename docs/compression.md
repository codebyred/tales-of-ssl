# Compressing / Decompressing

**Purpose:** Reduce data size for efficiency.

## Compressing

Compressing takes raw data and encodes it in a way that takes up less space using algorithms such as:

- `gzip`
- `zlib`
- `brotli`
- etc.

## Decompressing

Decompressing restores the compressed data back to its original form.

## Compression Types

- **Lossless Compression:** No data is lost during compression.
  - Examples: `ZIP`, `PNG`, `FLAC`
- **Lossy Compression:** Some data is discarded to reduce size, usually in a way that is acceptable for the use case.
  - Examples: `MP3`, `JPEG`, `MPEG`
