# Atopsy

## Ruleset

### CPU Rules

1. CPU usage [@Instant]
2. Rate of change of CPU load avg [@Window]
3. No. of CPUs above threshold [@Instant]
4. Sustained IRQ time above threshold [@Window]

### Memory Rules

1. Number of OOM kills [@Instant]
2. Sustained high memory usage [@Window]
3. Sustained high swap usage [@Window]
4. Number of allocation stalls [@Instant]

### Pressure Rules

1. Pressure above threshold [@Instant]
