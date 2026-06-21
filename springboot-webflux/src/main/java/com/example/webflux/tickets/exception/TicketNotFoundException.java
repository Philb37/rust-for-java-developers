package com.example.webflux.tickets.exception;

public class TicketNotFoundException extends RuntimeException {

    public TicketNotFoundException(Integer id) {
        super("Ticket not found: " + id);
    }
}
