package com.example.tickets.repository;

import org.springframework.data.jpa.domain.Specification;

import com.example.tickets.model.Priority;
import com.example.tickets.model.Ticket;
import com.example.tickets.model.TicketStatus;

// The equivalent of SeaORM's QuerySelect::apply_if — each method contributes a
// WHERE clause only when its parameter is present.
public final class TicketSpecifications {

    private TicketSpecifications() { }

    public static Specification<Ticket> hasStatus(TicketStatus status) {
        return (root, query, cb) ->
                status == null ? cb.conjunction() : cb.equal(root.get("status"), status);
    }

    public static Specification<Ticket> hasPriority(Priority priority) {
        return (root, query, cb) ->
                priority == null ? cb.conjunction() : cb.equal(root.get("priority"), priority);
    }
}
