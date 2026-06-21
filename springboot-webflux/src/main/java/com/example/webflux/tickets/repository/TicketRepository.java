package com.example.webflux.tickets.repository;

import org.springframework.data.r2dbc.repository.R2dbcRepository;

import com.example.webflux.tickets.model.Ticket;

// Reactive CRUD (R2dbcRepository) plus the custom fragment (TicketRepositoryCustom),
// which holds the dynamic filter + GROUPING SETS queries backed by R2dbcEntityTemplate.
public interface TicketRepository extends R2dbcRepository<Ticket, Integer>, TicketRepositoryCustom { }
