package com.example.tickets.model;

import jakarta.persistence.*;
import java.time.LocalDateTime;
import java.util.Optional;

// Rust parallel:
// struct Ticket {
//     id: u64,
//     title: String,
//     description: Option<String>,
//     status: TicketStatus,
//     priority: Priority,
//     assignee: Option<String>,
//     created_at: DateTime,
// }
@Entity
@Table(name = "tickets")
public class Ticket {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;

    @Column(columnDefinition = "text", nullable = false)
    private String title;

    // Option<String> — may or may not have a description
    @Column(columnDefinition = "text")
    private String description;

    @Enumerated(EnumType.STRING)
    @Column(columnDefinition = "text", nullable = false)
    private TicketStatus status;

    @Enumerated(EnumType.STRING)
    @Column(columnDefinition = "text", nullable = false)
    private Priority priority;

    // Option<String> — may or may not be assigned
    @Column(columnDefinition = "text")
    private String assignee;

    @Column(nullable = false)
    private LocalDateTime createdAt;

    @PrePersist
    protected void onCreate() {
        this.createdAt = LocalDateTime.now();
        if (this.status == null) {
            this.status = TicketStatus.OPEN;
        }
    }

    public Integer getId() { return id; }
    public void setId(Integer id) { this.id = id; }

    public String getTitle() { return title; }
    public void setTitle(String title) { this.title = title; }

    // Returns Optional<String> — explicit Option<T> parallel
    public Optional<String> getDescription() { return Optional.ofNullable(description); }
    public void setDescription(String description) { this.description = description; }

    public TicketStatus getStatus() { return status; }
    public void setStatus(TicketStatus status) { this.status = status; }

    public Priority getPriority() { return priority; }
    public void setPriority(Priority priority) { this.priority = priority; }

    // Returns Optional<String> — explicit Option<T> parallel
    public Optional<String> getAssignee() { return Optional.ofNullable(assignee); }
    public void setAssignee(String assignee) { this.assignee = assignee; }

    public LocalDateTime getCreatedAt() { return createdAt; }
    public void setCreatedAt(LocalDateTime createdAt) { this.createdAt = createdAt; }
}
