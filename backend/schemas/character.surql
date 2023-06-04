DEFINE TABLE character SCHEMAFULL;

DEFINE FIELD name ON TABLE character TYPE string
  ASSERT $value != NONE;
DEFINE FIELD image ON TABLE character TYPE string;
DEFINE FIELD description ON TABLE character TYPE string;
DEFINE FIELD movie ON TABLE character TYPE record (movie)
  ASSERT $value != NONE;