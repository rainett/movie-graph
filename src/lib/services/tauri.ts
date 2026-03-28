import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import type { Project, RecentProject, ValidationResult } from '../types/project';
import type { MovieNode, ActorNode } from '../types/node';
import type { Edge } from '../types/edge';
import type { SearchResults, MovieResult, PersonResult, MovieDetails, PersonDetails } from '../types/tmdb';

export async function createProject(path: string, name: string): Promise<Project> {
  return tauriInvoke('create_project', { path, name });
}

export async function openProject(path: string): Promise<Project> {
  return tauriInvoke('open_project', { path });
}

export async function saveProject(project: Project): Promise<void> {
  return tauriInvoke('save_project', { project });
}

export async function getRecentProjects(): Promise<RecentProject[]> {
  return tauriInvoke('get_recent_projects');
}

export async function validateProject(path: string): Promise<ValidationResult> {
  return tauriInvoke('validate_project', { path });
}

export async function pickFolder(): Promise<string | null> {
  return tauriInvoke('pick_folder');
}

export async function saveMovies(projectPath: string, movies: MovieNode[]): Promise<void> {
  return tauriInvoke('save_movies', { projectPath, movies });
}

export async function saveActors(projectPath: string, actors: ActorNode[]): Promise<void> {
  return tauriInvoke('save_actors', { projectPath, actors });
}

export async function saveEdges(projectPath: string, edges: Edge[]): Promise<void> {
  return tauriInvoke('save_edges', { projectPath, edges });
}

export async function searchMovies(
  query: string,
  page?: number,
  apiKey?: string,
): Promise<SearchResults<MovieResult>> {
  return tauriInvoke('search_movies', { query, page, apiKey });
}

export async function searchPeople(
  query: string,
  page?: number,
  apiKey?: string,
): Promise<SearchResults<PersonResult>> {
  return tauriInvoke('search_people', { query, page, apiKey });
}

export async function getMovieDetails(tmdbId: number, apiKey?: string): Promise<MovieDetails> {
  return tauriInvoke('get_movie_details', { tmdbId, apiKey });
}

export async function getPersonDetails(tmdbId: number, apiKey?: string): Promise<PersonDetails> {
  return tauriInvoke('get_person_details', { tmdbId, apiKey });
}

export async function testApiKey(apiKey: string): Promise<boolean> {
  return tauriInvoke('test_api_key', { apiKey });
}
