DEFINE TABLE sentence SCHEMAFULL;

DEFINE FIELD text ON TABLE sentence TYPE string
  ASSERT $value != NONE;
DEFINE FIELD speaker ON TABLE sentence TYPE record (character)
  ASSERT $value != NONE;
DEFINE FIELD conversation ON TABLE sentence TYPE record (conversation)
  ASSERT $value != NONE;
DEFINE FIELD directed_to ON TABLE sentence TYPE array
  ASSERT array::len($value) > 0;
DEFINE FIELD directed_to.* ON TABLE sentence TYPE record (character);
DEFINE FIELD movie ON TABLE sentence TYPE record (movie)
  ASSERT $value != NONE;