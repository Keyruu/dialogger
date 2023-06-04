DEFINE TABLE scene SCHEMAFULL;

DEFINE FIELD name ON TABLE scene TYPE string
  ASSERT $value != NONE;
DEFINE FIELD location ON TABLE scene TYPE record (location)
  ASSERT $value != NONE;
DEFINE FIELD movie ON TABLE scene TYPE record (movie)
  ASSERT $value != NONE;
