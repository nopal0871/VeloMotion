-- VeloMotion Clone Database Schema (Supabase / PostgreSQL)

create table projects (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null,
  title text not null default 'Untitled Project',
  timeline_json jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index idx_projects_user_id on projects (user_id);
create index idx_projects_updated_at on projects (updated_at desc);

-- Enable Row Level Security (RLS)
alter table projects enable row level security;

create policy "Users can CRUD their own projects"
  on projects for all
  using (auth.uid() = user_id)
  with check (auth.uid() = user_id);
