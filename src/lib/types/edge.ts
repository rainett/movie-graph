export type Relationship = 'acted_in' | 'liked_actor' | 'recommended';

export type Edge = {
  id: string;
  from: string;
  to: string;
  relationship: Relationship;
  note: string | null;
  created_at: string;
};
