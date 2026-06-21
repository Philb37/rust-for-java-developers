#!/bin/bash -e

echo -e "\n"
echo "Starting database init script..."

POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-password}"

echo -e "\n"
echo "Creating ticket table"


psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    create table if not exists public.tickets (
        id integer generated always as identity primary key,
        title text not null,
        description text,
        status text not null,
        priority text not null,
        assignee text,
        created_at timestamp with time zone default now() not null
    );

    create index idx_tickets_status
    on public.tickets(status);

    create index idx_tickets_priority
    on public.tickets(priority);

    create index idx_tickets_status_priority
    on public.tickets(status, priority);
EOSQL

echo -e "\n"
echo "Ending database init script."