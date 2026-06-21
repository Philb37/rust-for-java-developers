package com.example.tickets.dto;

import com.example.tickets.model.TicketStatus;

import jakarta.validation.constraints.NotNull;

public record UpdateStatusRequest(@NotNull TicketStatus status) {}
