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

### Top - Down Planning
1. Rule Engine - 
    Responbile for creating instant and window\
    Instant Rule - threshold, Calculate Score\
    Window Rule - threshold, Calculate Score over a slice\

2. Cpu Rule - 
    CPU Usage - 
