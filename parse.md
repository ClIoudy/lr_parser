- returns: Result\<S>
- lookahead = last stack element
- next(): push next token to stack, or eof if none present
- loop over actions 

- shift:
  - get next state
  - advance to next state
- reduction:
  - get length n
  - construct new symbol with the last n stack elements
  - check if end state
  - revert to state n symbols ago
  - advance state by new rule symbol?
- if neither: error with expected tokens

# give variant a generic for super type?

## requires:
table has...
- end state
- expected tokens for given state
  - tokens to consider as "expected": literal tokens + first elements of expected rule symbols
- actions via state + token (including eof) 