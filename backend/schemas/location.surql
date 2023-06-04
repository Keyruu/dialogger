DEFINE TABLE location SCHEMAFULL;

DEFINE FIELD name ON TABLE location TYPE string
  ASSERT $value != NONE;
DEFINE FIELD movie ON TABLE location TYPE record (movie)
  ASSERT $value != NONE;
