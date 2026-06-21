package com.example.webflux.tickets.repository;

import org.springframework.data.relational.core.query.Criteria;

import com.example.webflux.tickets.model.Priority;
import com.example.webflux.tickets.model.TicketStatus;

// The equivalent of SeaORM's QuerySelect::apply_if — each filter contributes a
// WHERE clause only when its parameter is present. Combining with Criteria.empty()
// means a null filter adds nothing, so all-null returns "select everything".
public final class TicketCriteria {

    private TicketCriteria() { }

    public static Criteria forFilters(TicketStatus status, Priority priority) {
        Criteria criteria = Criteria.empty();
        if (status != null) {
            criteria = criteria.and("status").is(status);
        }
        if (priority != null) {
            criteria = criteria.and("priority").is(priority);
        }

        return criteria;
    }
}
