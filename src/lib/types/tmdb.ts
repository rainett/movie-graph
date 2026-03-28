export type SearchResults<T> = {
  page: number;
  total_pages: number;
  total_results: number;
  results: T[];
};

export type MovieResult = {
  id: number;
  title: string;
  release_date: string | null;
  overview: string | null;
  poster_path: string | null;
  vote_average: number;
};

export type PersonResult = {
  id: number;
  name: string;
  profile_path: string | null;
  known_for_department: string | null;
};

export type CastMember = {
  id: number;
  name: string;
  character: string | null;
  profile_path: string | null;
  order: number;
};

export type MovieDetails = {
  id: number;
  title: string;
  release_date: string | null;
  overview: string | null;
  poster_path: string | null;
  runtime: number | null;
  vote_average: number;
  genres: { id: number; name: string }[];
  credits: { cast: CastMember[] };
};

export type MovieCredit = {
  id: number;
  title: string;
  release_date: string | null;
  character: string | null;
  poster_path: string | null;
  popularity: number | null;
  vote_average: number | null;
};

export type PersonDetails = {
  id: number;
  name: string;
  biography: string | null;
  profile_path: string | null;
  birthday: string | null;
  place_of_birth: string | null;
  movie_credits: { cast: MovieCredit[] };
};

export const TMDB_IMAGE_BASE = 'https://image.tmdb.org/t/p';

export function tmdbImage(path: string | null, size: 'w200' | 'w500' | 'original' = 'w200'): string | null {
  if (!path) return null;
  return `${TMDB_IMAGE_BASE}/${size}${path}`;
}

export function releaseYear(date: string | null | undefined): string {
  if (!date) return '';
  return date.slice(0, 4);
}
