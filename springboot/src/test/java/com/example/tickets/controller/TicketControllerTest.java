package com.example.tickets.controller;

import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.verifyNoInteractions;
import static org.mockito.Mockito.when;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.patch;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.post;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;
import java.util.Optional;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.webmvc.test.autoconfigure.WebMvcTest;
import org.springframework.http.MediaType;
import org.springframework.test.context.bean.override.mockito.MockitoBean;
import org.springframework.test.web.servlet.MockMvc;

import com.example.tickets.dto.StatsResponse;
import com.example.tickets.dto.TicketResponse;
import com.example.tickets.exception.TicketNotFoundException;
import com.example.tickets.model.Priority;
import com.example.tickets.model.TicketStatus;
import com.example.tickets.service.TicketService;

// Web slice only: controller + validation, service is mocked, no database needed
@WebMvcTest(TicketController.class)
class TicketControllerTest {

    @Autowired
    private MockMvc mockMvc;

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
    void createReturns201WhenRequestIsValid() throws Exception {
        when(service.create(any())).thenReturn(SAMPLE);

        mockMvc.perform(post("/tickets")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content("""
                                {"title": "Printer on fire", "priority": "HIGH"}"""))
                .andExpect(status().isCreated())
                .andExpect(jsonPath("$.id").value(1))
                .andExpect(jsonPath("$.status").value("OPEN"));
    }

    @Test
    void createReturns400WhenTitleIsBlank() throws Exception {
        mockMvc.perform(post("/tickets")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content("""
                                {"title": "   ", "priority": "HIGH"}"""))
                .andExpect(status().isBadRequest());

        verifyNoInteractions(service);
    }

    @Test
    void createReturns400WhenPriorityIsMissing() throws Exception {
        mockMvc.perform(post("/tickets")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content("""
                                {"title": "Printer on fire"}"""))
                .andExpect(status().isBadRequest());

        verifyNoInteractions(service);
    }

    @Test
    void updateStatusReturns400WhenStatusIsMissing() throws Exception {
        mockMvc.perform(patch("/tickets/1/status")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content("{}"))
                .andExpect(status().isBadRequest());

        verifyNoInteractions(service);
    }

    @Test
    void getByIdReturns404WhenTicketDoesNotExist() throws Exception {
        when(service.getById(42)).thenThrow(new TicketNotFoundException(42));

        mockMvc.perform(get("/tickets/42"))
                .andExpect(status().isNotFound());
    }

    @Test
    void getByIdReturns200WhenTicketExists() throws Exception {
        when(service.getById(1)).thenReturn(SAMPLE);

        mockMvc.perform(get("/tickets/1"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.title").value("Printer on fire"));
    }

    @Test
    void listPassesQueryParametersToTheService() throws Exception {
        when(service.list(TicketStatus.OPEN, Priority.HIGH)).thenReturn(List.of(SAMPLE));

        mockMvc.perform(get("/tickets")
                        .queryParam("status", "OPEN")
                        .queryParam("priority", "HIGH"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$[0].id").value(1));
    }

    @Test
    void listWorksWithoutQueryParameters() throws Exception {
        when(service.list(null, null)).thenReturn(List.of(SAMPLE));

        mockMvc.perform(get("/tickets"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.length()").value(1));
    }

    @Test
    void updateStatusReturns200WhenRequestIsValid() throws Exception {
        when(service.updateStatus(1, TicketStatus.RESOLVED)).thenReturn(SAMPLE);

        mockMvc.perform(patch("/tickets/1/status")
                        .contentType(MediaType.APPLICATION_JSON)
                        .content("""
                                {"status": "RESOLVED"}"""))
                .andExpect(status().isOk());
    }

    @Test
    void statsReturnsCountsGroupedByStatusAndPriority() throws Exception {
        when(service.stats()).thenReturn(new StatsResponse(
                Map.of(TicketStatus.OPEN, 2L),
                Map.of(Priority.HIGH, 2L)));

        mockMvc.perform(get("/tickets/stats"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.byStatus.OPEN").value(2))
                .andExpect(jsonPath("$.byPriority.HIGH").value(2));
    }
}
