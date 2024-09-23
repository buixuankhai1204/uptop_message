# Module Identification of Uptop Backend

The purpose of this project is to write a all application that follows basic Domain Driven Design principles.

## Distribution Domain

- User
- Ip Address
- Ip Logs

## Structure

The source code is explicitly split into four of the typical DDD layers:

- **Domain** - Where the business rules of the application reside.
- **Application** - The layer that orchestrates Domain and Infrastructure, and contains the use cases for your application.
- **Infrastructure** - Contains implementations of the abstractions defined in the Domain and Application layer, and other infrastructure details.
- **Interfaces** - Defines how to present the data and defined the controller actions.

`main.rs` contains the initializations of the infrastructure implementations.
