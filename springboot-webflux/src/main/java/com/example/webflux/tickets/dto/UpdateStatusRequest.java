package com.example.webflux.tickets.dto;

import com.example.webflux.tickets.model.TicketStatus;

import jakarta.validation.constraints.NotNull;

public record UpdateStatusRequest(@NotNull TicketStatus status) {}
