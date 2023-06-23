-- Creating the movie table
CREATE TABLE movie (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

-- Creating the character table
CREATE TABLE character (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    image VARCHAR(255),
    description TEXT,
    movie_id BIGINT NOT NULL REFERENCES movie(id)
);

-- Creating the location table
CREATE TABLE location (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    movie_id BIGINT NOT NULL REFERENCES movie(id)
);

-- Creating the scene table
CREATE TABLE scene (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    location_id BIGINT REFERENCES location(id),
    movie_id BIGINT NOT NULL REFERENCES movie(id)
);

-- Creating the conversation table
CREATE TABLE conversation (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    scene_id BIGINT REFERENCES scene(id),
    movie_id BIGINT NOT NULL REFERENCES movie(id)
);

-- Creating the table for conversation participants
CREATE TABLE conversation_participants (
    conversation_id BIGINT REFERENCES conversation(id),
    participant_id BIGINT REFERENCES character(id),
    PRIMARY KEY(conversation_id, participant_id)
);

-- Creating the sentence table
CREATE TABLE sentence (
    id BIGSERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    speaker_id BIGINT REFERENCES character(id),
    conversation_id BIGINT REFERENCES conversation(id),
    movie_id BIGINT NOT NULL REFERENCES movie(id)
);

-- Creating the table for sentence's directed to
CREATE TABLE sentence_directed_to (
    sentence_id BIGINT NOT NULL REFERENCES sentence(id),
    directed_to_id BIGINT NOT NULL REFERENCES character(id),
    PRIMARY KEY(sentence_id, directed_to_id)
);
