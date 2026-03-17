package com.example.tickets.dto;

import com.example.tickets.model.TicketStatus;

// Rust parallel: struct UpdateStatusRequest { status: TicketStatus }
public record UpdateStatusRequest(TicketStatus status) {}
