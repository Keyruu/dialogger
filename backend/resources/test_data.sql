-- Inserting into movie
INSERT INTO movie (id, name) VALUES 
(1, 'Der Wixxer');

-- Inserting into character
INSERT INTO character (id, name, description, movie_id) VALUES
(1, 'Even Longer', 'Chief Inspector', 1),
(2, 'Very Long', 'Inspector', 1),
(3, 'Sir John', NULL, 1);

-- Inserting into location
INSERT INTO location (id, name, movie_id) VALUES 
(1, 'Blackwhite Castle', 1),
(2, 'Scotland Yard - Sir John''s Office', 1);

-- Inserting into scene
INSERT INTO scene (id, name, location_id, movie_id) VALUES
(1, 'First Castle Visit', 1, 1),
(2, 'Very Long''s Introduction', 2, 1);

-- Inserting into conversation
INSERT INTO conversation (id, name, scene_id, movie_id) VALUES
(1, 'Arrival', 1, 1),
(2, 'Very Long''s Introduction', 2, 1);

-- Inserting into conversation_participants
INSERT INTO conversation_participants (conversation_id, participant_id) VALUES
(1, 1),
(1, 2),
(2, 1),
(2, 2),
(2, 3);

-- Inserting into sentence
INSERT INTO sentence (id, text, speaker_id, conversation_id, movie_id) VALUES
(1, 'Ich habe einen neuen Partner für Sie.', 3, 2, 1),
(2, 'Einen wunderschönen guten Morgen. Ich bin ja so froh...', 2, 2, 1),
(3, 'Sind Sie jetzt vollständig verkalkt?', 1, 2, 1),
(4, 'Er war der Beste auf der Polizeischule!', 3, 2, 1),
(5, 'Und er ist eine geborene Frohnatur.', 3, 2, 1),
(6, 'Genau das, was Sie in Ihrem augenblicklichen Gemütszustand dringend brauchen.', 3, 2, 1),
(7, '...froh, Sie endlich kennenzulernen.', 2, 2, 1),
(8, 'Ups! Meine Melone. Hauptsache, die Vitamine sind nicht raus.', 2, 2, 1);

-- Inserting into sentence_directed_to
INSERT INTO sentence_directed_to (sentence_id, directed_to_id) VALUES
(1, 1),
(2, 1),
(2, 3),
(3, 3),
(4, 1),
(5, 1),
(6, 1),
(7, 1),
(7, 3),
(8, 1),
(8, 3);
