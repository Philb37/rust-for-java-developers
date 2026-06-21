package com.example.tickets.controller;

import java.util.List;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PatchMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import com.example.tickets.dto.CreateTicketRequest;
import com.example.tickets.dto.StatsResponse;
import com.example.tickets.dto.TicketResponse;
import com.example.tickets.dto.UpdateStatusRequest;
import com.example.tickets.exception.TicketNotFoundException;
import com.example.tickets.model.Priority;
import com.example.tickets.model.TicketStatus;
import com.example.tickets.service.TicketService;

import jakarta.validation.Valid;

@RestController
@RequestMapping("/tickets")
public class TicketController {

    private final TicketService service;

    public TicketController(TicketService service) {
        this.service = service;
    }

    // POST /tickets
    @PostMapping
    public ResponseEntity<TicketResponse> create(@Valid @RequestBody CreateTicketRequest request) {
        return ResponseEntity.status(HttpStatus.CREATED).body(service.create(request));
    }

    // GET /tickets/stats  — declared before /{id} so Spring resolves "stats" before the path variable
    @GetMapping("/stats")
    public ResponseEntity<StatsResponse> stats() {
        return ResponseEntity.ok(service.stats());
    }

    // GET /tickets/{id}
     @GetMapping("/{id}")
    public ResponseEntity<TicketResponse> getById(@PathVariable Integer id) {
        return ResponseEntity.ok(service.getById(id));
    }

    // GET /tickets?status=OPEN&priority=HIGH
    @GetMapping
    public ResponseEntity<List<TicketResponse>> list(
            @RequestParam(required = false) TicketStatus status,
            @RequestParam(required = false) Priority priority) {
        return ResponseEntity.ok(service.list(status, priority));
    }

    // PATCH /tickets/{id}/status
    @PatchMapping("/{id}/status")
    public ResponseEntity<TicketResponse> updateStatus(
            @PathVariable Integer id,
            @Valid @RequestBody UpdateStatusRequest request) {
        return ResponseEntity.ok(service.updateStatus(id, request.status()));
    }

    // Maps TicketNotFoundException → HTTP 404
    @ExceptionHandler(TicketNotFoundException.class)
    public ResponseEntity<String> handleNotFound(TicketNotFoundException ex) {
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(ex.getMessage());
    }
}
