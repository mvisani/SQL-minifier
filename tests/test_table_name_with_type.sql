-- Your SQL goes here
CREATE TABLE IF NOT EXISTS taxaINTEGER_BOOLEAN_INT (
    -- The unique identifier for the taxon
    id UUID PRIMARY KEY,
    -- The scientific name of the taxon
    name TEXT NOT NULL,
    -- The NCBI Taxon ID is a unique identifier for a taxon in the NCBI Taxonomy database
    -- which may be NULL when this taxon is not present in the NCBI Taxonomy database.
    ncbi_taxon_id INTEGER
);
