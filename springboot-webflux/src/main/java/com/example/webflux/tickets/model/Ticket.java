package com.example.webflux.tickets.model;

import java.time.OffsetDateTime;
import java.util.Optional;

import org.springframework.data.annotation.Id;
import org.springframework.data.relational.core.mapping.Column;
import org.springframework.data.relational.core.mapping.Table;

@Table("tickets")
public class Ticket {

    @Id
    private Integer id;

    private String title;

    private String description;

    // Stored as text — Spring Data R2DBC writes enums via name() by default
    private TicketStatus status;

    private Priority priority;

    private String assignee;

    // Default naming keeps property names verbatim, so map camelCase → snake_case explicitly
    @Column("created_at")
    private OffsetDateTime createdAt;

    // Defaults (createdAt, status) are applied by TicketDefaultsCallback before the
    // entity is converted to a row — the reactive equivalent of JPA's @PrePersist.

    public Integer getId() { return id; }
    public void setId(Integer id) { this.id = id; }

    public String getTitle() { return title; }
    public void setTitle(String title) { this.title = title; }

    public Optional<String> getDescription() { return Optional.ofNullable(description); }
    public void setDescription(String description) { this.description = description; }

    public TicketStatus getStatus() { return status; }
    public void setStatus(TicketStatus status) { this.status = status; }

    public Priority getPriority() { return priority; }
    public void setPriority(Priority priority) { this.priority = priority; }

    public Optional<String> getAssignee() { return Optional.ofNullable(assignee); }
    public void setAssignee(String assignee) { this.assignee = assignee; }

    public OffsetDateTime getCreatedAt() { return createdAt; }
    public void setCreatedAt(OffsetDateTime createdAt) { this.createdAt = createdAt; }
}
