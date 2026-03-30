export type AppConfig = {
  tmdb_api_key: string | null;
  tmdb_read_access_token: string | null;
  recent_projects: Array<{ path: string; name: string; last_opened: string }>;
  max_recent_projects: number;
  auto_save: boolean;
  auto_save_interval_ms: number;
  auto_backup: boolean;
  max_backups: number;
  sound_enabled: boolean;
  reduced_motion: boolean;
  theme: string;
  device_layout_preset: string;
};
