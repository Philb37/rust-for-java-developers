-- seed.sql
TRUNCATE public.tickets RESTART IDENTITY;
INSERT INTO public.tickets (title, description, status, priority, assignee, created_at)
SELECT 'seed ' || g,
       (ARRAY['Random desc', 'Another desc'])[random(1, 4)],
       (ARRAY['OPEN','IN_PROGRESS','RESOLVED','CLOSED'])[random(1, 4)],
       (ARRAY['LOW','MEDIUM','HIGH','CRITICAL'])[random(1, 4)],
       (ARRAY['Me', 'You', 'Them'])[random(1, 4)],
       now()
FROM generate_series(1, 10000) g;