Given that your tree, containers and sequences already work at a basic level, I would build the project in vertical slices: get one mathematical expression working from the tree through layout, rendering, cursor movement and editing before implementing the full set of mathematical structures.

I would not start by implementing every render primitive, every container type or the multi-window debugger. Those depend on interfaces you haven't tested against a working expression yet.

The target for the first milestone should be:

> Enter `a + b`, render it in the terminal, move the cursor around it, insert and delete text, and verify that the tree and rendered layout stay consistent.

Then extend that working pipeline to fractions, nested expressions, selection and alternative backends.

## Suggested implementation roadmap

## 1

Finish the minimum viable core

Start here

Your tree is already working, so stabilise its invariants before building on top of it.

* Enforce parent/child consistency and valid sequence references.

* Establish reliable insertion, deletion and container removal.

* Define what happens when a cursor's child is deleted or a container is removed.

* Add tree validation and unit tests.

* Implement a simple text sequence and operator as the first complete expression.

Exit criterion: you can construct, modify and validate nested sequences without corrupting the tree.

## 2

Implement the layout foundation

Before implementing the full renderer interface, settle the types and layout conventions.

* `Measurement`, `Point`, `Rect` and coordinate units.

* Baseline, ascent, descent and width conventions.

* `PositionedNodeRef` and `MeasurementResult`.

* Text measurement through `FontLayout`.

* Layout for sequences, text and operators.

* A clear correspondence between positioned elements and the source tree.

I would settle the last point early: the layout needs a reliable way to recover structural identity for cursor hit-testing, even if the `MeasurementResult` itself does not store `SequenceId`.

Exit criterion: `a + b` produces correct measurements and positioned nodes, with predictable baselines and dimensions.

## 3

Build one complete terminal rendering path

Start with one backend, probably ratatui, rather than implementing terminal and GUI simultaneously.

* Render text and operators.

* Map layout coordinates to terminal cells.

* Handle clipping and viewport boundaries.

* Render the cursor.

* Keep drawing separate from layout calculations.

Test long text, empty sequences, nested sequences and resizing.

Exit criterion: the terminal accurately displays a simple expression and its cursor, with no layout calculations hidden in the renderer.

## 4

Complete logical cursor movement and editing

Now connect your existing event-handling design to the working layout and renderer.

* Left/right movement within text and between children.

* Insert and delete text.

* Move into and out of containers.

* Delete empty containers while preserving the root.

* Keep the cursor valid after every structural mutation.

Write tests against cursor and tree state, not screenshots.

Exit criterion: you can edit `a + b` entirely from the keyboard, and every operation leaves a valid cursor and tree.

## 5

Implement your first real container: fractions

Fractions test nearly every important part of the architecture without requiring the complexity of matrices or large operators.

* Numerator and denominator sequences.

* Width calculation and horizontal centring.

* Baseline and vertical positioning.

* Fraction-bar rendering.

* Cursor entry and exit.

* Nested fractions and empty numerator/denominator handling.

Keep the fraction layout algorithm shared where possible, with backend-specific measurement and drawing.

Exit criterion: nested fractions render correctly and support logical cursor navigation.

## 6

Add layout-aware cursor interaction

This is where your earlier design for the `LayoutEngine` starts to pay off.

* Resolve cursor positions from layout results.

* Implement click-to-position using hit-testing.

* Implement spatial up/down movement.

* Preserve preferred horizontal position across repeated vertical movements.

* Handle transitions between outer sequences and nested containers.

Keep the navigation policy in the editor; use layout geometry to identify candidate positions.

Exit criterion: clicking and spatial navigation work inside and around fractions, including when moving between the numerator, denominator and outer expression.

## 7

Implement selection and partial containers

With cursor movement stable, implement the more complicated editing behaviours.

* Selection across text, operators and nested containers.

* Wrapping selected expressions in new containers.

* Highlighting selected text and containers.

* Partial-container confirmation and reversion.

* Greedy consumption and relinquishing unused sequence content.

Selection and partial containers interact heavily, so test them together once each works independently.

Exit criterion: selecting an expression and wrapping it in a fraction works reliably, including nested selections and deletion.

## 8

Extract shared layout algorithms and add the GUI backend

Now you have a real expression editor against which to test the abstractions.

* Move common mathematical positioning rules into shared core algorithms.

* Let backends supply font metrics, units and glyph capabilities.

* Implement the GUI layout and renderer.

* Compare terminal and GUI output for the same expressions.

* Add scripts, radicals, delimiters and matrices progressively.

Exit criterion: both backends use the same tree and editing logic and follow the same mathematical layout conventions, within their respective rendering constraints.

## 9

Build the debugger and render-type laboratory

Build this after the layout data structures have proved useful in both backends.

* Owned expression snapshots.

* Read-only render previews.

* Layout bounds, baselines and cursor-position overlays.

* Alternative layout strategies.

* Separate preview windows.

* Event traces, tree inspection and reproducible test cases.

Exit criterion: you can compare two layout strategies on the same expression and inspect precisely where their results differ.

## 10

Expand the mathematical expression system

Add the remaining containers according to their structural and layout complexity.

1. Superscripts and subscripts.

2. Radicals and extensible delimiters.

3. Matrices, vectors and aligned expressions.

4. Large operators such as sums and integrals.

5. Piecewise expressions, limits and more specialised structures.

Add the functional language or REPL integration after the expression model can represent, edit and display the mathematical structures you need.

## The dependency order at a glance

Tree invariants and tests

Layout foundation

Terminal rendering

Logical editing and cursor movement

Fractions → hit-testing and spatial movement

Selection and partial containers

Shared algorithms → GUI backend

Debugger → remaining mathematics → REPL

## Three decisions to make early

These are worth settling before implementing the layout engine in depth.

1. Layout units: allow the GUI to use fractional units, even if the terminal uses integer cells. Avoid assumptions that every unit is an integer.

2. Structural identity: decide how layout nodes correspond to `SequenceId`, `ContainerId` and child indices. This is necessary for hit-testing, spatial navigation and debugging.

3. Layout invalidation: determine when layout results become stale after editing. Initially, recomputing the entire layout after each edit is reasonable. Optimise incrementally only when you have evidence that it is too slow.

I would also add a small set of layout conformance tests early: the numerator of a fraction should be centred, matrix columns should align, and superscripts should occupy the expected region. Those tests will help preserve consistency as you add backends.

The next concrete task I would do is Phase 2: define the layout data types and render `a + b` end-to-end. That will test your interfaces before fractions, spatial navigation and the debugger make them harder to change.

I can turn that first phase into a file-by-file Rust implementation checklist, using your existing `Sequence`, `Container`, `MeasurementResult` and `LayoutEngine` designs.
