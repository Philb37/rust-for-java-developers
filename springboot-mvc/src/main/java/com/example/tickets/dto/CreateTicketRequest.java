package com.example.tickets.dto;

import com.example.tickets.model.Priority;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;

// Java 21 record — immutable data carrier
public record CreateTicketRequest(
        @NotBlank String title,
        @NotNull Priority priority,
        String description,  // null → Option::None
        String assignee      // null → Option::None
) {}
