## Week 12 Rust Demo

In this week demo, I want to use Rust to solve Dining Philosophers problem. The Dining Philosophers problem is a classic synchronization problem that illustrates the challenges of managing concurrent threads and avoiding deadlocks. This implementation creates five philosophers and five forks (represented by Mutex) at a shared table (Arc). Each philosopher tries to pick up the left fork, waits a bit, and then picks up the right fork. After eating, the philosopher releases both forks. Using Mutex ensures that only one philosopher can hold a fork at a time. The order in which philosophers pick up forks is designed to avoid deadlocks.

I have 5 philosophers, one solution show as following:

```
Socrates picked up left fork
Descartes picked up left fork
Plato picked up left fork
Aristotle picked up left fork
Descartes picked up right fork
Descartes is eating
Descartes finished eating
Socrates picked up right fork
Socrates is eating
Socrates finished eating
Plato picked up right fork
Plato is eating
Plato finished eating
Aristotle picked up right fork
Aristotle is eating
Aristotle finished eating
Spinoza picked up left fork
Spinoza picked up right fork
Spinoza is eating
Spinoza finished eating
```

