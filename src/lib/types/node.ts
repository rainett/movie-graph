export type Status = 'watched' | 'watching' | 'want_to_watch' | 'dropped' | 'none';

export type Position = {
  x: number;
  y: number;
};

export type MovieNode = {
  id: string;
  tmdb_id: number;
  title: string;
  year: number;
  status: Status;
  my_rating: number | null;
  poster: string;
  poster_options: string[];
  notes: string;
  position: Position;
  added_at: string;
};

export type ActorNode = {
  id: string;
  tmdb_id: number;
  name: string;
  photo: string;
  photo_options: string[];
  notes: string;
  position: Position;
  added_at: string;
};
