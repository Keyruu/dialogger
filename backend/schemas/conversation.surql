DEFINE TABLE conversation SCHEMAFULL;

DEFINE FIELD name ON TABLE conversation TYPE string;
DEFINE FIELD participants ON TABLE conversation TYPE array
  ASSERT array::len($value) > 0;
DEFINE FIELD participants.* ON TABLE conversation TYPE record (character);
DEFINE FIELD scene ON TABLE conversation TYPE record (scene);
DEFINE FIELD movie ON TABLE conversation TYPE record (movie)
  ASSERT $value != NONE;