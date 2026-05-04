# Skills and learn scheduler notes

* Introduction to the problem
    - What is the real world problem trying to solve
* What is the classification of this problem in computer science
    - What is NP complete
    - How NP-comlete problems are typically solved
* Algorithms used for solving optimisation Algorithms
    - Hill climbing
    - Genetic search
    - Simulated annealing
    - Tabu search 
* Modelling the problem
    - The schedule
        - Grid dimensions
    - The constraint model
* The basic algorithm
    - Simple hill climber
    - The penalties
    - The move operators
    - The accepting and rejecting of schedules
* The optimisation algorithm development
    - The development journey
        - Starting off
            * 0c19962 - (4 months ago) Initial commit - Danuk
            * 5cfd2fb - (4 months ago) Added initial schedule struct and constraint struct - Danuk
            * dc88e1c - (4 months ago) Added placeholder functions for constraint builder - Danuk
        - Implementing the penalty functions
            * 53d4aae - (4 months ago) Added wip function for calculating aggregated penalty for a constraint - Danuk
            * fe540b5 - (4 months ago) Added penalty functions to calculate allowed slots based penalty and validity based penalty - Danuk
            * d292501 - (4 months ago) Added penalty function for preferred slot based penalty and refactored some code - Danuk
            * 1b83dce - (4 months ago) Added initial state generation and calculating total penalty for a solution - Danuk
        - Implementing optimisation algorithm
            * 44b814a - (4 months ago) Added wip hill climber - Danuk
            * bcab246 - (4 months ago) Added a constraint store to isolate storing constraints and picking constraints randomly wieghted by its penalty - Danuk
            * 4ca9b4c - (4 months ago) Reworked schedule.rs - Danuk
            * 1dd29db - (4 months ago) Removed scheduled slot from constraint.rs - Danuk
        - Added the initial move operators
            * c647b77 - (3 months ago) Refactored codebase and added make small change function - Danuk
        - Small improvements
            * ca90d5e - (3 months ago) Refactored the hillclimber - Danuk
            * 7d07d9e - (2 months ago) Added docstring - Danuk
            * 43b098d - (2 months ago) Moved penalties out of constraint store - Danuk
            * 44ba729 - (8 weeks ago) Fixed penalty calculation syntax and added docstrings - Danuk
        - Added reversing logic
            * acf0b42 - (8 weeks ago) Added reverting back changes when no improvement is found - Danuk
            * 9dba0bf - (8 weeks ago) Added docstring and fixed typo - Danuk
            * f53babf - (8 weeks ago) Commented out some unused code - Danuk
            * 465233d - (8 weeks ago) Added some debugging statements - Danuk
        - Getting first outputs
            * 03de291 - (8 weeks ago) Added a csv serialiser for the schedule - Danuk
        - Some nice quality of life improvements
            * 3e91fc7 - (7 weeks ago) Added json based constraint store serialiser/deserialiser and added some more improvements to the hill climber - danuk
        - Getting smarter (Not really)
            * 87586f8 - (6 weeks ago) Implemented simulated annealing - danuk
        - Diving deep into the problem
            * ed90309 - (6 weeks ago) Added printing scheduling report - danuk
        - Incremental improvement
            * 9e45e9a - (5 weeks ago) Refactored change types into mode fundamental Scheduled and Unscheduled types - danuk
        - A shot in the dark
            * 92fc9d4 - (4 weeks ago) Added new optimisation step where a random duration is selected and unscheduled to make way for a new scheduling - danuk
            * 8ebfb7a - (4 weeks ago) Replaced optimisation step - danuk
            * 6aa0ab7 - (4 weeks ago) Fixed bug with step size when unscheduling multiple constraints - danuk
            * f6e5a1f - (4 weeks ago) Cleaned up the code base a little - danuk
        - Some nice software engineering bits
            * 0c64128 - (4 weeks ago) Added a logger and refactored printlns with log statements - danuk
            * f22eeae - (4 weeks ago) Fixed typo - danuk
            * 1016f88 - (4 weeks ago) Refactored hill_climber.rs to be more readable - danuk
            * 85f7faa - (4 weeks ago) Moved initial temperature and cooling factor as function parameters of the algorithm - danuk
            * f943d89 - (4 weeks ago) Added panicing upon erroring out - danuk
            * 3a340ce - (4 weeks ago) Small readability improvement - danuk
        - To be continued
            * b673a26 - (4 weeks ago) Updated gitignore to ignore flamegraph - danuk
            * a669576 - (4 weeks ago) Allowed dead code for constraint builder - danuk
        - Diving even deeper
            * d6fdd4b - (4 weeks ago) Added a wip penalty report - danuk
            * bdbdf33 - (4 weeks ago) Implemented penalty report - danuk
            * 60b87bb - (4 weeks ago) Added constraint id to the penalty report - danuk
            * 1dd7a39 - (3 weeks ago) Fixed bug with calculating gap between constraint schedulings - danuk
        - Its getting hot
            * d281089 - (3 weeks ago) Added reheating based on stagnant counter and fixed a potential divide by zero bug - danuk
            * d2d444e - (3 weeks ago) Added new struct to store statistics on optimisation algorithm for better debugging - danuk
            * 2e1ca47 - (3 weeks ago) Added a optimisation algorithm report for debugging - danuk
            * fb1b62c - (16 minutes ago) Added some stats and probablities to choosing a move - danuk
* Quick run through the code 
* The next step and my vision for the tool 
    - Having static constraints (Constraints that are pinned on the schedule and is not optimisable)
    - Tracking how the week is going and readjusting the schedule based on that
