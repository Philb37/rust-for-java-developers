package com.example.webflux.tickets.repository;

import com.example.webflux.tickets.model.Priority;
import com.example.webflux.tickets.model.Ticket;
import com.example.webflux.tickets.model.TicketStatus;

import reactor.core.publisher.Flux;

// Data-access operations that don't fit the derived-query model. Implemented in
// TicketRepositoryImpl with R2dbcEntityTemplate, so that template never leaks into
// the service layer.
public interface TicketRepositoryCustom {

    Flux<Ticket> findByFilters(TicketStatus status, Priority priority);

    Flux<StatCount> countByStatusAndPriority();
}
