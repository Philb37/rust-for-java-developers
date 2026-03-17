package com.example.tickets.dto;

import com.example.tickets.model.Priority;
import com.example.tickets.model.Ticket;
import com.example.tickets.model.TicketStatus;

import java.time.LocalDateTime;
import java.util.Optional;

// Rust parallel: struct TicketResponse { id: u64, title: String, description: Option<String>, ... }
public record TicketResponse(
        Long id,
        String title,
        Optional<String> description,
        TicketStatus status,
        Priority priority,
        Optional<String> assignee,
        LocalDateTime createdAt
) {
    // Rust parallel: impl From<Ticket> for TicketResponse
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
