package com.example.webflux.tickets.repository;

import org.springframework.data.r2dbc.core.R2dbcEntityTemplate;
import org.springframework.data.relational.core.query.Query;

import com.example.webflux.tickets.model.Priority;
import com.example.webflux.tickets.model.Ticket;
import com.example.webflux.tickets.model.TicketStatus;

import reactor.core.publisher.Flux;

// Spring Data picks up the "<Repository>Impl" fragment by name and injects the
// R2dbcEntityTemplate bean here — so the template stays in the persistence layer.
public class TicketRepositoryImpl implements TicketRepositoryCustom {

    private final R2dbcEntityTemplate template;

    public TicketRepositoryImpl(R2dbcEntityTemplate template) {
        this.template = template;
    }

    @Override
    public Flux<Ticket> findByFilters(TicketStatus status, Priority priority) {
        // Filtering happens in SQL — the Criteria becomes the WHERE clause; capped at LIMIT 100.
        Query query = Query.query(TicketCriteria.forFilters(status, priority)).limit(100);
        return template.select(query, Ticket.class);
    }

    @Override
    public Flux<StatCount> countByStatusAndPriority() {
        // Counts grouped by status AND priority in a single pass (Postgres GROUPING SETS).
        return template.getDatabaseClient()
                .sql("""
                        SELECT status, priority, COUNT(*) AS count
                        FROM tickets
                        GROUP BY GROUPING SETS ((status), (priority))
                        """)
                .map((row, meta) -> new StatCount(
                        row.get("status", String.class),
                        row.get("priority", String.class),
                        row.get("count", Long.class)))
                .all();
    }
}
