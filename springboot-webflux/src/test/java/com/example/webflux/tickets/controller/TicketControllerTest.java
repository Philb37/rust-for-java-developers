package com.example.webflux.tickets.controller;

import java.time.OffsetDateTime;
import java.util.Map;
import java.util.Optional;

import org.junit.jupiter.api.Test;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.verifyNoInteractions;
import static org.mockito.Mockito.when;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.webflux.test.autoconfigure.WebFluxTest;
import org.springframework.http.MediaType;
import org.springframework.test.context.bean.override.mockito.MockitoBean;
import org.springframework.test.web.reactive.server.WebTestClient;

import com.example.webflux.tickets.dto.StatsResponse;
import com.example.webflux.tickets.dto.TicketResponse;
import com.example.webflux.tickets.exception.TicketNotFoundException;
import com.example.webflux.tickets.model.Priority;
import com.example.webflux.tickets.model.TicketStatus;
import com.example.webflux.tickets.service.TicketService;

import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

// Web slice only: controller + validation, service is mocked, no database needed
@WebFluxTest(TicketController.class)
class TicketControllerTest {

    @Autowired
    private WebTestClient webClient;

    @MockitoBean
    private TicketService service;

    private static final TicketResponse SAMPLE = new TicketResponse(
            1,
            "Printer on fire",
            Optional.empty(),
            TicketStatus.OPEN,
            Priority.HIGH,
            Optional.empty(),
            OffsetDateTime.now()
    );

    @Test
    void createReturns201WhenRequestIsValid() {
        when(service.create(any())).thenReturn(Mono.just(SAMPLE));

        webClient.post().uri("/tickets")
                .contentType(MediaType.APPLICATION_JSON)
                .bodyValue("""
                        {"title": "Printer on fire", "priority": "HIGH"}""")
                .exchange()
                .expectStatus().isCreated()
                .expectBody()
                .jsonPath("$.id").isEqualTo(1)
                .jsonPath("$.status").isEqualTo("OPEN");
    }

    @Test
    void createReturns400WhenTitleIsBlank() {
        webClient.post().uri("/tickets")
                .contentType(MediaType.APPLICATION_JSON)
                .bodyValue("""
                        {"title": "   ", "priority": "HIGH"}""")
                .exchange()
                .expectStatus().isBadRequest();

        verifyNoInteractions(service);
    }

    @Test
    void createReturns400WhenPriorityIsMissing() {
        webClient.post().uri("/tickets")
                .contentType(MediaType.APPLICATION_JSON)
                .bodyValue("""
                        {"title": "Printer on fire"}""")
                .exchange()
                .expectStatus().isBadRequest();

        verifyNoInteractions(service);
    }

    @Test
    void updateStatusReturns400WhenStatusIsMissing() {
        webClient.patch().uri("/tickets/1/status")
                .contentType(MediaType.APPLICATION_JSON)
                .bodyValue("{}")
                .exchange()
                .expectStatus().isBadRequest();

        verifyNoInteractions(service);
    }

    @Test
    void getByIdReturns404WhenTicketDoesNotExist() {
        when(service.getById(42)).thenReturn(Mono.error(new TicketNotFoundException(42)));

        webClient.get().uri("/tickets/42")
                .exchange()
                .expectStatus().isNotFound();
    }

    @Test
    void getByIdReturns200WhenTicketExists() {
        when(service.getById(1)).thenReturn(Mono.just(SAMPLE));

        webClient.get().uri("/tickets/1")
                .exchange()
                .expectStatus().isOk()
                .expectBody()
                .jsonPath("$.title").isEqualTo("Printer on fire");
    }

    @Test
    void listPassesQueryParametersToTheService() {
        when(service.list(TicketStatus.OPEN, Priority.HIGH)).thenReturn(Flux.just(SAMPLE));

        webClient.get().uri(uri -> uri.path("/tickets")
                        .queryParam("status", "OPEN")
                        .queryParam("priority", "HIGH")
                        .build())
                .exchange()
                .expectStatus().isOk()
                .expectBody()
                .jsonPath("$[0].id").isEqualTo(1);
    }

    @Test
    void listWorksWithoutQueryParameters() {
        when(service.list(null, null)).thenReturn(Flux.just(SAMPLE));

        webClient.get().uri("/tickets")
                .exchange()
                .expectStatus().isOk()
                .expectBody()
                .jsonPath("$.length()").isEqualTo(1);
    }

    @Test
    void updateStatusReturns200WhenRequestIsValid() {
        when(service.updateStatus(1, TicketStatus.RESOLVED)).thenReturn(Mono.just(SAMPLE));

        webClient.patch().uri("/tickets/1/status")
                .contentType(MediaType.APPLICATION_JSON)
                .bodyValue("""
                        {"status": "RESOLVED"}""")
                .exchange()
                .expectStatus().isOk();
    }

    @Test
    void statsReturnsCountsGroupedByStatusAndPriority() {
        when(service.stats()).thenReturn(Mono.just(new StatsResponse(
                Map.of(TicketStatus.OPEN, 2L),
                Map.of(Priority.HIGH, 2L))));

        webClient.get().uri("/tickets/stats")
                .exchange()
                .expectStatus().isOk()
                .expectBody()
                .jsonPath("$.byStatus.OPEN").isEqualTo(2)
                .jsonPath("$.byPriority.HIGH").isEqualTo(2);
    }
}
