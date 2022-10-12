# Loosely-Coupled-Pallets
Loose coupling solves the problem of coupling to a specific implementation. When loosely coupling to another pallet, you add an associated type to the pallet's configuration trait and ensure the supplied type implements the necessary behavior by specifying a trait bound.
