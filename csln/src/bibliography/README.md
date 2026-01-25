# csln-bibliography

This library implements the core bibliography data model for CSLNext. It is designed to be highly structured where needed (e.g., for names and dates) while remaining flexible for diverse bibliographic data.

## Key Concepts

### InputReference
The primary unit of data. It is an enum with several variants:
- **Monograph**: Books, reports, etc.
- **Collection**: Edited volumes, anthologies.
- **CollectionComponent**: Chapters or parts of a collection.
- **SerialComponent**: Articles in journals, newspapers, etc.

### Contributor
Represents persons or organizations. Supports simple strings, structured names (given/family), and lists. It includes formatting logic for names (e.g., initials, sorting order).

### Date (EDTF)
Dates are stored as EDTF strings, allowing for flexible date-time encoding (uncertain dates, intervals, seasons). The library provides utilities to extract years and months from these strings.

## Usage
The `InputBibliography` type is a `HashMap<String, InputReference>`, where the key is the citation key (ID).

JSON schemas for these models can be generated using the `csln-schemas` binary in the `cli` crate.

