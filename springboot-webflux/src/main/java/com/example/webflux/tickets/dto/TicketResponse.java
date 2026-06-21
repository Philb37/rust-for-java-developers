package com.example.webflux.tickets.dto;

import java.time.OffsetDateTime;
import java.util.Optional;

import com.example.webflux.tickets.model.Priority;
import com.example.webflux.tickets.model.Ticket;
import com.example.webflux.tickets.model.TicketStatus;

public record TicketResponse(
        Integer id,
        String title,
        Optional<String> description,
        TicketStatus status,
        Priority priority,
        Optional<String> assignee,
        OffsetDateTime createdAt
) {
    public static TicketResponse from(Ticket ticket) {
        return new TicketResponse(
                ticket.getId(),
                ticket.getTitle(),
                ticket.getDescription(),  // already Optional<String>
                ticket.getStatus(),
                ticket.getPriority(),
                ticket.getAssignee(),     // already Optional<String>
                ticket.getCreatedAt()
        );
    }
}