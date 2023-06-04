use ns test;
use db test;

insert into movie { id: 'movie:wixxer', name: 'Der Wixxer'};

insert into character {
  id: 'character:even', 
  name: 'Even Longer', 
  description: 'Chief Inspector', 
  movie: movie:wixxer
};
insert into character { 
  id: 'character:very', 
  name: 'Very Long', 
  description: 'Inspector', 
  movie: movie:wixxer
};
insert into character { 
  id: 'character:john', 
  name: 'Sir John',
  movie: movie:wixxer
};

insert into location { 
  id: 'location:castle', 
  name: 'Blackwhite Castle', 
  movie: movie:wixxer
};
insert into location { 
  id: 'location:sirjohnsoffice', 
  name: 'Scotland Yard - Sir John\'s Office', 
  movie: movie:wixxer
};

insert into scene { 
  id: 'scene:firstcastlevisit', 
  name: 'First Castle Visit', 
  location: location:castle, 
  movie: movie:wixxer
};
insert into scene { 
  id: 'scene:veryintroduction', 
  name: 'Very Long\'s Introduction', 
  location: location:sirjohnsoffice, 
  movie: movie:wixxer
};

insert into conversation { 
  id: 'conversation:castlevisit', 
  name: 'Arrival', 
  participants: [character:even, character:very], 
  scene: scene:firstcastlevisit, 
  movie: movie:wixxer
};
insert into conversation { 
  id: 'conversation:veryintroduction', 
  name: 'Very Long\'s Introduction', 
  participants: [character:even, character:very, character:john], 
  scene: scene:veryintroduction, 
  movie: movie:wixxer
};

insert into sentence { 
  text: 'Ich habe einen neuen Partner für Sie.', 
  speaker: character:john, 
  directed_to: [character:even], 
  conversation: conversation:veryintroduction, 
  movie: movie:wixxer
};
insert into sentence { 
  text: 'Einen wunderschönen guten Morgen. Ich bin ja so froh...', 
  speaker: character:very, 
  directed_to: [character:even, character:john], 
  conversation: conversation:veryintroduction, 
  movie: movie:wixxer
};
insert into sentence { 
  text: 'Sind Sie jetzt vollständig verkalkt?', 
  speaker: character:even, 
  directed_to: [character:john], 
  conversation: conversation:veryintroduction, 
  movie: movie:wixxer
};
insert into sentence { 
  text: 'Er war der Beste auf der Polizeischule!', 
  speaker: character:john, 
  directed_to: [character:even], 
  conversation: conversation:veryintroduction, 
  movie: movie:wixxer
};
insert into sentence { 
  text: 'Und er ist eine geborene Frohnatur.', 
  speaker: character:john, 
  directed_to: [character:even], 
  conversation: conversation:veryintroduction, 
  movie: movie:wixxer
};
insert into sentence { 
  text: 'Genau das, was Sie in Ihrem augenblicklichen Gemütszustand dringend brauchen.', 
  speaker: character:john, 
  directed_to: [character:even], 
  conversation: conversation:veryintroduction, 
  movie: movie:wixxer
};
insert into sentence { 
  text: '...froh, Sie endlich kennenzulernen.', 
  speaker: character:very, 
  directed_to: [character:even, character:john], 
  conversation: conversation:veryintroduction, 
  movie: movie:wixxer
};
insert into sentence { 
  text: 'Ups! Meine Melone. Hauptsache, die Vitamine sind nicht raus.', 
  speaker: character:very, 
  directed_to: [character:even, character:john], 
  conversation: conversation:veryintroduction, 
  movie: movie:wixxer
};