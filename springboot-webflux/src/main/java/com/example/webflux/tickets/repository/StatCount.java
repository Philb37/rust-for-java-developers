package com.example.webflux.tickets.repository;

// One row of the GROUPING SETS aggregate: either a status or a priority is set
// (the other is null), plus its count.
public record StatCount(String status, String priority, Long count) {}
