package com.example.tickets.controller;

import com.example.tickets.dto.CreateTicketRequest;
import com.example.tickets.dto.StatsResponse;
import com.example.tickets.dto.TicketResponse;
import com.example.tickets.dto.UpdateStatusRequest;
import com.example.tickets.exception.TicketNotFoundException;
import com.example.tickets.model.Priority;
import com.example.tickets.model.TicketStatus;
import com.example.tickets.service.TicketService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/tickets")
public class TicketController {

    private final TicketService service;

    public TicketController(TicketService service) {
        this.service = service;
    }

    // POST /tickets
    @PostMapping
    public ResponseEntity<TicketResponse> create(@RequestBody CreateTicketRequest request) {
        return ResponseEntity.status(HttpStatus.CREATED).body(service.create(request));
    }

    // GET /tickets/stats  — declared before /{id} so Spring resolves "stats" before the path variable
    @GetMapping("/stats")
    public ResponseEntity<StatsResponse> stats() {
        return ResponseEntity.ok(service.stats());
    }

    // GET /tickets/{id}
    // Rust parallel: findById → Option<Ticket> → .ok_or(TicketNotFoundError)?
    @GetMapping("/{id}")
    public ResponseEntity<TicketResponse> getById(@PathVariable Integer id) {
        return ResponseEntity.ok(service.getById(id));
    }

    // GET /tickets?status=OPEN&priority=HIGH
    // Rust parallel: list(status: Option<TicketStatus>, priority: Option<Priority>) -> Vec<TicketResponse>
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
            @RequestBody UpdateStatusRequest request) {
        return ResponseEntity.ok(service.updateStatus(id, request.status()));
    }

    // Maps TicketNotFoundException → HTTP 404
    // Rust parallel: match result { Err(TicketNotFoundError(id)) => HttpResponse::NotFound()... }
    @ExceptionHandler(TicketNotFoundException.class)
    public ResponseEntity<String> handleNotFound(TicketNotFoundException ex) {
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(ex.getMessage());
    }
}
