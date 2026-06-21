package com.example.webflux.tickets.controller;

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
import org.springframework.web.bind.annotation.ResponseStatus;
import org.springframework.web.bind.annotation.RestController;

import com.example.webflux.tickets.dto.CreateTicketRequest;
import com.example.webflux.tickets.dto.StatsResponse;
import com.example.webflux.tickets.dto.TicketResponse;
import com.example.webflux.tickets.dto.UpdateStatusRequest;
import com.example.webflux.tickets.exception.TicketNotFoundException;
import com.example.webflux.tickets.model.Priority;
import com.example.webflux.tickets.model.TicketStatus;
import com.example.webflux.tickets.service.TicketService;

import jakarta.validation.Valid;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

@RestController
@RequestMapping("/tickets")
public class TicketController {

    private final TicketService service;

    public TicketController(TicketService service) {
        this.service = service;
    }

    // POST /tickets
    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public Mono<TicketResponse> create(@Valid @RequestBody CreateTicketRequest request) {
        return service.create(request);
    }

    // GET /tickets/stats  — declared before /{id} so Spring resolves "stats" before the path variable
    @GetMapping("/stats")
    public Mono<StatsResponse> stats() {
        return service.stats();
    }

    // GET /tickets/{id}
    @GetMapping("/{id}")
    public Mono<TicketResponse> getById(@PathVariable Integer id) {
        return service.getById(id);
    }

    // GET /tickets?status=OPEN&priority=HIGH
    @GetMapping
    public Flux<TicketResponse> list(
            @RequestParam(required = false) TicketStatus status,
            @RequestParam(required = false) Priority priority) {
        return service.list(status, priority);
    }

    // PATCH /tickets/{id}/status
    @PatchMapping("/{id}/status")
    public Mono<TicketResponse> updateStatus(
            @PathVariable Integer id,
            @Valid @RequestBody UpdateStatusRequest request) {
        return service.updateStatus(id, request.status());
    }

    // Maps TicketNotFoundException → HTTP 404
    @ExceptionHandler(TicketNotFoundException.class)
    public ResponseEntity<String> handleNotFound(TicketNotFoundException ex) {
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(ex.getMessage());
    }
}
