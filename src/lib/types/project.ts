export type ViewState = {
  camera_x: number;
  camera_y: number;
  zoom: number;
  selected_node: string | null;
};

export type DevicePosition = {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  visible: boolean;
};

export type DeviceLayout = {
  preset: string;
  devices: DevicePosition[];
};

export type Manifest = {
  version: string;
  name: string;
  created_at: string;
  modified_at: string;
  view_state: ViewState;
  device_layout: DeviceLayout;
};

export type Project = {
  path: string;
  manifest: Manifest;
  movies: import('./node').MovieNode[];
  actors: import('./node').ActorNode[];
  edges: import('./edge').Edge[];
};

export type RecentProject = {
  path: string;
  name: string;
  last_opened: string;
};

export type ValidationResult = {
  valid: boolean;
  errors: string[];
};
