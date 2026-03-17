package com.example.tickets.exception;

// Rust parallel: a custom error type used with Result<T, TicketNotFoundError>
// In Java: extends RuntimeException so Spring maps it to HTTP 404 via @ExceptionHandler
public class TicketNotFoundException extends RuntimeException {

    public TicketNotFoundException(Long id) {
        super("Ticket not found: " + id);
    }
}
