# Notes for tracking new features and improvements to be done

## Algorithm
* For task like sleep, the task should be able to span across 2 days (start at the end of a day and finish towards the earlier part of the next day)
* Add the ability to give certain tasks free time before and after it
* Add the ability to give certain tasks descriptions about them (Useful for instance what workout in the gym that day is for)
* Change the `constraint name` to `constraint type` and reintroduce a field constraint name that can be whatever the user wants (must be unique)
* Add generic pairwise gaps (i.e. Setting a gap between two constraints not of the same type. e.g. gym and badminton should have a gap)
    - Can also introduce a new type of constraint that makes it so that two tasks of differying types cannot be scheduled on the same day as well
* Tabu search make the optimisation algorithm runs faster
* Constraints to explicitly specify where a task should not be scheduled
    - Can specify a time for which after/before a task should not be scheduled
* Algorithm optimisations to make it run faster
* A way to make the days more balanced. It should avoid scheduling everything for instance on a Tuesday and nothing on a Sunday and instead should balance the days

## UI
* Given a manually set schedule, a way to list out what constraints it is breaking

